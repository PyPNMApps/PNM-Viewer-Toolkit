/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Generation-guarded batch import and temporal RxMER analysis.
use super::*;
use std::sync::atomic::{AtomicBool, Ordering};
use viewer::measurements::rxmer::collection::AggregateResult;
use viewer::measurements::rxmer::{
    collection::{self, Collection, ImportLimits},
    RxMerAnalysisRequest,
};

#[derive(Default)]
pub(super) struct State {
    generation: u64,
    cancel: Arc<AtomicBool>,
    collection: Option<Arc<Collection>>,
    result: Option<Arc<AggregateResult>>,
}
impl bridge::ViewerBackend {
    /// Invalidates pending import/group work while retaining the last usable result.
    /// # Returns
    /// Idle state; old workers observe cancellation and cannot publish.
    pub fn cancel_rxmer_collection(mut self: Pin<&mut Self>) {
        let state = &mut self.as_mut().rust_mut().rxmer_collection;
        state.cancel.store(true, Ordering::Relaxed);
        state.generation = state.generation.wrapping_add(1);
        self.as_mut().set_rxmer_collection_busy(false);
        self.as_mut().cancel_rxmer_analysis();
    }
    /// Imports a directory or archive in Rust without unpacking member files.
    /// # Arguments
    /// * `url` - Local file or directory selected with native dialogs.
    /// # Returns
    /// Starts a worker; success publishes only a small device/group catalog.
    /// Invalid input preserves the previous collection and reports a file error.
    pub fn import_rxmer_collection(mut self: Pin<&mut Self>, url: &QUrl) {
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_rxmer_collection_error(QString::from(error.message));
                return;
            }
        };
        self.as_mut().cancel_rxmer_collection();
        self.as_mut().rust_mut().rxmer_collection.cancel = Arc::new(AtomicBool::new(false));
        let generation = self.rust().rxmer_collection.generation;
        let cancel = self.rust().rxmer_collection.cancel.clone();
        self.as_mut().set_rxmer_collection_error(QString::default());
        self.as_mut().set_rxmer_collection_busy(true);
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("rxmer-import".into())
            .spawn(move || {
                let result = Collection::scan(&path, ImportLimits::default(), &cancel).and_then(
                    |collection| {
                        let json = serde_json::to_string(&collection.catalog)
                            .map_err(|error| Diagnostic::new("Json", error))?;
                        Ok((Arc::new(collection), json))
                    },
                );
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().rxmer_collection.generation != generation {
                        return;
                    }
                    backend.as_mut().set_rxmer_collection_busy(false);
                    match result {
                        Ok((collection, json)) => {
                            backend.as_mut().rust_mut().rxmer_collection.collection =
                                Some(collection);
                            backend.as_mut().rust_mut().rxmer_collection.result = None;
                            backend
                                .as_mut()
                                .set_rxmer_collection_view(QString::from("{}"));
                            backend
                                .as_mut()
                                .set_rxmer_collection_catalog(QString::from(json));
                        }
                        Err(error) => {
                            backend.set_rxmer_collection_error(QString::from(error.message))
                        }
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_collection_busy(false);
            self.set_rxmer_collection_error(QString::from(error.to_string()));
        }
    }
    /// Aggregates one header-compatible device/channel/grid group on a worker.
    /// # Arguments
    /// * `id` - Group ID from the current collection catalog.
    /// # Returns
    /// Publishes bounded Min/Avg/Max curves and retains full-resolution statistics.
    /// Errors preserve the prior plot. Superseded workers cannot publish.
    pub fn select_rxmer_group(mut self: Pin<&mut Self>, id: u32) {
        let Some(collection) = self.rust().rxmer_collection.collection.clone() else {
            return;
        };
        self.as_mut().cancel_rxmer_collection();
        let generation = self.rust().rxmer_collection.generation;
        self.as_mut().set_rxmer_collection_busy(true);
        self.as_mut().set_rxmer_collection_error(QString::default());
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("rxmer-aggregate".into())
            .spawn(move || {
                let result = collection.aggregate(id as usize).and_then(|result| {
                    let viewport = collection::viewport(&result, 0, result.points.len())?;
                    Ok((Arc::new(result), viewport.to_string()))
                });
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().rxmer_collection.generation != generation {
                        return;
                    }
                    backend.as_mut().set_rxmer_collection_busy(false);
                    match result {
                        Ok((result, json)) => {
                            backend.as_mut().rust_mut().rxmer_collection.result = Some(result);
                            backend.set_rxmer_collection_view(QString::from(json));
                        }
                        Err(error) => {
                            backend.set_rxmer_collection_error(QString::from(error.message))
                        }
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_collection_busy(false);
            self.set_rxmer_collection_error(QString::from(error.to_string()));
        }
    }
    /// Displays a single immutable capture using the shared Rust chart pipeline.
    ///
    /// # Arguments
    /// * `group`, `frame` - Catalog group and ascending header-epoch position.
    /// * `start`, `count` - Source-carrier range, retained across playback frames.
    ///
    /// # Returns
    /// Publishes a bounded plot and retains only the current frame for analysis.
    /// Invalid selections preserve the prior result and report a file error.
    pub fn show_rxmer_frame(
        mut self: Pin<&mut Self>,
        group: u32,
        frame: u32,
        start: u32,
        count: u32,
    ) {
        if *self.rxmer_collection_busy() {
            return;
        }
        let Some(collection) = self.rust().rxmer_collection.collection.clone() else {
            return;
        };
        // One decoded frame only: no file I/O or collection-wide work on each tick.
        let result = collection
            .frame(group as usize, frame as usize)
            .and_then(|result| {
                let view = collection::playback_viewport(
                    &result,
                    group as usize,
                    frame as usize,
                    collection.catalog.groups[group as usize].capture_count,
                    start as usize,
                    count as usize,
                )?;
                Ok((Arc::new(result), view.to_string()))
            });
        match result {
            Ok((result, json)) => {
                self.as_mut().cancel_rxmer_collection();
                self.as_mut().set_rxmer_collection_error(QString::default());
                self.as_mut().rust_mut().rxmer_collection.result = Some(result);
                self.set_rxmer_collection_view(QString::from(json));
            }
            Err(error) => self.set_rxmer_collection_error(QString::from(error.message)),
        }
    }
    /// Retrieves a bounded viewport from the full-resolution aggregate.
    /// # Arguments
    /// * `start`, `count` - Exact source-carrier positions requested by mouse zoom.
    /// # Returns
    /// Updates the view on success; invalid ranges report errors without changing it.
    pub fn show_rxmer_collection_range(self: Pin<&mut Self>, start: u32, count: u32) {
        let Some(result) = self.rust().rxmer_collection.result.clone() else {
            return;
        };
        match collection::viewport(&result, start as usize, count as usize) {
            Ok(view) => self.set_rxmer_collection_view(QString::from(view.to_string())),
            Err(error) => self.set_rxmer_collection_error(QString::from(error.message)),
        }
    }
    /// Analyzes unrounded aggregate averages using the existing modulation models.
    /// # Arguments
    /// * `start`, `count` - Exact average-line range, not displayed indices.
    /// * `model`, `margin_db`, `cap` - Existing Modulation & Capacity controls.
    /// # Returns
    /// Publishes to the shared analysis dialog; cancellation or group changes prevent stale results.
    pub fn analyze_rxmer_average(
        mut self: Pin<&mut Self>,
        start: u32,
        count: u32,
        model: &QString,
        margin_db: f64,
        cap: u32,
    ) {
        if *self.rxmer_collection_busy() || *self.rxmer_analysis_busy() {
            return;
        }
        let Some(result) = self.rust().rxmer_collection.result.clone() else {
            return;
        };
        let (Some(model), Ok(cap)) = (
            viewer::measurements::rxmer::BitLoadingModel::from_name(&model.to_string()),
            u8::try_from(cap),
        ) else {
            self.set_rxmer_analysis_error(QString::from("Select a supported model and cap."));
            return;
        };
        self.as_mut().cancel_rxmer_analysis();
        let generation = self.rust().rxmer_collection.generation;
        let analysis_generation = self.rust().rxmer.generation;
        self.as_mut().set_rxmer_analysis_busy(true);
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("rxmer-average-analysis".into())
            .spawn(move || {
                let analysis = result
                    .analyze_average(
                        start as usize,
                        count as usize,
                        RxMerAnalysisRequest {
                            model,
                            extra_margin_db: margin_db,
                            maximum_bit_loading: cap,
                            ..Default::default()
                        },
                    )
                    .map_err(|error| Diagnostic::new("RxMerAnalysis", error))
                    .and_then(|analysis| {
                        let plot = viewer::measurements::rxmer::bounded(analysis.clone());
                        let json = serde_json::to_string(&plot)
                            .map_err(|error| Diagnostic::new("Json", error))?;
                        Ok((Arc::new(analysis), json))
                    });
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().rxmer_collection.generation != generation
                        || backend.rust().rxmer.generation != analysis_generation
                    {
                        return;
                    }
                    backend.as_mut().set_rxmer_analysis_busy(false);
                    match analysis {
                        Ok((analysis, json)) => {
                            backend.as_mut().rust_mut().rxmer.analysis = Some(analysis);
                            backend.set_rxmer_analysis_json(QString::from(json));
                        }
                        Err(error) => {
                            backend.set_rxmer_analysis_error(QString::from(error.message))
                        }
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_analysis_busy(false);
            self.set_rxmer_analysis_error(QString::from(error.to_string()));
        }
    }
    /// Exports the full aggregate independently of the visible zoom and trace toggles.
    /// # Arguments
    /// * `url` - New local JSON destination from the native Save dialog.
    /// # Returns
    /// Starts a bounded no-clobber write; errors appear in the file popup.
    pub fn export_rxmer_collection(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.rxmer_collection_busy() {
            return;
        }
        let Some(result) = self.rust().rxmer_collection.result.clone() else {
            return;
        };
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_rxmer_collection_error(QString::from(error.message));
                return;
            }
        };
        let generation = self.rust().rxmer_collection.generation;
        self.as_mut().set_rxmer_collection_error(QString::default());
        self.as_mut().set_rxmer_collection_busy(true);
        let thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("rxmer-statistics-export".into())
            .spawn(move || {
                let outcome = collection::write_json_new(&result, &path);
                let _ = thread.queue(move |mut backend| {
                    if backend.rust().rxmer_collection.generation != generation {
                        return;
                    }
                    backend.as_mut().set_rxmer_collection_busy(false);
                    if let Err(error) = outcome {
                        backend.set_rxmer_collection_error(QString::from(error.message));
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_collection_busy(false);
            self.set_rxmer_collection_error(QString::from(error.to_string()));
        }
    }
}
