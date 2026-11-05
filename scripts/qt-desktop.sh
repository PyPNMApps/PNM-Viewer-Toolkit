#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Build/run/check the Qt prototype with a local or explicitly selected Qt SDK.
# Arguments: run (default), build, check; subsequent run arguments go to the app.
# No downloads, system installation or persistent environment changes occur.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
if [[ "${1:-}" == --help ]]; then
  echo "Usage: scripts/qt-desktop.sh [run|build|check]"
  exit 0
fi
QT_SDK_DIR="${QT_SDK_DIR:-${PROJECT_DIR}/local/qt/6.8.3/gcc_64}"
if [[ ! -x "${QT_SDK_DIR}/bin/qmake" ]]; then
  printf 'Qt SDK missing: %s\nSee docs/development/qt.md for setup.\n' "${QT_SDK_DIR}" >&2
  exit 1
fi
export PATH="${QT_SDK_DIR}/bin:${PATH}"
export LD_LIBRARY_PATH="${QT_SDK_DIR}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}"
export QMAKE="${QT_SDK_DIR}/bin/qmake"
export QT_VERSION_MAJOR=6
export QT_QUICK_CONTROLS_STYLE=Fusion
MANIFEST="${PROJECT_DIR}/apps/qt/Cargo.toml"
ACTION="${1:-run}"
if [[ $# -gt 0 ]]; then shift; fi
if [[ -x "${PROJECT_DIR}/local/cargo/bin/cargo" ]]; then
  CARGO_COMMAND=("${SCRIPT_DIR}/cargo-local.sh")
else
  CARGO_COMMAND=(cargo)
fi
case "${ACTION}" in
  run)
    # Remote-editor terminals can reuse this user's active VM desktop session.
    # Import only display settings; never evaluate the session environment as code.
    if [[ "$(uname -s)" == Linux && "${QT_QPA_PLATFORM:-xcb}" == xcb && -z "${DISPLAY:-}" ]] && command -v systemctl >/dev/null 2>&1; then
      while IFS='=' read -r setting value; do
        case "${setting}" in
          DISPLAY) export DISPLAY="${value}" ;;
          XAUTHORITY)
            if [[ -z "${XAUTHORITY:-}" ]]; then
              export XAUTHORITY="${value}"
            fi
            ;;
        esac
      done < <(systemctl --user show-environment 2>/dev/null || true)
    fi
    # The bundled Linux SDK uses XCB by default (also through XWayland).
    # Explicit offscreen/minimal platforms remain available for automated tests.
    if [[ "$(uname -s)" == Linux && "${QT_QPA_PLATFORM:-xcb}" == xcb && -z "${DISPLAY:-}" ]]; then
      echo "Cannot open the Qt viewer: DISPLAY is not set." >&2
      echo "Run this command from a terminal inside your graphical desktop session." >&2
      echo "For SSH, use working X11 forwarding; see docs/development/qt.md." >&2
      exit 1
    fi
    exec "${CARGO_COMMAND[@]}" run --manifest-path "${MANIFEST}" --locked -- "$@"
    ;;
  build) exec "${CARGO_COMMAND[@]}" build --manifest-path "${MANIFEST}" --locked "$@" ;;
  check)
    python3 "${SCRIPT_DIR}/qt-sdk.py" check
    ;;
  *) printf 'Usage: %s [run|build|check]\n' "$0" >&2; exit 2 ;;
esac
