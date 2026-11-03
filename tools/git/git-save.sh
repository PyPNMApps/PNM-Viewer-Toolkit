#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"
COMMIT_MSG=""

usage() {
  cat <<'EOF'
Usage: git-save.sh --commit-msg <message>

Stage all changes in the project and create a Git commit.

Options:
  --commit-msg <message>  Required commit subject.
  -h, --help              Show this help and exit.

Example:
  tools/git/git-save.sh --commit-msg "Feature: Add setup modules"
EOF
}

die() {
  echo "ERROR: $*" >&2
  exit 1
}

reject_nested_skill_checkouts() {
  local nested_git

  while IFS= read -r -d '' nested_git; do
    die "Nested Git metadata is not allowed under project skills: ${nested_git}"
  done < <(
    find "${PROJECT_DIR}/skills" -mindepth 2 \
      \( -type d -o -type f \) -name .git -print0
  )

  while IFS= read -r mode _ _ path; do
    if [[ "${mode}" == "160000" && -e "${PROJECT_DIR}/${path}" ]]; then
      die "Company skills must not be committed as a Gitlink: ${path}"
    fi
  done < <(
    git -C "${PROJECT_DIR}" ls-files --stage -- \
      skills/ai_skills skills/mxl-skills
  )

}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit-msg)
      [[ $# -ge 2 ]] || die "--commit-msg requires a message"
      COMMIT_MSG="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *) die "Unknown argument: $1 (see --help)" ;;
  esac
done

[[ -n "${COMMIT_MSG}" ]] || die "--commit-msg is required"
git -C "${PROJECT_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1 || \
  die "Project directory is not a Git working tree: ${PROJECT_DIR}"

reject_nested_skill_checkouts
git -C "${PROJECT_DIR}" add --all

if git -C "${PROJECT_DIR}" ls-files --stage -- \
    skills/ai_skills skills/mxl-skills | awk '$1 == "160000" { found = 1 } END { exit !found }'; then
  die "Company skills Gitlink remains staged; remove it before committing"
fi

if git -C "${PROJECT_DIR}" diff --cached --quiet; then
  die "No staged changes to commit"
fi

echo "Staged changes:"
git -C "${PROJECT_DIR}" diff --cached --stat
echo
echo "Commit message: ${COMMIT_MSG}"

git -C "${PROJECT_DIR}" commit -m "${COMMIT_MSG}"
