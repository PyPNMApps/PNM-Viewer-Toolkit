#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
#
# Run Cargo with the ignored project-local toolchain.
# Forward all arguments and the Cargo exit status; do not install tools.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
export RUSTUP_HOME="${PROJECT_DIR}/local/rustup"
export CARGO_HOME="${PROJECT_DIR}/local/cargo"
export PATH="${CARGO_HOME}/bin:${PATH}"
cd "${PROJECT_DIR}"
case "${1:-}" in
  build|test|check|clippy|run)
    python3 "${PROJECT_DIR}/tools/maintenance/rust_clean.py" --if-disk-full
    ;;
esac
exec "${CARGO_HOME}/bin/cargo" "$@"
