---
name: pnm-viewer-development
description: Use the original PNM Viewer requirements and PyPNM source map when planning desktop architecture, ownership boundaries, retrieval, or packaging in this repository.
---

# PNM Viewer development references

Use root AGENTS.md and the current implementation as the active project contract.
Consult the original design only for the feature being worked on:

- [Bootstrap guide](references/bootstrap-guide.md): product requirements,
  Rust/UI ownership, file sessions, retrieval, scientific processing and packaging.
- [PyPNM source map](references/pypnm-source-map.md): historical read-only source
  investigation pinned to commit 82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9.

These are historical references, not instructions to implement the entire project
when this skill loads. Verify source paths and behavior before reuse. Current user
choices supersede the original bootstrap, including authorization to use clear
versioned OSSI text extracts. Do not claim rendered-PDF or latest-issue verification.

Use the canonical coding-style, docsis-31-ossi and docsis-40-ossi skills for their
respective work; their guidance is not duplicated here. Track current support in
docs/format-support.md and APIs in docs/rust-development.md. Keep evolving local
plans in ignored development/; the references here remain available in fresh clones.

Archive checksums and consolidation provenance are in
[references/source-provenance.json](references/source-provenance.json).
