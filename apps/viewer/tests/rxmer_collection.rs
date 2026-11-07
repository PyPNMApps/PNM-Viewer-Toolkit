/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Batch container, compatibility, limit and full-resolution export contracts.
use std::{io::Write, path::Path, sync::atomic::AtomicBool};
use viewer::measurements::rxmer::collection::{self, Collection, ImportLimits};
fn capture(epoch: u32, channel: u8) -> Vec<u8> {
    let mut bytes = pnm_fixtures::RXMER.to_vec();
    bytes[6..10].copy_from_slice(&epoch.to_be_bytes());
    bytes[10] = channel;
    bytes
}
fn scan(path: &Path) -> Collection {
    Collection::scan(path, ImportLimits::default(), &AtomicBool::new(false)).unwrap()
}
fn tar_bytes() -> Vec<u8> {
    let mut builder = tar::Builder::new(Vec::new());
    for (name, epoch) in [("wrong-channel-999.bin", 20), ("nested/z.bin", 10)] {
        let bytes = capture(epoch, 34);
        let mut header = tar::Header::new_gnu();
        header.set_size(bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        builder
            .append_data(&mut header, name, bytes.as_slice())
            .unwrap();
    }
    builder.into_inner().unwrap()
}
#[test]
fn mixed_directory_groups_by_headers_deduplicates_and_sorts_epochs() {
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(directory.path().join("z-late.bin"), capture(20, 34)).unwrap();
    std::fs::write(directory.path().join("a-early.bin"), capture(10, 34)).unwrap();
    std::fs::write(directory.path().join("same-copy.bin"), capture(10, 34)).unwrap();
    std::fs::write(
        directory.path().join("rxmer-channel-34.bin"),
        capture(30, 35),
    )
    .unwrap();
    std::fs::write(
        directory.path().join("not-rxmer.bin"),
        pnm_fixtures::HISTOGRAM,
    )
    .unwrap();
    let collection = scan(directory.path());
    assert_eq!(collection.catalog.accepted_count, 3);
    assert_eq!(collection.catalog.duplicate_count, 1);
    assert_eq!(collection.catalog.skipped_count, 1);
    assert_eq!(collection.catalog.groups.len(), 2);
    let group = &collection.catalog.groups[0];
    assert!(group.tree_label.starts_with("Channel 34 · "));
    assert!(group.tree_label.ends_with(" MHz"));
    assert_eq!(
        group
            .sources
            .iter()
            .map(|source| source.capture_time)
            .collect::<Vec<_>>(),
        vec![10, 20]
    );
    let aggregate = collection.aggregate(0).unwrap();
    assert_eq!(aggregate.sources[0].capture_time, 10);
    assert_eq!(aggregate.sources[1].capture_time, 20);
    assert_eq!(aggregate.group.channel_id, 34);
    assert!(collection.aggregate(2).is_err());
    let view = collection::viewport(&aggregate, 0, aggregate.points.len()).unwrap();
    assert!(view["points"].as_array().unwrap().len() <= 1024);
    assert_eq!(
        view["points"].as_array().unwrap().len(),
        view["rxmer_statistics"]["minimum"]
            .as_array()
            .unwrap()
            .len()
    );
    let output = directory.path().join("statistics.json");
    collection::write_json_new(&aggregate, &output).unwrap();
    assert!(collection::write_json_new(&aggregate, &output).is_err());
    let json: serde_json::Value = serde_json::from_slice(&std::fs::read(output).unwrap()).unwrap();
    assert_eq!(json["points"].as_array().unwrap().len(), 7480);
    assert_eq!(json["average_method"], "arithmetic_db_valid_readings");
    assert!(collection::viewport(&aggregate, usize::MAX, 2).is_err());
}
#[test]
fn tar_gzip_zip_and_zstd_members_are_content_detected() {
    let directory = tempfile::tempdir().unwrap();
    let tar = tar_bytes();
    let gzip = {
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), Default::default());
        encoder.write_all(&tar).unwrap();
        encoder.finish().unwrap()
    };
    let zstd = zstd::stream::encode_all(capture(11, 34).as_slice(), 1).unwrap();
    for (name, bytes, count) in [
        ("misnamed.data", tar, 2),
        ("compressed.anything", gzip, 2),
        ("zstd.data", zstd, 1),
    ] {
        let path = directory.path().join(name);
        std::fs::write(&path, bytes).unwrap();
        assert_eq!(scan(&path).catalog.accepted_count, count);
    }
    let path = directory.path().join("bundle.zip");
    let mut archive = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
    archive
        .start_file(
            "nested/update.zst",
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
    archive
        .write_all(&zstd::stream::encode_all(capture(15, 34).as_slice(), 1).unwrap())
        .unwrap();
    archive
        .start_file("../escape.bin", zip::write::SimpleFileOptions::default())
        .unwrap();
    archive.write_all(&capture(16, 34)).unwrap();
    archive.finish().unwrap();
    let imported = scan(&path);
    assert_eq!(imported.catalog.accepted_count, 1);
    assert_eq!(imported.catalog.skipped_count, 1);
    assert!(!directory
        .path()
        .parent()
        .unwrap()
        .join("escape.bin")
        .exists());
}
#[test]
fn import_limits_cancellation_and_corrupt_containers_fail_explicitly() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("capture.bin");
    std::fs::write(&path, capture(1, 34)).unwrap();
    let stopped = AtomicBool::new(true);
    assert!(Collection::scan(&path, ImportLimits::default(), &stopped).is_err());
    for limits in [
        ImportLimits {
            max_file_bytes: 10,
            ..Default::default()
        },
        ImportLimits {
            max_expanded_bytes: 10,
            ..Default::default()
        },
        ImportLimits {
            max_samples: 1,
            ..Default::default()
        },
        ImportLimits {
            max_entries: 0,
            ..Default::default()
        },
    ] {
        assert!(Collection::scan(&path, limits, &AtomicBool::new(false)).is_err());
    }
    std::fs::write(&path, [0x1f, 0x8b, 0]).unwrap();
    assert!(Collection::scan(&path, Default::default(), &AtomicBool::new(false)).is_err());
    std::fs::write(&path, b"PK\x03\x04bad").unwrap();
    assert!(Collection::scan(&path, Default::default(), &AtomicBool::new(false)).is_err());
}

