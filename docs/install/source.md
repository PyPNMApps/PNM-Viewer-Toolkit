# Build the CLI from source

From a checkout of `PNM-Viewer-Toolkit`, with the pinned Rust toolchain available:

```bash
cargo build --release --locked -p pnm-decode
```

This standard Cargo command selects only the CLI and its required Rust libraries.
It does not build a UI or require Qt development packages.
The same command applies to Linux, macOS and Windows; native Windows/macOS
execution has not yet been validated for this project.

| Platform | Build output |
| --- | --- |
| Linux/macOS | `target/release/pnm-decode` |
| Windows | `target\release\pnm-decode.exe` |

Linux/macOS example:

```bash
./target/release/pnm-decode capture.bin --output capture.json
```

PowerShell example:

```powershell
.\target\release\pnm-decode.exe capture.bin --output capture.json
```

## Repository-local toolchain

If the repository's development environment is already installed, use:

```bash
./scripts/cargo-local.sh build --release --locked -p pnm-decode
```

The wrapper uses ignored local Cargo/Rustup directories and does not install
missing tools. The [development installer](development.md) provisions the broader
contributor environment; it is not a prerequisite when a suitable standard Rust
installation is already available.

## Install into a prefix on Linux/macOS

With the repository-local toolchain prepared:

```bash
./install.sh --cli --prefix "$PWD/local/cli-install"
./local/cli-install/bin/pnm-decode --help
man -l local/cli-install/share/man/man1/pnm-decode.1
```

The installer builds from source and installs the executable, man page, license
and NOTICE. It does not modify PATH, request sudo or install UI dependencies.
It refuses existing installation files; upgrade/uninstall support is pending.
The Bash installer accepts Linux/macOS, but only Ubuntu 22.04 x86_64 has been
verified. Windows source builds use Cargo directly; Windows packaging is pending.
