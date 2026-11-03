/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! RxMER version 1.0, based on I27 Table 68 and I13 Table 101 text extracts.
use super::{
    pnm_header::{HasPnmHeader, PnmFormatVersion, PnmHeader},
    reader::Reader,
};
use crate::diagnostic::{ErrorCode, ParseError};
use crate::pnm::data_type::units::{HZ_PER_KHZ, QUARTER_DB_PER_DB};
use crate::pnm::data_type::{
    CaptureTime, ChannelId, DataLengthBytes, FileVersionComponent, FrequencyHz, MacAddress,
    PnmFileType, RxMerDb, SampleCount, SubcarrierIdx, SubcarrierSpacingKhz,
};

const RXMER_FILE_TYPE: PnmFileType = [0x50, 0x4e, 0x4e, 0x04];
const RXMER_FORMAT_VERSION: PnmFormatVersion = PnmFormatVersion { major: 1, minor: 0 };
const RXMER_ZERO_FREQUENCY_OFFSET: usize = 17;
const RXMER_VERSION_OFFSET: usize = 4;
const RXMER_LENGTH_OFFSET: usize = 24;
const RXMER_HEADER_BYTES: usize = 28;
const RXMER_UNAVAILABLE: u8 = 0xff;
const DEFAULT_MAX_SAMPLES: SampleCount = 65_536;

/// Application resource policy, not a DOCSIS format limit.
#[derive(Debug, Clone, Copy)]
pub struct ParseLimits {
    /// Maximum payload samples accepted by this caller.
    pub max_samples: SampleCount,
}
impl Default for ParseLimits {
    /// Builds the default application sample budget.
    ///
    /// # Returns
    /// A limit of 65,536 samples; this is not a DOCSIS limit.
    fn default() -> Self {
        Self {
            max_samples: DEFAULT_MAX_SAMPLES,
        }
    }
}

