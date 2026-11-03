---
name: docsis-cm-ossi-3-1-i27
description: Look up and interpret CableLabs DOCSIS 3.1 CM-OSSI I27 (CM-SP-CM-OSSIv3.1-I27-250219). Use for cable-modem management requirements, SNMP/MIB behavior, fault, configuration, performance and security management, PNM objects and file formats, CM status and interface models, QoS/DSID objects, event reporting, compliance reviews, and specification-derived tests. Do not use it as sole authority for PHY, MULPI, SEC, CCAP-OSSI, or requirements delegated to an RFC or external MIB.
---

# DOCSIS 3.1 CM-OSSI I27 Reference Skill

Use this skill when a task depends on the DOCSIS 3.1 **Cable Modem
Operations Support System Interface Specification**, issue **I27**, document
`CM-SP-CM-OSSIv3.1-I27-250219`.

This corpus is a layout-preserving text extraction for lookup. The original
CableLabs PDF remains authoritative, especially when a table, object
definition, MIB fragment, figure, formula, footnote, or page layout is
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
   field, status, interface, or technical term. Prefer `rg -n` so the result
   can be located again.
4. Read the complete surrounding paragraph, list, object table, file-format
   table, and applicable notes. Follow any cross-reference that affects the
   conclusion.
5. Confirm the actual section heading around the text. Section files preserve
   complete PDF pages, so the first or last page can contain carryover from an
   adjacent section.
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
- Identify the requirement subject precisely: CM, management system/NMS, provisioning system, CMTS where explicitly referenced, or another actor. If the subject is unclear from the complete source context, say so rather than guessing.
- Distinguish clearly among normative text, informative text, examples, and engineering inference.
- Do not combine separate normative statements into a synthesized requirement unless the synthesis is explicitly labeled engineering inference.
- Do not invent OIDs, object names, access permissions, syntax, enumerations, defaults, ranges, units, timers, file-format fields, TLV behavior, event codes, or management behavior.
- For MIB/object tables and file-format tables, verify row/column association against the PDF when there is any extraction ambiguity.
- Treat Appendix I spectrum-analysis use cases as informative; do not promote them to normative requirements.
- Prefer concise paraphrase. Quote only the short portion needed to preserve
  exact normative meaning.

## Cross-specification boundaries

When CM-OSSI delegates or references behavior in PHY, MULPI, SEC, CCAP-OSSI, CMCI, an RFC, a MIB module, or another specification:

- Identify the referenced specification or standard.
- State what CM-OSSI itself establishes.
- Do not infer the external source's normative behavior unless that source is available in the repository.
- If the referenced skill is available, use it separately and identify which conclusion comes from which specification.
- Never silently mix text or requirements from different CM-OSSI revisions.

## Results

- Cite substantive results as
  `CM-SP-CM-OSSIv3.1-I27-250219, Section <n>, PDF page <p>`.
- Include the annex, table, object, figure, MIB, or file-format identifier when
  it materially supports the result.
- For a code, requirement, or Jira compliance review, report:
  `source -> normative force and actor -> object or implementation mapping ->
  pass, gap, or unresolved dependency`.
- For specification-derived tests, cover only cases supported by cited
  behavior. Include positive, negative, boundary, access-control,
  state-transition, notification, or interoperability cases when they apply.
- State when the extraction is inconclusive and verification against the
  authoritative PDF is still required.

## Tables, MIBs, and information models

- Preserve table, object, attribute, figure, and MIB identifiers when discussing them.
- Treat extracted table text as potentially layout-sensitive.
- If a row/column relationship, access type, data type, enumeration, unit, or
  value range is unclear, do not choose a value. Verify the PDF when available;
  otherwise report the limitation.
- Do not assume that an object mentioned in the specification is writable, mandatory, or supported without checking its complete requirement context.
- Normative annexes A-G may contain the controlling requirement even when the main body provides only overview text.

## Version and corpus discipline

All material in this skill is for **DOCSIS 3.1 CM-OSSI I27, issued
2025-02-19**. Do not silently apply requirements from another CM-OSSI issue.
When the requested issue is unclear and the difference could matter, state the
issue used. If the user requests another issue, use a separate versioned skill
or request that source.

Treat the extracted files as generated reference material. Do not casually edit
normative text, page markers, section extracts, or keyword indexes. Add another
DOCSIS document or revision as a separate skill so agents cannot silently mix
versions.

Source and extraction details are recorded in
[the specification metadata](references/spec-metadata.yaml).
