/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! RxMER decoding over the shared bounded archive traversal.
use crate::capture::{collection, Diagnostic};
pub use collection::ImportLimits;
use pnm::pnm::{
    analysis::rxmer::aggregate::Series,
    parser::{
        cm_ds_ofdm_rx_mer::CmDsOfdmRxMer, parse_limits::ParseLimits, pnm_file_type::PnmDecoderKind,
        pnm_header::PnmHeader,
    },
};
use std::{path::Path, sync::atomic::AtomicBool};

pub(super) fn scan(
    path: &Path,
    limits: ImportLimits,
    cancel: &AtomicBool,
) -> Result<collection::Scanned<Series>, Diagnostic> {
    let scanned = collection::scan(path, limits, cancel, decode)?;
    if scanned.series.is_empty() {
        return Err(Diagnostic::new(
            "RxMerImport",
            format!(
                "No usable RxMER captures found. {}",
                scanned
                    .issues
                    .first()
                    .map(String::as_str)
                    .unwrap_or("Select a capture directory or archive.")
            ),
        ));
    }
    Ok(scanned)
}
fn decode(bytes: &[u8], name: &str) -> Result<Option<(Series, usize)>, Diagnostic> {
    let header = match PnmHeader::from_bytes(bytes) {
        Ok(header) if header.descriptor().decoder == PnmDecoderKind::RxMer => header,
        _ => return Ok(None),
    };
    let capture = CmDsOfdmRxMer::from_header(header, ParseLimits::default())
        .map_err(|error| Diagnostic::new("RxMerImport", error))?;
    let series = Series::from_capture(&capture, name.into())
        .map_err(|error| Diagnostic::new("RxMerImport", error))?;
    Ok(Some((series, capture.raw_values().len())))
}
