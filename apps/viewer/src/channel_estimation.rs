/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Downstream channel-response analysis and bounded I/Q presentation.
use crate::{
    chart::{ChartMetadata, Chip, MeasurementView},
    Diagnostic,
};
use pnm::{
    common::signal_processing::channel_response::{
        group_delay_seconds, magnitude_db, GroupDelaySeconds, MagnitudeDb,
    },
    pnm::{
        data_type::{
            units::{HZ_PER_KHZ, MICROSECONDS_PER_SECOND},
            FrequencyHz, IqPair,
        },
        parser::cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef,
    },
};
use serde::Serialize;

/// Bounded 32-by-32 density map; every valid I/Q pair contributes to one cell.
const IQ_GRID_SIDE: usize = 32;
const IQ_AXIS_PADDING: f64 = 1.05;
/// Group delay in microseconds, matching the PyPNM chart convention.
pub type GroupDelayUs = f64;

/// Frequency and phase-derived delay at one original carrier position.
#[derive(Serialize)]
pub struct GroupDelayPoint {
    pub frequency_hz: FrequencyHz,
    pub delay_s: Option<GroupDelaySeconds>,
}

/// Full-resolution derived data for file export; never sent as viewport metadata.
#[derive(Serialize)]
pub struct ChannelEstimationAnalysis {
    pub frequencies_hz: Vec<FrequencyHz>,
    pub magnitude_db: Vec<Option<MagnitudeDb>>,
    pub group_delay: Vec<GroupDelayPoint>,
}
impl ChannelEstimationAnalysis {
    /// Calculates unsmoothed response magnitude and phase-derived delay.
    /// # Arguments
    /// * `capture` - Decoded channel estimate; original I/Q remains unchanged.
    /// # Returns
    /// Aligned arrays with nulls for undefined values and excluded carriers.
    /// # Errors
    /// Rejects zero spacing or a frequency outside the u32 Hz domain.
    pub fn from_capture(capture: &CmDsOfdmChanEstimateCoef<'_>) -> Result<Self, Diagnostic> {
        let spacing = u32::from(capture.subcarrier_spacing_khz) * HZ_PER_KHZ;
        if spacing == 0 {
            return Err(Diagnostic::new(
                "InvalidFrequencyGrid",
                "Channel estimate spacing must be positive.",
            ));
        }
        let samples: Vec<_> = capture.iq().collect();
        let frequencies_hz = (0..samples.len())
            .map(|index| {
                let frequency = u64::from(capture.subcarrier_zero_frequency_hz)
                    + (u64::from(capture.first_active_subcarrier_index) + index as u64)
                        * u64::from(spacing);
                FrequencyHz::try_from(frequency).map_err(|_| {
                    Diagnostic::new(
                        "InvalidFrequencyGrid",
                        "Channel estimate frequency exceeds the supported Hz range.",
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let group_delay = frequencies_hz
            .iter()
            .copied()
            .zip(group_delay_seconds(&samples, spacing))
            .map(|(frequency_hz, delay_s)| GroupDelayPoint {
                frequency_hz,
                delay_s,
            })
            .collect();
        Ok(Self {
            frequencies_hz,
            magnitude_db: samples.iter().map(|sample| magnitude_db(*sample)).collect(),
            group_delay,
        })
    }
    /// Converts delay to the Qt plot's display unit without changing exported seconds.
    /// # Returns
    /// Microseconds at each carrier, preserving unavailable values.
    pub fn group_delay_us(&self) -> Vec<Option<GroupDelayUs>> {
        self.group_delay
            .iter()
            .map(|point| {
                point
                    .delay_s
                    .map(|seconds| seconds * MICROSECONDS_PER_SECOND)
            })
            .collect()
    }
}
#[derive(Serialize)]
pub(crate) struct ChannelEstimationExport<'capture, 'input> {
    #[serde(flatten)]
    pub capture: &'capture CmDsOfdmChanEstimateCoef<'input>,
    pub channel_estimation_analysis: ChannelEstimationAnalysis,
}

/// Occupied grid cell, not an invented individual I/Q sample.
#[derive(Serialize)]
pub struct IqDensityCell {
    pub column: usize,
    pub row: usize,
    pub count: usize,
}
/// Equal I/Q axis limits and counts for a bounded density scatter overview.
#[derive(Serialize)]
pub struct IqDensity {
    pub minimum: f64,
    pub maximum: f64,
    pub grid_side: usize,
    pub valid_count: usize,
    pub excluded_count: usize,
    pub maximum_cell_count: usize,
    pub cells: Vec<IqDensityCell>,
}
impl IqDensity {
    /// Aggregates all valid complex pairs into at most 1024 occupied cells.
    /// # Arguments
    /// * `samples` - Decoded I/Q, retaining zero pairs and skipping excluded pairs.
    /// # Returns
    /// Symmetric equal axes, grid counts and valid/excluded totals. Empty inputs
    /// use [-1, 1] bounds; no normalization is applied to the response.
    pub fn from_samples(samples: &[Option<IqPair>]) -> Self {
        let valid: Vec<_> = samples
            .iter()
            .flatten()
            .filter(|pair| pair.iter().all(|value| value.is_finite()))
            .collect();
        let extent = valid
            .iter()
            .flat_map(|pair| pair.iter())
            .fold(0.0_f64, |extent, value| extent.max(value.abs()));
        let extent = if extent > 0.0 {
            extent * IQ_AXIS_PADDING
        } else {
            1.0
        };
        let mut counts = vec![0; IQ_GRID_SIDE * IQ_GRID_SIDE];
        for pair in &valid {
            let cell_index = |value: f64| {
                (((value + extent) / (2.0 * extent) * IQ_GRID_SIDE as f64).floor() as usize)
                    .min(IQ_GRID_SIDE - 1)
            };
            counts[cell_index(pair[1]) * IQ_GRID_SIDE + cell_index(pair[0])] += 1;
        }
        let maximum_cell_count = counts.iter().copied().max().unwrap_or(0);
        let cells = counts
            .into_iter()
            .enumerate()
            .filter(|(_, count)| *count > 0)
            .map(|(index, count)| IqDensityCell {
                column: index % IQ_GRID_SIDE,
                row: index / IQ_GRID_SIDE,
                count,
            })
            .collect();
        Self {
            minimum: -extent,
            maximum: extent,
            grid_side: IQ_GRID_SIDE,
            valid_count: valid.len(),
            excluded_count: samples.len() - valid.len(),
            maximum_cell_count,
            cells,
        }
    }
}

/// Builds channel-estimation tab labels and data-dependent plot bounds.
/// # Arguments
/// * `capture` - Parsed downstream response.
/// * `view` - Selected measurement tab.
/// # Returns
/// Small chip/tab configuration, with no sample arrays.
/// # Errors
/// Rejects invalid frequency grids through the shared analysis mapper.
pub(crate) fn metadata(
    capture: &CmDsOfdmChanEstimateCoef<'_>,
    view: MeasurementView,
) -> Result<ChartMetadata, Diagnostic> {
    let analysis = ChannelEstimationAnalysis::from_capture(capture)?;
    let delay_us = analysis.group_delay_us();
    let (axis_label, unit, values) = match view {
        MeasurementView::GroupDelay => ("Group delay", "µs", &delay_us),
        _ => ("Magnitude", "dB", &analysis.magnitude_db),
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
    let mac = capture
        .mac_address
        .iter()
        .map(|octet| format!("{octet:02x}"))
        .collect::<Vec<_>>()
        .join(":");
    Ok(ChartMetadata {
        title: "Downstream Channel Estimation",
        unit,
        axis_label,
        sample_count: analysis.frequencies_hz.len(),
        minimum: minimum - margin,
        maximum: maximum + margin,
        chips: vec![
            Chip::new("Channel", capture.channel_id),
            Chip::new("MAC", mac),
        ],
        spectrum_analysis: None,
        views: vec!["Magnitude", "Group delay", "I/Q scatter"],
        view_index: view as u32,
    })
}
