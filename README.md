# PNM Viewer

An offline desktop viewer for DOCSIS Proactive Network Maintenance (PNM) capture
files, with a Rust backend and a planned Tauri 2 + React/TypeScript interface.
The parser design follows PyPNM, with shared file headers, typed measurement
objects, and human-readable JSON exports.

**In the dev phase:** the Rust parsing libraries and development tools are
implemented. The desktop interface and installable application are not yet
available. You can run the parsers and export JSON today.

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
**header-only stub**. See [format support](docs/format-support.md) for the exact
coverage and JSON differences from PyPNM. This is not a claim of blanket
DOCSIS 3.1 or 4.0 compliance.

## Get started

Clone this repository using GitHub's **Code** button, then open a terminal in the
checkout. The development installer targets Ubuntu 22.04/24.04 on x86_64 or ARM64;
see [setup and platform limitations](docs/development-setup.md) for validation scope.

```bash
cd pnm-viewer
./install.sh --development --dry-run
./install.sh --development
./install.sh --development --check
```

Run as your normal user. Installation uses `sudo` for missing OS packages and
network access for missing dependencies. Pinned Rust and Node.js tools are kept
in ignored `local/`; the wrappers below use those tools.

### Export a capture to JSON

From the repository root:

```bash
mkdir -p local/pnm-json
scripts/cargo-local.sh run --quiet --locked -p pnm --example pnm_json -- \
  tests/fixtures/pypnm/tests/files/rxmer.bin > local/pnm-json/rxmer.json
```

Replace the input path with another supported capture. For example, export
channel-estimate I/Q coefficients:

```bash
scripts/cargo-local.sh run --quiet --locked -p pnm --example pnm_json -- \
  tests/fixtures/pypnm/tests/files/channel_estimation.bin \
  > local/pnm-json/channel_estimation.json
```

JSON goes to stdout; diagnostics go to stderr. Generated files under `local/`
are ignored by Git. Parsing local captures requires no PyPNM server or modem
connection. See [fixture provenance](tests/fixtures/pypnm/README.md) for the
included inputs and their licenses.

See [public parser examples](crates/core/examples/README.md) for a runnable
example of each parser and direct access to typed data.

### Run development checks

```bash
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
```

## Design direction

Rust owns binary decoding, numerical processing, exports, and future analysis
and storage. The planned React interface presents results through Tauri's system
WebView, without a local HTTP backend. Optional retrieval of existing PyPNM
captures is planned; modem control, capture initiation, SNMP/TFTP, Python
sidecars, and remote analysis are outside the viewer's scope.

## Repository layout

| Path | Purpose |
| --- | --- |
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

## Documentation and Git workflow

- [Development setup](docs/development-setup.md)
- [Rust APIs, numeric helpers and validation](docs/rust-development.md)
- [Supported formats and limitations](docs/format-support.md)
- [API and IPC design](docs/api-contract.md)
- [Packaging requirements](docs/packaging.md)
- [Git save and push helpers](docs/git-workflow.md)
- [MAJOR.MINOR.PATCH versioning](docs/release.md)
- [Bootstrap decisions](docs/bootstrap-review.md)

Use the project Git helpers described above to save and push changes. Version
bumps are separate from commits, tags, and publication. See
[AGENTS.md](AGENTS.md) for repository conventions when using a coding agent.

## License and acknowledgment

See [LICENSE](LICENSE) for the component-specific licensing:

- Reusable Rust libraries and project-owned development tooling:
  [Apache-2.0](LICENSES/Apache-2.0.txt).
- Planned desktop shell and React/TypeScript UI:
  [GPL-3.0-only](LICENSES/GPL-3.0-only.txt).

Third-party material retains its original terms and notices.
Acknowledgment: **PyPNM - Proactive Network Maintenance Toolkit**.
See [NOTICE](NOTICE) and the [fixture references](tests/fixtures/pypnm/README.md).
