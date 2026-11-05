# PNM Viewer Toolkit {{TAG}}

**Under development.** Download the package for your system under **Assets**.
The native Qt viewer includes the same-version `pnm-decode` CLI and manual;
Ubuntu installs the CLI as a separate package dependency. Rust, Python and a
separate Qt SDK are not needed to run the packaged application. APT installs
required Ubuntu system libraries and may need network access.

## Choose your download

{{ASSET_TABLE}}

Sizes above are compressed download bytes expressed in decimal MB, not installed
sizes. `SHA256SUMS`, `release-index.json` and platform inventories accompany the
packages. GitHub's **Source code** ZIP/tar archives contain source, not installers.
CLI archives install no desktop UI.

## Ubuntu 22.04 / 24.04 / 26.04 — x64

Download both same-version DEBs from the Ubuntu row. In the download directory:

```bash
sudo apt install ./pnm-decode_{{VERSION}}_amd64.deb ./pnm-viewer-toolkit_{{VERSION}}_amd64.deb
```

Launch **PNM Viewer Toolkit** from the application menu, or run `pnm-viewer`
inside your graphical desktop session. The CLI is available as `pnm-decode`;
read its manual with `man pnm-decode`.

**CLI only:** install just `pnm-decode_{{VERSION}}_amd64.deb` using APT.
The Linux CLI archive is also available for manual extraction.

**Update:** quit the viewer, then install both new-version DEBs with APT.

**Uninstall the viewer:**

```bash
sudo apt remove pnm-viewer-toolkit
```

This leaves the separately owned CLI installed. To remove it too, run
`sudo apt remove pnm-decode`.

[Ubuntu installation and uninstall guide]({{SOURCE_URL}}/docs/install/ubuntu.md)

## Windows — x64

1. Download and run the **Windows x64 EXE installer** from the table.
2. Select an installation folder and launch **PNM Viewer Toolkit** from Start.

The default per-user location is `%LOCALAPPDATA%\Programs\PNM Viewer Toolkit`.
Qt and Microsoft runtime DLLs are app-local. The bundled CLI is in that folder;
the installer does not add it to PATH. From PowerShell at the default location:

```powershell
& "$env:LOCALAPPDATA\Programs\PNM Viewer Toolkit\pnm-decode.exe" --help
```

**CLI only:** extract the Windows CLI ZIP and run `pnm-decode.exe` from its
versioned folder. Add that folder to your user PATH only if desired.

The installer is unsigned. If Windows displays a reputation or publisher prompt,
verify the project download and checksum before approving it; do not disable
Windows protections globally.

**Update:** quit the viewer and run the new EXE using the same installation folder.

**Uninstall:** Settings → Apps → Installed apps → PNM Viewer Toolkit → Uninstall,
or run `uninstall.exe` in its installation directory. A separately extracted CLI
is independent of the desktop installation.

[Windows installation and uninstall guide]({{SOURCE_URL}}/docs/install/windows.md)

## macOS — Apple Silicon or Intel

1. Choose **arm64** for an Apple M-series Mac or **x64** for an Intel Mac.
2. Open the matching DMG and drag **PNM Viewer Toolkit** onto **Applications**.
3. Eject the disk image and open the app from Applications.

The deployment target is macOS 13 or newer; native CI uses macOS 15 runners.
The app includes Qt and the CLI. It is ad-hoc signed, without Developer ID
signing or Apple notarization. If macOS blocks opening it, verify the download
source and checksum, then use the operating system's application-approval controls.

The bundled CLI can be run from a terminal:

```bash
"/Applications/PNM Viewer Toolkit.app/Contents/MacOS/pnm-decode" --help
```

**CLI only:** extract the matching Darwin CLI `.tar.gz`; run `pnm-decode`
from its versioned folder. The archive includes the manual and license notices.

**Update:** quit the viewer and replace the app in Applications from the new DMG.

**Uninstall:** quit the app and move it from Applications to Trash. Any separately
extracted CLI remains independent.

[macOS installation and uninstall guide]({{SOURCE_URL}}/docs/install/macos.md)

## First use

Choose **Open capture**, **Open**, Ctrl+O (Cmd+O on macOS), or drop one local PNM file onto the
window. The viewer detects its type from its contents. It supports RxMER,
Spectrum Analyzer, DS Channel Estimation, Histogram, FEC Summary, Modulation
Profile, Constellation Display, current US pre-EQ and last-update US pre-EQ.
Inspect the graph and use the export icon to save full-resolution JSON to a new
file. Choose System, Dark or Light under Settings → System → Appearance.
Viewing local captures works offline after setup; no modem connection is needed.

[Desktop guide and screenshots]({{SOURCE_URL}}/docs/ui/README.md) ·
[Measurement gallery]({{SOURCE_URL}}/docs/ui/measurements.md) ·
[CLI guide]({{SOURCE_URL}}/docs/cli/usage.md) ·
[Complete JSON examples]({{SOURCE_URL}}/docs/cli/examples/README.md)

## Checksums and validation

Download `SHA256SUMS` with your chosen package and compare its matching entry
against your local SHA-256 result. Examples from the download directory:

```bash
# Ubuntu
sha256sum pnm-viewer-toolkit_{{VERSION}}_amd64.deb
# macOS Apple Silicon; substitute macos-x64 for an Intel package
shasum -a 256 PNM-Viewer-Toolkit-{{VERSION}}-macos-arm64.dmg
```

```powershell
Get-FileHash .\PNM-Viewer-Toolkit-{{VERSION}}-windows-x64.exe -Algorithm SHA256
```

These packages are assembled only after CodeQL, code checks, the complete native
matrix, package integrity/size checks and installed-package lifecycle tests pass
for this tag. Lifecycle tests cover install, open/render/export and removal on
all three Ubuntu versions, Windows x64, macOS arm64 and macOS x64. See
[this tag's workflow runs]({{RUNS_URL}}) for results. These development builds
are not a release-readiness claim. Publication is automatic only after uploaded
asset metadata and downloaded bytes are verified.

[Packaging, licenses and prerequisites]({{SOURCE_URL}}/docs/release/packaging.md) ·
[JSON field contracts]({{SOURCE_URL}}/docs/reference/formats.md)
