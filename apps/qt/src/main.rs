/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod backend;
mod spectrum_collection;
mod update;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};
#[cfg(target_os = "linux")]
const DESKTOP_ID: &str = "pnm-viewer-toolkit";
/// Starts the Qt desktop, or handles authenticated Ubuntu installation before Qt.
///
/// The internal --install-update entry point stages verified packages and invokes
/// apt only from the installed Ubuntu executable with administrator identity.
///
/// # Panics
/// Terminates if Qt cannot create its application or QML engine.
fn main() {
    if std::env::args_os().nth(1).as_deref() == Some(std::ffi::OsStr::new("--install-update")) {
        let arguments: Vec<_> = std::env::args_os().skip(2).collect();
        let result = if arguments.len() == 1 {
            viewer::updates::ubuntu::install_elevated(std::path::Path::new(&arguments[0]))
        } else {
            Err("Expected one verified update directory.".into())
        };
        if let Err(message) = result {
            eprintln!("Update installation failed: {message}");
            std::process::exit(1);
        }
        return;
    }
    // Select native GTK open/save dialogs before Qt starts any threads.
    // Keep explicit desktop-portal or other platform-theme preferences.
    #[cfg(target_os = "linux")]
    if std::env::var_os("QT_QPA_PLATFORMTHEME").is_none_or(|theme| theme.is_empty()) {
        std::env::set_var("QT_QPA_PLATFORMTHEME", "gtk3");
    }
    // Native Windows/macOS control styles do not consistently honor our palette.
    // Match the development/test style before Qt starts; OS file dialogs remain native.
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
    let mut application = QGuiApplication::new();
    #[cfg(target_os = "linux")]
    {
        let desktop_id = cxx_qt_lib::QString::from(DESKTOP_ID);
        // Set before any window exists so X11 WM_CLASS and the desktop entry agree.
        application
            .as_mut()
            .expect("Qt application unavailable")
            .set_application_name(&desktop_id);
        QGuiApplication::set_desktop_file_name(&desktop_id);
    }
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
