/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Capture ownership and bounded chart presentation.
pub mod chart;
pub mod collection;
mod diagnostic;
mod file;

pub use diagnostic::Diagnostic;
pub use file::{Capture, Metadata, Point, Viewport, MAX_INPUT_BYTES, MAX_VIEW_SAMPLES};
