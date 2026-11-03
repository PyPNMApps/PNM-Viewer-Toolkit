/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable unit conversion factors, independent of file formats.

/// Number of hertz in one kilohertz.
/// Multiply an integer kHz value by this factor to express it in Hz.
pub const HZ_PER_KHZ: u32 = 1_000;

/// Number of quarter-decibel steps in one decibel.
/// Divide a valid quarter-dB sample by this factor to express it in dB.
/// Format-specific unavailable-value handling belongs to the decoder.
pub const QUARTER_DB_PER_DB: f64 = 4.0;
