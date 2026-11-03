---
name: pnm-viewer-release
description: Prepare PNM Viewer three-component version bumps and release commands. Use for this repository's major, minor, patch, tag, and release workflow; not for other repositories.
---

# PNM Viewer release workflow

Read `docs/release.md`, `VERSION`, and the repository's helpers before preparing
commands. Resolve those paths from the repository root (two levels above this
canonical skill directory). VERSION is the only version source until Cargo/Tauri/UI manifests exist; integrate the future build before claiming artifact versions
are synchronized.

Use `tools/bump-version.sh --bump major|minor|patch`, with `--dry-run` to preview.
Require exactly MAJOR.MINOR.PATCH. Major resets minor and patch; minor resets
patch. Use tags `vMAJOR.MINOR.PATCH`. Do not copy PyPNM's fourth component.

Inspect tracked, staged, and untracked changes before drafting a save command;
the save helper stages all non-ignored files. Follow AGENTS.md message format.
Supply copy/paste commands beginning with `cd /home/dev01/Projects/pnm-viewer &&`
unless explicitly asked to execute. A request to add tooling is not a request to
release. Keep version preparation, commit, tag, and push explicit.

Before a release tag, require a clean main/master branch, verify VERSION matches
the committed file, review validation evidence, and check local and remote tag
availability. Never replace existing tags. Use the guarded push helper for the
branch and an explicit tag ref for the tag, never `git push --tags` or force.
If publication partly fails, inspect local and remote refs before retrying;
do not bump again automatically or delete published refs.

The project is in the dev phase. A version or tag alone is not evidence of a
working application, tested platform, packaged artifact, or published release.
