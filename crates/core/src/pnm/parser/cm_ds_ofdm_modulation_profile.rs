/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Modulation profiles: I27 Table 80 and D.2.1.7 / I13 Table 113.
use super::{
    parse_limits::ParseLimits,
    pnm_file_type::PnmDecoderKind,
    pnm_header::{HasPnmHeader, PnmHeader},
    support::{
        payload::{budget, error, finish, require_decoder},
        serialization::{serialize_header, serialize_mac, serialize_spacing},
    },
};
use crate::common::reader::Reader;
use crate::{diagnostic::ParseError, pnm::data_type::*};
use serde::Serialize;

/// Decoded measurement retaining its common header and original payload.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CmDsOfdmModulationProfile<'input> {
    #[serde(rename = "pnm_header", serialize_with = "serialize_header")]
    header: PnmHeader<'input>,
    pub channel_id: ChannelId,
    #[serde(serialize_with = "serialize_mac")]
    pub mac_address: MacAddress,
    pub num_profiles: ProfileCount,
    #[serde(rename = "subcarrier_zero_frequency")]
    pub subcarrier_zero_frequency_hz: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    #[serde(rename = "subcarrier_spacing", serialize_with = "serialize_spacing")]
    pub subcarrier_spacing_khz: SubcarrierSpacingKhz,
    pub profile_data_length_bytes: DataLengthBytes,
    pub profiles: Vec<ModulationProfile>,
}
impl<'input> CmDsOfdmModulationProfile<'input> {
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
        require_decoder(&header, PnmDecoderKind::ModulationProfile)?;
        let mut reader = Reader::with_origin(header.pnm_data(), header.payload_offset())?;
        let channel_id = reader.read_u8("channel_id")?;
        let mac_address = reader.read_bytes("mac_address")?;
        let num_profiles = reader.read_u8("num_profiles")?;
        let subcarrier_zero_frequency_hz = reader.read_u32_be("zero_frequency")?;
        let first_active_subcarrier_index = reader.read_u16_be("first_subcarrier")?;
        let subcarrier_spacing_khz = reader.read_u8("spacing_khz")?;
        let length_offset = reader.position();
        let profile_data_length_bytes = reader.read_u32_be("profile_data_length_bytes")?;
        let length = usize::try_from(profile_data_length_bytes).map_err(|_| {
            error(
                crate::diagnostic::ErrorCode::LimitExceeded,
                length_offset,
                "profile_data_length_bytes",
            )
        })?;
        let origin = reader.position();
        let bytes = reader.read_slice(length, "profiles")?;
        finish(&reader)?;
        let mut profiles_reader = Reader::with_origin(bytes, origin)?;
        let mut remaining = limits.max_samples;
        budget(usize::from(num_profiles), &mut remaining, origin)?;
        let mut profiles = Vec::new();
        for _ in 0..num_profiles {
            let profile_id = profiles_reader.read_u8("profile_id")?;
            let length = usize::from(profiles_reader.read_u16_be("profile_length")?);
            let origin = profiles_reader.position();
            let mut scheme_reader =
                Reader::with_origin(profiles_reader.read_slice(length, "schemes")?, origin)?;
            let mut schemes = Vec::new();
            while !scheme_reader.remaining().is_empty() {
                let offset = scheme_reader.position();
                budget(1, &mut remaining, offset)?;
                let scheme_type = scheme_reader.read_u8("schema_type")?;
                let scheme = match scheme_type {
                    RANGE_SCHEME => ModulationScheme::Range {
                        modulation_order: scheme_reader.read_u8("modulation_order")?,
                        num_subcarriers: scheme_reader.read_u16_be("num_subcarriers")?,
                    },
                    SKIP_SCHEME => ModulationScheme::Skip {
                        main_modulation_order: scheme_reader.read_u8("main_modulation_order")?,
                        skip_modulation_order: scheme_reader.read_u8("skip_modulation_order")?,
                        num_subcarriers: scheme_reader.read_u16_be("num_subcarriers")?,
                    },
                    _ => {
                        return Err(error(
                            crate::diagnostic::ErrorCode::UnsupportedLayout,
                            offset,
                            "schema_type",
                        ))
                    }
                };
                schemes.push(scheme);
            }
            profiles.push(ModulationProfile {
                profile_id,
                schemes,
            });
        }
        finish(&profiles_reader)?;
        Ok(Self {
            header,
            channel_id,
            mac_address,
            num_profiles,
            subcarrier_zero_frequency_hz,
            first_active_subcarrier_index,
            subcarrier_spacing_khz,
            profile_data_length_bytes,
            profiles,
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
impl HasPnmHeader for CmDsOfdmModulationProfile<'_> {
    /// Borrows the shared first-tier header.
    ///
    /// # Returns
    /// Header metadata and the original payload view.
    fn pnm_header(&self) -> &PnmHeader<'_> {
        &self.header
    }
}
const RANGE_SCHEME: u8 = 0;
const SKIP_SCHEME: u8 = 1;
/// One profile retaining compact assignments rather than expanded subcarrier arrays.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModulationProfile {
    pub profile_id: ProfileId,
    pub schemes: Vec<ModulationScheme>,
}
/// Encoded assignments; numeric modulation values preserve unknown/reserved values.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "schema_type")]
pub enum ModulationScheme {
    #[serde(rename = "range")]
    Range {
        modulation_order: ModulationOrder,
        num_subcarriers: SubcarrierIdx,
    },
    #[serde(rename = "skip")]
    Skip {
        main_modulation_order: ModulationOrder,
        skip_modulation_order: ModulationOrder,
        num_subcarriers: SubcarrierIdx,
    },
}
