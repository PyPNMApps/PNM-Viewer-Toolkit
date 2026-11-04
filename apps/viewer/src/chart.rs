/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Measurement-neutral chart data; binary interpretation stays in the core parsers.
use crate::{spectrum_analysis::SpectrumAnalysis, Capture, Diagnostic, MAX_VIEW_SAMPLES};
use pnm::pnm::{
    data_type::{units::HZ_PER_KHZ, FrequencyHz},
    parser::pnm_file::DecodedPnm,
};
use serde::Serialize;

/// Measurement tab; the default is magnitude for channel estimation.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum MeasurementView {
    #[default]
    Default = 0,
    GroupDelay = 1,
    IqScatter = 2,
}

/// Display label and formatted value for one metadata pill.
#[derive(Serialize)]
pub struct Chip {
    pub title: String,
    pub value: String,
}
impl Chip {
    /// Formats one small metadata chip.
    /// # Arguments
    /// * `title` - Human-readable field name.
    /// * `value` - Display value, including units where applicable.
    /// # Returns
    /// An owned label/value pair for UI serialization.
    pub(crate) fn new(title: &str, value: impl ToString) -> Self {
        Self {
            title: title.into(),
            value: value.to_string(),
        }
    }
}
/// Small chart configuration; vertical bounds use the declared measurement unit.
#[derive(Serialize)]
pub struct ChartMetadata {
    pub views: Vec<&'static str>,
    pub view_index: u32,
    pub title: &'static str,
    pub unit: &'static str,
    pub axis_label: &'static str,
    pub sample_count: usize,
    pub minimum: f64,
    pub maximum: f64,
    pub chips: Vec<Chip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spectrum_analysis: Option<SpectrumAnalysis>,
}
/// Original sample position and value; None preserves missing readings.
#[derive(Clone, Serialize)]
pub struct ChartPoint {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub value: Option<f64>,
}
/// Selected source range and bounded display points for that range.
#[derive(Serialize)]
pub struct ChartViewport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq_density: Option<crate::channel_estimation::IqDensity>,
    pub start: usize,
    pub count: usize,
    pub total: usize,
    pub points: Vec<ChartPoint>,
}
impl Capture {
    /// Builds measurement-specific labels, units and full-capture vertical bounds.
    ///
    /// # Returns
    /// Small metadata and inline chips for the default measurement view.
    /// # Errors
    /// Propagates invalid captures or unsupported frequency grids.
    pub fn chart_metadata(&self) -> Result<ChartMetadata, Diagnostic> {
        self.chart_metadata_for(MeasurementView::Default)
    }
    /// Builds metadata for a selected tab, rejecting tabs unavailable for a capture.
    /// # Arguments
    /// * `view` - Default measurement, group delay or I/Q scatter.
    /// # Returns
    /// Bounded configuration and tab names, without full sample arrays.
    /// # Errors
    /// Propagates parser/grid errors or an unsupported tab.
    pub fn chart_metadata_for(&self, view: MeasurementView) -> Result<ChartMetadata, Diagnostic> {
        let decoded = self.decoded()?;
        validate_view(&decoded, view)?;
        match decoded {
            DecodedPnm::ChannelEstimate(capture) => {
                crate::channel_estimation::metadata(&capture, view)
            }
            DecodedPnm::RxMer(_) => {
                let metadata = self.metadata()?;
                Ok(ChartMetadata {
                    views: vec![],
                    view_index: 0,
                    spectrum_analysis: None,
                    title: "Downstream RxMER",
                    unit: "dB",
                    axis_label: "RxMER",
                    sample_count: metadata.sample_count,
                    minimum: 0.0,
                    maximum: 64.0,
                    chips: vec![
                        Chip::new("Channel", metadata.channel_id),
                        Chip::new("MAC", metadata.mac_address),
                        Chip::new("Subcarriers", metadata.sample_count),
                        Chip::new("Spacing", format!("{} kHz", metadata.spacing_khz)),
                    ],
                })
            }
            DecodedPnm::SpectrumAnalysis(spectrum) => {
                let points = self.chart_points(MeasurementView::Default)?;
                let analysis = SpectrumAnalysis::from_capture(&spectrum)?;
                let minimum = points
                    .iter()
                    .filter_map(|point| point.value)
                    .fold(0.0_f64, f64::min);
                let maximum = points
                    .iter()
                    .filter_map(|point| point.value)
                    .fold(0.0_f64, f64::max);
                let mac = spectrum
                    .mac_address
                    .iter()
                    .map(|octet| format!("{octet:02x}"))
                    .collect::<Vec<_>>()
                    .join(":");
                Ok(ChartMetadata {
                    views: vec![],
                    view_index: 0,
                    title: "Spectrum Analyzer",
                    unit: "dBmV",
                    axis_label: "Amplitude",
                    sample_count: points.len(),
                    minimum: ((minimum - 5.0) / 10.0).floor() * 10.0,
                    maximum: ((maximum + 5.0) / 10.0).ceil() * 10.0,
                    chips: vec![
                        Chip::new("MAC", mac),
                        Chip::new(
                            "RBW",
                            format!(
                                "{:.3} kHz",
                                analysis.resolution_bandwidth_hz / f64::from(HZ_PER_KHZ)
                            ),
                        ),
                    ],
                    spectrum_analysis: Some(analysis),
                })
            }
            _ => Err(Diagnostic::new(
                "UnsupportedViewer",
                "No chart for this measurement.",
            )),
        }
    }
    fn chart_points(&self, view: MeasurementView) -> Result<Vec<ChartPoint>, Diagnostic> {
        let decoded = self.decoded()?;
        validate_view(&decoded, view)?;
        match decoded {
            DecodedPnm::ChannelEstimate(capture) => {
                let analysis =
                    crate::channel_estimation::ChannelEstimationAnalysis::from_capture(&capture)?;
                let values = if view == MeasurementView::GroupDelay {
                    analysis.group_delay_us()
                } else {
                    analysis.magnitude_db
                };
                Ok(analysis
                    .frequencies_hz
                    .into_iter()
                    .zip(values)
                    .enumerate()
                    .map(|(sample_index, (frequency_hz, value))| ChartPoint {
                        sample_index,
                        frequency_hz,
                        value,
                    })
                    .collect())
            }
            DecodedPnm::RxMer(capture) => Ok(capture
                .frequencies_hz()?
                .zip(capture.values_db())
                .enumerate()
                .map(|(sample_index, (frequency_hz, value))| ChartPoint {
                    sample_index,
                    frequency_hz,
                    value,
                })
                .collect()),
            DecodedPnm::SpectrumAnalysis(capture) => {
                let span = u64::from(capture.segment_frequency_span);
                let bins = u64::from(capture.num_bins_per_segment);
                let first = u64::from(capture.first_segment_center_frequency);
                let last = u64::from(capture.last_segment_center_frequency);
                if span == 0
                    || span < bins
                    || last < first
                    || (last - first) % span != 0
                    || first * 2 < span
                {
                    return Err(Diagnostic::new(
                        "InvalidFrequencyGrid",
                        "Spectrum segment frequencies or bin spacing are invalid.",
                    ));
                }
                let configured_bins = ((last - first) / span + 1) * bins;
                // OSSI D.2.4.1.5: contiguous segment centers advance by the span.
                // Use PyPNM's bin-center convention; round each position to Hz,
                // not the spacing, so rounding does not accumulate across bins.
                let origin = (first * 2 - span) * bins;
                capture
                    .amplitudes_db()
                    .enumerate()
                    .map(|(sample_index, value)| {
                        let index = sample_index as u64;
                        if index >= configured_bins {
                            return Err(Diagnostic::new(
                                "InvalidFrequencyGrid",
                                "Spectrum contains more bins than its configured span.",
                            ));
                        }
                        let frequency = (origin + (2 * index + 1) * span) / (2 * bins);
                        let frequency_hz = FrequencyHz::try_from(frequency).map_err(|_| {
                            Diagnostic::new(
                                "InvalidFrequencyGrid",
                                "Spectrum frequency exceeds the supported Hz range.",
                            )
                        })?;
                        Ok(ChartPoint {
                            sample_index,
                            frequency_hz,
                            value: Some(value),
                        })
                    })
                    .collect()
            }
            _ => Err(Diagnostic::new(
                "UnsupportedViewer",
                "No chart for this measurement.",
            )),
        }
    }
    /// Returns bounded full-range overviews or exact zoom detail for the default chart.
    ///
    /// # Arguments
    /// * `start` - Zero-based first sample.
    /// * `count` - Number of source samples in the selected range.
    /// # Returns
    /// At most 1024 points with source indices, extrema and conservative gaps.
    /// # Errors
    /// Rejects invalid ranges, captures or frequency grids.
    pub fn chart_range(&self, start: usize, count: usize) -> Result<ChartViewport, Diagnostic> {
        self.chart_range_for(MeasurementView::Default, start, count)
    }
    /// Queries a bounded view for a selected measurement tab.
    /// # Arguments
    /// * `view` - Frequency plot or full-capture I/Q density overview.
    /// * `start` - First source sample; I/Q requires zero.
    /// * `count` - Source sample count; I/Q requires the full capture.
    /// # Returns
    /// At most 1024 plot points or occupied I/Q cells.
    /// # Errors
    /// Rejects unsupported tabs, invalid ranges and frequency grids.
    pub fn chart_range_for(
        &self,
        view: MeasurementView,
        start: usize,
        count: usize,
    ) -> Result<ChartViewport, Diagnostic> {
        if view == MeasurementView::IqScatter {
            let decoded = self.decoded()?;
            validate_view(&decoded, view)?;
            if let DecodedPnm::ChannelEstimate(capture) = decoded {
                let samples: Vec<_> = capture.iq().collect();
                if start != 0 || count != samples.len() {
                    return Err(Diagnostic::new(
                        "InvalidRange",
                        "I/Q overview requires the full capture.",
                    ));
                }
                return Ok(ChartViewport {
                    start,
                    count,
                    total: samples.len(),
                    points: vec![],
                    iq_density: Some(crate::channel_estimation::IqDensity::from_samples(&samples)),
                });
            }
        }
        let samples = self.chart_points(view)?;
        let total = samples.len();
        let end = start
            .checked_add(count)
            .filter(|end| *end <= total)
            .ok_or_else(|| {
                Diagnostic::new("InvalidRange", "Zoom range is outside this capture.")
            })?;
        let points = overview(&samples[start..end]);
        Ok(ChartViewport {
            iq_density: None,
            start,
            count,
            total,
            points,
        })
    }
}
fn overview(samples: &[ChartPoint]) -> Vec<ChartPoint> {
    if samples.len() <= MAX_VIEW_SAMPLES {
        return samples.to_vec();
    }
    let bucket_count = (MAX_VIEW_SAMPLES - 2) / 2;
    let bucket_size = samples.len().div_ceil(bucket_count);
    let mut points = Vec::with_capacity(MAX_VIEW_SAMPLES);
    points.push(samples[0].clone());
    for bucket in samples.chunks(bucket_size) {
        if let Some(missing) = bucket.iter().find(|point| point.value.is_none()) {
            points.push(missing.clone());
        } else {
            let minimum = bucket
                .iter()
                .min_by(|left, right| left.value.partial_cmp(&right.value).unwrap())
                .unwrap();
            let maximum = bucket
                .iter()
                .max_by(|left, right| left.value.partial_cmp(&right.value).unwrap())
                .unwrap();
            let mut extrema = [minimum, maximum];
            extrema.sort_by_key(|point| point.sample_index);
            points.extend(extrema.into_iter().cloned());
        }
    }
    points.push(samples[samples.len() - 1].clone());
    points.dedup_by_key(|point| point.sample_index);
    points
}

fn validate_view(decoded: &DecodedPnm<'_>, view: MeasurementView) -> Result<(), Diagnostic> {
    if view != MeasurementView::Default && !matches!(decoded, DecodedPnm::ChannelEstimate(_)) {
        return Err(Diagnostic::new(
            "UnsupportedView",
            "This capture does not support that view.",
        ));
    }
    Ok(())
}
