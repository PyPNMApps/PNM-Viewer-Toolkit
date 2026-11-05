---
name: viewer-module-organization
description: Organize new implementations and moved files under apps/viewer/src by responsibility, updating Rust imports, module wiring and documentation. Applies to this desktop viewer crate, not a blanket restructuring rule for core parsers or Qt/QML.
---

# Viewer module organization

Keep `apps/viewer/src/lib.rs` as a small crate entry point. When adding a new
implementation, create a meaningfully named feature/responsibility subfolder,
or extend the existing folder that already owns the behavior. Do not add another
implementation file directly under `apps/viewer/src/`. Do not create empty
folders, a new crate or one wrapper module per helper merely to satisfy this rule.

Current owners:

- `capture/`: original bytes, bounded file I/O, diagnostics, shared capture types,
  exports and common chart/range presentation.
- `measurements/<measurement>/`: adapters for a particular measurement family,
  such as `channel_estimation/` or `modulation_profile/`.
- `updates/`: application update checks; `release.rs` owns GitHub release HTTP
  lookup and version interpretation, independently of capture state.

Use snake_case Rust module names and meaningful implementation filenames. A
feature's implementation may live in its `mod.rs`; split further when distinct
responsibilities justify it. Reserve `lib.rs` for wiring and deliberate public
exports. Existing root capture-type convenience exports may remain for callers,
but use the owning module path in new or modified code.

When moving a file, update `mod` declarations and every affected Rust import or
qualified path, including Qt adapters and integration tests. Update current
source references in docs and the local architecture; keep historical records
historical. Search the repository for obsolete paths after the move. Preserve
behavior, JSON contracts, licenses and original capture ownership. Do not widen
public visibility merely to make imports compile; use the narrowest visibility
needed for sibling adapters.

Run formatting, locked workspace tests and Clippy, plus Qt checks when its imports
or capture boundary change. Maintain the local architecture/checklist. Follow
AGENTS.md for canonical skill linkage and validation in both discovery directories.
