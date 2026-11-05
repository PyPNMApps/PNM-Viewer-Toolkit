# Installation

PNM Viewer Toolkit is **under development**. Open the
[GitHub Releases page](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases)
and choose a published release with installer assets. Each release page has
version-specific downloads, installation/removal commands and checksums.
Tag CI publishes automatically after testing and uploaded-byte verification. If no
published installer is available, use the [source build](source.md) for the CLI
or the [Qt development setup](../development/qt.md) for the desktop.

## Choose a component

| Platform | Desktop viewer | CLI only |
| --- | --- | --- |
| Ubuntu 22.04 / 24.04 / 26.04 x64 | Viewer DEB plus same-version CLI DEB; install both with APT | CLI DEB or Linux `.tar.gz` |
| Windows x64 | Per-user EXE installer with private Qt, Microsoft runtime DLLs and bundled CLI | ZIP containing `pnm-decode.exe` |
| macOS Apple Silicon | arm64 DMG; drag the app into Applications | Darwin arm64 `.tar.gz` |
| macOS Intel | x64 DMG; drag the app into Applications | Darwin x64 `.tar.gz` |

Desktop packages provide the same-version decoder and manual. The viewer uses
shared Rust code directly. Rust, Python and a separate Qt SDK are not needed to
run packaged binaries. Ubuntu uses declared system display libraries; APT may
need network access. Source builds have separate development prerequisites.
GitHub's automatically generated **Source code** archives are not installers.

Windows installers are unsigned and do not modify PATH. macOS applications use
ad-hoc signing and are not notarized; their deployment target is macOS 13 or
newer, while CI runs on macOS 15. Use the release page's download/checksum and
OS approval instructions. Keep captures outside installation directories.

## Update and remove

Quit the viewer before updating. On Ubuntu, install both new-version DEBs using
APT. On Windows, run the new EXE with the same installation directory. On macOS,
replace the application using the matching architecture's new DMG.

Remove the Ubuntu viewer with `sudo apt remove pnm-viewer-toolkit`; the separate
CLI remains installed until you remove `pnm-decode`. On Windows, uninstall through
Installed apps or the installation directory's `uninstall.exe`. On macOS, move
the application to Trash. Independently extracted CLI archives are separate from
the Windows/macOS desktop installations and can be removed independently.

See the [desktop guide](../ui/README.md), [CLI guide](../cli/usage.md),
[measurement gallery](../ui/measurements.md) and
[packaging/validation details](../release/packaging.md). Build contributors should
start with the [development environment](development.md).
