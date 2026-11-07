/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Standard-only import, chronology, settings isolation and full-resolution tools.
use std::{
    io::Write,
    sync::{atomic::AtomicBool, Arc},
};
use viewer::measurements::spectrum_analysis::collection::{Collection, ImportLimits, Session};
fn capture(epoch: u32, window: u16) -> Vec<u8> {
    let mut bytes = pnm_fixtures::SPECTRUM.to_vec();
    bytes[6..10].copy_from_slice(&epoch.to_be_bytes());
    bytes[33..35].copy_from_slice(&window.to_be_bytes());
    bytes
}
fn scan(path: &std::path::Path) -> Collection {
    Collection::scan(path, ImportLimits::default(), &AtomicBool::new(false)).unwrap()
}
#[test]
fn import_uses_headers_settings_epochs_and_skips_nonstandard_payloads() {
    let directory = tempfile::tempdir().unwrap();
    for (name, bytes) in [
        ("z-name-9999.bin", capture(100, 1)),
        ("a-name-1.bin", capture(200, 1)),
        ("copy.bin", capture(100, 1)),
        ("same-range-different-window.bin", capture(300, 2)),
        ("rxmer.bin", pnm_fixtures::RXMER.to_vec()),
        ("snmp-private.bin", vec![1; 440]),
    ] {
        std::fs::write(directory.path().join(name), bytes).unwrap();
    }
    let collection = scan(directory.path());
    assert_eq!(collection.catalog.accepted_count, 3);
    assert_eq!(collection.catalog.duplicate_count, 1);
    assert_eq!(collection.catalog.skipped_count, 2);
    assert_eq!(collection.catalog.groups.len(), 2);
    assert_eq!(collection.catalog.groups[0].sources[0].capture_time, 100);
    assert_eq!(collection.catalog.groups[0].sources[1].capture_time, 200);
    assert!(collection.catalog.groups[0]
        .tree_label
        .starts_with("Settings 1"));
}
#[test]
fn archive_content_detection_limits_cancel_and_corrupt_inputs() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("not-a-zip-extension.bin");
    let file = std::fs::File::create(&path).unwrap();
    let mut zip = zip::ZipWriter::new(file);
    zip.start_file("capture.bin", zip::write::SimpleFileOptions::default())
        .unwrap();
    zip.write_all(&capture(10, 1)).unwrap();
    zip.finish().unwrap();
    assert_eq!(scan(&path).catalog.accepted_count, 1);
    assert!(Collection::scan(&path, ImportLimits::default(), &AtomicBool::new(true)).is_err());
    assert!(Collection::scan(
        &path,
        ImportLimits {
            max_samples: 1,
            ..Default::default()
        },
        &AtomicBool::new(false)
    )
    .is_err());
    std::fs::write(&path, b"PK\x03\x04broken").unwrap();
    assert!(Collection::scan(&path, ImportLimits::default(), &AtomicBool::new(false)).is_err());
}
#[test]
fn range_average_power_tabs_playback_export_are_source_resolution() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("1.bin"), capture(100, 1)).unwrap();
    std::fs::write(directory.path().join("2.bin"), capture(200, 1)).unwrap();
    let collection = Arc::new(scan(directory.path()));
    let mut session = Session::new(collection, 0, None).unwrap();
    let total = session.result.points.len();
    let view = session.viewport().unwrap();
    assert!(view["spectrum_statistics"].is_object());
    assert!(view["points"].as_array().unwrap().len() <= 1024);
    let first = f64::from(session.result.points[0].frequency_hz);
    let last = f64::from(session.result.points[200].frequency_hz);
    assert!(session.band(first - 1.0, last, "Hz").is_err());
    assert!(session.band(first, last, "invalid").is_err());
    session.open_band(first / 1e6, last / 1e6, "MHz").unwrap();
    session.average(5).unwrap();
    session.power(0.1, 0.9).unwrap();
    let band = session.viewport().unwrap();
    assert_eq!(band["workspace"]["active"], 1);
    assert!(band["workspace"]["power"]["level_dbmv"].is_number());
    assert_eq!(band["spectrum_average"]["window_points"], 5);
    session.show_frame(1).unwrap();
    let frame = session.viewport().unwrap();
    assert_eq!(frame["capture_epoch"], 200);
    assert!(frame["spectrum_statistics"].is_null());
    assert_eq!(frame["spectrum_average"]["window_points"], 5);
    assert_eq!(frame["workspace"]["active"], 1);
    session.tab(0, false);
    assert_eq!(session.viewport().unwrap()["total"], total);
    session.tab(1, true);
    assert_eq!(
        session.viewport().unwrap()["workspace"]["tabs"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(session.show_frame(99).is_err());
    assert!(session.range(total, 1).is_err());
    assert!(session.average(0).is_err());
    let output = directory.path().join("result.json");
    viewer::capture::collection::write_json_new(&session.result, &output).unwrap();
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(&output).unwrap()).unwrap();
    assert_eq!(json["points"].as_array().unwrap().len(), total);
    assert!(viewer::capture::collection::write_json_new(&session.result, &output).is_err());
}
