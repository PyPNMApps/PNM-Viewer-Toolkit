# Three-component versions

PNM Viewer uses `MAJOR.MINOR.PATCH`, initially `0.1.0`, in the root `VERSION`
file. The project is in the dev phase; this is not a published release.
Python 3 is needed for the development helper, not the planned Rust/Tauri runtime.

| Bump | Example | Intended use |
| --- | --- | --- |
| patch | 0.1.0 -> 0.1.1 | Compatible fixes |
| minor | 0.1.5 -> 0.2.0 | New development features |
| major | 0.2.5 -> 1.0.0 | Deliberate major compatibility or maturity milestone |

## Preview and update

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/bump-version.sh --bump patch --dry-run
```

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/bump-version.sh --bump patch
```

Replace `patch` with `minor` or `major` as appropriate. The helper only writes
VERSION. It does not stage, commit, tag, push, or publish. It rejects malformed
versions, including four components and leading zeros. Paths resolve from the
helper, so invoking it from another directory still targets this repository.

## Commit and release

Review all changes and use the [Git workflow](../development/git.md) to save and push.
Commit messages must cover everything staged, not just the version change.
A release tag uses `vMAJOR.MINOR.PATCH` at the reviewed version commit.
Before creating it, verify a clean main/master branch, that committed VERSION
matches the intended tag, that the tag is absent locally and remotely, and that
relevant checks pass. Push the branch through `tools/git/git-push.sh`, then push
only the intended tag with an explicit tag ref. Never overwrite tags.

Tag creation and publication are explicit release actions; the bump helper does
not perform them. The project skill `pnm-viewer-release` guides this workflow.
There is no application packaging or hosted-release automation yet. When the Cargo/Tauri/UI build is added, consume VERSION or add tested
synchronization across package manifests before release.

## Tool validation

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 -m unittest discover -s tools/release -p 'test_*.py'
```
