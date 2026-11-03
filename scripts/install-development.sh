#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Install Ubuntu native dependencies and isolated Rust/Node development tools.
# --check and --dry-run never download, elevate privileges or change files.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
DEVELOPMENT=0
MODE=install

# Describe supported modes, system changes and project-local destinations.
# Return success without checking or changing the host.
usage() {
  cat <<'HELP'
Usage: ./install.sh --development [--check | --dry-run]

  -d, --development  Install development tools (Ubuntu 22.04/24.04)
      --check        Report missing prerequisites; exit 1 if incomplete
      --dry-run      Print setup plan without changing anything
  -h, --help         Show this help

Supports Linux x86_64 and ARM64. Install mode uses sudo apt-get for missing
system packages. Run as your normal user; Rust and Node stay under local/.
No shell profiles, global npm packages or application services are changed.
HELP
}
for argument in "$@"; do
  case "$argument" in
    -d|--development) DEVELOPMENT=1 ;;
    --check|--dry-run)
      if [[ "$MODE" != install ]]; then
        printf 'Choose only one of --check or --dry-run.\n' >&2
        exit 2
      fi
      MODE="${argument#--}"
      ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown option: %s\n' "$argument" >&2; usage >&2; exit 2 ;;
  esac
done
if [[ "$DEVELOPMENT" != 1 ]]; then
  usage >&2
  exit 2
fi
if [[ "$(uname -s)" != Linux || ! -r /etc/os-release ]]; then
  printf 'Supported hosts: Ubuntu 22.04/24.04 Linux.\n' >&2
  exit 2
fi
# shellcheck source=/dev/null
source /etc/os-release
case "${ID:-}:${VERSION_ID:-}" in
  ubuntu:22.04|ubuntu:24.04) ;;
  *) printf 'Unsupported host: %s\n' "${PRETTY_NAME:-unknown}" >&2; exit 2 ;;
esac
# Official Node v24.22.2 SHASUMS256.txt; update with .node-version.
case "$(uname -m)" in
  x86_64) NODE_ARCH=x64; NODE_SHA256=48cc0e21e048790e54c0cf5749c2725005313d652651c40b7a21feb9697aaf75 ;;
  aarch64) NODE_ARCH=arm64; NODE_SHA256=eb187837a2230d4047161ac0be008584e16b46f726e9c9490ee35dc6b17d9ba8 ;;
  *) printf 'Unsupported CPU architecture.\n' >&2; exit 2 ;;
esac
NODE_VERSION="$(cat "${PROJECT_DIR}/.node-version")"
NODE_DIR="${PROJECT_DIR}/local/node-v${NODE_VERSION}"
# Extract the quoted channel assignment, e.g. channel = "1.85.1" -> 1.85.1.
RUST_VERSION="$(sed -n 's/^channel = "\([^"]*\)"$/\1/p' "${PROJECT_DIR}/rust-toolchain.toml")"
if [[ -z "$RUST_VERSION" ]]; then
  printf 'Missing Rust channel in rust-toolchain.toml.\n' >&2
  exit 2
fi
export CARGO_HOME="${PROJECT_DIR}/local/cargo"
export RUSTUP_HOME="${PROJECT_DIR}/local/rustup"
export PATH="${CARGO_HOME}/bin:${PATH}"
PACKAGES=(
  ca-certificates curl git build-essential pkg-config libssl-dev
  libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev
  wget file xz-utils unzip python3 python3-venv shellcheck
)
MISSING_PACKAGES=()
for package in "${PACKAGES[@]}"; do
  if [[ "$(dpkg-query -W -f='${Status}' "$package" 2>/dev/null || true)" != 'install ok installed' ]]; then
    MISSING_PACKAGES+=("$package")
  fi
done
RUST_READY=0
if [[ -x "${CARGO_HOME}/bin/rustup" ]] &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" rustc --version >/dev/null 2>&1 &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" cargo fmt --version >/dev/null 2>&1 &&
   "${CARGO_HOME}/bin/rustup" run "$RUST_VERSION" cargo clippy --version >/dev/null 2>&1; then
  RUST_READY=1
fi
NODE_READY=0
if [[ -x "${NODE_DIR}/bin/node" && -x "${NODE_DIR}/bin/npm" ]] &&
   [[ "$("${NODE_DIR}/bin/node" --version)" == "v${NODE_VERSION}" ]]; then
  NODE_READY=1
fi
printf 'Host: %s (%s)\n' "$PRETTY_NAME" "$NODE_ARCH"
printf 'Missing apt packages: %s\n' "${MISSING_PACKAGES[*]:-none}"
printf 'Local Rust %s ready: %s; Node %s ready: %s\n' "$RUST_VERSION" "$RUST_READY" "$NODE_VERSION" "$NODE_READY"
if [[ "$MODE" == check ]]; then
  [[ ${#MISSING_PACKAGES[@]} == 0 && "$RUST_READY" == 1 && "$NODE_READY" == 1 ]]
  exit $?
fi
if [[ "$MODE" == dry-run ]]; then
  printf 'Would install missing apt packages, Rust with rustfmt/Clippy and Node/npm.\nLocal tools: %s/local/; no frontend manifest to install yet.\n' "$PROJECT_DIR"
  exit 0
fi
if [[ "$EUID" == 0 ]]; then
  printf 'Run as your normal user, without sudo; apt alone uses sudo.\n' >&2
  exit 2
fi
if [[ ${#MISSING_PACKAGES[@]} -gt 0 ]]; then
  sudo apt-get update
  sudo env DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends "${MISSING_PACKAGES[@]}"
fi
mkdir -p "${PROJECT_DIR}/local"
TEMP_DIR="$(mktemp -d "${PROJECT_DIR}/local/development-install.XXXXXX")"
trap 'rm -rf -- "$TEMP_DIR"' EXIT
if [[ "$RUST_READY" != 1 ]]; then
  if [[ ! -x "${CARGO_HOME}/bin/rustup" ]]; then
    curl --proto '=https' --tlsv1.2 -fsSL https://sh.rustup.rs -o "${TEMP_DIR}/rustup-init.sh"
    bash "${TEMP_DIR}/rustup-init.sh" -y --no-modify-path --profile minimal --default-toolchain none
  fi
  "${CARGO_HOME}/bin/rustup" toolchain install "$RUST_VERSION" --profile minimal --component rustfmt --component clippy
fi
if [[ "$NODE_READY" != 1 ]]; then
  if [[ -e "$NODE_DIR" ]]; then
    printf 'Incomplete Node installation at %s; move it aside and rerun.\n' "$NODE_DIR" >&2
    exit 1
  fi
  NODE_ARCHIVE="node-v${NODE_VERSION}-linux-${NODE_ARCH}.tar.xz"
  curl --proto '=https' --tlsv1.2 -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/${NODE_ARCHIVE}" -o "${TEMP_DIR}/${NODE_ARCHIVE}"
  (cd "$TEMP_DIR" && printf '%s  %s\n' "$NODE_SHA256" "$NODE_ARCHIVE" | sha256sum --check -)
  mkdir "${TEMP_DIR}/node"
  tar -xJf "${TEMP_DIR}/${NODE_ARCHIVE}" -C "${TEMP_DIR}/node" --strip-components=1
  mv "${TEMP_DIR}/node" "$NODE_DIR"
fi
bash "${PROJECT_DIR}/install.sh" --development --check
printf 'Development tools ready. Run scripts/cargo-local.sh test --workspace --locked.\n'
