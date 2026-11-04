#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Build and install the standalone decoder to an explicit user-selected prefix.
# No GUI, OS packages, PATH edits or elevated privileges are required.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
INSTALL_PREFIX=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --prefix)
      if [[ $# -lt 2 || -z "$2" ]]; then
        echo "--prefix requires a directory" >&2
        exit 2
      fi
      INSTALL_PREFIX="$2"
      shift 2
      ;;
    -h|--help)
      echo "Usage: ./install.sh --cli --prefix DIRECTORY"
      echo "Build with the local Rust toolchain; install CLI, man page and licenses."
      echo "Existing installation files are refused; upgrades are not implemented."
      exit 0
      ;;
    *) echo "Unknown CLI installer option: $1" >&2; exit 2 ;;
  esac
done
if [[ -z "${INSTALL_PREFIX}" ]]; then
  echo "An explicit --prefix is required; see --help" >&2
  exit 2
fi
case "$(uname -s)" in
  Linux|Darwin) ;;
  *) echo "Source installer supports Linux/macOS; Windows packaging is pending." >&2; exit 2 ;;
esac
mkdir -p -- "${INSTALL_PREFIX}"
INSTALL_PREFIX="$(cd -- "${INSTALL_PREFIX}" && pwd)"
DESTINATIONS=("bin/pnm-decode" "share/man/man1/pnm-decode.1" "share/doc/pnm-decode/LICENSE" "share/doc/pnm-decode/NOTICE")
for destination in "${DESTINATIONS[@]}"; do
  if [[ -e "${INSTALL_PREFIX}/${destination}" || -L "${INSTALL_PREFIX}/${destination}" ]]; then
    echo "Refusing existing installation file: ${INSTALL_PREFIX}/${destination}" >&2
    exit 1
  fi
done
"${SCRIPT_DIR}/cargo-local.sh" build --release --locked -p pnm-decode
mkdir -p -- "${INSTALL_PREFIX}/bin" "${INSTALL_PREFIX}/share/man/man1" "${INSTALL_PREFIX}/share/doc/pnm-decode"
# noclobber protects against a destination appearing after the preflight check.
set -o noclobber
cat "${PROJECT_DIR}/target/release/pnm-decode" > "${INSTALL_PREFIX}/bin/pnm-decode"
chmod 755 "${INSTALL_PREFIX}/bin/pnm-decode"
cat "${PROJECT_DIR}/crates/cli/man/pnm-decode.1" > "${INSTALL_PREFIX}/share/man/man1/pnm-decode.1"
cat "${PROJECT_DIR}/LICENSES/Apache-2.0.txt" > "${INSTALL_PREFIX}/share/doc/pnm-decode/LICENSE"
cat "${PROJECT_DIR}/NOTICE" > "${INSTALL_PREFIX}/share/doc/pnm-decode/NOTICE"
echo "Installed CLI only: ${INSTALL_PREFIX}/bin/pnm-decode"
