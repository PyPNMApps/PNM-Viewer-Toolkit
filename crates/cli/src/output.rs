/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Publish complete JSON without replacing an existing destination.
use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
const MAX_TEMP_ATTEMPTS: usize = 100;
static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);
struct TemporaryFile(PathBuf);
impl Drop for TemporaryFile {
    /// Removes only the temporary file created by this operation.
    /// # Returns
    /// Nothing; cleanup failure cannot replace the original operation result.
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.0);
    }
}
/// Writes and publishes a new output file without replacing any existing entry.
/// # Arguments
/// * `destination` - Output path; parent must already exist.
/// * `json` - Complete serialized JSON bytes.
/// # Returns
/// Success after publication; input and existing outputs are never overwritten.
/// # Errors
/// Returns I/O errors including existing destinations or unsupported hard links.
/// A hard link publishes the complete sibling file atomically without a replace
/// race. Filesystems without hard-link support fail safely, without a fallback.
pub fn save(destination: &Path, json: &[u8]) -> io::Result<()> {
    if fs::symlink_metadata(destination).is_ok() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            "destination already exists",
        ));
    }
    let parent = destination
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    for _ in 0..MAX_TEMP_ATTEMPTS {
        let sequence = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".pnm-decode-{}-{sequence}.tmp", std::process::id()));
        let mut temporary = match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        };
        let cleanup = TemporaryFile(path);
        temporary.write_all(json)?;
        temporary.write_all(b"\n")?;
        temporary.sync_all()?;
        drop(temporary);
        fs::hard_link(&cleanup.0, destination)?;
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::AlreadyExists,
        "cannot allocate a temporary output file",
    ))
}
