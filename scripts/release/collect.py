#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Assemble all verified platform artifacts into one draft-release asset set."""
import argparse
import json
from pathlib import Path
import shutil

from verify import verify

EXPECTED_TARGETS = {('Linux', 'x64'), ('Windows', 'x64'), ('Darwin', 'arm64'), ('Darwin', 'x64')}


def collect(source, output, allow_partial=False, include_artifacts=None):
    """Verify native artifacts before selecting public installers and checksums.

    source contains one subdirectory per uploaded platform artifact. output must
    be absent. Reject target duplication, missing targets or basename collisions;
    Return the fresh directory with installers, matching checksums and a local
    release index for rendering notes. Reports and CLI archives stay in native
    workflow artifacts; the index is not uploaded to the public release.
    allow_partial explicitly permits a nonempty known subset. include_artifacts
    restricts input to same-run names approved by the platform eligibility gate.
    """
    manifests = []
    if include_artifacts is not None:
        from targets import RUNNERS
        if not include_artifacts or not set(include_artifacts).issubset(RUNNERS):
            raise ValueError('Invalid eligible artifact set')
        if any(not (source / name).is_dir() for name in include_artifacts):
            raise ValueError('Eligible platform artifact missing')
    targets = set()
    names = set()
    for directory in sorted(source.iterdir()):
        if not directory.is_dir():
            continue
        if include_artifacts is not None and directory.name not in include_artifacts:
            continue
        manifest = verify(directory)
        target = (manifest['system'], manifest['architecture'])
        if target in targets:
            raise ValueError('Duplicate platform artifacts')
        targets.add(target)
        for item in manifest['artifacts']:
            if item['name'] in names:
                raise ValueError('Duplicate release asset name')
            names.add(item['name'])
        installers = [item for item in manifest['artifacts']
                      if Path(item['name']).suffix in ('.deb', '.msi', '.dmg')]
        if not installers:
            raise ValueError('Platform has no public installer')
        manifests.append((directory, dict(manifest, artifacts=installers)))
    if not targets or not targets.issubset(EXPECTED_TARGETS) or (not allow_partial and targets != EXPECTED_TARGETS):
        raise ValueError(f'Incomplete platform set: {targets}')
    output.mkdir(exist_ok=False)
    sums = []
    for directory, manifest in manifests:
        installer_names = {item['name'] for item in manifest['artifacts']}
        for line in (directory / 'SHA256SUMS').read_text(encoding='utf-8').splitlines():
            if line.split('  ', 1)[1] in installer_names:
                sums.append(line)
        for name in installer_names:
            shutil.copy2(directory / name, output / name)
    (output / 'SHA256SUMS').write_text('\n'.join(sorted(sums)) + '\n', encoding='utf-8')
    (output / 'release-index.json').write_text(json.dumps([manifest for _, manifest in manifests], indent=2) + '\n', encoding='utf-8')


def main():
    """Collect artifact subdirectories without network or Git operations."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source', required=True, type=Path)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--allow-partial', action='store_true')
    parser.add_argument('--include-artifacts', type=json.loads)
    args = parser.parse_args()
    collect(args.source, args.output, args.allow_partial, args.include_artifacts)


if __name__ == '__main__':
    main()
