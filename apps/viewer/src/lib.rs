/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded capture ownership and desktop chart data.
pub mod channel_estimation;
pub mod chart;
pub mod constellation;
pub mod fec_summary;
pub mod histogram;
pub mod modulation_profile;
pub mod spectrum_analysis;
pub mod upstream_pre_equalization;
use pnm::pnm::data_type::{ChannelId, FrequencyHz, RxMerDb, SubcarrierSpacingKhz};
use pnm::pnm::parser::{
    cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
    pnm_file::{DecodedPnm, PnmFile},
};
use pnm::pnm::parser::{pnm_file_type::PnmFileKind, pnm_header::HasPnmHeader};
use serde::Serialize;
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub const MAX_INPUT_BYTES: usize = 16 * 1024 * 1024;
pub const MAX_VIEW_SAMPLES: usize = 1024;

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
    pub offset: Option<usize>,
}
impl Diagnostic {
    /// Creates an application diagnostic without a binary field location.
    /// # Arguments
    /// * `code` - Stable error category.
    /// * `message` - User-facing explanation.
    /// # Returns
    /// An error suitable for IPC serialization.
    pub fn new(code: &str, message: impl ToString) -> Self {
        Self {
            code: code.into(),
            message: message.to_string(),
            field: None,
            offset: None,
        }
    }
}
impl From<pnm::diagnostic::ParseError> for Diagnostic {
    /// Preserves a parser diagnostic's field and original byte offset.
    fn from(error: pnm::diagnostic::ParseError) -> Self {
        Self {
            code: format!("{:?}", error.code),
            message: error.to_string(),
            field: Some(error.field.into()),
            offset: Some(error.offset),
        }
    }
}
#[derive(Serialize)]
pub struct Metadata {
    pub filename: String,
    pub channel_id: ChannelId,
    pub mac_address: String,
    pub sample_count: usize,
    pub spacing_khz: SubcarrierSpacingKhz,
}
#[derive(Clone, Serialize)]
pub struct Point {
    pub sample_index: usize,
    pub frequency_hz: FrequencyHz,
    pub rxmer_db: Option<RxMerDb>,
}
#[derive(Serialize)]
pub struct Viewport {
    pub count: usize,
    pub start: usize,
    pub total: usize,
    pub points: Vec<Point>,
}
pub struct Capture {
    bytes: Vec<u8>,
    filename: String,
}
impl Capture {
    /// Reads one selected file with a bounded allocation, preserving source bytes.
    /// # Arguments
    /// * `path` - Path chosen by the native file dialog.
    /// # Returns
    /// A validated supported capture owning its original bytes.
    /// # Errors
    /// Reports I/O, byte limits, unsupported types or parser diagnostics.
    pub fn read(path: &Path) -> Result<Self, Diagnostic> {
        let metadata = std::fs::metadata(path).map_err(|error| Diagnostic::new("Io", error))?;
        if !metadata.is_file() {
            return Err(Diagnostic::new("Io", "Select a regular capture file."));
        }
        let file = File::open(path).map_err(|error| Diagnostic::new("Io", error))?;
        let mut bytes = Vec::new();
        file.take((MAX_INPUT_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|error| Diagnostic::new("Io", error))?;
        Self::from_bytes(
            bytes,
            path.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        )
    }
    /// Validates an owned byte buffer through the central file dispatcher.
    /// # Arguments
    /// * `bytes` - Original file bytes.
    /// * `filename` - Display name without directory information.
    /// # Returns
    /// A capture holding one copy of the bytes.
    /// # Errors
    /// Rejects oversized, invalid or unsupported captures.
    pub fn from_bytes(bytes: Vec<u8>, filename: String) -> Result<Self, Diagnostic> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(Diagnostic::new("LimitExceeded", "File exceeds 16 MiB."));
        }
        let capture = Self { bytes, filename };
        capture.decoded()?;
        Ok(capture)
    }
    /// Exports complete decoded JSON to a newly created destination.
    ///
    /// # Arguments
    /// * `path` - Destination; existing files are never replaced.
    /// # Returns
    /// Completion after writing and syncing all JSON bytes.
    /// # Errors
    /// Reports serialization, existing destination and I/O failures. A partial
    /// file created by this operation is removed after a write failure.
    pub fn write_json_new(&self, path: &Path) -> Result<(), Diagnostic> {
        let json = self.json()?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| {
                Diagnostic::new("Export", format!("Choose a new destination. {error}"))
            })?;
        if let Err(error) = output
            .write_all(json.as_bytes())
            .and_then(|()| output.sync_all())
        {
            drop(output);
            let _cleanup = std::fs::remove_file(path);
            return Err(Diagnostic::new("Export", error));
        }
        Ok(())
    }
    fn decoded(&self) -> Result<DecodedPnm<'_>, Diagnostic> {
        let decoded = PnmFile::from_bytes(&self.bytes).decode(ParseLimits::default())?;
        match decoded {
            DecodedPnm::UpstreamPreEqualization(ref capture) if capture.pnm_header().descriptor().kind == PnmFileKind::UpstreamPreEqualization => Ok(decoded),
            DecodedPnm::UpstreamPreEqualization(_) => Err(Diagnostic::new("UnsupportedViewer", "Last-update upstream pre-equalization is not yet supported in the viewer. Use pnm-decode for this file.")),
            DecodedPnm::ConstellationDisplay(_) | DecodedPnm::RxMer(_) | DecodedPnm::SpectrumAnalysis(_) | DecodedPnm::ChannelEstimate(_) | DecodedPnm::Histogram(_) | DecodedPnm::FecSummary(_) | DecodedPnm::ModulationProfile(_) => Ok(decoded),
            _ => Err(Diagnostic::new("UnsupportedViewer", "This viewer supports RxMER, Spectrum Analyzer, DS Channel Estimation, Histogram, FEC Summary, Modulation Profile, Constellation Display and current Upstream Pre-EQ captures. Other formats remain available in pnm-decode.")),
        }
    }
    fn parsed(&self) -> Result<CmDsOfdmRxMer<'_>, Diagnostic> {
        match PnmFile::from_bytes(&self.bytes).decode(ParseLimits::default())? {
            DecodedPnm::RxMer(capture) => Ok(capture),
            _ => Err(Diagnostic::new("UnsupportedViewer", "This first viewer supports RxMER captures. Other formats remain available in pnm-decode.")),
        }
    }
    /// Returns small presentation metadata without sample arrays.
    /// # Returns
    /// Filename, channel, MAC and sample-grid information.
    /// # Errors
    /// Propagates parser validation failures.
    pub fn metadata(&self) -> Result<Metadata, Diagnostic> {
        let parsed = self.parsed()?;
        Ok(Metadata {
            filename: self.filename.clone(),
            channel_id: parsed.channel_id,
            mac_address: parsed
                .mac_address
                .iter()
                .map(|octet| format!("{octet:02x}"))
                .collect::<Vec<_>>()
                .join(":"),
            sample_count: parsed.raw_values().len(),
            spacing_khz: parsed.subcarrier_spacing_khz,
        })
    }
    /// Returns an exact, bounded sample window, preserving null gaps.
    /// # Arguments
    /// * `start` - Zero-based sample index, at most the sample count.
    /// # Returns
    /// Up to 1024 consecutive frequency/dB points; no decimation is applied.
    /// # Errors
    /// Rejects out-of-range starts and propagates parsing errors.
    pub fn viewport(&self, start: usize) -> Result<Viewport, Diagnostic> {
        let parsed = self.parsed()?;
        let total = parsed.raw_values().len();
        if start > total {
            return Err(Diagnostic::new(
                "InvalidRange",
                "Sample window is outside this capture.",
            ));
        }
        let points: Vec<Point> = parsed
            .frequencies_hz()?
            .zip(parsed.values_db())
            .skip(start)
            .take(MAX_VIEW_SAMPLES)
            .enumerate()
            .map(|(index, (frequency_hz, rxmer_db))| Point {
                sample_index: start + index,
                frequency_hz,
                rxmer_db,
            })
            .collect();
        Ok(Viewport {
            count: points.len(),
            start,
            total,
            points,
        })
    }
    /// Returns a bounded overview of any sample range for interactive zoom.
    ///
    /// # Arguments
    /// * `start` - First sample index.
    /// * `count` - Number of samples in the requested range.
    /// # Returns
    /// At most 1024 points with original sample indices. Small ranges are exact.
    /// Larger ranges retain ordered bucket extrema and both range endpoints.
    /// A bucket containing missing data is represented by a gap, conservatively
    /// hiding its valid values until zoomed rather than bridging missing data.
    /// # Errors
    /// Rejects ranges outside the capture and propagates parser errors.
    pub fn range_viewport(&self, start: usize, count: usize) -> Result<Viewport, Diagnostic> {
        self.parsed()?;
        let view = self.chart_range(start, count)?;
        Ok(Viewport {
            start: view.start,
            count: view.count,
            total: view.total,
            points: view
                .points
                .into_iter()
                .map(|point| Point {
                    sample_index: point.sample_index,
                    frequency_hz: point.frequency_hz,
                    rxmer_db: point.value,
                })
                .collect(),
        })
    }

    /// Generates full-resolution core JSON for a file export, never a viewport.
    /// # Returns
    /// The parser's JSON with null missing values, plus derived spectrum bandwidth
    /// or channel-estimation analysis for those measurement types.
    /// # Errors
    /// Reports parsing or the 16 MiB JSON serialization limit.
    pub fn json(&self) -> Result<String, Diagnostic> {
        let decoded = self.decoded()?;
        let result = match &decoded {
            DecodedPnm::ChannelEstimate(capture) => {
                let export = channel_estimation::ChannelEstimationExport {
                    capture,
                    channel_estimation_analysis:
                        channel_estimation::ChannelEstimationAnalysis::from_capture(capture)?,
                };
                pnm::common::json::to_json(&export, pnm::common::json::DEFAULT_MAX_JSON_BYTES)
            }
            DecodedPnm::SpectrumAnalysis(capture) => {
                let export = spectrum_analysis::SpectrumExport {
                    capture,
                    spectrum_analysis: spectrum_analysis::SpectrumAnalysis::from_capture(capture)?,
                };
                pnm::common::json::to_json(&export, pnm::common::json::DEFAULT_MAX_JSON_BYTES)
            }
            _ => decoded.to_json(pnm::common::json::DEFAULT_MAX_JSON_BYTES),
        };
        result.map_err(|error| Diagnostic::new("Json", error))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_range_overview_and_zoom_are_bounded() {
        let capture =
            Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
        let overview = capture.range_viewport(0, 7480).unwrap();
        assert_eq!(overview.count, 7480);
        assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
        assert_eq!(overview.points.first().unwrap().frequency_hz, 640_000_000);
        assert_eq!(overview.points.last().unwrap().frequency_hz, 826_975_000);
        assert!(overview
            .points
            .windows(2)
            .all(|pair| pair[0].sample_index < pair[1].sample_index));
        let detail = capture.range_viewport(7000, 480).unwrap();
        assert_eq!(detail.points.len(), 480);
        assert_eq!(detail.points[0].sample_index, 7000);
        assert_eq!(detail.points[479].sample_index, 7479);
        assert!(capture.range_viewport(7480, 0).unwrap().points.is_empty());
        assert!(capture.range_viewport(7480, 1).is_err());
        assert!(capture.range_viewport(usize::MAX, 2).is_err());
    }
    #[test]
    fn overview_preserves_extrema_and_does_not_bridge_gaps() {
        let mut bytes = pnm_fixtures::RXMER.to_vec();
        let payload_start = bytes.len() - 7480;
        bytes[payload_start..].fill(160);
        bytes[payload_start + 105] = 0;
        bytes[payload_start + 106] = 240;
        bytes[payload_start + 205] = 255;
        let capture = Capture::from_bytes(bytes, "synthetic.bin".into()).unwrap();
        let overview = capture.range_viewport(0, 7480).unwrap();
        assert!(overview
            .points
            .iter()
            .any(|point| point.sample_index == 105 && point.rxmer_db == Some(0.0)));
        assert!(overview
            .points
            .iter()
            .any(|point| point.sample_index == 106 && point.rxmer_db == Some(60.0)));
        assert!(overview
            .points
            .iter()
            .any(|point| point.sample_index == 205 && point.rxmer_db.is_none()));
        let detail = capture.range_viewport(200, 10).unwrap();
        assert_eq!(detail.points.len(), 10);
        assert_eq!(detail.points[5].rxmer_db, None);
        assert_eq!(detail.points[6].rxmer_db, Some(40.0));
    }
    #[test]
    fn export_preserves_existing_files_and_original_capture() {
        let folder = tempfile::tempdir().unwrap();
        let input = folder.path().join("capture.bin");
        std::fs::write(&input, pnm_fixtures::RXMER).unwrap();
        let capture = Capture::read(&input).unwrap();
        assert!(capture.write_json_new(&input).is_err());
        assert_eq!(std::fs::read(&input).unwrap(), pnm_fixtures::RXMER);
        let output = folder.path().join("capture.json");
        capture.write_json_new(&output).unwrap();
        assert_eq!(
            std::fs::read_to_string(&output).unwrap(),
            capture.json().unwrap()
        );
        assert!(capture.write_json_new(&output).is_err());
    }
    #[test]
    fn fixture_viewport_and_export_are_consistent() {
        let capture =
            Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
        let view = capture.viewport(0).unwrap();
        assert_eq!(view.total, 7480);
        assert_eq!(view.points.len(), 1024);
        assert_eq!(view.points[0].frequency_hz, 640_000_000);
        assert_eq!(view.points[0].rxmer_db, Some(42.75));
        assert_eq!(capture.viewport(7168).unwrap().points.len(), 312);
        assert!(capture.viewport(7481).is_err());
        let json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
        assert_eq!(json["values"].as_array().unwrap().len(), 7480);
        assert_eq!(json["values"][0], 42.75);
    }
    #[test]
    fn missing_samples_remain_null() {
        let mut bytes = pnm_fixtures::RXMER.to_vec();
        *bytes.last_mut().unwrap() = 0xff;
        let capture = Capture::from_bytes(bytes, "synthetic-missing.bin".into()).unwrap();
        assert_eq!(capture.viewport(7479).unwrap().points[0].rxmer_db, None);
        let dto = serde_json::to_value(capture.viewport(7479).unwrap()).unwrap();
        assert_eq!(dto["start"], 7479);
        assert_eq!(dto["total"], 7480);
        assert_eq!(dto["points"][0]["frequency_hz"], 826_975_000);
        assert!(dto["points"][0]["rxmer_db"].is_null());
    }
    #[test]
    fn reject_invalid_and_other_measurements() {
        assert!(Capture::from_bytes(vec![0; 4], "bad".into()).is_err());
        assert!(Capture::from_bytes(
            pnm_fixtures::UPSTREAM_PRE_EQ_LAST.to_vec(),
            "last-update".into()
        )
        .is_err());
        assert!(Capture::from_bytes(vec![0; MAX_INPUT_BYTES + 1], "large".into()).is_err());
    }
}
