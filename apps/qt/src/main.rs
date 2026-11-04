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
