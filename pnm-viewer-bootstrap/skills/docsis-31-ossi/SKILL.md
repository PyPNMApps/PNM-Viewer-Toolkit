---
name: docsis-31-ossi
description: Ground DOCSIS 3.1 OSSI PNM parsing and interpretation in user-provided PDFs with field provenance and explicit unsupported cases. Use for 3.1 PNM protocol work, not configuration TLVs or another DOCSIS version.
---

# DOCSIS 3.1 OSSI evidence workflow

This scaffold contains no protocol definitions. Require the user's DOCSIS 3.1 OSSI PDF and any normative documents referenced for the needed PNM file layout. Until supplied, continue framework/raw-byte work and leave affected decoders unsupported. Never invent file formats.

1. Keep PDFs locally under specs/private/docsis-31/, excluded from Git and release bundles. Record exact document ID, title, issue, publication date, filename and SHA-256 in specs/manifests/docsis-31.json. Do not assume an issue or silently substitute a downloaded revision.
2. Extract text with page boundaries. Inspect rendered pages for tables, equations, diagrams, footnotes and scanned text. Flag OCR uncertainty and record physical PDF pages and printed page labels. Unreadable rows cannot support protocol claims.
3. Locate the actual measurement binary definition. Management objects, capture controls, transfer settings, OIDs, API schemas and file extensions do not establish byte layouts. Follow normative cross-references only when the referenced source is supplied or otherwise explicitly authorized and verified. Do not substitute MULPI configuration TLVs for OSSI PNM formats.
4. For each field record document hash/issue, pages, section/table/figure, offset expression, width, signedness, byte order, scaling, units, count/length rules, range, sentinels and applicability. Include a short supporting excerpt or faithful paraphrase. Unknown properties remain unknown, never convenient defaults. Do not redistribute PDFs or excessive excerpts.
5. Distinguish normative evidence from PyPNM observations pinned by commit/path/lines. Flag conflicts. Implementation-derived compatibility needs explicit labeling and fixtures; it is not specification compliance.
6. Keep 3.1 evidence independent from the other DOCSIS version. Do not transfer layouts without independent evidence. File-format versions and DOCSIS versions are separate identities; ambiguous applicability stays explicit.
7. Implement bounded Rust decoders and golden tests using legally usable captures or labeled synthetic fixtures based on evidenced layouts. Test lengths, endian rules, scaling, reserved bits, sentinels and unknown versions where grounded. Synthetic fixtures must not create a new supposed PNM format.
8. Update specs/field-provenance.csv and specs/coverage.csv with implemented, partial, unsupported or awaiting-source status. Cite exact evidence in decoder docs/test notes. Insufficient evidence means preserve bytes and stop affected semantic interpretation, while generic viewing continues.

Use references/evidence-record.json as the field-record shape. Null means not established. This scaffold does not assert that OSSI alone defines every requested capture family.
