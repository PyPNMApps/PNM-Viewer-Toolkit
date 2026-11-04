# Development setup

PNM Viewer Toolkit remains under development. The Rust parsers, native Qt viewer
and retained Tauri/React prototype have runnable source builds. See the
[desktop user guide](../ui/README.md) for the main Qt workflow.

## Install

Run as your normal user, from the repository root:

```bash
./install.sh --development --dry-run
./install.sh --development
./install.sh --development --check
```

The entry point also works by absolute path from another directory. No arguments
or invalid options exit 2. Help exits 0. Check exits 0 when all prerequisites are
ready, 1 when incomplete; unsupported hosts exit 2. Dry run prints the plan and
exits 0 on a supported host, even if dependencies are missing. Both inspection
modes avoid network access, sudo and filesystem writes. Do not combine them.

Installation supports Ubuntu 22.04 and 24.04 on x86_64 and ARM64. It rejects other
platforms and root execution before installation. Only the Ubuntu 22.04 x86_64
inspection path and mocked installer behavior have been tested; a full native
installation and the other supported combinations remain unverified.

Missing OS packages are installed with `sudo apt-get update` and
`sudo apt-get install -y --no-install-recommends`. Sudo may request your password.
Configured Ubuntu repositories must provide the packages; the installer does not
add repositories. Network access is required for missing tools. Package-manager
or download failures stop setup and propagate a nonzero exit status.

## Dependencies and ownership

- Native development: build-essential, pkg-config, libssl-dev, WebKitGTK 4.1,
  libxdo-dev, libayatana-appindicator3-dev and librsvg2-dev. The desktop libraries
  follow [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/).
- Utilities: Git, CA certificates, curl, wget, file, xz-utils, unzip, Python 3,
  Python venv and ShellCheck. Python supports repository tooling, not the app.
- Rust: the channel from `rust-toolchain.toml`, with rustfmt and Clippy, installed
  into `local/cargo` and `local/rustup`. Bootstrap uses the official HTTPS rustup
  installer at https://sh.rustup.rs without modifying shell profiles.
- Node.js/npm: version from `.node-version`, installed into `local/node-vVERSION`.
  Official Node archives are checked against committed SHA-256 values in
  `scripts/install-development.sh`. Update both architecture hashes when changing
  the pin; source: https://nodejs.org/dist/v24.22.2/SHASUMS256.txt.

The installer reuses ready local tools and skips apt when packages are present.
Downloads are staged in a temporary directory under `local/` and cleaned on exit.
An incomplete existing Node directory causes an explicit error; move it aside
before retrying. Existing global Rust/Node installations and shell profiles are
untouched. System package versions follow the configured Ubuntu repositories.
No Docker, Kubernetes, Java, Python services or global npm packages are needed.
Qt uses a separate [project-local SDK setup](../development/qt.md#linux-development).
The retained Tauri frontend uses the [documented npm setup](../development/ui.md#run-locally).

## Use the tools

```bash
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/node-local.sh node --version
scripts/node-local.sh npm --version
```

The Node wrapper preserves the caller's directory so future frontend commands can
run inside their package directory. Its npm cache stays in `local/npm-cache`.
The Rust wrapper runs from the repository root. Neither wrapper installs tools.

## Validate installer changes

```bash
bash -n install.sh scripts/install-development.sh scripts/node-local.sh
python3 scripts/tests/test_install.py
shellcheck install.sh scripts/install-development.sh scripts/node-local.sh
./install.sh --development --dry-run
./install.sh --development --check
```

Tests use an isolated temporary checkout with fake OS tools. They exercise argument
handling, unsupported hosts, read-only modes, prerequisite failures, and repeat
setup without system installation or downloads. They do not validate downloaded
binaries, real apt installation or GUI execution.

The design follows PyPNM's explicit development mode and OSCAR-UI's prerequisite
checks and project-local dependency ownership, adapted to this Rust/Tauri stack.

## CLI-only installation

After preparing the Rust toolchain, use `./install.sh --cli --prefix DIRECTORY`.
This builds the decoder without UI dependencies. See [CLI usage](../cli/usage.md).
