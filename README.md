# PNM Viewer Toolkit — Under Development

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/pnm-viewer-toolkit-logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/images/pnm-viewer-toolkit-logo-light.svg">
    <img alt="PyPNMApps PNM Viewer Toolkit" src="docs/images/pnm-viewer-toolkit-logo-light.svg" width="461">
  </picture>
</p>

[![CI](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/ci.yml)
[![Native packages](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/release.yml/badge.svg)](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/release.yml)
[![Ubuntu matrix](https://img.shields.io/badge/Ubuntu-22.04%20%7C%2024.04%20%7C%2026.04-E95420?logo=ubuntu)](docs/development/ci.md)
[![Windows matrix](https://img.shields.io/badge/Windows-x64-0078D4)](docs/development/ci.md)
[![macOS matrix](https://img.shields.io/badge/macOS-Apple%20Silicon%20%7C%20Intel-555555?logo=apple)](docs/development/ci.md)
[![Component licenses](https://img.shields.io/badge/license-Apache--2.0%20%2F%20GPL--3.0--only-blue)](LICENSE)

An offline desktop viewer for DOCSIS PNM capture files. Open a capture, explore
its graph, and export decoded JSON. The native Qt interface uses a Rust backend.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="docs/images/user-guide/home-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="docs/images/user-guide/home-light.png">
  <img alt="PNM Viewer Toolkit starting screen with the left navigation pane open" src="docs/images/user-guide/home-dark.png" width="1000">
</picture>

## Start here

[Launch the viewer](docs/ui/README.md#launch) ·
[User guide](docs/ui/README.md) ·
[Screenshots and measurements](docs/ui/measurements.md)

The current viewer supports:

| Capture | Views |
| --- | --- |
| RxMER | MER by frequency |
| Spectrum Analyzer | Amplitude by frequency and calculated RBW |
| DS Channel Estimation | Magnitude, group delay and I/Q density |
| Histogram | Hit counts by bin index |
| FEC Summary | Per-profile interval codewords over UTC time |
| Constellation Display | Soft/hard decisions and symbol boundaries |
| Modulation Profile | Ideal Shannon minimum MER and bit loading by frequency |

Choose System, Dark or Light appearance. Drag across a graph to zoom and
double-click to reset; the I/Q density view shows the full capture.

Tag builds prepare [native packages](docs/release/packaging.md) for a draft release
after all required checks pass. Public downloads are not available yet. Local
builds have been exercised on Ubuntu 22.04 x86_64; hosted results for the full
platform matrix remain pending.

## CLI and development

- [Build the standalone `pnm-decode` CLI](docs/install/source.md) · [CLI user guide](docs/cli/usage.md)
- [JSON examples for each PNM file type](docs/cli/usage.md#json-examples-by-file-type)
- [Supported formats and limits](docs/reference/formats.md)
- [Development setup](docs/install/development.md) · [Qt build and checks](docs/development/qt.md)
- [Rust APIs and examples](docs/development/rust.md)
- [Developer guide](docs/development/README.md) · [Version bumps and releases](docs/development/releasing.md)
- [All documentation](docs/README.md) · [Repository conventions](AGENTS.md)

## License

Core libraries and development tooling use Apache-2.0; desktop components use
GPL-3.0-only. See [LICENSE](LICENSE) and [NOTICE](NOTICE).
Built with guidance and example captures from **PyPNM**; third-party material
retains its original notices.
