/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! In-memory archive traversal: never unpack member paths into the filesystem.
mod decimation;
mod export;
use crate::capture::Diagnostic;
pub use decimation::joint_indices;
pub use export::write_json_new;
use sha2::{Digest, Sha256};
use std::{
    collections::HashSet,
    io::{Cursor, Read},
    path::{Component, Path},
    sync::atomic::{AtomicBool, Ordering},
};

const MAX_SOURCE_NAME_BYTES: usize = 4096;
/// Decoder callback returning one owned format-specific series and its sample count.
pub type CaptureDecoder<Series> = fn(&[u8], &str) -> Result<Option<(Series, usize)>, Diagnostic>;

/// Application limits shared across the entire recursive import.
#[derive(Clone, Copy)]
pub struct ImportLimits {
    pub max_entries: usize,
    pub max_depth: usize,
    pub max_file_bytes: usize,
    pub max_expanded_bytes: usize,
    pub max_samples: usize,
}
impl Default for ImportLimits {
    /// Bounds desktop imports without imposing new DOCSIS format requirements.
    /// # Returns
    /// 4096 entries, depth 8, 64 MiB/file, 256 MiB total reads and 4M samples.
    fn default() -> Self {
        Self {
            max_entries: 4096,
            max_depth: 8,
            max_file_bytes: 64 * 1024 * 1024,
            max_expanded_bytes: 256 * 1024 * 1024,
            max_samples: 4_000_000,
        }
    }
}
pub struct Scanned<Series> {
    pub series: Vec<Series>,
    pub duplicates: usize,
    pub skipped: usize,
    pub issues: Vec<String>,
}
struct Scanner<'request, Series> {
    decode: CaptureDecoder<Series>,
    limits: ImportLimits,
    cancel: &'request AtomicBool,
    entries: usize,
    bytes: usize,
    samples: usize,
    seen: HashSet<[u8; 32]>,
    result: Scanned<Series>,
}
/// Traverses bounded local captures/archives using the supplied format decoder.
///
/// # Arguments
/// * `path` - User-selected local root.
/// * `limits` - Shared entry, byte, nesting and decoded-sample bounds.
/// * `cancel` - Cooperative cancellation flag.
/// * `decode` - Returns one owned series/count, None for other formats, or a diagnostic.
/// # Returns
/// Accepted series and bounded duplicate/skipped diagnostics; may be empty.
/// # Errors
/// Rejects unsafe roots, corrupt archives, exhausted limits and cancellation.
pub fn scan<Series>(
    path: &Path,
    limits: ImportLimits,
    cancel: &AtomicBool,
    decode: CaptureDecoder<Series>,
) -> Result<Scanned<Series>, Diagnostic> {
    let mut scanner = Scanner {
        decode,
        limits,
        cancel,
        entries: 0,
        bytes: 0,
        samples: 0,
        seen: HashSet::new(),
        result: Scanned {
            series: Vec::new(),
            duplicates: 0,
            skipped: 0,
            issues: Vec::new(),
        },
    };
    scanner.path(path, 0)?;
    Ok(scanner.result)
}
impl<Series> Scanner<'_, Series> {
    fn check(&mut self, depth: usize) -> Result<(), Diagnostic> {
        if self.cancel.load(Ordering::Relaxed) {
            return Err(Diagnostic::new("Cancelled", "Capture import cancelled."));
        }
        self.entries += 1;
        if self.entries > self.limits.max_entries || depth > self.limits.max_depth {
            return Err(Diagnostic::new(
                "ImportLimit",
                "Capture import exceeds entry or nesting limits.",
            ));
        }
        Ok(())
    }
    fn skip(&mut self, name: &str, reason: impl std::fmt::Display) {
        self.result.skipped += 1;
        if self.result.issues.len() < 64 {
            self.result.issues.push(format!("{name}: {reason}"));
        }
    }
    fn read(&mut self, reader: impl Read) -> Result<Vec<u8>, Diagnostic> {
        let available = self
            .limits
            .max_expanded_bytes
            .saturating_sub(self.bytes)
            .min(self.limits.max_file_bytes);
        let mut bytes = Vec::new();
        reader
            .take(available as u64 + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| Diagnostic::new("ImportRead", error))?;
        if bytes.len() > available {
            return Err(Diagnostic::new(
                "ImportLimit",
                "Capture import exceeds file or expanded-byte limits.",
            ));
        }
        self.bytes += bytes.len();
        Ok(bytes)
    }
    fn path(&mut self, path: &Path, depth: usize) -> Result<(), Diagnostic> {
        self.check(depth)?;
        let metadata = std::fs::symlink_metadata(path)
            .map_err(|error| Diagnostic::new("ImportRead", error))?;
        if metadata.file_type().is_symlink() {
            self.skip(&path.to_string_lossy(), "Symlinks are not followed.");
            return Ok(());
        }
        if metadata.is_dir() {
            let mut paths = Vec::new();
            for entry in
                std::fs::read_dir(path).map_err(|error| Diagnostic::new("ImportRead", error))?
            {
                if paths.len() >= self.limits.max_entries {
                    return Err(Diagnostic::new(
                        "ImportLimit",
                        "Directory exceeds entry limit.",
                    ));
                }
                paths.push(
                    entry
                        .map_err(|error| Diagnostic::new("ImportRead", error))?
                        .path(),
                );
            }
            paths.sort();
            for child in paths {
                self.path(&child, depth + 1)?;
            }
        } else if metadata.is_file() {
            if metadata.len() > self.limits.max_file_bytes as u64 {
                return Err(Diagnostic::new(
                    "ImportLimit",
                    "Input file exceeds the import limit.",
                ));
            }
            let file =
                std::fs::File::open(path).map_err(|error| Diagnostic::new("ImportRead", error))?;
            let bytes = self.read(file)?;
            self.bytes(&bytes, &path.to_string_lossy(), depth)?;
        } else {
            self.skip(&path.to_string_lossy(), "Not a regular file.");
        }
        Ok(())
    }
    fn bytes(&mut self, bytes: &[u8], name: &str, depth: usize) -> Result<(), Diagnostic> {
        self.check(depth)?;
        if name.len() > MAX_SOURCE_NAME_BYTES {
            return Err(Diagnostic::new(
                "ImportLimit",
                "Archive source name exceeds the metadata limit.",
            ));
        }
        if bytes.starts_with(&[0x1f, 0x8b]) {
            let expanded = self.read(flate2::read::MultiGzDecoder::new(bytes))?;
            return self.bytes(&expanded, name, depth + 1);
        }
        if bytes.starts_with(&[0x28, 0xb5, 0x2f, 0xfd]) {
            let mut decoder = zstd::stream::read::Decoder::new(bytes)
                .map_err(|error| Diagnostic::new("Archive", error))?;
            decoder
                .window_log_max(23)
                .map_err(|error| Diagnostic::new("Archive", error))?;
            let expanded = self.read(decoder)?;
            return self.bytes(&expanded, name, depth + 1);
        }
        if bytes.starts_with(b"PK\x03\x04") || bytes.starts_with(b"PK\x05\x06") {
            let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
                .map_err(|error| Diagnostic::new("Archive", error))?;
            if archive.len() > self.limits.max_entries.saturating_sub(self.entries) {
                return Err(Diagnostic::new("ImportLimit", "ZIP exceeds entry limit."));
            }
            for index in 0..archive.len() {
                self.check(depth + 1)?;
                let mut member = archive
                    .by_index(index)
                    .map_err(|error| Diagnostic::new("Archive", error))?;
                let label = format!("{name}!{}", member.name());
                if !safe_path(Path::new(member.name()))
                    || member
                        .unix_mode()
                        .is_some_and(|mode| mode & 0o170000 == 0o120000)
                {
                    self.skip(&label, "Unsafe archive path or symlink.");
                    continue;
                }
                if member.is_dir() {
                    continue;
                }
                let content = self.read(&mut member)?;
                self.bytes(&content, &label, depth + 1)?;
            }
            return Ok(());
        }
        if bytes.get(257..262) == Some(b"ustar") {
            let mut archive = tar::Archive::new(bytes);
            for member in archive
                .entries()
                .map_err(|error| Diagnostic::new("Archive", error))?
            {
                self.check(depth + 1)?;
                let mut member = member.map_err(|error| Diagnostic::new("Archive", error))?;
                let path = member
                    .path()
                    .map_err(|error| Diagnostic::new("Archive", error))?
                    .into_owned();
                let label = format!("{name}!{}", path.display());
                if !safe_path(&path) || !member.header().entry_type().is_file() {
                    if !member.header().entry_type().is_dir() {
                        self.skip(&label, "Unsafe archive path or nonregular member.");
                    }
                    continue;
                }
                let content = self.read(&mut member)?;
                self.bytes(&content, &label, depth + 1)?;
            }
            return Ok(());
        }
        let (series, sample_count) = match (self.decode)(bytes, name) {
            Ok(Some(decoded)) => decoded,
            Ok(None) => {
                self.skip(name, "Not a supported capture for this collection.");
                return Ok(());
            }
            Err(error) => {
                self.skip(name, error.message);
                return Ok(());
            }
        };
        let digest: [u8; 32] = Sha256::digest(bytes).into();
        if !self.seen.insert(digest) {
            self.result.duplicates += 1;
            return Ok(());
        }
        self.samples = self
            .samples
            .checked_add(sample_count)
            .ok_or_else(|| Diagnostic::new("ImportLimit", "Sample count overflow."))?;
        if self.samples > self.limits.max_samples {
            return Err(Diagnostic::new(
                "ImportLimit",
                "Capture import exceeds decoded sample limit.",
            ));
        }
        self.result.series.push(series);
        Ok(())
    }
}
fn safe_path(path: &Path) -> bool {
    let name = path.to_string_lossy();
    !name.contains('\\')
        && !name.contains(':')
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_) | Component::CurDir))
}
