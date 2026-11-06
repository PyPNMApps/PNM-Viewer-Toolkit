/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded paired traces retain joint extrema/gaps; summaries/exports use full data.
use super::super::channel_estimation::IqDensity;
use crate::{Capture, Diagnostic, MAX_VIEW_SAMPLES};
pub use pnm::pnm::analysis::upstream_pre_equalization::comparison::{
    ComparisonPoint, CorrectionMode, PairMetadata, PreEqComparison,
};
use pnm::pnm::parser::{
    pnm_file::DecodedPnm, pnm_file_type::PnmFileKind, pnm_header::HasPnmHeader,
};
use serde::Serialize;
impl Capture {
    /// Reports availability by central file identity, not a presentation title.
    /// # Returns
    /// True only for a decoded current PNN6 baseline.
    pub fn supports_pre_eq_comparison(&self) -> bool {
        matches!(self.decoded(),Ok(DecodedPnm::UpstreamPreEqualization(capture)) if capture.pnm_header().descriptor().kind==PnmFileKind::UpstreamPreEqualization)
    }
    /// Validates an attached correction while preserving both original files.
    /// # Arguments
    /// * `correction` - Independently read last-update capture.
    /// # Returns
    /// Exact shared grid and file-time metadata.
    /// # Errors
    /// Reports wrong roles or field-specific compatibility failures.
    pub fn pre_eq_precheck(&self, correction: &Capture) -> Result<PairMetadata, Diagnostic> {
        let (
            DecodedPnm::UpstreamPreEqualization(original),
            DecodedPnm::UpstreamPreEqualization(correction),
        ) = (self.decoded()?, correction.decoded()?)
        else {
            return Err(Diagnostic::new(
                "PreEqPrecheck",
                "Both captures must be OFDMA Pre-EQ files.",
            ));
        };
        pnm::pnm::analysis::upstream_pre_equalization::comparison::precheck(&original, &correction)
            .map_err(|error| Diagnostic::new("PreEqPrecheck", error))
    }
    /// Predicts a complete compatible pair using an explicit interpretation.
    /// # Arguments
    /// * `correction` - Attached last-update bytes, never substituted for the baseline.
    /// * `mode` - User-selected Adjust/Set.
    /// # Returns
    /// Full-data comparison, independent of current viewport sampling.
    /// # Errors
    /// Propagates parsing and the mandatory compatibility precheck.
    pub fn pre_eq_compare(
        &self,
        correction: &Capture,
        mode: CorrectionMode,
    ) -> Result<PreEqComparison, Diagnostic> {
        let (
            DecodedPnm::UpstreamPreEqualization(original),
            DecodedPnm::UpstreamPreEqualization(correction),
        ) = (self.decoded()?, correction.decoded()?)
        else {
            return Err(Diagnostic::new(
                "PreEqPrecheck",
                "Both captures must be OFDMA Pre-EQ files.",
            ));
        };
        PreEqComparison::calculate(&original, &correction, mode)
            .map_err(|error| Diagnostic::new("PreEqComparison", error))
    }
}
#[derive(Serialize)]
pub struct ComparisonPlot<'result> {
    pub start: usize,
    pub count: usize,
    pub total: usize,
    pub pair: &'result PairMetadata,
    pub mode: CorrectionMode,
    pub summary:
        &'result pnm::pnm::analysis::upstream_pre_equalization::comparison::ComparisonSummary,
    pub assumptions: &'result [&'static str; 4],
    pub points: Vec<&'result ComparisonPoint>,
    pub magnitude_gaps: Vec<usize>,
    pub delay_gaps: Vec<usize>,
    pub original_iq: IqDensity,
    pub predicted_iq: IqDensity,
    pub delta_iq: IqDensity,
}
/// Samples both traces jointly so their extrema and intervening gaps stay aligned.
/// # Arguments
/// * `analysis` - Full immutable result whose summaries remain unchanged.
/// # Returns
/// At most MAX_VIEW_SAMPLES paired frequency points and bounded full-data I/Q maps.
/// # Panics
/// Panics for a manually constructed empty comparison; core calculations reject
/// empty pairs. Use bounded_range for a fallible query of caller-provided data.
pub fn bounded(analysis: &PreEqComparison) -> ComparisonPlot<'_> {
    bounded_range(analysis, 0, analysis.points.len()).expect("Validated nonempty analysis")
}
/// Queries a bounded exact source range without changing full-data summaries/exports.
/// # Arguments
/// * `analysis` - Full immutable comparison.
/// * `start`, `count` - Original carrier indices, independent of display sampling.
/// # Returns
/// Joint paired traces and full-data I/Q maps.
/// # Errors
/// Rejects empty or out-of-bounds ranges without changing the current view.
pub fn bounded_range(
    analysis: &PreEqComparison,
    start: usize,
    count: usize,
) -> Result<ComparisonPlot<'_>, Diagnostic> {
    let end = start
        .checked_add(count)
        .filter(|end| *end <= analysis.points.len() && count > 0)
        .ok_or_else(|| Diagnostic::new("InvalidRange", "Select a nonempty comparison range."))?;
    let source = &analysis.points[start..end];
    let fields: [fn(&ComparisonPoint) -> Option<f64>; 6] = [
        |point| point.original_aligned_db,
        |point| point.predicted_aligned_db,
        |point| point.aligned_delta_db,
        |point| point.original_delay_s,
        |point| point.predicted_delay_s,
        |point| point.delta_delay_s,
    ];
    let width = source.len().div_ceil(MAX_VIEW_SAMPLES / 20).max(1);
    let points = if source.len() <= MAX_VIEW_SAMPLES {
        source.iter().collect()
    } else {
        let mut selected = Vec::new();
        for bucket in source.chunks(width) {
            let mut indices = vec![0, bucket.len() - 1];
            for field in fields {
                for reverse in [false, true] {
                    if let Some((index, _)) = bucket
                        .iter()
                        .enumerate()
                        .filter_map(|(index, point)| field(point).map(|value| (index, value)))
                        .min_by(|first, last| {
                            if reverse {
                                last.1.total_cmp(&first.1)
                            } else {
                                first.1.total_cmp(&last.1)
                            }
                        })
                    {
                        indices.push(index);
                    }
                }
                if let Some(index) = bucket.iter().position(|point| field(point).is_none()) {
                    indices.push(index);
                }
            }
            indices.sort_unstable();
            indices.dedup();
            selected.extend(indices.into_iter().map(|index| &bucket[index]));
        }
        selected
    };
    let mut magnitude_gaps = Vec::new();
    let mut delay_gaps = Vec::new();
    for pair in points.windows(2) {
        let skipped = &analysis.points[pair[0].sample_index..=pair[1].sample_index];
        if skipped
            .iter()
            .any(|point| point.original_db.is_none() || point.predicted_db.is_none())
        {
            magnitude_gaps.push(pair[1].sample_index);
        }
        if skipped
            .iter()
            .any(|point| point.original_delay_s.is_none() || point.predicted_delay_s.is_none())
        {
            delay_gaps.push(pair[1].sample_index);
        }
    }
    Ok(ComparisonPlot {
        start,
        count,
        total: analysis.points.len(),
        pair: &analysis.pair,
        mode: analysis.mode,
        summary: &analysis.summary,
        assumptions: &analysis.assumptions,
        points,
        magnitude_gaps,
        delay_gaps,
        original_iq: IqDensity::from_samples(
            &analysis
                .points
                .iter()
                .map(|point| point.original_iq)
                .collect::<Vec<_>>(),
        ),
        predicted_iq: IqDensity::from_samples(
            &analysis
                .points
                .iter()
                .map(|point| point.predicted_iq)
                .collect::<Vec<_>>(),
        ),
        delta_iq: IqDensity::from_samples(
            &analysis
                .points
                .iter()
                .map(|point| point.delta_iq)
                .collect::<Vec<_>>(),
        ),
    })
}
/// Writes a full-resolution result to a new local JSON destination.
/// # Arguments
/// * `analysis` - Full comparison, never the sampled plot.
/// * `path` - New file chosen by the user.
/// # Returns
/// Success after a bounded JSON write and filesystem sync.
/// # Errors
/// Rejects existing outputs; deletes only a newly created partial file on failure.
pub fn write_json_new(
    analysis: &PreEqComparison,
    path: &std::path::Path,
) -> Result<(), Diagnostic> {
    use std::io::Write;
    let json = analysis
        .to_json(pnm::common::json::DEFAULT_MAX_JSON_BYTES)
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
        let _cleanup = std::fs::remove_file(path);
        return Err(Diagnostic::new("Export", error));
    }
    Ok(())
}
