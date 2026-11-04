#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Shared native release paths, subprocesses and artifact contracts."""
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts'))
sys.path.insert(0, str(ROOT / 'tools/release'))
import importlib
qt = importlib.import_module('qt-sdk')
from versions import synchronize, validate

MAX_PACKAGE_BYTES = 50_000_000
DEPENDENCY_NOTICES = None


def run(command, **kwargs):
    """Run an argument list from the checkout and propagate subprocess errors."""
    return subprocess.run([str(item) for item in command], cwd=ROOT, check=True, **kwargs)


def release_version():
    """Read canonical VERSION and reject stale manifests before any build."""
    version = validate((ROOT / 'VERSION').read_text(encoding='utf-8').strip())
    if synchronize(ROOT, version):
        raise ValueError('Version metadata differs; synchronize before packaging')
    return version


def native_target():
    """Return the supported native system/architecture or reject this host."""
    system, machine = platform.system(), platform.machine().lower()
    if system in ('Linux', 'Windows') and machine in ('x86_64', 'amd64'):
        return system, 'x64'
    if system == 'Darwin' and machine in ('arm64', 'x86_64'):
        return system, 'arm64' if machine == 'arm64' else 'x64'
    raise ValueError(f'Unsupported release host: {system}/{machine}')


def binary_path(gui=False):
    """Locate the release executable using Cargo's metadata target directory."""
    manifest = ROOT / ('apps/qt/Cargo.toml' if gui else 'Cargo.toml')
    result = run(['cargo', 'metadata', '--no-deps', '--format-version', '1',
                  '--manifest-path', manifest], capture_output=True, text=True)
    target = Path(json.loads(result.stdout)['target_directory'])
    suffix = '.exe' if os.name == 'nt' else ''
    return target / 'release' / (('pnm-viewer-qt' if gui else 'pnm-decode') + suffix)


def notices(destination):
    """Copy applicable project license texts, notices and CLI manual/docs.

    Do not copy development fixtures, private evidence or source-only toolchains.
    """
    destination.mkdir(parents=True, exist_ok=True)
    shutil.copytree(ROOT / 'LICENSES', destination / 'LICENSES')
    for filename in ('LICENSE', 'NOTICE'):
        shutil.copy2(ROOT / filename, destination / filename)
    shutil.copy2(ROOT / 'crates/cli/man/pnm-decode.1', destination / 'pnm-decode.1')
    shutil.copy2(ROOT / 'docs/cli/usage.md', destination / 'cli-usage.md')
    if DEPENDENCY_NOTICES is not None:
        shutil.copytree(DEPENDENCY_NOTICES, destination / 'dependency-licenses')


def inventory(directory, output):
    """Write a payload inventory with SHA-256/bytes and measured installed size.

    Inventory every regular file after native deployment. This is a file-level
    SBOM supplement; Cargo's locked dependency graph is recorded separately.
    """
    files = []
    for path in sorted(directory.rglob('*')):
        if path.is_file():
            files.append({'path': path.relative_to(directory).as_posix(),
                          'bytes': path.stat().st_size,
                          'sha256': hashlib.sha256(path.read_bytes()).hexdigest()})
    output.write_text(json.dumps({'installed_bytes': sum(item['bytes'] for item in files),
                                  'files': files}, indent=2) + '\n', encoding='utf-8')


def verify_artifacts(output):
    """Require nonempty native packages within budget; write their SHA256SUMS.

    Return measured artifacts. Every primary installer/archive has the same
    50,000,000-byte cap; metadata does not exempt oversized packages.
    """
    artifacts = sorted(path for path in output.iterdir()
                       if path.suffix in ('.deb', '.exe', '.dmg', '.zip', '.gz'))
    if not artifacts:
        raise ValueError('No native package artifacts produced')
    checksums = []
    for path in artifacts:
        size = path.stat().st_size
        if not 0 < size <= MAX_PACKAGE_BYTES:
            raise ValueError(f'Package size outside budget: {path.name}: {size} bytes')
        checksums.append(f'{hashlib.sha256(path.read_bytes()).hexdigest()}  {path.name}\n')
        print(f'Package: {path.name}: {size} bytes', flush=True)
    (output / 'SHA256SUMS').write_text(''.join(checksums), encoding='utf-8')
    return artifacts
