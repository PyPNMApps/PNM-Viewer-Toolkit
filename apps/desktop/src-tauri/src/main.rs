/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#![forbid(unsafe_code)]
mod capture;
use capture::{Capture, Diagnostic, Metadata, Viewport};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    io::Write,
    sync::{Arc, Mutex},
};
use tauri::{Manager, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Default)]
struct OperationGate(Arc<AtomicBool>);
struct OperationGuard(Arc<AtomicBool>);
impl OperationGate {
    fn enter(&self) -> Result<OperationGuard, Diagnostic> {
        self.0
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .map_err(|_| Diagnostic::new("Busy", "Finish the current file operation first."))?;
        Ok(OperationGuard(self.0.clone()))
    }
}
impl Drop for OperationGuard {
    /// Releases the file-operation gate when its worker completes or unwinds.
    fn drop(&mut self) {
        self.0.store(false, Ordering::Release);
    }
}

#[derive(Default)]
struct Sessions(Mutex<Session>);
#[derive(Default)]
struct Session {
    generation: u32,
    capture: Option<Arc<Capture>>,
}
#[derive(Serialize)]
struct Opened {
    session_id: u32,
    metadata: Metadata,
    viewport: Viewport,
}

fn selected(state: &Sessions, session_id: u32) -> Result<Arc<Capture>, Diagnostic> {
    let session = state
        .0
        .lock()
        .map_err(|_| Diagnostic::new("Session", "Session unavailable."))?;
    if session.generation != session_id {
        return Err(Diagnostic::new(
            "StaleSession",
            "This capture has been closed.",
        ));
    }
    session
        .capture
        .clone()
        .ok_or_else(|| Diagnostic::new("NoCapture", "Open a capture first."))
}
#[tauri::command]
async fn open_capture(app: tauri::AppHandle) -> Result<Option<Opened>, Diagnostic> {
    let guard = app.state::<OperationGate>().enter()?;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        let Some(path) = app
            .dialog()
            .file()
            .set_title("Open RxMER capture")
            .blocking_pick_file()
        else {
            return Ok(None);
        };
        let path = path
            .into_path()
            .map_err(|error| Diagnostic::new("Io", error))?;
        let capture = Arc::new(Capture::read(&path)?);
        let metadata = capture.metadata()?;
        let viewport = capture.viewport(0)?;
        let state = app.state::<Sessions>();
        let mut session = state
            .0
            .lock()
            .map_err(|_| Diagnostic::new("Session", "Session unavailable."))?;
        session.generation = session.generation.checked_add(1).ok_or_else(|| {
            Diagnostic::new("Session", "Restart the viewer to open another session.")
        })?;
        session.capture = Some(capture);
        Ok(Some(Opened {
            session_id: session.generation,
            metadata,
            viewport,
        }))
    })
    .await
    .map_err(|error| Diagnostic::new("Worker", error))?
}
#[tauri::command]
fn viewport(
    state: State<'_, Sessions>,
    session_id: u32,
    start: usize,
) -> Result<Viewport, Diagnostic> {
    selected(&state, session_id)?.viewport(start)
}
#[tauri::command]
fn close_capture(state: State<'_, Sessions>, session_id: u32) -> Result<(), Diagnostic> {
    let mut session = state
        .0
        .lock()
        .map_err(|_| Diagnostic::new("Session", "Session unavailable."))?;
    if session.generation == session_id {
        session.capture = None;
    }
    Ok(())
}
#[tauri::command]
async fn export_json(app: tauri::AppHandle, session_id: u32) -> Result<bool, Diagnostic> {
    let capture = selected(&app.state::<Sessions>(), session_id)?;
    let guard = app.state::<OperationGate>().enter()?;
    tauri::async_runtime::spawn_blocking(move || {
        let _guard = guard;
        let Some(path) = app
            .dialog()
            .file()
            .set_title("Export full-resolution JSON (new file)")
            .set_file_name("rxmer.json")
            .add_filter("JSON", &["json"])
            .blocking_save_file()
        else {
            return Ok(false);
        };
        let path = path
            .into_path()
            .map_err(|error| Diagnostic::new("Io", error))?;
        let json = capture.json()?;
        let mut output = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| {
                Diagnostic::new("Export", format!("Choose a new destination. {error}"))
            })?;
        if let Err(error) = output
            .write_all(json.as_bytes())
            .and_then(|()| output.sync_all())
        {
            drop(output);
            let _cleanup = std::fs::remove_file(&path);
            return Err(Diagnostic::new("Export", error));
        }
        Ok(true)
    })
    .await
    .map_err(|error| Diagnostic::new("Worker", error))?
}
/// Starts the offline desktop viewer and its constrained native commands.
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(Sessions::default())
        .manage(OperationGate::default())
        .invoke_handler(tauri::generate_handler![
            open_capture,
            viewport,
            close_capture,
            export_json
        ])
        .run(tauri::generate_context!())
        .expect("Unable to start PNM Viewer Toolkit");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sessions_reject_stale_and_closed_handles() {
        let sessions = Sessions(Mutex::new(Session {
            generation: 2,
            capture: Some(Arc::new(
                Capture::from_bytes(pnm_fixtures::RXMER.to_vec(), "rxmer.bin".into()).unwrap(),
            )),
        }));
        assert!(selected(&sessions, 1).is_err());
        assert!(selected(&sessions, 2).is_ok());
        sessions.0.lock().unwrap().capture = None;
        assert!(selected(&sessions, 2).is_err());
    }
    #[test]
    fn only_one_file_operation_runs_at_a_time() {
        let gate = OperationGate::default();
        let guard = gate.enter().unwrap();
        assert!(gate.enter().is_err());
        drop(guard);
        assert!(gate.enter().is_ok());
    }
}
