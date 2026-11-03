# Original PNM Viewer development guide

Historical reference from the 2026-11-02 bootstrap. Current user instructions,
root AGENTS.md and implemented source take precedence. This reference does not
authorize new implementation or external actions.

Build a new project named PNM Viewer. Complete a working, reviewable bootstrap and an evidence-supported first vertical slice. Do not modify PyPNM. Do not stop after proposing an architecture. If PDFs/captures are missing, complete the runnable shell, raw binary viewer, contracts, retrieval mock tests and packaging configuration, leaving affected semantic decoders explicitly unsupported.

PRODUCT REQUIREMENTS
Use Tauri 2, React, TypeScript and Vite for a polished lightweight desktop UI. Rust handles all backend behavior: local files, parsing, analysis, aggregation, scientific filtering, decimation, retrieval, cache, persistence and data exports. React handles rendering, navigation, selections and transient UI state; coordinate transforms for drawing are fine, scientific interpretation in TypeScript is not. No embedded Python/PyPNM, Python sidecar, production Node runtime, Electron, local HTTP backend, SNMP, TFTP, modem control or measurement triggering. Bundle all frontend assets locally: no CDN, remote fonts, telemetry or network requirement for startup/local viewing.

REFERENCE ARCHITECTURE: VERIFY BEFORE IMPLEMENTATION
Inspect https://github.com/PyPNMApps/PyPNM at commit 82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9 using the accompanying PYPNM-ARCHITECTURE.txt source map. Work in a separate viewer repository; never write into the reference checkout or import/run its application startup. If targeting another revision, record it and reverify contracts.
PyPNM api/routes groups feature routers, services and schemas. The RxMER service inherits CommonMeasureService and orchestrates SNMP/TFTP capture: do not port that machinery. advance/common/operation_manager.py persists operation-to-capture-group mappings; common/classes/file_capture/pnm_file_opearation.py resolves operations into groups and transactions (opearation is the actual filename). CaptureDataAggregator reads group artifacts into TransactionCollection; measurement aggregators such as DsRxMerAggregator organize typed captures by channel/time and calculate summaries. Binary decoding is in pnm/parser, with PnmHeader, PnmFileType and GetPnmParserAndParameters; typed results live in pnm/parser/model. Analysis/report logic is under api/routes/common/classes/analysis, api/routes/basic and api/routes/advance/analysis. PnmArtifactStore manages compression/materialization.
Adopt those conceptual boundaries, improving separation so Rust parsing and numerical libraries have no API/UI dependencies. PyPNM is Apache-2.0 implementation evidence, not normative DOCSIS authority. Header exceptions, scaling/clamping and dispatch support require verification. A parser filename does not establish working format support.

PROPOSED DIRECTORY TREE
pnm-viewer/
  AGENTS.md
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  package.json
  package-lock.json
  LICENSES/{GPL-3.0-only.txt,Apache-2.0.txt}
  NOTICE
  README.md
  apps/desktop/
    package.json
    vite.config.ts
    src/{app,components,features,charts,ipc,styles}/
    src-tauri/
      Cargo.toml
      tauri.conf.json
      capabilities/default.json
      src/{main.rs,lib.rs,commands.rs,state.rs,dto.rs}
  crates/
    pnm-types/src/         # units, capture identity, typed results, diagnostics, provenance
    pnm-parser/src/        # bounded reader, registry, headers, measurement modules
    pnm-analysis/src/      # scientific transforms/statistics
    pnm-aggregate/src/     # compatible multi-capture comparison and alignment
    pnm-io/src/            # bounded import, compression, cache, persistence, export
    pnm-pypnm-client/src/  # optional Rust HTTP adapter and API DTOs
  tests/{fixtures,contracts,integration}/
  fuzz/
  specs/
    private/{docsis-31,docsis-40}/  # ignored, user-provided PDFs
    manifests/
    field-provenance.csv
    coverage.csv
  .agents/skills/{docsis-31-ossi,docsis-40-ossi}/
    SKILL.md
    references/evidence-record.json
  docs/{architecture,api-contract,format-support,packaging,validation}.md
  scripts/
  .github/workflows/{checks,packages}.yml

Use a Cargo workspace with narrow, acyclic dependencies. Measurement families are modules, not one crate per measurement. Reusable libraries depend on domain types, never on app/Tauri. Analysis consumes parsed types; aggregation consumes typed captures/results; transport and I/O feed the same local pipeline. Use abstraction where it enables mock adapters and tests, not speculative plugin machinery.
Copy the supplied permanent AGENTS.md initially and maintain it with architecture decisions. Copy the two supplied skill folders into .agents/skills, without globally installing them. GPL-3.0-only applies to the desktop shell and React UI; reusable crates above are Apache-2.0. Set each package license explicitly, include full license texts and preserve upstream Apache notices where translation/copying requires them. Do not relicense dependencies. Inventory third-party licenses and review distribution compatibility. Document corresponding-source obligations for shipped GPL app binaries.

