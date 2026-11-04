# PNM Viewer Toolkit documentation

The CLI, Rust parsers and initial RxMER desktop source are available; no desktop
installer or published cross-platform download is available yet.

## Use the toolkit

- [Installation and binary availability](install/README.md)
- [Build only the CLI from source](install/source.md)
- [CLI commands, output and errors](cli/usage.md)
- [Supported file formats](reference/formats.md)
- [Man page](../crates/cli/man/pnm-decode.1)

## Develop the toolkit

- [Development environment](install/development.md)
- [Rust APIs, numeric helpers and tests](development/rust.md)
- [Runnable parser examples](../crates/core/examples/README.md)
- [Desktop UI roadmap](development/ui.md)
- [Planned UI IPC and optional retrieval](development/api-contract.md)
- [Git workflow](development/git.md)
- [Continuous integration](development/ci.md)

## Release and background

- [Versioning](release/versioning.md)
- [Packaging and platform validation](release/packaging.md)
- [Bootstrap decisions](history/bootstrap-review.md)
- [Original source investigation](history/source-review.md)

## Organization

User installation instructions belong in `install/`, command behavior in `cli/`,
format contracts in `reference/`, contributor/API guidance in `development/`,
and distribution requirements in `release/`. Historical investigations belong
in `history/` and must not be presented as current capabilities.

Detailed working plans and private specification evidence remain in the separate,
ignored repository-root `development/`. They are not part of downloaded checkouts.
This structure follows PyPNM's topic-oriented docs and Oscar's separation of
usage, reference, and historical material; it does not copy their runtime design.

- [Qt Quick desktop prototype](development/qt.md): setup, launch and tests.
