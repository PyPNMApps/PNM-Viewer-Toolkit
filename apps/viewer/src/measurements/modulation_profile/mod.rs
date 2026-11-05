/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded modulation expansion using the existing raw decoder and Shannon math.
use crate::capture::{
    chart::{overview_with_limit, ChartMetadata, ChartPoint, ChartViewport, Chip, MeasurementView},
    Diagnostic, MAX_VIEW_SAMPLES,
};
pub use pnm::pnm::data_type::modulation::CarrierType;
use pnm::pnm::{
    analysis::modulation_profile::ModulationProfileAnalysis,
    data_type::{
        modulation::ModulationAssignment,
        units::{BITS_PER_MEGABIT, HZ_PER_KHZ},
        BitRateBps, FrequencyHz, ModulationOrder, ProfileId,
    },
    parser::{
        cm_ds_ofdm_modulation_profile::{CmDsOfdmModulationProfile, ModulationScheme},
        parse_limits::ParseLimits,
    },
};
use serde::Serialize;
use std::collections::BTreeMap;

/// Shared budget for grouped special-carrier markers across every trace.
pub const MAX_CARRIER_MARKERS: usize = 4096;

/// Selects theoretical Shannon SNR or physical constellation bits per symbol.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum ModulationMetric {
    #[default]
    Shannon,
    BitLoading,
}
/// A compact profile choice with bounded expanded carrier count.
#[derive(Serialize)]
pub struct ProfileChoice {
    pub profile_id: ProfileId,
    pub bandwidth_bps: Option<BitRateBps>,
    pub bandwidth_text: String,
    pub carriers: usize,
}
/// Small profile/metric controls and full-profile assignment counts.
#[derive(Serialize)]
pub struct CarrierChoice {
    pub kind: CarrierType,
    pub label: &'static str,
    pub count: usize,
}
/// Small profile/metric controls and full-profile assignment counts.
#[derive(Serialize)]
pub struct ModulationMetadata {
    pub bandwidth_summary: String,
    pub carrier_types: Vec<CarrierChoice>,
    pub selected_carrier_types: Vec<CarrierType>,
    pub profiles: Vec<ProfileChoice>,
    pub selected_indices: Vec<usize>,
    pub metric_index: u32,
    pub assignment_summary: String,
}

/// One bounded trace; source index gives stable trace identity across selections.
#[derive(Serialize)]
pub struct ProfileSeries {
    pub profile_index: usize,
    pub profile_id: ProfileId,
    pub points: Vec<ChartPoint>,
}
/// Selected traces share the frequency grid and one display-point budget.
#[derive(Serialize)]
pub struct ModulationViewport {
    pub carrier_lanes: Vec<CarrierType>,
    pub markers: Vec<CarrierMarker>,
    pub series: Vec<ProfileSeries>,
}

