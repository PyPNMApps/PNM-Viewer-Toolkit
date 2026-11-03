#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run a Node.js development command using the pinned project-local distribution.
# Pass node, npm or npx followed by its arguments; preserve the command exit code.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
NODE_VERSION="$(cat "${PROJECT_DIR}/.node-version")"
export PATH="${PROJECT_DIR}/local/node-v${NODE_VERSION}/bin:${PATH}"
export npm_config_cache="${PROJECT_DIR}/local/npm-cache"
case "${1:-}" in
  node|npm|npx) ;;
  *) printf 'Usage: %s <node|npm|npx> [arguments...]\n' "$0" >&2; exit 2 ;;
esac
NODE_COMMAND="$1"
shift
exec "${PROJECT_DIR}/local/node-v${NODE_VERSION}/bin/${NODE_COMMAND}" "$@"