/// Decoded RxMER metadata with samples borrowed from the caller.
/// No statistics, I/O or allocation of sample arrays occurs here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CmDsOfdmRxMer<'a> {
    header: PnmHeader,
    pub channel_id: ChannelId,
    pub mac_address: MacAddress,
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    raw_values: &'a [u8],
}
impl HasPnmHeader for CmDsOfdmRxMer<'_> {
    /// Borrows the composed common header.
    ///
    /// # Returns
    /// A shared header reference valid for the capture borrow.
    fn pnm_header(&self) -> &PnmHeader {
        &self.header
    }
}
impl<'a> CmDsOfdmRxMer<'a> {
    /// Decodes an RxMER 1.0 file without copying its samples.
    ///
    /// # Arguments
    /// * `bytes` - Complete capture retained for the returned capture lifetime.
    /// * `limits` - Caller-selected sample-count budget.
    ///
    /// # Returns
    /// A capture borrowing bytes; absent measurements remain distinguishable.
    ///
    /// # Errors
    /// Reports field/offset errors for truncation, unsupported type/version,
    /// exceeded limits, frequency overflow or a payload length mismatch, including trailing bytes.
    pub fn parse(bytes: &'a [u8], limits: ParseLimits) -> Result<Self, ParseError> {
        let error = |code, offset, field| ParseError {
            code,
            offset,
            field,
        };
        let mut reader = Reader::new(bytes);
        let file_type: PnmFileType = reader.take("file_type")?;
        if file_type != RXMER_FILE_TYPE {
            return Err(error(ErrorCode::UnsupportedType, 0, "file_type"));
        }
        let major_version: FileVersionComponent = reader.take::<1>("major_version")?[0];
        let minor_version: FileVersionComponent = reader.take::<1>("minor_version")?[0];
        let format_version = PnmFormatVersion {
            major: major_version,
            minor: minor_version,
        };
        // Vendor minor versions are preserved as unsupported until verified.
        if format_version != RXMER_FORMAT_VERSION {
            return Err(error(
                ErrorCode::UnsupportedVersion,
                RXMER_VERSION_OFFSET,
                "format_version",
            ));
        }
        let capture_time = Some(CaptureTime::from_be_bytes(reader.take("capture_time")?));
        let header = PnmHeader {
            file_type,
            format_version,
            capture_time,
        };
        let channel_id: ChannelId = reader.take::<1>("channel_id")?[0];
        let mac_address: MacAddress = reader.take("mac_address")?;
        let subcarrier_zero_frequency_hz =
            FrequencyHz::from_be_bytes(reader.take("zero_frequency")?);
        let first_active_subcarrier_index =
            SubcarrierIdx::from_be_bytes(reader.take("first_subcarrier")?);
        let subcarrier_spacing_khz: SubcarrierSpacingKhz = reader.take::<1>("spacing_khz")?[0];
        let data_length_bytes = DataLengthBytes::from_be_bytes(reader.take("data_length")?);
        let data_length_bytes = usize::try_from(data_length_bytes)
            .map_err(|_| error(ErrorCode::LimitExceeded, RXMER_LENGTH_OFFSET, "data_length"))?;
        if data_length_bytes > limits.max_samples {
            return Err(error(
                ErrorCode::LimitExceeded,
                RXMER_LENGTH_OFFSET,
                "data_length",
            ));
        }
        let raw_values = &bytes[RXMER_HEADER_BYTES..]; // Reader established the complete fixed header.
        if raw_values.len() != data_length_bytes {
            return Err(error(
                ErrorCode::LengthMismatch,
                RXMER_LENGTH_OFFSET,
                "data_length",
            ));
        }
        let capture = Self {
            header,
            channel_id,
            mac_address,
            subcarrier_zero_frequency_hz,
            first_active_subcarrier_index,
            subcarrier_spacing_khz,
            raw_values,
        };
        // Range validation is eager; sample iteration is not needed during parsing.
        let _frequencies = capture.frequencies_hz()?;
        Ok(capture)
    }
    /// Borrows the unchanged encoded RxMER samples.
    ///
    /// # Returns
    /// The original sample slice, including unavailable-value sentinels.
    pub fn raw_values(&self) -> &'a [u8] {
        self.raw_values
    }
    /// Lazily converts quarter-dB samples without allocating.
    ///
    /// # Returns
    /// An iterator of dB values; None denotes an unavailable sample.
    pub fn values_db(&self) -> impl Iterator<Item = Option<RxMerDb>> + '_ {
        self.raw_values.iter().map(|&value| {
            if value == RXMER_UNAVAILABLE {
                None
            } else {
                Some(f64::from(value) / QUARTER_DB_PER_DB)
            }
        })
    }
    /// Validates the frequency range and lazily yields sample frequencies.
    ///
    /// # Returns
    /// A no-allocation Hz iterator, empty for an empty payload. Zero spacing is
    /// preserved. Revalidation protects callers after public metadata changes.
    /// No PHY-range validation is implied.
    ///
    /// # Errors
    /// Returns ArithmeticOverflow when a sample frequency cannot fit FrequencyHz.
    pub fn frequencies_hz(&self) -> Result<impl Iterator<Item = FrequencyHz>, ParseError> {
        let overflow = || ParseError {
            code: ErrorCode::ArithmeticOverflow,
            offset: RXMER_ZERO_FREQUENCY_OFFSET,
            field: "sample_frequency_hz",
        };
        let sample_count = u32::try_from(self.raw_values.len()).map_err(|_| overflow())?;
        let spacing_hz = FrequencyHz::from(self.subcarrier_spacing_khz) * HZ_PER_KHZ;
        let start_hz = if sample_count == 0 {
            self.subcarrier_zero_frequency_hz
        } else {
            FrequencyHz::from(self.first_active_subcarrier_index)
                .checked_mul(spacing_hz)
                .and_then(|offset_hz| self.subcarrier_zero_frequency_hz.checked_add(offset_hz))
                .ok_or_else(overflow)?
        };
        if sample_count > 0 {
            (sample_count - 1)
                .checked_mul(spacing_hz)
                .and_then(|offset_hz| start_hz.checked_add(offset_hz))
                .ok_or_else(overflow)?;
        }
        // Nonnegative, monotonic offsets are bounded by the checked final sample.
        Ok((0..sample_count).map(move |index| start_hz + index * spacing_hz))
    }
}
