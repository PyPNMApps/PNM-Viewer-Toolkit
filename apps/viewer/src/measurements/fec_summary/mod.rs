/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded per-profile FEC interval counts; preserve the decoder's raw JSON.
use crate::capture::{
    chart::{ChartMetadata, ChartViewport, Chip, MeasurementView},
    Diagnostic, MAX_VIEW_SAMPLES,
};
use pnm::pnm::{
    data_type::{CaptureTime, CounterValue, FecSummaryType, ProfileId},
    parser::cm_ds_ofdm_fec_summary::{CmDsOfdmFecSummary, CodewordEntry},
};
use serde::Serialize;

/// Maximum number of extrema/endpoints contributed by a three-series bucket.
const POINTS_PER_BUCKET: usize = 8;
/// Exact sum of u32 interval counts within the parser's bounded record budget.
pub type CodewordTotal = u64;

/// A profile choice identified by its source index, even if IDs repeat.
#[derive(Serialize)]
pub struct FecProfileChoice {
    pub profile_id: ProfileId,
    pub records: usize,
}
/// Small FEC selector metadata, without any interval arrays.
#[derive(Serialize)]
pub struct FecMetadata {
    pub profiles: Vec<FecProfileChoice>,
    pub profile_index: usize,
    pub summary_type: FecSummaryType,
}
/// One original record in chronological display order.
#[derive(Clone, Serialize)]
pub struct FecPoint {
    pub sample_index: usize,
    #[serde(flatten)]
    pub record: CodewordEntry,
    pub break_before: bool,
}
/// Selected-window statistics and a bounded three-series overview.
#[derive(Serialize)]
pub struct FecViewport {
    pub points: Vec<FecPoint>,
    pub total_maximum: CounterValue,
    pub error_maximum: CounterValue,
    pub total_codewords: CodewordTotal,
    pub corrected: CodewordTotal,
    pub uncorrectable: CodewordTotal,
    pub last: Option<CodewordEntry>,
    pub first_timestamp: Option<CaptureTime>,
    pub last_timestamp: Option<CaptureTime>,
    pub decimated: bool,
}

fn profile_index(
    capture: &CmDsOfdmFecSummary<'_>,
    view: MeasurementView,
) -> Result<usize, Diagnostic> {
    let index = match view {
        MeasurementView::Default => capture
            .fec_summary_data
            .iter()
            .position(|profile| profile.profile_id == 0)
            .unwrap_or(0),
        MeasurementView::FecProfile(index) => index,
        _ => {
            return Err(Diagnostic::new(
                "UnsupportedView",
                "FEC Summary requires a profile.",
            ))
        }
    };
    if index >= capture.fec_summary_data.len()
        && !(index == 0 && capture.fec_summary_data.is_empty())
    {
        return Err(Diagnostic::new(
            "InvalidProfile",
            "FEC profile is outside this capture.",
        ));
    }
    Ok(index)
}
fn records(
    capture: &CmDsOfdmFecSummary<'_>,
    view: MeasurementView,
) -> Result<Vec<CodewordEntry>, Diagnostic> {
    let index = profile_index(capture, view)?;
    let mut entries: Vec<_> = capture
        .fec_summary_data
        .get(index)
        .map(|profile| profile.entries().collect())
        .unwrap_or_default();
    entries.sort_by_key(|record| record.timestamp);
    Ok(entries)
}

/// Builds compact channel/MAC chips and bounded profile choices.
/// # Arguments
/// * `capture` - Validated FEC summary with original profile order.
/// * `view` - Default profile (ID 0 preferred) or an explicit source profile index.
/// # Returns
/// Selected-profile count and selector metadata without record arrays.
/// # Errors
/// Rejects unsupported views and invalid profile indices.
pub(crate) fn metadata(
    capture: &CmDsOfdmFecSummary<'_>,
    view: MeasurementView,
) -> Result<ChartMetadata, Diagnostic> {
    let index = profile_index(capture, view)?;
    Ok(ChartMetadata {
        modulation_profile: None,
        fec_summary: Some(FecMetadata {
            profiles: capture
                .fec_summary_data
                .iter()
                .map(|profile| FecProfileChoice {
                    profile_id: profile.profile_id,
                    records: usize::from(profile.number_of_sets),
                })
                .collect(),
            profile_index: index,
            summary_type: capture.summary_type,
        }),
        histogram: false,
        views: vec![],
        view_index: 0,
        title: "Downstream FEC Summary",
        unit: "codewords",
        axis_label: "Codewords per interval",
        sample_count: capture
            .fec_summary_data
            .get(index)
            .map_or(0, |profile| usize::from(profile.number_of_sets)),
        minimum: 0.0,
        maximum: 1.0,
        chips: vec![
            Chip::new("Channel", capture.channel_id),
            Chip::new(
                "MAC",
                capture
                    .mac_address
                    .iter()
                    .map(|octet| format!("{octet:02x}"))
                    .collect::<Vec<_>>()
                    .join(":"),
            ),
        ],
        spectrum_analysis: None,
    })
}

