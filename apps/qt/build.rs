/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use cxx_qt_build::{CxxQtBuilder, QmlModule};
/// Compiles the Qt bridge and embeds the local QML interface.
///
/// Links the system Qt Quick renderer; no WebView assets are included.
fn main() {
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
            rust_files: &["src/backend.rs"],
            qml_files: &[
                "qml/Main.qml",
                "qml/Theme.qml",
                "qml/RxMerChart.qml",
                "qml/PanelToggle.qml",
            ],
            ..Default::default()
        })
        .build();
}
