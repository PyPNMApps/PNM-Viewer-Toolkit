# PNM Viewer Toolkit documentation

The Qt viewer, CLI and Rust parsers are available from source. The project is
under development. Tag automation prepares draft native packages; public
downloads are not available yet.

## Use the toolkit

- [Desktop user guide and starting screen](ui/README.md)
- [Measurement screenshots](ui/measurements.md)

- [Installation and binary availability](install/README.md)
- [Build only the CLI from source](install/source.md)
- [CLI user guide, output and errors](cli/usage.md)
- [JSON examples for each PNM file type](cli/usage.md#json-examples-by-file-type)
- [Supported file formats](reference/formats.md)
- [Man page](../crates/cli/man/pnm-decode.1)

## Develop the toolkit

- [Development environment](install/development.md)
- [Rust APIs, numeric helpers and tests](development/rust.md)
- [QAM resource conversion and decision boundaries](development/qam-resources.md)
- [Runnable parser examples](../crates/core/examples/README.md)
- [Retained Tauri RxMER prototype](development/ui.md)
- [Planned UI IPC and optional retrieval](development/api-contract.md)
- [Git workflow](development/git.md)
- [Continuous integration](development/ci.md)

## Release and background

- [Versioning](release/versioning.md)
- [Packaging and platform validation](release/packaging.md)
- [Bootstrap decisions](history/bootstrap-review.md)
- [Original source investigation](history/source-review.md)

## Organization

User workflows and screenshots belong in `ui/`, installation in `install/`,
command behavior in `cli/`,
format contracts in `reference/`, contributor/API guidance in `development/`,
and distribution requirements in `release/`. Historical investigations belong
in `history/` and must not be presented as current capabilities.

Detailed working plans and private specification evidence remain in the separate,
ignored repository-root `development/`. They are not part of downloaded checkouts.
This structure follows PyPNM's topic-oriented docs and Oscar's separation of
usage, reference, and historical material; it does not copy their runtime design.

- [Qt Quick desktop prototype](development/qt.md): setup, launch and tests.
