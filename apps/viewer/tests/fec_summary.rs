/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use pnm::pnm::parser::{cm_ds_ofdm_fec_summary::CmDsOfdmFecSummary, parse_limits::ParseLimits};
use viewer::capture::{chart::MeasurementView, Capture, MAX_VIEW_SAMPLES};

// Synthetic profiles reuse only the fixture's validated identity/common fields.
fn synthetic(profiles: &[(u8, Vec<[u32; 4]>)], summary_type: u8) -> Capture {
    let mut bytes = pnm_fixtures::FEC_SUMMARY[..15].to_vec();
    bytes[13] = summary_type;
    bytes[14] = profiles.len() as u8;
    for (profile_id, entries) in profiles {
        bytes.push(*profile_id);
        bytes.extend_from_slice(&(entries.len() as u16).to_be_bytes());
        for entry in entries {
            for field in entry {
                bytes.extend_from_slice(&field.to_be_bytes());
            }
        }
    }
    Capture::from_bytes(bytes, "synthetic-fec.bin".into()).unwrap()
}

#[test]
fn fixture_profiles_counts_and_raw_json_are_preserved() {
    let capture =
        Capture::from_bytes(pnm_fixtures::FEC_SUMMARY.to_vec(), "fec_summary.bin".into()).unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Downstream FEC Summary");
    assert_eq!(metadata.sample_count, 600);
    assert_eq!(metadata.chips[0].value, "160");
    let choices = metadata.fec_summary.unwrap();
    assert_eq!(
        choices
            .profiles
            .iter()
            .map(|profile| profile.profile_id)
            .collect::<Vec<_>>(),
        vec![255, 0, 1, 2, 3]
    );
    assert_eq!(choices.profile_index, 1);
    assert_eq!(choices.summary_type, 2);
    let default = capture.chart_range(0, 600).unwrap().fec_summary.unwrap();
    assert_eq!(default.total_codewords, 23_724_950);
    assert_eq!(default.corrected, 23_724_863);
    assert_eq!(default.uncorrectable, 0);
    assert_eq!(default.first_timestamp, Some(1_762_636_604));
    assert_eq!(default.last_timestamp, Some(1_762_637_203));
    assert!(!default.decimated);
    assert!(default.points.iter().all(|point| !point.break_before));
    let sums = [26_666_584, 23_724_950, 402, 0, 95];
    for (index, sum) in sums.into_iter().enumerate() {
        let viewport = capture
            .chart_range_for(MeasurementView::FecProfile(index), 0, 600)
            .unwrap()
            .fec_summary
            .unwrap();
        assert_eq!(viewport.total_codewords, sum);
        assert_eq!(viewport.points.len(), 600);
    }
    let raw = CmDsOfdmFecSummary::parse(pnm_fixtures::FEC_SUMMARY, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.json().unwrap()).unwrap(),
        serde_json::to_value(raw).unwrap()
    );
    assert!(capture
        .chart_metadata_for(MeasurementView::FecProfile(5))
        .is_err());
    assert!(capture
        .chart_metadata_for(MeasurementView::GroupDelay)
        .is_err());
}

#[test]
fn utc_selection_sorts_for_display_preserves_raw_order_and_gaps() {
    let capture = synthetic(
        &[(7, vec![[120, 30, 3, 1], [100, 10, 1, 0], [101, 20, 2, 0]])],
        2,
    );
    let full = capture.chart_range(0, 3).unwrap().fec_summary.unwrap();
    assert_eq!(
        full.points
            .iter()
            .map(|point| point.record.timestamp)
            .collect::<Vec<_>>(),
        vec![100, 101, 120]
    );
    assert!(!full.points[1].break_before);
    assert!(full.points[2].break_before);
    assert_eq!(full.last.unwrap().timestamp, 120);
    let selected = capture
        .fec_time_range(MeasurementView::Default, 101, 100)
        .unwrap();
    assert_eq!(selected.count, 2);
    assert_eq!(selected.fec_summary.unwrap().total_codewords, 30);
    let empty = capture
        .fec_time_range(MeasurementView::Default, 102, 119)
        .unwrap();
    assert_eq!(empty.count, 0);
    assert!(empty.fec_summary.unwrap().last.is_none());
    let raw: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    assert_eq!(
        raw["fec_summary_data"][0]["codeword_entries"][0]["timestamp"],
        120
    );
    assert!(capture.chart_range(2, 2).is_err());
    assert!(capture.chart_range(usize::MAX, 1).is_err());
}

#[test]
fn overview_preserves_all_series_extrema_and_exact_large_sums() {
    let mut rows: Vec<_> = (0..4097)
        .map(|index| [index + 100, u32::MAX, 0, 0])
        .collect();
    rows[17][2] = 300;
    rows[18][3] = 700;
    rows[19][1] = 1;
    // Gap inside a bucket must survive even when its immediate neighbors are skipped.
    for entry in &mut rows[2000..] {
        entry[0] += 10;
    }
    let capture = synthetic(&[(0, rows)], 2);
    let overview = capture.chart_range(0, 4097).unwrap().fec_summary.unwrap();
    assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
    assert!(overview.decimated);
    assert_eq!(overview.total_codewords, 17_592_186_040_321);
    assert_eq!(overview.corrected, 300);
    assert_eq!(overview.uncorrectable, 700);
    assert_eq!(overview.error_maximum, 700);
    for sample in [0, 17, 18, 19, 4096] {
        assert!(overview
            .points
            .iter()
            .any(|point| point.sample_index == sample));
    }
    assert!(overview.points.iter().any(|point| point.break_before));
    let detail = capture.chart_range(16, 5).unwrap().fec_summary.unwrap();
    assert_eq!(detail.points.len(), 5);
    assert!(!detail.decimated);
}

#[test]
fn empty_zero_single_duplicate_and_unknown_summary_are_safe() {
    for capture in [synthetic(&[], 2), synthetic(&[(4, vec![])], 2)] {
        assert_eq!(capture.chart_metadata().unwrap().sample_count, 0);
        let viewport = capture.chart_range(0, 0).unwrap().fec_summary.unwrap();
        assert_eq!(viewport.total_maximum, 1);
        assert_eq!(viewport.error_maximum, 1);
        assert_eq!(viewport.total_codewords, 0);
        assert!(viewport.points.is_empty());
    }
    let capture = synthetic(&[(4, vec![[u32::MAX, 0, 0, 0]])], 1);
    let selected = capture
        .fec_time_range(MeasurementView::Default, u32::MAX, u32::MAX)
        .unwrap()
        .fec_summary
        .unwrap();
    assert_eq!(selected.points.len(), 1);
    assert_eq!(selected.last_timestamp, Some(u32::MAX));
    let capture = synthetic(&[(0, vec![[1, 2, 1, 0], [1, 3, 0, 0], [100, 4, 0, 1]])], 1);
    let selected = capture
        .fec_time_range(MeasurementView::Default, 1, 1)
        .unwrap()
        .fec_summary
        .unwrap();
    assert_eq!(selected.total_codewords, 5);
    assert_eq!(selected.points.len(), 2);
    assert!(capture
        .chart_range(0, 3)
        .unwrap()
        .fec_summary
        .unwrap()
        .points
        .iter()
        .all(|point| !point.break_before));
    let minute = synthetic(
        &[(0, vec![[60, 1, 0, 0], [120, 1, 0, 0], [240, 1, 0, 0]])],
        3,
    );
    let points = minute
        .chart_range(0, 3)
        .unwrap()
        .fec_summary
        .unwrap()
        .points;
    assert!(!points[1].break_before);
    assert!(points[2].break_before);
}
