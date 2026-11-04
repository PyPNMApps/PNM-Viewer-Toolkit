# Qt Quick desktop prototype

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
it installs no system packages and currently supports Linux x86_64 only.
`QT_SDK_DIR` can select an existing Qt 6.8.3 SDK. CXX-Qt is pinned to 0.7.3,
with matching CXX runtime/generator versions and committed Cargo lockfiles.

```bash
scripts/qt-desktop.sh build
scripts/qt-desktop.sh check
```

## Workflow

The initial Home page is measurement-neutral. Choose Open capture or Open PNM
file; RxMER format 1.0 is currently the only supported UI measurement. A valid
capture opens the Downstream RxMER view, and Close returns to Home. Other file
types retain the existing unsupported-viewer diagnostic.

Open a local RxMER capture, for example
`tests/fixtures/pypnm/tests/files/rxmer.bin`. Detection uses file contents.
Inspect channel, MAC, sample count and spacing. The chart pages through up to
1,024 exact samples. Hover or use the keyboard-accessible sample slider;
missing readings appear as gaps and are labeled unavailable.

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
Windows/macOS installers, signing, CLI bundling and the 50 MB distribution gate
remain pending. The development binary requires Qt runtime dependencies.

## Validation

The check runs Rust formatting, adapter tests, Clippy, a native build and
Qt Quick tests for themes, missing readings, page coordinates and keyboard
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