RUST PIPELINE AND CONTRACTS
Use one pipeline for local and retrieved captures: bounded import -> compression identification -> format identification -> checked parse -> normalized typed capture -> analysis -> compatible aggregation -> bounded display DTO/export. Preserve original bytes and SHA-256. Separate source identity, file-format identity and optional DOCSIS applicability. Keep units, invalid-value masks, absent values and provenance explicit. Unknown types/versions produce Unsupported diagnostics and raw hex inspection, never a best-guess decoder. Do not turn absent capture time into a real epoch-zero observation.
Return structured errors with code, field/byte offset where known, user-readable message and source context. Check count*stride and arithmetic before slicing/allocation. Handle truncated headers/payloads, impossible lengths, trailing bytes, unsupported variants and bad encodings without panic. Define documented defaults for file bytes, samples, archive entries, total expansion, nesting and work/time limits. Avoid unsafe code without a measured need and review.
Keep large data in Rust behind opaque session IDs with resource limits and cleanup. Define typed commands for open/import, metadata, hex range, series viewport, analysis, comparison, export, close/cancel and optional remote listing/download. Generate or verify TypeScript DTOs against Rust. Test nulls, error unions and integer ranges; serialize large integers without JS precision loss. Run expensive work off the UI thread with bounded concurrency, progress and cancellation. Prevent stale jobs from replacing the latest selection.
Rust performs transforms and peak-preserving viewport reduction. Preserve gaps, invalid samples and extrema, and label display versus full resolution. Never send a whole capture as JSON merely to draw a small viewport. Export full-resolution data from Rust, with units/provenance; frontend image export may capture an already rendered chart.
Aggregation requires compatible device/channel/measurement/format/units and frequency grids. Do not silently compare by array index, mix MACs or resample grids. If explicit alignment is added, document and test the method. Preserve timestamps and transaction/operation/capture identities separately.

UI AND INITIAL IMPLEMENTATION
Provide native file-open and drag/drop through constrained Tauri commands, recent files, metadata/provenance, support badges, paginated hex/ASCII offsets, clear error states and responsive charts. Include keyboard accessibility, visible focus, light/dark themes, explicit units, zoom/pan/reset, hover values, channel/capture selection and compatible overlays. Provide optional PyPNM settings without making server configuration part of opening a local file. Use minimal Tauri capabilities and CSP; do not expose arbitrary filesystem/shell/network access to the frontend.
Start with a runnable local raw-byte viewer and tested Rust ingestion. Prioritize one RxMER vertical slice once evidence and usable captures are supplied. Then add spectrum, constellation, channel estimation, upstream pre-equalization, histogram, FEC summary and modulation profiles as evidence permits. Do not claim blanket DOCSIS 3.1/4.0 support. Every support-matrix row identifies format/version, evidence, fixture and supported analysis. If sources are absent, generic byte fixtures test hex/import only; do not invent synthetic PNM formats to demonstrate charts. Clearly labeled UI demo data is not a valid capture fixture.

OPTIONAL PYPNM RETRIEVAL: RUST ONLY
Feature-gate the adapter so offline-only builds exclude HTTP dependencies. Use a bounded Rust HTTP client; React must not fetch PyPNM directly. Support a configured base URL/path prefix, encoded path segments, TLS validation, timeouts, streaming limits, cancellation and explicit connection state. Allow intentionally configured local HTTP; prefer HTTPS for remote profiles. Store secrets in an appropriate OS credential store or memory, not logs/plain settings/frontend state. Avoid cross-origin credential forwarding; reject unsafe redirects and HTTPS downgrade. Treat server filenames as untrusted, generate cache names and atomically commit completed validated downloads.
Verified pinned routes under /docs/pnm/files (preserve casing and trailing slash):
- GET /getMacAddresses/ returns {mac_addresses: [{mac_address, system_description?}]}.
- GET /searchFiles/{mac_address} returns {files: {MAC: [{transaction_id, filename, pnm_test_type, timestamp, system_description?}]}}; this is a map, not a flat array. The service can mask lookup errors as empty results, so an empty list alone does not prove no captures exist.
- GET /download/transactionID/{transaction_id} returns the physical stored artifact as application/octet-stream; it may be compressed despite router prose describing raw data.
- GET /download/filename/{filename} resolves a registered filename and explicitly materializes an uncompressed file. This is a useful initial download path after selecting a search result when client compression support is absent.
- GET /download/macAddress/{mac_address} and /download/operationID/{operation_id} return ZIPs of physical stored files, possibly compressed internally. Missing stored files can be skipped; do not claim archive completeness.
Begin with discovery and individual retrieval. Group ZIP retrieval may remain unsupported until extraction rejects traversal, absolute paths, links, collisions and expansion/nesting-limit violations. Detect only supported compression from content and verified contracts, not extension alone. Unsupported compression yields a clear error, never compressed bytes passed to a PNM parser. Do not call upload, getAnalysis, remote hex dump, capture start/stop, SNMP or TFTP endpoints. All parsing/analysis stays local in Rust.
Verify the configured deployment's OpenAPI when available and user-authorized; do not start or modify PyPNM to generate it. Keep pinned contract fixtures and report schema mismatches. Test with a mock server; live integration remains explicitly unverified without a supplied endpoint.

