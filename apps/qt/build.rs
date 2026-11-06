/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};

/// Compiles the branded Windows shell icon into the GUI executable.
///
/// Uses the active Windows SDK resource compiler without changing PATH or
/// adding a runtime dependency. The resource is linked only into the GUI binary.
///
/// # Panics
/// Terminates the build if required paths or the resource compiler are missing.
fn windows_icon() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    let manifest = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let root = manifest.parent().and_then(std::path::Path::parent).unwrap();
    let icon = root.join("packaging/windows/pnm-viewer.ico");
    println!("cargo:rerun-if-changed={}", icon.display());
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    let source = output.join("viewer-icon.rc");
    let resource = output.join("viewer-icon.res");
    let icon_path = icon
        .to_str()
        .expect("Icon path must be UTF-8")
        .replace('\\', "/");
    std::fs::write(
        &source,
        format!("#pragma code_page(65001)\n1 ICON \"{icon_path}\"\n"),
    )
    .unwrap();
    let status = std::process::Command::new("rc.exe")
        .arg("/nologo")
        .arg("/fo")
        .arg(&resource)
        .arg(&source)
        .status()
        .expect("Windows SDK rc.exe is required to embed the application icon");
    assert!(status.success(), "Windows icon resource compilation failed");
    println!(
        "cargo:rustc-link-arg-bin=pnm-viewer-qt={}",
        resource.display()
    );
}
/// Compiles the Qt bridge and embeds the local QML interface.
///
/// Links the system Qt Quick renderer; no WebView assets are included.
fn main() {
    windows_icon();
    CxxQtBuilder::new()
        .qrc("branding.qrc")
        .cc_builder(|builder| {
            builder.include("src");
        })
        .qt_module("Network")
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qml_module(QmlModule {
            uri: "Pnm.Viewer",
            rust_files: &["src/backend.rs", "src/update.rs"],
            qml_files: &[
                "qml/Main.qml",
                "qml/AboutDialog.qml",
                "qml/UpdateChip.qml",
                "qml/Theme.qml",
                "qml/ThemeButton.qml",
                "qml/MeasurementChart.qml",
                "qml/SpectrumControls.qml",
                "qml/SpectrumTools.qml",
                "qml/IqScatterChart.qml",
                "qml/ConstellationChart.qml",
                "qml/ConstellationMerOverlay.qml",
                "qml/HistogramChart.qml",
                "qml/FecSummaryChart.qml",
                "qml/PanelToggle.qml",
                "qml/SmokeTest.qml",
                "qml/CaptureDropArea.qml",
                "qml/ModulationControls.qml",
                "qml/MeasurementToggle.qml",
                "qml/ProfileBandwidth.qml",
            ],
            ..Default::default()
        })
        .build();
}
