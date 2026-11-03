/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Export the preserved PyPNM RxMER fixture using the actual parser.
use pnm_core::pnm::parser::{
    cm_ds_ofdm_rx_mer::{CmDsOfdmRxMer, ParseLimits},
    model::DEFAULT_MAX_JSON_BYTES,
};

const RXMER_FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/pypnm/tests/files/rxmer.bin");

/// Parses the bundled fixture and prints its decoded JSON to stdout.
///
/// # Returns
/// Success after generating the complete bounded JSON document.
/// # Errors
/// Propagates parsing and serialization errors to the command exit status.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let capture = CmDsOfdmRxMer::parse(RXMER_FIXTURE, ParseLimits::default())?;
    println!("{}", capture.to_json(DEFAULT_MAX_JSON_BYTES)?);
    Ok(())
}
