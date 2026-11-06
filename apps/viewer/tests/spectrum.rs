/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Spectrum chart contracts: fixture values plus explicitly synthetic edge cases.
use viewer::capture::{Capture, MAX_VIEW_SAMPLES};

#[test]
fn spectrum_fixture_metadata_overview_detail_and_export() {
    let capture =
        Capture::from_bytes(pnm_fixtures::SPECTRUM.to_vec(), "spectrum.bin".into()).unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.title, "Spectrum Analyzer");
    assert_eq!(metadata.unit, "dBmV");
    assert_eq!(metadata.sample_count, 20736);
    assert!(!metadata.chips.iter().any(|chip| chip.title == "Channel"));
    assert!(metadata.minimum < -77.3);
    assert!(metadata.maximum > -22.8);
    let overview = capture.chart_range(0, 20736).unwrap();
    assert!(overview.points.len() <= MAX_VIEW_SAMPLES);
    assert_eq!(overview.points[0].frequency_hz, 296_264_648);
    assert_eq!(overview.points[0].value, Some(-22.8));
    let last = overview.points.last().unwrap();
    assert_eq!(last.frequency_hz, 903_735_351);
    assert_eq!(last.value, Some(-77.3));
    let detail = capture.chart_range(255, 2).unwrap();
    assert_eq!(detail.points[0].frequency_hz, 303_735_351);
    assert_eq!(detail.points[1].frequency_hz, 303_764_648);
    assert_eq!(detail.points[1].sample_index, 256);
    let json: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    let segments = json["amplitude_bin_segments_float"].as_array().unwrap();
    assert_eq!(segments.len(), 81);
    assert_eq!(segments[0][0], -22.8);
    assert_eq!(segments[80][255], -77.3);
    assert!(capture.metadata().is_err()); // The small RxMER metadata helper rejects other measurement types.
}

fn truncated_spectrum(sample_count: usize) -> Vec<u8> {
    let mut bytes = pnm_fixtures::SPECTRUM[..39 + sample_count * 2].to_vec();
    bytes[35..39].copy_from_slice(&((sample_count * 2) as u32).to_be_bytes());
    bytes
}

#[test]
fn partial_segment_keeps_original_spacing_without_stretching() {
    let capture = Capture::from_bytes(truncated_spectrum(257), "partial.bin".into()).unwrap();
    let view = capture.chart_range(0, 257).unwrap();
    assert_eq!(view.points.len(), 257);
    assert_eq!(view.points[256].frequency_hz, 303_764_648);
    assert_eq!(capture.chart_metadata().unwrap().sample_count, 257);
    assert!(capture.chart_range(257, 1).is_err());
    assert!(capture.chart_range(usize::MAX, 2).is_err());
}

#[test]
fn invalid_frequency_grids_fail_without_wrapping() {
    let mut zero_span = truncated_spectrum(256);
    zero_span[25..29].copy_from_slice(&0_u32.to_be_bytes());
    let capture = Capture::from_bytes(zero_span, "zero-span.bin".into()).unwrap();
    assert_eq!(
        capture.chart_metadata().err().unwrap().code,
        "InvalidFrequencyGrid"
    );
    let mut overflow = truncated_spectrum(256);
    overflow[17..21].copy_from_slice(&4_294_000_000_u32.to_be_bytes());
    overflow[21..25].copy_from_slice(&4_294_000_000_u32.to_be_bytes());
    let capture = Capture::from_bytes(overflow, "overflow.bin".into()).unwrap();
    assert_eq!(
        capture.chart_range(0, 256).err().unwrap().code,
        "InvalidFrequencyGrid"
    );
}

#[test]
fn signed_power_values_and_empty_capture_are_supported() {
    let mut bytes = truncated_spectrum(3);
    bytes[39..].copy_from_slice(&[0x80, 0x00, 0x00, 0x00, 0x7f, 0xff]);
    let capture = Capture::from_bytes(bytes, "signed.bin".into()).unwrap();
    let view = capture.chart_range(0, 3).unwrap();
    assert_eq!(
        view.points
            .iter()
            .map(|point| point.value)
            .collect::<Vec<_>>(),
        vec![Some(-327.68), Some(0.0), Some(327.67)]
    );
    let metadata = capture.chart_metadata().unwrap();
    assert!(metadata.minimum < -327.68 && metadata.maximum > 327.67);
    let empty = Capture::from_bytes(truncated_spectrum(0), "empty.bin".into()).unwrap();
    assert!(empty.chart_range(0, 0).unwrap().points.is_empty());
    assert_eq!(empty.chart_metadata().unwrap().sample_count, 0);
}

#[test]
fn rbw_metadata_and_export_preserve_decoded_fields() {
    let capture =
        Capture::from_bytes(pnm_fixtures::SPECTRUM.to_vec(), "spectrum.bin".into()).unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert_eq!(metadata.chips.len(), 2);
    assert_eq!(metadata.chips[1].title, "RBW");
    assert_eq!(metadata.chips[1].value, "32.227 kHz");
    let analysis = metadata.spectrum_analysis.unwrap();
    assert_eq!(analysis.bin_spacing_hz, 29296.875);
    assert_eq!(analysis.equivalent_noise_bandwidth_factor, 1.1);
    assert!((analysis.resolution_bandwidth_hz - 32226.5625).abs() < 1e-9);
    let mut exported: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    let derived = exported
        .as_object_mut()
        .unwrap()
        .remove("spectrum_analysis")
        .unwrap();
    assert_eq!(derived, serde_json::to_value(analysis).unwrap());
    let raw = pnm::pnm::parser::cm_spectrum_analysis::CmSpectrumAnalysis::parse(
        pnm_fixtures::SPECTRUM,
        pnm::pnm::parser::parse_limits::ParseLimits::default(),
    )
    .unwrap();
    assert_eq!(exported, serde_json::to_value(raw).unwrap());
    assert_eq!(exported["equivalent_noise_bandwidth"], 110);
}

