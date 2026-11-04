# Continuous integration

[Documentation index](../README.md) · [Native packaging](../release/packaging.md)

## Master and pull requests

[CI](../../.github/workflows/ci.yml) runs on pushes to `master`/`main`, pull
requests, manual dispatch, and reusable calls from the tag workflow.

```text
CodeQL + code checks
         │ both pass
         ├── Rust tests on every OS
         ├── CLI release build + JSON smoke on every OS
         └── Qt + retained Tauri checks on every OS
                          │ all pass
                    Required checks
```

CodeQL scans Rust, TypeScript/JavaScript, Python tooling and GitHub Actions. Code checks run
Rustfmt for all three Cargo workspaces, core Clippy with warnings denied,
version consistency, Bash syntax and ShellCheck. Completing
CodeQL analysis uploads findings to code scanning; it does not by itself mean
there are no alerts. Configure repository code-scanning rules to block findings
at the desired severity.

All native matrices require:

| Runner | Architecture |
| --- | --- |
| Ubuntu 22.04 | x64 |
| Ubuntu 24.04 | x64 |
| Ubuntu 26.04 | x64 |
| Windows Server 2022 | x64 |
| macOS 15 | Apple Silicon |
| macOS 15 Intel | x64 |

Each OS runs all workspace parser/math/header/CLI tests, version/packaging-tool
unit tests, a release CLI build and RxMER JSON smoke test. All three Ubuntu
versions also run mocked installer and push-tool tests. Desktop checks run the retained Tauri typecheck, lint,
frontend tests/build and Rust adapter checks, followed by Qt Rust tests,
Clippy/build and QML presentation tests. Every matrix uses `fail-fast: false`
so one failure does not hide results from other platforms.

`tests.yml`, `cli-build.yml` and `desktop.yml` are reusable children; they have no
independent push triggers. Their jobs start only after CodeQL and code checks
pass. A failed, canceled or skipped stage makes **Required checks** fail.

In GitHub repository settings, protect `master` with the **Required checks**
status and require the branch to be current before merging. Workflow files
provide the check; repository branch protection must be configured separately.
Direct pushes can trigger CI after upload; the local
[push gate](git.md) checks the current host before upload.

## Tags

[Native packages](../../.github/workflows/release.yml) runs only for pushed `v*`
tags. It first requires exact `vMAJOR.MINOR.PATCH` agreement with VERSION and every
manifest/lockfile, plus tag-commit ancestry in `origin/master`. It calls the same
CI workflow on that exact tagged commit, then builds packages on native runners.

The Ubuntu 22.04-built DEBs are downloaded unchanged and installed/tested on
**22.04, 24.04 and 26.04**. Windows and each macOS architecture test their own
native installers/images. Every test checks package hashes, GUI open/render/JSON
export for all six current measurements, CLI operation, manual installation and
removal. Linux also verifies that removing the viewer preserves the CLI package.

Only after every lifecycle job succeeds does one job assemble the complete
artifact set and attach it to a **draft** GitHub release. It refuses to modify an
already public release. There is no automatic public publication, external
signing or notarization. Failed packages, missing targets and packages above
50,000,000 bytes block the draft. See [packaging](../release/packaging.md).

## Evidence and badges

README CI and package badges show actual workflow state. Platform badges describe
the required matrix, not passing results. A configured runner is not evidence of
a successful hosted run; inspect the first run after pushing these changes.
Native Windows/macOS and Ubuntu 24.04/26.04 results remain pending until hosted CI.

The local pre-push gate checks versions, tooling, Rust, Tauri and Linux Qt without
installing dependencies. Native package installation is restricted to disposable
GitHub runners. Local package builds do not install or publish anything.
