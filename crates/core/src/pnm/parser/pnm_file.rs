/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Original-byte container and the sole concrete parser dispatch path.
use super::{
    cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
    pnm_file_type::PnmDecoderKind,
    pnm_header::PnmHeader,
};
use crate::diagnostic::{ErrorCode, ParseError};

/// An imported byte view, retaining diagnostics when the header is unsupported.
#[derive(Debug)]
pub struct PnmFile<'input> {
    bytes: &'input [u8],
    header: Result<PnmHeader<'input>, ParseError>,
}
/// Implemented second-tier decoder results; add variants as decoders land.
#[derive(Debug, PartialEq)]
pub enum DecodedPnm<'input> {
    RxMer(CmDsOfdmRxMer<'input>),
}
impl<'input> PnmFile<'input> {
    /// Inspects common metadata while retaining even unknown or malformed files.
    ///
    /// # Arguments
    /// * `bytes` - Original bytes owned by the caller for this object's lifetime.
    /// # Returns
    /// An inspectable file; header failures are available through header().
    pub fn from_bytes(bytes: &'input [u8]) -> Self {
        Self {
            bytes,
            header: PnmHeader::from_bytes(bytes),
        }
    }
    /// Borrows the complete unchanged file.
    ///
    /// # Returns
    /// Original bytes, including when header recognition failed.
    pub fn bytes(&self) -> &'input [u8] {
        self.bytes
    }
    /// Borrows the first-tier header or its recognition diagnostic.
    ///
    /// # Returns
    /// Parsed header and payload view on success.
    /// # Errors
    /// Reports the original header diagnostic without attempting to decode again.
    pub fn header(&self) -> Result<&PnmHeader<'input>, &ParseError> {
        self.header.as_ref()
    }
    /// Dispatches the parsed header to its central decoder route.
    ///
    /// # Arguments
    /// * `limits` - Caller resource policy for the implemented RxMER decoder.
    /// # Returns
    /// Typed decoded data; the file remains available for raw inspection.
    /// # Errors
    /// Propagates header/payload errors or reports a known but unavailable decoder.
    #[tracing::instrument(level = "debug", skip_all, fields(input_bytes = self.bytes.len()), err)]
    pub fn decode(&self, limits: ParseLimits) -> Result<DecodedPnm<'input>, ParseError> {
        let header = self.header.as_ref().map_err(Clone::clone)?;
        match header.descriptor().decoder {
            PnmDecoderKind::RxMer => Ok(DecodedPnm::RxMer(CmDsOfdmRxMer::from_header(
                header.clone(),
                limits,
            )?)),
            _ => Err(ParseError {
                code: ErrorCode::UnsupportedDecoder,
                offset: 0,
                field: "file_type",
            }),
        }
    }
}
