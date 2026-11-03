# PNM Viewer

A desktop PNM viewer using Tauri 2, Rust, React, TypeScript and Vite.
The user adopted this stack on 2026-11-02, superseding the initial Java plan.
Status: in the dev phase; Rust measurement parsers and tooling; no runnable desktop app yet.

Local files work offline without an HTTP backend. Rust owns decoding, analysis,
aggregation, storage and optional retrieval of existing PyPNM captures. React
renders the interface through the system WebView. No modem capture/control,
Python sidecar, production Node runtime, or remote analysis is planned.

- [Requirements](development/requirements.md)
- [Architecture and milestones](development/architecture.md)
- [Bootstrap review](docs/bootstrap-review.md)
- [API and IPC contracts](docs/api-contract.md)
- [Format support and evidence](docs/format-support.md)
- [Packaging](docs/packaging.md)
- [Validation checklist](development/checklist.md)
- [Source investigation](docs/source-review.md)
- [Git workflow](docs/git-workflow.md)
- [Versioning](docs/release.md)

The [development reference skill](skills/pnm-viewer-development/SKILL.md)
contains the original bootstrap guide and PyPNM source map. Duplicate root
bootstrap files have been removed. Working plans under `development/` are ignored
and local-only; their links require that folder.

## Project layout

| Directory | Purpose |
| --- | --- |
| `crates/` | Rust core and logging libraries |
| `docs/` | Project documentation and usage |
| `skills/` | Canonical agent skills and reference material |
| `specs/` | Format evidence and coverage records |
| `tests/fixtures/` | Preserved PyPNM binary captures and provenance |
| `scripts/`, `tools/` | Setup, build, Git and release helpers |
| `development/` | Ignored working plans and investigations |
| `local/` | Ignored toolchains, local data and JSON exports |
| `target/` | Ignored Cargo build cache |


- [Rust development and validation](docs/rust-development.md)

## Development setup

On Ubuntu 22.04/24.04 (x86_64 or ARM64), run as your normal user:

```bash
./install.sh --development --dry-run
./install.sh --development
```

The installer uses sudo for missing system packages and keeps pinned Rust and
Node.js tools in ignored `local/`. See [development setup](docs/development-setup.md)
for read-only checks, tool commands and platform limitations.
