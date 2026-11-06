/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Spectrum frequency validation and integration over original source bins.
use crate::capture::{chart::MeasurementView, Capture, Diagnostic};
use pnm::common::signal_processing::spectrum_power::{integrated_level_dbmv, SpectrumRange};
use pnm::pnm::data_type::units::HZ_PER_KHZ;
use serde::Serialize;

/// Small frequency selection/result DTO, independent of a decimated viewport.
#[derive(Debug, Serialize)]
pub struct SpectrumBand {
    pub start_hz: f64,
    pub stop_hz: f64,
    pub bandwidth_hz: f64,
    pub level_dbmv: Option<f64>,
    pub first_sample: usize,
    pub sample_count: usize,
}
impl Capture {
    /// Resolves a unit-bearing frequency range and optionally integrates power.
    /// # Arguments
    /// * `start`, `stop` - Finite ordered frequencies in `unit`.
    /// * `unit` - Exactly Hz, kHz, MHz or GHz; conversion is performed in Rust.
    /// * `calculate_power` - Integrate original source bins when true.
    /// # Returns
    /// Validated Hz endpoints, source indices and optional integrated dBmV.
    /// # Errors
    /// Rejects non-spectrum files, out-of-capture ranges, invalid units, empty
    /// ranges and missing/invalid bins. Endpoints must be inside center bounds.
    pub fn spectrum_band(
        &self,
        start: f64,
        stop: f64,
        unit: &str,
        calculate_power: bool,
    ) -> Result<SpectrumBand, Diagnostic> {
        let multiplier = match unit {
            "Hz" => 1.0,
            "kHz" => f64::from(HZ_PER_KHZ),
            "MHz" => f64::from(HZ_PER_KHZ).powi(2),
            "GHz" => f64::from(HZ_PER_KHZ).powi(3),
            _ => {
                return Err(Diagnostic::new(
                    "InvalidUnit",
                    "Select Hz, kHz, MHz or GHz.",
                ))
            }
        };
        let start_hz = start * multiplier;
        let stop_hz = stop * multiplier;
        let metadata = self.chart_metadata()?;
        let analysis = metadata.spectrum_analysis.ok_or_else(|| {
            Diagnostic::new(
                "UnsupportedView",
                "Frequency tools require a spectrum capture.",
            )
        })?;
        let samples = self.chart_points(MeasurementView::Default)?;
        let first = samples
            .first()
            .ok_or_else(|| Diagnostic::new("InvalidRange", "The spectrum is empty."))?;
        let last = samples.last().unwrap();
        if !start_hz.is_finite()
            || !stop_hz.is_finite()
            || start_hz >= stop_hz
            || start_hz < f64::from(first.frequency_hz)
            || stop_hz > f64::from(last.frequency_hz)
        {
            return Err(Diagnostic::new(
                "InvalidRange",
                "Start must precede Stop, within the capture frequency bounds.",
            ));
        }
        let first_sample =
            samples.partition_point(|point| f64::from(point.frequency_hz) < start_hz);
        let end = samples.partition_point(|point| f64::from(point.frequency_hz) <= stop_hz);
        if !calculate_power && end.saturating_sub(first_sample) < 2 {
            return Err(Diagnostic::new(
                "InvalidRange",
                "Select at least two source bin centers.",
            ));
        }
        // Chart centers are truncated to integer Hz; recover the exact grid
        // from the parsed header rather than integrating rounded display points.
        let decoded = self.decoded()?;
        let pnm::pnm::parser::pnm_file::DecodedPnm::SpectrumAnalysis(capture) = decoded else {
            return Err(Diagnostic::new(
                "UnsupportedView",
                "Not a spectrum capture.",
            ));
        };
        let first_center_hz = f64::from(capture.first_segment_center_frequency)
            - f64::from(capture.segment_frequency_span) / 2.0
            + analysis.bin_spacing_hz / 2.0;
        let level_dbmv = if calculate_power {
            let values: Vec<_> = samples.iter().map(|point| point.value).collect();
            Some(
                integrated_level_dbmv(
                    &values,
                    first_center_hz,
                    analysis.bin_spacing_hz,
                    analysis.resolution_bandwidth_hz,
                    SpectrumRange { start_hz, stop_hz },
                )
                .ok_or_else(|| {
                    Diagnostic::new(
                        "IncompleteRange",
                        "Cannot integrate invalid or missing spectrum bins.",
                    )
                })?,
            )
        } else {
            None
        };
        Ok(SpectrumBand {
            start_hz,
            stop_hz,
            bandwidth_hz: stop_hz - start_hz,
            level_dbmv,
            first_sample,
            sample_count: end - first_sample,
        })
    }
}
