/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Locate read-only resources relative to the running installed application.
use std::path::{Path, PathBuf};

const DEMO_DIRECTORY: &str = "files/demo";

/// Finds the bundled capture directory for native desktop package layouts.
///
/// # Arguments
/// * `executable` - Resolved executable path supplied by the desktop shell.
///
/// # Returns
/// The first existing `files/demo` directory for Windows/Linux or a macOS app,
/// otherwise `None`. This does not read captures, write files or search user data.
pub fn examples_directory(executable: &Path) -> Option<PathBuf> {
    let binaries = executable.parent()?;
    let mut candidates = vec![binaries.join(DEMO_DIRECTORY)];
    if let Some(contents) = binaries.parent() {
        candidates.push(contents.join("Resources").join(DEMO_DIRECTORY));
    }
    candidates.into_iter().find(|directory| directory.is_dir())
}
