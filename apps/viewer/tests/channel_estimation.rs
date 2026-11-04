/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Fixture contracts and explicitly synthetic channel-response cases.
use pnm::pnm::parser::{
    cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef, parse_limits::ParseLimits,
};
use viewer::{chart::MeasurementView, Capture, MAX_VIEW_SAMPLES};

#[test]
fn fixture_views_are_bounded_and_export_preserves_raw_fields() {
    let capture = Capture::from_bytes(
        pnm_fixtures::CHANNEL_ESTIMATE.to_vec(),
        "channel.bin".into(),
    )
    .unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Downstream Channel Estimation");
    assert_eq!(metadata.views, ["Magnitude", "Group delay", "I/Q scatter"]);
    assert_eq!(metadata.sample_count, 7480);
    let overview = capture.chart_range(0, 7480).unwrap();
    assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(overview.points.first().unwrap().frequency_hz, 640_000_000);
    assert_eq!(overview.points.last().unwrap().frequency_hz, 826_975_000);
    let first = capture.chart_range(0, 1).unwrap().points[0].value.unwrap();
    // Independently evaluated from the fixture's first decoded S2.13 pair.
    assert!((first - 1.489266439357412).abs() < 1e-10);
    for view in [MeasurementView::Default, MeasurementView::GroupDelay] {
        let metadata = capture.chart_metadata_for(view).unwrap();
        assert!(metadata.minimum.is_finite() && metadata.maximum > metadata.minimum);
        let detail = capture.chart_range_for(view, 7000, 480).unwrap();
        assert_eq!(detail.points.len(), 480);
        assert_eq!(detail.points[0].sample_index, 7000);
    }
    let density = capture
        .chart_range_for(MeasurementView::IqScatter, 0, 7480)
        .unwrap()
        .iq_density
        .unwrap();
    assert!(density.cells.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(density.valid_count + density.excluded_count, 7480);
    assert_eq!(
        density.cells.iter().map(|cell| cell.count).sum::<usize>(),
        density.valid_count
    );
    assert!(capture
        .chart_range_for(MeasurementView::IqScatter, 1, 100)
        .is_err());
    let mut json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    let analysis = json
        .as_object_mut()
        .unwrap()
        .remove("channel_estimation_analysis")
        .unwrap();
    assert_eq!(analysis["magnitude_db"].as_array().unwrap().len(), 7480);
    assert_eq!(analysis["group_delay"].as_array().unwrap().len(), 7480);
    assert_eq!(analysis["frequencies_hz"][7479], 826_975_000);
    assert!(analysis.get("group_delay_us").is_none());
    for (index, point) in analysis["group_delay"]
        .as_array()
        .unwrap()
        .iter()
        .enumerate()
    {
        assert_eq!(point["frequency_hz"], analysis["frequencies_hz"][index]);
    }
    let first_delay_s = analysis["group_delay"][0]["delay_s"].as_f64().unwrap();
    let displayed = capture
        .chart_range_for(MeasurementView::GroupDelay, 0, 1)
        .unwrap()
        .points[0]
        .value
        .unwrap();
    assert!((first_delay_s * 1_000_000.0 - displayed).abs() < 1e-12);
    assert!(capture
        .json()
        .unwrap()
        .contains(&format!("\"delay_s\": {first_delay_s:e}")));
    let raw =
        CmDsOfdmChanEstimateCoef::parse(pnm_fixtures::CHANNEL_ESTIMATE, ParseLimits::default())
            .unwrap();
    assert_eq!(json, serde_json::to_value(raw).unwrap());
    assert!(capture.metadata().is_err()); // Retained Tauri view is still RxMER-only.
}
fn synthetic(samples: &[[i16; 2]]) -> Vec<u8> {
    let mut bytes = pnm_fixtures::CHANNEL_ESTIMATE[..28].to_vec();
    bytes[24..28].copy_from_slice(&((samples.len() * 4) as u32).to_be_bytes());
    for sample in samples {
        for component in sample {
            bytes.extend_from_slice(&component.to_be_bytes());
        }
    }
    bytes
}
#[test]
fn excluded_and_zero_pairs_are_not_fake_zero_db_or_delay() {
    let bytes = synthetic(&[
        [8192, 0],
        [i16::MIN, i16::MIN],
        [0, 0],
        [8192, 0],
        [0, -8192],
    ]);
    let capture = Capture::from_bytes(bytes, "synthetic.bin".into()).unwrap();
    let magnitude = capture.chart_range(0, 5).unwrap();
    assert_eq!(
        magnitude
            .points
            .iter()
            .map(|point| point.value)
            .collect::<Vec<_>>(),
        [Some(0.0), None, None, Some(0.0), Some(0.0)]
    );
    let delay = capture
        .chart_range_for(MeasurementView::GroupDelay, 0, 5)
        .unwrap();
    assert!(delay.points[..3].iter().all(|point| point.value.is_none()));
    assert!((delay.points[3].value.unwrap() - 10.0).abs() < 1e-12);
    let density = capture
        .chart_range_for(MeasurementView::IqScatter, 0, 5)
        .unwrap()
        .iq_density
        .unwrap();
    assert_eq!(density.valid_count, 4); // A zero pair still belongs in I/Q.
    assert_eq!(density.excluded_count, 1);
    let json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    assert!(json["channel_estimation_analysis"]["magnitude_db"][2].is_null());
    let delays = &json["channel_estimation_analysis"]["group_delay"];
    assert!(delays[0]["delay_s"].is_null());
    assert!(delays[1]["delay_s"].is_null());
    assert!(delays[2]["delay_s"].is_null());
    assert!((delays[3]["delay_s"].as_f64().unwrap() - 1e-5).abs() < 1e-16);
    assert_eq!(delays[3]["frequency_hz"], 640_075_000);
}
#[test]
fn empty_invalid_grid_and_wrong_tabs_are_handled() {
    let empty = Capture::from_bytes(synthetic(&[]), "empty.bin".into()).unwrap();
    assert_eq!(empty.chart_metadata().unwrap().sample_count, 0);
    assert!(empty.chart_range(0, 0).unwrap().points.is_empty());
    let density = empty
        .chart_range_for(MeasurementView::IqScatter, 0, 0)
        .unwrap()
        .iq_density
        .unwrap();
    assert!(density.cells.is_empty());
    assert_eq!((density.minimum, density.maximum), (-1.0, 1.0));
    let mut overflow = synthetic(&[[8192, 0]]);
    overflow[17..21].copy_from_slice(&u32::MAX.to_be_bytes());
    let capture = Capture::from_bytes(overflow, "overflow.bin".into()).unwrap();
    assert_eq!(
        capture.chart_metadata().err().unwrap().code,
        "InvalidFrequencyGrid"
    );
    let mut zero_spacing = synthetic(&[[8192, 0]]);
    zero_spacing[23] = 0;
    let capture = Capture::from_bytes(zero_spacing, "zero.bin".into()).unwrap();
    assert_eq!(capture.json().err().unwrap().code, "InvalidFrequencyGrid");
    let rxmer = Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
    assert!(rxmer
        .chart_metadata_for(MeasurementView::GroupDelay)
        .is_err());
    assert!(rxmer
        .chart_range_for(MeasurementView::IqScatter, 0, 7480)
        .is_err());
}
