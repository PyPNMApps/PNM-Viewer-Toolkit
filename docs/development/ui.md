# RxMER desktop viewer

The [Qt Quick prototype](qt.md) supports RxMER, Spectrum Analyzer and DS Channel
Estimation without a WebView. Both shells share Rust capture behavior in
apps/viewer. This page describes the retained Tauri RxMER prototype.

The first desktop slice lives under `apps/desktop/`: Tauri 2, React/TypeScript
and the existing Rust parser. It remains in the dev phase. Installable desktop
packages and distribution testing are still future work.

## Run locally

After `./install.sh --development` has prepared the local toolchains:

```bash
scripts/node-local.sh npm ci --prefix apps/desktop
scripts/desktop.sh
```

With standard Node and Rust installations on Windows, macOS or Linux:

```bash
cd apps/desktop
npm ci
npm run tauri -- dev
```

Install the [native Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)
for your platform first. The development Vite server serves frontend assets only;
production embeds those assets and uses native IPC, with no HTTP backend.

Choose **Open PNM file**, then select an RxMER format 1.0 capture. The repository's
`tests/fixtures/pypnm/tests/files/rxmer.bin` is a preserved example. Detection uses
file contents, not its extension. Other measurements remain supported by the CLI
but do not yet have charts in the Tauri prototype.

- Inspect channel, MAC address, sample count and subcarrier spacing.
- Move through exact windows of up to 1,024 samples using Previous and Next.
- Hover over the chart or use the keyboard-accessible sample slider for readings.
  Missing values remain null and break the line. No regression or smoothing runs.
- Export full-resolution core JSON to a **new** file. Existing files, including
  the input capture, are never overwritten. The save dialog's cancel leaves
  everything unchanged.
- Close releases the capture. Opening another valid file replaces it; canceling
  or failing an import keeps the previous capture.
- Choose **System**, **Dark**, or **Light** under Appearance. System is the default
  and follows live OS preference changes. An explicit selection overrides the OS
  until changed. This initial preference lasts for the current app session.

## Ownership and scope

```text
apps/desktop/
  src/                     React shell, theme selector, chart and typed IPC adapter
  src-tauri/
    src/main.rs            Native dialogs, session lifecycle and Tauri commands
    Cargo.toml             Separate desktop workspace and native dependencies
    Cargo.lock             Locked desktop dependency graph
apps/viewer/src/lib.rs     Shared bytes, bounded reads and viewport/export DTOs
apps/qt/                  Qt Quick shell and CXX-Qt adapter
crates/core/               Reusable parsing; no UI dependency
crates/cli/                Independently buildable pnm-decode
```

The desktop workspace is deliberately separate: CLI-only Cargo commands do not
compile GTK, WebKit or the desktop runtime. Both use the same core library.
Rust owns bytes and JSON export; React receives small metadata and exact sample
windows rather than full captures. See [IPC contracts](api-contract.md).

Run all desktop checks with `scripts/check-desktop.sh`. This runs frontend
TypeScript, lint, tests and production build, then Rust formatting, tests, Clippy
and the native executable build. The main/master push gate includes these checks.
The desktop workflow also runs on native Linux, Windows and macOS runners;
workflow configuration does not establish successful execution on those systems.

Later work: full-band peak-preserving reduction and zoom, persistent preferences,
raw-byte inspection for unsupported files, additional charts, drag/drop, recent
files and native packaging that includes the matching CLI. Nothing in this first
slice starts captures, contacts modems or requires a server.

## Dependency policy

The desktop lockfile retains tauri-runtime 2.9.2 and wry 0.53.4 for the pinned
Rust 1.85.1/Tauri 2.9.5 combination. Updating those independently selected
incompatible APIs in validation; update and test the native stack together.
Frontend tooling overrides minimatch to 11.0.1 to avoid the advisory affecting
older transitive versions; the pinned Node 24 supports it. Lint and frontend
tests exercise this override. These development dependencies are not bundled
in the desktop application.

## Validation on this PC

Ubuntu 22.04 x86_64: frontend typecheck/lint/four tests/production build, desktop
Rust formatting/tests/Clippy/native build, and the full pre-push gate passed.
An isolated X display exercised the actual bundled native executable with the
preserved RxMER fixture: native open dialog, channel/MAC metadata, Dark/Light
selection, sample paging, native JSON save dialog and session close. The exported
JSON contained 7,480 readings, first value 42.75 dB, and was 78,925 bytes.
System-theme change behavior is covered by frontend tests; the host's OS settings
were not changed. Windows/macOS runtime and installed-package checks remain unrun.

The frontend includes dark/light ICO favicons from `docs/images/`, selected by
browser/system color preference. Each contains 16, 32, 48, 64, 128 and 256 pixel
variants with transparent outer corners. Vite bundles these assets for offline use.
