# PNM Viewer

A desktop PNM viewer using Tauri 2, Rust, React, TypeScript and Vite.
The user adopted this stack on 2026-11-02, superseding the initial Java plan.
Status: in the dev phase; requirements and tooling only, no runnable app yet.

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

The [original bootstrap guide](development/development-guide.md) is
converted to Markdown alongside its source map in ignored `development/`.
These local planning links require that folder; it is not included in Git.
The original ZIP retains the source files covered by the bundled checksums.
Current repository instructions
retain the existing Git and MAJOR.MINOR.PATCH workflows.
