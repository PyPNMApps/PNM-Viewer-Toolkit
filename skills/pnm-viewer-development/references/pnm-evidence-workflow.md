# Shared PNM evidence workflow

Use this workflow with the applicable versioned CM-OSSI skill. Keep DOCSIS 3.1
and 4.0 evidence independent, even when their layouts appear identical.

- The user authorized clear versioned text extracts for decoding. Request the
  PDF when table layout, formulas or footnotes are ambiguous. Do not claim
  rendered-PDF verification without inspection, or assume the pinned issue is
  the latest. Insufficient evidence blocks only the affected semantic decoding;
  preserve raw bytes and structured diagnostics.
- Locate the actual binary measurement definition. MIB objects, capture controls,
  OIDs, API JSON, extensions and MULPI configuration TLVs do not establish file
  layouts. Follow required normative cross-references only with available,
  authorized sources; label unresolved dependencies.
- Record exact document ID, title, issue, publication date and source SHA-256,
  section/table/page, field offset expression, width, signedness, byte order,
  scaling, units, lengths, ranges, sentinels and applicability. Distinguish PDF
  pages from printed page labels. Keep supporting text short.
- Use [evidence-record.json](evidence-record.json) as the record shape. Set
  protocol_version explicitly to the applicable version; null means unknown.
  File-format version is separate from DOCSIS version.
- Separate normative requirements, informative examples, implementation evidence
  and inference. Pin PyPNM observations by commit/path/lines and flag conflicts;
  compatibility with PyPNM is not specification compliance.
- Store private PDFs in ignored `development/specs/private/docsis-31/` or
  `docsis-40/` and manifests in `development/specs/manifests/docsis-31.json` or
  `docsis-40.json`. Update `development/specs/field-provenance.csv` and
  `development/specs/coverage.csv`. These local records are absent in fresh clones.
  Preserve source hashes and do not redistribute private PDFs or excessive text.
- Implement bounded decoders with real captures or explicitly synthetic fixtures
  grounded in the sources. Test length, byte order, scale, sentinel and version
  behavior without inventing protocol facts. Follow the
  [testing skill](../../pnm-testing/SKILL.md); record implemented, partial,
  unsupported or awaiting-source status and cite evidence in decoder/test notes.