/// Exact carrier or explicitly grouped carriers within a frequency interval.
#[derive(Serialize)]
pub struct CarrierMarker {
    pub profile_index: usize,
    pub profile_id: ProfileId,
    pub carrier_type: CarrierType,
    pub first_frequency_hz: FrequencyHz,
    pub last_frequency_hz: FrequencyHz,
    pub count: usize,
}
fn carrier_label(kind: CarrierType) -> &'static str {
    match kind {
        CarrierType::Data => "Data",
        CarrierType::ContinuousPilot => "Pilots",
        CarrierType::Plc => "PLC",
        CarrierType::Exclusion => "Excluded",
        CarrierType::ZeroBitLoaded => "Zero-loaded",
        CarrierType::Reserved => "Reserved",
        CarrierType::Unknown => "Unknown",
    }
}
fn selected_carriers(view: &MeasurementView) -> Vec<CarrierType> {
    let mut selected = match view {
        MeasurementView::ModulationProfile {
            carrier_types: Some(selected),
            ..
        } => selected.clone(),
        _ => CarrierType::ALL.to_vec(),
    };
    selected.sort_unstable();
    selected.dedup();
    selected
}
fn selection(
    capture: &CmDsOfdmModulationProfile<'_>,
    view: MeasurementView,
) -> Result<(Vec<usize>, ModulationMetric), Diagnostic> {
    let default_index = capture
        .profiles
        .iter()
        .position(|profile| profile.profile_id == 0)
        .unwrap_or(0);
    let default = || {
        if capture.profiles.is_empty() {
            vec![]
        } else {
            vec![default_index]
        }
    };
    let (mut indices, metric) = match view {
        MeasurementView::Default => (default(), ModulationMetric::Shannon),
        MeasurementView::ModulationProfile {
            indices, metric, ..
        } => (indices.unwrap_or_else(default), metric),
        _ => {
            return Err(Diagnostic::new(
                "UnsupportedView",
                "Modulation Profile requires profiles and a metric.",
            ))
        }
    };
    if indices.len() > capture.profiles.len()
        || indices.iter().any(|index| *index >= capture.profiles.len())
    {
        return Err(Diagnostic::new(
            "InvalidProfile",
            "Modulation profile is outside this capture.",
        ));
    }
    indices.sort_unstable();
    indices.dedup();
    let mut expanded = 0_usize;
    for index in &indices {
        expanded = expanded
            .checked_add(count(&capture.profiles[*index].schemes)?)
            .filter(|total| *total <= ParseLimits::default().max_samples)
            .ok_or_else(|| {
                Diagnostic::new(
                    "LimitExceeded",
                    "Selected profiles exceed the carrier budget.",
                )
            })?;
    }
    Ok((indices, metric))
}
fn total_carriers(capture: &CmDsOfdmModulationProfile<'_>) -> Result<usize, Diagnostic> {
    capture
        .profiles
        .iter()
        .map(|profile| count(&profile.schemes))
        .try_fold(0, |maximum, count| Ok(maximum.max(count?)))
}
fn count(schemes: &[ModulationScheme]) -> Result<usize, Diagnostic> {
    let mut total = 0_usize;
    for scheme in schemes {
        let count = match scheme {
            ModulationScheme::Range {
                num_subcarriers, ..
            }
            | ModulationScheme::Skip {
                num_subcarriers, ..
            } => *num_subcarriers,
        };
        total = total
            .checked_add(usize::from(count))
            .filter(|total| *total <= ParseLimits::default().max_samples)
            .ok_or_else(|| {
                Diagnostic::new(
                    "LimitExceeded",
                    "Expanded profile exceeds the carrier budget.",
                )
            })?;
    }
    Ok(total)
}
fn assignments(
    capture: &CmDsOfdmModulationProfile<'_>,
    index: usize,
) -> Result<Vec<ModulationOrder>, Diagnostic> {
    let Some(profile) = capture.profiles.get(index) else {
        return Ok(vec![]);
    };
    let mut values = Vec::with_capacity(count(&profile.schemes)?);
    for scheme in &profile.schemes {
        match *scheme {
            ModulationScheme::Range {
                modulation_order,
                num_subcarriers,
            } => {
                values.extend(std::iter::repeat_n(
                    modulation_order,
                    usize::from(num_subcarriers),
                ));
            }
            ModulationScheme::Skip {
                main_modulation_order,
                skip_modulation_order,
                num_subcarriers,
            } => {
                for offset in 0..num_subcarriers {
                    values.push(if offset % 2 == 0 {
                        main_modulation_order
                    } else {
                        skip_modulation_order
                    });
                }
            }
        }
    }
    Ok(values)
}
fn label(code: ModulationOrder) -> String {
    let assignment = ModulationAssignment::from_code(code);
    if matches!(
        assignment.carrier_type,
        CarrierType::Reserved | CarrierType::Unknown
    ) {
        format!("Reserved ({code})")
    } else {
        assignment.display_label.into()
    }
}

