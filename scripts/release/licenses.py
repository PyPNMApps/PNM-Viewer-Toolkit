#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Collect Rust and Qt source license notices for native runtime redistribution."""
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import tarfile
import urllib.request

from common import ROOT, qt, run

QT_MODULES = ('qtbase', 'qtdeclarative', 'qtshadertools', 'qtsvg')
MAX_SOURCE_BYTES = 150_000_000


def download_source(module):
    """Cache a pinned Qt source tag archive locally with a measured SHA-256.

    Verify TLS normally and bound downloads. Sources remain ignored development
    inputs; only notices and source provenance enter the package.
    """
    cache = ROOT / 'local/release-sources'
    cache.mkdir(parents=True, exist_ok=True)
    path = cache / f'{module}-{qt.QT_VERSION}.tar.gz'
    url = f'https://codeload.github.com/qt/{module}/tar.gz/refs/tags/v{qt.QT_VERSION}'
    if not path.exists():
        partial = path.with_suffix('.partial')
        try:
            with urllib.request.urlopen(url, timeout=60) as source, partial.open('wb') as target:
                total = 0
                while True:
                    chunk = source.read(1024 * 1024)
                    if not chunk:
                        break
                    total += len(chunk)
                    if total > MAX_SOURCE_BYTES:
                        raise ValueError('Qt license source archive exceeds download limit')
                    target.write(chunk)
            partial.rename(path)
        finally:
            if partial.exists():
                partial.unlink()
    if path.stat().st_size > MAX_SOURCE_BYTES:
        raise ValueError('Cached Qt source exceeds limit')
    return path, {'module': module, 'version': qt.QT_VERSION, 'source': url,
                  'sha256': hashlib.sha256(path.read_bytes()).hexdigest()}


def qt_notices(destination):
    """Copy Qt license/attribution files and referenced third-party notices.

    Read archive entries only; never extract arbitrary paths or execute source.
    Reject large notice files and paths escaping the archive's source root.
    """
    provenance = []
    for module in QT_MODULES:
        archive, origin = download_source(module)
        provenance.append(origin)
        with tarfile.open(archive) as source:
            members = {item.name: item for item in source.getmembers() if item.isfile()}
            selected = set()
            for name, member in members.items():
                basename = PurePosixPath(name).name.lower()
                if basename.startswith(('license', 'licence', 'copying', 'notice')) or basename == 'qt_attribution.json':
                    selected.add(name)
                if basename == 'qt_attribution.json':
                    # Qt attribution sources permit literal newlines inside strings.
                    data = json.load(source.extractfile(member), strict=False)
                    entries = data if isinstance(data, list) else [data]
                    for entry in entries:
                        files = entry.get('LicenseFile', [])
                        if isinstance(files, str):
                            files = [files]
                        for filename in files:
                            candidate = str(PurePosixPath(name).parent / filename)
                            # Normalize referenced ../ paths without extracting a tree.
                            parts = []
                            for part in PurePosixPath(candidate).parts:
                                if part == '..':
                                    if not parts:
                                        raise ValueError('Escaping Qt notice path')
                                    parts.pop()
                                elif part != '.':
                                    parts.append(part)
                            candidate = '/'.join(parts)
                            if candidate in members:
                                selected.add(candidate)
                            else:
                                raise ValueError(f'Missing referenced Qt license: {candidate}')
            for name in sorted(selected):
                relative = PurePosixPath(name)
                if relative.is_absolute() or '..' in relative.parts:
                    raise ValueError('Unsafe Qt notice path')
                member = members[name]
                if member.size > 8_000_000:
                    raise ValueError('Qt notice file exceeds limit')
                target = destination / 'qt' / Path(*relative.parts)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(source.extractfile(member).read())
    (destination / 'qt-source-provenance.json').write_text(json.dumps(provenance, indent=2) + '\n', encoding='utf-8')


def collect(destination):
    """Collect license files from locked Cargo sources and pinned Qt sources.

    destination is a fresh notice cache. Resolve Cargo package paths from metadata;
    do not copy sources, private captures or dependencies into production runtimes.
    """
    destination.mkdir(parents=True, exist_ok=False)
    components = {}
    for manifest in ('Cargo.toml', 'apps/qt/Cargo.toml'):
        result = run(['cargo', 'metadata', '--locked', '--format-version', '1',
                      '--manifest-path', ROOT / manifest], capture_output=True, text=True,
                     env=qt.qt_environment())
        for package in json.loads(result.stdout)['packages']:
            key = package['name'] + '-' + package['version']
            components[key] = {'type': 'library', 'name': package['name'],
                               'version': package['version'],
                               'licenses': [{'expression': package['license']}] if package.get('license') else [],
                               'purl': f'pkg:cargo/{package["name"]}@{package["version"]}'}
            root = Path(package['manifest_path']).parent
            selected = [path for path in root.iterdir() if path.is_file()
                        and path.name.lower().startswith(('license', 'copying', 'notice'))]
            if package.get('license_file'):
                selected.append(root / package['license_file'])
            for path in selected:
                target = destination / 'cargo' / key / path.name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(path, target)
    qt_notices(destination)
    for module in QT_MODULES:
        components[module] = {'type': 'library', 'name': module, 'version': qt.QT_VERSION,
                              'externalReferences': [{'type': 'vcs', 'url': f'https://github.com/qt/{module}/tree/v{qt.QT_VERSION}'}]}
    (destination / 'bom.cdx.json').write_text(json.dumps({
        'bomFormat': 'CycloneDX', 'specVersion': '1.6', 'version': 1,
        'components': list(components.values())
    }, indent=2) + '\n', encoding='utf-8')
    (destination / 'SOURCE.md').write_text(
        'Build instructions: docs/release/packaging.md in the corresponding project tag.\n'
        'Project source: https://github.com/PyPNMApps/PNM-Viewer-Toolkit\n'
        'Qt source archives and exact SHA-256 digests: qt-source-provenance.json.\n'
        'Cargo source/version/license metadata: bom.cdx.json and the locked dependency graphs.\n', encoding='utf-8')
