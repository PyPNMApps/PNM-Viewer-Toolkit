#!/usr/bin/env python3
"""Bump the repository's three-component VERSION without Git mutations."""
import argparse
from pathlib import Path
import re


def bumped(version: str, part: str) -> str:
    if not re.fullmatch(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)", version):
        raise ValueError("VERSION must contain MAJOR.MINOR.PATCH without leading zeros")
    parts = [int(value) for value in version.split(".")]
    index = ("major", "minor", "patch").index(part)
    parts[index] += 1
    parts[index + 1:] = [0] * (2 - index)
    return ".".join(map(str, parts))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bump", required=True, choices=("major", "minor", "patch"))
    parser.add_argument("--dry-run", action="store_true", help="Preview without writing")
    args = parser.parse_args()
    version_file = Path(__file__).resolve().parents[2] / "VERSION"
    try:
        current = version_file.read_text(encoding="utf-8").strip()
        new = bumped(current, args.bump)
        if not args.dry_run:
            version_file.write_text(new + "\n", encoding="utf-8")
    except (OSError, ValueError) as error:
        parser.exit(1, f"ERROR: {error}\n")
    print(f"{current} -> {new}" + (" (dry run)" if args.dry_run else ""))


if __name__ == "__main__":
    main()
