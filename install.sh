#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Route development setup or standalone CLI installation to focused modules.
# Arguments and exit status are forwarded unchanged. No default install action.
set -euo pipefail
PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
if [[ "${1:-}" == "--cli" ]]; then
  shift
  exec bash "${PROJECT_DIR}/scripts/install-cli.sh" "$@"
fi
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
  echo "CLI-only installation: ./install.sh --cli --prefix DIRECTORY"
  echo "Full toolkit/UI installation is not implemented yet."
fi
exec bash "${PROJECT_DIR}/scripts/install-development.sh" "$@"
