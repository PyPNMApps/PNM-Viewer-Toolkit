#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
#
# Push a clean main/master branch to origin without forcing.
# Validate remote ancestry before pushing; propagate Git failures.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"
REMOTE="origin"

# Print command usage to stdout without changing repository state.
# Arguments are unused unless described above; failures stop this script.
usage() {
  cat <<'EOF'
Usage: git-push.sh

Push the clean project main/master branch to origin.

Safety checks:
  - The working tree must have no tracked or untracked changes.
  - The current branch must be main or master.
  - The origin remote must exist.
  - The local branch must not be behind or diverged from origin.
  - Required local checks must pass; no skip option is provided.
  - Branch, commit and working tree must stay unchanged during checks.
  - Force-push is never used.
EOF
}

# Print the supplied error to stderr and exit with status 1.
# Arguments are unused unless described above; failures stop this script.
die() {
  echo "ERROR: $*" >&2
  exit 1
}

case "${1:-}" in
  "") ;;
  -h|--help)
    usage
    exit 0
    ;;
  *) die "Unknown argument: $1 (see --help)" ;;
esac

if ! git -C "${PROJECT_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  die "Project directory is not a Git working tree: ${PROJECT_DIR}"
fi

if [[ -n "$(git -C "${PROJECT_DIR}" status --porcelain)" ]]; then
  git -C "${PROJECT_DIR}" status --short >&2
  die "Working tree is not clean; commit or remove changes first"
fi

branch="$(git -C "${PROJECT_DIR}" branch --show-current)"
case "${branch}" in
  main|master) ;;
  "") die "HEAD is detached; check out main or master first" ;;
  *) die "Refusing to push branch '${branch}'; use main or master" ;;
esac

if ! git -C "${PROJECT_DIR}" remote get-url "${REMOTE}" >/dev/null 2>&1; then
  die "Remote '${REMOTE}' is not configured"
fi

remote_head="$(git -C "${PROJECT_DIR}" ls-remote --heads "${REMOTE}" "refs/heads/${branch}")"
if [[ -n "${remote_head}" ]]; then
  echo "Fetching ${REMOTE}/${branch}..."
  git -C "${PROJECT_DIR}" fetch "${REMOTE}" "${branch}"
  remote_ref="${REMOTE}/${branch}"

  if ! git -C "${PROJECT_DIR}" merge-base --is-ancestor "${remote_ref}" HEAD; then
    die "Local ${branch} is behind or diverged from ${remote_ref}"
  fi
else
  echo "Remote branch ${REMOTE}/${branch} does not exist; creating it."
fi

validated_commit="$(git -C "${PROJECT_DIR}" rev-parse HEAD)"
echo "Validating ${branch} before push..."
bash "${PROJECT_DIR}/scripts/check-before-push.sh"
if [[ "$(git -C "${PROJECT_DIR}" rev-parse HEAD)" != "${validated_commit}" ||
      "$(git -C "${PROJECT_DIR}" branch --show-current)" != "${branch}" ||
      -n "$(git -C "${PROJECT_DIR}" status --porcelain)" ]]; then
  die "Branch, commit or working tree changed during validation; retry after saving"
fi
echo "Pushing validated commit ${validated_commit} to ${REMOTE}/${branch}..."
git -C "${PROJECT_DIR}" push --set-upstream "${REMOTE}" "${validated_commit}:refs/heads/${branch}"
