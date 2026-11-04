# Desktop UI roadmap

Status: planned, not implemented. PNM Viewer Toolkit still needs its desktop UI.
The approved direction is Tauri 2 with React/TypeScript presentation and the
existing Rust core for parsing, math and exports. It is a desktop application
using the system WebView, without a local HTTP backend.

## Intended layout and ownership

```text
apps/desktop/              Future desktop application
  src/                     React/TypeScript interface
  src-tauri/               Rust Tauri shell and bounded IPC
crates/core/               Existing reusable pnm library
crates/cli/                Existing standalone pnm-decode command
crates/logging/            Shared logging setup
```

React will render navigation, metadata, charts and transient interaction state.
Rust will own file access, capture sessions, parsing, processing, exports and
bounded viewport data. The UI calls core APIs directly; it does not launch the
CLI to parse captures. Reuse centralized file detection and typed capture objects.

## Suggested implementation order

1. Create the desktop shell and application manifests.
2. Open a local capture and show detected identity, header and clear diagnostics.
3. Add an RxMER chart and JSON export using existing core APIs.
4. Add views for other supported measurements and preserve unavailable samples.
5. Add bounded sessions/viewport queries, cancellation and cleanup tests.
6. Package the full toolkit with the CLI and validate native installation on
   Windows, Linux and both macOS architectures.

These are upcoming tasks, not claims of working features. Symbol payload decoding
remains unsupported; latency lacks real-fixture validation. Optional retrieval
of existing PyPNM files is later scope, with no modem capture/control operations.

See [IPC design](api-contract.md), [format support](../reference/formats.md),
[development environment](../install/development.md) and
[packaging requirements](../release/packaging.md). Detailed evolving architecture
and checklists remain under ignored repository-root `development/`.
