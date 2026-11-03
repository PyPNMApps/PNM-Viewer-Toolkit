/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Upstream coefficients: I27 Tables 77/78 / I13 Tables 110/111.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{finish, payload, require_decoder},
        serialization::{serialize_header, serialize_mac, serialize_spacing},
        values::{ComplexValues, COMPLEX_BYTES},
    },
};
use crate::common::reader::Reader;
use crate::pnm::lib::fixed_point_decoder::QFormat;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmUsOfdmaPreEq<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    #[serde(serialize_with = "serialize_mac")]
    pub cmts_mac_address: MacAddress,
    #[serde(rename = "subcarrier_zero_frequency")]
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    #[serde(rename = "subcarrier_spacing", serialize_with = "serialize_spacing")]
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    pub data_length: DataLengthBytes,
    values: ComplexValues<'input>,
}
impl<'input> CmUsOfdmaPreEq<'input> {
    /// Parses a complete capture, borrowing its original bytes.
    ///
    /// # Arguments
    /// * `bytes` - Complete input retained by the caller.
    /// * `limits` - Application sample budget.
    /// # Returns
    /// Typed metadata and borrowed measurement data.
    /// # Errors
    /// Reports unsupported identity/version, truncation, lengths or resource limits.
    pub fn parse(bytes: &'input [u8], limits: ParseLimits) -> Result<Self, ParseError> {
        Self::from_header(PnmHeader::from_bytes(bytes)?, limits)
    }
    /// Decodes the second tier using the centrally identified header.
    ///
    /// # Arguments
    /// * `header` - Header retaining original payload and offsets.
    /// * `limits` - Application sample budget.
    /// # Returns
    /// A capture with unchanged raw data retained by reference.
    /// # Errors
    /// Reports wrong identity, truncated fields, inconsistent lengths or limits.
    #[tracing::instrument(level = "debug", skip_all, fields(file_type = header.descriptor().tag), err)]
    pub fn from_header(header: PnmHeader<'input>, limits: ParseLimits) -> Result<Self, ParseError> {
        require_decoder(&header, PnmDecoderKind::UpstreamPreEqualization)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let cmts_mac_address = reader.read_bytes("cmts_mac_address")?;
        let subcarrier_zero_frequency_hz = reader.read_u32_be("zero_frequency")?;
        let first_active_subcarrier_index = reader.read_u16_be("first_subcarrier")?;
        let subcarrier_spacing_khz = reader.read_u8("spacing_khz")?;
        let bytes = payload(&mut reader, COMPLEX_BYTES, limits)?;
        finish(&reader)?;
        let data_length = bytes.len() as DataLengthBytes;
        let format = match header.descriptor().kind {
            super::pnm_file_type::PnmFileKind::UpstreamPreEqualizationLastUpdate => QFormat::S1_14,
            _ => QFormat::S2_13,
        };
        let values = ComplexValues {
            bytes,
            format,
            excluded_pair: false,
        };
        Ok(Self {
            header,
            channel_id,
            mac_address,
            cmts_mac_address,
            subcarrier_zero_frequency_hz,
            first_active_subcarrier_index,
            subcarrier_spacing_khz,
            data_length,
            values,
        })
    }
    /// Serializes the capture directly into bounded pretty JSON.
    ///
    /// # Arguments
    /// * `max_bytes` - Maximum encoded output size.
    /// # Returns
    /// Human-readable JSON without a duplicate capture model.
    /// # Errors
    /// Propagates serialization and output-budget failures.
    pub fn to_json(&self, max_bytes: usize) -> serde_json::Result<String> {
        crate::common::json::to_json(self, max_bytes)
    }
}
impl HasPnmHeader for CmUsOfdmaPreEq<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
impl<'input> CmUsOfdmaPreEq<'input> {
    /// Borrows the unchanged encoded complex samples.
    ///
    /// # Returns
    /// Original I/Q bytes, with no second copy of the file.
    pub fn raw_values(&self) -> &'input [u8] {
        self.values.bytes
    }
    /// Lazily decodes fixed-point pairs.
    ///
    /// # Returns
    /// Real/imaginary pairs; channel-estimate exclusion pairs become None.
    pub fn values(&self) -> impl Iterator<Item = Option<ComplexValue>> + '_ {
        self.values.values()
    }
}
