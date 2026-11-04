/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Independent update state: network workers never hold or block capture jobs.
#[cxx_qt::bridge]
mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, update_state)]
        #[qproperty(QString, update_status)]
        #[qproperty(QString, latest_version)]
        #[qproperty(QString, release_url)]
        type UpdateBackend = super::UpdateBackendRust;

        /// Starts one bounded release check independently of capture operations.
        /// Calls while checking do nothing; results return to the Qt thread.
        #[qinvokable]
        fn check_updates(self: Pin<&mut UpdateBackend>);
    }
    impl cxx_qt::Threading for UpdateBackend {}
}
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use std::pin::Pin;
use viewer::update::{check_release, RELEASES_URL};

/// Small Rust-owned release state exposed to About and the update indicator.
pub struct UpdateBackendRust {
    update_state: QString,
    update_status: QString,
    latest_version: QString,
    release_url: QString,
}
impl Default for UpdateBackendRust {
    /// Creates idle update state without network or filesystem access.
    ///
    /// # Returns
    /// A state object that can be checked explicitly after UI startup.
    fn default() -> Self {
        Self {
            update_state: QString::from("idle"),
            update_status: QString::from("Updates have not been checked."),
            latest_version: QString::default(),
            release_url: QString::from(RELEASES_URL),
        }
    }
}
impl bridge::UpdateBackend {
    /// Checks for a published stable release on a separate Rust worker.
    ///
    /// # Returns
    /// Immediately updates checking state, then queues the completed comparison
    /// onto the Qt thread. Overlapping requests are ignored. Failure is confined
    /// to update status; closing the controller safely rejects queued results.
    pub fn check_updates(mut self: Pin<&mut Self>) {
        if self.update_state().to_string() == "checking" {
            return;
        }
        self.as_mut().set_update_state(QString::from("checking"));
        self.as_mut()
            .set_update_status(QString::from("Checking for updates…"));
        let qt_thread = self.qt_thread();
        let worker = std::thread::Builder::new()
            .name("release-update-check".into())
            .spawn(move || {
                let result = check_release(env!("CARGO_PKG_VERSION"));
                let _queued = qt_thread.queue(move |mut controller| match result {
                    Ok(release) => {
                        controller.as_mut().set_latest_version(QString::from(
                            release.latest_version.as_deref().unwrap_or(""),
                        ));
                        controller
                            .as_mut()
                            .set_release_url(QString::from(&release.release_url));
                        controller
                            .as_mut()
                            .set_update_status(QString::from(&release.status));
                        controller
                            .as_mut()
                            .set_update_state(QString::from(release.state));
                    }
                    Err(message) => {
                        controller
                            .as_mut()
                            .set_update_status(QString::from(&message));
                        controller.as_mut().set_update_state(QString::from("error"));
                    }
                });
            });
        if worker.is_err() {
            self.as_mut().set_update_status(QString::from(
                "Unable to start the update check. Try again.",
            ));
            self.set_update_state(QString::from("error"));
        }
    }
}
