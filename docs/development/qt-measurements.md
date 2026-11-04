# Qt measurement internals

[Developer index](README.md) · [Qt setup](qt.md) · [User guide](../ui/measurements.md)

Rust owns parsing, analysis and exports. QML renders bounded viewports and
transient controls. This page describes implementation, not setup or a release
validation record. Current user behavior and screenshots live in the user guide.

## Compact workspace

The top row is Open, Settings and About, left-aligned, with the navigation toggle
at the far right. Open launches the native file chooser; Settings → System
contains appearance. About shows version, licenses and the project link.
Navigation starts hidden. There is no right panel or bottom status bar; the
main content fills the available space with minimal margins. Capture metadata,
Export JSON and Close capture are in the chart header. Busy work is indicated
in the toolbar and errors remain visible in the content area. Preferences are
session-only.

Each measurement chart fills the available height below slim inline metadata chips.
Drag across the plot to select a frequency range and release to zoom into it.
The bottom-left readout shows the selected frequency range in MHz, updating
while you drag. Scroll also zooms around the pointer; double-click resets the
full capture. The heading identifies the measurement without a filename. There are no paging buttons or sample slider. The download and × icon buttons export
JSON and close the capture; tooltips identify both actions.

Rust returns at most 1,024 plotted points per zoom range. Wide views retain
ordered bucket minima/maxima and range endpoints. Buckets containing missing
readings appear as gaps, conservatively hiding neighboring valid readings until
you zoom in. Ranges of 1,024 samples or fewer show every original reading.
The graph has no hover tooltip. JSON export always includes all samples.

