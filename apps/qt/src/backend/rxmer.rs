/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent asynchronous analysis snapshots; the capture bytes stay shared.
use super::*;
use viewer::measurements::rxmer::{BitLoadingModel, RxMerAnalysisRequest, RxMerBitLoadingAnalysis};

#[derive(Default)]
pub(super) struct State {
    pub(super) generation: u64,
    pub(super) analysis: Option<Arc<RxMerBitLoadingAnalysis>>,
}
impl State {
    fn invalidate(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        self.analysis = None;
    }
    fn accepts(
        &self,
        generation: u64,
        current: Option<&Arc<Capture>>,
        source: &Arc<Capture>,
    ) -> bool {
        self.generation == generation && current.is_some_and(|current| Arc::ptr_eq(current, source))
    }
}
impl bridge::ViewerBackend {
    /// Cancels publication and drops the retained snapshot when closing/changing captures.
    /// # Returns
    /// Empty analysis state. Running bounded workers can finish but cannot publish.
    pub fn cancel_rxmer_analysis(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().rxmer.invalidate();
        self.as_mut().set_rxmer_analysis_busy(false);
        self.as_mut().set_rxmer_analysis_json(QString::from("{}"));
        self.as_mut().set_rxmer_analysis_error(QString::default());
    }
    /// Starts analysis from original indices and validated model controls.
    /// # Arguments
    /// * `start`, `count` - Exact visible source-sample range.
    /// * `model` - Stable policy identifier.
    /// * `margin_db` - Additional nonnegative MER margin.
    /// * `cap` - Maximum allowed data bits, checked before integer narrowing.
    /// # Returns
    /// Immediately starts a worker; completion publishes bounded plots on the Qt thread.
    /// Invalid requests preserve the prior snapshot and report analysis-only errors.
    pub fn analyze_rxmer(
        mut self: Pin<&mut Self>,
        start: u32,
        count: u32,
        model: &QString,
        margin_db: f64,
        cap: u32,
    ) {
        if *self.busy() || *self.rxmer_analysis_busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let Some(model) = BitLoadingModel::from_name(&model.to_string()) else {
            self.set_rxmer_analysis_error(QString::from("Unknown bit-loading model."));
            return;
        };
        let Ok(cap) = u8::try_from(cap) else {
            self.set_rxmer_analysis_error(QString::from("Invalid modulation cap."));
            return;
        };
        self.as_mut().rust_mut().rxmer.generation = self.rust().rxmer.generation.wrapping_add(1);
        let generation = self.rust().rxmer.generation;
        self.as_mut().set_rxmer_analysis_busy(true);
        self.as_mut().set_rxmer_analysis_error(QString::default());
        let qt_thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("qt-rxmer-analysis".into())
            .spawn(move || {
                let request = RxMerAnalysisRequest {
                    model,
                    extra_margin_db: margin_db,
                    maximum_bit_loading: cap,
                    ..Default::default()
                };
                let result = capture
                    .rxmer_bit_loading(start as usize, count as usize, request)
                    .and_then(|analysis| {
                        let plot = viewer::measurements::rxmer::bounded(analysis.clone());
                        let json = serde_json::to_string(&plot)
                            .map_err(|error| Diagnostic::new("Json", error))?;
                        Ok((Arc::new(analysis), json))
                    });
                let _queued = qt_thread.queue(move |mut backend| {
                    if !backend.rust().rxmer.accepts(
                        generation,
                        backend.rust().capture.as_ref(),
                        &capture,
                    ) {
                        return;
                    }
                    backend.as_mut().set_rxmer_analysis_busy(false);
                    match result {
                        Ok((analysis, json)) => {
                            backend.as_mut().rust_mut().rxmer.analysis = Some(analysis);
                            backend
                                .as_mut()
                                .set_rxmer_analysis_json(QString::from(json));
                        }
                        Err(error) => backend
                            .as_mut()
                            .set_rxmer_analysis_error(QString::from(error.message)),
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_analysis_busy(false);
            self.set_rxmer_analysis_error(QString::from(error.to_string()));
        }
    }
    /// Exports the last successfully calculated full-resolution snapshot on a worker.
    /// # Arguments
    /// * `url` - Local new-file destination from the native Save dialog.
    /// # Returns
    /// Starts a no-clobber export; invalid/busy/no-result requests do not write.
    pub fn export_rxmer_analysis(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.rxmer_analysis_busy() {
            return;
        }
        let Some(analysis) = self.rust().rxmer.analysis.clone() else {
            return;
        };
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_rxmer_analysis_error(QString::from(error.message));
                return;
            }
        };
        let generation = self.rust().rxmer.generation;
        let qt_thread = self.qt_thread();
        self.as_mut().set_rxmer_analysis_busy(true);
        self.as_mut().set_rxmer_analysis_error(QString::default());
        let spawn = std::thread::Builder::new()
            .name("qt-export-rxmer".into())
            .spawn(move || {
                let result = viewer::measurements::rxmer::write_json_new(&analysis, &path);
                let _queued = qt_thread.queue(move |mut backend| {
                    if backend.rust().rxmer.generation != generation {
                        return;
                    }
                    backend.as_mut().set_rxmer_analysis_busy(false);
                    if let Err(error) = result {
                        backend
                            .as_mut()
                            .set_rxmer_analysis_error(QString::from(error.message));
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_rxmer_analysis_busy(false);
            self.set_rxmer_analysis_error(QString::from(error.to_string()));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn closed_replaced_or_superseded_requests_cannot_publish() {
        let source = Arc::new(
            Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "fixture.bin".into()).unwrap(),
        );
        let replacement = Arc::new(
            Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "other.bin".into()).unwrap(),
        );
        let mut state = State::default();
        assert!(state.accepts(0, Some(&source), &source));
        assert!(!state.accepts(0, None, &source));
        assert!(!state.accepts(0, Some(&replacement), &source));
        state.invalidate();
        assert!(!state.accepts(0, Some(&source), &source));
    }
}
