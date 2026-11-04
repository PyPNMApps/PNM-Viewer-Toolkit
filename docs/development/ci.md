# Continuous integration

[Documentation index](../README.md) · [Native packaging](../release/packaging.md)

## Master and pull requests

[CI](../../.github/workflows/ci.yml) runs on pushes to `master`/`main`, pull
requests, manual dispatch, and reusable calls from the tag workflow.

```text
CodeQL + code checks
         │ both pass
         └── One native job on each OS:
               Rust tests + tooling tests
               CLI release build + JSON smoke
               Qt tests + Clippy + build + QML tests
                          │ all pass
                    Required checks
```

CodeQL scans Rust, Python tooling and GitHub Actions. Code checks run
Rustfmt for both Cargo workspaces, core Clippy with warnings denied,
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
versions also run mocked installer and push-tool tests. Desktop checks run Qt Rust tests,
Clippy/build and QML presentation tests. Every matrix uses `fail-fast: false`
so one failure does not hide results from other platforms.

`native-checks.yml` is a reusable child with no independent push trigger.
Its six jobs start only after CodeQL and code checks pass. Tests, release CLI
build/smoke and desktop checks share one runner per platform. This replaces the
three separate six-platform matrices, reducing hosted runner allocations from
18 to six while retaining every check on every platform. A failed, canceled or skipped stage makes **Required checks** fail.

In GitHub repository settings, protect `master` with the **Required checks**
status and require the branch to be current before merging. Workflow files
provide the check; repository branch protection must be configured separately.
Direct pushes can trigger CI after upload; the local
[push gate](git.md) checks the current host before upload.

## Windows linker selection

After initializing x64 MSVC, both native validation and packaging run
`python scripts/qt-sdk.py export-msvc-linker` before Rust cache/toolchain or
build steps. This validates the existing compiler tools and writes Cargo's
absolute MSVC linker setting to `GITHUB_ENV`; it needs no Qt installation.
Subsequent Git Bash steps retain that setting even when Bash prepends Git's
own `usr/bin/link.exe` to PATH. Explicit Cargo linker overrides are preserved.

In [the v0.1.3 run](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41923732617/job/126905079546),
Windows workspace tests selected Git's linker and failed with
`/usr/bin/link: extra operand`. The Qt child-process protection did not cover
these earlier workspace commands after matrix consolidation. The job-wide export
extends the same selection logic to them; it does not remove any tests. Local
synthetic-tool tests cover the export, but hosted Windows validation is still
required. Re-running the old tag uses its original workflow, without this fix.

## Caching and runtime

Native validation and package builds cache the pinned Qt SDK under `local/qt`.
Keys include the exact runner image label, architecture and SDK setup-script
hash. Setup still checks the SDK version and presentation-test executable on
cache hits; a hit never skips the build or tests. The Python installer virtual
environment is not cached.

[Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) caches Cargo downloads
and compiled dependencies for both workspaces. The additional key isolates
runner image labels and Qt setup revisions; the action also keys by Rust host,
compiler and Cargo manifests/lockfiles. Workspace application crates remain
subject to normal Cargo rebuilds, and installed tools under Cargo's bin directory
are excluded. Validation and packaging use separate Rust cache scopes so packaging can save
its own release-profile dependencies instead of competing with an immutable
validation cache. The Qt SDK cache is shared. Release binaries and all tests
still run for the current commit.

Cold runs and cache evictions still require full downloads/builds. CodeQL remains
before the matrix. No fixed runtime is promised: GitHub runner queue time and
macOS availability are outside this repository's control. The Required checks
job continues to reject failures, skipped jobs and cancellations.

The [v0.1.1 run](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41914986990)
failed because its macOS ARM desktop job never acquired a hosted runner. GitHub
reported capacity constraints. The completed matrix jobs passed. The Intel macOS
Qt job spent 4m10s in SDK setup and 16m29s in checks; CodeQL took 4m34s before the
matrix. Those are observed uncached timings, not estimates for future runs.
Re-running failed jobs on that old tag can recover from transient runner
capacity, but it does not pick up workflow changes committed later. Do not move
an existing release tag to apply these optimizations.

## Tags

[Native packages](../../.github/workflows/release.yml) runs only for pushed `v*`
tags. It first requires exact `vMAJOR.MINOR.PATCH` agreement with VERSION and every
manifest/lockfile, plus tag-commit ancestry in `origin/master`. It calls the same
CI workflow on that exact tagged commit, then builds packages on native runners.

The Ubuntu 22.04-built DEBs are downloaded unchanged and installed/tested on
**22.04, 24.04 and 26.04**. Windows and each macOS architecture test their own
native installers/images. Every test checks package hashes, GUI open/render/JSON
export for all current measurements, CLI operation, manual installation and
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
The linked run completed native build/test checks on all three Ubuntu versions,
Windows x64 and macOS Intel. Its macOS ARM job never acquired a runner, and
package/lifecycle jobs did not run. This revised matrix and its cache timings
still require validation in a new hosted run.

The local pre-push gate checks versions, tooling, Rust and Linux Qt without
installing dependencies. Native package installation is restricted to disposable
GitHub runners. Local package builds do not install or publish anything.
