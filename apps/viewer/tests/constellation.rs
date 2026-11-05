/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use viewer::{
    capture::{chart::MeasurementView, Capture, MAX_VIEW_SAMPLES},
    measurements::constellation::IqBounds,
};
fn capture(code: u16) -> Capture {
    let mut bytes = pnm_fixtures::CONSTELLATION.to_vec();
    bytes[21..23].copy_from_slice(&code.to_be_bytes());
    Capture::from_bytes(bytes, "constellation.bin".into()).unwrap()
}
#[test]
fn fixture_has_equal_axes_and_bounded_measured_ideal_overlay() {
    let capture = capture(7);
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Downstream Constellation Display");
    assert_eq!(metadata.sample_count, 8192);
    assert!(metadata
        .chips
        .iter()
        .any(|chip| chip.title == "QAM" && chip.value == "256-QAM"));
    let plot = capture.chart_range(0, 8192).unwrap().constellation.unwrap();
    assert!(plot.soft_decisions.len() + plot.hard_decisions.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(plot.decision_boundaries.len(), 30);
    for edge in &plot.decision_boundaries {
        for point in edge {
            assert!(point[0] >= plot.bounds.minimum_i && point[0] <= plot.bounds.maximum_i);
            assert!(point[1] >= plot.bounds.minimum_q && point[1] <= plot.bounds.maximum_q);
        }
    }
    assert_eq!((plot.soft_total, plot.soft_visible), (8192, 8192));
    assert_eq!((plot.hard_total, plot.hard_visible), (256, 256));
    assert_eq!(plot.soft_decisions[0], [0.376953125, -1.15625]);
    assert_eq!(
        *plot.soft_decisions.last().unwrap(),
        [1.146484375, 0.0859375]
    );
    assert_eq!(plot.bounds.minimum_i, plot.bounds.minimum_q);
    assert_eq!(plot.bounds.maximum_i, plot.bounds.maximum_q);
    assert!(capture.chart_range(1, 8191).is_err());
    assert!(capture
        .chart_metadata_for(MeasurementView::IqScatter)
        .is_err());
    let parser = pnm::pnm::parser::cm_ds_const_disp_meas::CmDsConstDispMeas::parse(
        pnm_fixtures::CONSTELLATION,
        pnm::pnm::parser::parse_limits::ParseLimits::default(),
    )
    .unwrap();
    assert_eq!(capture.json().unwrap(), parser.to_json(4_000_000).unwrap());
}
#[test]
fn zoom_filters_in_rust_and_restores_exact_detail_for_dense_qam() {
    let capture = capture(13);
    let bounds = IqBounds {
        minimum_i: -0.1,
        maximum_i: 0.1,
        minimum_q: -0.1,
        maximum_q: 0.1,
    };
    let plot = capture
        .constellation_range(bounds)
        .unwrap()
        .constellation
        .unwrap();
    assert_eq!(plot.hard_total, 16384);
    assert_eq!(plot.hard_visible, 100);
    assert_eq!(plot.hard_decisions.len(), 100);
    assert!(plot.soft_visible < 8192);
    for point in plot.soft_decisions.iter().chain(plot.hard_decisions.iter()) {
        assert!(point.iter().all(|value| value.abs() <= 0.1));
    }
    let reversed = IqBounds {
        minimum_i: 0.1,
        maximum_i: -0.1,
        minimum_q: 0.05,
        maximum_q: -0.05,
    };
    let plot = capture
        .constellation_range(reversed)
        .unwrap()
        .constellation
        .unwrap();
    assert!(
        (plot.bounds.maximum_i
            - plot.bounds.minimum_i
            - (plot.bounds.maximum_q - plot.bounds.minimum_q))
            .abs()
            < 1e-14
    );
    let empty = capture
        .constellation_range(IqBounds {
            minimum_i: 10.0,
            maximum_i: 11.0,
            minimum_q: 10.0,
            maximum_q: 11.0,
        })
        .unwrap()
        .constellation
        .unwrap();
    assert!(empty.soft_decisions.is_empty() && empty.hard_decisions.is_empty());
}
#[test]
fn unknown_modulation_empty_samples_and_invalid_bounds_remain_explicit() {
    let capture = capture(u16::MAX);
    let plot = capture.chart_range(0, 8192).unwrap().constellation.unwrap();
    assert!(!plot.reference_available);
    assert!(plot.decision_boundaries.is_empty());
    assert!(plot.hard_decisions.is_empty());
    assert_eq!(plot.soft_total, 8192);
    for bounds in [
        IqBounds {
            minimum_i: f64::NAN,
            maximum_i: 1.0,
            minimum_q: 0.0,
            maximum_q: 1.0,
        },
        IqBounds {
            minimum_i: 0.0,
            maximum_i: 0.0,
            minimum_q: 0.0,
            maximum_q: 1.0,
        },
        IqBounds {
            minimum_i: -f64::MAX,
            maximum_i: f64::MAX,
            minimum_q: 0.0,
            maximum_q: 1.0,
        },
    ] {
        assert!(capture.constellation_range(bounds).is_err());
    }
    let mut bytes = pnm_fixtures::CONSTELLATION[..30].to_vec();
    bytes[23..25].fill(0);
    bytes[26..30].fill(0);
    let empty = Capture::from_bytes(bytes, "empty.bin".into()).unwrap();
    let plot = empty.chart_range(0, 0).unwrap().constellation.unwrap();
    assert_eq!(plot.soft_total, 0);
    assert!(plot.soft_decisions.is_empty());
    assert_eq!(plot.hard_total, 256);
}
