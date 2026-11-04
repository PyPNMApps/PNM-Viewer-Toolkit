# Qt Quick desktop prototype

For opening captures, chart controls and screenshots, start with the
[desktop user guide](../ui/README.md). This page covers developer setup and implementation.

The Qt prototype lives under `apps/qt/`. Tauri remains under `apps/desktop/`.
Both use the GPL-3.0-only Rust capture layer in `apps/viewer`, backed by the
existing Apache-2.0 parsers. Qt Quick renders directly through Qt without a
browser, WebView or HTTP backend. This is development software.

## Linux development

Prepare the existing toolchain with `./install.sh --development`. A C++17
compiler, CMake and Python venv support are also required. Then:

```bash
scripts/setup-qt.sh
scripts/qt-desktop.sh
```

The setup helper downloads Qt 6.8.3 and aqtinstall 3.3.0 into ignored `local/`;
it installs no system packages. The shared Python helper also supports native
Windows x64 and macOS arm64/x64 SDK setup (see below).
`QT_SDK_DIR` can select an existing Qt 6.8.3 SDK. CXX-Qt is pinned to 0.7.3,
with matching CXX runtime/generator versions and committed Cargo lockfiles.

```bash
scripts/qt-desktop.sh build
scripts/qt-desktop.sh check
```

## Workflow

The initial Home page shows its title and a centered Open capture button. Choose
Open capture or Open in the toolbar. RxMER, Spectrum Analyzer, DS Channel Estimation and Histogram format 1.0
captures are detected by file contents and open their matching chart. Close
returns to Home. Other measurements retain the unsupported-viewer diagnostic.

Try `tests/fixtures/pypnm/tests/files/rxmer.bin` or `spectrum_analyzer.bin` in
that same directory. Frequency charts use labeled chips, full-range overviews,
drag selection, mouse-wheel zoom, double-click reset, and a frequency-range readout.
Spectrum shows amplitude in dBmV with automatic vertical bounds; RxMER uses dB.
Spectrum chips show MAC and calculated RBW. Its
channel ID is not meaningful and is therefore not displayed.

Export JSON saves the entire decoded capture to a new file. Inputs and existing
destinations are never overwritten. File work runs on a Rust worker; overlapping
operations are prevented. A failed open preserves the previous capture. Close
releases capture bytes; destroying the window safely rejects pending callbacks.

System/Dark/Light are session preferences. System follows Qt's platform color
scheme; no preference file is written. Rust owns all parsing, filesystem work
and export. The generated CXX boundary is confined to the Qt adapter.

## Windows and macOS

Install Qt 6.8.3 with Declarative/Quick and tools, a native C++ compiler (MSVC
on Windows or Xcode tools on macOS), and the pinned Rust toolchain. In a native
developer terminal, add Qt's bin directory to PATH and set QMAKE to its qmake:

```text
cargo run --locked --manifest-path apps/qt/Cargo.toml
```

These source-build instructions do not establish native runtime verification.
Native package builders now cover Windows/macOS, CLI bundling and the size
gate; their hosted results remain pending. External signing/notarization remains
separate. The development binary requires Qt runtime dependencies.

## Validation

The check runs Rust formatting, adapter tests, Clippy, a native build and
Qt Quick tests for themes, missing readings, sample coordinates and mouse zoom
inspection. Shared capture tests cover fixtures, JSON and safe export. The
pre-push gate checks both shells. Native file pickers and platform theme behavior
also need desktop smoke tests; offscreen tests alone cannot verify OS integration.

## Verified on this PC

Ubuntu 22.04 x86_64: the full pre-push gate passed, including both shells. An
isolated X desktop exercised the actual Qt executable: native file picker,
fixture metadata and chart, Dark/Light selection, paging, JSON save and Close.
The export contained 7,480 readings, first value 42.75 dB, and was 78,925 bytes.
The retained Tauri executable passed the same native regression workflow.
System scheme switching is tested through Qt properties; host OS preferences
were not changed. Native Windows/macOS and installable packages remain unverified.

