/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded capture ownership and exact RxMER viewport data.
use pnm::pnm::data_type::{ChannelId, FrequencyHz, RxMerDb, SubcarrierSpacingKhz};
use pnm::pnm::parser::{
    cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
    pnm_file::{DecodedPnm, PnmFile},
};
use serde::Serialize;
use std::{fs::File, io::Read, path::Path};

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
#[derive(Serialize)]
pub struct Point {
    pub frequency_hz: FrequencyHz,
    pub rxmer_db: Option<RxMerDb>,
}
#[derive(Serialize)]
pub struct Viewport {
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
    /// A validated RxMER capture owning its original bytes.
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
    /// Rejects oversized, invalid or non-RxMER captures.
    pub fn from_bytes(bytes: Vec<u8>, filename: String) -> Result<Self, Diagnostic> {
        if bytes.len() > MAX_INPUT_BYTES {
            return Err(Diagnostic::new("LimitExceeded", "File exceeds 16 MiB."));
        }
        let capture = Self { bytes, filename };
        capture.parsed()?;
        Ok(capture)
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
        let points = parsed
            .frequencies_hz()?
            .zip(parsed.values_db())
            .skip(start)
            .take(MAX_VIEW_SAMPLES)
            .map(|(frequency_hz, rxmer_db)| Point {
                frequency_hz,
                rxmer_db,
            })
            .collect();
        Ok(Viewport {
            start,
            total,
            points,
        })
    }
    /// Generates full-resolution core JSON for a file export, never a viewport.
    /// # Returns
    /// The parser's existing PyPNM-aligned JSON with null missing values.
    /// # Errors
    /// Reports parsing or the 16 MiB JSON serialization limit.
    pub fn json(&self) -> Result<String, Diagnostic> {
        self.parsed()?
            .to_json(pnm::common::json::DEFAULT_MAX_JSON_BYTES)
            .map_err(|error| Diagnostic::new("Json", error))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
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
        assert!(Capture::from_bytes(pnm_fixtures::HISTOGRAM.to_vec(), "histogram".into()).is_err());
        assert!(Capture::from_bytes(vec![0; MAX_INPUT_BYTES + 1], "large".into()).is_err());
    }
}
