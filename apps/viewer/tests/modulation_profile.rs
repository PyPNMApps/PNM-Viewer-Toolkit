/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use pnm::pnm::parser::{
    cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile, parse_limits::ParseLimits,
};
use viewer::{
    chart::MeasurementView, modulation_profile::ModulationMetric, Capture, MAX_VIEW_SAMPLES,
};

fn trace(mut viewport: viewer::chart::ChartViewport) -> viewer::chart::ChartViewport {
    viewport.points = viewport
        .modulation_profiles
        .take()
        .unwrap()
        .series
        .into_iter()
        .next()
        .map(|series| series.points)
        .unwrap_or_default();
    viewport
}
fn view(index: usize, metric: ModulationMetric) -> MeasurementView {
    MeasurementView::ModulationProfile {
        carrier_types: None,
        indices: Some(vec![index]),
        metric,
    }
}
// Synthetic compressed profile using only the fixture's validated header shape.
fn synthetic(schemes: &[u8], zero_hz: u32, first: u16, spacing_khz: u8) -> Capture {
    let mut bytes = pnm_fixtures::MODULATION_PROFILE[..29].to_vec();
    bytes[17] = 1;
    bytes[18..22].copy_from_slice(&zero_hz.to_be_bytes());
    bytes[22..24].copy_from_slice(&first.to_be_bytes());
    bytes[24] = spacing_khz;
    bytes[25..29].copy_from_slice(&(3_u32 + schemes.len() as u32).to_be_bytes());
    bytes.push(7);
    bytes.extend_from_slice(&(schemes.len() as u16).to_be_bytes());
    bytes.extend_from_slice(schemes);
    Capture::from_bytes(bytes, "synthetic-modulation.bin".into()).unwrap()
}

#[test]
fn fixture_profiles_grid_counts_and_raw_export() {
    let capture = Capture::from_bytes(
        pnm_fixtures::MODULATION_PROFILE.to_vec(),
        "modulation_profile.bin".into(),
    )
    .unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Downstream Modulation Profile");
    assert_eq!(metadata.sample_count, 7480);
    assert_eq!(metadata.chips.len(), 2);
    let controls = metadata.modulation_profile.unwrap();
    assert_eq!(controls.selected_indices, [3]);
    assert_eq!(controls.bandwidth_summary, "0 · 1481.600 Mbit/s");
    for (profile, expected) in controls.profiles.iter().zip([
        2_222_400_000_u64,
        2_037_200_000,
        1_852_000_000,
        1_481_600_000,
    ]) {
        assert_eq!(profile.bandwidth_bps, Some(expected));
    }
    assert_eq!(controls.profiles[0].bandwidth_text, "2222.400 Mbit/s");
    assert_eq!(
        controls
            .profiles
            .iter()
            .map(|profile| profile.profile_id)
            .collect::<Vec<_>>(),
        [3, 2, 1, 0]
    );
    assert!(controls.assignment_summary.contains("256-QAM: 7408"));
    assert!(controls.assignment_summary.contains("Pilots: 56"));
    assert!(controls.assignment_summary.contains("PLC: 16"));
    let full = trace(capture.chart_range(0, 7480).unwrap());
    assert!(full.points.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(full.points.first().unwrap().frequency_hz, 640_000_000);
    assert_eq!(full.points.last().unwrap().frequency_hz, 826_975_000);
    for (index, expected) in [12.0, 11.0, 10.0, 8.0].into_iter().enumerate() {
        let detail = capture
            .chart_range_for(view(index, ModulationMetric::BitLoading), 0, 1)
            .map(trace)
            .unwrap();
        assert_eq!(detail.points[0].value, Some(expected));
    }
    let default = trace(capture.chart_range(0, 1).unwrap());
    assert!((default.points[0].value.unwrap() - 24.06540180433955).abs() < 1e-10);
    let parsed =
        CmDsOfdmModulationProfile::parse(pnm_fixtures::MODULATION_PROFILE, ParseLimits::default())
            .unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.json().unwrap()).unwrap(),
        serde_json::to_value(parsed).unwrap()
    );
    assert!(capture
        .chart_metadata_for(view(4, ModulationMetric::Shannon))
        .is_err());
    assert!(capture
        .chart_metadata_for(MeasurementView::GroupDelay)
        .is_err());
    assert!(capture
        .chart_range_for(MeasurementView::FecProfile(0), 0, 1)
        .is_err());
}