/// Builds profile choices, Channel/MAC chips and full-profile assignment counts.
/// # Arguments
/// * `capture` - Validated compressed schemes; all expanded counts are bounded.
/// * `view` - Default ID0/first profile or explicit profile/metric.
/// # Returns
/// Small controls; no full carrier arrays cross into QML.
/// # Errors
/// Rejects invalid views, profile indices or expansion budgets.
pub(crate) fn metadata(
    capture: &CmDsOfdmModulationProfile<'_>,
    view: MeasurementView,
) -> Result<ChartMetadata, Diagnostic> {
    let selected_carrier_types = selected_carriers(&view);
    let (indices, metric) = selection(capture, view)?;
    let mut counts = [0_usize; 256];
    for index in &indices {
        for code in assignments(capture, *index)? {
            counts[usize::from(code)] += 1;
        }
    }
    let summary = counts
        .iter()
        .enumerate()
        .filter(|(_, count)| **count > 0)
        .map(|(code, count)| format!("{}: {count}", label(code as ModulationOrder)))
        .collect::<Vec<_>>()
        .join(" · ");
    let profiles: Vec<ProfileChoice> = capture
        .profiles
        .iter()
        .map(|profile| {
            let bandwidth_bps =
                ModulationProfileAnalysis::calculate(profile, capture.subcarrier_spacing_khz)
                    .theoretical_throughput_bps;
            Ok(ProfileChoice {
                bandwidth_bps,
                bandwidth_text: match bandwidth_bps {
                    Some(rate) => format!("{:.3} Mbit/s", rate as f64 / BITS_PER_MEGABIT as f64),
                    None => "Unavailable".into(),
                },
                profile_id: profile.profile_id,
                carriers: count(&profile.schemes)?,
            })
        })
        .collect::<Result<_, Diagnostic>>()?;
    let bandwidth_summary = match indices.as_slice() {
        [index] => format!(
            "{} · {}",
            profiles[*index].profile_id, profiles[*index].bandwidth_text
        ),
        [] => "None selected".into(),
        _ => format!("{} profiles", indices.len()),
    };
    let (axis_label, unit, maximum) = match metric {
        ModulationMetric::Shannon => ("Shannon minimum MER (ideal)", "dB", 45.0),
        ModulationMetric::BitLoading => ("Bit loading", "bits/symbol", 15.0),
    };
    let assignment_summary = if indices.is_empty() {
        "No profiles selected".into()
    } else if indices.len() > 1 {
        format!("Selected assignments: {summary}")
    } else {
        summary
    };
    Ok(ChartMetadata {
        modulation_profile: Some(ModulationMetadata {
            bandwidth_summary,
            carrier_types: CarrierType::ALL
                .into_iter()
                .map(|kind| CarrierChoice {
                    kind,
                    label: carrier_label(kind),
                    count: counts
                        .iter()
                        .enumerate()
                        .filter(|(code, _)| {
                            ModulationAssignment::from_code(*code as ModulationOrder).carrier_type
                                == kind
                        })
                        .map(|(_, count)| count)
                        .sum(),
                })
                .collect(),
            selected_carrier_types,
            profiles,
            selected_indices: indices,
            metric_index: if metric == ModulationMetric::Shannon {
                0
            } else {
                1
            },
            assignment_summary,
        }),
        fec_summary: None,
        histogram: false,
        views: vec![],
        view_index: 0,
        title: "Downstream Modulation Profile",
        unit,
        axis_label,
        sample_count: total_carriers(capture)?,
        minimum: 0.0,
        maximum,
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

/// Expands one profile into checked Hz coordinates and selected modulation values.
/// # Arguments
/// * `capture` - Decoded header/schemes; original bytes remain untouched.
/// * `codes` - Expanded assignments for one selected profile.
/// * `metric` - Shannon bound or constellation bits per symbol.
/// * `selected_carrier_types` - Roles allowed to contribute numeric points.
/// # Returns
/// Temporary bounded points for selected data, pilot and PLC constellations.
/// Unknown/excluded/reserved assignments are gaps; zero-loaded
/// carriers retain zero bits in JSON and are displayed in a separate lane. Frequency alignment
/// follows PyPNM: zero_hz + (first_active_index + ordinal) * spacing_hz.
/// # Errors
/// Rejects zero spacing, u32 Hz overflow, invalid profiles and expanded limits.
fn points_for(
    capture: &CmDsOfdmModulationProfile<'_>,
    codes: &[ModulationOrder],
    metric: ModulationMetric,
    selected_carrier_types: &[CarrierType],
) -> Result<Vec<ChartPoint>, Diagnostic> {
    codes
        .iter()
        .copied()
        .enumerate()
        .map(|(sample_index, code)| {
            let frequency_hz = frequency(capture, sample_index)?;
            let assignment = ModulationAssignment::from_code(code);
            let value = if selected_carrier_types.contains(&assignment.carrier_type)
                && assignment.constellation().is_some()
            {
                match metric {
                    ModulationMetric::Shannon => assignment.shannon_minimum_mer_db(),
                    ModulationMetric::BitLoading => assignment.bit_loading.map(f64::from),
                }
            } else {
                None
            };
            Ok(ChartPoint {
                sample_index,
                frequency_hz,
                value,
            })
        })
        .collect()
}

/// Builds synchronized overlays with a shared bounded display-point budget.
/// # Arguments
/// * `capture` - Compact decoded profiles on a shared frequency grid.
/// * `view` - Selected source indices and metric; empty selection is allowed.
/// * `start`, `count` - Shared zero-based carrier window, not concatenated traces.
/// # Returns
/// Stable profile identities and at most MAX_VIEW_SAMPLES total display points,
/// plus two frequency-axis anchors. Shorter profiles remain gaps after their end.
/// # Errors
/// Rejects invalid selections, ranges, expansion budgets or frequency overflow.
pub(crate) fn viewport(
    capture: &CmDsOfdmModulationProfile<'_>,
    view: MeasurementView,
    start: usize,
    count: usize,
) -> Result<ChartViewport, Diagnostic> {
    let selected_carrier_types = selected_carriers(&view);
    let (indices, metric) = selection(capture, view)?;
    let total = total_carriers(capture)?;
    let end = start
        .checked_add(count)
        .filter(|end| *end <= total)
        .ok_or_else(|| Diagnostic::new("InvalidRange", "Zoom range is outside this capture."))?;
    // The file has at most 255 profiles (u8), so each trace receives at least
    // four of the 1024 points: enough for endpoints and bounded extrema.
    let limit = MAX_VIEW_SAMPLES / indices.len().max(1);
    let marker_bucket_count =
        (MAX_CARRIER_MARKERS / (indices.len().max(1) * (CarrierType::ALL.len() - 1))).max(1);
    let marker_bucket_width = count.div_ceil(marker_bucket_count).max(1);
    let mut markers = Vec::new();
    let mut present = Vec::new();
    let mut series = Vec::new();
    for index in indices {
        let codes = assignments(capture, index)?;
        for code in &codes {
            let kind = ModulationAssignment::from_code(*code).carrier_type;
            if kind != CarrierType::Data
                && selected_carrier_types.contains(&kind)
                && !present.contains(&kind)
            {
                present.push(kind);
            }
        }
        let mut grouped: BTreeMap<(usize, CarrierType), CarrierMarker> = BTreeMap::new();
        for (ordinal, code) in codes.iter().enumerate().take(end).skip(start) {
            let kind = ModulationAssignment::from_code(*code).carrier_type;
            if kind == CarrierType::Data || !selected_carrier_types.contains(&kind) {
                continue;
            }
            let frequency_hz = frequency(capture, ordinal)?;
            let marker = grouped
                .entry(((ordinal - start) / marker_bucket_width, kind))
                .or_insert(CarrierMarker {
                    profile_index: index,
                    profile_id: capture.profiles[index].profile_id,
                    carrier_type: kind,
                    first_frequency_hz: frequency_hz,
                    last_frequency_hz: frequency_hz,
                    count: 0,
                });
            marker.count += 1;
            marker.last_frequency_hz = frequency_hz;
        }
        markers.extend(grouped.into_values());
        let points = if selected_carrier_types.iter().any(|kind| {
            matches!(
                kind,
                CarrierType::Data | CarrierType::ContinuousPilot | CarrierType::Plc
            )
        }) {
            points_for(capture, &codes, metric, &selected_carrier_types)?
        } else {
            vec![]
        };
        let first = start.min(points.len());
        let last = end.min(points.len());
        series.push(ProfileSeries {
            profile_index: index,
            profile_id: capture.profiles[index].profile_id,
            points: overview_with_limit(&points[first..last], limit),
        });
    }
    // Axis anchors keep the complete shared frequency range even with no traces.
    let mut anchors = Vec::new();
    if count > 0 {
        for sample_index in [start, end - 1] {
            let frequency_hz = frequency(capture, sample_index)?;
            anchors.push(ChartPoint {
                sample_index,
                frequency_hz,
                value: None,
            });
        }
    }
    Ok(ChartViewport {
        constellation: None,
        modulation_profiles: Some(ModulationViewport {
            series,
            markers,
            carrier_lanes: CarrierType::ALL
                .into_iter()
                .filter(|kind| present.contains(kind))
                .collect(),
        }),
        fec_summary: None,
        histogram: None,
        iq_density: None,
        start,
        count,
        total,
        points: anchors,
    })
}
fn frequency(
    capture: &CmDsOfdmModulationProfile<'_>,
    index: usize,
) -> Result<FrequencyHz, Diagnostic> {
    let spacing = FrequencyHz::from(capture.subcarrier_spacing_khz)
        .checked_mul(HZ_PER_KHZ)
        .filter(|spacing| *spacing > 0);
    FrequencyHz::try_from(index)
        .ok()
        .and_then(|index| {
            index.checked_add(FrequencyHz::from(capture.first_active_subcarrier_index))
        })
        .zip(spacing)
        .and_then(|(index, spacing)| index.checked_mul(spacing))
        .and_then(|offset| capture.subcarrier_zero_frequency_hz.checked_add(offset))
        .ok_or_else(|| {
            Diagnostic::new(
                "InvalidFrequencyGrid",
                "Modulation frequency or spacing is outside the supported Hz range.",
            )
        })
}
