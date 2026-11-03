# Bootstrap bundle review

Reviewed 2026-11-02. Extracted pnm-viewer-bootstrap.zip into the sibling
pnm-viewer-bootstrap/ directory without replacing project files. All seven
manifest checksums match. Those original files were review inputs; see consolidation below for current paths.

## Decisions adopted

The user explicitly chose the bundle's Tauri 2/Rust/React stack, replacing Java.
Retain project Git helpers and MAJOR.MINOR.PATCH versioning. Install the supplied
evidence skills in canonical skills/ with both discovery links, rather than
copying separate skill bodies into discovery directories. Preserve versioned
OSSI extraction skills and distinguish them from PDF-backed field evidence.

Adopt offline-first local processing, optional existing-file retrieval only,
strict domain/UI boundaries, bounded opaque sessions, raw unsupported-file
inspection, evidence-gated semantic decoders, native packaging targets, the
50,000,000-byte package limit, and the supplied library/app licensing split.

## Source review

Local /home/dev01/Projects/PyPNM HEAD matches
82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9. Static inspection confirmed the files API
map shape, error-to-empty-list behavior, physical transaction downloads versus
uncompressed filename materialization, and unimplemented parser dispatch cases.
See api-contract.md and format-support.md. No PyPNM files were changed and no
application startup, capture services or live API requests were run.

## Findings and limits

- Tauri uses an embedded WebView; it satisfies no local HTTP backend, but replaces
  the earlier Java/no-WebView direction following explicit user selection.
- The bundle is detailed guidance, not runnable source. No app/CI/package exists.
- Existing I27/I13 extractions do not satisfy the bundle's rendered-PDF field
  verification gate or establish latest revision status.
- A parser filename is insufficient support evidence; the first delivery must
  show unsupported states rather than fabricate PNM formats.
- Runtime contracts and package-size promises require actual tests/measurements.
- Current toolchain/framework versions, resource limits and OS baselines remain
  implementation decisions; do not infer them from this review.

The original Java source investigation is retained as historical context. Missing
Java files no longer block the selected stack. No full bundle architecture audit,
scientific decoder validation or packaged-platform verification is claimed.

## Local development workspace

At the user's request, architecture, requirements and checklist documents moved
to ignored development/. BOOTSTRAP-PROMPT.txt and PYPNM-ARCHITECTURE.txt were
moved there and converted to development-guide.md and pypnm-architecture.md.
The working development files are local-only and will not be present in a fresh clone.

## Consolidation — 2026-11-03

The root ZIP and extracted `pnm-viewer-bootstrap/` directory were removed after
verifying all seven archive manifest entries. The four extracted OSSI skill files
were byte-identical to their canonical copies under `skills/`; those canonical
copies remain. Root `AGENTS.md` supersedes the old bootstrap instructions.

The two original planning references are preserved as Markdown under
[skills/pnm-viewer-development/references](../skills/pnm-viewer-development/references/).
The new [skill entry point](../skills/pnm-viewer-development/SKILL.md) makes those
references discoverable and distinguishes historical guidance from active policy.
`source-provenance.json` records the original archive hash and manifest. Discovery
links exist under both `.agents/skills/` and `.claude/skills/`.

Existing working plans, generated JSON, local toolchains and Cargo cache were
retained. No application source or parser behavior changed during consolidation.