#[test]
fn skip_starts_main_and_restarts_after_each_scheme() {
    let capture = synthetic(
        &[1, 8, 10, 0, 5, 0, 12, 0, 1, 1, 4, 6, 0, 3],
        100_000_000,
        10,
        25,
    );
    let chart = capture
        .chart_range_for(view(0, ModulationMetric::BitLoading), 0, 9)
        .map(trace)
        .unwrap();
    assert_eq!(
        chart
            .points
            .iter()
            .map(|point| point.value.unwrap() as u8)
            .collect::<Vec<_>>(),
        [8, 10, 8, 10, 8, 12, 4, 6, 4]
    );
    assert_eq!(chart.points[0].frequency_hz, 100_250_000);
    assert_eq!(chart.points[8].frequency_hz, 100_450_000);
    assert!(capture.chart_range(8, 2).is_err());
    assert!(capture.chart_range(usize::MAX, 1).is_err());
}

#[test]
fn special_reserved_and_zero_assignments_are_not_data_modulation() {
    let mut schemes = Vec::new();
    for code in [0, 1, 2, 3, 4, 5, 6, 14, 16, 20, 255] {
        schemes.extend_from_slice(&[0, code, 0, 1]);
    }
    let capture = synthetic(&schemes, 0, 0, 25);
    let bits = capture
        .chart_range_for(view(0, ModulationMetric::BitLoading), 0, 11)
        .map(trace)
        .unwrap();
    assert_eq!(
        bits.points
            .iter()
            .map(|point| point.value)
            .collect::<Vec<_>>(),
        [
            None, // Zero-loaded is now displayed in its own carrier lane.
            None,
            Some(2.0),
            None,
            Some(4.0),
            None,
            Some(6.0),
            Some(14.0),
            None,
            None,
            None
        ]
    );
    let shannon = trace(capture.chart_range(0, 11).unwrap());
    assert!(shannon.points[0].value.is_none());
    assert!((shannon.points[2].value.unwrap() - 4.771212547196624).abs() < 1e-10);
    assert!((shannon.points[7].value.unwrap() - 42.14393431255206).abs() < 1e-10);
    let metadata = capture
        .chart_metadata()
        .unwrap()
        .modulation_profile
        .unwrap();
    assert!(metadata.assignment_summary.contains("Reserved (255): 1"));
    assert_eq!(metadata.profiles[0].bandwidth_bps, None);
    assert_eq!(metadata.profiles[0].bandwidth_text, "Unavailable");
    assert!(metadata.assignment_summary.contains("Excluded: 1"));
}

#[test]
fn empty_profiles_and_all_missing_values_have_safe_metadata() {
    let capture = synthetic(&[], 0, 0, 25);
    assert_eq!(capture.chart_metadata().unwrap().sample_count, 0);
    assert!(capture.chart_range(0, 0).unwrap().points.is_empty());
    let capture = synthetic(&[0, 16, 0, 10], 0, 0, 25);
    assert!(capture
        .chart_range(0, 10)
        .unwrap()
        .points
        .iter()
        .all(|point| point.value.is_none()));
}

#[test]
fn expansion_and_frequency_limits_fail_without_large_allocations() {
    let excessive = synthetic(&[0, 8, 255, 255, 0, 8, 0, 2], 0, 0, 25);
    assert!(excessive.chart_metadata().is_err());
    assert!(excessive.chart_range(0, 1).is_err());
    for capture in [
        synthetic(&[0, 8, 0, 1], 100, 0, 0),
        synthetic(&[0, 8, 0, 1], u32::MAX - 10, 1, 25),
        synthetic(&[0, 8, 0, 3], u32::MAX - 30_000, 0, 25),
    ] {
        assert!(capture.chart_range(0, 1).is_err());
    }
    let bounded = synthetic(&[0, 8, 16, 1], 0, 0, 25);
    let overview = trace(bounded.chart_range(0, 4097).unwrap());
    assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(overview.points.first().unwrap().sample_index, 0);
    assert_eq!(overview.points.last().unwrap().sample_index, 4096);
}

