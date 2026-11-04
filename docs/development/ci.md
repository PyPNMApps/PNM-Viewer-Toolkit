# Continuous integration

Three focused workflows run on pushes to main/master, pull requests and manual
dispatch, with separate README badges:

| Badge | Workflow | Checks |
| --- | --- | --- |
| CLI build | [cli-build.yml](../../.github/workflows/cli-build.yml) | Release decoder build, help/version and RxMER export smoke test |
| Tests | [tests.yml](../../.github/workflows/tests.yml) | All Rust workspace tests, release-tool tests, Linux mocked installer tests |
| Code checks | [ci.yml](../../.github/workflows/ci.yml) | Rustfmt, Clippy with warnings denied, installer Bash syntax and ShellCheck |

Tests include every parser, shared headers/dispatch, Q-format math and CLI
integration cases. Preserved upstream Python fixture/reference files are not a
runnable PyPNM application test suite here. Installer tests use fake host tools
on Linux and do not provision the runner.

Builds and tests run on these native runners; code checks run once on Linux:

- Ubuntu 22.04 x64
- Windows Server 2022 x64
- macOS 15 Apple Silicon
- macOS 15 Intel

Rust and components come from `rust-toolchain.toml`; dependencies use Cargo.lock.
CI uses the runner's Rustup/Cargo directly, not the local development installer.
No UI, Node.js, Tauri setup, signing or publication is included. No artifacts or
caches are uploaded initially; add these when a concrete distribution need arises.

Each README badge reports its workflow result across all its jobs. Runner configuration
is not proof of a passing platform test: inspect the first hosted run after push.
The Rust badge reflects the toolchain pin; update it when the pin changes. The
license badge links to component-specific terms, not a choice of licenses for
every file. No release/download badge is shown while binaries are unpublished.

GitHub's [runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners)
documents the selected native labels. The
[checkout action](https://github.com/actions/checkout) is used with persisted
credentials disabled. Extend this workflow with UI checks only when the desktop
application manifests and tests exist.

The [push helper](git.md) also runs a mandatory local validation gate before
pushing main/master. CI tests the gate with fake Git, including failure and
checkout-change cases; it does not perform real test pushes.
