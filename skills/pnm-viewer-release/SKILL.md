---
name: pnm-viewer-release
description: Prepare PNM Viewer three-component version bumps and release commands. Use for this repository's major, minor, patch, tag, and release workflow; not for other repositories.
---

# PNM Viewer release workflow

Read `docs/release/versioning.md`, `VERSION`, and the repository's helpers before preparing
commands. Resolve those paths from the repository root (two levels above this
canonical skill directory). VERSION is the canonical source. The bump helper synchronizes local Cargo
manifests/lockfiles, including the standalone Qt crate. Run
`python3 tools/release/versions.py` to reject drift; never override a version from
a tag or edit consumer manifests independently.

Use `tools/bump-version.sh --bump major|minor|patch`, with `--dry-run` to preview.
Require exactly MAJOR.MINOR.PATCH. Major resets minor and patch; minor resets
patch. Use tags `vMAJOR.MINOR.PATCH`. Do not copy PyPNM's fourth component.

Inspect tracked, staged, and untracked changes before drafting a save command;
the save helper stages all non-ignored files. Follow AGENTS.md message format.
Supply copy/paste commands beginning with `cd /home/dev01/Projects/PNM-Viewer-Toolkit &&`
unless explicitly asked to execute. A request to add tooling is not a request to
release. Combined --tag-patch, --tag-minor and --tag-major modes explicitly
automate versioning, checks, commit, annotated tag and atomic push when the user
runs them. Plain push stays branch-only; the standalone bump helper stays local.

Before a release tag, require clean master (the tag workflow checks origin/master), verify VERSION matches
the committed file, review validation evidence, and check local and remote tag
availability. Never replace existing tags. Prefer the combined release modes
with --dry-run for a preview. They validate the bumped candidate before committing
and atomically push exact objects; never use `git push --tags` or Git force-push.
For existing tags or failed uploads use git-push.sh --tag vMAJOR.MINOR.PATCH:
it checks VERSION/HEAD and revalidates without bumping again. If both origin refs
match it is a no-op. Preserve local candidate/commit/tag state after failures;
inspect it before retrying. See docs/development/releasing.md for recovery.
A bump at an already published current-version HEAD is a no-op by default.
The user may explicitly request `--force` with a bump mode for a rebuild/update
test without new development changes. This creates a fresh version/tag while
retaining clean-tree, validation, collision and normal atomic-push checks.
Do not supply it by default or use it to bypass unpublished release recovery.

The project is in the dev phase. A version or tag alone is not evidence of a
working application, tested platform, packaged artifact, or published release.

## Native CI and packages

Read `docs/development/ci.md` and `docs/release/packaging.md` for tag automation.
A pushed stable tag must match VERSION and belong to master. The tag workflow
calls the same ordered CI as master before packaging; all Ubuntu 22.04/24.04/26.04,
Windows x64 and macOS arm64/x64 checks are required. Do not skip a platform to
make a release pass. Missing, skipped and canceled jobs block the aggregate.

Build locally with `python3 scripts/release/build.py --output dist/release`
(`python` on Windows) only after native prerequisites are present. Preserve
existing outputs; the builder refuses reuse. Verify artifact versions, hashes
and the 50,000,000-byte budget with `scripts/release/verify.py`.

Installed lifecycle checks belong on disposable GitHub runners; do not set
GITHUB_ACTIONS locally to bypass their guard. Report local relocated-binary smoke
checks separately from native install/remove checks. Ubuntu installs the same
22.04-built DEBs on all three versions, and removing the UI must preserve the CLI.

User authorization on 2026-11-04 supersedes the earlier draft-only rule.
Only a complete validated artifact set may be uploaded to a temporary draft.
Verify uploaded sizes, GitHub DEB SHA-256 digests and re-downloaded bytes before
automatically publishing and marking latest. Never mutate a public release.
Preserve the temporary Ubuntu-only CI exception in AGENTS.md; a green result
there is not multi-OS validation. External signing/notarization remain separate.
A workflow file or local build is not proof of hosted platform success.
