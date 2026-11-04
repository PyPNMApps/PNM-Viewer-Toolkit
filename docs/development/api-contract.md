# Qt adapter and optional retrieval contracts

[Developer index](README.md) · [Qt implementation](qt-measurements.md)

## Implemented desktop boundary

The native Qt application lives in `apps/qt/`. `src/backend.rs` exposes a CXX-Qt
QObject to QML. File pickers produce local QUrl values; Rust validates paths,
reads files and exports JSON. There is no browser, JavaScript IPC or HTTP server.

| Operation | Adapter methods |
| --- | --- |
| Open/export | `open_file`, `export_json` |
| Frequency/bin zoom | `show_range` |
| I/Q region zoom | `zoom_constellation` |
| Channel-estimation tabs | `select_view` |
| FEC profile/time range | `select_fec_profile`, `show_fec_time_range` |
| Modulation selection | `toggle_modulation_profile`, `select_all_modulation_profiles`, `toggle_modulation_carrier`, `select_all_modulation_carriers`, `select_modulation_metric` |
| Close capture | `close_capture` |

Properties carry version, loaded/busy state, metadata JSON, bounded viewport
JSON, status and error text. JSON is a transport representation of Rust-owned
DTOs; QML does not add measurement fields or calculate scientific values.
See the [measurement internals](qt-measurements.md) for each viewport budget.

`apps/viewer` owns the original file bytes. Queries borrow the same buffer;
workers retain an Arc rather than duplicating it. Qt queues worker completion
onto the UI thread. File operations are gated; a failed open or invalid query
retains the previous valid capture/view. Window teardown safely rejects pending
callbacks. Close releases the active capture when no file operation is running.

Input and full JSON output each have a 16 MiB limit; decoding uses the core's
65,536-item budget. Existing destinations and original captures are never
overwritten. Native dialog cancellation makes no change. There is no mid-read
cancellation for these bounded operations. Streaming progress and cancellation
tokens remain future work.

Parser diagnostics retain field and byte offset in Rust; the adapter displays
code/message text. Rust tests cover bounds, missing values and exports. QML
tests cover controls, themes, drop routing and zoom; actual native picker and
installed-package behavior require separate platform checks.

## Future operations

Hex ranges, compatible capture comparison, persistence and optional retrieval
are plans rather than implemented Qt commands. Keep filesystem/network work
in Rust, expose bounded typed operations and retain errors and missing values.

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
Store secrets in OS credentials or memory, never QML state or plain settings.
Reverify a configured deployment's OpenAPI when authorized. No live service was
called and no runtime contract tests have run in this review.
