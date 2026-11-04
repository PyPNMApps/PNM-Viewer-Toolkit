/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Reusable spectrum bandwidth calculations without file-format dependencies.

/// Bandwidth in Hz, retaining fractional-Hz precision.
pub type BandwidthHz = f64;
/// Equivalent noise bandwidth divided by bin spacing (dimensionless).
pub type NoiseBandwidthFactor = f64;

/// Calculates the equivalent noise bandwidth of a spectrum resolution filter.
///
/// # Arguments
/// * `bin_spacing_hz` - Positive, finite frequency separation between bins.
/// * `noise_bandwidth_factor` - Positive, finite normalized ENBW multiplier.
/// # Returns
/// RBW in Hz, or None for invalid inputs, overflow or underflow to zero.
/// The caller owns conversion from any protocol-specific encoded factor.
pub fn resolution_bandwidth_hz(
    bin_spacing_hz: BandwidthHz,
    noise_bandwidth_factor: NoiseBandwidthFactor,
) -> Option<BandwidthHz> {
    if !bin_spacing_hz.is_finite()
        || bin_spacing_hz <= 0.0
        || !noise_bandwidth_factor.is_finite()
        || noise_bandwidth_factor <= 0.0
    {
        return None;
    }
    let bandwidth = bin_spacing_hz * noise_bandwidth_factor;
    if bandwidth.is_finite() && bandwidth > 0.0 {
        Some(bandwidth)
    } else {
        None
    }
}
