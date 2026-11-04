/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Qt adapter for the same Rust capture layer used by the Tauri prototype.
//! Unsafe extern declarations are confined to CXX-Qt's typed interop boundary.
#[cxx_qt::bridge]
mod bridge {
    unsafe extern "C++" {
        include!("branding.h");
        fn setViewerIcon(dark: bool);
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, app_version)]
        #[qproperty(bool, busy)]
        #[qproperty(bool, loaded)]
        #[qproperty(QString, metadata_json)]
        #[qproperty(QString, viewport_json)]
        #[qproperty(QString, status)]
        #[qproperty(QString, error_message)]
        type ViewerBackend = super::ViewerBackendRust;

        /// Selects the embedded desktop icon for the current appearance.
        /// Called on the UI thread; dark selects the dark-theme asset.
        #[qinvokable]
        fn set_dark_icon(self: Pin<&mut ViewerBackend>, dark: bool);

        /// Opens a selected local file, validating it on a Rust worker.
        /// `url` must be a local file URL. Errors retain the previous capture.
        #[qinvokable]
        fn open_file(self: Pin<&mut ViewerBackend>, url: &QUrl);

        /// Exports full core JSON to a new local destination on a worker.
        /// `url` is the selected destination; existing files are rejected.
        #[qinvokable]
        fn export_json(self: Pin<&mut ViewerBackend>, url: &QUrl);

        /// Fetches a bounded overview or exact detail for the requested range.
        /// Out-of-range requests leave the previous viewport intact.
        #[qinvokable]
        fn show_range(self: Pin<&mut ViewerBackend>, start: u32, count: u32);

        /// Releases the active capture when no file operation is running.
        /// Busy calls do nothing; native dialog cancellation never calls this.
        #[qinvokable]
        fn close_capture(self: Pin<&mut ViewerBackend>);
    }
    impl cxx_qt::Threading for ViewerBackend {}
}
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QString, QUrl};
use std::{path::PathBuf, pin::Pin, sync::Arc};
use viewer::{Capture, Diagnostic};

/// Rust-owned state; only small strings and UI status cross the Qt boundary.
pub struct ViewerBackendRust {
    app_version: QString,
    busy: bool,
    loaded: bool,
    metadata_json: QString,
    viewport_json: QString,
    status: QString,
    error_message: QString,
    capture: Option<Arc<Capture>>,
}
struct LoadedCapture {
    capture: Arc<Capture>,
    metadata: String,
    viewport: String,
}
impl Default for ViewerBackendRust {
    /// Creates an empty viewer with no capture bytes or filesystem activity.
    ///
    /// # Returns
    /// Idle state and valid empty JSON presentation objects.
    fn default() -> Self {
        Self {
            app_version: QString::from(env!("CARGO_PKG_VERSION")),
            busy: false,
            loaded: false,
            metadata_json: QString::from("{}"),
            viewport_json: QString::from("{\"start\":0,\"total\":0,\"points\":[]}"),
            status: QString::from("Ready to open a local capture"),
            error_message: QString::default(),
            capture: None,
        }
    }
}
fn local_path(url: &QUrl) -> Result<PathBuf, Diagnostic> {
    let path = url
        .to_local_file()
        .ok_or_else(|| Diagnostic::new("InvalidPath", "Select a local file."))?
        .to_string();
    if path.is_empty() {
        return Err(Diagnostic::new("InvalidPath", "Select a local file."));
    }
    Ok(PathBuf::from(path))
}
fn load_capture(path: PathBuf) -> Result<LoadedCapture, Diagnostic> {
    let capture = Arc::new(Capture::read(&path)?);
    let metadata = serde_json::to_string(&capture.metadata()?)
        .map_err(|error| Diagnostic::new("Json", error))?;
    let viewport =
        serde_json::to_string(&capture.range_viewport(0, capture.metadata()?.sample_count)?)
            .map_err(|error| Diagnostic::new("Json", error))?;
    Ok(LoadedCapture {
        capture,
        metadata,
        viewport,
    })
}
impl bridge::ViewerBackend {
    /// Updates the native window icon to match the active theme.
    ///
    /// # Arguments
    /// * `dark` - Whether the active appearance uses a dark background.
    pub fn set_dark_icon(self: Pin<&mut Self>, dark: bool) {
        bridge::setViewerIcon(dark);
    }