#[test]
fn zero_noise_bandwidth_is_not_reported_as_a_valid_rbw() {
    let mut bytes = truncated_spectrum(256);
    bytes[31..33].copy_from_slice(&0_u16.to_be_bytes());
    let capture = Capture::from_bytes(bytes, "zero-enbw.bin".into()).unwrap();
    assert_eq!(
        capture.chart_metadata().err().unwrap().code,
        "InvalidBandwidth"
    );
    assert_eq!(capture.json().err().unwrap().code, "InvalidBandwidth");
}

#[test]
fn window_average_uses_original_bins_preserves_zoom_and_bounded_export() {
    use viewer::capture::chart::MeasurementView;
    let capture = Capture::from_bytes(truncated_spectrum(257), "average.bin".into()).unwrap();
    let before = capture.json().unwrap();
    let raw = capture.chart_range(0, 257).unwrap();
    let view = MeasurementView::SpectrumAverage { window_points: 3 };
    let full = capture.chart_range_for(view.clone(), 0, 257).unwrap();
    let averaged = full.spectrum_average.unwrap();
    assert_eq!(averaged.window_points, 3);
    let expected = 10.0
        * ((99..=101)
            .map(|index| 10.0_f64.powf(raw.points[index].value.unwrap() / 10.0))
            .sum::<f64>()
            / 3.0)
            .log10();
    assert!((averaged.points[100].value.unwrap() - expected).abs() < 1e-10);
    let zoom = capture.chart_range_for(view, 100, 1).unwrap();
    assert!((zoom.spectrum_average.unwrap().points[0].value.unwrap() - expected).abs() < 1e-10);
    assert_eq!(zoom.start, 100);
    assert_eq!(zoom.count, 1);
    assert_eq!(zoom.points[0].frequency_hz, raw.points[100].frequency_hz);
    assert_eq!(capture.json().unwrap(), before);
    assert!(capture
        .chart_range_for(
            MeasurementView::SpectrumAverage { window_points: 0 },
            0,
            257
        )
        .is_err());
    assert!(
        Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into())
            .unwrap()
            .chart_range_for(MeasurementView::SpectrumAverage { window_points: 3 }, 0, 1)
            .is_err()
    );
    let capture = Capture::from_bytes(pnm_fixtures::SPECTRUM.to_vec(), "full.bin".into()).unwrap();
    let full = capture
        .chart_range_for(
            MeasurementView::SpectrumAverage { window_points: 101 },
            0,
            20736,
        )
        .unwrap();
    assert!(full.points.len() + full.spectrum_average.unwrap().points.len() <= MAX_VIEW_SAMPLES);
}

#[test]
fn frequency_units_bounds_and_original_source_power() {
    let mut bytes = truncated_spectrum(257);
    // Synthetic constant 0-dBmV trace, preserving the fixture's frequency grid.
    bytes[39..].fill(0);
    let capture = Capture::from_bytes(bytes, "constant.bin".into()).unwrap();
    let original = capture.json().unwrap();
    for (start, stop, unit) in [
        (300_000_000.0, 303_000_000.0, "Hz"),
        (300_000.0, 303_000.0, "kHz"),
        (300.0, 303.0, "MHz"),
        (0.300, 0.303, "GHz"),
    ] {
        let band = capture.spectrum_band(start, stop, unit, true).unwrap();
        assert_eq!(band.start_hz, 300_000_000.0);
        assert_eq!(band.stop_hz, 303_000_000.0);
        assert_eq!(band.bandwidth_hz, 3_000_000.0);
        // Flat 0 dBmV: 10 log10(3 MHz / 32226.5625 Hz).
        assert!((band.level_dbmv.unwrap() - 19.689_072_714_815_868).abs() < 1e-9);
        assert!(band.sample_count > 2);
        assert!(capture
            .spectrum_band(start, stop, unit, false)
            .unwrap()
            .level_dbmv
            .is_none());
    }
    for (start, stop, unit) in [
        (1.0, 303.0, "MHz"),
        (300.0, 400.0, "MHz"),
        (303.0, 300.0, "MHz"),
        (300.0, 300.0, "MHz"),
        (f64::NAN, 303.0, "MHz"),
        (300.0, f64::INFINITY, "MHz"),
        (300.0, 303.0, "invalid"),
    ] {
        assert!(capture.spectrum_band(start, stop, unit, false).is_err());
    }
    assert!(capture
        .spectrum_band(300_000_000.0, 300_000_001.0, "Hz", false)
        .is_err());
    assert!(capture
        .spectrum_band(300_000_000.0, 300_000_001.0, "Hz", true)
        .is_ok());
    assert_eq!(capture.json().unwrap(), original);
    let rxmer = Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
    assert!(rxmer.spectrum_band(640.0, 650.0, "MHz", true).is_err());
}
