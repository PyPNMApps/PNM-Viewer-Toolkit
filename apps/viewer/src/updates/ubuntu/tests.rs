/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Adversarial metadata, download and package-control regression cases.
use super::download::{copy_verified, trusted_download_address};
#[cfg(target_os = "linux")]
use super::install::validate_deb;
use super::*;
use sha2::{Digest, Sha256};
#[cfg(target_os = "linux")]
use std::process::Command;
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
fn plan() -> InstallPlan {
    InstallPlan::from_assets("0.2.0", &PACKAGES.map(|package| ReleaseAsset {
            name: format!("{package}_0.2.0_amd64.deb"), size: 3,
            digest: Some(format!("sha256:{:x}", Sha256::digest(b"abc"))),
            browser_download_url: format!("https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases/download/v0.2.0/{package}_0.2.0_amd64.deb"),
        })).unwrap()
}
#[test]
fn rejects_spoofed_missing_duplicate_or_unbounded_metadata() {
    let valid = plan();
    let mut invalid = valid.clone();
    invalid.packages[0].name = "../bad.deb".into();
    assert!(invalid.validate().is_err());
    for size in [0, MAX_PACKAGE_BYTES + 1] {
        let mut invalid = valid.clone();
        invalid.packages[0].size = size;
        assert!(invalid.validate().is_err());
    }
    for address in [
        "http://github.com/bad",
        "https://example.com/bad",
        "https://github.com/another/repo",
    ] {
        let mut invalid = valid.clone();
        invalid.packages[0].url = address.into();
        assert!(invalid.validate().is_err());
    }
    let mut invalid = valid.clone();
    invalid.packages.pop();
    assert!(invalid.validate().is_err());
    let mut invalid = valid.clone();
    invalid.packages[0].sha256 = "x".repeat(64);
    assert!(invalid.validate().is_err());
    assert!(InstallPlan::from_assets("0.2.0", &[]).is_err());
    let asset = ReleaseAsset {
        name: valid.packages[0].name.clone(),
        size: 3,
        digest: None,
        browser_download_url: valid.packages[0].url.clone(),
    };
    assert!(InstallPlan::from_assets("0.2.0", &[asset]).is_err());
}
#[test]
fn verified_copy_rejects_truncation_corruption_oversize_and_cancellation() {
    let package = plan().packages.remove(0);
    let canceled = AtomicBool::new(false);
    for input in [b"ab".as_slice(), b"abd", b"abcd"] {
        assert!(copy_verified(
            input,
            Vec::new(),
            &package,
            &canceled,
            Instant::now() + Duration::from_secs(1),
            |_| {}
        )
        .is_err());
    }
    let mut output = Vec::new();
    copy_verified(
        b"abc".as_slice(),
        &mut output,
        &package,
        &canceled,
        Instant::now() + Duration::from_secs(1),
        |_| {},
    )
    .unwrap();
    assert_eq!(output, b"abc");
    canceled.store(true, Ordering::Relaxed);
    assert!(copy_verified(
        b"abc".as_slice(),
        Vec::new(),
        &package,
        &canceled,
        Instant::now() + Duration::from_secs(1),
        |_| {}
    )
    .unwrap_err()
    .contains("canceled"));
    canceled.store(false, Ordering::Relaxed);
    assert!(copy_verified(
        b"abc".as_slice(),
        Vec::new(),
        &package,
        &canceled,
        Instant::now(),
        |_| {}
    )
    .unwrap_err()
    .contains("timed out"));
}
#[test]
fn redirects_are_tls_only_and_cancellation_precedes_network() {
    for address in [
        "http://github.com/a",
        "https://github.com.evil.test/a",
        "https://user@github.com/a",
        "https://example.com/a",
        "https://github.com:444/a",
    ] {
        assert!(!trusted_download_address(
            &url::Url::parse(address).unwrap()
        ));
    }
    for address in [
        "https://github.com/a",
        "https://release-assets.githubusercontent.com/a?signature=synthetic",
        "https://objects.githubusercontent.com/a",
    ] {
        assert!(trusted_download_address(&url::Url::parse(address).unwrap()));
    }
    assert!(download(&plan(), &AtomicBool::new(true), |_| {})
        .unwrap_err()
        .contains("canceled"));
}
#[test]
#[cfg(target_os = "linux")]
fn package_control_identity_is_checked_before_apt() {
    let directory = tempfile::tempdir().unwrap();
    let stage = directory.path().join("stage/DEBIAN");
    fs::create_dir_all(&stage).unwrap();
    fs::write(stage.join("control"), "Package: pnm-decode\nVersion: 0.2.0\nArchitecture: amd64\nMaintainer: Fixture <fixture@example.invalid>\nDescription: synthetic update fixture\n").unwrap();
    let package = directory.path().join("fixture.deb");
    assert!(Command::new("/usr/bin/dpkg-deb")
        .args(["--build"])
        .arg(stage.parent().unwrap())
        .arg(&package)
        .output()
        .unwrap()
        .status
        .success());
    validate_deb(&package, "pnm-decode", "0.2.0").unwrap();
    assert!(validate_deb(&package, "pnm-viewer-toolkit", "0.2.0").is_err());
    assert!(validate_deb(&package, "pnm-decode", "0.3.0").is_err());
    fs::write(
        stage.join("control"),
        fs::read_to_string(stage.join("control"))
            .unwrap()
            .replace("amd64", "arm64"),
    )
    .unwrap();
    assert!(Command::new("/usr/bin/dpkg-deb")
        .args(["--build"])
        .arg(stage.parent().unwrap())
        .arg(&package)
        .output()
        .unwrap()
        .status
        .success());
    assert!(validate_deb(&package, "pnm-decode", "0.2.0").is_err());
}
#[test]
fn source_build_cannot_install_or_authenticate() {
    assert!(!supported());
    assert!(install(Path::new("/nonexistent")).is_err());
    #[cfg(target_os = "linux")]
    assert!(validate_deb(Path::new("/nonexistent"), "pnm-decode", "0.2.0").is_err());
}
