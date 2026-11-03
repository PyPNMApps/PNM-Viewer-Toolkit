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
