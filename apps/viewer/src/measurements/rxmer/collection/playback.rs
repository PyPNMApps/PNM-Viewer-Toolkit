/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Read-only access to individual captures in header-epoch order.
use super::{Collection, Diagnostic, RxMerAggregate};

impl Collection {
    /// Retrieves one full-resolution capture as a single-source analysis result.
    ///
    /// # Arguments
    /// * `group_id` - Compatible group ID from this collection's catalog.
    /// * `frame_index` - Zero-based position in ascending header epoch order.
    ///
    /// # Returns
    /// An owned single-source result; the collection and other frames are unchanged.
    /// Its average values equal the original decoded readings, including gaps.
    ///
    /// # Errors
    /// Rejects unknown groups, missing frames or invalid sample data.
    pub fn frame(&self, group_id: usize, frame_index: usize) -> Result<RxMerAggregate, Diagnostic> {
        let series = self
            .groups
            .get(group_id)
            .and_then(|group| group.get(frame_index))
            .ok_or_else(|| Diagnostic::new("RxMerFrame", "Select an available RxMER capture."))?;
        RxMerAggregate::calculate(&[series]).map_err(|error| Diagnostic::new("RxMerFrame", error))
    }
}

/// Builds a bounded playback plot with the full capture's average MER.
///
/// # Arguments
/// * `result` - Single-frame result retained at full resolution in Rust.
/// * `group_id`, `frame_index`, `frame_count` - Collection navigation metadata.
/// * `start`, `count` - Exact source-carrier range requested for plotting.
///
/// # Returns
/// A bounded viewport with arithmetic mean in dB over all valid carriers in
/// this capture, independent of zoom and decimation. All unavailable returns null.
///
/// # Errors
/// Rejects invalid carrier ranges through the shared viewport contract.
pub fn playback_viewport(
    result: &RxMerAggregate,
    group_id: usize,
    frame_index: usize,
    frame_count: usize,
    start: usize,
    count: usize,
) -> Result<serde_json::Value, Diagnostic> {
    let mut view = super::viewport(result, start, count)?;
    let (sum_db, valid_count) = result
        .points
        .iter()
        .filter_map(|point| point.average_db)
        .fold((0.0, 0usize), |(sum_db, valid_count), value_db| {
            (sum_db + value_db, valid_count + 1)
        });
    let average_mer_db = if valid_count > 0 {
        Some(sum_db / valid_count as f64)
    } else {
        None
    };
    view.as_object_mut().unwrap().remove("rxmer_statistics");
    view["frame_index"] = serde_json::json!(frame_index);
    view["group_id"] = serde_json::json!(group_id);
    view["frame_count"] = serde_json::json!(frame_count);
    view["capture_epoch"] =
        serde_json::json!(result.sources.first().map(|source| source.capture_time));
    view["average_mer_db"] = serde_json::json!(average_mer_db);
    Ok(view)
}
