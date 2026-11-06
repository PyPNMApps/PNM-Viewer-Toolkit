/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! DOCSIS spectrum fields mapped to reusable bandwidth math and derived JSON.
use crate::capture::Diagnostic;
use pnm::{
    common::signal_processing::spectrum::{
        resolution_bandwidth_hz, BandwidthHz, NoiseBandwidthFactor,
    },
    pnm::parser::cm_spectrum_analysis::CmSpectrumAnalysis,
};
use serde::Serialize;

// CM-OSSI Table 57: ENBW is encoded in hundredths of bin spacing.
const ENBW_UNITS_PER_FACTOR: f64 = 100.0;

/// Derived spectrum values, distinct from the unchanged decoded fields.
#[derive(Debug, Serialize)]
pub struct SpectrumAnalysis {
    pub bin_spacing_hz: BandwidthHz,
    pub equivalent_noise_bandwidth_factor: NoiseBandwidthFactor,
    pub resolution_bandwidth_hz: BandwidthHz,
}
impl SpectrumAnalysis {
    /// Converts encoded DOCSIS bandwidth fields and calculates fractional-Hz RBW.
    ///
    /// # Arguments
    /// * `capture` - Parsed spectrum header, including span, bins and ENBW.
    /// # Returns
    /// Numeric values for chart metadata and the spectrum_analysis export section.
    /// # Errors
    /// Rejects zero or otherwise invalid bandwidth inputs instead of guessing.
    pub fn from_capture(capture: &CmSpectrumAnalysis<'_>) -> Result<Self, Diagnostic> {
        let bin_spacing_hz =
            f64::from(capture.segment_frequency_span) / f64::from(capture.num_bins_per_segment);
        let equivalent_noise_bandwidth_factor =
            f64::from(capture.equivalent_noise_bandwidth) / ENBW_UNITS_PER_FACTOR;
        let resolution_bandwidth_hz =
            resolution_bandwidth_hz(bin_spacing_hz, equivalent_noise_bandwidth_factor).ok_or_else(
                || {
                    Diagnostic::new(
                "InvalidBandwidth",
                "Spectrum bin spacing and equivalent noise bandwidth must be positive and finite.",
            )
                },
            )?;
        Ok(Self {
            bin_spacing_hz,
            equivalent_noise_bandwidth_factor,
            resolution_bandwidth_hz,
        })
    }
}

#[derive(Serialize)]
pub(crate) struct SpectrumExport<'capture, 'input> {
    #[serde(flatten)]
    pub capture: &'capture CmSpectrumAnalysis<'input>,
    pub spectrum_analysis: SpectrumAnalysis,
}

/// Bounded averaged trace sharing the source frequency axis and sample indices.
#[derive(Serialize)]
pub struct WindowAverageView {
    pub window_points: pnm::common::signal_processing::moving_average::WindowPointCount,
    pub points: Vec<crate::capture::chart::ChartPoint>,
}

/// Smooths the full spectrum before selecting and bounding a zoomed display.
/// # Arguments
/// * `samples` - Complete, ordered source spectrum points in dBmV.
/// * `window_points` - Positive centered window length; not a pixel count.
/// * `start`, `end` - Validated source range with an exclusive end.
/// # Returns
/// Original and averaged traces with at most MAX_VIEW_SAMPLES combined points.
/// The average uses linear power, converted back to dBmV; it is not integrated
/// channel power or a change to the instrument resolution bandwidth.
/// # Errors
/// Rejects invalid window lengths or ranges without modifying the capture.
pub(crate) fn window_average_view(
    samples: &[crate::capture::chart::ChartPoint],
    window_points: pnm::common::signal_processing::moving_average::WindowPointCount,
    start: usize,
    end: usize,
) -> Result<crate::capture::chart::ChartViewport, Diagnostic> {
    use crate::capture::{
        chart::{overview_with_limit, ChartViewport},
        MAX_VIEW_SAMPLES,
    };
    use pnm::common::signal_processing::spectrum::window_average_dbmv;
    let values: Vec<_> = samples.iter().map(|point| point.value).collect();
    let averages = window_average_dbmv(&values, window_points).ok_or_else(|| {
        Diagnostic::new("InvalidWindow", "Window size must be a positive integer.")
    })?;
    if start > end || end > samples.len() {
        return Err(Diagnostic::new(
            "InvalidRange",
            "Window range is outside this capture.",
        ));
    }
    let averaged: Vec<_> = samples
        .iter()
        .zip(averages)
        .map(|(point, value)| {
            let mut point = point.clone();
            point.value = value;
            point
        })
        .collect();
    Ok(ChartViewport {
        spectrum_average: Some(WindowAverageView {
            window_points,
            points: overview_with_limit(&averaged[start..end], MAX_VIEW_SAMPLES / 2),
        }),
        constellation: None,
        modulation_profiles: None,
        fec_summary: None,
        histogram: None,
        iq_density: None,
        start,
        count: end - start,
        total: samples.len(),
        points: overview_with_limit(&samples[start..end], MAX_VIEW_SAMPLES / 2),
    })
}

pub mod range;
