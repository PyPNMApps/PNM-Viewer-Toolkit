# PNM Viewer Toolkit

An offline desktop viewer for DOCSIS Proactive Network Maintenance (PNM) capture
files, with a Rust backend and a planned Tauri 2 + React/TypeScript interface.
The repository is `PNM-Viewer-Toolkit`; the standalone CLI is `pnm-decode`.
The parser design follows PyPNM, with shared file headers, typed measurement
objects, and human-readable JSON exports.

**In the dev phase:** the Rust parsing libraries and development tools are
implemented, including the standalone CLI and Linux source installer. The
desktop interface is not yet available.

## What works today

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
The desktop UI remains [planned work](docs/development/ui.md).

## Design direction

Rust owns binary decoding, numerical processing, exports, and future analysis
and storage. The planned React interface presents results through Tauri's system
WebView, without a local HTTP backend. Optional retrieval of existing PyPNM
captures is planned; modem control, capture initiation, SNMP/TFTP, Python
sidecars, and remote analysis are outside the viewer's scope.

## Repository layout

| Path | Purpose |
| --- | --- |
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
- Planned desktop shell and React/TypeScript UI:
  [GPL-3.0-only](LICENSES/GPL-3.0-only.txt).

Third-party material retains its original terms and notices.
Acknowledgment: **PyPNM - Proactive Network Maintenance Toolkit**.
See [NOTICE](NOTICE) and the [fixture references](tests/fixtures/pypnm/README.md).
