/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded measured/ideal I/Q overlays; scientific normalization stays in core.
use crate::{
    chart::{ChartMetadata, ChartViewport, Chip},
    Capture, Diagnostic, MAX_VIEW_SAMPLES,
};
use pnm::pnm::{
    data_type::IqPair,
    parser::{cm_ds_const_disp_meas::CmDsConstDispMeas, pnm_file::DecodedPnm},
};
use serde::Serialize;

const AXIS_PADDING: f64 = 1.07;
/// Smallest desktop zoom span, an application policy rather than a PHY limit.
const MINIMUM_IQ_SPAN: f64 = 1e-9;
const POINTS_PER_LAYER: usize = MAX_VIEW_SAMPLES / 2;

/// Square coordinate window with equal I/Q scale.
#[derive(Debug, Clone, Copy, Serialize)]
pub struct IqBounds {
    pub minimum_i: f64,
    pub maximum_i: f64,
    pub minimum_q: f64,
    pub maximum_q: f64,
}
impl IqBounds {
    fn contains(&self, point: &IqPair) -> bool {
        point[0] >= self.minimum_i
            && point[0] <= self.maximum_i
            && point[1] >= self.minimum_q
            && point[1] <= self.maximum_q
    }
    fn square(self) -> Result<Self, Diagnostic> {
        if ![
            self.minimum_i,
            self.maximum_i,
            self.minimum_q,
            self.maximum_q,
        ]
        .into_iter()
        .all(f64::is_finite)
        {
            return Err(Diagnostic::new(
                "InvalidRange",
                "I/Q bounds must be finite.",
            ));
        }
        let lower_i = self.minimum_i.min(self.maximum_i);
        let lower_q = self.minimum_q.min(self.maximum_q);
        let span_i = (self.maximum_i - self.minimum_i).abs();
        let span_q = (self.maximum_q - self.minimum_q).abs();
        let span = span_i.max(span_q);
        if !span.is_finite() || span_i < MINIMUM_IQ_SPAN || span_q < MINIMUM_IQ_SPAN {
            return Err(Diagnostic::new(
                "InvalidRange",
                "Select a finite I/Q region with positive width and height.",
            ));
        }
        let center_i = lower_i + span_i / 2.0;
        let center_q = lower_q + span_q / 2.0;
        let bounds = Self {
            minimum_i: center_i - span / 2.0,
            maximum_i: center_i + span / 2.0,
            minimum_q: center_q - span / 2.0,
            maximum_q: center_q + span / 2.0,
        };
        if ![
            bounds.minimum_i,
            bounds.maximum_i,
            bounds.minimum_q,
            bounds.maximum_q,
        ]
        .into_iter()
        .all(f64::is_finite)
            || bounds.maximum_i <= bounds.minimum_i
            || bounds.maximum_q <= bounds.minimum_q
        {
            return Err(Diagnostic::new(
                "InvalidRange",
                "I/Q zoom exceeds the supported coordinate range.",
            ));
        }
        Ok(bounds)
    }
}
/// Only bounded display samples cross IPC; counts distinguish shown and source points.
#[derive(Serialize)]
pub struct ConstellationViewport {
    pub bounds: IqBounds,
    pub measured: Vec<IqPair>,
    pub ideal: Vec<IqPair>,
    pub measured_total: usize,
    pub ideal_total: usize,
    pub measured_visible: usize,
    pub ideal_visible: usize,
    pub reference_available: bool,
    pub measured_label: String,
    pub ideal_label: String,
}

/// Builds compact constellation identification without copying sample arrays.
/// # Arguments
/// * `capture` - Decoded s2.13 capture with shared static reference metadata.
/// # Returns
/// Channel/MAC, QAM order and original sample count chips; no measurement tabs.
pub(crate) fn metadata(capture: &CmDsConstDispMeas<'_>) -> ChartMetadata {
    let reference = capture.constellation_analysis.reference;
    let modulation = reference
        .map(|reference| format!("{}-QAM", reference.order))
        .unwrap_or_else(|| format!("Unknown ({})", capture.actual_modulation_order));
    let mac = capture
        .mac_address
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join(":");
    ChartMetadata {
        modulation_profile: None,
        fec_summary: None,
        histogram: false,
        views: vec![],
        view_index: 0,
        title: "Downstream Constellation Display",
        unit: "",
        axis_label: "I/Q",
        sample_count: capture.iq().count(),
        minimum: -1.0,
        maximum: 1.0,
        spectrum_analysis: None,
        chips: vec![
            Chip::new("Channel", capture.channel_id),
            Chip::new("MAC", mac),
            Chip::new("QAM", modulation),
            Chip::new("Samples", capture.iq().count()),
        ],
    }
}

