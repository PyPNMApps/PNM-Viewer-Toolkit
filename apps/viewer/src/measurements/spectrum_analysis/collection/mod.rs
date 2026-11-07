/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Standard-spectrum collections grouped by device, exact acquisition and epoch.
mod session;
use crate::capture::{collection as importer, Diagnostic};
pub use importer::ImportLimits;
use pnm::pnm::{
    analysis::spectrum::{GroupKey, Series, Source, SpectrumAggregate},
    parser::{
        cm_spectrum_analysis::CmSpectrumAnalysis, parse_limits::ParseLimits,
        pnm_file_type::PnmDecoderKind, pnm_header::PnmHeader,
    },
};
use serde::Serialize;
pub use session::Session;
use std::{collections::BTreeMap, path::Path, sync::atomic::AtomicBool};

/// Compatible acquisition group and ordered binary-header capture chronology.
#[derive(Serialize)]
pub struct Group {
    pub id: usize,
    pub label: String,
    pub tree_label: String,
    #[serde(flatten)]
    pub key: GroupKey,
    pub sources: Vec<Source>,
    pub capture_count: usize,
    pub first_epoch: u32,
    pub last_epoch: u32,
}
/// Small import catalog; payloads remain in Rust.
#[derive(Serialize)]
pub struct Catalog {
    pub groups: Vec<Group>,
    pub accepted_count: usize,
    pub duplicate_count: usize,
    pub skipped_count: usize,
    pub issues: Vec<String>,
}
/// Owns each decoded capture once; settings groups have independent chronology.
pub struct Collection {
    pub catalog: Catalog,
    groups: Vec<Vec<Series>>,
}
impl Collection {
    /// Imports standard PNN9 files from a folder/archive using shared safety bounds.
    /// # Arguments
    /// * `path` - Local user-selected input, independent of file/member names.
    /// * `limits` - Archive, byte and decoded-sample budgets.
    /// * `cancel` - Cooperative cancellation flag.
    /// # Returns
    /// Device/settings groups sorted by exact keys and ascending header epoch.
    /// # Errors
    /// Rejects exhausted/cancelled imports, invalid archives and no supported captures.
    pub fn scan(
        path: &Path,
        limits: ImportLimits,
        cancel: &AtomicBool,
    ) -> Result<Self, Diagnostic> {
        let scanned = importer::scan(path, limits, cancel, decode)?;
        if scanned.series.is_empty() {
            return Err(Diagnostic::new(
                "SpectrumImport",
                "No usable standard PNM spectrum captures found.",
            ));
        }
        let mut grouped: BTreeMap<GroupKey, Vec<Series>> = BTreeMap::new();
        for series in scanned.series {
            grouped.entry(series.key.clone()).or_default().push(series);
        }
        let mut groups: Vec<_> = grouped.into_values().collect();
        for captures in &mut groups {
            captures.sort_by(|first, last| {
                (first.capture_time, &first.source).cmp(&(last.capture_time, &last.source))
            });
        }
        let catalog_groups=groups.iter().enumerate().map(|(id,captures)| {
            let key=captures[0].key.clone();
            let frequencies=key.frequencies_hz().expect("Validated imported spectrum grid");
            let label=format!("Settings {} · {:.3}–{:.3} MHz · RBW {:.3} kHz · Ch {} · {} bins/segment · ENBW {} · Window {} · v{}.{} · {} bins", id+1,f64::from(frequencies[0])/1e6,f64::from(*frequencies.last().unwrap())/1e6,key.resolution_bandwidth_hz().unwrap()/1e3,key.channel_id,key.bins_per_segment,key.encoded_enbw,key.window_function,key.major_version,key.minor_version,key.sample_count);
            Group { id,tree_label:format!("Settings {} · {:.3}–{:.3} MHz",id+1,f64::from(frequencies[0])/1e6,f64::from(*frequencies.last().unwrap())/1e6),label,key,
                capture_count:captures.len(),first_epoch:captures[0].capture_time,last_epoch:captures.last().unwrap().capture_time,
                sources:captures.iter().map(|series|Source{name:series.source.clone(),capture_time:series.capture_time}).collect() }
        }).collect();
        Ok(Self {
            catalog: Catalog {
                groups: catalog_groups,
                accepted_count: groups.iter().map(Vec::len).sum(),
                duplicate_count: scanned.duplicates,
                skipped_count: scanned.skipped,
                issues: scanned.issues,
            },
            groups,
        })
    }
    /// Calculates full-resolution temporal statistics for one exact settings group.
    /// # Arguments
    /// * `group` - Current catalog ID.
    /// * `frame` - One ascending epoch index for playback, or None for all captures.
    /// # Returns
    /// Rust-owned unrounded statistics and chronological sources.
    /// # Errors
    /// Rejects unavailable groups/frames or incompatible series.
    pub fn aggregate(
        &self,
        group: usize,
        frame: Option<usize>,
    ) -> Result<SpectrumAggregate, Diagnostic> {
        let captures = self
            .groups
            .get(group)
            .ok_or_else(|| Diagnostic::new("SpectrumGroup", "Select capture settings."))?;
        let selected: Vec<_> = match frame {
            Some(index) => vec![captures.get(index).ok_or_else(|| {
                Diagnostic::new("SpectrumFrame", "Select an available capture epoch.")
            })?],
            None => captures.iter().collect(),
        };
        SpectrumAggregate::calculate(&selected)
            .map_err(|error| Diagnostic::new("SpectrumAggregate", error))
    }
}
fn decode(bytes: &[u8], name: &str) -> Result<Option<(Series, usize)>, Diagnostic> {
    let header = match PnmHeader::from_bytes(bytes) {
        Ok(header) if header.descriptor().decoder == PnmDecoderKind::SpectrumAnalysis => header,
        _ => return Ok(None),
    };
    let capture = CmSpectrumAnalysis::from_header(header, ParseLimits::default())
        .map_err(|error| Diagnostic::new("SpectrumImport", error))?;
    let series = Series::from_capture(&capture, name.into())
        .map_err(|error| Diagnostic::new("SpectrumImport", error))?;
    let count = series.values_dbmv.len();
    Ok(Some((series, count)))
}
