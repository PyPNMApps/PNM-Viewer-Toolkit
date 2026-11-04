/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! DOCSIS spectrum fields mapped to reusable bandwidth math and derived JSON.
use crate::Diagnostic;
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
