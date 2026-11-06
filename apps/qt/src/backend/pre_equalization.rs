/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Owned correction attachment, generation-safe precheck/prediction and full export.
use super::*;
use viewer::measurements::upstream_pre_equalization::comparison::{
    self, CorrectionMode, PreEqComparison,
};
#[derive(Default)]
pub(super) struct State {
    generation: u64,
    correction: Option<Arc<Capture>>,
    result: Option<Arc<PreEqComparison>>,
}
impl State {
    fn accepts(
        &self,
        generation: u64,
        current: Option<&Arc<Capture>>,
        original: &Arc<Capture>,
    ) -> bool {
        self.generation == generation
            && current.is_some_and(|current| Arc::ptr_eq(current, original))
    }
}
impl bridge::ViewerBackend {
    /// Detaches correction/result and invalidates any running comparison worker.
    /// # Returns
    /// Original capture/viewport unchanged, comparison state empty and idle.
    pub fn remove_pre_eq_update(mut self: Pin<&mut Self>) {
        let generation = self.rust().pre_eq.generation.wrapping_add(1);
        self.as_mut().rust_mut().pre_eq = State {
            generation,
            ..Default::default()
        };
        self.as_mut()
            .set_pre_eq_attachment_json(QString::from("{}"));
        self.as_mut().set_pre_eq_json(QString::from("{}"));
        self.as_mut().set_pre_eq_error(QString::default());
        self.as_mut().set_pre_eq_busy(false);
    }
    /// Reads a candidate and runs the mandatory core precheck before attaching it.
    /// # Arguments
    /// * `url` - One local file chosen/dropped in the left gutter.
    /// # Returns
    /// Starts bounded background validation; a failure preserves the previous attachment.
    pub fn attach_pre_eq_update(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.busy() || *self.pre_eq_busy() || !*self.pre_eq_current() {
            return;
        }
        let Some(original) = self.rust().capture.clone() else {
            return;
        };
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_pre_eq_error(QString::from(error.message));
                return;
            }
        };
        let generation = self.rust().pre_eq.generation.wrapping_add(1);
        self.as_mut().rust_mut().pre_eq.generation = generation;
        self.as_mut().set_pre_eq_busy(true);
        self.as_mut().set_pre_eq_error(QString::default());
        let qt_thread = self.qt_thread();
        let spawn=std::thread::Builder::new().name("qt-pre-eq-precheck".into()).spawn(move || {
            let result=Capture::read(&path).and_then(|correction| {
                let pair=original.pre_eq_precheck(&correction)?;
                let metadata=serde_json::to_string(&serde_json::json!({"pair":pair,"filename":path.file_name().unwrap_or_default().to_string_lossy()})).map_err(|error|Diagnostic::new("Json",error))?;
                Ok((Arc::new(correction),metadata))
            });
            let _queued=qt_thread.queue(move |mut backend| {
                if !backend.rust().pre_eq.accepts(generation,backend.rust().capture.as_ref(),&original) {return;}
                backend.as_mut().set_pre_eq_busy(false);
                match result {
                    Ok((correction,metadata))=>{
                        backend.as_mut().rust_mut().pre_eq.correction=Some(correction);
                        backend.as_mut().rust_mut().pre_eq.result=None;
                        backend.as_mut().set_pre_eq_json(QString::from("{}"));
                        backend.as_mut().set_pre_eq_attachment_json(QString::from(metadata));
                    },
                    Err(error)=>backend.set_pre_eq_error(QString::from(error.message)),
                }
            });
        });
        if let Err(error) = spawn {
            self.as_mut().set_pre_eq_busy(false);
            self.set_pre_eq_error(QString::from(error.to_string()));
        }
    }
    /// Calculates the immutable validated pair using an explicit mode.
    /// # Arguments
    /// * `mode` - adjust or set, never inferred from correction values.
    /// # Returns
    /// Publishes a bounded paired plot only if the same capture/generation is active.
    pub fn compare_pre_eq(mut self: Pin<&mut Self>, mode: &QString) {
        if *self.busy() || *self.pre_eq_busy() {
            return;
        }
        let Some(mode) = CorrectionMode::from_name(&mode.to_string()) else {
            self.set_pre_eq_error(QString::from("Select Adjust or Set first."));
            return;
        };
        let (Some(original), Some(correction)) = (
            self.rust().capture.clone(),
            self.rust().pre_eq.correction.clone(),
        ) else {
            return;
        };
        let generation = self.rust().pre_eq.generation.wrapping_add(1);
        self.as_mut().rust_mut().pre_eq.generation = generation;
        self.as_mut().set_pre_eq_busy(true);
        self.as_mut().set_pre_eq_error(QString::default());
        let qt_thread = self.qt_thread();
        let spawn = std::thread::Builder::new()
            .name("qt-pre-eq-compare".into())
            .spawn(move || {
                let result = original
                    .pre_eq_compare(&correction, mode)
                    .and_then(|analysis| {
                        let json = serde_json::to_string(&comparison::bounded(&analysis))
                            .map_err(|error| Diagnostic::new("Json", error))?;
                        Ok((Arc::new(analysis), json))
                    });
                let _queued = qt_thread.queue(move |mut backend| {
                    if !backend.rust().pre_eq.accepts(
                        generation,
                        backend.rust().capture.as_ref(),
                        &original,
                    ) {
                        return;
                    }
                    backend.as_mut().set_pre_eq_busy(false);
                    match result {
                        Ok((analysis, json)) => {
                            backend.as_mut().rust_mut().pre_eq.result = Some(analysis);
                            backend.set_pre_eq_json(QString::from(json));
                        }
                        Err(error) => {
                            let status = if backend.rust().pre_eq.correction.is_some() {
                                "Candidate rejected; previous correction retained."
                            } else {
                                "Precheck failed."
                            };
                            backend.set_pre_eq_error(QString::from(format!(
                                "{status} {}",
                                error.message
                            )));
                        }
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_pre_eq_busy(false);
            self.set_pre_eq_error(QString::from(error.to_string()));
        }
    }
    /// Updates comparison plots from exact original sample indices.
    /// # Arguments
    /// * `start`, `count` - Nonempty original-source range.
    /// # Returns
    /// Bounded paired traces; full snapshot and export remain unchanged.
    pub fn show_pre_eq_range(self: Pin<&mut Self>, start: u32, count: u32) {
        if *self.pre_eq_busy() {
            return;
        }
        let Some(result) = self.rust().pre_eq.result.clone() else {
            return;
        };
        let json =
            comparison::bounded_range(&result, start as usize, count as usize).and_then(|view| {
                serde_json::to_string(&view).map_err(|error| Diagnostic::new("Json", error))
            });
        match json {
            Ok(json) => self.set_pre_eq_json(QString::from(json)),
            Err(error) => self.set_pre_eq_error(QString::from(error.message)),
        }
    }
    /// Exports the successful full-resolution result with no destination overwrite.
    /// # Arguments
    /// * `url` - New local JSON destination from the native save dialog.
    /// # Returns
    /// Starts a worker; errors retain the result and are reported separately.
    pub fn export_pre_eq(mut self: Pin<&mut Self>, url: &QUrl) {
        if *self.pre_eq_busy() {
            return;
        }
        let Some(result) = self.rust().pre_eq.result.clone() else {
            return;
        };
        let path = match local_path(url) {
            Ok(path) => path,
            Err(error) => {
                self.set_pre_eq_error(QString::from(error.message));
                return;
            }
        };
        let generation = self.rust().pre_eq.generation;
        let qt_thread = self.qt_thread();
        self.as_mut().set_pre_eq_busy(true);
        self.as_mut().set_pre_eq_error(QString::default());
        let spawn = std::thread::Builder::new()
            .name("qt-pre-eq-export".into())
            .spawn(move || {
                let result = comparison::write_json_new(&result, &path);
                let _queued = qt_thread.queue(move |mut backend| {
                    if backend.rust().pre_eq.generation != generation {
                        return;
                    }
                    backend.as_mut().set_pre_eq_busy(false);
                    if let Err(error) = result {
                        backend.set_pre_eq_error(QString::from(error.message));
                    }
                });
            });
        if let Err(error) = spawn {
            self.as_mut().set_pre_eq_busy(false);
            self.set_pre_eq_error(QString::from(error.to_string()));
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detached_replaced_and_superseded_jobs_cannot_publish() {
        let original = Arc::new(
            Capture::from_bytes(
                pnm_fixtures::UPSTREAM_PRE_EQ.to_vec(),
                "original.bin".into(),
            )
            .unwrap(),
        );
        let other = Arc::new(
            Capture::from_bytes(pnm_fixtures::UPSTREAM_PRE_EQ.to_vec(), "other.bin".into())
                .unwrap(),
        );
        let state = State {
            generation: 7,
            ..Default::default()
        };
        assert!(state.accepts(7, Some(&original), &original));
        assert!(!state.accepts(6, Some(&original), &original));
        assert!(!state.accepts(7, Some(&other), &original));
        assert!(!state.accepts(7, None, &original));
    }
}