## No display connection

When DISPLAY is unset, the launcher first reads DISPLAY and XAUTHORITY from
this user's systemd session environment. This supports VS Code Remote on the VM
while displaying the window on its logged-in desktop, as in OSCAR-UI. Explicit
display settings are preserved. No desktop login or access permissions are created.

If no desktop settings are available, run the launcher from a terminal
opened inside your graphical desktop session. A plain SSH, text console or
remote editor terminal may have no display connection. Check with:

```bash
printf 'DISPLAY=%s\n' "$DISPLAY"
```

The bundled Linux SDK uses XCB, including XWayland on Wayland desktops, and
requires a valid DISPLAY and permission to connect to it. Do not blindly set
DISPLAY=:0: the correct display and authentication belong to the desktop session.
For remote use, connect with SSH X11 forwarding (`ssh -X user@host`) from a
machine with an X server and with server-side forwarding configured, then launch
from that forwarded session. A remote desktop session is another option.

Qt may also print an xcb-cursor warning after a display connection failure.
Verify actual missing libraries before installing packages:

```bash
ldd local/qt/6.8.3/gcc_64/plugins/platforms/libqxcb.so
```

On this PC libxcb-cursor and the other XCB dependencies resolve. Reinstalling Qt
is not needed for an empty DISPLAY. Offscreen mode is for automated tests and
will not show an interactive window.

The sidebar uses the README's original dark/light PNG logos. The native window
icon follows the same theme using the desktop icons in `docs/images/`. These
assets are embedded through `apps/qt/branding.qrc`; no runtime source-tree paths
or duplicate image files are needed.

## Native file chooser

On Linux the launcher defaults QT_QPA_PLATFORMTHEME to gtk3 so Open and Export
use the OS GTK file chooser instead of the Qt Quick fallback. The SDK includes
the GTK platform plugin; the host needs its GTK 3 runtime dependencies. Explicit
platform-theme overrides are preserved. Direct binary launches should set
QT_QPA_PLATFORMTHEME=gtk3 on this Ubuntu VM as well.

The OS controls the dialog appearance independently of the viewer theme. Press
Ctrl+L in the GTK chooser to show and edit the full path. Windows and macOS
continue using their platform defaults.

Reference: [Qt FileDialog native availability](https://doc.qt.io/qt-6.5/qml-qtquick-dialogs-filedialog.html).

Both desktop icon PNGs have transparent backgrounds outside their rounded cyan
outline. White areas inside the light icon remain part of the artwork. Matching
multi-resolution dark/light ICO favicons are available in `docs/images/` for the
retained web frontend; Qt continues to use the PNG window icons.

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

## Cross-platform CI and native release builds

`python3 scripts/qt-sdk.py setup` installs Qt 6.8.3 through the pinned aqt helper
in ignored `local/` storage on Linux x64, Windows x64, and macOS arm64/x64.
Use `python` on Windows from an x64 MSVC developer shell. The macOS SDK is
universal; Cargo builds for the native runner architecture. Explicit
`QT_SDK_DIR` overrides must already contain a complete SDK.

`python3 scripts/qt-sdk.py check` runs Qt formatting, Rust tests/Clippy/build and
offscreen QML tests on the native host. The existing Linux shell entry points
delegate setup/check behavior to this shared helper. Running the viewer on
Windows/macOS is possible from a configured developer shell with Cargo and Qt;
native hosted results remain pending until CI runs.

See [CI](ci.md) for all required runners and
[packaging](../release/packaging.md) for DEB, NSIS and DMG artifacts. The explicit
`--smoke-test INPUT_FILE_URL NEW_OUTPUT_FILE_URL` mode opens, renders and exports
one capture, logs `UI_SMOKE_OK` and exits. It uses the same Rust adapter and does
not overwrite existing outputs. Normal launches do not activate this mode.
