# Continuous integration

[Documentation index](../README.md) · [Native packaging](../release/packaging.md)

## Automatic multi-OS releases

A `v*` tag starts [Native releases](../../.github/workflows/release.yml):

1. Validate VERSION and master ancestry, then require CodeQL and code checks.
2. Run the six-platform native matrix. Build packages on Ubuntu 22.04, Windows
   x64, macOS arm64 and macOS Intel **in the same jobs after their tests**. This
   reuses dependency outputs instead of starting a second cold packaging matrix.
3. Exercise Windows MSI baseline/major-upgrade/downgrade/uninstall and both Mac
   DMG copied-app lifecycles on those native hosts.
4. Install the identical Ubuntu 22.04-built DEBs on 22.04, 24.04 and 26.04.
5. Verify the complete asset set, hashes and 50 MB budgets, then upload to a draft,
   verify remote metadata and downloaded bytes, and publish automatically.

Every stage is required for a complete stable release. Failed platforms remain
failed, while successful ones can publish a partial prerelease as described below. Windows
Server 2022 is the x64 test host, not proof of Windows 11 security-dialog behavior.
Mac builds use native Cocoa QML event delivery; Linux/Windows QML tests use
software/offscreen rendering. No failing interaction or pixel assertion is skipped.
Earlier Windows axis-width and Mac offscreen interaction failures prompted dynamic
font-based axis margins, test synchronization and the Cocoa selection; native
hosted success remains to be established for this revised configuration.

Master/PR CI remains lean CodeQL/static checking. Manual CI can request the full
native test matrix without packages; tags always require it with packages.
[The former Ubuntu-only workflow](../../.github/workflows/ubuntu-deb-temp.yml)
is now manual troubleshooting only; it has no tag trigger, avoiding duplicate
release builds. Never run it against an already published tag to change assets.

Ubuntu and both macOS architectures carry dependency artifacts between successful
ancestor release runs with matching compiler/image/Qt identity. Native checkout
uses full history so ancestry can be verified; a shallow checkout fails explicitly
instead of silently missing every older commit. Mac artifacts use a separate
prefix, with architecture included in the fingerprint. Only Ubuntu 22.04 and
the two Mac packaging hosts export/restore these archives; Windows retains its
existing Rust/Qt caches. Qt test setup changes
correctly change the current fingerprint, so the first revised release may be
cold; subsequent matching tags can reuse its cache. Other platforms use their
native Cargo/Qt caches. No test results or public packages are reused.

The first tag with the Mac artifact steps seeds those caches. Earlier tags,
including v0.2.5, cannot acquire the new steps while running. A later compatible
tag must demonstrate restoration and shorter compilation before claiming a
speedup. Cargo still validates dependencies; workspace binaries, test results
and incremental state are excluded. All native and installer tests run again.

## Full CI mode

[CI](../../.github/workflows/ci.yml) runs on pushes to `master`/`main`, pull
requests, manual dispatch, and reusable calls. Master/PR runs use the lean scope above; tags and explicit manual full mode use
the native matrix below.

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
canceled or skipped stage makes **Required checks** fail. Lean master/PR
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

v0.2.6's Intel Mac QML run reported profile index 1 after the modulation test
clicked index 0. That test used a fixed 50 ms delay after replacing the model.
Profile interaction tests now wait for both GridLayout and Flow polish and a
rendered button before sending clicks, assert non-overlapping geometry, and
verify the requested index after every toggle. This addresses a possible layout
race without retrying tests or weakening selection assertions; native Mac
confirmation is still required on the next tag.
A focused regression replaces one-, three- and two-profile models and clicks
every profile on and off, checking the exact routed index after each event.

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

Linux native jobs install LLVM's `lld` linker, which the pinned Qt build helper
selects automatically when available. This addresses its missing-linker warning
on Ubuntu 26.04 without overriding the system linker or relaxing build checks.

The cache-export unit test captures and asserts its deliberately generated
warning instead of sending a synthetic annotation to GitHub. Real export
failures still warn and keep optional cache artifacts out of uploads.

Warnings observed in v0.2.2 that need separate follow-up:

- CodeQL reports a column-kind mismatch while merging its own language SARIF
  files; analysis and result upload succeed. Do not rewrite generated positions
  or disable a language scan to silence this tooling warning.
- `ilammy/msvc-dev-cmd@v1` still declares Node 20. Its latest published release,
  v1.13.0, has no Node 24 replacement as of this investigation. The forced Node
  24 execution passed; upgrading first-party cache/artifact actions does not
  change this third-party action's metadata.
- Qt macOS deployment includes database plugins with private-API warnings and
  missing external database-client libraries. The viewer lifecycle passes, but
  these plugin messages require a focused deployment review. Do not suppress
  warnings or remove required platform/dialog plugins to make logs look clean.

Native validation and package builds cache the pinned Qt SDK under `local/qt`.
Keys include the exact runner image label, architecture and SDK setup-script
hash. Setup still checks the SDK version and presentation-test executable on
cache hits; a hit never skips the build or tests. The Python installer virtual
environment is not cached.

Both native and manual troubleshooting workflows invoke Qt setup with
`--download-attempts 3`. Only recognized transient archive network failures are
retried, after 10 and then 20 seconds. Each retry repeats the complete pinned
archive request even after partial extraction. Installer diagnostics remain in
the job log; exhausting retries preserves failure. Invalid SDKs, checksum or
configuration errors, tests, builds and installer lifecycle checks are not
retried. Local setup defaults to one attempt. This cannot guarantee recovery
from a persistently unavailable Qt mirror.