#[test]
fn all_profile_combinations_share_zoom_and_bounded_traces() {
    let capture = Capture::from_bytes(
        pnm_fixtures::MODULATION_PROFILE.to_vec(),
        "profiles.bin".into(),
    )
    .unwrap();
    let original_json = capture.json().unwrap();
    for mask in 0..16 {
        let selected: Vec<usize> = (0..4).filter(|index| mask & (1 << index) != 0).collect();
        for metric in [ModulationMetric::Shannon, ModulationMetric::BitLoading] {
            let view = MeasurementView::ModulationProfile {
                carrier_types: None,
                indices: Some(selected.clone()),
                metric,
            };
            let metadata = capture.chart_metadata_for(view.clone()).unwrap();
            assert_eq!(metadata.sample_count, 7480);
            assert_eq!(
                metadata.modulation_profile.unwrap().selected_indices,
                selected
            );
            for (start, count) in [(0, 7480), (17, 32), (7000, 480), (7480, 0)] {
                let viewport = capture.chart_range_for(view.clone(), start, count).unwrap();
                assert_eq!(
                    (viewport.start, viewport.count, viewport.total),
                    (start, count, 7480)
                );
                let series = viewport.modulation_profiles.unwrap().series;
                assert_eq!(series.len(), selected.len());
                assert!(
                    series
                        .iter()
                        .map(|series| series.points.len())
                        .sum::<usize>()
                        <= MAX_VIEW_SAMPLES
                );
                if count > 0 {
                    assert_eq!(
                        viewport.points.first().unwrap().frequency_hz,
                        640_000_000 + start as u32 * 25_000
                    );
                    assert_eq!(
                        viewport.points.last().unwrap().frequency_hz,
                        640_000_000 + (start + count - 1) as u32 * 25_000
                    );
                }
                for (series, source_index) in series.iter().zip(&selected) {
                    assert_eq!(series.profile_index, *source_index);
                    assert_eq!(series.profile_id, 3 - *source_index as u8);
                    assert!(series
                        .points
                        .iter()
                        .all(|point| point.sample_index >= start
                            && point.sample_index < start + count));
                    if metric == ModulationMetric::BitLoading && start == 17 {
                        assert_eq!(
                            series.points[0].value,
                            Some([12.0, 11.0, 10.0, 8.0][*source_index])
                        );
                    }
                }
            }
        }
    }
    assert_eq!(capture.json().unwrap(), original_json);
}

// Synthetic profiles vary in length to verify shared axes without padding data.
fn multiple_profiles(lengths: &[u16]) -> Capture {
    let mut bytes = pnm_fixtures::MODULATION_PROFILE[..29].to_vec();
    bytes[17] = lengths.len() as u8;
    bytes[25..29].copy_from_slice(&(lengths.len() as u32 * 7).to_be_bytes());
    for (index, length) in lengths.iter().enumerate() {
        bytes.extend_from_slice(&[index as u8, 0, 4, 0, 8]);
        bytes.extend_from_slice(&length.to_be_bytes());
    }
    Capture::from_bytes(bytes, "synthetic-profiles.bin".into()).unwrap()
}

#[test]
fn mixed_lengths_empty_selection_and_invalid_indices_are_explicit() {
    let capture = multiple_profiles(&[2, 4]);
    let view = MeasurementView::ModulationProfile {
        carrier_types: None,
        indices: Some(vec![0, 1]),
        metric: ModulationMetric::BitLoading,
    };
    let viewport = capture.chart_range_for(view.clone(), 0, 4).unwrap();
    let series = viewport.modulation_profiles.unwrap().series;
    assert_eq!(series[0].points.len(), 2);
    assert_eq!(series[1].points.len(), 4);
    assert_eq!(viewport.points.last().unwrap().frequency_hz, 640_075_000);
    let zoom = capture
        .chart_range_for(view, 2, 2)
        .unwrap()
        .modulation_profiles
        .unwrap()
        .series;
    assert!(zoom[0].points.is_empty());
    assert_eq!(zoom[1].points.len(), 2);
    let none = MeasurementView::ModulationProfile {
        carrier_types: None,
        indices: Some(vec![]),
        metric: ModulationMetric::BitLoading,
    };
    let metadata = capture.chart_metadata_for(none.clone()).unwrap();
    assert_eq!(metadata.sample_count, 4);
    assert_eq!(
        metadata.modulation_profile.unwrap().assignment_summary,
        "No profiles selected"
    );
    assert!(capture
        .chart_range_for(none, 0, 4)
        .unwrap()
        .modulation_profiles
        .unwrap()
        .series
        .is_empty());
    assert!(capture
        .chart_metadata_for(MeasurementView::ModulationProfile {
            carrier_types: None,
            indices: Some(vec![2]),
            metric: ModulationMetric::Shannon
        })
        .is_err());
}

