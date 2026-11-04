# Permanent project guidance

Build PNM Viewer Toolkit with Rust backend behavior and parallel desktop prototypes:
Tauri 2/React under apps/desktop and Qt Quick/QML under apps/qt. Preserve both.

- Work only in this viewer repository. PyPNM is read-only reference material. Do not change its files, configuration, dependencies or services, or run capture/startup code.
- No SNMP, TFTP, modem controls, capture initiation, Python sidecars, Node production runtime or remote analysis. Optional PyPNM HTTP retrieval runs in Rust and retrieves existing files only.
- React and QML render charts, navigation and transient interaction state. Rust owns filesystem access, parsing, scientific interpretation, analysis, aggregation, persistence, downloads, exports and decimation.
- Domain/parser libraries must not depend on Tauri, HTTP or app crates. Separate decoding from analysis, aggregation, storage and transport. Keep Cargo dependencies acyclic.
- Reusable libraries are Apache-2.0. Desktop shell and UI are GPL-3.0-only. Set package licenses explicitly; preserve third-party licenses, copyrights and applicable NOTICE content.
- Use separate project skills for DOCSIS 3.1 and 4.0 OSSI work. The user authorized clear versioned text extracts for RxMER; request PDFs for ambiguous layouts and never claim rendered-PDF verification. Do not substitute MULPI configuration TLVs, MIB objects or API JSON schemas for PNM binary layouts.
- Never invent file identifiers, offsets, widths, byte order, scaling, sentinels or version equivalence. Trace implemented fields to pinned PDF evidence or explicitly labeled implementation evidence. Protocol version and file-format version are separate.
- Missing evidence blocks affected semantic decoders, not the generic bootstrap. Unknown files remain viewable as bytes with structured diagnostics. Do not label unsupported formats complete.
- Bound external-input allocations, arithmetic, archive expansion and work. Return structured errors with offsets; preserve original capture bytes and never overwrite inputs.
- Keep large datasets in Rust behind opaque sessions. Use bounded typed IPC queries, cancellation, progress and cleanup; do not serialize whole captures for a viewport.
- Aggregate compatible device/channel/type/unit/format and frequency grids only. Preserve invalid samples and gaps. Document and test any explicit alignment transformation.
- Offline viewing must work without a server or network. Feature-gate the HTTP adapter and test offline-only and retrieval-enabled builds.
- Pin toolchains and lockfiles. Run Rust formatting, Clippy and tests; UI typecheck, lint, tests and production build. Verify IPC contracts and installed application behavior.
- CI builds Windows, macOS Intel/Apple Silicon and Ubuntu packages on suitable native runners. Report unverified targets honestly; a successful cross-compile is not runtime validation.
- Each primary architecture-specific distributable must be at most 50,000,000 bytes, counting bundled prerequisites and assets. Fail the release gate when exceeded. Report installer bytes, installed footprint and external prerequisites separately.
- Offline viewing after setup and offline installation on a pristine OS are different promises. Do not claim both fit the budget without measurement.
- Produce local artifacts and checksums. Publishing, external signing/notarization and system-wide installation require user authorization. Keep credentials, private PDFs and customer captures out of Git and release bundles.

## Project naming

Display name: PNM Viewer Toolkit. Repository/directory: `PNM-Viewer-Toolkit`.
Standalone CLI: `pnm-decode` (`pnm-decode.exe` on Windows); manual: `pnm-decode.1`.
Existing Cargo library names and copyright notices retain their current identities.

## Project status and guidance

The user adopted the bundle's Tauri/Rust stack on 2026-11-02, superseding the
initial Java plan. On 2026-11-03 the user requested a parallel Qt prototype while retaining Tauri.
Shared GPL desktop capture behavior lives under apps/viewer; Qt uses CXX-Qt at
the adapter boundary only. Core parsers remain safe Rust.
The project remains in the dev phase; the initial RxMER desktop slice is implemented under apps/desktop/. Native packaging remains pending. The core reader/header foundation is implemented.
Use development/requirements.md, development/architecture.md, development/checklist.md and
development/development-guide.md together. Maintain the docs as work lands.
Original bootstrap references live in skills/pnm-viewer-development/references/;
they are historical guidance, not generated app source.

