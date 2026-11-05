# Continuous integration

[Documentation index](../README.md) · [Native packaging](../release/packaging.md)

## Temporary Ubuntu-only delivery

By user request, automatic multi-OS runs are paused while Ubuntu DEBs are tested.
[Ubuntu 22.04 DEB (temporary)](../../.github/workflows/ubuntu-deb-temp.yml) runs
only on pushed `v*` tags. It validates VERSION and master ancestry, requires
CodeQL/static checks, runs the complete local gate on Ubuntu 22.04, builds both
viewer/CLI DEBs with existing size/checksum gates, and tests installed packages
on a disposable Ubuntu 22.04 host. Successful packages are retained as workflow
artifacts for 30 days. A final job re-verifies the packages and attaches them to
an explicitly Ubuntu-only release. It verifies uploaded sizes, GitHub DEB
digests and downloaded bytes before automatically publishing. It refuses to
change an already public release. Diagnostics last 14 days.
See [artifact installation and upgrade testing](../release/ubuntu-deb-testing.md).

Master/PR CI still runs CodeQL and static checks automatically. The full native
matrix requires `run_native_checks: true`: this is the default for manual CI,
but false for reusable calls and absent on ordinary pushes/PRs. Required checks
explicitly expect the native job to be skipped in this temporary automatic mode;
failed/canceled code checks still fail. In full mode, every native result must
succeed. A green automatic CI badge therefore does **not** certify all OSes.

The original full package workflow is preserved with a manual trigger only.
Run it against an existing matching tag to explicitly request full validation;
it passes `run_native_checks: true` and retains every original packaging/lifecycle
and publication gate. The temporary workflow never invokes it.

The [investigated run](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41934494719)
passed all three Ubuntu native jobs. Windows failed the pre-EQ label-width QML
test; both macOS targets failed chart interaction/rendering tests. Those failures
remain unresolved and are not relabeled as passing. Restore normal scheduling
only after the platform issues are addressed: restore the full release tag
trigger, re-enable automatic native CI, and remove the temporary workflow.

## Preserved full CI mode

[CI](../../.github/workflows/ci.yml) runs on pushes to `master`/`main`, pull
requests, manual dispatch, and reusable calls. Automatic runs currently use
the reduced scope above; the diagram below describes explicit full mode.

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

When full native mode is explicitly enabled, its matrix requires:

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
18 to six while retaining every check on every platform. In full mode, a failed,
canceled or skipped stage makes **Required checks** fail. The temporary automatic
mode accepts only the deliberately skipped native job; CodeQL/quality must pass.

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

## QML failure diagnostics

