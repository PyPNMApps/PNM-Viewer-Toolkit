/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Native Qt adapter for the Rust capture and presentation layer.
//! Unsafe extern declarations are confined to CXX-Qt's typed interop boundary.
mod pre_equalization;
mod rxmer;
mod rxmer_collection;
mod spectrum;

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
        #[qproperty(QString, rxmer_collection_catalog)]
        #[qproperty(QString, rxmer_collection_view)]
        #[qproperty(QString, rxmer_collection_error)]
        #[qproperty(bool, rxmer_collection_busy)]
        #[qproperty(QString, app_version)]
        #[qproperty(bool, busy)]
        #[qproperty(bool, loaded)]
        #[qproperty(QString, metadata_json)]
        #[qproperty(QString, rxmer_analysis_json)]
        #[qproperty(QString, rxmer_analysis_error)]
        #[qproperty(bool, rxmer_analysis_busy)]
        #[qproperty(QString, spectrum_workspace_json)]
        #[qproperty(bool, pre_eq_current)]
        #[qproperty(bool, pre_eq_busy)]
        #[qproperty(QString, pre_eq_attachment_json)]
        #[qproperty(QString, pre_eq_json)]
        #[qproperty(QString, pre_eq_error)]
        #[qproperty(QString, viewport_json)]
        #[qproperty(QString, status)]
        #[qproperty(QString, error_message)]
        type ViewerBackend = super::ViewerBackendRust;

        /// Imports a local batch archive or directory asynchronously.
        #[qinvokable]
        fn import_rxmer_collection(self: Pin<&mut ViewerBackend>, url: &QUrl);
        /// Cancels pending import/group work and retains the previous result.
        #[qinvokable]
        fn cancel_rxmer_collection(self: Pin<&mut ViewerBackend>);
        /// Selects one compatible catalog group for full-resolution aggregation.
        #[qinvokable]
        fn select_rxmer_group(self: Pin<&mut ViewerBackend>, id: u32);
        /// Retrieves exact source indices for a temporal-statistics viewport.
        #[qinvokable]
        fn show_rxmer_collection_range(self: Pin<&mut ViewerBackend>, start: u32, count: u32);
        /// Applies Modulation & Capacity to the average line without quantization.
        #[qinvokable]
        fn analyze_rxmer_average(
            self: Pin<&mut ViewerBackend>,
            start: u32,
            count: u32,
            model: &QString,
            margin_db: f64,
            cap: u32,
        );
        /// Saves complete temporal statistics to a new JSON file.
        #[qinvokable]
        fn export_rxmer_collection(self: Pin<&mut ViewerBackend>, url: &QUrl);
        /// Returns the installed example captures folder, or an empty local URL.
        /// Resolves the executable location without opening captures or writing files.
        #[qinvokable]
        fn examples_folder(self: &ViewerBackend) -> QUrl;

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

        /// Analyzes original RxMER source indices using a shared Rust model and cap.
        #[qinvokable]
        fn analyze_rxmer(
            self: Pin<&mut ViewerBackend>,
            start: u32,
            count: u32,
            model: &QString,
            margin_db: f64,
            cap: u32,
        );
        /// Invalidates in-flight analysis and releases the immutable snapshot.
        #[qinvokable]
        fn cancel_rxmer_analysis(self: Pin<&mut ViewerBackend>);
        /// Exports full-resolution shared analysis to a new local destination.
        #[qinvokable]
        fn export_rxmer_analysis(self: Pin<&mut ViewerBackend>, url: &QUrl);

        /// Applies a positive spectrum averaging window to the current zoom.
        /// Invalid requests leave the previous trace and selection intact.
        #[qinvokable]
        fn apply_spectrum_average(self: Pin<&mut ViewerBackend>, window_points: u32);

        /// Opens a validated frequency range as a new spectrum tab.
        #[qinvokable]
        fn open_spectrum_band(self: Pin<&mut ViewerBackend>, start: f64, stop: f64, unit: &QString);

        /// Returns an empty message only for a valid spectrum frequency range.
        #[qinvokable]
        fn spectrum_range_error(
            self: &ViewerBackend,
            start: f64,
            stop: f64,
            unit: &QString,
        ) -> QString;

        /// Selects a spectrum tab, preserving the previous tab zoom/average.
        #[qinvokable]
        fn select_spectrum_tab(self: Pin<&mut ViewerBackend>, index: u32);

        /// Removes a derived tab; the original spectrum cannot be removed.
        #[qinvokable]
        fn close_spectrum_tab(self: Pin<&mut ViewerBackend>, index: u32);

        /// Integrates a dragged fraction range from the current spectrum view.
        #[qinvokable]
        fn calculate_spectrum_power(self: Pin<&mut ViewerBackend>, first: f64, last: f64);

        /// Queries a square I/Q window; invalid requests preserve the prior view.
        #[qinvokable]
        fn zoom_constellation(
            self: Pin<&mut ViewerBackend>,
            first_i: f64,
            last_i: f64,
            first_q: f64,
            last_q: f64,
        );

        /// Selects a supported measurement tab and resets its displayed range.
        /// Invalid tabs leave the prior presentation intact.
        #[qinvokable]
        fn select_view(self: Pin<&mut ViewerBackend>, index: u32);

        /// Selects one source FEC profile and resets its displayed interval range.
        /// Invalid indices leave the previous view intact.
        #[qinvokable]
        fn select_fec_profile(self: Pin<&mut ViewerBackend>, index: u32);

        /// Toggles one modulation profile while preserving its metric.
        /// Invalid requests retain the previous capture view.
        #[qinvokable]
        fn toggle_modulation_profile(self: Pin<&mut ViewerBackend>, index: u32);

        /// Enables every source modulation profile, retaining the current metric.
        /// Invalid or busy calls leave the previous view intact.
        #[qinvokable]
        fn select_all_modulation_profiles(self: Pin<&mut ViewerBackend>);

        /// Toggles a carrier classification by its stable metadata index.
        /// Preserves selected profiles and metric; invalid calls retain state.
        #[qinvokable]
        fn toggle_modulation_carrier(self: Pin<&mut ViewerBackend>, index: u32);

        /// Enables every carrier classification while retaining profile selection.
        #[qinvokable]
        fn select_all_modulation_carriers(self: Pin<&mut ViewerBackend>);

        /// Selects Shannon (0) or bit loading (1) for the current profile.
        /// Resets the frequency zoom after validation.
        #[qinvokable]
        fn select_modulation_metric(self: Pin<&mut ViewerBackend>, index: u32);

        /// Queries an inclusive UTC time range for the selected FEC profile.
        /// Epoch-second bounds are normalized and processed in Rust.
        #[qinvokable]
        fn show_fec_time_range(self: Pin<&mut ViewerBackend>, first: u32, last: u32);

        /// Releases the active capture when no file operation is running.
        /// Busy calls do nothing; native dialog cancellation never calls this.
        #[qinvokable]
        fn close_capture(self: Pin<&mut ViewerBackend>);
        /// Prechecks a last-update attachment against the current baseline.
        #[qinvokable]
        fn attach_pre_eq_update(self: Pin<&mut ViewerBackend>, url: &QUrl);
        /// Predicts a validated attachment using an explicit Adjust/Set mode.
        #[qinvokable]
        fn compare_pre_eq(self: Pin<&mut ViewerBackend>, mode: &QString);
        /// Detaches and invalidates outstanding comparison work.
        #[qinvokable]
        fn remove_pre_eq_update(self: Pin<&mut ViewerBackend>);
        /// Exports full comparison JSON to a new local file.
        #[qinvokable]
        fn export_pre_eq(self: Pin<&mut ViewerBackend>, url: &QUrl);
        /// Shows an exact source-carrier range of the retained comparison.
        #[qinvokable]
        fn show_pre_eq_range(self: Pin<&mut ViewerBackend>, start: u32, count: u32);

    }
    impl cxx_qt::Threading for ViewerBackend {}
}
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QString, QUrl};
use std::{path::PathBuf, pin::Pin, sync::Arc};
use viewer::capture::{chart::MeasurementView, Capture, Diagnostic};