    /// Reads and validates the chosen local capture without blocking Qt's UI thread.
    ///
    /// # Arguments
    /// * `url` - Native picker result; remote URLs are rejected.
    /// # Returns
    /// Immediately starts one bounded worker, or reports an input error. A queued
    /// completion replaces the capture only after successful validation.
    pub fn open_file(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.busy() {
            return;
        }
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.fail(error);
                return;
            }
        };
        self.as_mut().set_busy(true);
        self.as_mut().set_error_message(QString::default());
        self.as_mut().set_status(QString::from("Decoding capture…"));
        let qt_thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("qt-open-capture".into())
            .spawn(move || {
                let result = load_capture(path);
                // A closed window safely rejects queued work; no raw QObject pointers escape.
                let _queued = qt_thread.queue(move |mut backend| {
                    backend.as_mut().set_busy(false);
                    match result {
                        Ok(loaded) => {
                            backend.as_mut().rust_mut().capture = Some(loaded.capture);
                            backend
                                .as_mut()
                                .set_metadata_json(QString::from(&loaded.metadata));
                            backend
                                .as_mut()
                                .set_viewport_json(QString::from(&loaded.viewport));
                            backend.as_mut().set_loaded(true);
                            backend
                                .as_mut()
                                .set_status(QString::from("Capture decoded locally"));
                        }
                        Err(error) => backend.fail(error),
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_busy(false);
            self.fail(Diagnostic::new("Worker", error));
        }
    }
    /// Saves full-resolution JSON through the shared no-clobber export operation.
    ///
    /// # Arguments
    /// * `url` - Local output chosen by the native save dialog.
    /// # Returns
    /// Immediately starts one export worker. Busy/no-capture calls do nothing;
    /// failures update the visible diagnostic without discarding the capture.
    pub fn export_json(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.fail(error);
                return;
            }
        };
        self.as_mut().set_busy(true);
        self.as_mut().set_error_message(QString::default());
        self.as_mut()
            .set_status(QString::from("Exporting full-resolution JSON…"));
        let qt_thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("qt-export-json".into())
            .spawn(move || {
                let result = capture.write_json_new(&path);
                let _queued = qt_thread.queue(move |mut backend| {
                    backend.as_mut().set_busy(false);
                    match result {
                        Ok(()) => {
                            backend.set_status(QString::from("Full-resolution JSON exported"))
                        }
                        Err(error) => backend.fail(error),
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_busy(false);
            self.fail(Diagnostic::new("Worker", error));
        }
    }
    /// Returns a small exact sample window for the current capture.
    ///
    /// # Arguments
    /// * `start` - Zero-based sample index, validated by the shared capture layer.
    /// * `count` - Sample range length; large ranges use a bounded overview.
    /// # Returns
    /// Updates bounded viewport JSON or reports an error; never serializes the
    /// complete capture. Calls during file operations are ignored.
    pub fn show_range(mut self: Pin<&mut Self>, start: u32, count: u32) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        match capture
            .range_viewport(start as usize, count as usize)
            .and_then(|viewport| {
                serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))
            }) {
            Ok(json) => {
                self.as_mut().set_error_message(QString::default());
                self.as_mut().set_viewport_json(QString::from(&json));
                self.set_status(QString::from("Sample window updated"));
            }
            Err(error) => self.fail(error),
        }
    }
    /// Drops the active capture after a completed operation.
    ///
    /// # Returns
    /// Empty presentation state; busy calls leave the running job intact.
    pub fn close_capture(mut self: Pin<&mut Self>) {
        if *self.busy() {
            return;
        }
        self.as_mut().rust_mut().capture = None;
        self.as_mut().set_loaded(false);
        self.as_mut().set_metadata_json(QString::from("{}"));
        self.as_mut()
            .set_viewport_json(QString::from("{\"start\":0,\"total\":0,\"points\":[]}"));
        self.as_mut().set_error_message(QString::default());
        self.set_status(QString::from("Capture closed"));
    }
    fn fail(mut self: Pin<&mut Self>, error: Diagnostic) {
        self.as_mut()
            .set_error_message(QString::from(&format!("{}: {}", error.code, error.message)));
        self.set_status(QString::from("Operation failed"));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reject_remote_urls() {
        assert!(local_path(&QUrl::from("https://example.invalid/capture.bin")).is_err());
    }
    #[test]
    fn bounded_contract_preserves_metadata() {
        let capture =
            Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap();
        let metadata = serde_json::to_value(capture.metadata().unwrap()).unwrap();
        assert_eq!(metadata["channel_id"], 34);
        assert_eq!(metadata["mac_address"], "a1:b2:c3:d4:e5:f6");
        assert_eq!(metadata["sample_count"], 7480);
        assert_eq!(capture.viewport(0).unwrap().points.len(), 1024);
    }
}
