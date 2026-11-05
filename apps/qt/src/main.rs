/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
mod backend;
mod update;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
/// Starts the Qt Quick desktop application using embedded QML resources.
///
/// # Panics
/// Terminates if Qt cannot create its application or QML engine.
fn main() {
    // Select native GTK open/save dialogs before Qt starts any threads.
    // Keep explicit desktop-portal or other platform-theme preferences.
    #[cfg(target_os = "linux")]
    if std::env::var_os("QT_QPA_PLATFORMTHEME").is_none_or(|theme| theme.is_empty()) {
        std::env::set_var("QT_QPA_PLATFORMTHEME", "gtk3");
    }
    let mut application = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();
    engine
        .as_mut()
        .expect("Qt QML engine unavailable")
        .load(&QUrl::from("qrc:/qt/qml/Pnm/Viewer/qml/Main.qml"));
    application
        .as_mut()
        .expect("Qt application unavailable")
        .exec();
}
