/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Measurement-neutral chart data; binary interpretation stays in the core parsers.
use crate::{
    capture::{Capture, Diagnostic, MAX_VIEW_SAMPLES},
    measurements::spectrum_analysis::SpectrumAnalysis,
};
use pnm::common::signal_processing::moving_average::WindowPointCount;
use pnm::pnm::{
    data_type::{units::HZ_PER_KHZ, CaptureTime, FrequencyHz},
    parser::pnm_file::DecodedPnm,
};
use serde::Serialize;

/// Measurement tab; the default is magnitude for channel estimation.
#[derive(Clone, Default, PartialEq, Eq)]
pub enum MeasurementView {
    #[default]
    Default,
    GroupDelay,
    IqScatter,
    FecProfile(usize),
    SpectrumAverage {
        window_points: WindowPointCount,
    },
    ModulationProfile {
        indices: Option<Vec<usize>>,
        carrier_types: Option<Vec<crate::measurements::modulation_profile::CarrierType>>,
        metric: crate::measurements::modulation_profile::ModulationMetric,
    },
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulation_profile: Option<crate::measurements::modulation_profile::ModulationMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec_summary: Option<crate::measurements::fec_summary::FecMetadata>,
    pub histogram: bool,
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
    pub spectrum_average: Option<crate::measurements::spectrum_analysis::WindowAverageView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constellation: Option<crate::measurements::constellation::ConstellationViewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modulation_profiles: Option<crate::measurements::modulation_profile::ModulationViewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec_summary: Option<crate::measurements::fec_summary::FecViewport>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub histogram: Option<crate::measurements::histogram::HistogramView>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iq_density: Option<crate::measurements::channel_estimation::IqDensity>,
    pub start: usize,
    pub count: usize,
    pub total: usize,
    pub points: Vec<ChartPoint>,
}
impl Capture {
    /// Selects FEC records by inclusive UTC epoch seconds in Rust.
    /// # Arguments
    /// * `view` - Default or explicitly selected FEC profile.
    /// * `first`, `last` - Inclusive epoch-second bounds; reversed bounds are accepted.
    /// # Returns
    /// Bounded points and exact window statistics; empty selections stay empty.
    /// # Errors
    /// Rejects non-FEC captures, invalid profiles and decoder failures.
    pub fn fec_time_range(
        &self,
        view: MeasurementView,
        first: CaptureTime,
        last: CaptureTime,
    ) -> Result<ChartViewport, Diagnostic> {
        if let DecodedPnm::FecSummary(capture) = self.decoded()? {
            return crate::measurements::fec_summary::time_range(&capture, view, first, last);
        }
        Err(Diagnostic::new(
            "UnsupportedView",
            "UTC selection requires FEC Summary.",
        ))
    }
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
        validate_view(&decoded, &view)?;
        match decoded {
            DecodedPnm::ConstellationDisplay(capture) => {
                Ok(crate::measurements::constellation::metadata(&capture))
            }
            DecodedPnm::ModulationProfile(capture) => {
                crate::measurements::modulation_profile::metadata(&capture, view)
            }
            DecodedPnm::FecSummary(capture) => {
                crate::measurements::fec_summary::metadata(&capture, view)
            }
            DecodedPnm::Histogram(capture) => {
                Ok(crate::measurements::histogram::metadata(&capture))
            }
            DecodedPnm::UpstreamPreEqualization(capture) => {
                crate::measurements::upstream_pre_equalization::metadata(&capture, view)
            }
            DecodedPnm::ChannelEstimate(capture) => {
                crate::measurements::channel_estimation::metadata(&capture, view)
            }
            DecodedPnm::RxMer(_) => {
                let metadata = self.metadata()?;
                Ok(ChartMetadata {
                    modulation_profile: None,
                    fec_summary: None,
                    histogram: false,
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
                    modulation_profile: None,
                    fec_summary: None,
                    histogram: false,
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
    /// Builds original source points for the requested measurement view.
    /// # Arguments
    /// * `view` - Supported measurement/metric selection for this capture.
    /// # Returns
    /// Full-resolution internal points; desktop callers must use bounded DTOs.
    /// # Errors
    /// Rejects unsupported views, invalid source grids and decoding failures.
    pub(crate) fn chart_points(
        &self,
        view: MeasurementView,
    ) -> Result<Vec<ChartPoint>, Diagnostic> {
        let decoded = self.decoded()?;
        validate_view(&decoded, &view)?;
        match decoded {
            DecodedPnm::UpstreamPreEqualization(capture) => {
                let analysis = pnm::pnm::analysis::upstream_pre_equalization::PreEqualizationAnalysis::from_capture(&capture)?;
                let values = if view == MeasurementView::GroupDelay {
                    analysis
                        .group_delay
                        .into_iter()
                        .map(|point| {
                            point.delay_s.map(|delay| {
                                delay * pnm::pnm::data_type::units::MICROSECONDS_PER_SECOND
                            })
                        })
                        .collect()
                } else {
                    analysis.magnitude_db
                };
                Ok(response_points(analysis.frequencies_hz, values))
            }
            DecodedPnm::ChannelEstimate(capture) => {
                let analysis =
                    crate::measurements::channel_estimation::ChannelEstimationAnalysis::from_capture(&capture)?;
                let values = if view == MeasurementView::GroupDelay {
                    analysis.group_delay_us()
                } else {
                    analysis.magnitude_db
                };
                Ok(response_points(analysis.frequencies_hz, values))
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
        if let DecodedPnm::ConstellationDisplay(capture) = self.decoded()? {
            if view != MeasurementView::Default {
                return Err(Diagnostic::new(
                    "UnsupportedView",
                    "Constellation Display has no such tab.",
                ));
            }
            if start != 0 || count != capture.iq().count() {
                return Err(Diagnostic::new(
                    "InvalidRange",
                    "Use I/Q bounds to zoom a constellation.",
                ));
            }
            return crate::measurements::constellation::viewport(&capture, None);
        }
        if let DecodedPnm::ModulationProfile(capture) = self.decoded()? {
            return crate::measurements::modulation_profile::viewport(&capture, view, start, count);
        }
        if let DecodedPnm::FecSummary(capture) = self.decoded()? {
            return crate::measurements::fec_summary::viewport(&capture, view, start, count);
        }
        if let DecodedPnm::Histogram(capture) = self.decoded()? {
            if view != MeasurementView::Default {
                return Err(Diagnostic::new(
                    "UnsupportedView",
                    "Histogram has no such tab.",
                ));
            }
            return crate::measurements::histogram::viewport(&capture, start, count);
        }
        if view == MeasurementView::IqScatter {
            let decoded = self.decoded()?;
            validate_view(&decoded, &view)?;
            let samples: Vec<_> = match decoded {
                DecodedPnm::ChannelEstimate(capture) => capture.iq().collect(),
                DecodedPnm::UpstreamPreEqualization(capture) => capture.iq().collect(),
                _ => {
                    return Err(Diagnostic::new(
                        "UnsupportedView",
                        "This measurement has no I/Q density view.",
                    ))
                }
            };
            if start != 0 || count != samples.len() {
                return Err(Diagnostic::new(
                    "InvalidRange",
                    "I/Q overview requires the full capture.",
                ));
            }
            return Ok(ChartViewport {
                spectrum_average: None,
                constellation: None,
                modulation_profiles: None,
                fec_summary: None,
                histogram: None,
                start,
                count,
                total: samples.len(),
                points: vec![],
                iq_density: Some(
                    crate::measurements::channel_estimation::IqDensity::from_samples(&samples),
                ),
            });
        }
        let average_window = match view {
            MeasurementView::SpectrumAverage { window_points } => Some(window_points),
            _ => None,
        };
        let samples = self.chart_points(view)?;
        let total = samples.len();
        let end = start
            .checked_add(count)
            .filter(|end| *end <= total)
            .ok_or_else(|| {
                Diagnostic::new("InvalidRange", "Zoom range is outside this capture.")
            })?;
        if let Some(window_points) = average_window {
            return crate::measurements::spectrum_analysis::window_average_view(
                &samples,
                window_points,
                start,
                end,
            );
        }
        let points = overview(&samples[start..end]);
        Ok(ChartViewport {
            spectrum_average: None,
            constellation: None,
            modulation_profiles: None,
            fec_summary: None,
            histogram: None,
            iq_density: None,
            start,
            count,
            total,
            points,
        })
    }
}
fn overview(samples: &[ChartPoint]) -> Vec<ChartPoint> {
    overview_with_limit(samples, MAX_VIEW_SAMPLES)
}

/// Bounds display points while retaining endpoints, extrema and missing buckets.
/// # Arguments
/// * `samples` - Ordered source points in one selected range.
/// * `limit` - Display budget, at least four for endpoints and bucket extrema.
/// # Returns
/// At most limit points in source order; empty input remains empty.
/// # Panics
/// May panic if limit is below four and the input requires decimation.
pub(crate) fn overview_with_limit(samples: &[ChartPoint], limit: usize) -> Vec<ChartPoint> {
    if samples.len() <= limit {
        return samples.to_vec();
    }
    let bucket_count = (limit - 2) / 2;
    let bucket_size = samples.len().div_ceil(bucket_count);
    let mut points = Vec::with_capacity(limit);
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

fn validate_view(decoded: &DecodedPnm<'_>, view: &MeasurementView) -> Result<(), Diagnostic> {
    let supported = match decoded {
        DecodedPnm::SpectrumAnalysis(_) => matches!(
            view,
            MeasurementView::Default | MeasurementView::SpectrumAverage { .. }
        ),
        DecodedPnm::FecSummary(_) => matches!(
            view,
            MeasurementView::Default | MeasurementView::FecProfile(_)
        ),
        DecodedPnm::ChannelEstimate(_) | DecodedPnm::UpstreamPreEqualization(_) => matches!(
            view,
            MeasurementView::Default | MeasurementView::GroupDelay | MeasurementView::IqScatter
        ),
        DecodedPnm::ModulationProfile(_) => matches!(
            view,
            MeasurementView::Default | MeasurementView::ModulationProfile { .. }
        ),
        _ => *view == MeasurementView::Default,
    };
    if !supported {
        return Err(Diagnostic::new(
            "UnsupportedView",
            "This measurement has no such view.",
        ));
    }
    Ok(())
}

fn response_points(frequencies: Vec<FrequencyHz>, values: Vec<Option<f64>>) -> Vec<ChartPoint> {
    frequencies
        .into_iter()
        .zip(values)
        .enumerate()
        .map(|(sample_index, (frequency_hz, value))| ChartPoint {
            sample_index,
            frequency_hz,
            value,
        })
        .collect()
}
