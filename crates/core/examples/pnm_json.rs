/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Export a local PNM capture through centralized dispatch.
use pnm::pnm::parser::{
    model::DEFAULT_MAX_JSON_BYTES, parse_limits::ParseLimits, pnm_file::PnmFile,
};
use std::io::Read;

const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;

/// Reads one local file and writes its decoded JSON to stdout.
///
/// # Returns
/// Success after bounded input, parsing and bounded output.
/// # Errors
/// Reports missing arguments, file I/O, size limits or decoding errors.
fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logging::init()?;
    let mut arguments = std::env::args_os().skip(1);
    let path = arguments.next().ok_or("usage: pnm_json <capture.bin>")?;
    if arguments.next().is_some() {
        return Err("usage: pnm_json <capture.bin>".into());
    }
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err("capture exceeds example input limit".into());
    }
    let decoded = PnmFile::from_bytes(&bytes).decode(ParseLimits::default())?;
    println!("{}", decoded.to_json(DEFAULT_MAX_JSON_BYTES)?);
    Ok(())
}
