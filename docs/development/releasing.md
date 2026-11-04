# Prepare a version and release tag

[Developer index](README.md) · [Version rules](../release/versioning.md) ·
[Native packages](../release/packaging.md)

## Version ownership

Keep the tracked root `VERSION`. It contains one `MAJOR.MINOR.PATCH` value and
is authoritative for the CLI, Qt viewer and reusable crates.
Cargo manifests still require their own version fields; the helper
synchronizes those consumers and their lockfiles without updating dependencies.
The CLI version, Qt About dialog and package filenames use this shared version.

This follows OSCAR/OSCAR-UI's three-component versioning and immutable release
identity. In this Rust workspace a root file provides one source across
the core workspace and standalone Qt crate. Version preparation remains separate from commit,
tag and push so each operation can be reviewed.

## First tag

The initial version is `0.1.0`. For the first development release, keep that
version and tag the reviewed commit `v0.1.0`; no bump is needed just to create
the first tag. The project remains under development after tagging.

Before saving the release preparation:

1. Review [release notes](../release/release-notes.md) and current supported views.
2. Refresh [UI screenshots](screenshots.md), including dark/light Home and every
   supported measurement. Remove stale descriptions of retired controls.
3. Confirm all versions agree:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/release/versions.py --tag v0.1.0
```

Review and commit all preparation changes with the normal save helper. Then
push the clean master branch through the guarded helper; it runs the complete
local checks and refuses to push if they fail:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh
```

Review the resulting master CI run for all required operating systems before
creating the tag. Local Ubuntu checks alone do not verify Windows or macOS.
Check `git status --short` is empty and `git branch --show-current` is `master`.
Verify the intended tag does not already exist (both commands should print
nothing and succeed):

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
git tag --list v0.1.0 &&
git ls-remote --tags origin refs/tags/v0.1.0
```

Only after those checks, create the annotated tag and push its explicit ref:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/release/versions.py --tag v0.1.0 &&
git tag -a v0.1.0 -m "PNM Viewer Toolkit 0.1.0" &&
git push origin refs/tags/v0.1.0:refs/tags/v0.1.0
```

Never replace a tag or force-push it. If the push fails after tag creation,
inspect the local and remote refs before retrying that same explicit push;
do not bump again or recreate an existing tag.

The [tag workflow](ci.md#tags) validates the version and master ancestry,
repeats required CI, builds native packages, validates installed behavior and
creates a **draft** release only after the complete artifact set passes.
A pushed tag does not itself publish a release. Review artifacts and checksums
before any separate public publication.

## Later versions: automatic bumps

Choose the component explicitly; do not infer it from commit messages.

| Option | Example | Lower components |
| --- | --- | --- |
| `--bump patch` | `0.1.0` → `0.1.1` | Preserved |
| `--bump minor` | `0.1.7` → `0.2.0` | Patch reset |
| `--bump major` | `0.2.7` → `1.0.0` | Minor and patch reset |

Preview all affected paths, then apply the chosen bump:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/bump-version.sh --bump patch --dry-run
```

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/bump-version.sh --bump patch &&
python3 tools/release/versions.py
```

Use `python3 tools/release/bump-version.py --bump patch` directly on platforms
without Bash (`python` where that is the Python 3 launcher). The same major and
minor options apply. Both previews and writes validate all metadata inputs.
Preview changes no version metadata. Neither mode commits, tags, pushes or publishes.

Review the diff and release notes, commit, run the guarded push, review CI, and
use the new `vMAJOR.MINOR.PATCH` everywhere in the tag steps above. Do not edit
individual consumer versions by hand.
