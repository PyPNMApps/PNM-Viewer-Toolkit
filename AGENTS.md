# Permanent project guidance

Build PNM Viewer Toolkit with a Rust backend and native Qt Quick/QML desktop
under apps/qt/. Qt is the sole desktop UI; do not reintroduce a web frontend.

- Work only in this viewer repository. PyPNM is read-only reference material. Do not change its files, configuration, dependencies or services, or run capture/startup code.
- No SNMP, TFTP, modem controls, capture initiation, Python sidecars, Node production runtime or remote analysis. Optional PyPNM HTTP retrieval runs in Rust and retrieves existing files only.
- QML renders charts, navigation and transient interaction state. Rust owns filesystem access, parsing, scientific interpretation, analysis, aggregation, persistence, downloads, exports and decimation.
- Domain/parser libraries must not depend on Qt, HTTP or app crates. Separate decoding from analysis, aggregation, storage and transport. Keep Cargo dependencies acyclic.
- Reusable libraries are Apache-2.0. Desktop shell and UI are GPL-3.0-only. Set package licenses explicitly; preserve third-party licenses, copyrights and applicable NOTICE content.
- Use separate project skills for DOCSIS 3.1 and 4.0 OSSI work. The user authorized clear versioned text extracts for RxMER; request PDFs for ambiguous layouts and never claim rendered-PDF verification. Do not substitute MULPI configuration TLVs, MIB objects or API JSON schemas for PNM binary layouts.
- Never invent file identifiers, offsets, widths, byte order, scaling, sentinels or version equivalence. Trace implemented fields to pinned PDF evidence or explicitly labeled implementation evidence. Protocol version and file-format version are separate.
- Missing evidence blocks affected semantic decoders, not the generic bootstrap. Unknown files remain viewable as bytes with structured diagnostics. Do not label unsupported formats complete.
- Bound external-input allocations, arithmetic, archive expansion and work. Return structured errors with offsets; preserve original capture bytes and never overwrite inputs.
- Keep large datasets in Rust behind opaque sessions. Use bounded typed IPC queries, cancellation, progress and cleanup; do not serialize whole captures for a viewport.
- Aggregate compatible device/channel/type/unit/format and frequency grids only. Preserve invalid samples and gaps. Document and test any explicit alignment transformation.
- Offline viewing must work without a server or network. Feature-gate the HTTP adapter and test offline-only and retrieval-enabled builds.
- Pin toolchains and lockfiles. Run Rust formatting, Clippy and tests; Qt adapter and QML tests and native builds. Verify IPC contracts and installed application behavior.
- CI builds Windows, macOS Intel/Apple Silicon and Ubuntu packages on suitable native runners. Report unverified targets honestly; a successful cross-compile is not runtime validation.
- Each primary architecture-specific distributable must be at most 50,000,000 bytes, counting bundled prerequisites and assets. Fail the release gate when exceeded. Report installer bytes, installed footprint and external prerequisites separately.
- Offline viewing after setup and offline installation on a pristine OS are different promises. Do not claim both fit the budget without measurement.
- Produce local artifacts and checksums. Publishing, external signing/notarization and system-wide installation require user authorization. Keep credentials, private PDFs and customer captures out of Git and release bundles.

## Project naming

Display name: PNM Viewer Toolkit. Repository/directory: `PNM-Viewer-Toolkit`.
Standalone CLI: `pnm-decode` (`pnm-decode.exe` on Windows); manual: `pnm-decode.1`.
Existing Cargo library names and copyright notices retain their current identities.

## Project status and guidance

The user selected Qt as the sole desktop UI on 2026-11-04, superseding the
original bootstrap stack and the parallel-prototype period.
Shared GPL desktop capture behavior lives under apps/viewer; Qt uses CXX-Qt at
the adapter boundary only. Core parsers remain safe Rust.
The project remains in the dev phase. Qt supports the measurement views listed
in docs/ui/measurements.md. Native packaging is configured; hosted platform
validation and release publication require their own evidence.
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
- Branch push requires clean main/master and origin; never force-push.
- Explicit --tag-patch, --tag-minor and --tag-major push modes require master,
  bump VERSION/Cargo metadata, validate the candidate, commit, annotate and
  atomically push master and that tag. --tag TAG retries an existing matching
  tag at HEAD without bumping. Supply commands unless asked to execute.
- The push helper must run scripts/check-before-push.sh successfully and push
  only tested commits/tags. Do not bypass validation or add a skip option.
- Explicit `--force` with a version-bump mode permits a fresh release from an
  already published clean HEAD; retain every validation and normal push guard.
  It is not Git force-push and cannot bypass pending local release recovery.
- See docs/development/git.md for commands and helper behavior.

## Versioning

- Use the `pnm-viewer-release` skill for version bumps and release work.
- Keep VERSION in MAJOR.MINOR.PATCH format; use tools/bump-version.sh.
- See docs/release/versioning.md. Version bumps do not authorize commits, tags, or pushes.

