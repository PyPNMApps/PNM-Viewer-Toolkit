/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded hit-count bars; retain encoded bin order without inferred normalization.
use crate::capture::{
    chart::{ChartMetadata, ChartViewport, Chip},
    Diagnostic, MAX_VIEW_SAMPLES,
};
use pnm::pnm::parser::cm_ds_hist::CmDsHist;
use serde::Serialize;

/// Sum of unsigned hit counts; the decoder budget keeps this below 2^53.
pub type HitCountTotal = u64;
/// Contiguous original bins represented by one display bar.
#[derive(Serialize)]
pub struct HistogramBar {
    pub first_bin: usize,
    pub bin_count: usize,
    pub hit_count: HitCountTotal,
}
/// Bounded histogram presentation, independent of frequency-domain charts.
#[derive(Serialize)]
pub struct HistogramView {
    pub bars: Vec<HistogramBar>,
    pub maximum: HitCountTotal,
    pub total_hits: HitCountTotal,
    pub grouped: bool,
}

/// Builds histogram labels without interpreting the encoded symmetry flag.
/// # Arguments
/// * `capture` - Validated downstream histogram.
/// # Returns
/// MAC chip and hit-count/bin-index chart configuration.
pub(crate) fn metadata(capture: &CmDsHist<'_>) -> ChartMetadata {
    let mac = capture
        .mac_address
        .iter()
        .map(|octet| format!("{octet:02x}"))
        .collect::<Vec<_>>()
        .join(":");
    ChartMetadata {
        modulation_profile: None,
        fec_summary: None,
        histogram: true,
        views: vec![],
        view_index: 0,
        title: "Downstream Histogram",
        unit: "hits",
        axis_label: "Hit count",
        sample_count: capture.hit_counts().count(),
        minimum: 0.0,
        maximum: 1.0,
        chips: vec![Chip::new("MAC", mac)],
        spectrum_analysis: None,
    }
}

/// Sums contiguous bins into at most 1024 bars without dropping hits.
/// # Arguments
/// * `capture` - Validated histogram; dwell counts remain in raw JSON only.
/// * `start` - First zero-based bin.
/// * `count` - Number of bins selected for display.
/// # Returns
/// Exact bars for small ranges; summed buckets with explicit source ranges otherwise.
/// # Errors
/// Rejects ranges outside the source or overflowing range arithmetic.
pub(crate) fn viewport(
    capture: &CmDsHist<'_>,
    start: usize,
    count: usize,
) -> Result<ChartViewport, Diagnostic> {
    let counts: Vec<_> = capture.hit_counts().collect();
    let end = start
        .checked_add(count)
        .filter(|end| *end <= counts.len())
        .ok_or_else(|| Diagnostic::new("InvalidRange", "Bin range is outside this histogram."))?;
    let bucket_size = count.div_ceil(MAX_VIEW_SAMPLES).max(1);
    let bars: Vec<_> = counts[start..end]
        .chunks(bucket_size)
        .enumerate()
        .map(|(index, bucket)| HistogramBar {
            first_bin: start + index * bucket_size,
            bin_count: bucket.len(),
            hit_count: bucket.iter().copied().map(u64::from).sum(),
        })
        .collect();
    let maximum = bars
        .iter()
        .map(|bar| bar.hit_count)
        .max()
        .unwrap_or(0)
        .max(1);
    let total_hits = bars.iter().map(|bar| bar.hit_count).sum();
    Ok(ChartViewport {
        constellation: None,
        modulation_profiles: None,
        fec_summary: None,
        start,
        count,
        total: counts.len(),
        points: vec![],
        iq_density: None,
        histogram: Some(HistogramView {
            bars,
            maximum,
            total_hits,
            grouped: bucket_size > 1,
        }),
    })
}
