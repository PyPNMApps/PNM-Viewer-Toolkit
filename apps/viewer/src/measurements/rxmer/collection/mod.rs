/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded batch import and immutable temporal RxMER collections.
mod import;
use crate::capture::{Diagnostic, MAX_VIEW_SAMPLES};
pub use import::ImportLimits;
pub use pnm::pnm::analysis::rxmer::aggregate::RxMerAggregate as AggregateResult;
use pnm::pnm::analysis::rxmer::aggregate::{GroupKey, RxMerAggregate, Series};
use serde::Serialize;
use std::{collections::BTreeMap, path::Path, sync::atomic::AtomicBool};

/// A compatible group; IDs are collection-local and unrelated to filenames.
#[derive(Serialize)]
pub struct Group {
    pub id: usize,
    pub label: String,
    #[serde(flatten)]
    pub key: GroupKey,
    pub capture_count: usize,
    pub first_epoch: u32,
    pub last_epoch: u32,
}
/// Import accounting and device/channel choices without payload arrays.
#[derive(Serialize)]
pub struct Catalog {
    pub groups: Vec<Group>,
    pub accepted_count: usize,
    pub duplicate_count: usize,
    pub skipped_count: usize,
    pub issues: Vec<String>,
}
/// Owns decoded samples once; the UI receives only catalog and bounded plots.
pub struct Collection {
    groups: Vec<Vec<Series>>,
    pub catalog: Catalog,
}
impl Collection {
    /// Scans a directory, capture or supported archive without extracting to disk.
    /// # Arguments
    /// * `path` - Local directory or archive chosen by the user.
    /// * `limits` - Import safety bounds, independent of protocol limits.
    /// * `cancel` - Set by a superseding request or closed workspace.
    /// # Returns
    /// Deduplicated compatible groups and bounded skipped-file diagnostics.
    /// # Errors
    /// Rejects invalid roots, corrupt archives, exhausted limits or cancellation.
    pub fn scan(
        path: &Path,
        limits: ImportLimits,
        cancel: &AtomicBool,
    ) -> Result<Self, Diagnostic> {
        let scanned = import::scan(path, limits, cancel)?;
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
        let catalog_groups = groups
            .iter()
            .enumerate()
            .map(|(id, captures)| Group {
                id,
                label: format!(
                    "Channel {} · {:.3} MHz zero · first {} · {} kHz · {} carriers",
                    captures[0].key.channel_id,
                    f64::from(captures[0].key.zero_frequency_hz) / 1_000_000.0,
                    captures[0].key.first_active_subcarrier_index,
                    captures[0].key.spacing_hz / 1000,
                    captures[0].key.sample_count
                ),
                key: captures[0].key.clone(),
                capture_count: captures.len(),
                first_epoch: captures[0].capture_time,
                last_epoch: captures[captures.len() - 1].capture_time,
            })
            .collect();
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
    /// Calculates full-resolution statistics for one catalog group.
    /// # Arguments
    /// * `id` - Group ID from this collection's catalog.
    /// # Returns
    /// Immutable result suitable for export and average-line analysis.
    /// # Errors
    /// Rejects unknown IDs or incompatible/invalid sample data.
    pub fn aggregate(&self, id: usize) -> Result<RxMerAggregate, Diagnostic> {
        let group = self
            .groups
            .get(id)
            .ok_or_else(|| Diagnostic::new("RxMerGroup", "Select a device/channel group."))?;
        RxMerAggregate::calculate(&group.iter().collect::<Vec<_>>())
            .map_err(|error| Diagnostic::new("RxMerAggregate", error))
    }
}
/// Builds a common chart viewport preserving all three curves and missing gaps.
/// # Arguments
/// * `result` - Full aggregate retained by Rust.
/// * `start`, `count` - Exact source carrier range.
/// # Returns
/// At most MAX_VIEW_SAMPLES joint plot positions, with source indices and counts.
/// # Errors
/// Rejects empty or out-of-bounds ranges.
pub fn viewport(
    result: &RxMerAggregate,
    start: usize,
    count: usize,
) -> Result<serde_json::Value, Diagnostic> {
    let end = start
        .checked_add(count)
        .filter(|end| *end <= result.points.len() && count > 0)
        .ok_or_else(|| {
            Diagnostic::new("InvalidRange", "Select a range inside the RxMER aggregate.")
        })?;
    let source = &result.points[start..end];
    let mut indices = Vec::new();
    let width = source.len().div_ceil(MAX_VIEW_SAMPLES / 9).max(1);
    for (bucket_index, bucket) in source.chunks(width).enumerate() {
        let offset = bucket_index * width;
        indices.extend([offset, offset + bucket.len() - 1]);
        for field in 0..3 {
            for minimum in [true, false] {
                let value = |index: usize| match field {
                    0 => bucket[index].minimum_db,
                    1 => bucket[index].average_db,
                    _ => bucket[index].maximum_db,
                };
                if let Some(index) = (0..bucket.len())
                    .filter(|index| value(*index).is_some())
                    .min_by(|first, last| {
                        let order = value(*first).unwrap().total_cmp(&value(*last).unwrap());
                        if minimum {
                            order
                        } else {
                            order.reverse()
                        }
                    })
                {
                    indices.push(offset + index);
                }
            }
        }
        if let Some(index) = bucket.iter().position(|point| point.valid_count == 0) {
            indices.push(offset + index);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    let points: Vec<_> = indices.iter().map(|index| &source[*index]).collect();
    let curve = |field: usize| {
        points.iter().enumerate().map(|(index, point)| {
        let value = if index>0 && source[indices[index-1]..=indices[index]].iter().any(|point| point.valid_count==0) {None}
            else {match field {0=>point.minimum_db,1=>point.average_db,_=>point.maximum_db}};
        serde_json::json!({"sample_index":point.sample_index,"frequency_hz":point.frequency_hz,"value":value})
    }).collect::<Vec<_>>()
    };
    Ok(
        serde_json::json!({"start":start,"count":count,"total":result.points.len(),"points":curve(1),
        "rxmer_statistics":{"minimum":curve(0),"maximum":curve(2)},"group":result.group,
        "capture_count":result.sources.len(),"first_epoch":result.sources.first().map(|source|source.capture_time),
        "last_epoch":result.sources.last().map(|source|source.capture_time)}),
    )
}
/// Exports all source epochs, valid counts and full-resolution statistics.
/// # Arguments
/// * `result` - Complete aggregate, never a decimated plot.
/// * `path` - New writable destination.
/// # Returns
/// Success after syncing the new JSON file.
/// # Errors
/// Rejects existing destinations, serialization limits and write failures.
pub fn write_json_new(result: &RxMerAggregate, path: &Path) -> Result<(), Diagnostic> {
    use std::io::Write;
    let json = pnm::common::json::to_json(result, pnm::common::json::DEFAULT_MAX_JSON_BYTES)
        .map_err(|error| Diagnostic::new("Json", error))?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Diagnostic::new("Export", error))?;
    if let Err(error) = file
        .write_all(json.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(Diagnostic::new("Export", error));
    }
    Ok(())
}
