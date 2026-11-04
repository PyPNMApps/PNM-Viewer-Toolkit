/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Presentation of current and last-update upstream coefficients using core analysis.
use crate::{
    chart::{ChartMetadata, Chip, MeasurementView},
    Diagnostic,
};
use pnm::pnm::{
    analysis::upstream_pre_equalization::PreEqualizationAnalysis,
    data_type::{units::MICROSECONDS_PER_SECOND, MacAddress},
    parser::{
        cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, pnm_file_type::PnmFileKind, pnm_header::HasPnmHeader,
    },
};

/// Prepares compact metadata for the coefficient response, delay and I/Q tabs.
/// # Arguments
/// * `capture` - Current coefficients or the last RNG-RSP update coefficients.
/// * `view` - Validated response tab.
/// # Returns
/// Titles, units, chips and full-capture vertical bounds; no full arrays.
/// # Errors
/// Preserves core diagnostics for invalid frequency grids.
pub(crate) fn metadata(
    capture: &CmUsOfdmaPreEq<'_>,
    view: MeasurementView,
) -> Result<ChartMetadata, Diagnostic> {
    let last_update =
        capture.pnm_header().descriptor().kind == PnmFileKind::UpstreamPreEqualizationLastUpdate;
    let analysis = PreEqualizationAnalysis::from_capture(capture)?;
    let values = if view == MeasurementView::GroupDelay {
        analysis
            .group_delay
            .iter()
            .map(|point| point.delay_s.map(|delay| delay * MICROSECONDS_PER_SECOND))
            .collect()
    } else {
        analysis.magnitude_db
    };
    let minimum = values
        .iter()
        .flatten()
        .copied()
        .reduce(f64::min)
        .unwrap_or(-1.0);
    let maximum = values
        .iter()
        .flatten()
        .copied()
        .reduce(f64::max)
        .unwrap_or(1.0);
    let margin = ((maximum - minimum) * 0.1).max(0.1);
    let mac = |address: MacAddress| {
        address
            .iter()
            .map(|octet| format!("{octet:02x}"))
            .collect::<Vec<_>>()
            .join(":")
    };
    Ok(ChartMetadata {
        modulation_profile: None,
        fec_summary: None,
        histogram: false,
        spectrum_analysis: None,
        title: if last_update {
            "Upstream Pre-EQ Last Update"
        } else {
            "Upstream Pre-EQ"
        },
        unit: if view == MeasurementView::GroupDelay {
            "µs"
        } else {
            "dB"
        },
        axis_label: if view == MeasurementView::GroupDelay {
            if last_update {
                "Update group delay"
            } else {
                "Coefficient group delay"
            }
        } else if last_update {
            "Update magnitude"
        } else {
            "Pre-EQ magnitude"
        },
        sample_count: analysis.frequencies_hz.len(),
        minimum: minimum - margin,
        maximum: maximum + margin,
        chips: vec![
            Chip::new("Channel", capture.channel_id),
            Chip::new("MAC", mac(capture.mac_address)),
            Chip::new("CMTS", mac(capture.cmts_mac_address)),
            Chip::new(
                "Capture",
                if last_update {
                    "Last update"
                } else {
                    "Current"
                },
            ),
        ],
        views: vec!["Magnitude", "Group delay", "I/Q scatter"],
        view_index: match view {
            MeasurementView::GroupDelay => 1,
            MeasurementView::IqScatter => 2,
            _ => 0,
        },
    })
}