/// Maps an inclusive UTC window to chronological source indices in Rust.
/// # Arguments
/// * `capture` - Validated FEC summary.
/// * `view` - Selected profile.
/// * `first`, `last` - Inclusive epoch seconds; reverse selections are normalized.
/// # Returns
/// Bounded viewport including exact selected-window statistics.
/// # Errors
/// Rejects unsupported profiles; an empty time window returns an empty view.
pub(crate) fn time_range(
    capture: &CmDsOfdmFecSummary<'_>,
    view: MeasurementView,
    first: CaptureTime,
    last: CaptureTime,
) -> Result<ChartViewport, Diagnostic> {
    let entries = records(capture, view.clone())?;
    let lower = first.min(last);
    let upper = first.max(last);
    let start = entries.partition_point(|record| record.timestamp < lower);
    let end = entries.partition_point(|record| record.timestamp <= upper);
    viewport(capture, view, start, end - start)
}

/// Computes exact counts and a bounded extrema-preserving overview.
/// # Arguments
/// * `capture` - Validated FEC intervals; raw record order is never modified.
/// * `view` - Selected profile.
/// * `start`, `count` - Range in chronological stable display order.
/// # Returns
/// At most 1024 points preserving all three series' extrema, gaps, and endpoints;
/// all-window sums and last values are computed before decimation.
/// # Errors
/// Rejects invalid profile/range and overflowing range arithmetic.
pub(crate) fn viewport(
    capture: &CmDsOfdmFecSummary<'_>,
    view: MeasurementView,
    start: usize,
    count: usize,
) -> Result<ChartViewport, Diagnostic> {
    let entries = records(capture, view)?;
    let end = start
        .checked_add(count)
        .filter(|end| *end <= entries.len())
        .ok_or_else(|| Diagnostic::new("InvalidRange", "FEC range is outside this profile."))?;
    let selected = &entries[start..end];
    let expected_seconds = match capture.summary_type {
        2 => Some(1),
        3 => Some(60),
        _ => None,
    };
    let bucket_size = if count <= MAX_VIEW_SAMPLES {
        1
    } else {
        count.div_ceil(MAX_VIEW_SAMPLES / POINTS_PER_BUCKET)
    };
    let mut indices = Vec::new();
    for (bucket_index, bucket) in selected.chunks(bucket_size).enumerate() {
        let offset = bucket_index * bucket_size;
        indices.push(offset);
        indices.push(offset + bucket.len() - 1);
        for field in 0..3 {
            let value = |entry: &CodewordEntry| match field {
                0 => entry.total_codewords,
                1 => entry.corrected,
                _ => entry.uncorrectable,
            };
            indices.push(
                offset
                    + bucket
                        .iter()
                        .enumerate()
                        .min_by_key(|(_, entry)| value(entry))
                        .unwrap()
                        .0,
            );
            indices.push(
                offset
                    + bucket
                        .iter()
                        .enumerate()
                        .max_by_key(|(_, entry)| value(entry))
                        .unwrap()
                        .0,
            );
        }
    }
    indices.sort_unstable();
    indices.dedup();
    let mut previous = None;
    let points = indices
        .into_iter()
        .map(|index| {
            let break_before = previous.is_some_and(|previous_index| {
                expected_seconds.is_some_and(|interval| {
                    selected[previous_index..=index]
                        .windows(2)
                        .any(|pair| pair[1].timestamp.saturating_sub(pair[0].timestamp) > interval)
                })
            });
            previous = Some(index);
            FecPoint {
                sample_index: start + index,
                record: selected[index].clone(),
                break_before,
            }
        })
        .collect();
    Ok(ChartViewport {
        spectrum_average: None,
        constellation: None,
        modulation_profiles: None,
        fec_summary: Some(FecViewport {
            points,
            total_maximum: selected
                .iter()
                .map(|entry| entry.total_codewords)
                .max()
                .unwrap_or(0)
                .max(1),
            error_maximum: selected
                .iter()
                .map(|entry| entry.corrected.max(entry.uncorrectable))
                .max()
                .unwrap_or(0)
                .max(1),
            total_codewords: selected
                .iter()
                .map(|entry| u64::from(entry.total_codewords))
                .sum(),
            corrected: selected
                .iter()
                .map(|entry| u64::from(entry.corrected))
                .sum(),
            uncorrectable: selected
                .iter()
                .map(|entry| u64::from(entry.uncorrectable))
                .sum(),
            first_timestamp: selected.first().map(|entry| entry.timestamp),
            last_timestamp: selected.last().map(|entry| entry.timestamp),
            last: selected.last().cloned(),
            decimated: count > MAX_VIEW_SAMPLES,
        }),
        histogram: None,
        iq_density: None,
        start,
        count,
        total: entries.len(),
        points: vec![],
    })
}
