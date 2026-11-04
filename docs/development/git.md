# Git workflow

The project uses the standard helpers copied from
`build-local-mxl-build-server/tools/git/`. Both locate this repository relative
to their script location, independently of the caller's working directory.

## Inspect changes

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
git status --short &&
git diff --stat &&
git diff &&
git diff --cached
```

Also inspect all untracked files before saving. `git-save.sh` stages all
non-ignored changes, including untracked files such as the original archive.

## Save a local commit

Replace the example message with one describing the full change scope.

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/git/git-save.sh --commit-msg "Feature: Add project tooling
- Add standard Git save and push helpers
- Tests: Run Bash syntax and helper checks"
```

The helper requires a message, rejects nested Git metadata under skills, stages
all changes, and creates a local commit. It does not push. No staged changes
is reported as an error. Existing Git author configuration is used unchanged.

## Push

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/git/git-push.sh
```

The push helper requires a clean working tree, main or master, and an existing
origin remote. It checks the remote branch and fetches it if present, rejects a
local branch behind or diverged from origin, and pushes with upstream tracking.
Before pushing, it runs `scripts/check-before-push.sh`: Rust formatting, Clippy,
locked workspace tests, release-helper and push-gate tests, mocked installer
tests on Linux, Bash/ShellCheck, and a release CLI build and JSON smoke test.
Required missing tools or any failed check stop the push; there is no skip flag.
It uses the local Rust toolchain when installed, otherwise standard Cargo.

The helper verifies branch, HEAD and the clean working tree again after checks,
then pushes the exact tested commit. It can create a missing remote branch and
never force-pushes or creates commits. A remote lookup failure stops immediately.

Run the checks without pushing:

```bash
bash scripts/check-before-push.sh
```

These checks validate the current host. The GitHub workflows cover native
Windows/Linux/macOS tests and builds. Direct `git push` can bypass this helper;
to enforce checks for everyone, configure GitHub rules for main/master requiring
pull requests and the CI checks before merging. Those server-side rules have not
been configured by this change.

## Help

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/git/git-save.sh --help &&
tools/git/git-push.sh --help
```

For MAJOR.MINOR.PATCH bumps and release tags, see [release workflow](../release/versioning.md).
