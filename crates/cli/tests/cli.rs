/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Command-level tests run the actual executable with isolated files.
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
struct Workspace(PathBuf);
impl Workspace {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "pnm-cli-test-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_pnm-decode"))
            .current_dir(&self.0)
            .args(args)
            .output()
            .unwrap()
    }
}
impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn every_fixture_matches_core_json() {
    let workspace = Workspace::new();
    for bytes in [
        pnm_fixtures::RXMER,
        pnm_fixtures::CHANNEL_ESTIMATE,
        pnm_fixtures::CONSTELLATION,
        pnm_fixtures::HISTOGRAM,
        pnm_fixtures::FEC_SUMMARY,
        pnm_fixtures::SPECTRUM,
        pnm_fixtures::MODULATION_PROFILE,
        pnm_fixtures::UPSTREAM_PRE_EQ,
        pnm_fixtures::UPSTREAM_PRE_EQ_LAST,
    ] {
        fs::write(workspace.0.join("capture.bin"), bytes).unwrap();
        let result = workspace.run(&["capture.bin", "--stdout"]);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let expected = pnm::pnm::parser::pnm_file::PnmFile::from_bytes(bytes)
            .decode(Default::default())
            .unwrap()
            .to_json(16 * 1024 * 1024)
            .unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap(),
            serde_json::from_str::<serde_json::Value>(&expected).unwrap()
        );
        assert!(result.stderr.is_empty());
    }
}
#[test]
fn output_paths_no_overwrite_and_cleanup() {
    let workspace = Workspace::new();
    fs::write(workspace.0.join("café capture.bin"), pnm_fixtures::RXMER).unwrap();
    assert!(workspace.run(&["café capture.bin"]).status.success());
    let output = fs::read(workspace.0.join("café capture.json")).unwrap();
    assert_eq!(workspace.run(&["café capture.bin"]).status.code(), Some(1));
    assert_eq!(
        fs::read(workspace.0.join("café capture.json")).unwrap(),
        output
    );
    assert_eq!(
        workspace
            .run(&["café capture.bin", "-o", "café capture.bin"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        fs::read(workspace.0.join("café capture.bin")).unwrap(),
        pnm_fixtures::RXMER
    );
    assert!(workspace
        .run(&["café capture.bin", "-o", "chosen.json"])
        .status
        .success());
    assert_eq!(
        workspace
            .run(&["café capture.bin", "-o", "absent/out.json"])
            .status
            .code(),
        Some(1)
    );
    assert!(fs::read_dir(&workspace.0).unwrap().all(|entry| !entry
        .unwrap()
        .file_name()
        .to_string_lossy()
        .ends_with(".tmp")));
}
#[test]
fn usage_errors_limits_and_header_only() {
    let workspace = Workspace::new();
    for args in [
        &[][..],
        &["--bad"][..],
        &["--output"][..],
        &["file", "--stdout", "-o", "out"][..],
    ] {
        assert_eq!(workspace.run(args).status.code(), Some(2));
    }
    assert!(workspace.run(&["--help"]).status.success());
    assert!(workspace.run(&["--version"]).status.success());
    assert_eq!(workspace.run(&["missing"]).status.code(), Some(1));
    fs::write(workspace.0.join("unknown"), [0; 10]).unwrap();
    assert_eq!(workspace.run(&["unknown"]).status.code(), Some(3));
    fs::write(workspace.0.join("short"), &pnm_fixtures::RXMER[..20]).unwrap();
    assert_eq!(workspace.run(&["short"]).status.code(), Some(4));
    let mut symbol = pnm_fixtures::RXMER[..10].to_vec();
    symbol[3] = 1;
    fs::write(workspace.0.join("symbol"), symbol).unwrap();
    assert_eq!(workspace.run(&["symbol"]).status.code(), Some(3));
    let result = workspace.run(&["symbol", "--header-only", "--stdout"]);
    assert!(result.status.success());
    serde_json::from_slice::<serde_json::Value>(&result.stdout).unwrap();
    fs::write(workspace.0.join("-capture"), pnm_fixtures::RXMER).unwrap();
    assert!(workspace
        .run(&["--stdout", "--", "-capture"])
        .status
        .success());
    let oversized = fs::File::create(workspace.0.join("oversized")).unwrap();
    oversized.set_len(16 * 1024 * 1024 + 1).unwrap();
    assert_eq!(workspace.run(&["oversized"]).status.code(), Some(4));
}
