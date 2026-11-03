# Permanent project guidance

Build PNM Viewer with Tauri 2, React/TypeScript presentation and Rust backend behavior.

- Work only in this viewer repository. PyPNM is read-only reference material. Do not change its files, configuration, dependencies or services, or run capture/startup code.
- No SNMP, TFTP, modem controls, capture initiation, Python sidecars, Node production runtime or remote analysis. Optional PyPNM HTTP retrieval runs in Rust and retrieves existing files only.
- React renders charts, navigation and transient interaction state. Rust owns filesystem access, parsing, scientific interpretation, analysis, aggregation, persistence, downloads, exports and decimation.
- Domain/parser libraries must not depend on Tauri, HTTP or app crates. Separate decoding from analysis, aggregation, storage and transport. Keep Cargo dependencies acyclic.
- Reusable libraries are Apache-2.0. Desktop shell and UI are GPL-3.0-only. Set package licenses explicitly; preserve third-party licenses, copyrights and applicable NOTICE content.
- Use separate project skills for DOCSIS 3.1 and 4.0 OSSI work. Require user-provided PDFs for normative claims. Do not substitute MULPI configuration TLVs, MIB objects or API JSON schemas for PNM binary layouts.
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

## Project status and guidance

The user adopted the bundle's Tauri/Rust stack on 2026-11-02, superseding the
initial Java plan. The project remains in the dev phase; no application exists.
Use development/requirements.md, development/architecture.md, development/checklist.md and
development/development-guide.md together. Maintain the docs as work lands.
The bootstrap bundle is preserved reference material, not generated app source.

Keep canonical skills under skills/ and relative links under both .agents/skills/
and .claude/skills/. Use docsis-31-ossi and docsis-40-ossi for field-evidence work;
existing versioned extraction skills are lookup aids, not proof of PDF validation.
Validate changed skills and discovery links. The ZIP retains the original bundle.
Keep architecture, requirements, development guidance and checklists in ignored
development/. These local working documents are not distributed by Git; use them
when present, and request the bundle if needed in a fresh checkout.

## Git workflow

- Use `tools/git/git-save.sh --commit-msg "..."` for local commits and
  `tools/git/git-push.sh` for pushes. Inspect the helpers before constructing commands.
- Supply copy/paste commands unless the user explicitly asks to execute a commit
  or push. Begin command blocks with `cd /home/dev01/Projects/pnm-viewer &&`.
- Before writing a commit message, inspect `git status --short`, `git diff --stat`,
  `git diff`, staged changes, and every untracked file. The save helper stages all
  non-ignored changes, so the message must describe the complete scope.
- Use `Feature:`, `BugFix:`, or `Documentation:` followed by an imperative summary,
  then `- ` detail lines including validation. Keep each message line at most
  60 characters. Pass multiline messages as one double-quoted argument.
- Push requires a clean main/master branch and origin; never force-push.
- See docs/git-workflow.md for commands and helper behavior.

## Versioning

- Use the `pnm-viewer-release` skill for version bumps and release work.
- Keep VERSION in MAJOR.MINOR.PATCH format; use tools/bump-version.sh.
- See docs/release.md. Version bumps do not authorize commits, tags, or pushes.
