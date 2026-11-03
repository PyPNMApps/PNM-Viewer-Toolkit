# Requirements

## Confirmed direction

| ID | Requirement |
| --- | --- |
| R1 | Name the project pnm-viewer. |
| R2 | Implement a Java desktop application with a modern UI, without an HTTP-based UI. |
| R3 | Use PyPNM as an example for PNM behavior and analysis. |
| R4 | Assess the existing Java implementation for reuse and updates. |
| R5 | Maintain DOCSIS 3.1 and 4.0 CM-OSSI reference skills for PNM work. |
| R6 | Use `/home/dev01/Projects/pnm-viewer` as the repository; copy initial setup from PersonalDevelopmentEnviroment. |

## Proposed first increment — pending scope agreement

Start with offline PNM files: open via file chooser or drag and drop, identify the
format/version, show available metadata, and display measurements as charts and
tables. Begin with downstream OFDM RxMER; add channel estimation and upstream
OFDMA pre-equalization after shared header handling is verified.

Proposed UI: file list on the left, measurement chart in the center, metadata and
sample details beside it. Include chart zoom/reset, cursor values with units,
keyboard navigation, readable scaling, and clear empty/loading/error states.
Theme choice, comparison overlays, and export remain optional follow-up scope.

Parsing should run off the UI thread with cancellation and bounded resource use.
Malformed, truncated, or unsupported input should produce a useful error without
crashing or presenting fabricated measurements. Preserve source precision in the
model; display rounding must not change exported or analyzed values.

## Decisions to resolve

- Offline viewer only initially, or live SNMP acquisition too?
- Linux, Windows, macOS: which platforms are required for the first increment?
- Java toolkit, supported JDK, build tool, chart library, and distribution format.
- Which file types and versions are highest priority, with representative captures?
- Location of the actual legacy Java files; supplied archive has directories only.
- Output formats, multi-file comparison, and settings persistence requirements.
- License and provenance of legacy Java code before importing it.
- Verify latest CM-OSSI issues and obtain newer source material if needed.

DOCSIS 4.0 is a requirements target, not a present support claim. Maintain an
explicit supported-format/version matrix as each parser is implemented.
