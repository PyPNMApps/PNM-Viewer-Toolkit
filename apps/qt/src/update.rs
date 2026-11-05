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
        #[qproperty(bool, install_available)]
        #[qproperty(QString, install_state)]
        #[qproperty(QString, install_status)]
        #[qproperty(u32, install_progress)]
        type UpdateBackend = super::UpdateBackendRust;

        /// Starts one bounded release check independently of capture operations.
        /// Calls while checking do nothing; results return to the Qt thread.
        #[qinvokable]
        fn check_updates(self: Pin<&mut UpdateBackend>);
        /// Downloads the selected release and requests administrator installation.
        #[qinvokable]
        fn install_update(self: Pin<&mut UpdateBackend>);
        /// Cancels downloads before administrator authentication begins.
        #[qinvokable]
        fn cancel_install(self: Pin<&mut UpdateBackend>);
        /// Starts the new viewer after a successful installation.
        #[qinvokable]
        fn restart_updated(self: Pin<&mut UpdateBackend>) -> bool;
    }
    impl cxx_qt::Threading for UpdateBackend {}
}
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use std::{
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use viewer::updates::release::{check_release, RELEASES_URL};
use viewer::updates::ubuntu::{self, InstallPlan};

/// Small Rust-owned release state exposed to About and the update indicator.
pub struct UpdateBackendRust {
    update_state: QString,
    update_status: QString,
    latest_version: QString,
    release_url: QString,
    install_available: bool,
    install_state: QString,
    install_status: QString,
    install_progress: u32,
    plan: Option<InstallPlan>,
    canceled: Arc<AtomicBool>,
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
            install_available: false,
            install_state: QString::from("idle"),
            install_status: QString::default(),
            install_progress: 0,
            plan: None,
            canceled: Arc::new(AtomicBool::new(false)),
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
        if self.update_state().to_string() == "checking"
            || matches!(
                self.install_state().to_string().as_str(),
                "downloading" | "installing" | "installed" | "restarting"
            )
        {
            return;
        }
        self.as_mut().set_update_state(QString::from("checking"));
        self.as_mut()
            .set_update_status(QString::from("Checking for updates…"));
        self.as_mut().set_install_available(false);
        self.as_mut().rust_mut().plan = None;
        let qt_thread = self.qt_thread();
        let worker = std::thread::Builder::new()
            .name("release-update-check".into())
            .spawn(move || {
                let result = check_release(env!("CARGO_PKG_VERSION"));
                let _queued = qt_thread.queue(move |mut controller| match result {
                    Ok(release) => {
                        let can_install = release.state == "available"
                            && release.ubuntu.is_some()
                            && ubuntu::supported();
                        controller.as_mut().rust_mut().plan = release.ubuntu;
                        controller.as_mut().set_install_available(can_install);
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

impl bridge::UpdateBackend {
    /// Downloads verified packages off the UI thread, then invokes Polkit.
    /// # Returns
    /// Immediately; progress and completion are queued onto the Qt thread.
    pub fn install_update(mut self: Pin<&mut Self>) {
        if !*self.install_available()
            || matches!(
                self.install_state().to_string().as_str(),
                "downloading" | "installing" | "installed" | "restarting"
            )
        {
            return;
        }
        let Some(plan) = self.rust().plan.clone() else {
            return;
        };
        let canceled = Arc::new(AtomicBool::new(false));
        self.as_mut().rust_mut().canceled = canceled.clone();
        self.as_mut()
            .set_install_state(QString::from("downloading"));
        self.as_mut()
            .set_install_status(QString::from("Downloading and verifying both packages…"));
        self.as_mut().set_install_progress(0);
        let qt_thread = self.qt_thread();
        let worker = std::thread::Builder::new().name("ubuntu-update-install".into()).spawn(move || {
            let result = ubuntu::download(&plan, &canceled, |progress| {
                let _queued = qt_thread.queue(move |mut controller| controller.as_mut().set_install_progress(progress));
            }).and_then(|directory| {
                if canceled.load(Ordering::Relaxed) {
                    return Err("Update canceled. The installed version is unchanged.".into());
                }
                let _queued = qt_thread.queue(|mut controller| {
                    controller.as_mut().set_install_state(QString::from("installing"));
                    controller.as_mut().set_install_status(QString::from("Approve the operating system's administrator prompt. Installing both packages…"));
                });
                ubuntu::install(directory.path())
            });
            let _queued = qt_thread.queue(move |mut controller| {
                match result {
                    Ok(()) => {
                        controller.as_mut().set_install_available(false);
                        controller.as_mut().set_update_state(QString::from("installed"));
                        controller.as_mut().set_update_status(QString::from("Update installed. Restarting the viewer…"));
                        controller.as_mut().set_install_status(QString::from("Update installed. Restarting the viewer…"));
                        controller.as_mut().set_install_state(QString::from("installed"));
                    }
                    Err(message) => {
                        controller.as_mut().set_install_state(QString::from("error"));
                        controller.as_mut().set_install_status(QString::from(message));
                    }
                }
            });
        });
        if worker.is_err() {
            self.as_mut().set_install_state(QString::from("error"));
            self.as_mut()
                .set_install_status(QString::from("Unable to start the update worker."));
        }
    }
    /// Signals download cancellation; authentication/apt cannot be interrupted here.
    /// # Returns
    /// Immediately; the worker reports cancellation on its next read.
    pub fn cancel_install(self: Pin<&mut Self>) {
        if self.install_state().to_string() == "downloading" {
            self.rust().canceled.store(true, Ordering::Relaxed);
        }
    }
    /// Launches the updated installed executable before QML closes this window.
    /// # Returns
    /// True only after successful installation and child process creation.
    pub fn restart_updated(mut self: Pin<&mut Self>) -> bool {
        if self.install_state().to_string() != "installed" {
            return false;
        }
        match ubuntu::restart() {
            Ok(_) => {
                self.as_mut().set_install_state(QString::from("restarting"));
                true
            }
            Err(message) => {
                self.as_mut().set_install_status(QString::from(format!(
                    "Update installed, but the viewer could not restart: {message}. Try Restart viewer again."
                )));
                false
            }
        }
    }
}
