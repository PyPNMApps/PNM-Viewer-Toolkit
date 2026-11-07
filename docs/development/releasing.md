# Prepare a version and release tag

[Developer index](README.md) · [Version rules](../release/versioning.md) ·
[Native packages](../release/packaging.md)

## Temporary tag delivery

Tag push commands are unchanged. They currently trigger only the
[Ubuntu 22.04 development workflow](../../.github/workflows/ubuntu-deb-temp.yml)
and produce downloadable Actions artifacts after Ubuntu checks/lifecycle tests.
After verification, a final job uploads assets to a temporary draft, verifies
remote metadata and downloaded bytes, then automatically publishes the Ubuntu-only
release with install/upgrade notes. See
[DEB installation and upgrade testing](../release/ubuntu-deb-testing.md).
The preserved full package workflow is manual-only against a selected tag;
its complete multi-OS gates remain mandatory if explicitly run.

## Version ownership

The tracked root `VERSION` is authoritative for the CLI, Qt viewer and reusable
crates. Cargo manifests and lockfiles require version copies; the shared version
helper synchronizes them without updating dependencies. The CLI version, Qt
About dialog and package filenames use this version.

## Combined release commands

Like OSCAR-UI, the push helper supports explicit release modes:

| Mode | Example from `0.1.7` |
| --- | --- |
| `--tag-patch` | `0.1.8` |
| `--tag-minor` | `0.2.0` |
| `--tag-major` | `1.0.0` |

Push implementation changes to the branch first and wait for **Required checks**,
including desktop preflight, before creating a release tag. See the
[CI reliability audit](ci.md#recurring-failure-audit). Combined tag-push modes do
not wait for a separate branch CI run.

Commit development changes first. Release modes require a clean **master** branch,
consistent version metadata and an origin that is not ahead/diverged. They read
local/remote tags and reject collisions before changing files.

Preview a release without running builds or changing versions, commits, tags or
remote refs (origin is read and Git fetch metadata may be updated):

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag-patch --dry-run
```

Create and push a patch release:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag-patch
```

Use `--tag-minor` or `--tag-major` for the other components. The command:

1. Bumps `VERSION` and synchronizes all project-owned Cargo versions.
2. Runs the full local pre-push gate on the **bumped candidate**.
3. Rejects changes to the branch, commit or candidate during checks.
4. Commits only version files and creates an annotated `vMAJOR.MINOR.PATCH` tag.
5. Atomically pushes the exact validated commit and tag to origin.

No force-push, extra tags, automatic public publication or skip-check option is
provided. If HEAD is already published as the current version tag, a repeat bump
is a no-op by default. For an intentional rebuild/update test without a new
application change, pass `--force` with a bump mode:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag-patch --force
```

This creates a new version and tag from an already published clean HEAD. It
still requires clean master and consistent committed metadata, runs every check,
and refuses existing target tags. It does not force-push or skip validation.
Use `--dry-run` with the same options to preview. `--force` is invalid with
branch-only pushes or `--tag` recovery; an unpublished local release still
requires recovery with `--tag` rather than another bump.
Plain `git-push.sh` (also `--master` or `--branch`) still pushes only the current
main/master branch; it does not bump, commit or tag.

Before release preparation, review [release notes](../release/release-notes.md),
refresh [UI screenshots](screenshots.md), and update user guides for changed
controls or supported measurements. Local Ubuntu validation does not prove
Windows/macOS runtime behavior.

## First tag or an already prepared version

For the first `0.1.0` development tag, no bump is needed. After committing the
reviewed preparation, confirm `v0.1.0` is unused locally and remotely:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/release/versions.py --tag v0.1.0 &&
git tag --list v0.1.0 &&
git ls-remote --tags origin refs/tags/v0.1.0
```

Both tag listings should be empty. Then create the local annotation and use the
validated existing-tag mode to push master and that tag together:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
git tag -a v0.1.0 -m "PNM Viewer Toolkit 0.1.0" &&
./tools/git/git-push.sh --tag v0.1.0
```

`--tag` requires an existing local tag at HEAD matching committed VERSION. It
never bumps, creates a commit or creates/replaces a tag. A remote annotation
mismatch is rejected even if the target commit is identical. If both refs already
match origin, the command returns without checks or another upload.

## Failure and recovery

Validation or commit failures retain local version edits for review. Fix the
failure, review the candidate, run the gate, commit the prepared version, and
create its matching annotation before using `--tag`. Do not bump again merely
because the earlier operation stopped. Nothing resets or discards work.

If upload fails after the release commit/tag exist, atomic push prevents uploading
only one ref. The helper prints a recovery command. Inspect local and remote refs,
then retry that same version, for example:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag v0.1.1
```

This revalidates the prepared release without another bump or commit. If only
its commit exists because tag creation failed, inspect it and create the matching
annotation first. Never overwrite published tags or force-push history.

## Prepare versions without publishing

The independent version helper remains available:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/bump-version.sh --bump patch --dry-run
```

Remove `--dry-run` to apply. Use `python3 tools/release/bump-version.py --bump patch`
without Bash (`python` where that is the Python 3 launcher). This helper never
commits, tags or pushes. Do not manually bump before using a combined bump mode;
use the already-prepared-version procedure instead.

The [tag workflow](ci.md#tags) validates version/master ancestry, repeats all
required OS checks, builds native packages and tests installed behavior. Only a
complete passing artifact set is marked stable/latest. If a platform fails,
successful platforms can publish a clearly labeled partial prerelease after
their own lifecycle checks; the workflow remains failed and the previous stable
release remains latest. Every uploaded asset is checked remotely before
publication. See [CI partial-release policy](ci.md#partial-releases-after-a-platform-failure).
The project remains in development.


## Release page and automatic publication

Maintain [the installation-focused release template](../release/release-notes.md)
with package/runtime changes. It supplies package selection, per-OS install,
update/removal steps, CLI-only options, first use and checksum instructions.

The final workflow job renders version-specific download links from the verified
collected release index, with source-guide links pinned to the tag. Review the
rendered page and attached assets in the workflow. The final job verifies remote
asset metadata and downloads the uploaded bytes before publishing automatically.
See [release-page generation](../release/packaging.md#release-page) for a local
preview command. Retrying a draft refreshes its instructions; the workflow
refuses to edit a public release.
