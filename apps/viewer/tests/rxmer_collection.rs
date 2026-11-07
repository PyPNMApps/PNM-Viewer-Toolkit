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
    assert!(group.tree_label.ends_with(" MHz (index 0)"));
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
