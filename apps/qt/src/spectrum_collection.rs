/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent, generation-guarded standard spectrum collection session.
#[cxx_qt::bridge]
mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("cxx-qt-lib/qurl.h");
        type QUrl = cxx_qt_lib::QUrl;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, catalog_json)]
        #[qproperty(QString, view_json)]
        #[qproperty(QString, error_message)]
        #[qproperty(bool, busy)]
        type SpectrumCollectionBackend = super::SpectrumState;
        /// Imports standard PNN9 captures from one local directory/archive asynchronously.
        #[qinvokable]
        fn import_collection(self: Pin<&mut SpectrumCollectionBackend>, url: &QUrl);
        /// Cancels workers and invalidates stale callbacks, retaining usable prior data.
        #[qinvokable]
        fn cancel(self: Pin<&mut SpectrumCollectionBackend>);
        /// Selects one exact acquisition-settings group for aggregation or playback.
        #[qinvokable]
        fn select_group(self: Pin<&mut SpectrumCollectionBackend>, group: u32, playback: bool);
        /// Advances to one ascending-header-epoch frame, retaining active ranges.
        #[qinvokable]
        fn show_frame(self: Pin<&mut SpectrumCollectionBackend>, frame: u32);
        /// Requests a source-bin zoom range from the Rust session.
        #[qinvokable]
        fn show_range(self: Pin<&mut SpectrumCollectionBackend>, start: u32, count: u32);
        /// Sets the current tab's frequency window average.
        #[qinvokable]
        fn average(self: Pin<&mut SpectrumCollectionBackend>, window: u32);
        /// Validates frequency input without changing the session.
        #[qinvokable]
        fn range_error(
            self: &SpectrumCollectionBackend,
            start: f64,
            stop: f64,
            unit: &QString,
        ) -> QString;
        /// Opens a bounded independent frequency-range tab.
        #[qinvokable]
        fn open_band(
            self: Pin<&mut SpectrumCollectionBackend>,
            start: f64,
            stop: f64,
            unit: &QString,
        );
        /// Selects or closes a derived frequency tab.
        #[qinvokable]
        fn select_tab(self: Pin<&mut SpectrumCollectionBackend>, index: u32, close: bool);
        /// Calculates original/temporal-mean spectrum power for mouse fractions.
        #[qinvokable]
        fn calculate_power(self: Pin<&mut SpectrumCollectionBackend>, first: f64, last: f64);
        /// Writes the unrounded full-resolution result to a new JSON destination.
        #[qinvokable]
        fn export_json(self: Pin<&mut SpectrumCollectionBackend>, url: &QUrl);
    }
    impl cxx_qt::Threading for SpectrumCollectionBackend {}
}
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::{QString, QUrl};
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use viewer::{
    capture::{collection::write_json_new, Diagnostic},
    measurements::spectrum_analysis::collection::{Collection, ImportLimits, Session},
};
/// Rust-owned independent collection state; Qt receives compact catalogs/viewports.
pub struct SpectrumState {
    catalog_json: QString,
    view_json: QString,
    error_message: QString,
    busy: bool,
    generation: u64,
    cancel: Arc<AtomicBool>,
    collection: Option<Arc<Collection>>,
    session: Option<Session>,
}
impl Default for SpectrumState {
    /// Creates empty state without opening files or starting network operations.
    fn default() -> Self {
        Self {
            catalog_json: QString::from("{}"),
            view_json: QString::from("{}"),
            error_message: QString::default(),
            busy: false,
            generation: 0,
            cancel: Arc::new(AtomicBool::new(false)),
            collection: None,
            session: None,
        }
    }
}
impl bridge::SpectrumCollectionBackend {
    /// Invalidates active import/selection callbacks and releases their busy state.
    pub fn cancel(mut self: Pin<&mut Self>) {
        self.as_mut()
            .rust_mut()
            .cancel
            .store(true, Ordering::Relaxed);
        let generation = self.rust().generation.wrapping_add(1);
        self.as_mut().rust_mut().generation = generation;
        self.set_busy(false);
    }
    /// Starts bounded import; previous session remains intact if the input fails.
    /// # Arguments
    /// * `url` - User-selected native local path.
    pub fn import_collection(mut self: Pin<&mut Self>, url: &QUrl) {
        let path = match crate::backend::local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_error_message(QString::from(error.message));
                return;
            }
        };
        self.as_mut().cancel();
        self.as_mut().rust_mut().cancel = Arc::new(AtomicBool::new(false));
        let cancel = self.rust().cancel.clone();
        let generation = self.rust().generation;
        self.as_mut().set_busy(true);
        self.as_mut().set_error_message(QString::default());
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("spectrum-import".into())
            .spawn(move || {
                let result = Collection::scan(&path, ImportLimits::default(), &cancel).and_then(
                    |collection| {
                        serde_json::to_string(&collection.catalog)
                            .map(|json| (Arc::new(collection), json))
                            .map_err(|error| Diagnostic::new("Json", error))
                    },
                );
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().generation != generation {
                        return;
                    }
                    backend.as_mut().set_busy(false);
                    match result {
                        Ok((collection, json)) => {
                            backend.as_mut().rust_mut().collection = Some(collection);
                            backend.as_mut().rust_mut().session = None;
                            backend.as_mut().set_view_json(QString::from("{}"));
                            backend.set_catalog_json(QString::from(json));
                        }
                        Err(error) => backend.set_error_message(QString::from(error.message)),
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_busy(false);
            self.set_error_message(QString::from(error.to_string()));
        }
    }
    /// Aggregates one group on a worker; superseded results cannot replace current state.
    /// # Arguments
    /// * `group` - Catalog settings-group ID.
    /// * `playback` - Start with the oldest individual frame when true.
    pub fn select_group(mut self: Pin<&mut Self>, group: u32, playback: bool) {
        let Some(collection) = self.rust().collection.clone() else {
            return;
        };
        self.as_mut().cancel();
        let generation = self.rust().generation;
        self.as_mut().set_busy(true);
        self.as_mut().set_error_message(QString::default());
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("spectrum-aggregate".into())
            .spawn(move || {
                let result = Session::new(collection, group as usize, playback.then_some(0))
                    .and_then(|session| session.viewport().map(|view| (session, view.to_string())));
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().generation != generation {
                        return;
                    }
                    backend.as_mut().set_busy(false);
                    match result {
                        Ok((session, json)) => {
                            backend.as_mut().rust_mut().session = Some(session);
                            backend.set_view_json(QString::from(json));
                        }
                        Err(error) => backend.set_error_message(QString::from(error.message)),
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_busy(false);
            self.set_error_message(QString::from(error.to_string()));
        }
    }
    fn change(
        mut self: Pin<&mut Self>,
        operation: impl FnOnce(&mut Session) -> Result<(), Diagnostic>,
    ) {
        if *self.busy() {
            return;
        }
        let outcome = self
            .as_mut()
            .rust_mut()
            .session
            .as_mut()
            .map(|session| operation(session).and_then(|()| session.viewport()));
        if let Some(result) = outcome {
            match result {
                Ok(view) => {
                    self.as_mut().set_error_message(QString::default());
                    self.set_view_json(QString::from(view.to_string()));
                }
                Err(error) => self.set_error_message(QString::from(error.message)),
            }
        }
    }
    /// Shows one original frame in header epoch order while retaining tab settings.
    pub fn show_frame(self: Pin<&mut Self>, frame: u32) {
        self.change(|session| session.show_frame(frame as usize));
    }
    /// Displays a bounded source-bin range without modifying the full result.
    pub fn show_range(self: Pin<&mut Self>, start: u32, count: u32) {
        self.change(|session| session.range(start as usize, count as usize));
    }
    /// Applies an N-point linear-power frequency average to the active trace.
    pub fn average(self: Pin<&mut Self>, window: u32) {
        self.change(|session| session.average(window));
    }
    /// Validates user frequency endpoints against the session's complete source grid.
    pub fn range_error(&self, start: f64, stop: f64, unit: &QString) -> QString {
        self.rust()
            .session
            .as_ref()
            .map(|session| {
                session
                    .band(start, stop, &unit.to_string())
                    .err()
                    .map(|error| QString::from(error.message))
                    .unwrap_or_default()
            })
            .unwrap_or_else(|| QString::from("Select capture settings first."))
    }
    /// Creates an independent frequency-range tab after Rust validates its bounds.
    pub fn open_band(self: Pin<&mut Self>, start: f64, stop: f64, unit: &QString) {
        self.change(|session| session.open_band(start, stop, &unit.to_string()));
    }
    /// Selects or closes an existing derived range tab; original tab cannot be removed.
    pub fn select_tab(self: Pin<&mut Self>, index: u32, close: bool) {
        self.change(|session| {
            session.tab(index as usize, close);
            Ok(())
        });
    }
    /// Calculates ENBW-corrected power on unrounded original/temporal-average bins.
    pub fn calculate_power(self: Pin<&mut Self>, first: f64, last: f64) {
        self.change(|session| session.power(first, last));
    }
    /// Exports unrounded full-resolution aggregate/frame JSON on a worker.
    /// # Arguments
    /// * `url` - New native Save destination; existing files are never overwritten.
    pub fn export_json(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.busy() {
            return;
        }
        let Some(session) = self.rust().session.as_ref() else {
            return;
        };
        let result = session.result.clone();
        let path = match crate::backend::local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_error_message(QString::from(error.message));
                return;
            }
        };
        let generation = self.rust().generation;
        self.as_mut().set_busy(true);
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("spectrum-export".into())
            .spawn(move || {
                let outcome = write_json_new(&result, &path);
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().generation != generation {
                        return;
                    }
                    backend.as_mut().set_busy(false);
                    if let Err(error) = outcome {
                        backend.set_error_message(QString::from(error.message));
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_busy(false);
            self.set_error_message(QString::from(error.to_string()));
        }
    }
}

impl Drop for SpectrumState {
    /// Stops archive work when its QObject/session is destroyed.
    fn drop(&mut self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}
