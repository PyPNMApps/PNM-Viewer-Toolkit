#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify downloaded native release artifact bytes before installation or upload."""
import argparse
import hashlib
import json
from pathlib import Path

from common import MAX_PACKAGE_BYTES, release_version


def verify(directory):
    """Check manifest, size limits and SHA256SUMS without modifying artifacts.

    directory is one platform's downloaded artifact directory. Require an exact
    package-set match and version agreement; reject unsafe names or empty sets.
    """
    manifest = json.loads((directory / 'release-manifest.json').read_text(encoding='utf-8'))
    if manifest['version'] != release_version():
        raise ValueError('Downloaded artifact version differs from checkout')
    expected = {item['name']: item['bytes'] for item in manifest['artifacts']}
    actual = {}
    for line in (directory / 'SHA256SUMS').read_text(encoding='utf-8').splitlines():
        digest, name = line.split('  ', 1)
        if Path(name).name != name or name in actual:
            raise ValueError('Unsafe or duplicate checksum entry')
        path = directory / name
        size = path.stat().st_size
        if not 0 < size <= MAX_PACKAGE_BYTES:
            raise ValueError(f'Artifact outside size budget: {name}')
        if hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise ValueError(f'Checksum mismatch: {name}')
        actual[name] = size
    if not actual or actual != expected:
        raise ValueError('Artifact manifest/checksum set mismatch')
    return manifest


def main():
    """Verify one platform directory supplied by --packages; fail on drift."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--packages', required=True, type=Path)
    args = parser.parse_args()
    verify(args.packages)
    print('PASS: artifact versions, sizes and checksums')


if __name__ == '__main__':
    main()
