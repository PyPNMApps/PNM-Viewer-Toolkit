# Original PyPNM source inspection

Historical reference from the 2026-11-02 bootstrap. Current user instructions,
root AGENTS.md and implemented source take precedence. This reference does not
authorize new implementation or external actions.

PyPNM architecture inspection
Inspected public PyPNMApps/PyPNM main at 82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9, using an isolated shallow clone. The connected account has administrative access to this organization repository; the personal mgarcia01752/PyPNM repository was listed with size zero. The organization repository is the inspection target assumed here. No PyPNM code, configuration, dependencies or remote state was modified. No Python application imports or capture services were run. Static source inspection only; no live server contract has been tested.

Architecture and source evidence

FastAPI composition
Creates FastAPI and runs startup initialization. api/utils/auto_load.py discovers router.py modules and registers their exported router objects. Do not import the app merely to inspect it: startup has side effects.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/main.py

Discovery
RouterRegistrar scans api/routes, honors __skip_autoregister__, imports modules and includes routers; registration errors are collected.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/utils/auto_load.py

Feature organization
Feature directories group router.py, service.py and schemas where needed. CmDsOfdmRxMerService extends CommonMeasureService and selects a measurement type. This is modem capture orchestration involving SNMP/TFTP, not a pure file decoder.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/docs/pnm/ds/ofdm/rxmer/service.py

Multi-capture services
MultiRxMerService extends AbstractCaptureService; repeated capture cycles call measurement services. Related workflows live under advance/multi_ds_chan_est and advance/multi_us_ofdma_pre_eq.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/advance/multi_rxmer/service.py

Operation persistence
OperationManager associates operation_id with capture_group_id, metadata and persisted state in a JSON database with locking. Operation IDs, capture groups and individual file transactions are distinct identities.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/advance/common/operation_manager.py

Group resolution
OperationCaptureGroupResolver bridges operations to capture groups and transaction records. The spelling opearation is the actual source filename.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/common/classes/file_capture/pnm_file_opearation.py

Artifact aggregation
CaptureDataAggregator resolves a capture group, reads transaction records through PnmArtifactStore and builds TransactionCollection; it can release retained byte payloads.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/advance/common/capture_data_aggregator.py

Measurement aggregation
DsRxMerAggregator accepts typed RxMER parsers, groups captures by channel/time, enforces one MAC, validates equal frequency/value lengths and computes min/average/max. The viewer should additionally reject incompatible frequency grids before comparing captures.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/common/classes/collection/ds_rxmer_aggregator.py

Parser entry and dispatch
GetPnmParserAndParameters dispatches to measurement-specific parsers. Other helpers are pnm_file_type.py and pnm_type_header_mapper.py. Some branches explicitly raise NotImplementedError; do not claim every named type is supported.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/pnm/parser/pnm_parameter.py

Common header
PnmHeader parses a common header with a special FEC-summary branch lacking capture_time. This is implementation evidence, not proof that a layout applies to all DOCSIS versions.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/pnm/parser/pnm_header.py

Typed decoder example
CmDsOfdmRxMer validates the measurement type, reads binary fields using struct, checks payload length and builds a typed model. It also includes derived frequencies/statistics, so Rust should separate parsing from analysis more strictly.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/pnm/parser/CmDsOfdmRxMer.py

Models
Pydantic parser results live here and in pnm_base_model.py. HTTP models are feature schemas.py; analysis models are under api/routes/common/classes/analysis/model; transaction/compression records are under file_capture/types.py.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/pnm/parser/model/parser_rtn_models.py

Analysis
Analysis dispatch and measurement calculations are separate from route declarations, but live under the API package. Basic report generators live in api/routes/basic; advanced analysis lives in api/routes/advance/analysis. Reuse the conceptual layers, not the Python package coupling.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/common/classes/analysis/analysis.py

Storage and compression
PnmArtifactStore handles durable artifacts, at-rest compression and materialized caches. Retrieval may return a stored compressed file rather than decoded measurement bytes.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/pnm/lib/pnm_artifact_store.py

Retrieval declarations
PnmFileManager declares the /docs/pnm/files prefix and the exact paths listed below.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/docs/pnm/files/router.py

Retrieval behavior
PnmFileService implements transaction resolution, physical storage retrieval, uncompressed materialization, ZIP grouping, upload and analysis.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/docs/pnm/files/service.py

Retrieval JSON shapes
FileQueryResponse.files maps MAC strings to FileEntry arrays. Entries carry transaction_id, filename, pnm_test_type, timestamp and optional system_description. MAC listing returns mac_addresses entries with mac_address and optional system_description.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/src/pypnm/api/routes/docs/pnm/files/schemas.py

License
Apache License 2.0; NOTICE is also present. Preserve applicable upstream notices if translating or copying expressive code.
Source: https://github.com/PyPNMApps/PyPNM/blob/82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9/LICENSE

Verified retrieval routes (case and trailing slash matter)
GET /docs/pnm/files/getMacAddresses/ — registered MAC/system-description records.
GET /docs/pnm/files/searchFiles/{mac_address} — JSON files map, not a flat list. The service catches broad exceptions and may return an empty list on lookup failure; an empty response alone cannot prove no data exists.
GET /docs/pnm/files/download/transactionID/{transaction_id} — physical stored artifact, application/octet-stream. Can be compressed. Router prose saying raw binary does not override service behavior.
GET /docs/pnm/files/download/filename/{filename} — database-resolved filename, explicitly materialized uncompressed, application/octet-stream.
GET /docs/pnm/files/download/macAddress/{mac_address} — ZIP of physical stored artifacts, possibly compressed internally.
GET /docs/pnm/files/download/operationID/{operation_id} — ZIP via operation/capture-group resolution, possibly compressed internally.
GET /docs/pnm/files/getHexdump/transactionID/{transaction_id} — server JSON hex dump; unnecessary for Rust local decoding.
POST /docs/pnm/files/upload and POST /docs/pnm/files/getAnalysis also exist. The viewer must not call them: retrieval only, with all analysis in Rust. Measurement capture routes and advanced capture start/stop operations are outside viewer scope.

Design consequence: adopt feature modules and typed boundaries, but omit modem controls, Python runtime, server-side analysis and persistent capture orchestration. Prefer individual transaction retrieval with explicitly supported compression, or the verified uncompressed filename route after selecting a search result. Use the same Rust import/parse/analyze pipeline for local and retrieved files.

Packaging sources (checked during preparation)
https://v2.tauri.app/distribute/
https://v2.tauri.app/distribute/windows-installer/
https://v2.tauri.app/reference/config/
Tauri supports platform-native packages and configurable WebView2 installation. A compact app package does not imply a self-contained offline WebView2 installer. Measure actual artifacts rather than promising a size from framework choice.
