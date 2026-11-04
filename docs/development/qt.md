# Qt Quick desktop prototype

For opening captures, chart controls and screenshots, start with the
[desktop user guide](../ui/README.md). This page covers developer setup and validation.

The Qt desktop lives under `apps/qt/`. It uses the GPL-3.0-only Rust capture
layer in `apps/viewer`, backed by the
existing Apache-2.0 parsers. Qt Quick renders directly through Qt without a
browser, WebView or HTTP backend. This is development software.

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
