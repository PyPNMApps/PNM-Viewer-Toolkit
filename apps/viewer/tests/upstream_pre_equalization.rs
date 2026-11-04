/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Current and last-update upstream capture views, range bounds and shared CLI/core export contract.
use pnm::pnm::parser::pnm_file::PnmFile;
use viewer::{chart::MeasurementView, Capture, MAX_VIEW_SAMPLES};
#[test]
fn current_views_are_bounded_zoomable_and_preserve_full_export() {
    let capture =
        Capture::from_bytes(pnm_fixtures::UPSTREAM_PRE_EQ.to_vec(), "current.bin".into()).unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Upstream Pre-EQ");
    assert_eq!(metadata.views, ["Magnitude", "Group delay", "I/Q scatter"]);
    assert_eq!(metadata.sample_count, 1776);
    assert!(metadata
        .chips
        .iter()
        .any(|chip| chip.title == "Capture" && chip.value == "Current"));
    let json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    let core = PnmFile::from_bytes(pnm_fixtures::UPSTREAM_PRE_EQ)
        .decode(Default::default())
        .unwrap();
    assert_eq!(
        json,
        serde_json::from_str::<serde_json::Value>(&core.to_json(16 * 1024 * 1024).unwrap())
            .unwrap()
    );
    for view in [MeasurementView::Default, MeasurementView::GroupDelay] {
        let metadata = capture.chart_metadata_for(view.clone()).unwrap();
        assert!(metadata.maximum > metadata.minimum);
        let overview = capture.chart_range_for(view.clone(), 0, 1776).unwrap();
        assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
        assert_eq!(overview.points.first().unwrap().frequency_hz, 39_900_000);
        assert_eq!(overview.points.last().unwrap().frequency_hz, 84_275_000);
        let detail = capture.chart_range_for(view.clone(), 1500, 276).unwrap();
        assert_eq!(detail.points.len(), 276);
        assert_eq!(detail.points[0].sample_index, 1500);
        let expected = if view == MeasurementView::GroupDelay {
            json["pre_equalization_analysis"]["group_delay"][1500]["delay_s"]
                .as_f64()
                .unwrap()
                * 1_000_000.0
        } else {
            json["pre_equalization_analysis"]["magnitude_db"][1500]
                .as_f64()
                .unwrap()
        };
        assert!((detail.points[0].value.unwrap() - expected).abs() < 1e-10);
        assert!(capture.chart_range_for(view.clone(), 1775, 2).is_err());
        assert!(capture.chart_range_for(view, usize::MAX, 1).is_err());
    }
    let density = capture
        .chart_range_for(MeasurementView::IqScatter, 0, 1776)
        .unwrap()
        .iq_density
        .unwrap();
    assert_eq!(density.valid_count, 1776);
    assert_eq!(density.excluded_count, 0);
    assert_eq!(
        density.cells.iter().map(|cell| cell.count).sum::<usize>(),
        1776
    );
    assert!(density.cells.len() <= MAX_VIEW_SAMPLES);
    assert!(capture
        .chart_range_for(MeasurementView::IqScatter, 1, 100)
        .is_err());
    assert!(capture
        .chart_metadata_for(MeasurementView::FecProfile(0))
        .is_err());
}
#[test]
fn last_update_views_use_their_own_metadata_and_shared_core_json() {
    let capture = Capture::from_bytes(
        pnm_fixtures::UPSTREAM_PRE_EQ_LAST.to_vec(),
        "last.bin".into(),
    )
    .unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Upstream Pre-EQ Last Update");
    assert_eq!(metadata.axis_label, "Update magnitude");
    assert!(metadata
        .chips
        .iter()
        .any(|chip| chip.title == "Capture" && chip.value == "Last update"));
    let json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    let core = PnmFile::from_bytes(pnm_fixtures::UPSTREAM_PRE_EQ_LAST)
        .decode(Default::default())
        .unwrap();
    assert_eq!(
        json,
        serde_json::from_str::<serde_json::Value>(&core.to_json(16 * 1024 * 1024).unwrap())
            .unwrap()
    );
    assert_eq!(
        json["iq"][0],
        serde_json::json!([0.03173828125, -0.169921875])
    );
    for view in [MeasurementView::Default, MeasurementView::GroupDelay] {
        let metadata = capture.chart_metadata_for(view.clone()).unwrap();
        assert_eq!(metadata.views, ["Magnitude", "Group delay", "I/Q scatter"]);
        assert_eq!(metadata.sample_count, 1776);
        assert!(metadata.minimum.is_finite() && metadata.maximum > metadata.minimum);
        if view == MeasurementView::GroupDelay {
            assert_eq!(
                (metadata.axis_label, metadata.unit),
                ("Update group delay", "µs")
            );
        }
        let overview = capture.chart_range_for(view.clone(), 0, 1776).unwrap();
        assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
        assert_eq!(overview.points.first().unwrap().frequency_hz, 39_900_000);
        assert_eq!(overview.points.last().unwrap().frequency_hz, 84_275_000);
        let detail = capture.chart_range_for(view.clone(), 1500, 276).unwrap();
        assert_eq!(detail.points.len(), 276);
        let expected = if view == MeasurementView::GroupDelay {
            json["pre_equalization_analysis"]["group_delay"][1500]["delay_s"]
                .as_f64()
                .unwrap()
                * 1_000_000.0
        } else {
            json["pre_equalization_analysis"]["magnitude_db"][1500]
                .as_f64()
                .unwrap()
        };
        assert!((detail.points[0].value.unwrap() - expected).abs() < 1e-10);
        assert!(capture.chart_range_for(view, usize::MAX, 2).is_err());
    }
    let density = capture
        .chart_range_for(MeasurementView::IqScatter, 0, 1776)
        .unwrap()
        .iq_density
        .unwrap();
    assert_eq!(density.valid_count, 1776);
    assert_eq!(density.excluded_count, 0);
    assert!(density.cells.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(
        density.cells.iter().map(|cell| cell.count).sum::<usize>(),
        1776
    );
    assert!(capture
        .chart_range_for(MeasurementView::IqScatter, 1, 100)
        .is_err());
    assert!(capture
        .chart_metadata_for(MeasurementView::FecProfile(0))
        .is_err());
}