fn bounded(points: &[IqPair]) -> Vec<IqPair> {
    if points.len() <= POINTS_PER_LAYER {
        return points.to_vec();
    }
    // Deterministic display sampling includes both endpoints; original JSON is unchanged.
    (0..POINTS_PER_LAYER)
        .map(|index| points[index * (points.len() - 1) / (POINTS_PER_LAYER - 1)])
        .collect()
}

/// Filters and bounds both layers using one Rust-owned square coordinate window.
/// # Arguments
/// * `capture` - Original measured samples, with no second normalization.
/// * `requested` - Optional drag/wheel bounds; None fits measured and ideal data.
/// # Returns
/// At most MAX_VIEW_SAMPLES combined points, exact counts and a shared equal scale.
/// Empty and unknown-modulation captures remain viewable without invented points.
/// # Errors
/// Rejects non-finite, degenerate or overflowing coordinate windows.
pub(crate) fn viewport(
    capture: &CmDsConstDispMeas<'_>,
    requested: Option<IqBounds>,
) -> Result<ChartViewport, Diagnostic> {
    let measured: Vec<_> = capture.iq().flatten().collect();
    let ideal = capture
        .constellation_analysis
        .reference
        .map(|reference| reference.iq)
        .unwrap_or(&[]);
    let bounds = if let Some(bounds) = requested {
        bounds.square()?
    } else {
        let extent = measured
            .iter()
            .chain(ideal.iter())
            .flat_map(|pair| pair.iter())
            .fold(0.0_f64, |extent, value| extent.max(value.abs()));
        let extent = if extent > 0.0 {
            extent * AXIS_PADDING
        } else {
            1.0
        };
        IqBounds {
            minimum_i: -extent,
            maximum_i: extent,
            minimum_q: -extent,
            maximum_q: extent,
        }
    };
    let visible_measured: Vec<_> = measured
        .iter()
        .copied()
        .filter(|pair| bounds.contains(pair))
        .collect();
    let visible_ideal: Vec<_> = ideal
        .iter()
        .copied()
        .filter(|pair| bounds.contains(pair))
        .collect();
    let measured_points = bounded(&visible_measured);
    let ideal_points = bounded(&visible_ideal);
    let measured_label = format!(
        "Measured: {} of {} visible · {} total",
        measured_points.len(),
        visible_measured.len(),
        measured.len()
    );
    let ideal_label = if capture.constellation_analysis.reference.is_some() {
        format!(
            "Ideal: {} of {} visible · {} total",
            ideal_points.len(),
            visible_ideal.len(),
            ideal.len()
        )
    } else {
        "Ideal reference unavailable for this modulation".into()
    };
    Ok(ChartViewport {
        constellation: Some(ConstellationViewport {
            bounds,
            measured: measured_points,
            ideal: ideal_points,
            measured_total: measured.len(),
            ideal_total: ideal.len(),
            measured_visible: visible_measured.len(),
            ideal_visible: visible_ideal.len(),
            reference_available: capture.constellation_analysis.reference.is_some(),
            measured_label,
            ideal_label,
        }),
        modulation_profiles: None,
        fec_summary: None,
        histogram: None,
        iq_density: None,
        start: 0,
        count: measured.len(),
        total: measured.len(),
        points: vec![],
    })
}
impl Capture {
    /// Queries a coordinate zoom without altering capture bytes or JSON export.
    /// # Arguments
    /// * `bounds` - Requested I/Q region; reversed bounds are accepted and squared.
    /// # Returns
    /// A bounded constellation viewport with exact visible/source counts.
    /// # Errors
    /// Rejects non-constellation captures and invalid coordinate bounds.
    pub fn constellation_range(&self, bounds: IqBounds) -> Result<ChartViewport, Diagnostic> {
        match self.decoded()? {
            DecodedPnm::ConstellationDisplay(capture) => viewport(&capture, Some(bounds)),
            _ => Err(Diagnostic::new(
                "UnsupportedView",
                "I/Q region selection requires Constellation Display.",
            )),
        }
    }
}
