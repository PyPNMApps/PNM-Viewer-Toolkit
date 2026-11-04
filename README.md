# PNM Viewer Toolkit

<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="docs/images/pnm-viewer-toolkit-logo-dark.svg">
    <source media="(prefers-color-scheme: light)" srcset="docs/images/pnm-viewer-toolkit-logo-light.svg">
    <img alt="PyPNMApps PNM Viewer Toolkit" src="docs/images/pnm-viewer-toolkit-logo-light.svg" width="461">
  </picture>
</p>

[![CLI build](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/cli-build.yml/badge.svg)](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/cli-build.yml)
[![Tests](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/tests.yml/badge.svg)](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/tests.yml)
[![Code checks](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/ci.yml/badge.svg)](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/workflows/ci.yml)
[![Desktop checks: pending](https://img.shields.io/badge/Desktop_checks-pending-lightgrey)](.github/workflows/desktop.yml)
[![Rust 1.85.1](https://img.shields.io/badge/Rust-1.85.1-orange?logo=rust)](rust-toolchain.toml)
[![Status: in development](https://img.shields.io/badge/status-in_development-yellow)](docs/development/ui.md)
[![Component licenses](https://img.shields.io/badge/license-Apache--2.0%20%2F%20GPL--3.0--only-blue)](LICENSE)

An offline desktop viewer for DOCSIS Proactive Network Maintenance (PNM) capture
files, with a Rust backend and parallel Qt Quick and Tauri/React prototypes.
The repository is `PNM-Viewer-Toolkit`; the standalone CLI is `pnm-decode`.
The parser design follows PyPNM, with shared file headers, typed measurement
objects, and human-readable JSON exports.

**In the dev phase:** the Rust parsing libraries and development tools are
implemented, including the standalone CLI and Linux source installer. The
first desktop slice opens RxMER captures, graphs exact samples and exports JSON.
Desktop installers remain planned. See [run the viewer](docs/development/ui.md).

## What works today

- Open RxMER files in the desktop viewer, inspect sample windows and export JSON.
- Choose System, Dark or Light appearance, including live system-theme changes.

- Decode RxMER, channel estimates, constellation samples, histograms, upstream
  pre-equalization coefficients and last updates, FEC summaries, spectrum
  analysis, and modulation profiles from file-format version 1.0 captures.
- Export decoded measurements as bounded, formatted JSON. Complex samples use
  `iq`, ordered `[I, Q]` (in-phase, quadrature).
- Preserve original capture bytes and report structured errors for unsupported
  formats, truncation, inconsistent lengths, and resource limits.
- Test against nine preserved PyPNM binary fixtures, with additional malformed
  and boundary cases.

Latency decoding currently has **synthetic tests only**. Symbol capture is a
**header-only stub**. See [format support](docs/reference/formats.md) for the exact
coverage and JSON differences from PyPNM. This is not a claim of blanket
DOCSIS 3.1 or 4.0 compliance.

## Get started

The standalone decoder builds with standard Cargo from the repository root:

```bash
cargo build --release --locked -p pnm-decode
```

Run `target/release/pnm-decode` on Linux/macOS or
`target\release\pnm-decode.exe` on Windows. For example:

```bash
./target/release/pnm-decode capture.bin --output capture.json
```

Only Ubuntu 22.04 x86_64 has been built and tested so far. Prebuilt Windows,
macOS and Linux downloads are planned; they are not published yet.
See [installation options](docs/install/README.md),
[source builds](docs/install/source.md) and [CLI usage](docs/cli/usage.md).

Contributors can follow [development setup](docs/install/development.md),
[Rust APIs and tests](docs/development/rust.md), and
[parser examples](crates/core/examples/README.md).
See the [desktop viewer guide and remaining work](docs/development/ui.md).

## Qt desktop prototype

The Qt Quick/Rust prototype lives alongside the retained Tauri app. It opens
RxMER captures, plots samples, exports JSON, and offers System/Dark/Light themes.
See [Qt setup and launch instructions](docs/development/qt.md). Both prototypes
remain in development; desktop installers are not yet available.

## Design direction

Rust owns binary decoding, numerical processing, exports, and future analysis
and storage. Qt Quick renders directly through Qt; the retained React interface
uses Tauri's system WebView. Both share the same Rust capture layer and work
without a local HTTP backend. Optional retrieval of existing PyPNM
captures is planned; modem control, capture initiation, SNMP/TFTP, Python
sidecars, and remote analysis are outside the viewer's scope.

## Repository layout

| Path | Purpose |
| --- | --- |
| `apps/qt/` | Qt Quick/QML desktop and Rust adapter |
| `apps/desktop/` | Retained Tauri/React desktop |
| `apps/viewer/` | Shared Rust desktop capture and JSON export layer |
| `crates/cli/` | Standalone `pnm-decode` command and man page |
| `crates/core/` | Rust `pnm` package: parsers, shared types, Q-format math and utilities |
| `crates/logging/` | Application logging setup |
| `tests/fixtures/` | PyPNM binary examples, licenses and provenance |
| `docs/` | Setup, format support, API and packaging documentation |
| `skills/` | Canonical development skills and reference material |
| `scripts/` | Development setup and local toolchain wrappers |
| `tools/` | Git and three-part version/release helpers |
| `development/` | Ignored local plans and specification evidence |
| `local/` | Ignored toolchains and generated JSON |
| `target/` | Ignored Cargo build output |

Working plans and `development/specs/` evidence records are local-only and are
not included in GitHub clones. The tracked
[development reference skill](skills/pnm-viewer-development/SKILL.md) preserves
the original bootstrap guidance and PyPNM source map.

## Documentation

Start with the [documentation index](docs/README.md) for installation, CLI usage,
format support, contributor guides, the UI roadmap, release procedures and history.
See [AGENTS.md](AGENTS.md) for repository conventions when using a coding agent.

## License and acknowledgment

See [LICENSE](LICENSE) for the component-specific licensing:

- Reusable Rust libraries and project-owned development tooling:
  [Apache-2.0](LICENSES/Apache-2.0.txt).
- Desktop shells, UIs and shared desktop capture layer:
  [GPL-3.0-only](LICENSES/GPL-3.0-only.txt).

Third-party material retains its original terms and notices.
Acknowledgment: **PyPNM - Proactive Network Maintenance Toolkit**.
See [NOTICE](NOTICE) and the [fixture references](tests/fixtures/pypnm/README.md).