/// Rust-owned state; only small strings and UI status cross the Qt boundary.
pub struct ViewerBackendRust {
    app_version: QString,
    busy: bool,
    loaded: bool,
    metadata_json: QString,
    viewport_json: QString,
    spectrum_workspace_json: QString,
    spectrum_workspace: spectrum::Workspace,
    rxmer: rxmer::State,
    rxmer_collection: rxmer_collection::State,
    rxmer_collection_catalog: QString,
    rxmer_collection_view: QString,
    rxmer_collection_error: QString,
    rxmer_collection_busy: bool,
    pre_eq: pre_equalization::State,
    pre_eq_current: bool,
    pre_eq_busy: bool,
    pre_eq_attachment_json: QString,
    pre_eq_json: QString,
    pre_eq_error: QString,
    rxmer_analysis_json: QString,
    rxmer_analysis_error: QString,
    rxmer_analysis_busy: bool,
    status: QString,
    error_message: QString,
    capture: Option<Arc<Capture>>,
    view: MeasurementView,
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
            spectrum_workspace_json: QString::from("{}"),
            spectrum_workspace: spectrum::Workspace::default(),
            rxmer: rxmer::State::default(),
            rxmer_collection: rxmer_collection::State::default(),
            rxmer_collection_catalog: QString::from("{}"),
            rxmer_collection_view: QString::from("{}"),
            rxmer_collection_error: QString::default(),
            rxmer_collection_busy: false,
            pre_eq: pre_equalization::State::default(),
            pre_eq_current: false,
            pre_eq_busy: false,
            pre_eq_attachment_json: QString::from("{}"),
            pre_eq_json: QString::from("{}"),
            pre_eq_error: QString::default(),
            rxmer_analysis_json: QString::from("{}"),
            rxmer_analysis_error: QString::default(),
            rxmer_analysis_busy: false,
            capture: None,
            view: MeasurementView::Default,
        }
    }
}
impl bridge::ViewerBackend {
    /// Adds a full-resolution-derived average while preserving the current zoom.
    /// # Arguments
    /// * `window_points` - Positive count of source bins, not display points.
    /// # Returns
    /// Updates the bounded spectrum viewport on success. Busy/no-capture calls
    /// do nothing; invalid requests report a diagnostic and retain prior state.
    pub fn apply_spectrum_average(mut self: Pin<&mut Self>, window_points: u32) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let result = serde_json::from_str::<serde_json::Value>(&self.viewport_json().to_string())
            .map_err(|error| Diagnostic::new("Json", error))
            .and_then(|range| {
                let start = serde_json::from_value::<usize>(range["start"].clone())
                    .map_err(|error| Diagnostic::new("Json", error))?;
                let count = serde_json::from_value::<usize>(range["count"].clone())
                    .map_err(|error| Diagnostic::new("Json", error))?;
                capture.chart_range_for(
                    MeasurementView::SpectrumAverage { window_points },
                    start,
                    count,
                )
            })
            .and_then(|viewport| {
                serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))
            });
        match result {
            Ok(json) => {
                self.as_mut().rust_mut().view = MeasurementView::SpectrumAverage { window_points };
                self.as_mut().set_error_message(QString::default());
                self.as_mut().set_viewport_json(QString::from(&json));
                self.save_spectrum_tab();
            }
            Err(error) => self.fail(error),
        }
    }

    /// Finds packaged examples for the initial native Open dialog location.
    ///
    /// # Returns
    /// A local file URL, or an empty URL when launched without packaged resources.
    pub fn examples_folder(&self) -> QUrl {
        std::env::current_exe()
            .ok()
            .and_then(|executable| viewer::resources::examples_directory(&executable))
            .map(|directory| {
                QUrl::from_local_file(&QString::from(directory.to_string_lossy().as_ref()))
            })
            .unwrap_or_default()
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
    let chart_metadata = capture.chart_metadata()?;
    let metadata =
        serde_json::to_string(&chart_metadata).map_err(|error| Diagnostic::new("Json", error))?;
    let viewport = serde_json::to_string(&capture.chart_range(0, chart_metadata.sample_count)?)
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
                            backend.as_mut().cancel_rxmer_analysis();
                            backend.as_mut().remove_pre_eq_update();
                            backend
                                .as_mut()
                                .set_pre_eq_current(loaded.capture.supports_pre_eq_comparison());
                            backend.as_mut().rust_mut().capture = Some(loaded.capture);
                            backend.as_mut().rust_mut().view = MeasurementView::Default;
                            backend
                                .as_mut()
                                .set_metadata_json(QString::from(&loaded.metadata));
                            backend
                                .as_mut()
                                .set_viewport_json(QString::from(&loaded.viewport));
                            backend.as_mut().reset_spectrum_workspace();
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
        let (start, count) = self.spectrum_zoom_bounds(start as usize, count as usize);
        match capture
            .chart_range_for(self.rust().view.clone(), start, count)
            .and_then(|viewport| {
                serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))
            }) {
            Ok(json) => {
                self.as_mut().set_error_message(QString::default());
                self.as_mut().set_viewport_json(QString::from(&json));
                self.as_mut().save_spectrum_tab();
                self.set_status(QString::from("Sample window updated"));
            }
            Err(error) => self.fail(error),
        }
    }
    /// Selects a constellation coordinate window through the shared Rust viewer.
    /// # Arguments
    /// * `first_i`, `last_i` - Requested in-phase bounds, in normalized amplitude.
    /// * `first_q`, `last_q` - Requested quadrature bounds in the same units.
    /// # Returns
    /// Bounded measured/reference points; busy or failed calls retain the prior view.
    pub fn zoom_constellation(
        mut self: Pin<&mut Self>,
        first_i: f64,
        last_i: f64,
        first_q: f64,
        last_q: f64,
    ) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let bounds = viewer::measurements::constellation::IqBounds {
            minimum_i: first_i,
            maximum_i: last_i,
            minimum_q: first_q,
            maximum_q: last_q,
        };
        match capture.constellation_range(bounds).and_then(|viewport| {
            serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))
        }) {
            Ok(viewport) => {
                self.as_mut().set_viewport_json(QString::from(&viewport));
                self.as_mut().set_error_message(QString::default());
            }
            Err(error) => self.fail(error),
        }
    }

    /// Changes a channel-response tab only after both payloads validate.
    /// # Arguments
    /// * `index` - Zero for the default chart, one for delay, two for I/Q.
    /// # Returns
    /// Updates metadata and a bounded full-range viewport. Invalid or busy calls
    /// preserve the current view; errors appear in the existing diagnostic area.
    pub fn select_view(self: Pin<&mut Self>, index: u32) {
        if *self.busy() {
            return;
        }
        let view = match index {
            0 => MeasurementView::Default,
            1 => MeasurementView::GroupDelay,
            2 => MeasurementView::IqScatter,
            _ => {
                self.fail(Diagnostic::new(
                    "UnsupportedView",
                    "Unknown measurement tab.",
                ));
                return;
            }
        };
        self.change_view(view);
    }

    /// Selects a FEC profile by its source index, preserving state on failure.
    /// # Arguments
    /// * `index` - Zero-based position in the capture's profile list.
    /// # Returns
    /// Updates small metadata and the profile's full-range bounded viewport.
    pub fn select_fec_profile(self: Pin<&mut Self>, index: u32) {
        self.change_view(MeasurementView::FecProfile(index as usize));
    }

    /// Displays a selected UTC window through the shared Rust FEC mapper.
    /// # Arguments
    /// * `first`, `last` - Inclusive epoch seconds; reverse drags are accepted.
    /// # Returns
    /// Updates only bounded viewport JSON; invalid calls retain the prior view.
    pub fn show_fec_time_range(mut self: Pin<&mut Self>, first: u32, last: u32) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let result = capture
            .fec_time_range(self.rust().view.clone(), first, last)
            .and_then(|viewport| {
                serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))
            });
        match result {
            Ok(json) => {
                self.as_mut().set_viewport_json(QString::from(&json));
                self.as_mut().set_error_message(QString::default());
            }
            Err(error) => self.fail(error),
        }
    }

    /// Toggles one source profile while retaining the metric and other traces.
    /// # Arguments
    /// * `index` - Zero-based profile position, not its wire ID.
    /// # Returns
    /// Validated colored overlays; deselecting the final profile leaves the axes.
    pub fn toggle_modulation_profile(self: Pin<&mut Self>, index: u32) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let metadata = match capture.chart_metadata_for(self.rust().view.clone()) {
            Ok(metadata) => metadata,
            Err(error) => {
                self.fail(error);
                return;
            }
        };
        let Some(controls) = metadata.modulation_profile else {
            return;
        };
        if index as usize >= controls.profiles.len() {
            self.fail(Diagnostic::new(
                "InvalidProfile",
                "Modulation profile is outside this capture.",
            ));
            return;
        }
        let mut indices = controls.selected_indices;
        if let Some(position) = indices
            .iter()
            .position(|selected| *selected == index as usize)
        {
            indices.remove(position);
        } else {
            indices.push(index as usize);
        }
        let metric = self.modulation_metric();
        let carrier_types = self.modulation_carriers();
        self.change_view(MeasurementView::ModulationProfile {
            indices: Some(indices),
            carrier_types,
            metric,
        });
    }

    /// Enables all source profiles while retaining the current modulation metric.
    /// # Returns
    /// Validated overlays under the shared carrier/display budgets; failures
    /// retain the previous chart and are reported through the diagnostic area.
    pub fn select_all_modulation_profiles(self: Pin<&mut Self>) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let controls = match capture.chart_metadata_for(self.rust().view.clone()) {
            Ok(metadata) => metadata.modulation_profile,
            Err(error) => {
                self.fail(error);
                return;
            }
        };
        let Some(controls) = controls else {
            return;
        };
        let metric = self.modulation_metric();
        let carrier_types = self.modulation_carriers();
        self.change_view(MeasurementView::ModulationProfile {
            indices: Some((0..controls.profiles.len()).collect()),
            carrier_types,
            metric,
        });
    }

    fn modulation_carriers(
        &self,
    ) -> Option<Vec<viewer::measurements::modulation_profile::CarrierType>> {
        match &self.rust().view {
            MeasurementView::ModulationProfile { carrier_types, .. } => carrier_types.clone(),
            _ => None,
        }
    }

    /// Toggles one carrier role without changing the profile or metric selection.
    /// # Arguments
    /// * `index` - Position in the stable carrier_types metadata list.
    /// # Returns
    /// Validated traces/markers or a diagnostic, leaving previous state on error.
    pub fn toggle_modulation_carrier(self: Pin<&mut Self>, index: u32) {
        self.change_modulation_carriers(Some(index as usize));
    }

    /// Enables all carrier roles, preserving the selected profiles and metric.
    /// # Returns
    /// Validated data traces and special-carrier lanes.
    pub fn select_all_modulation_carriers(self: Pin<&mut Self>) {
        self.change_modulation_carriers(None);
    }

    fn change_modulation_carriers(self: Pin<&mut Self>, toggle: Option<usize>) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let controls = match capture.chart_metadata_for(self.rust().view.clone()) {
            Ok(metadata) => metadata.modulation_profile,
            Err(error) => {
                self.fail(error);
                return;
            }
        };
        let Some(controls) = controls else {
            return;
        };
        let mut selected = controls.selected_carrier_types;
        if let Some(index) = toggle {
            let Some(choice) = controls.carrier_types.get(index) else {
                self.fail(Diagnostic::new(
                    "UnsupportedView",
                    "Unknown carrier classification.",
                ));
                return;
            };
            if let Some(position) = selected.iter().position(|kind| *kind == choice.kind) {
                selected.remove(position);
            } else {
                selected.push(choice.kind);
            }
        } else {
            selected = controls
                .carrier_types
                .iter()
                .map(|choice| choice.kind)
                .collect();
        }
        let metric = self.modulation_metric();
        self.change_view(MeasurementView::ModulationProfile {
            indices: Some(controls.selected_indices),
            carrier_types: Some(selected),
            metric,
        });
    }

    fn modulation_metric(&self) -> viewer::measurements::modulation_profile::ModulationMetric {
        match &self.rust().view {
            MeasurementView::ModulationProfile { metric, .. } => *metric,
            _ => viewer::measurements::modulation_profile::ModulationMetric::Shannon,
        }
    }

    /// Selects the modulation metric and resets the current profile's zoom.
    /// # Arguments
    /// * `index` - Zero for ideal Shannon MER, one for bit loading.
    /// # Returns
    /// Updates the chart only after validation; unsupported values retain state.
    pub fn select_modulation_metric(self: Pin<&mut Self>, index: u32) {
        let metric = match index {
            0 => viewer::measurements::modulation_profile::ModulationMetric::Shannon,
            1 => viewer::measurements::modulation_profile::ModulationMetric::BitLoading,
            _ => {
                self.fail(Diagnostic::new(
                    "UnsupportedView",
                    "Unknown modulation metric.",
                ));
                return;
            }
        };
        let profiles = match &self.rust().view {
            MeasurementView::ModulationProfile { indices, .. } => indices.clone(),
            _ => None,
        };
        let carrier_types = self.modulation_carriers();
        self.change_view(MeasurementView::ModulationProfile {
            carrier_types,
            indices: profiles,
            metric,
        });
    }

    fn change_view(mut self: Pin<&mut Self>, view: MeasurementView) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let result = (|| {
            let metadata = capture.chart_metadata_for(view.clone())?;
            let viewport = capture.chart_range_for(view.clone(), 0, metadata.sample_count)?;
            let metadata_json =
                serde_json::to_string(&metadata).map_err(|error| Diagnostic::new("Json", error))?;
            let viewport_json =
                serde_json::to_string(&viewport).map_err(|error| Diagnostic::new("Json", error))?;
            Ok::<_, Diagnostic>((metadata_json, viewport_json))
        })();
        match result {
            Ok((metadata, viewport)) => {
                self.as_mut().rust_mut().view = view;
                self.as_mut().set_viewport_json(QString::from(&viewport));
                self.as_mut().set_metadata_json(QString::from(&metadata));
                self.as_mut().set_error_message(QString::default());
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
        self.as_mut().cancel_rxmer_analysis();
        self.as_mut().remove_pre_eq_update();
        self.as_mut().set_pre_eq_current(false);
        self.as_mut().rust_mut().spectrum_workspace = spectrum::Workspace::default();
        self.as_mut()
            .set_spectrum_workspace_json(QString::from("{}"));
        self.as_mut().rust_mut().capture = None;
        self.as_mut().rust_mut().view = MeasurementView::Default;
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
        assert_eq!(metadata["mac_address"], "aa:bb:cc:dd:ee:ff");
        assert_eq!(metadata["sample_count"], 7480);
        assert_eq!(capture.viewport(0).unwrap().points.len(), 1024);
    }
}
