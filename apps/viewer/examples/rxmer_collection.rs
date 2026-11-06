/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Offline batch API example: PATH prints a catalog; PATH GROUP_ID prints statistics.
use std::{path::Path, sync::atomic::AtomicBool};
use viewer::measurements::rxmer::collection::{Collection, ImportLimits};
fn main() -> Result<(), String> {
    let arguments: Vec<_> = std::env::args().skip(1).collect();
    if !(1..=2).contains(&arguments.len()) {
        return Err("Usage: rxmer_collection DIRECTORY_OR_ARCHIVE [GROUP_ID]".into());
    }
    let collection = Collection::scan(
        Path::new(&arguments[0]),
        ImportLimits::default(),
        &AtomicBool::new(false),
    )
    .map_err(|error| error.message)?;
    let json = if let Some(id) = arguments.get(1) {
        let result = collection
            .aggregate(id.parse().map_err(|_| "Invalid group ID.")?)
            .map_err(|error| error.message)?;
        pnm::common::json::to_json(&result, pnm::common::json::DEFAULT_MAX_JSON_BYTES)
    } else {
        serde_json::to_string_pretty(&collection.catalog)
    }
    .map_err(|error| error.to_string())?;
    println!("{json}");
    Ok(())
}
