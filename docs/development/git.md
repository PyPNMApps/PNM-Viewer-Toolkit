# Git workflow

The save helper and branch push gate follow the standard project workflow.
Release modes follow OSCAR-UI, adapted to this repository's VERSION and full
validation gate. Helpers resolve this checkout from their own location,
independently of the caller's working directory.

## Inspect changes

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
git status --short &&
git diff --stat &&
git diff &&
git diff --cached
```

Also inspect all untracked files before saving. `git-save.sh` stages all
non-ignored changes, including untracked files.

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

## Push a branch

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/git/git-push.sh
```

The push helper requires a clean working tree, main or master, and an existing
origin remote. It checks the remote branch and fetches it if present, rejects a
local branch behind or diverged from origin, and pushes with upstream tracking.
Before pushing, it runs `scripts/check-before-push.sh`: Rust formatting, Clippy,
locked workspace tests, release-helper and push-gate tests, mocked installer
tests on Linux, Bash/ShellCheck, a release CLI build/JSON smoke test, and Qt
adapter/build/QML checks.
Required missing tools or any failed check stop the push; there is no skip flag.

The gate starts with `scripts/sw-qa-hygiene.sh`. It checks the approved MAC
namespace in example captures, demo/archive members and linked JSON/hex outputs.
Run it independently before sharing data; see the [hygiene guide](../../tools/security/README.md)
for standalone sanitizing commands and multi-modem alias assignment.
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

## Bump, tag and push a release

Commit development work first, then use a combined release mode from clean master:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./tools/git/git-push.sh --tag-patch
```

Also available: `--tag-minor`, `--tag-major`, and release previews with `--dry-run`.
The helper bumps VERSION and Cargo metadata, validates the bumped candidate,
commits only version files, creates an annotated tag, then atomically pushes
master and that tag. It never overwrites tags or force-pushes.
Add `--force` to a bump mode to intentionally create a new version when HEAD is
already published and there are no new development changes. This still requires
a clean checkout and runs all checks; it is not a Git force-push option.

Use `--tag vMAJOR.MINOR.PATCH` to validate/push an existing matching tag at HEAD,
including retries after upload failure without another version bump. No release
mode publishes a public GitHub release directly. See [release preparation](releasing.md)
for first-tag instructions, candidate failures, no-op behavior and recovery.

## Help

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
tools/git/git-save.sh --help &&
tools/git/git-push.sh --help
```

For MAJOR.MINOR.PATCH bumps and release tags, see [release workflow](../release/versioning.md).
