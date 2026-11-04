#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
# Run required local checks without committing, pushing or installing dependencies.
# Any failed or unavailable required check stops the gate with a nonzero status.
set -euo pipefail
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
cd "${PROJECT_DIR}"
if [[ -x "${PROJECT_DIR}/local/cargo/bin/cargo" ]]; then
  CARGO_COMMAND=("${SCRIPT_DIR}/cargo-local.sh")
else
  CARGO_COMMAND=(cargo)
fi
command -v python3 >/dev/null
command -v shellcheck >/dev/null
"${CARGO_COMMAND[@]}" fmt --all -- --check
"${CARGO_COMMAND[@]}" clippy --workspace --all-targets --locked -- -D warnings
"${CARGO_COMMAND[@]}" test --workspace --locked
python3 -m unittest discover -s tools/release -p 'test_*.py'
python3 -m unittest discover -s tools/git/tests -p 'test_*.py'
if [[ "$(uname -s)" == Linux ]]; then
  python3 -m unittest discover -s scripts/tests -p 'test_*.py'
fi
SHELL_SCRIPTS=(install.sh scripts/install-cli.sh scripts/check-before-push.sh tools/git/git-save.sh tools/git/git-push.sh)
for script in "${SHELL_SCRIPTS[@]}"; do
  bash -n "${script}"
done
shellcheck "${SHELL_SCRIPTS[@]}"
"${CARGO_COMMAND[@]}" build --release --locked -p pnm-decode
# cargo run honors Cargo target-directory configuration and platform executable suffixes.
SMOKE_DIRECTORY="$(mktemp -d)"
trap 'rm -rf -- "${SMOKE_DIRECTORY}"' EXIT
"${CARGO_COMMAND[@]}" run --release --locked --quiet -p pnm-decode -- --version
"${CARGO_COMMAND[@]}" run --release --locked --quiet -p pnm-decode -- --help
"${CARGO_COMMAND[@]}" run --release --locked --quiet -p pnm-decode -- \
  tests/fixtures/pypnm/tests/files/rxmer.bin --output "${SMOKE_DIRECTORY}/rxmer.json"
python3 -m json.tool "${SMOKE_DIRECTORY}/rxmer.json" > /dev/null
echo "All local pre-push checks passed."
