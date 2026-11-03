# Format support and evidence

No semantic decoder or raw-byte viewer is implemented yet.

| Target | Status | Evidence still needed |
| --- | --- | --- |
| Generic hex/ASCII | Planned first delivery | Bounded import and byte fixtures |
| RxMER | Awaiting source and fixtures | Pinned PDF field records, usable captures, independent expected values |
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