Keep canonical skills under skills/ and relative links under both .agents/skills/
and .claude/skills/. Use docsis-cm-ossi-3-1-i27 and docsis-cm-ossi-4-0-i13 for OSSI lookup
and field-evidence work. Both link to one shared PNM evidence workflow; text
extractions do not establish rendered-PDF validation.
Validate changed skills and discovery links. The redundant root ZIP and extracted
bootstrap directory have been consolidated into the canonical skills.
Keep architecture, requirements, development guidance and checklists in ignored
development/. These local working documents are not distributed by Git; use them
when present. In a fresh checkout, consult skills/pnm-viewer-development/SKILL.md
for the preserved original guidance before creating new working plans.

## Git workflow

- Use `tools/git/git-save.sh --commit-msg "..."` for local commits and
  `tools/git/git-push.sh` for pushes. Inspect the helpers before constructing commands.
- Supply copy/paste commands unless the user explicitly asks to execute a commit
  or push. Begin command blocks with `cd /home/dev01/Projects/PNM-Viewer-Toolkit &&`.
- Before writing a commit message, inspect `git status --short`, `git diff --stat`,
  `git diff`, staged changes, and every untracked file. The save helper stages all
  non-ignored changes, so the message must describe the complete scope.
- Use `Feature:`, `BugFix:`, or `Documentation:` followed by an imperative summary,
  then `- ` detail lines including validation. Keep each message line at most
  60 characters. Pass multiline messages as one double-quoted argument.
- Push requires a clean main/master branch and origin; never force-push.
- The push helper must run scripts/check-before-push.sh successfully and push
  only the tested commit. Do not bypass validation or add a skip option.
- See docs/development/git.md for commands and helper behavior.

## Versioning

- Use the `pnm-viewer-release` skill for version bumps and release work.
- Keep VERSION in MAJOR.MINOR.PATCH format; use tools/bump-version.sh.
- See docs/release/versioning.md. Version bumps do not authorize commits, tags, or pushes.

## Coding style

- Apply the coding-style skill for source, tests and scripts.
- Use the full multiline copyright/SPDX file banner from coding-style, not a
  standalone SPDX line. Preserve upstream notices and the component license.
- Project-owned development tooling uses Apache-2.0; desktop/UI uses GPL-3.0-only.

- Introduce readable domain types incrementally for meaningful fields/APIs. Use
  primitives underneath; aliases name concepts, newtypes enforce separation,
  and structs group related fields. Follow coding-style for placement and reuse.

- Use meaningful identifiers and lifetimes (for example `capture_bytes` and
  `'input`); reserve single letters for obvious, simple local counters.

## Local specification evidence

Keep field provenance, coverage, source hashes, manifests and private PDFs under
`development/specs/`. This directory is ignored and local-only. Tracked docs may
reference it but must state that evidence files are absent from fresh clones.
Use the versioned specification skills and parser tests to reconstruct evidence
when needed; do not assume those local records are distributed with the project.

## Testing

Apply `skills/pnm-testing/SKILL.md` for Rust parser and numeric work. Keep one
integration-test file per concrete parser and independent math/Q-format tests.

## Documentation layout

Use docs/README.md as the index. Group user setup under docs/install/, CLI usage
under docs/cli/, format contracts under docs/reference/, contributor and UI
plans under docs/development/, packaging/versioning under docs/release/, and
historical investigations under docs/history/. Keep root development/ ignored
for working plans. Update relative links whenever documents move.

Native CI/release ordering: docs/development/ci.md. Keep master and tag validation
shared; require Ubuntu 22.04/24.04/26.04, Windows x64 and macOS arm64/x64 checks.
Tag packaging targets Qt and includes the same-version CLI/manual. Preserve Tauri
regression checks. Native package lifecycle tests run on disposable CI runners;
local builds do not authorize system installation or publication.

Qt setup and validation: docs/development/qt.md. Run scripts/qt-desktop.sh check
for Qt changes and shared capture changes; preserve Tauri regression checks.
