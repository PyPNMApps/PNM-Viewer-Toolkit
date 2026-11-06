/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Per-tab spectrum presentation, sharing the existing Arc<Capture>.
use super::*;
use serde_json::{json, Value};

const MAX_SPECTRUM_TABS: usize = 16;
#[derive(Default)]
pub(super) struct Workspace {
    tabs: Vec<BandTab>,
    active: usize,
    minimum_hz: u32,
    maximum_hz: u32,
}
struct BandTab {
    title: String,
    start: usize,
    count: usize,
    viewport: String,
    view: MeasurementView,
    power: Value,
}
impl bridge::ViewerBackend {
    /// Starts a fresh spectrum workspace after a successful capture load.
    /// # Returns
    /// One original tab for a nonempty spectrum, otherwise an empty workspace.
    pub(super) fn reset_spectrum_workspace(mut self: Pin<&mut Self>) {
        self.as_mut().rust_mut().spectrum_workspace = Workspace::default();
        let Some(capture) = self.rust().capture.as_ref() else {
            return;
        };
        if let Ok(metadata) = capture.chart_metadata() {
            if metadata.spectrum_analysis.is_some() && metadata.sample_count >= 2 {
                if let Ok(points) = capture.chart_range(0, metadata.sample_count) {
                    let workspace = Workspace {
                        tabs: vec![BandTab {
                            title: "Spectrum".into(),
                            start: 0,
                            count: metadata.sample_count,
                            viewport: self.viewport_json().to_string(),
                            view: MeasurementView::Default,
                            power: Value::Null,
                        }],
                        active: 0,
                        minimum_hz: points.points[0].frequency_hz,
                        maximum_hz: points.points.last().unwrap().frequency_hz,
                    };
                    self.as_mut().rust_mut().spectrum_workspace = workspace;
                }
            }
        }
        self.publish_spectrum_workspace();
    }
    fn publish_spectrum_workspace(mut self: Pin<&mut Self>) {
        let workspace = &self.rust().spectrum_workspace;
        let tabs: Vec<_> = workspace
            .tabs
            .iter()
            .map(|tab| json!({"title": tab.title}))
            .collect();
        let power = workspace.tabs.get(workspace.active).map(|tab| &tab.power);
        let state = json!({"tabs": tabs, "active": workspace.active,
            "minimum_hz": workspace.minimum_hz, "maximum_hz": workspace.maximum_hz, "power": power});
        self.as_mut()
            .set_spectrum_workspace_json(QString::from(state.to_string()));
    }
    /// Saves bounded presentation state in the currently active spectrum tab.
    /// # Returns
    /// Retains zoom/averaging without duplicating the shared capture bytes.
    pub(super) fn save_spectrum_tab(mut self: Pin<&mut Self>) {
        let viewport = self.viewport_json().to_string();
        let view = self.rust().view.clone();
        let workspace = &mut self.as_mut().rust_mut().spectrum_workspace;
        if let Some(tab) = workspace.tabs.get_mut(workspace.active) {
            tab.viewport = viewport;
            tab.view = view;
        }
    }
    /// Restricts index-based zoom/reset requests to the active frequency band.
    /// # Arguments
    /// * `start`, `count` - Requested source-bin window, potentially full capture.
    /// # Returns
    /// An intersected nonempty tab window; other measurements pass through.
    pub(super) fn spectrum_zoom_bounds(&self, start: usize, count: usize) -> (usize, usize) {
        let workspace = &self.rust().spectrum_workspace;
        if let Some(tab) = workspace.tabs.get(workspace.active) {
            let end = start.saturating_add(count).min(tab.start + tab.count);
            let start = start.max(tab.start).min(tab.start + tab.count - 1);
            return (start, end.saturating_sub(start).max(1));
        }
        (start, count)
    }
    /// Validates frequency input using the shared Rust capture API.
    /// # Arguments
    /// * `start`, `stop` - Frequency values in the selected unit.
    /// * `unit` - Hz, kHz, MHz or GHz.
    /// # Returns
    /// Empty text for valid input, otherwise a user-facing validation message.
    pub fn spectrum_range_error(&self, start: f64, stop: f64, unit: &QString) -> QString {
        let result = self
            .rust()
            .capture
            .as_ref()
            .ok_or_else(|| Diagnostic::new("NoCapture", "Open a spectrum capture first."))
            .and_then(|capture| capture.spectrum_band(start, stop, &unit.to_string(), false));
        result
            .err()
            .map(|error| QString::from(error.message))
            .unwrap_or_default()
    }
    /// Opens an independent range tab sharing the current capture bytes.
    /// # Arguments
    /// * `start`, `stop` - Ordered frequencies in `unit` within capture bounds.
    /// * `unit` - Hz, kHz, MHz or GHz.
    /// # Returns
    /// Selects a new bounded viewport, or reports failure without changing tabs.
    pub fn open_spectrum_band(mut self: Pin<&mut Self>, start: f64, stop: f64, unit: &QString) {
        if *self.busy() {
            return;
        }
        if self.rust().spectrum_workspace.tabs.len() >= MAX_SPECTRUM_TABS {
            self.fail(Diagnostic::new(
                "TabLimit",
                "Close a range tab before opening another (limit 16).",
            ));
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let result = capture
            .spectrum_band(start, stop, &unit.to_string(), false)
            .and_then(|band| {
                let viewport = capture.chart_range(band.first_sample, band.sample_count)?;
                Ok((
                    band,
                    serde_json::to_string(&viewport)
                        .map_err(|error| Diagnostic::new("Json", error))?,
                ))
            });
        match result {
            Ok((band, viewport)) => {
                self.as_mut().save_spectrum_tab();
                let tab = BandTab {
                    title: format!(
                        "{:.3}–{:.3} MHz",
                        band.start_hz / 1_000_000.0,
                        band.stop_hz / 1_000_000.0
                    ),
                    start: band.first_sample,
                    count: band.sample_count,
                    viewport: viewport.clone(),
                    view: MeasurementView::Default,
                    power: Value::Null,
                };
                let workspace = &mut self.as_mut().rust_mut().spectrum_workspace;
                workspace.tabs.push(tab);
                workspace.active = workspace.tabs.len() - 1;
                self.as_mut().rust_mut().view = MeasurementView::Default;
                self.as_mut().set_viewport_json(QString::from(viewport));
                self.as_mut().set_error_message(QString::default());
                self.publish_spectrum_workspace();
            }
            Err(error) => self.fail(error),
        }
    }
    /// Restores a tab's saved zoom and averaging without copying capture bytes.
    /// # Arguments
    /// * `index` - Existing zero-based tab index; invalid/busy calls are ignored.
    /// # Returns
    /// Restored viewport and active-tab metadata.
    pub fn select_spectrum_tab(mut self: Pin<&mut Self>, index: u32) {
        if *self.busy() || index as usize >= self.rust().spectrum_workspace.tabs.len() {
            return;
        }
        self.as_mut().save_spectrum_tab();
        let workspace = &mut self.as_mut().rust_mut().spectrum_workspace;
        workspace.active = index as usize;
        self.restore_spectrum_tab();
    }
    fn restore_spectrum_tab(mut self: Pin<&mut Self>) {
        let workspace = &self.rust().spectrum_workspace;
        let tab = &workspace.tabs[workspace.active];
        let viewport = tab.viewport.clone();
        let view = tab.view.clone();
        self.as_mut().rust_mut().view = view;
        self.as_mut().set_viewport_json(QString::from(viewport));
        self.publish_spectrum_workspace();
    }
    /// Closes a derived tab while retaining the original spectrum tab.
    /// # Arguments
    /// * `index` - Nonzero existing tab index; busy/invalid requests are ignored.
    /// # Returns
    /// Selects the original tab if the active derived tab is removed.
    pub fn close_spectrum_tab(mut self: Pin<&mut Self>, index: u32) {
        if *self.busy() || index == 0 || index as usize >= self.rust().spectrum_workspace.tabs.len()
        {
            return;
        }
        self.as_mut().save_spectrum_tab();
        let workspace = &mut self.as_mut().rust_mut().spectrum_workspace;
        workspace.tabs.remove(index as usize);
        match workspace.active.cmp(&(index as usize)) {
            std::cmp::Ordering::Equal => workspace.active = 0,
            std::cmp::Ordering::Greater => workspace.active -= 1,
            std::cmp::Ordering::Less => {}
        }
        self.restore_spectrum_tab();
    }
    /// Integrates a mouse-selected band using original bins and acquisition ENBW.
    /// # Arguments
    /// * `first`, `last` - Ordered or reversed normalized fractions in [0, 1]
    ///   of the currently visible source frequency range.
    /// # Returns
    /// Publishes integrated dBmV in the current tab, or reports a diagnostic.
    pub fn calculate_spectrum_power(mut self: Pin<&mut Self>, first: f64, last: f64) {
        if *self.busy() {
            return;
        }
        let Some(capture) = self.rust().capture.clone() else {
            return;
        };
        let result = (|| {
            if !first.is_finite()
                || !last.is_finite()
                || !(0.0..=1.0).contains(&first)
                || !(0.0..=1.0).contains(&last)
            {
                return Err(Diagnostic::new(
                    "InvalidRange",
                    "Select a frequency range inside the graph.",
                ));
            }
            let view: Value = serde_json::from_str(&self.viewport_json().to_string())
                .map_err(|error| Diagnostic::new("Json", error))?;
            let points = view["points"]
                .as_array()
                .ok_or_else(|| Diagnostic::new("InvalidRange", "No spectrum bins."))?;
            let lower = points
                .first()
                .and_then(|point| point["frequency_hz"].as_f64())
                .ok_or_else(|| Diagnostic::new("InvalidRange", "No spectrum bins."))?;
            let upper = points
                .last()
                .and_then(|point| point["frequency_hz"].as_f64())
                .unwrap_or(lower);
            capture.spectrum_band(
                lower + first.min(last) * (upper - lower),
                lower + first.max(last) * (upper - lower),
                "Hz",
                true,
            )
        })();
        match result {
            Ok(band) => {
                let workspace = &mut self.as_mut().rust_mut().spectrum_workspace;
                if let Some(tab) = workspace.tabs.get_mut(workspace.active) {
                    tab.power = json!(band);
                }
                self.as_mut().set_error_message(QString::default());
                self.publish_spectrum_workspace();
            }
            Err(error) => self.fail(error),
        }
    }
}
