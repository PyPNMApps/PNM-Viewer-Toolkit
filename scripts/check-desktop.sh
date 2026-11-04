#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Validate the desktop frontend and native Rust shell without installing packages.
# Returns the first failing check's status. Run npm ci in apps/desktop beforehand.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
if [[ -x "${PROJECT_DIR}/local/cargo/bin/cargo" ]]; then
  CARGO_COMMAND=("${SCRIPT_DIR}/cargo-local.sh")
else
  CARGO_COMMAND=(cargo)
fi
if [[ -x "${PROJECT_DIR}/local/node-v$(cat "${PROJECT_DIR}/.node-version")/bin/npm" ]]; then
  NPM_COMMAND=("${SCRIPT_DIR}/node-local.sh" npm)
else
  NPM_COMMAND=(npm)
fi
cd "${PROJECT_DIR}/apps/desktop"
"${NPM_COMMAND[@]}" run typecheck
"${NPM_COMMAND[@]}" run lint
"${NPM_COMMAND[@]}" test
"${NPM_COMMAND[@]}" run build
MANIFEST="${PROJECT_DIR}/apps/desktop/src-tauri/Cargo.toml"
"${CARGO_COMMAND[@]}" fmt --manifest-path "${MANIFEST}" -- --check
"${CARGO_COMMAND[@]}" test --manifest-path "${MANIFEST}" --locked
"${CARGO_COMMAND[@]}" clippy --manifest-path "${MANIFEST}" --all-targets --locked -- -D warnings
"${CARGO_COMMAND[@]}" build --manifest-path "${MANIFEST}" --locked --features custom-protocol
