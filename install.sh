#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Prepare development dependencies; delegate setup to the focused module.
# Arguments and exit status are forwarded unchanged. No default install action.
set -euo pipefail
PROJECT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec bash "${PROJECT_DIR}/scripts/install-development.sh" "$@"
