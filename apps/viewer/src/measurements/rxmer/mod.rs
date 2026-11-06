/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded native presentation of the shared full-resolution RxMER analysis.
use crate::capture::{Capture, Diagnostic, MAX_VIEW_SAMPLES};
use pnm::{common::json::DEFAULT_MAX_JSON_BYTES, pnm::parser::pnm_file::DecodedPnm};
pub use pnm::{
    common::signal_processing::bit_loading::BitLoadingModel,
    pnm::analysis::rxmer::{RxMerAnalysisRequest, RxMerBitLoadingAnalysis},
};
use serde::Serialize;
use std::path::Path;

impl Capture {
    /// Computes an immutable analysis snapshot for an exact source-sample range.
    /// # Arguments
    /// * `start`, `count` - Original sample range, never displayed point indices.
    /// * `request` - Selected policy, margin and cap; source range replaces Hz bounds.
    /// # Returns
    /// Full-resolution shared core analysis, independent of current display decimation.
    /// # Errors
    /// Rejects non-RxMER, empty/out-of-bounds ranges, invalid grid or analysis options.
    pub fn rxmer_bit_loading(
        &self,
        start: usize,
        count: usize,
        mut request: RxMerAnalysisRequest,
    ) -> Result<RxMerBitLoadingAnalysis, Diagnostic> {
        let DecodedPnm::RxMer(capture) = self.decoded()? else {
            return Err(Diagnostic::new(
                "UnsupportedView",
                "Bit loading requires an RxMER capture.",
            ));
        };
        let end = start
            .checked_add(count)
            .filter(|end| *end <= capture.raw_values().len() && count > 0)
            .ok_or_else(|| {
                Diagnostic::new(
                    "InvalidRange",
                    "Choose a nonempty range inside the RxMER capture.",
                )
            })?;
        let mut frequencies = capture.frequencies_hz()?;
        request.start_frequency_hz = frequencies.nth(start);
        request.stop_frequency_hz = if count == 1 {
            request.start_frequency_hz
        } else {
            frequencies.nth(end - start - 2)
        };
        RxMerBitLoadingAnalysis::calculate(&capture, request)
            .map_err(|error| Diagnostic::new("RxMerAnalysis", error))
    }
}
/// Bounded presentation plus explicit breaks across omitted invalid samples.
#[derive(Serialize)]
pub struct PlotAnalysis {
    #[serde(flatten)]
    pub analysis: RxMerBitLoadingAnalysis,
    pub plot_gaps: Vec<usize>,
    pub ideal_plot_gaps: Vec<usize>,
    pub modulation_caps: Vec<serde_json::Value>,
}
/// Retains source order while bounding plots and preserving low-MER bucket extrema.
/// # Arguments
/// * `analysis` - Full result to consume; its summaries remain unchanged.
/// # Returns
/// Same result with at most MAX_VIEW_SAMPLES representative source readings.
/// Missing readings remain explicit; each bucket retains missing/below-model samples.
pub fn bounded(mut analysis: RxMerBitLoadingAnalysis) -> PlotAnalysis {
    let original = &analysis.subcarriers;
    const REPRESENTATIVES: usize = 6;
    let bucket_width = analysis
        .subcarriers
        .len()
        .div_ceil(MAX_VIEW_SAMPLES / REPRESENTATIVES)
        .max(1);
    let mut points = Vec::new();
    for bucket in analysis.subcarriers.chunks(bucket_width) {
        let mut indices = vec![0, bucket.len() - 1];
        for choose_minimum in [true, false] {
            let selected = bucket
                .iter()
                .enumerate()
                .filter_map(|(index, point)| point.rxmer_db.map(|mer| (index, mer)))
                .min_by(|first, last| {
                    if choose_minimum {
                        first.1.total_cmp(&last.1)
                    } else {
                        last.1.total_cmp(&first.1)
                    }
                });
            if let Some((index, _)) = selected {
                indices.push(index);
            }
        }
        if let Some(index) = bucket.iter().position(|point| point.rxmer_db.is_none()) {
            indices.push(index);
        }
        if let Some(index) = bucket
            .iter()
            .position(|point| point.rxmer_db.is_some() && point.bit_loading.is_none())
        {
            indices.push(index);
        }
        indices.sort_unstable();
        indices.dedup();
        points.extend(indices.into_iter().map(|index| bucket[index].clone()));
    }
    if original.len() <= MAX_VIEW_SAMPLES {
        points = original.clone();
    }
    let mut plot_gaps = Vec::new();
    let mut ideal_plot_gaps = Vec::new();
    for pair in points.windows(2) {
        let skipped = &original[pair[0].sample_index - analysis.selection.first_sample_index
            ..=pair[1].sample_index - analysis.selection.first_sample_index];
        if skipped.iter().any(|point| point.bit_loading.is_none()) {
            plot_gaps.push(pair[1].sample_index);
        }
        if skipped
            .iter()
            .any(|point| point.ideal_bit_loading.is_none())
        {
            ideal_plot_gaps.push(pair[1].sample_index);
        }
    }
    analysis.subcarriers = points;
    let modulation_caps = (2..=u8::MAX).filter(|bits| pnm::common::signal_processing::bit_loading::is_data_bit_loading(*bits))
        .map(|bits| serde_json::json!({"bits":bits,"label":pnm::pnm::data_type::modulation::ModulationAssignment::from_code(bits).display_label})).collect();
    PlotAnalysis {
        analysis,
        plot_gaps,
        ideal_plot_gaps,
        modulation_caps,
    }
}
/// Writes a full-resolution analysis snapshot without replacing an existing file.
/// # Arguments
/// * `analysis` - Shared core result; not the bounded plot DTO.
/// * `path` - New user-selected destination.
/// # Returns
/// Success after serializing within the common limit and syncing the file.
/// # Errors
/// Reports JSON, existing-file or write failure; removes only a newly created partial file.
pub fn write_json_new(analysis: &RxMerBitLoadingAnalysis, path: &Path) -> Result<(), Diagnostic> {
    use std::io::Write;
    let json = analysis
        .to_json(DEFAULT_MAX_JSON_BYTES)
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
