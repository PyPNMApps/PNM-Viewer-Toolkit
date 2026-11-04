# API and IPC contract plan

## Local IPC

Use bounded typed commands for open/import, metadata, hex range, series viewport,
analysis, comparison, export, close/cancel and optional listing/download. Rust
owns session IDs and input access; frontend capabilities must not permit arbitrary
filesystem/shell/network operations. Restrict CSP and bundle assets locally.
Verify/generated DTOs must preserve nulls, error variants and large integers
without JavaScript precision loss. Document numeric limits before implementation.

## Optional PyPNM retrieval

Pinned reference: 82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9.
Source prefix: src/pypnm/api/routes/docs/pnm/files/.
Static inspection of router.py, service.py and schemas.py confirmed:

| Route under /docs/pnm/files | Contract/caution |
| --- | --- |
| GET /getMacAddresses/ | mac_addresses list with mac_address and optional system_description |
| GET /searchFiles/{mac_address} | files is a MAC-to-entry-list map; lookup exceptions may appear as empty results |
| GET /download/transactionID/{transaction_id} | Physical stored artifact; may be compressed |
| GET /download/filename/{filename} | Registered filename materialized uncompressed |

Group ZIP routes are recorded in the supplied source map; defer support until
archive path/link/collision and expansion-limit checks are implemented. Their
completeness is not guaranteed. Never call upload, getAnalysis, remote hex dump
or capture endpoints. Local and downloaded files enter the same Rust pipeline.

Feature-gate HTTP dependencies. Test exact paths/casing, base path prefixes,
encoding, TLS/redirect policy, credential isolation, timeouts, cancellation,
content/compression errors and bounded streaming/atomic cache writes with mocks.
Store secrets in OS credentials or memory, never frontend state or plain settings.
Reverify a configured deployment's OpenAPI when authorized. No live service was
called and no runtime contract tests have run in this review.

## Implemented RxMER commands

Only the main window has core capabilities. The frontend has no generic filesystem,
dialog, shell or network permission. Rust opens native pickers itself.

| Command | Inputs | Result |
| --- | --- | --- |
| `open_capture` | None | null on cancel; session_id, metadata, initial viewport on success |
| `viewport` | sessionId, start | start, total, up to 1,024 points |
| `export_json` | sessionId | false on cancel; true after writing a new full JSON file |
| `close_capture` | sessionId | Releases matching session; old handles cannot close a newer one |

A point contains `frequency_hz: u32` and `rxmer_db: number or null`. Viewports use
exact samples without reduction; start is a zero-based sample index. Session IDs
are monotonically checked u32 values, safe in JavaScript. Stale handles fail.
One active session owns original file bytes; replacement happens only after a
successful parse. Export workers retain an Arc, not a second byte buffer. Parsing
borrows these bytes afresh for each bounded query; no self-referential structs.

Input and JSON output each have a 16 MiB limit. Parsing uses the core's 65,536-item
budget. File operations run on a blocking worker; a backend operation gate allows
one open/export job at a time. The UI disables competing operations and reports
selection/loading/export status. Native dialogs provide cancellation; there is no
mid-read cancellation for these bounded local operations. Closing the window
releases process memory. Long jobs, streaming progress and cancellation tokens
remain future work.

Errors serialize `code`, `message`, optional `field` and `offset`. Parser offsets
refer to original file bytes. The frontend retains its previous capture on errors.
Rust tests verify bounds/nulls/core JSON parity and stale sessions; frontend tests
exercise theme changes, errors, dialog cancellation, paging, export and close.

## Parallel Qt shell

`apps/qt/src/backend.rs` exposes local-file open, JSON export, page selection and
close through a CXX-Qt QObject. Properties carry metadata, bounded viewports,
status, busy state and error text. Capture bytes stay in `apps/viewer`, shared
with Tauri. The Qt adapter accepts local QUrl paths and queues worker completions.
