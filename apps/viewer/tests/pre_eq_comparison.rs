/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use viewer::{
    measurements::upstream_pre_equalization::comparison::{
        bounded, bounded_range, write_json_new, CorrectionMode,
    },
    Capture, MAX_VIEW_SAMPLES,
};
#[test]
fn fixture_pair_precheck_roles_full_data_bounded_plots_and_exports() {
    let original = Capture::from_bytes(
        pnm_fixtures::UPSTREAM_PRE_EQ.to_vec(),
        "original.bin".into(),
    )
    .unwrap();
    assert!(original.supports_pre_eq_comparison());
    let equal = Capture::from_bytes(
        pnm_fixtures::UPSTREAM_PRE_EQ_LAST.to_vec(),
        "equal.bin".into(),
    )
    .unwrap();
    assert!(!equal.supports_pre_eq_comparison());
    assert!(original.pre_eq_precheck(&equal).is_err());
    let mut bytes = pnm_fixtures::UPSTREAM_PRE_EQ_LAST.to_vec();
    let time = u32::from_be_bytes(bytes[6..10].try_into().unwrap());
    bytes[6..10].copy_from_slice(&(time + 1).to_be_bytes());
    let update = Capture::from_bytes(bytes, "synthetic-later.bin".into()).unwrap();
    let result = original
        .pre_eq_compare(&update, CorrectionMode::Adjust)
        .unwrap();
    assert_eq!(result.points.len(), 1776);
    let plot = bounded(&result);
    assert!(plot.points.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(plot.points.first().unwrap().sample_index, 0);
    assert_eq!(plot.points.last().unwrap().sample_index, 1775);
    assert_eq!(plot.pair.sample_count, 1776);
    let zoom = bounded_range(&result, 100, 101).unwrap();
    assert_eq!(zoom.start, 100);
    assert_eq!(zoom.count, 101);
    assert_eq!(zoom.points.first().unwrap().sample_index, 100);
    assert_eq!(zoom.points.last().unwrap().sample_index, 200);
    assert_eq!(
        zoom.summary.original_magnitude_span_db,
        result.summary.original_magnitude_span_db
    );
    assert!(bounded_range(&result, 0, 0).is_err());
    assert!(bounded_range(&result, 1776, 1).is_err());
    assert_eq!(
        plot.summary.original_magnitude_span_db,
        result.summary.original_magnitude_span_db
    );
    let folder = tempfile::tempdir().unwrap();
    let path = folder.path().join("comparison.json");
    write_json_new(&result, &path).unwrap();
    assert_eq!(
        std::fs::read_to_string(&path).unwrap(),
        result.to_json(16 * 1024 * 1024).unwrap()
    );
    assert!(write_json_new(&result, &path).is_err());
    assert!(original.pre_eq_precheck(&original).is_err());
}
