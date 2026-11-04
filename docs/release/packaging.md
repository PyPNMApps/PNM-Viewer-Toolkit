# Native packaging

[CI and release ordering](../development/ci.md) · [Versioning](versioning.md)

The project remains **under development**. Tag automation builds development
packages and creates a draft release only after native checks pass. Public
publication, external signing and Apple notarization are separate actions.

## Outputs

| Native builder | Viewer | Standalone CLI |
| --- | --- | --- |
| Ubuntu 22.04 x64 | `pnm-viewer-toolkit_VERSION_amd64.deb` plus CLI DEB dependency | DEB and `.tar.gz` |
| Windows x64 | `PNM-Viewer-Toolkit-VERSION-windows-x64.exe` (NSIS) | `.zip` |
| macOS Apple Silicon | `PNM-Viewer-Toolkit-VERSION-macos-arm64.dmg` | `.tar.gz` |
| macOS Intel | `PNM-Viewer-Toolkit-VERSION-macos-x64.dmg` | `.tar.gz` |

The primary desktop is Qt Quick; Tauri remains covered by CI. UI packages contain
or depend on the same-version `pnm-decode` and manual. The UI calls shared Rust
libraries directly; it does not launch the CLI to decode files.

Linux uses a private Qt runtime under `/opt/pnm-viewer-toolkit`, a desktop entry,
icon, and `pnm-viewer` launcher. Its separate `pnm-decode` dependency owns
`/usr/bin/pnm-decode` and the manual. Removing the UI leaves the CLI installed.
The same Ubuntu 22.04 artifacts must pass installed tests on 22.04, 24.04 and
26.04; never rebuild a different DEB to make a newer OS check pass.

Windows installs per user under Local AppData with a Start Menu shortcut and
uninstaller. Qt and MSVC runtime DLLs are app-local. The bundled CLI/manual belong
to that private application directory; independently installed CLI files and
PATH entries are untouched. Windows packages are unsigned development builds.

The DMG contains the app and an Applications shortcut. Copy the app to an
Applications directory. Its bundled decoder is at
`PNM Viewer Toolkit.app/Contents/MacOS/pnm-decode`; the manual and notices live
under `Contents/Resources/share`. macOS builds use ad-hoc integrity signing,
not an Apple Developer certificate or notarization. Gatekeeper may require
explicit user approval. The deployment baseline is macOS 13 or newer.

## Local builds

Install the pinned Rust toolchain and Qt SDK first; use
`python3 scripts/qt-sdk.py setup` on Unix or `python scripts/qt-sdk.py setup` on
Windows. Windows builds need an x64 MSVC developer shell and NSIS 3.11; macOS
needs Xcode command-line tools. Ubuntu packaging needs `dpkg-deb`, `patchelf`
and the runtime libraries listed in `scripts/release/linux.py`.

From the checkout, with Cargo available:

```bash
python3 scripts/release/build.py --output dist/release
python3 scripts/release/verify.py --packages dist/release
```

Use `python` on Windows. Output and its sibling staging directory must be absent;
move previous output aside before retrying. The builder resolves project paths
from its location and never deletes existing output. It builds only for the
current native host, with locked dependencies. It does not install, push or
publish. The local Cargo wrapper uses ignored `local/` homes; expose those homes
when invoking Python packaging from a checkout using that toolchain.

## Gates and metadata

Every primary installer/archive must be nonempty and no larger than
**50,000,000 bytes**, including bundled prerequisites/assets. Oversized output
fails the job. `SHA256SUMS` and release manifests record measured bytes; payload
inventories report installed file bytes separately. Cargo dependency graphs
record exact locked inputs. A CycloneDX dependency inventory and collected
Cargo/Qt license notices accompany packages; the inventory covers the locked
source graphs, including platform-conditional dependencies. Qt notices come from
pinned source-tag archives cached under `local/release-sources/`, with measured
source hashes recorded for review. Packaging needs network access for the first
source/notice collection. Packages include project licenses, NOTICE and CLI
usage/manual. They exclude captures, private PDFs, development tools and fixtures.

Linux still requires its declared distribution libraries; installing those can
require network access. Offline viewing after setup is supported. This does not
promise pristine-host offline installation or claim external prerequisites fit
inside the artifact budget. Report those prerequisites separately.

Native lifecycle tests validate artifact hashes before installation. They open,
render and export RxMER, Spectrum Analyzer, DS Channel Estimation and Histogram
through the actual Qt backend, comparing raw fields against the CLI. Development
Qt environment paths are removed. Linux uses Xvfb/XCB. Native test logs are
retained even on failure. Offscreen QML tests alone do not establish installed
runtime support.

`test-installed.py` performs system-changing checks only when both
`--ci-disposable-host` and `GITHUB_ACTIONS=true` are present. Do not run it on a
personal workstation. For local relocated binaries, `smoke.py --gui PATH --cli
PATH` runs without installing; it requires a functioning display.

The first hosted tag run must establish Windows/macOS and newer Ubuntu results.
No configured matrix should be described as a passing platform before that run.