Native Qt checks run QML before native compilation and preserve explicit text,
JUnit XML and process logs under `local/qt-test-results/`. CI uploads those files
as `qml-tests-<runner>` artifacts even when the check fails. They are retained
for 14 days. See [isolated checks](qt.md#qml-reports-and-isolated-checks).

In [the v0.1.4 Windows job](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41926580364/job/126914502859),
workspace tests, the CLI build/smoke, Qt Rust tests, Clippy and native build all
passed. `qmltestrunner` then exited with status 1 without visible assertions.
That log establishes the failing stage, not the underlying test/startup cause.
Explicit reports address the missing diagnostic evidence; a new Windows run is
required to identify and resolve the original failure. Re-running the old tag
will not include the new reporting behavior.

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
macOS availability are outside this repository's control. Full-mode Required checks continue to reject native failures, skips and
cancellations; temporary automatic mode follows the explicit scope above.

The [v0.1.1 run](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41914986990)
failed because its macOS ARM desktop job never acquired a hosted runner. GitHub
reported capacity constraints. The completed matrix jobs passed. The Intel macOS
Qt job spent 4m10s in SDK setup and 16m29s in checks; CodeQL took 4m34s before the
matrix. Those are observed uncached timings, not estimates for future runs.
Re-running failed jobs on that old tag can recover from transient runner
capacity, but it does not pick up workflow changes committed later. Do not move
an existing release tag to apply these optimizations.

## Tags

[Native packages](../../.github/workflows/release.yml) is currently manual-only
and requires a selected `v*` tag. Its preserved full delivery flow follows. It first requires exact `vMAJOR.MINOR.PATCH` agreement with VERSION and every
manifest/lockfile, plus tag-commit ancestry in `origin/master`. It calls the same
CI workflow on that exact tagged commit, then builds packages on native runners.

The Ubuntu 22.04-built DEBs are downloaded unchanged and installed/tested on
**22.04, 24.04 and 26.04**. Windows and each macOS architecture test their own
native installers/images. Every test checks package hashes, GUI open/render/JSON
export for all current measurements, CLI operation, manual installation and
removal. Linux also verifies that removing the viewer preserves the CLI package.

Only after every lifecycle job succeeds does one job assemble the complete
artifact set and attach it to a **draft** GitHub release. It refuses to modify an
already public release. After verifying uploaded metadata and re-downloaded
bytes, it publishes automatically. External signing and notarization are
separate. Failed packages, missing targets and packages above
50,000,000 bytes block publication. See [packaging](../release/packaging.md).

## Evidence and badges

README CI and temporary-DEB badges show their actual workflow state. Platform badges describe
the required matrix, not passing results. A configured runner is not evidence of
a successful hosted run; inspect the first run after pushing these changes.
The linked run completed native build/test checks on all three Ubuntu versions,
Windows x64 and macOS Intel. Its macOS ARM job never acquired a runner, and
package/lifecycle jobs did not run. This revised matrix and its cache timings
still require validation in a new hosted run.

The local pre-push gate checks versions, tooling, Rust and Linux Qt without
installing dependencies. Native package installation is restricted to disposable
GitHub runners. Local package builds do not install or publish anything.

## Tag build timing investigation

The v0.1.7 Ubuntu run 41949867202 took 19m 34s after starting (queue time is
separate): CodeQL about 5m, the full local gate 8m 23s, packaging 4m 26s,
and installed lifecycle checks 24s. Its Rust restore explicitly reported
`No cache found`; Qt SDK restoration succeeded. Qt test compilation took 3m 24s,
Qt Clippy 3m 01s, GUI build 46s and the release GUI build 3m 47s.

Package workflows now use the same `native` Rust cache namespace as native
checks so compatible default-branch caches are candidates. Preserve runner,
Qt/toolchain/lockfile cache invalidation and all checks. GitHub caches created
under one tag are not available to a different tag; without a compatible master
cache a cold build is expected. No measured speedup is claimed until the next
hosted run. The user-paused native matrix is not re-enabled to warm caches.

Ubuntu lifecycle CI additionally exercises the real elevated Rust installer
using synthetic older control versions for both DEBs, verifies the resulting
versions and refuses same-version reinstallation. This is not historical-binary
upgrade evidence or automated proof of the interactive Polkit prompt.

Cache scope reference: [GitHub cache access restrictions](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching#restrictions-for-accessing-a-cache).

### Follow-up on v0.1.8

Run 41955696360 job 127011859530 still spent 7m52s in the full gate before
packaging. Sharing the Rust cache namespace alone did not remove the repeated
Qt native compilation cost. The temporary Ubuntu job now wraps C/C++ compilers
with ccache, retaining content/flag/compiler validation and all existing tests.
It caches within the job and restores compatible snapshots when GitHub scope
permits. A final statistics step reports hits/misses even after test failure.
No relaxed ccache correctness settings or disabled tests are used. The running
tag uses its original workflow; this change applies to a subsequent tag.
Measure the next run before claiming a speedup.

### Cross-tag reuse of native build outputs

The v0.1.9 run 41958820691 again enters a multi-minute native check phase.
The previous ccache change helps within a job, but actions/cache cannot restore
the successful v0.1.8 tag's outputs into v0.1.9. Merely changing keys does not
remove that ref isolation. We preserve tag-only execution and the existing
CodeQL/test/package/lifecycle/publication gates.

The temporary Ubuntu job now also retains a separate 14-day **native build cache
artifact**, not a public release asset. It selects only successful push runs of
the exact Ubuntu workflow whose stable tag commit is an ancestor of the current
validated commit. Artifact names include a fingerprint of runner image/ABI,
Rust compiler, C++ compiler and pinned Qt setup. PR/branch runs, expired artifacts,
incompatible fingerprints and non-ancestor commits cannot supply the cache.

`scripts/ci/native_cache.py` safely restores Cargo output trees and ccache data,
preserving executable modes and relative CXX-Qt header links. Archives have
entry/size/path/link restrictions and are staged before replacing build output.
Workspace application/test binaries and incremental state are excluded.
No credentials, user captures, package assets or test results are reused. Cargo
and ccache still validate source/configuration; every check and installed-package
test runs again, and public packages are rebuilt for the current version.

The first tag containing this mechanism seeds the artifact. Subsequent compatible
tags can reuse it; old workflows and the currently running tag cannot acquire
new cache steps retroactively. Do not promise a measured reduction before the
next compatible warm run. Compiler cache statistics are reset after restoration
so the reported hits/misses describe that run rather than prior releases.

Actual v0.1.9 timings: full gate **8m38s**, packaging **4m42s**, total workflow
**20m46s**. Ccache reported **56 hits / 793 calls (7.06%)**; Rust restoration
reported `No cache found`. These results confirm the previous changes did not
provide the requested reduction. The new artifact mechanism targets reuse of
complete native dependency outputs, not just repeated C++ calls inside one job.
Oversized/failed optional cache exports are not uploaded and do not block an
already tested package; no build, test or publication validation is bypassed.
