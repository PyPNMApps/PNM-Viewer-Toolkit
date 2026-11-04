#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Install the pinned Qt SDK in ignored local storage on Linux x86_64.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
if [[ "${1:-}" == --help ]]; then
  echo "Usage: scripts/setup-qt.sh"
  echo "Download Qt 6.8.3 and aqtinstall into local/ (Linux x86_64)."
  exit 0
fi
if [[ $# -ne 0 || "$(uname -s)" != Linux || "$(uname -m)" != x86_64 ]]; then
  echo "This setup helper supports Linux x86_64 only. See docs/development/qt.md." >&2
  exit 2
fi
QT_ROOT="${PROJECT_DIR}/local/qt"
QT_SDK="${QT_ROOT}/6.8.3/gcc_64"
if [[ -x "${QT_SDK}/bin/qmake" ]] &&
   LD_LIBRARY_PATH="${QT_SDK}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}" "${QT_SDK}/bin/qmake" -query QT_VERSION >/dev/null 2>&1 &&
   [[ -x "${QT_SDK}/bin/qmltestrunner" && -d "${QT_SDK}/qml/QtQuick/Controls" ]]; then
  echo "Qt SDK is already available: ${QT_SDK}"
  exit 0
fi
python3 -m venv "${PROJECT_DIR}/local/qt-tools"
"${PROJECT_DIR}/local/qt-tools/bin/python" -m pip install 'aqtinstall==3.3.0'
"${PROJECT_DIR}/local/qt-tools/bin/aqt" install-qt linux desktop 6.8.3 linux_gcc_64 \
  -O "${QT_ROOT}" --archives qtbase qtdeclarative qtshadertools qtsvg qttools icu
LD_LIBRARY_PATH="${QT_SDK}/lib${LD_LIBRARY_PATH:+:${LD_LIBRARY_PATH}}" "${QT_SDK}/bin/qmake" -query QT_VERSION
