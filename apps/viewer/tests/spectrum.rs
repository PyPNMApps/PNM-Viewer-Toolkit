/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Spectrum chart contracts: fixture values plus explicitly synthetic edge cases.
use viewer::{Capture, MAX_VIEW_SAMPLES};

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
    assert!(capture.metadata().is_err()); // Legacy Tauri remains RxMER-only.
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
