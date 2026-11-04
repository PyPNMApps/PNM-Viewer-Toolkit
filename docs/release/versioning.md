# Three-component versions

PNM Viewer uses `MAJOR.MINOR.PATCH`, initially `0.1.0`, in the root `VERSION`
file. The project is in the dev phase; this is not a published release.
Python 3 is needed for the development helper, not the Rust CLI or desktop runtime.

For the first `v0.1.0` tag, subsequent bumps and the ordered release checklist,
see [release preparation](../development/releasing.md). Keep the root `VERSION`;
it is the canonical version source across the CLI, Qt and reusable Rust crates.

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

Replace `patch` with `minor` or `major` as appropriate. The helper updates VERSION and all local Cargo package manifests and lockfiles
together, including the standalone Qt crate. It does not stage, commit, tag, push, or publish. It rejects malformed
versions, including four components and leading zeros. Paths resolve from the
helper, so invoking it from another directory still targets this repository.
The preview validates all metadata inputs and lists every version file it would
change without writing version metadata. The same Python entry point works
without Bash: `python3 tools/release/bump-version.py --bump patch --dry-run`.

## Commit and release

For the combined OSCAR-UI-style workflow, commit development changes and run:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag-patch
```

Use `--tag-minor` or `--tag-major` for other components; add `--dry-run` to preview.
These explicit release modes bump, validate the exact candidate, commit version
files, create an annotated tag and atomically push master and that tag. They
require clean master and consistent metadata. Plain push remains branch-only.
The standalone bump helper above continues to make version edits only.

Do not manually bump before a combined bump mode. For an already prepared
version or a failed upload, use `--tag vMAJOR.MINOR.PATCH` with its matching
existing tag at HEAD. See [release preparation](../development/releasing.md)
for first-tag instructions, no-op behavior and recovery without a second bump.
Never overwrite tags. Temporary tag delivery runs Ubuntu 22.04 checks and uploads test-package
artifacts, without creating a GitHub release. The full multi-OS workflow is
manual-only; see [DEB testing](ubuntu-deb-testing.md).

Run `python3 tools/release/versions.py` to detect version drift; do not edit
individual application versions independently.

## Tool validation

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 -m unittest discover -s tools/release -p 'test_*.py' &&
python3 -m unittest discover -s tools/git/tests -p 'test_*.py'
```