Cache and artifact actions use Node 24 releases (`actions/cache@v5` and
`actions/upload-artifact@v6`), following
[GitHub's Node 20 migration guidance](https://github.blog/changelog/2025-09-19-deprecation-of-node-20-on-github-actions-runners/).

Installed GUI smoke subprocesses set `QT_FORCE_STDERR_LOGGING=1`, so Windows
GUI-subsystem applications send Qt diagnostics and the required render marker
to the captured stderr pipe. This is test-only; normal desktop launches keep
their usual logging. A zero exit status without `UI_SMOKE_OK` still fails.

Desktop packages include the nine approved sanitized example captures. Installed
smoke checks hash them against the source fixtures and use those installed bytes
for all GUI/CLI exports; they also check the initial native Open folder.
Windows lifecycle tests opt out of the desktop shortcut on baseline install,
opt in during upgrade, verify the executable's icon resource and remove the
shortcut at uninstall. Mac tests require the ICNS referenced by Info.plist.
Interactive wizard checkbox/navigation and Explorer/Finder appearance still
require a native desktop review after a successful hosted package run.

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
cancellations; lean branch mode follows the explicit scope above.

The [v0.1.1 run](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41914986990)
failed because its macOS ARM desktop job never acquired a hosted runner. GitHub
reported capacity constraints. The completed matrix jobs passed. The Intel macOS
Qt job spent 4m10s in SDK setup and 16m29s in checks; CodeQL took 4m34s before the
matrix. Those are observed uncached timings, not estimates for future runs.
Re-running failed jobs on that old tag can recover from transient runner
capacity, but it does not pick up workflow changes committed later. Do not move
an existing release tag to apply these optimizations.

## Tags

### Partial releases after a platform failure

Successful native jobs finish and retain their packages even when another OS
fails. The package-discovery job runs after the matrix regardless of its result;
Ubuntu's three installer lifecycle jobs run if the baseline DEBs passed native
checks. This avoids skipping Ubuntu installation merely because a Mac failed.

Publication selects only same-run packages whose native jobs passed. Linux also
requires native checks and installed lifecycle checks on all three Ubuntu
versions. Global identity, CodeQL and code checks remain mandatory. Failed,
canceled, missing or ambiguous platform evidence excludes that platform.
No passing targets means no release. Package hashes, sizes, uploaded metadata
and re-downloaded bytes are verified for every included target.

If anything failed, successful installers appear in a clearly labeled **Partial
release — CI failed** prerelease. Its page lists unavailable platforms. CI stays
red, the partial release is not marked latest, and stable in-app update checks
do not offer it. Downloads remain available manually on Releases. Published
assets are immutable, so a subsequent fix uses a new tag. All-pass runs still
publish a complete stable development release. Existing tags retain their old
workflow; these changes do not retroactively publish v0.2.6 packages.

[Native releases](../../.github/workflows/release.yml) runs on `v*` pushes and
can be manually dispatched against a selected tag. It requires exact `vMAJOR.MINOR.PATCH` agreement with VERSION and every
manifest/lockfile, plus tag-commit ancestry in `origin/master`. It calls the same
CI workflow on that exact tagged commit, then builds packages on native runners.

The Ubuntu 22.04-built DEBs are downloaded unchanged and installed/tested on
**22.04, 24.04 and 26.04**. Windows and each macOS architecture test their own
native installers/images. Every test checks package hashes, GUI open/render/JSON
export for all current measurements, CLI operation, manual installation and
removal. Linux also verifies that removing the viewer preserves the CLI package.

After every lifecycle job succeeds, one job assembles the complete
artifact set and attach it to a **draft** GitHub release. It refuses to modify an
already public release. After verifying uploaded metadata and re-downloaded
bytes, it publishes automatically. External signing and notarization are
separate. Failed packages are never published; missing targets block stable
publication. Packages above 50,000,000 bytes block their platform. See
[packaging](../release/packaging.md) and the partial-release policy above.

Publication helper failures print captured GitHub CLI stderr and preserve the
original failure. If only publication fails after every required gate passed,
retry that failed job with its original tested artifacts. Do not rebuild assets
under the same public tag or change an already published release.

## Evidence and badges

README CI and native-release badges show their actual workflow state. Platform badges describe
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
Qt native compilation cost. The earlier temporary Ubuntu job wrapped C/C++ compilers
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

The earlier temporary Ubuntu job also retained a separate 14-day **native build cache
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

### First native-cache artifact: v0.1.11

[Run 41963123796](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/actions/runs/41963123796)
succeeded and published v0.1.11. Total elapsed time was **17m02s**, compared with
**20m46s** for v0.1.9 (3m44s shorter, about 18%).

| Stage | Elapsed time |
| --- | --- |
| CodeQL job | 4m12s |
| Full workspace/CLI/tooling/Qt checks | 6m29s |
| Packaging | 3m35s |
| Installed-package lifecycle tests | 38s |
| Native cache export and upload | 33s |
| Publication job | 12s |

Rust cache restoration missed, and no earlier native-output artifact existed.
The Qt SDK cache restored successfully. Ccache again reported 56/793 hits
(7.06%). The shorter run therefore does **not** demonstrate a warm native-cache
speedup. Qt test compilation took 2m40s, Qt Clippy 2m27s and the optimized Qt
build 3m00s; compilation remains the main opportunity for savings.

The run successfully retained a native dependency artifact containing 9,052
entries: 2,132,184,328 expanded bytes and 748,731,628 artifact download bytes.
The next compatible tag should select run 41963123796 and restore those outputs.
Verify the selected source, successful restoration, dependency build durations,
all repeated checks, installed-package tests and total time before claiming a
cache benefit. Runner-image/compiler/Qt changes may correctly require a cold
build. Keep all existing validation gates and CodeQL-before-native ordering.
