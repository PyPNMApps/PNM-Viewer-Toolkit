# Qt Quick desktop prototype

For opening captures, chart controls and screenshots, start with the
[desktop user guide](../ui/README.md). This page covers developer setup and validation.

The Qt desktop lives under `apps/qt/`. It uses the GPL-3.0-only Rust capture
layer in `apps/viewer`, backed by the
existing Apache-2.0 parsers. Qt Quick renders directly through Qt without a
browser, WebView or HTTP backend. This is development software.

The executable selects Fusion controls before Qt startup, matching development
and CI. Native Windows/macOS control backgrounds may ignore custom palette roles,
causing pale text on white buttons in dark mode. Fusion uses the application
palette for both themes; this does not select a fallback file chooser.
`ThemeButton.qml` explicitly paints text and backgrounds using the Basic control
implementation for About actions and the highlighted Open capture action. This
avoids Windows' pale button backgrounds even when palette inheritance differs.
Tests check rendered contrast for ordinary, highlighted and disabled buttons in
Dark, Light and both System appearances. About keeps its version, project link,
update status and actions; descriptive/license text and update instructions have
been removed from this dialog. Distribution license/NOTICE files remain intact.
Installed smoke tests clear development control-style overrides so the executable
must select its own style, as it does when opened from a desktop shortcut.
See [Qt control styles](https://doc.qt.io/qt-6/qtquickcontrols-styles.html).

Linux application startup defaults `QT_QPA_PLATFORMTHEME` to `gtk3` before
creating Qt, so Open and Export use the operating system's GTK file chooser
in both development and installed builds. An explicit platform-theme setting
is preserved. The DEB bundles Qt's GTK platform-theme plugin and depends on
the system GTK runtime. Windows and macOS retain their native defaults.
For the existing 0.1.6 installation, launch with
`QT_QPA_PLATFORMTHEME=gtk3 pnm-viewer` from a graphical desktop terminal.

[Developer index](README.md) · [Measurement implementation](qt-measurements.md)

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

## Windows linker selection

Run Qt builds from an initialized **x64 MSVC developer terminal**. GitHub jobs
initialize this environment with `ilammy/msvc-dev-cmd`. Git Bash can nevertheless
put its unrelated `Git/usr/bin/link.exe` ahead of Microsoft's linker.

Install the Windows SDK with Visual Studio's C++ workload: the GUI build also
uses `rc.exe` to embed the branded shell icon. Qt QRC window icons alone do not
provide Explorer/desktop executable icons. macOS release packaging uses the
committed ICNS; neither native build requires Pillow at runtime.

The shared `qt_environment()` helper resolves MSVC from `VCToolsInstallDir`,
validates the x64 compiler/linker/librarian, and prepends their directory to the
child PATH. It sets Cargo's default
`CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER` to the absolute MSVC linker path;
an explicit caller override is preserved. Windows environment-key casing is
handled. Qt checks and native package builds use this same helper; global PATH
and developer tool installations are unchanged.

Before any Cargo steps, Windows CI also runs
`python scripts/qt-sdk.py export-msvc-linker`. It reuses the same tool validation
to persist the Cargo linker in the job environment, covering workspace/CLI tests
outside the Qt helper. This CI action requires `GITHUB_ENV` and does not install
or access Qt.

If the toolchain is uninitialized or incomplete, setup/check fails with a specific
MSVC message before compiling. An error mentioning `/usr/bin/link: extra operand`
indicates the Git utility was selected; reinstalling Visual Studio is not the
remedy for that PATH collision. See [Cargo linker configuration](https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinker).

Regression tests use synthetic tool directories without downloading Qt or running
MSVC, and run on every desktop CI runner before SDK setup:

```bash
python3 -m unittest discover -s scripts/tests -p 'test_qt_sdk.py'
```

## Validation

The check runs Rust formatting, adapter tests, Clippy, a native build and
Qt Quick tests for themes, missing readings, sample coordinates and mouse zoom
inspection. Shared capture tests cover fixtures, JSON and safe export. The
pre-push gate checks the Rust workspace and Qt desktop. Native file pickers and platform theme behavior
also need desktop smoke tests; offscreen tests alone cannot verify OS integration.

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
outline. White areas inside the light icon remain part of the artwork. Qt uses the matching PNG window icons in both themes.

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
consult the workflow runs for current native results.

See [CI](ci.md) for all required runners and
[packaging](../release/packaging.md) for DEB, NSIS and DMG artifacts. The explicit
`--smoke-test INPUT_FILE_URL NEW_OUTPUT_FILE_URL` mode opens, renders and exports
one capture, logs `UI_SMOKE_OK` and exits. It uses the same Rust adapter and does
not overwrite existing outputs. Normal launches do not activate this mode.

## Measurement behavior

Use the [user guide](../ui/README.md) for controls and screenshots.
Rust/QML ownership, decimation and measurement-specific implementation notes
live in [Qt measurement internals](qt-measurements.md).

## QML reports and isolated checks

`python scripts/qt-sdk.py check` runs QML presentation tests before the expensive
native Rust/C++ checks. A failure stops the check; successful runs still execute
all native tests, formatting, Clippy and builds. To isolate the complete QML
suite with an already installed SDK and configured developer environment:

```bash
python scripts/qt-sdk.py qml-test
```

Each invocation creates a fresh directory under `local/qt-test-results/` with
`qml-tests.txt`, `qml-tests.xml` (JUnit) and `process.log`. Available text and
process diagnostics are printed even when the runner fails. The original exit
status still fails the check. CI retains per-platform reports for 14 days,
including failed runs. No test is skipped or retried to obtain a passing result.

Explicit files avoid relying on Windows GUI console output. See the
[Qt Test output options](https://doc.qt.io/qt-6/qtest-overview.html) and
[Windows debugging guidance](https://doc.qt.io/qt-6/debug.html#environment-variables-recognized-by-qt).
If startup fails before Qt Test can write its reports, inspect `process.log`;
a report may be absent when the process never initialized.

## Release-update indicator and About

`apps/viewer/src/updates/release.rs` owns read-only GitHub stable-release discovery and
version comparison. `apps/qt/src/update.rs` provides an independent CXX-Qt
controller: one request at a time, on a Rust worker, with queued UI-thread
completion. Capture busy/status state is unaffected. `UpdateChip.qml` and
`AboutDialog.qml` share this controller. Normal launches start one check; About
can retry. `--smoke-test` disables the startup request for offline lifecycle checks.

Metadata is capped at 1 MiB with a ten-second request timeout and redirects
disabled. 404 means no published release. Other HTTP, content-type, malformed
JSON, draft/prerelease or invalid-version cases produce update-only errors.
Release links are generated from the fixed project URL and validated numeric
tags, never arbitrary remote HTML/URLs. A newer stable release reveals the
green chip; equal/newer installed versions and errors keep it hidden.

The HTTPS dependency is pinned; both lockfiles retain Rust-1.85.1-compatible
IDNA dependencies. Preserve that compatibility when refreshing dependencies.
Core parsers and pnm-decode have no update networking dependency.

Tests use local HTTP fixtures and mocked QML update state. They do not require
a successful tag, published release, credentials or a live GitHub connection.
Ubuntu x64 installation lives in `apps/viewer/src/updates/ubuntu/`. Rust validates
both release assets, downloads with TLS/redirect/size/time/hash limits and
cancellation, and requests Polkit authentication only after verification.
The downloader and elevated installer create their respective temporary
directories with explicit Unix mode 0700 before writing content.
The fixed installed executable handles `--install-update` before constructing Qt:
it copies to root-private staging and rechecks hashes and DEB identity/version/arch.
The elevated installer still checks user ownership and
rejects group/other access. Rust regression tests exercise the shared directory
creator, including ownership and cleanup; the installed lifecycle test uses a
Python-created private fixture directory and does not exercise live downloads.
It requires both candidates to be newer, and runs apt on the pair. It performs no
network I/O as root. The controller reports progress/failure and exposes an
automatic restart after success. About queues the restart on installed state,
including when hidden, and defers it while the capture backend is busy. Rust
marks the successful spawn as restarting before QML requests application quit;
a launch failure keeps installed state and the old window with a retry button.
No Python runtime or shell command strings are used. Source builds, Windows and macOS keep release-download links; no Windows
installer automation is claimed. Interactive Polkit/restart requires VM testing.

## Viewer source organization

`apps/viewer/src/lib.rs` wires three responsibility groups:

```text
apps/viewer/src/
├── lib.rs
├── capture/                 # File ownership, diagnostics and shared charts
│   ├── mod.rs
│   ├── file.rs
│   ├── diagnostic.rs
│   └── chart.rs
├── measurements/
│   ├── mod.rs
│   ├── channel_estimation/mod.rs
│   ├── constellation/mod.rs
│   ├── fec_summary/mod.rs
│   ├── histogram/mod.rs
│   ├── modulation_profile/mod.rs
│   ├── spectrum_analysis/mod.rs
│   └── upstream_pre_equalization/mod.rs
└── updates/
    ├── mod.rs
    └── release.rs            # HTTP release lookup and comparison
```

Qt imports capture types from `viewer::capture`, chart DTOs from
`viewer::capture::chart`, measurement adapters from `viewer::measurements`,
and release checks from `viewer::updates::release`. Root capture-type exports
remain convenience imports for existing callers. New implementations reuse their
owning folder or introduce a relevant feature folder; apply the
[organization skill](../../skills/viewer-module-organization/SKILL.md).

## Ubuntu launcher identity

Before loading QML, the Linux viewer sets its Qt application name and desktop
file name to `pnm-viewer-toolkit`. The DEB installs the matching desktop entry
and `StartupWMClass=pnm-viewer-toolkit`. Keep these identifiers aligned so GNOME
can group running windows and pin the installed launcher. The display title and
CLI command are separate from this identity. Installed lifecycle checks verify
the launcher fields. Interactive GNOME pin/unpin behavior needs VM confirmation.

### Waiting for UI readiness in tests

Wait for layout polish before clicking controls, then assert the resulting
signal/state change. A static control may already have rendered; waiting for
another frame without scheduling one can time out on macOS. For Canvas tests
that require a new rendered frame, call `requestPaint()` before
`waitForRendering()`. Keep real mouse interaction and outcome assertions;
do not replace them with direct signal invocation or skip platform tests.

## Deterministic QML rendering tests

Use `RenderTestCase` from `apps/qt/tests/helpers/` when a test requires pixels or
a new render. Its `waitForFrame(item)` flushes window layout, schedules Canvas
painting when applicable and explicitly requests a window frame. Keep the
following pixel/interaction assertions; do not replace them with a sleep or a
passing retry. For input-only tests, wait for delegate creation, visibility and
polished geometry, then assert the actual signal and its exact arguments.
Reset transient test state between cases, and preserve dark/light coverage.

Master/PR now exercises native Cocoa QML on both Mac architectures before a tag
when the branch is pushed separately. Windows packaging-tool tests run in the
same lightweight preflight, without compiling Rust. See the
[CI failure audit and release sequence](ci.md#recurring-failure-audit).

## Shared chart style

`Theme.qml` owns session-scoped chart width and color overrides;
`ChartSettingsDialog.qml` exposes them under Settings → Charts. The default
width is 0.75 px. Empty color overrides preserve the dark/light palette. The
shared `chartStyleChanged` signal repaints Canvas-based line charts immediately,
while Min/Avg/Max legend outlines bind to the same effective colors. Grid and
carrier-marker geometry remain separate from trace width. No measurement math
or JSON contract changes. See the [user guide](../ui/settings.md).
