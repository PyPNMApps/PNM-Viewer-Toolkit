---
name: docsis-cm-ossi-4-0-i13
description: Look up and interpret CableLabs DOCSIS 4.0 CM-OSSI I13 (CM-SP-CM-OSSIv4.0-I13-260429). Use for cable-modem management requirements, SNMP/MIB behavior, FDX/FDD management, security-management models, PNM objects and file formats, status and interface models, QoS/DSID objects, event reporting, compliance reviews, and specification-derived tests. Do not use it as sole authority for MULPI, PHY, SEC, CCAP-OSSI, or requirements delegated to an RFC or external MIB.
---

# DOCSIS 4.0 CM-OSSI I13 Reference Skill

Use this skill when a task depends on the DOCSIS 4.0 **Cable Modem
Operations Support System Interface Specification**, issue **I13**, document
`CM-SP-CM-OSSIv4.0-I13-260429`.

This corpus is a layout-preserving text extraction for lookup. The original
CableLabs PDF remains authoritative, especially when a table, object
definition, MIB fragment, formula, figure, footnote, or page layout is
ambiguous in the extraction. Do not claim that a value was verified against
the PDF unless the PDF was available and inspected.

## PNM Viewer implementation

For PNM parser implementation or field-evidence work, apply the
[shared evidence workflow](../pnm-viewer-development/references/pnm-evidence-workflow.md).
Clear versioned text extracts are authorized; request PDFs for ambiguous layouts.
Keep this skill's revision-specific source and interpretation rules.

## Retrieval workflow

1. Read [the topic map](references/topic-map.md) to identify the likely area.
2. Use [the section index](references/section-index.md) to choose the smallest
   relevant file under `references/sections/`.
3. Search that section for the exact object, OID, event, notification, file
   field, status, association, interface, or technical term. Prefer `rg -n` so
   the result can be located again.
4. Read the complete surrounding paragraph, list, object table, file-format
   table, and applicable notes. Follow any cross-reference that affects the
   conclusion.
5. Confirm the actual section heading around the text. Section files preserve
   complete PDF pages, so adjacent extracts can share a boundary page and the
   first or last page can contain neighboring section text.
6. Use [the normative-keyword index](references/normative-keyword-index.tsv)
   only to locate candidate pages. Never treat its shortened context as the
   complete requirement.
7. Search [the full extraction](references/cm-ossi-full.txt) only when a topic
   spans major sections, a cross-reference cannot be resolved in the selected
   section, or a page-boundary split lacks necessary context.

## Interpretation rules

- Prefer the specification text over model memory or generic DOCSIS knowledge.
- Preserve normative force exactly. Never weaken or strengthen **MUST**,
  **MUST NOT**, **SHOULD**, **SHOULD NOT**, or **MAY**.
- Identify the requirement subject precisely: CM, management system/NMS, provisioning system, CMTS where explicitly referenced, or another actor. If unclear from complete context, say so rather than guessing.
- Distinguish normative text, informative text, examples, and engineering inference.
- Do not combine separate normative statements into a synthesized requirement unless explicitly labeled engineering inference.
- Do not invent OIDs, MIB/object names, access permissions, syntax, associations, enumerations, defaults, ranges, units, timers, file-format fields, TLV behavior, event codes, FDX/FDD behavior, security behavior, or management behavior.
- For object tables, MIB tables and file-format tables, verify row/column association against the PDF when there is any ambiguity.
- Treat Appendix I spectrum-analysis use cases as informative; do not promote them to normative requirements.
- Prefer concise paraphrase. Quote only the short portion needed to preserve
  exact normative meaning.

## DOCSIS 4.0-specific handling

- Keep FDX and FDD management requirements distinct. Do not infer one from the other.
- For DOCS-IF40-MIB, DOCS-FDX-MIB, and DOCS-FDD-MIB behavior, use the CM-OSSI text as the source only for what it explicitly states; if the external MIB module is needed for a conclusion, identify that dependency.
- Preserve distinctions among CM system configuration state, FDD system configuration state, FDX status, echo-cancellation training status, FDD capabilities, and FDX/FDD OFDMA interface objects.
- Do not back-port DOCSIS 4.0 object semantics into DOCSIS 3.1 unless a source explicitly establishes compatibility.

## Cross-specification boundaries

When CM-OSSI delegates or references behavior in MULPI, PHY, SEC, CCAP-OSSI, CMCI, an RFC, a MIB module, or another specification:

- Identify the referenced specification or standard.
- State what CM-OSSI itself establishes.
- Do not infer the external source's normative behavior unless that source is available in the repository.
- If the referenced skill is available, use it separately and identify which conclusion comes from which specification.
- Never silently mix text or requirements from DOCSIS 3.1 CM-OSSI or another DOCSIS 4.0 CM-OSSI revision.

## Results

- Cite substantive results as
  `CM-SP-CM-OSSIv4.0-I13-260429, Section <n>, PDF page <p>`.
- Include the annex, table, object, association, figure, MIB, or file-format
  identifier when it materially supports the result.
- For a code, requirement, or Jira compliance review, report:
  `source -> normative force and actor/mode -> object or implementation
  mapping -> pass, gap, or unresolved dependency`.
- For specification-derived tests, cover only cases supported by cited
  behavior. Include positive, negative, boundary, access-control,
  mode-applicability, state-transition, notification, or interoperability
  cases when they apply.
- State when the extraction is inconclusive and verification against the
  authoritative PDF is still required.

## Tables, MIBs, information models, and formulas

- Preserve table, object, attribute, association, figure, MIB, and formula identifiers when discussing them.
- Treat extracted table text and equations as potentially layout-sensitive.
- If a row/column relationship, access type, data type, enumeration, unit,
  range, subscript, operator, or formula is unclear, do not reconstruct it.
  Verify the PDF when available; otherwise report the limitation.
- Do not assume an object is writable, mandatory, persistent, or supported without checking its complete requirement context.
- Normative Annexes A-G may contain controlling requirements even when the main body provides overview text.

## Version and corpus discipline

All material in this skill is for **DOCSIS 4.0 CM-OSSI I13, issued
2026-04-29**. Never silently mix it with DOCSIS 3.1 CM-OSSI or another DOCSIS
4.0 CM-OSSI issue. When the requested issue is unclear and the difference
could matter, state the issue used. If the user requests another issue, use a
separate versioned skill or request that source.

Treat the extracted files as generated reference material. Do not casually edit
normative text, page markers, section extracts, or keyword indexes. Add another
DOCSIS document or revision as a separate skill so agents cannot silently mix
versions.

Source and extraction details are recorded in
[the specification metadata](references/spec-metadata.yaml).
