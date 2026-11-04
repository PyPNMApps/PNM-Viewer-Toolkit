#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run desktop development commands with the repository-local Node/Rust toolchains.
# Arguments: npm script (dev by default), followed by optional script arguments.
# Returns: the npm command's exit status; never installs system packages.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
export RUSTUP_HOME="${PROJECT_DIR}/local/rustup"
export CARGO_HOME="${PROJECT_DIR}/local/cargo"
export PATH="${CARGO_HOME}/bin:${PATH}"
cd "${PROJECT_DIR}/apps/desktop"
if [[ "${1:-dev}" == dev ]]; then
  exec "${SCRIPT_DIR}/node-local.sh" npm run tauri -- dev
fi
exec "${SCRIPT_DIR}/node-local.sh" npm run "$@"
