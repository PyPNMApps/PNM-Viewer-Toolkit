# Format support and evidence

RxMER file version 1.0 is implemented in pnm-core; no desktop viewer exists yet.

| Target | Status | Evidence still needed |
| --- | --- | --- |
| Generic hex/ASCII | Planned first delivery | Bounded import and byte fixtures |
| RxMER 1.0 | Parser and fixture tests pass | Text-extract evidence recorded; PDF rendering and wider capture coverage pending |
| Other measurement families | Planned, unsupported | Per-format evidence and fixtures before implementation |

Use skills/docsis-31-ossi and skills/docsis-40-ossi for field provenance. Their
JSON templates contain unknown fields, not protocol definitions. Preserve the
existing I27/I13 extraction skills as issue-specific lookup aids; do not present
them as rendered-PDF verification or verified latest issues.

Place user-provided PDFs under ignored specs/private/docsis-31 or docsis-40.
Record document ID, issue/date/hash, physical and printed pages, section/table,
offset/width/signedness/endian/scale/units/counts/sentinels and applicability.
Create manifests and field-provenance/coverage records when evidence arrives.
Keep implementation evidence separate from normative evidence; no inferred
cross-version equivalence. Review layout-sensitive PDF pages visually.

PyPNM parser names are not support proof: pnm_parameter.py at the pinned revision
raises NotImplementedError for latency reports and symbol capture. Its common
header has an FEC-summary capture-time exception; do not interpret absence as a
real epoch-zero sample. These observations are not normative file definitions.

## Available RxMER fixtures

Pinned PyPNM RxMER and non-RxMER rejection fixtures plus the upstream Python
test are available in [tests/fixtures/pypnm](../tests/fixtures/pypnm/README.md).
Copies match the recorded upstream Git blobs and hashes. No Rust parser or
semantic validation is implied by importing these inputs.

See [Rust development](rust-development.md) for API, limits and validation results.
