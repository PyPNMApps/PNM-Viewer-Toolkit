/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Full-resolution spectrum result with independent zoom/smoothing/range tabs.
use super::{Collection, Diagnostic};
use crate::capture::{
    chart::{overview_with_limit, ChartPoint},
    collection::joint_indices,
    MAX_VIEW_SAMPLES,
};
use pnm::{
    common::signal_processing::{
        spectrum::window_average_dbmv,
        spectrum_power::{integrated_level_dbmv, SpectrumRange},
    },
    pnm::analysis::spectrum::SpectrumAggregate,
};
use serde_json::{json, Value};
use std::sync::Arc;
#[derive(Clone)]
struct Tab {
    title: String,
    start: usize,
    count: usize,
    window: u32,
    first: usize,
    length: usize,
    power: Option<(f64, f64)>,
}
/// A selected aggregate/playback frame, independent from single-capture state.
pub struct Session {
    collection: Arc<Collection>,
    pub result: SpectrumAggregate,
    pub group: usize,
    pub frame: Option<usize>,
    tabs: Vec<Tab>,
    active: usize,
}
impl Session {
    /// Creates a selected settings-group result, preserving full-resolution values.
    /// # Arguments
    /// * `collection` - Immutable imported captures, shared without copying files.
    /// * `group`, `frame` - Catalog settings ID and optional ordered frame index.
    /// # Returns
    /// Full-frequency session with one original range tab.
    /// # Errors
    /// Propagates unknown selection and aggregation errors.
    pub fn new(
        collection: Arc<Collection>,
        group: usize,
        frame: Option<usize>,
    ) -> Result<Self, Diagnostic> {
        let result = collection.aggregate(group, frame)?;
        let count = result.points.len();
        Ok(Self {
            collection,
            result,
            group,
            frame,
            tabs: vec![Tab {
                title: "Full spectrum".into(),
                start: 0,
                count,
                window: 0,
                first: 0,
                length: count,
                power: None,
            }],
            active: 0,
        })
    }
    /// Advances playback within the selected acquisition settings, keeping ranges.
    /// # Arguments
    /// * `frame` - Ascending epoch position in this group.
    /// # Returns
    /// The new original frame with unchanged tab, zoom and smoothing settings.
    /// # Errors
    /// Rejects unknown frames without modifying the current result.
    pub fn show_frame(&mut self, frame: usize) -> Result<(), Diagnostic> {
        let result = self.collection.aggregate(self.group, Some(frame))?;
        self.result = result;
        self.frame = Some(frame);
        Ok(())
    }
    /// Updates the active tab's zoom using source-bin indices.
    /// # Arguments
    /// * `start`, `count` - Nonempty bounded source range.
    /// # Returns
    /// Success after changing transient zoom only.
    /// # Errors
    /// Rejects out-of-capture source ranges.
    pub fn range(&mut self, start: usize, count: usize) -> Result<(), Diagnostic> {
        if count == 0
            || start
                .checked_add(count)
                .is_none_or(|end| end > self.result.points.len())
        {
            return Err(Diagnostic::new(
                "SpectrumRange",
                "Select bins inside this spectrum.",
            ));
        }
        let tab = &mut self.tabs[self.active];
        let end = (start + count).min(tab.first + tab.length);
        let start = start.max(tab.first);
        if start >= end {
            return Err(Diagnostic::new(
                "SpectrumRange",
                "Select bins inside this range tab.",
            ));
        }
        tab.start = start;
        tab.count = end - start;
        Ok(())
    }
    /// Sets centered frequency smoothing on the current tab, distinct from time averaging.
    /// # Arguments
    /// * `window` - Positive N-point window bounded by the existing smoothing policy.
    /// # Returns
    /// Stored smoothing size; acquisition RBW is unchanged.
    /// # Errors
    /// Rejects zero or excessive windows before replacing the current setting.
    pub fn average(&mut self, window: u32) -> Result<(), Diagnostic> {
        if window == 0 {
            return Err(Diagnostic::new(
                "SpectrumWindow",
                "Enter a positive whole-number window.",
            ));
        }
        self.tabs[self.active].window = window;
        Ok(())
    }
    /// Validates user frequency endpoints against this complete source grid.
    /// # Arguments
    /// * `start`, `stop`, `unit` - Ordered values in Hz, kHz, MHz or GHz.
    /// # Returns
    /// Exact Hz endpoints and inclusive source-center index range.
    /// # Errors
    /// Rejects unknown units, nonfinite/outside/reversed or sub-two-bin ranges.
    pub fn band(
        &self,
        start: f64,
        stop: f64,
        unit: &str,
    ) -> Result<(f64, f64, usize, usize), Diagnostic> {
        let factor = match unit {
            "Hz" => 1.0,
            "kHz" => 1e3,
            "MHz" => 1e6,
            "GHz" => 1e9,
            _ => {
                return Err(Diagnostic::new(
                    "SpectrumUnit",
                    "Select Hz, kHz, MHz or GHz.",
                ))
            }
        };
        let (start, stop) = (start * factor, stop * factor);
        let points = &self.result.points;
        if !start.is_finite()
            || !stop.is_finite()
            || start >= stop
            || start < f64::from(points[0].frequency_hz)
            || stop > f64::from(points.last().unwrap().frequency_hz)
        {
            return Err(Diagnostic::new(
                "SpectrumRange",
                "Start must precede Stop within the capture frequency bounds.",
            ));
        }
        let first = points.partition_point(|point| f64::from(point.frequency_hz) < start);
        let end = points.partition_point(|point| f64::from(point.frequency_hz) <= stop);
        if end - first < 2 {
            return Err(Diagnostic::new(
                "SpectrumRange",
                "Select at least two source bin centers.",
            ));
        }
        Ok((start, stop, first, end - first))
    }
    /// Opens a bounded independent range tab from validated frequency input.
    /// # Arguments
    /// * `start`, `stop`, `unit` - Requested source-frequency interval.
    /// # Returns
    /// Selects the new tab with its own zoom, smoothing and power selection.
    /// # Errors
    /// Rejects invalid bounds or more than sixteen tabs.
    pub fn open_band(&mut self, start: f64, stop: f64, unit: &str) -> Result<(), Diagnostic> {
        let (start, stop, first, count) = self.band(start, stop, unit)?;
        if self.tabs.len() >= 16 {
            return Err(Diagnostic::new(
                "SpectrumTabs",
                "Close a frequency tab before opening another.",
            ));
        }
        self.tabs.push(Tab {
            title: format!("{:.3}–{:.3} MHz", start / 1e6, stop / 1e6),
            start: first,
            count,
            window: 0,
            first,
            length: count,
            power: None,
        });
        self.active = self.tabs.len() - 1;
        Ok(())
    }
    /// Selects or closes an existing range tab while preserving its presentation.
    /// # Arguments
    /// * `index` - Existing tab ID; the original tab cannot be closed.
    /// * `close` - Remove a derived tab instead of selecting it.
    /// # Returns
    /// Updated active-tab selection; invalid IDs are ignored.
    pub fn tab(&mut self, index: usize, close: bool) {
        if index >= self.tabs.len() {
            return;
        }
        if !close {
            self.active = index;
            return;
        }
        if index == 0 {
            return;
        }
        self.tabs.remove(index);
        match self.active.cmp(&index) {
            std::cmp::Ordering::Equal => self.active = 0,
            std::cmp::Ordering::Greater => self.active -= 1,
            std::cmp::Ordering::Less => {}
        }
    }
    /// Stores a power-selection band using source-view coordinates, never plot samples.
    /// # Arguments
    /// * `first`, `last` - Mouse fractions in [0,1], in either direction.
    /// # Returns
    /// A validated interval; power is recalculated against the active original/mean trace.
    /// # Errors
    /// Rejects invalid or empty selections.
    pub fn power(&mut self, first: f64, last: f64) -> Result<(), Diagnostic> {
        if !first.is_finite()
            || !last.is_finite()
            || !(0.0..=1.0).contains(&first)
            || !(0.0..=1.0).contains(&last)
            || first == last
        {
            return Err(Diagnostic::new(
                "SpectrumRange",
                "Select a frequency band inside the graph.",
            ));
        }
        let tab = &self.tabs[self.active];
        let points = &self.result.points;
        let lower = f64::from(points[tab.start].frequency_hz);
        let upper = f64::from(points[tab.start + tab.count - 1].frequency_hz);
        self.tabs[self.active].power = Some((
            lower + first.min(last) * (upper - lower),
            lower + first.max(last) * (upper - lower),
        ));
        Ok(())
    }
    /// Builds bounded curves and compact workspace metadata from full-resolution values.
    /// # Returns
    /// JSON presentation DTO with synchronized min/mean/max extrema, optional window
    /// average, source chronology, validated power chips and independent tab metadata.
    /// # Errors
    /// Rejects invalid smoothing or unavailable integrated source data.
    pub fn viewport(&self) -> Result<Value, Diagnostic> {
        let tab = &self.tabs[self.active];
        let source = &self.result.points[tab.start..tab.start + tab.count];
        let indices = joint_indices(source, |point, field| match field {
            0 => point.minimum_dbmv,
            1 => point.average_dbmv,
            _ => point.maximum_dbmv,
        });
        let curve = |field| {
            indices.iter().enumerate().map(|(position,index)| {
            let point=&source[*index];
            let value=if position>0 && source[indices[position-1]..=*index].iter().any(|point|point.valid_count==0) {None} else {match field {0=>point.minimum_dbmv,1=>point.average_dbmv,_=>point.maximum_dbmv}};
            json!({"sample_index":point.sample_index,"frequency_hz":point.frequency_hz,"value":value})
        }).collect::<Vec<_>>()
        };
        let values: Vec<_> = self
            .result
            .points
            .iter()
            .map(|point| point.average_dbmv)
            .collect();
        let mut view = json!({"points":curve(1),"start":tab.start,"count":tab.count,"total":self.result.points.len(),"group":self.result.group,"group_id":self.group,
            "capture_count":self.result.sources.len(),"first_epoch":self.result.sources.first().unwrap().capture_time,"last_epoch":self.result.sources.last().unwrap().capture_time,
            "resolution_bandwidth_hz":self.result.group.resolution_bandwidth_hz(),"frame_count":self.collection.catalog.groups[self.group].capture_count,"frame_index":self.frame.unwrap_or(0),"capture_epoch":self.result.sources[0].capture_time});
        if self.frame.is_none() {
            view["spectrum_statistics"] = json!({"minimum":curve(0),"maximum":curve(2)});
        }
        if tab.window > 0 {
            let average = window_average_dbmv(&values, tab.window)
                .ok_or_else(|| Diagnostic::new("SpectrumWindow", "Invalid window average."))?;
            let points: Vec<_> = self
                .result
                .points
                .iter()
                .zip(average)
                .map(|(point, value)| ChartPoint {
                    sample_index: point.sample_index,
                    frequency_hz: point.frequency_hz,
                    value,
                })
                .collect();
            view["spectrum_average"] = json!({"window_points":tab.window,"points":overview_with_limit(&points[tab.start..tab.start+tab.count],MAX_VIEW_SAMPLES/2)});
        }
        let valid: Vec<_> = source
            .iter()
            .flat_map(|point| [point.minimum_dbmv, point.maximum_dbmv])
            .flatten()
            .collect();
        let low = valid
            .iter()
            .copied()
            .min_by(f64::total_cmp)
            .unwrap_or(-80.0);
        let high = valid.iter().copied().max_by(f64::total_cmp).unwrap_or(0.0);
        view["minimum"] = json!(((low - 5.0) / 10.0).floor() * 10.0);
        view["maximum"] = json!(((high + 5.0) / 10.0).ceil() * 10.0);
        let mut power = Value::Null;
        if let Some((start_hz, stop_hz)) = tab.power {
            let key = &self.result.group;
            let first_center = f64::from(key.first_segment_center_hz)
                - f64::from(key.segment_span_hz) / 2.0
                + key.bin_spacing_hz() / 2.0;
            let level = integrated_level_dbmv(
                &values,
                first_center,
                key.bin_spacing_hz(),
                key.resolution_bandwidth_hz().unwrap(),
                SpectrumRange { start_hz, stop_hz },
            )
            .ok_or_else(|| {
                Diagnostic::new(
                    "SpectrumPower",
                    "Cannot integrate missing or invalid spectrum bins.",
                )
            })?;
            power = json!({"start_hz":start_hz,"stop_hz":stop_hz,"bandwidth_hz":stop_hz-start_hz,"level_dbmv":level});
        }
        view["workspace"] = json!({"tabs":self.tabs.iter().map(|tab|json!({"title":tab.title})).collect::<Vec<_>>(),"active":self.active,"minimum_hz":self.result.points[0].frequency_hz,"maximum_hz":self.result.points.last().unwrap().frequency_hz,"power":power});
        Ok(view)
    }
}
