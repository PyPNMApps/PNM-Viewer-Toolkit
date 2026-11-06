/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native analysis snapshots match core JSON; display sampling cannot hide limits.
use pnm::pnm::analysis::rxmer::RxMerAnalysisRequest;
use viewer::{
    capture::{Capture, MAX_VIEW_SAMPLES},
    measurements::rxmer::{bounded, write_json_new},
};
#[test]
fn native_zoom_uses_all_original_carriers_and_exports_the_same_snapshot() {
    let capture = Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
    let analysis = capture
        .rxmer_bit_loading(400, 1001, Default::default())
        .unwrap();
    assert_eq!(analysis.rate.estimated_gross_rate_bps, Some(250250000));
    let json = analysis.to_json(16 * 1024 * 1024).unwrap();
    let plot = bounded(analysis);
    assert!(plot.analysis.subcarriers.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(plot.analysis.summary.uniform_bit_loading, Some(10));
    assert!(capture.rxmer_bit_loading(0, 0, Default::default()).is_err());
    assert!(capture
        .rxmer_bit_loading(7480, 1, Default::default())
        .is_err());
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("analysis.json");
    let original = capture
        .rxmer_bit_loading(400, 1001, Default::default())
        .unwrap();
    write_json_new(&original, &path).unwrap();
    assert_eq!(std::fs::read_to_string(&path).unwrap(), json);
    assert!(write_json_new(&original, &path).is_err());
    assert_eq!(std::fs::read_to_string(path).unwrap(), json);
}
#[test]
fn bounded_plot_preserves_a_narrow_notch_and_breaks_over_omitted_missing_data() {
    let mut bytes = pnm_fixtures::RXMER[..28].to_vec();
    let mut values = vec![180; 5000];
    values[2049] = 132;
    values[1501] = 255;
    bytes[24..28].copy_from_slice(&5000_u32.to_be_bytes());
    bytes.extend(values);
    let capture = Capture::from_bytes(bytes, "synthetic.bin".into()).unwrap();
    let plot = bounded(
        capture
            .rxmer_bit_loading(0, 5000, RxMerAnalysisRequest::default())
            .unwrap(),
    );
    assert!(plot.analysis.subcarriers.len() <= MAX_VIEW_SAMPLES);
    assert!(plot
        .analysis
        .subcarriers
        .iter()
        .any(|point| point.sample_index == 2049 && point.bit_loading == Some(8)));
    assert!(plot
        .analysis
        .subcarriers
        .iter()
        .any(|point| point.sample_index == 1501 && point.rxmer_db.is_none()));
    assert!(!plot.plot_gaps.is_empty());
    assert_eq!(plot.analysis.summary.uniform_bit_loading, Some(8));
    assert_eq!(plot.analysis.coverage.measured_count, 4999);
}

#[test]
fn full_capture_analysis_keeps_the_weakest_carrier_and_can_use_lower_qam_targets() {
    use viewer::measurements::rxmer::BitLoadingModel;
    let capture = Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
    let viewport = capture.chart_range(0, 7480).unwrap();
    assert_eq!(viewport.count, 7480);
    assert!(viewport.points.len() <= MAX_VIEW_SAMPLES);
    let webui = capture
        .rxmer_bit_loading(
            viewport.start,
            viewport.count,
            RxMerAnalysisRequest {
                model: BitLoadingModel::WebuiMerHeuristic,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(webui.summary.minimum_rxmer_db, Some(28.25));
    assert_eq!(webui.summary.limiting_frequency_hz, Some(750_000_000));
    assert_eq!(webui.summary.status, "below_model_minimum");
    assert_eq!(webui.rate.estimated_gross_rate_bps, None);
    let plot = bounded(webui);
    assert!(plot.analysis.subcarriers.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(plot.analysis.selection.sample_count, 7480);
    assert!(plot
        .analysis
        .subcarriers
        .iter()
        .any(|point| point.frequency_hz == 750_000_000 && point.rxmer_db == Some(28.25)));
    let ossi = capture
        .rxmer_bit_loading(
            viewport.start,
            viewport.count,
            RxMerAnalysisRequest {
                model: BitLoadingModel::OssiCandidateDefaults,
                ..Default::default()
            },
        )
        .unwrap();
    assert_eq!(ossi.summary.uniform_modulation, Some("qam_256"));
    assert_eq!(ossi.rate.estimated_gross_rate_bps, Some(1_496_000_000));
}
