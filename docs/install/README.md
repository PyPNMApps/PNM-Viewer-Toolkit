# Installation

Choose the standalone decoder or, once available, the full desktop toolkit.

| Component | Current availability |
| --- | --- |
| CLI from source | Implemented; built and tested on Ubuntu 22.04 x86_64 |
| Prebuilt CLI downloads | Planned; no published downloads claimed |
| Desktop UI/full toolkit | Planned; not implemented |

## Prebuilt binaries (planned)

Most users should eventually download a binary without installing Rust:

| Platform | Planned CLI artifact |
| --- | --- |
| Windows x64 | ZIP with `pnm-decode.exe` |
| macOS Apple Silicon | ARM64 archive with `pnm-decode` |
| macOS Intel | x64 archive with `pnm-decode` |
| Linux x64 | Archive with `pnm-decode`; optional distribution packages later |

Artifacts will include documentation, licenses and checksums. Native platform
build-and-run verification is required before declaring support. Only the Linux
executable has been built and run so far; download links will be added when
artifacts are published.

## CLI only versus full toolkit

CLI-only installs the decoder and its documentation without a UI or WebView.
Future UI installers must include the same-version CLI. Both use the shared Rust
core directly. Removing the UI must preserve a separately installed CLI through
explicit package/component ownership rules; those installers are not built yet.

For today's CLI, see [source build](source.md), then [usage](../cli/usage.md).
UI contributors can read [development setup](development.md) and the
[UI roadmap](../development/ui.md). Packaging requirements live in
[release documentation](../release/packaging.md).