#[test]
fn oversized_archive_names_do_not_expand_tree_metadata_without_bounds() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("long-name.zip");
    let mut archive = zip::ZipWriter::new(std::fs::File::create(&path).unwrap());
    archive
        .start_file(
            format!("{}.bin", "x".repeat(4097)),
            zip::write::SimpleFileOptions::default(),
        )
        .unwrap();
    archive.write_all(&capture(1, 34)).unwrap();
    archive.finish().unwrap();
    let result = Collection::scan(&path, ImportLimits::default(), &AtomicBool::new(false));
    assert!(matches!(result,Err(error) if error.code=="ImportLimit"));
}

#[test]
fn playback_uses_header_epoch_order_and_preserves_exact_carrier_zoom() {
    let directory = tempfile::tempdir().unwrap();
    let mut early = capture(10, 34);
    // Synthetic unavailable carrier, low trough and high peak.
    early[28] = 0xff;
    early[29] = 4;
    early[30] = 200;
    std::fs::write(directory.path().join("z-early.bin"), &early).unwrap();
    std::fs::write(directory.path().join("a-late.bin"), capture(30, 34)).unwrap();
    let mut tied = capture(10, 34);
    tied[29] = 8;
    std::fs::write(directory.path().join("a-tied.bin"), tied).unwrap();
    let collection = scan(directory.path());
    let first = collection.frame(0, 0).unwrap();
    assert_eq!(first.sources.len(), 1);
    assert_eq!(first.sources[0].capture_time, 10);
    assert!(first.sources[0].name.ends_with("a-tied.bin"));
    assert_eq!(collection.frame(0, 2).unwrap().sources[0].capture_time, 30);
    let frame = collection.frame(0, 1).unwrap();
    assert_eq!(frame.points[0].average_db, None);
    assert_eq!(frame.points[1].average_db, Some(1.0));
    assert_eq!(frame.points[2].average_db, Some(50.0));
    let full = collection::viewport(&frame, 0, frame.points.len()).unwrap();
    assert!(full["points"].as_array().unwrap().len() <= 1024);
    for frame_index in 0..3 {
        let frame = collection.frame(0, frame_index).unwrap();
        let view = collection::viewport(&frame, 17, 200).unwrap();
        assert_eq!(view["start"], 17);
        assert_eq!(view["count"], 200);
        assert_eq!(view["points"][0]["sample_index"], 17);
    }
    assert!(collection.frame(1, 0).is_err());
    assert!(collection.frame(0, 3).is_err());
    assert!(collection.frame(usize::MAX, usize::MAX).is_err());
    assert_eq!(collection.aggregate(0).unwrap().sources.len(), 3);
}

#[test]
fn playback_average_uses_all_valid_carriers_independently_of_zoom() {
    let directory = tempfile::tempdir().unwrap();
    let mut bytes = capture(10, 34);
    bytes[28..].fill(0xff);
    bytes[29] = 40; // 10 dB, synthetic RxMER.
    bytes[30] = 120; // 30 dB.
    bytes[31] = 200; // 50 dB.
    std::fs::write(directory.path().join("frame.bin"), &bytes).unwrap();
    let frame = scan(directory.path()).frame(0, 0).unwrap();
    for (start, count) in [(0, frame.points.len()), (1, 1), (100, 20)] {
        let view = collection::playback_viewport(&frame, 0, 0, 1, start, count).unwrap();
        assert_eq!(view["average_mer_db"], 30.0);
        assert_eq!(view["capture_epoch"], 10);
        assert_eq!(view["frame_count"], 1);
        assert!(view.get("rxmer_statistics").is_none());
    }
    bytes[28..].fill(0xff);
    std::fs::write(directory.path().join("frame.bin"), bytes).unwrap();
    let frame = scan(directory.path()).frame(0, 0).unwrap();
    let view = collection::playback_viewport(&frame, 0, 0, 1, 0, frame.points.len()).unwrap();
    assert!(view["average_mer_db"].is_null());
}