DOCSIS 3.1/4.0 OSSI SKILLS
Use the supplied separate skills. Require user-provided version-specific OSSI PDFs and any referenced normative documents needed for actual PNM binary definitions. Do not substitute remembered layouts, MIB management objects, API schemas or MULPI configuration TLVs. These scaffolds contain no protocol definitions.
Record document ID/title/issue/date/SHA-256 and physical/printed pages, section/table/figure. For each field record offset/width/signedness/byte order/scale/units/count/length rules/sentinels/applicability and supporting evidence. Render-check layout-sensitive pages and flag OCR uncertainty. Keep 3.1 and 4.0 evidence independent and file-format version separate from protocol version. Label PyPNM observations with commit/path/lines and distinguish them from normative claims. Conflicting/missing evidence means partial/unsupported semantics, not invented defaults. Keep PDFs/private captures out of Git, published docs and installers. Missing PDFs must not block the generic bootstrap.

50 MB BUDGET AND PACKAGING
Interpret 50 MB as 50,000,000 bytes per primary architecture-specific distributable, including bundled prerequisites/assets. Enforce it on measured artifacts, not framework estimates. Report compressed installer bytes, executable bytes, installed footprint and external prerequisites separately. Target Windows x64 NSIS setup.exe (plus MSI if offered), macOS x86_64 and aarch64 DMGs separately, Ubuntu x86_64 .deb and AppImage if offered. Every offered primary format is gated; do not silently exempt a larger package. Additional architectures need explicit support/test rows.
Use native build runners, pinned toolchains/lockfiles and a documented Ubuntu compatibility baseline. Test WebKitGTK/runtime requirements; a newer Ubuntu build does not prove older-host support. Use system WKWebView on macOS and explicitly configure/test Windows WebView2 installation policy. Offline viewing after prerequisites are installed is required. Fully offline installation on a pristine Windows machine without WebView2 is a separate promise and may exceed budget. Measure any self-contained runtime installer separately and report the tradeoff; never hide runtime downloads. If a required package exceeds budget, give exact size/dependency attribution and request a concrete scope decision rather than disabling the gate or reporting success.
Use measured release optimization/LTO/stripping settings and separate debug symbols. Minimize features/plugins, audit dependency and frontend bundle contributions, choose a lightweight chart renderer and lazy-load where useful. Do not bundle Python, Chromium, PDFs, reference datasets or customer fixtures. Preserve parser correctness when optimizing. Produce checksums, dependency/license inventory, SBOM and build instructions. Prepare signing/notarization hooks without secrets, leave publication disabled and label unsigned local artifacts. Do not publish, externally sign/notarize or install system-wide without user authorization.
Packaging references: https://v2.tauri.app/distribute/ and https://v2.tauri.app/distribute/windows-installer/ . Recheck current Tauri 2 configuration documentation before choosing exact settings; pin actual dependency versions and toolchain rather than writing unverified version numbers.

TESTS, CI AND ACCEPTANCE
Rust: fmt, Clippy with warnings denied, workspace tests and offline/retrieval feature combinations; evidenced golden decodes; truncation/overflow/unknown-version/sentinel/endianness cases; independently calculated numerical checks; aggregation grid/device incompatibility; archive/compression limits; cancellation and fuzz targets. PyPNM differential tests are optional development aids, never the sole normative oracle or production dependency.
UI: typecheck, lint, production build, tests for empty/loading/error/unsupported states, IPC contract serialization, accessible controls and chart units/selections. Browser mock-IPC tests do not replace native Tauri integration.
Retrieval mock tests: exact paths/map shapes, base path, URL encoding, redirects, secret redaction, timeouts, wrong content, compressed artifacts, 404/5xx, interrupted downloads, cancellation and resource limits.
CI: Windows/macOS/Ubuntu native matrix, both macOS CPU targets with matching-hardware runtime tests or explicit limitations; locked dependencies; static checks/tests/builds; package size gates and retained checksums/reports. Run packaged install/open/local-file/offline/retrieval smoke tests in suitable clean environments and identify what actually ran. Cross-compilation alone is not runtime verification.
Benchmark cold start, import latency, peak memory and chart responsiveness with documented hardware and a representative legal fixture. Test the largest configured capture and cancellation under load; report measurements without invented guarantees.
Deliver runnable source, permanent AGENTS.md, both specification skills, architecture/API/coverage/packaging documentation, tests/CI and a validation report with actual commands/results/artifacts/sizes. If only the current OS can run, complete its checks and leave other runtime validations explicitly pending with CI configured. Missing source documents must be listed precisely. Never report unrun checks as passing, invent file formats, alter PyPNM or publish the project.
