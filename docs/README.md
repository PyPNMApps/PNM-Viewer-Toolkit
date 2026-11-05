# PNM Viewer Toolkit documentation

The Qt viewer, CLI and Rust parsers are available from source. The project is
under development. Tags temporarily build Ubuntu 22.04-only Actions artifacts
and a verified Ubuntu-only draft release;
see [DEB testing](release/ubuntu-deb-testing.md). The full native release workflow
is manual-only. See the
[Releases page](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases) for published downloads.

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

- [Developer starting point](development/README.md)
- [Native Qt setup, launch and checks](development/qt.md)
- [Qt measurement implementation](development/qt-measurements.md)
- [Development environment](install/development.md)
- [Rust APIs, numeric helpers and tests](development/rust.md)
- [QAM resource conversion and decision boundaries](development/qam-resources.md)
- [Runnable parser examples](../crates/core/examples/README.md)
- [Planned UI IPC and optional retrieval](development/api-contract.md)
- [Git workflow](development/git.md)
- [Continuous integration](development/ci.md)
- [Version bumps and first-tag checklist](development/releasing.md)
- [Refresh UI screenshots](development/screenshots.md)

## Release and background

- [Versioning](release/versioning.md)
- [Packaging and platform validation](release/packaging.md)
- [Temporary Ubuntu DEB and upgrade testing](release/ubuntu-deb-testing.md)
- [Release-page template](release/release-notes.md)
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