Metadata chips follow the PyPNM-WebUi single-capture presentation: pill borders,
subtle theme-aware fill, and an accent label beside its value. The Qt controls
are implemented locally; the reference now lives in
[PyPNM-CMTS-WebUI](https://github.com/PyPNMApps/PyPNM-CMTS-WebUI/blob/main/src/assets/styles.css).

## Spectrum frequency interpretation

The existing core Spectrum parser supplies signed hundredths of dBmV. The
viewer uses contiguous segments advancing by SegmentFrequencySpan, including
truncated final segments, per CM-OSSI 3.1 I27 and 4.0 I13 D.2.4.1.5 and the
Spectrum file tables (58 and 91). Bin-center placement follows PyPNM's analysis
convention, with each rational position rounded down to integer Hz instead of
rounding the spacing first. This avoids accumulating frequency drift. Short
captures are not stretched to the configured last segment. Invalid grids are
reported rather than plotted. No smoothing or total-channel-power calculation
is applied. Specification checks used versioned text extracts, not rendered PDFs.

The shared Qt chart is `MeasurementChart.qml`; Rust owns chart metadata, frequency
mapping, vertical bounds, and bounded overviews in `apps/viewer/src/chart.rs`.
The retained Tauri UI remains RxMER-only and keeps its existing DTO contract.

Spectrum native validation on Ubuntu: the fixture displayed 20,736 bins across
296.264648–903.735351 MHz. Dark/light display, drag zoom/reset, JSON export,
close, and retaining the active chart after an unsupported open all passed.
Export preserved 81 segments of 256 bins with endpoints -22.8 and -77.3 dBmV.

## Spectrum bandwidth analysis

Rust calculates RBW as bin spacing multiplied by the normalized ENBW factor.
The raw DOCSIS ENBW field is in hundredths of bin spacing, so divide it by 100
before calling `common::signal_processing::spectrum::resolution_bandwidth_hz`.
Bandwidth calculations retain fractional Hz; they do not alter integer-Hz
frequency fields. Zero/nonfinite bandwidth inputs are rejected.

The viewer's chart metadata and JSON export include:

```json
{
  "spectrum_analysis": {
    "bin_spacing_hz": 29296.875,
    "equivalent_noise_bandwidth_factor": 1.1,
    "resolution_bandwidth_hz": 32226.5625
  }
}
```

These are derived fields alongside the original decoded file fields. Floating
point serialization can include insignificant trailing digits. The chip displays
`RBW 32.227 kHz`. Core parser/CLI JSON remains raw decoded data; RxMER export is
unchanged. The pure DSP function has no DOCSIS, JSON or UI dependency.

## Downstream channel estimation

Open `tests/fixtures/pypnm/tests/files/channel_estimation.bin`. The 7,480-carrier
fixture opens with Magnitude, Group delay and I/Q scatter tabs, inspired by
[PyPNM-WebUI's channel-estimation view](https://github.com/PyPNMApps/PyPNM-CMTS-WebUI/blob/25f40bda432f3c72df1e606bb229ffc5ba4eacf8/src/pw/features/operations/SingleChannelEstCoeffCaptureView.tsx). Switching tabs resets to
the full capture. Magnitude and delay retain drag selection, wheel zoom and
double-click reset. Neither plot displays hover tooltips.

Rust calculates unsmoothed magnitude as `20 log10(hypot(I, Q))` in dB and group
delay as `-d(phase)/df / (2*pi)` in seconds, converted to microseconds for display. Phase is unwrapped independently
within contiguous valid runs. Excluded, nonfinite and zero pairs produce null
magnitude/delay rather than artificial values; isolated valid carriers have no
delay. Interior derivatives are central, edges use second-order differences for
three or more carriers, and two-carrier runs use their shared slope. Phase steps
larger than pi are ambiguous; these plots do not recover absolute cable delay.
No echo detection, cable-distance estimate or smoothing is included yet.

I/Q scatter is a 32-by-32 density overview with equal I/Q scales. Every valid
pair contributes to one cell; color intensity indicates relative count. Zero pairs
remain valid here. The footer reports valid and excluded counts. This view
shows the full capture and has no frequency zoom or individual-sample tooltip.
At most 1,024 occupied cells cross the UI boundary.

Desktop JSON export preserves every raw parser field, including the full `iq`
array, and adds this section (arrays align with the original carrier indices):

```json
{
  "channel_estimation_analysis": {
    "frequencies_hz": [640000000, 640025000],
    "magnitude_db": [0.0, null],
    "group_delay": [
      {"frequency_hz": 640000000, "delay_s": 2.154e-6},
      {"frequency_hz": 640025000, "delay_s": null}
    ]
  }
}
```

Delay is exported as numeric seconds, using scientific notation for small
values; JSON readers may show equivalent decimals. Qt displays microseconds.
The numbers above illustrate the schema, not the fixture. Core parser and
`pnm-decode` JSON remain raw; the desktop export adds derived analysis. The
reusable math lives in `crates/core/src/common/signal_processing/channel_response.rs`;
`apps/viewer/src/channel_estimation.rs` owns capture mapping and presentation.

## Downstream histogram

Open `tests/fixtures/pypnm/tests/files/histogram.bin` for the 256-bin fixture.
The chart follows PyPNM-WebUI's single histogram view: hit count on the vertical
axis and zero-based bin index on the horizontal axis. The MAC chip identifies
the capture. Drag across bins to zoom, use the wheel to change the range, and
double-click to reset. The footer shows the selected bin range and its total hits.
There is no frequency axis or hover tooltip.

Rust returns at most 1,024 bars. Larger ranges sum contiguous bins, explicitly
labeled "summed bins"; zooming reveals the original individual counts. Sums use
64-bit integers and preserve every hit in the selected range. Zero bins stay
zero; empty captures show an empty-state label. Qt only renders the supplied bars.

Export retains the parser's full `hit_count_values`, `dwell_count_values` and
encoded `symmetry` fields. No probability normalization, symmetry reconstruction
or amplitude calibration is inferred. This UI adds no derived JSON section.

## FEC Summary

The viewer routes FEC summary files through the existing core parser. The Rust
presentation module (`apps/viewer/src/fec_summary.rs`) supplies one selected
profile, at most 1024 extrema-preserving chronological points, exact window sums
and last-interval counts. QML renders explicit total/error axes, UTC labels and
series visibility. Profile changes reset the range; UTC selection maps to record
indices in Rust. Known interval gaps are preserved conservatively in overviews.

Display ordering is stable chronological order; original bytes and JSON ordering
remain unchanged. Counters are interval counts. Summary type 2 uses one-second
intervals and type 3 uses one-minute intervals; other values retain their numeric
label and do not infer missing samples. No error rates are calculated.

## Drag-and-drop opening

`CaptureDropArea.qml` covers the window overlay, including the toolbar, and
routes one local file URL to the same Rust `open_file` worker as the picker.
File type/limits/readability validation remain in Rust. The drag target explicitly
accepts CopyAction, never MoveAction, to preserve the source file. Busy operations,
dialogs and the settings menu disable drops. Rejected inputs retain the capture.
QML tests cover routing, copy semantics, URL/cardinality filters and blocked state.
Native file-manager drag integration still requires verification on each OS.

## Modulation Profile

`apps/viewer/src/modulation_profile.rs` expands bounded range/skip assignments
using the existing decoder and the shared Shannon helper in
`crates/core/src/common/signal_processing/shannon.rs`. QML reuses the common
frequency chart with step rendering and adds profile toggles, All and metric
controls. Selected source indices live in Rust; metadata exposes selected_indices
and viewport.modulation_profiles.series carries bounded traces with stable
source indices and IDs. Empty selections keep the shared frequency axes.
Source indices identify profiles even when IDs repeat. Channel-estimation/FEC
views reject modulation-specific selections. All expanded work is bounded by
ParseLimits (65,536 carrier assignments across selected profiles) and checked
u32 Hz arithmetic. All traces share 1,024 display points plus two axis anchors.
The full longest-profile frequency window stays independent of selection;
shorter traces are clipped to their own data. JSON export includes all original
profiles regardless of display selection.

See the [measurement guide](../ui/measurements.md#modulation-profile) for theoretical
metric limits, special assignments and the explicit PyPNM frequency convention.
Independent Shannon tests live in `test_shannon.rs`; dedicated viewer tests cover
fixtures, skip parity, special codes, expansion limits, ranges and overflow.


### Modulation carrier presentation and capacity

`MeasurementToggle.qml` gives profile, carrier and All buttons a shared size and
style. `ModulationControls.qml` aligns the short Y axis selector with both toggle
rows; explanatory/count prose is omitted from the controls. The chart's vertical
title is dB or Bit loading.

Rust owns carrier classification/filtering and viewport grouping. The viewport
includes `carrier_lanes` plus `markers` with profile identity, carrier type,
first/last Hz and count. At most 4,096 special-carrier markers accompany the
shared 1,024 data points and two axis anchors. Groups report actual type counts
in their interval, preserving gaps and never substituting a fake data value.
QML draws these in categorical lanes and formats hover details. Data, profile
and metric selections stay independent and persist across each other's changes.

Core `pnm::analysis::modulation_profile` computes ideal capacity from compressed
schemes and shared `common::signal_processing::ofdm` arithmetic. CLI and viewer
exports use the same capture serializer. See the
[JSON capacity contract](../reference/formats.md#per-profile-ideal-capacity-analysis)
for formulas, null handling, evidence and excluded overheads.


The **Profile BW** chip is implemented by `ProfileBandwidth.qml`. It opens a
modal, scrollable profile/bandwidth grid without changing selection or rate
values. The chip and grid consume `ProfileChoice.bandwidth_bps`,
`bandwidth_text` and `bandwidth_summary`, calculated/formatted in Rust using the
same core analysis as `profiles[].bandwidth_bps`. No scientific arithmetic or
JSON mutation is performed in QML. Metadata replacement closes the popup;
modal display blocks file drops and main-window file actions.

## Constellation Display

See [QAM resource conversion and decision geometry](qam-resources.md) for build-time
reference generation and Rust viewport clipping. QML renders Soft decisions,
Hard decisions and Decision boundaries; numeric axes and count footers are omitted.
JSON exports retain the full capture and reference geometry.