#[test]
fn overlay_limits_cover_total_expansion_and_maximum_profile_count() {
    let excessive = multiple_profiles(&[40_000, 40_000]);
    let all = MeasurementView::ModulationProfile {
        carrier_types: None,
        indices: Some(vec![0, 1]),
        metric: ModulationMetric::BitLoading,
    };
    assert!(excessive.chart_metadata_for(all.clone()).is_err());
    assert!(excessive.chart_range_for(all, 0, 1).is_err());
    assert!(excessive
        .chart_range_for(view(0, ModulationMetric::BitLoading), 0, 1)
        .is_ok());
    let capture = multiple_profiles(&[1; 255]);
    let all = MeasurementView::ModulationProfile {
        carrier_types: None,
        indices: Some((0..255).collect()),
        metric: ModulationMetric::BitLoading,
    };
    let viewport = capture.chart_range_for(all, 0, 1).unwrap();
    let series = viewport.modulation_profiles.unwrap().series;
    assert_eq!(series.len(), 255);
    assert_eq!(
        series
            .iter()
            .map(|series| series.points.len())
            .sum::<usize>(),
        255
    );
}

#[test]
fn every_carrier_type_has_a_filterable_lane_and_exact_marker() {
    use viewer::modulation_profile::CarrierType;
    let capture = synthetic(
        &[
            0, 0, 0, 1, 0, 1, 0, 1, 0, 2, 0, 1, 0, 3, 0, 1, 0, 16, 0, 1, 0, 20, 0, 1, 0, 255, 0, 1,
        ],
        0,
        0,
        25,
    );
    let expected = [
        CarrierType::ZeroBitLoaded,
        CarrierType::ContinuousPilot,
        CarrierType::Data,
        CarrierType::Reserved,
        CarrierType::Exclusion,
        CarrierType::Plc,
        CarrierType::Unknown,
    ];
    for (index, kind) in expected.into_iter().enumerate() {
        let selected = MeasurementView::ModulationProfile {
            indices: Some(vec![0]),
            carrier_types: Some(vec![kind]),
            metric: ModulationMetric::BitLoading,
        };
        let viewport = capture
            .chart_range_for(selected.clone(), 0, 7)
            .unwrap()
            .modulation_profiles
            .unwrap();
        if kind == CarrierType::Data {
            assert!(viewport.markers.is_empty());
            assert_eq!(viewport.series[0].points[2].value, Some(2.0));
        } else {
            assert!(viewport.series[0].points.is_empty());
            assert_eq!(viewport.carrier_lanes, [kind]);
            assert_eq!(viewport.markers.len(), 1);
            assert_eq!(viewport.markers[0].carrier_type, kind);
            assert_eq!(viewport.markers[0].count, 1);
            assert_eq!(
                viewport.markers[0].first_frequency_hz,
                index as u32 * 25_000
            );
            assert_eq!(viewport.markers[0].last_frequency_hz, index as u32 * 25_000);
        }
        let metadata = capture
            .chart_metadata_for(selected)
            .unwrap()
            .modulation_profile
            .unwrap();
        assert_eq!(metadata.selected_carrier_types, [kind]);
        assert_eq!(
            metadata
                .carrier_types
                .iter()
                .find(|choice| choice.kind == kind)
                .unwrap()
                .count,
            1
        );
    }
    let hidden = MeasurementView::ModulationProfile {
        indices: Some(vec![0]),
        carrier_types: Some(vec![]),
        metric: ModulationMetric::Shannon,
    };
    let viewport = capture.chart_range_for(hidden, 0, 7).unwrap();
    assert_eq!(viewport.points.len(), 2);
    let overlay = viewport.modulation_profiles.unwrap();
    assert!(
        overlay.markers.is_empty()
            && overlay.carrier_lanes.is_empty()
            && overlay.series[0].points.is_empty()
    );
}

#[test]
fn grouped_carrier_markers_are_bounded_and_preserve_counts() {
    use viewer::modulation_profile::{CarrierType, MAX_CARRIER_MARKERS};
    let capture = synthetic(&[1, 1, 20, 255, 255], 0, 0, 25);
    let overlay = capture
        .chart_range(0, 65535)
        .unwrap()
        .modulation_profiles
        .unwrap();
    assert!(overlay.markers.len() <= MAX_CARRIER_MARKERS);
    assert_eq!(
        overlay
            .markers
            .iter()
            .filter(|marker| marker.carrier_type == CarrierType::ContinuousPilot)
            .map(|marker| marker.count)
            .sum::<usize>(),
        32768
    );
    assert_eq!(
        overlay
            .markers
            .iter()
            .filter(|marker| marker.carrier_type == CarrierType::Plc)
            .map(|marker| marker.count)
            .sum::<usize>(),
        32767
    );
    let detail = capture
        .chart_range(23, 4)
        .unwrap()
        .modulation_profiles
        .unwrap();
    assert_eq!(detail.markers.len(), 4);
    assert!(detail
        .markers
        .iter()
        .all(|marker| marker.count == 1 && marker.first_frequency_hz == marker.last_frequency_hz));
}