## Viewer source organization

Apply skills/viewer-module-organization/SKILL.md when adding or moving
implementations under apps/viewer/src. Group them in relevant responsibility
or feature folders; keep lib.rs focused on module wiring. Update affected
imports, tests and current source references when moving files.

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
Tag packaging targets Qt and includes the same-version CLI/manual. Native package lifecycle tests run on disposable CI runners;
local builds do not authorize system installation or publication.

Qt setup and validation: docs/development/qt.md. Run scripts/qt-desktop.sh check
for Qt changes and capture-layer changes.

## Multi-OS release delivery

User instruction on 2026-11-04 restores automatic multi-OS tag releases, superseding
the earlier Ubuntu-only pause. release.yml requires CodeQL/static checks, all six
native test jobs, MSI/DMG packaging and installed lifecycle checks, baseline DEBs
on Ubuntu 22.04/24.04/26.04, complete assets, budgets and verified publication.
Tests and packaging share native hosts to avoid a second cold compilation matrix.
The old Ubuntu-only workflow is manual troubleshooting only; never trigger both
for an automatic tag. Master/PR runs CodeQL/static checks plus lightweight
Windows/Mac packaging and Mac Cocoa QML preflight without Rust compilation.
Full native checks remain available manually. No failing native tests may be skipped to make CI pass.
User authorization on 2026-11-05 permits a clearly labeled partial prerelease
from successful same-run platforms when another platform fails. Global identity,
CodeQL and code checks must pass. Each published platform must pass its native
and installed lifecycle checks; Linux requires all three Ubuntu versions. Keep
the workflow failed, omit failed targets and never mark a partial release latest.
Complete stable publication still requires the full matrix. Public assets remain
immutable; missing platforms require a new tag rather than modifying a release.
Windows x64 uses MSI; macOS arm64/x64 use DMGs. Windows/macOS in-app install and
restart are deferred by explicit user choice; Ubuntu keeps automatic restart.
Follow OSCAR-UI's Windows security guidance without disabling OS protections,
execution-policy bypasses, blanket unblock or elevated application launches.
Native hosted results and Windows 11/macOS interactive approvals require evidence.
Never modify an already public release. Signing/notarization remain separate.

## Build disk maintenance

Use tools/maintenance/rust_clean.py as the single Rust cleanup implementation.
Build helpers call its 95% filesystem-usage check before compilation. It calls
cargo clean for marked checkout target directories only; preserve SDKs, Cargo
downloads, toolchains, custom/external target directories and other repositories.
Reject redirects/tracked files before cleanup and stop if disk usage stays high.
See docs/development/maintenance.md for manual and dry-run commands. Never clean
active outputs by deleting directories directly; let Cargo own build locking.

## Fixture MAC hygiene

Imported single-fixture CM and CMTS MAC fields use 00ca.0000.0001, serialized
as 00:ca:00:00:00:01. Collections retain distinct sequential modem aliases. Run the read-only
`python3 tools/security/pnm-mac-updater.py` check before staging. Use its explicit
--write mode for fixture identity changes, preserve original/current provenance
hashes, and regenerate JSON examples and affected screenshots. See
`tools/security/README.md`. Do not rewrite arbitrary binary/text files or alter
upstream PyPNM. This policy concerns development fixtures, not user captures.

## Installed branding and examples

Desktop installers include nine single captures, generated RxMER collection ZIPs
and the sanitized pre-EQ comparison pair, with applicable license/NOTICE and
provenance. Expanded sources are registered in tests/fixtures/demos/installer-demos.json.
Install demos under files/demo beside Windows/Linux executables and under
Contents/Resources/files/demo on macOS. Initialize all capture file/folder/archive
pickers from that demo root; exports use a user-selected writable destination.
Use scripts/release/samples.py to check hygiene before creating ZIPs; never use
prebuilt demo archives as installer sources. Apply skills/sw-qa-hygiene/SKILL.md; do not ship Python tests,
private evidence or arbitrary customer captures. Standalone CLI archives stay
capture-free. Installed Open starts in the examples folder and exports target
user-selected writable locations. Test installed sample bytes and default folder.

Use packaging/ native icons and navy/teal WiX artwork derived from the existing
desktop PNG. Windows must offer an unchecked desktop-shortcut choice before
installation; macOS uses its bundle icon and standard user-created Finder alias.
Keep tools/branding/generate.py reproducible and development-only. Add new host
build/conversion prerequisites to --development and its docs; do not install
packages merely to validate script syntax or argument handling.

## Example and demo hygiene

Apply skills/sw-qa-hygiene/SKILL.md for capture examples and demo collections.
Use the standalone MAC sanitizer and retain distinct modem identities.
The read-only SW-QA hygiene gate is required before pushes and in CI.
