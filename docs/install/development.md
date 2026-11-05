# Development setup

PNM Viewer Toolkit remains under development. The Rust parsers, CLI and native
Qt viewer have runnable source builds. See the [desktop user guide](../ui/README.md)
for the Qt workflow and the [developer index](../development/README.md) for code guides.

## Install

Run as your normal user, from the repository root:

```bash
./install.sh --development --dry-run
./install.sh --development
./install.sh --development --check
./scripts/setup-qt.sh
```

The entry point also works by absolute path from another directory. No arguments
or invalid options exit 2. Help exits 0. Check exits 0 when host packages and the
local Rust tools are ready, 1 when incomplete; unsupported hosts exit 2. The Qt
SDK has its own setup/check commands in the [Qt guide](../development/qt.md).
Dry run prints the plan and exits 0 on a supported host even when dependencies
are missing. Both inspection modes avoid network access, sudo and filesystem
writes. Do not combine them.

Installation supports Ubuntu 22.04 and 24.04 on x86_64 and ARM64. It rejects other
platforms and root execution before installation. The Qt SDK setup supports the
native architectures listed in the Qt guide; installer architecture acceptance
alone does not establish Qt SDK availability. The local native viewer has been
exercised on Ubuntu 22.04 x86_64; other combinations require their own evidence.

Missing OS packages are installed with `sudo apt-get update` and
`sudo apt-get install -y --no-install-recommends`. Sudo may request your password.
Configured Ubuntu repositories must provide the packages; the installer adds no
repositories. Network access is required for missing tools. Package-manager or
download failures stop setup and propagate a nonzero exit status.

## Dependencies and ownership

- Native build tools: build-essential, CMake, pkg-config, libssl-dev and `lld`.
- Packaging/cache tools: `patchelf` and `ccache`; Xvfb/Xauth for headless checks.
- Artwork conversion: `python3-pil` (Pillow), used only by the
  [native asset generator](../../packaging/README.md). Released assets are committed;
  no image library is needed by the installed application.
- Qt host dependencies: OpenGL/EGL development and runtime libraries, XKB/XCB
  libraries including libxcb-cursor0, GTK 3 for the native Linux file chooser,
  and libb2. The exact package list lives in `scripts/install-development.sh`.
- Utilities: Git, CA certificates, curl, wget, file, xz-utils, unzip, Python 3,
  Python venv and ShellCheck. Python supports repository tooling, not the app.
- Rust: the channel from `rust-toolchain.toml`, with rustfmt and Clippy, installed
  into `local/cargo` and `local/rustup`. Bootstrap uses the official HTTPS rustup
  installer at https://sh.rustup.rs without modifying shell profiles.
- Qt SDK: pinned project-local Qt/CXX-Qt setup, documented in the
  [Qt development guide](../development/qt.md#linux-development).

The installer reuses ready local Rust tools and skips apt when packages are
present. Downloads are staged in a temporary directory under `local/` and cleaned
on exit. Global tool installations and shell profiles are untouched. System
package versions follow the configured Ubuntu repositories. No Node, npm,
WebKit or WebView toolchain is required. Previously installed system packages
and old local tool caches are not automatically uninstalled.

## Use the tools

```bash
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/qt-desktop.sh
```

The Rust wrapper runs from the repository root and does not install tools.
For native Windows/macOS setup, use the [Qt guide](../development/qt.md#windows-and-macos).

## Validate installer changes

```bash
bash -n install.sh scripts/install-development.sh scripts/cargo-local.sh
python3 scripts/tests/test_install.py
shellcheck install.sh scripts/install-development.sh scripts/cargo-local.sh
./install.sh --development --dry-run
./install.sh --development --check
```

Tests use an isolated temporary checkout with fake OS tools. They exercise argument
handling, unsupported hosts, read-only modes, prerequisite failures, and repeat
setup without system installation or downloads. They do not validate downloaded
binaries, real apt installation or GUI execution.

The design follows PyPNM's explicit development mode and OSCAR-UI's prerequisite
checks and project-local dependency ownership, adapted to Rust and Qt.

## CLI-only installation

After preparing the Rust toolchain, use `./install.sh --cli --prefix DIRECTORY`.
This builds the decoder without Qt dependencies. See [CLI usage](../cli/usage.md).
