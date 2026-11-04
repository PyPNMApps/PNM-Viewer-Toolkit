#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Bump the repository's three-component project version without Git mutations."""
import argparse
from pathlib import Path
from versions import synchronize, validate


def bumped(version: str, part: str) -> str:
    """Return a bumped three-component version without modifying files.

    Args:
        version: Whole numeric MAJOR.MINOR.PATCH string without leading zeros.
        part: Component to increment: major, minor or patch.
    Returns:
        Incremented version with lower components reset to zero.
    Raises:
        ValueError: Version syntax or component name is invalid.
    """
    validate(version)
    parts = [int(value) for value in version.split(".")]
    index = ("major", "minor", "patch").index(part)
    parts[index] += 1
    parts[index + 1:] = [0] * (2 - index)
    return ".".join(map(str, parts))


def main() -> None:
    """Parse CLI options and update project versions unless dry-run is requested.

    Uses paths relative to this script; prints the old/new versions.
    Returns None on success. Argument failures exit with status 2; read,
    validation or write failures exit with status 1. No Git actions occur.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bump", required=True, choices=("major", "minor", "patch"))
    parser.add_argument("--dry-run", action="store_true", help="Preview without writing")
    args = parser.parse_args()
    version_file = Path(__file__).resolve().parents[2] / "VERSION"
    try:
        current = version_file.read_text(encoding="utf-8").strip()
        new = bumped(current, args.bump)
        changes = synchronize(version_file.parent, new)
        if not args.dry_run:
            synchronize(version_file.parent, new, write=True)
    except (OSError, ValueError) as error:
        parser.exit(1, f"ERROR: {error}\n")
    print(f"{current} -> {new}" + (" (dry run)" if args.dry_run else ""))
    for name in ["VERSION", *changes]:
        print(f"  {name}")


if __name__ == "__main__":
    main()
