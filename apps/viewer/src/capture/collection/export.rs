/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use crate::capture::Diagnostic;
use std::path::Path;
/// Exports all source epochs, valid counts and full-resolution statistics.
/// # Arguments
/// * `result` - Complete aggregate, never a decimated plot.
/// * `path` - New writable destination.
/// # Returns
/// Success after syncing the new JSON file.
/// # Errors
/// Rejects existing destinations, serialization limits and write failures.
pub fn write_json_new<Document: serde::Serialize>(
    result: &Document,
    path: &Path,
) -> Result<(), Diagnostic> {
    use std::io::Write;
    let json = pnm::common::json::to_json(result, pnm::common::json::DEFAULT_MAX_JSON_BYTES)
        .map_err(|error| Diagnostic::new("Json", error))?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Diagnostic::new("Export", error))?;
    if let Err(error) = file
        .write_all(json.as_bytes())
        .and_then(|()| file.sync_all())
    {
        drop(file);
        let _ = std::fs::remove_file(path);
        return Err(Diagnostic::new("Export", error));
    }
    Ok(())
}
