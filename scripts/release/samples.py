#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check expanded demo sources, then generate and bundle installer examples."""
import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import subprocess
import sys
import tempfile
import zipfile

from common import ROOT

CAPTURE_FILES = ('rxmer.bin', 'spectrum_analyzer.bin', 'channel_estimation.bin',
                 'histogram.bin', 'fec_summary.bin', 'modulation_profile.bin',
                 'const_display.bin', 'us_pre_equalizer_coef.bin',
                 'us_pre_equalizer_coef_last.bin')
DEMO_ROOT = ROOT / 'tests/fixtures/demos'
DEMO_MANIFEST = DEMO_ROOT / 'installer-demos.json'
HYGIENE_TOOL = ROOT / 'tools/security/pnm-mac-updater.py'
MAX_DEMO_BYTES = 16 * 1024 * 1024


def digest(path):
    """Return a SHA-256 digest for one bounded, regular sample resource.

    Reject symlinks and oversized resources before reading. Sources and generated
    ZIPs share this limit, keeping demo packaging within its explicit budget.
    """
    if path.is_symlink() or not path.is_file() or path.stat().st_size > MAX_DEMO_BYTES:
        raise ValueError('Invalid or oversized example resource')
    return hashlib.sha256(path.read_bytes()).hexdigest()


def relative_path(value):
    """Validate a portable manifest path before joining it to a trusted root.

    Reject parent traversal, absolute paths, Windows separators and drive names.
    Returns a POSIX path for consistent ZIP/installer names across platforms.
    """
    path = PurePosixPath(value)
    if not value or path.is_absolute() or '..' in path.parts or '\\' in value or ':' in value or str(path) != value:
        raise ValueError('Unsafe demo manifest path')
    return path


def check_sources():
    """Validate every allowlisted expanded source before generating any ZIP.

    Run the shared MAC checks read-only. Require exact manifests, hashes and
    regular files; refuse prebuilt archives or extra/private files in demo roots.
    Returns the reviewed dataset descriptors. Failures leave sources unchanged.
    """
    manifest = json.loads(DEMO_MANIFEST.read_text(encoding='utf-8'))
    if manifest.get('schema_version') != 1:
        raise ValueError('Unsupported installer demo manifest')
    datasets = manifest['datasets']
    outputs = set()
    total = 0
    for entry in datasets:
        source_name = relative_path(entry['source'])
        output = relative_path(entry['output'])
        directory = DEMO_ROOT / source_name
        if directory.is_symlink() or not directory.is_dir() or len(source_name.parts) != 1:
            raise ValueError('Invalid expanded demo source')
        if str(output) in outputs or output.parts[0] not in ('rxmer', 'spectrum', 'us-pre-eq'):
            raise ValueError('Duplicate or invalid demo output')
        outputs.add(str(output))
        if entry['archive'] != str(output).endswith('.zip'):
            raise ValueError('Demo output/archive type mismatch')
        expected = set()
        for item in entry['files']:
            name = str(relative_path(item['path']))
            if name in expected or Path(name).suffix not in ('.bin', '.md', '.json', ''):
                raise ValueError('Duplicate or unsupported expanded demo file')
            expected.add(name)
            path = directory / name
            # Refuse redirected parent components as well as redirected files.
            if any(parent.is_symlink() for parent in path.parents if parent != DEMO_ROOT):
                raise ValueError('Demo source parents must not be symlinks')
            if digest(path) != item['sha256'] or path.stat().st_size != item['size_bytes']:
                raise ValueError('Expanded demo checksum mismatch')
            total += path.stat().st_size
        actual = set()
        for path in directory.rglob('*'):
            if path.is_symlink():
                raise ValueError('Demo source must not contain symlinks')
            if path.is_file():
                actual.add(path.relative_to(directory).as_posix())
        if actual != expected or not any(name.endswith('.bin') for name in expected):
            raise ValueError('Missing or unregistered expanded demo files')
    if total > MAX_DEMO_BYTES:
        raise ValueError('Expanded installer demos exceed total size budget')
    subprocess.run([sys.executable, str(HYGIENE_TOOL)], check=True, cwd=ROOT)
    for entry in datasets:
        subprocess.run([sys.executable, str(HYGIENE_TOOL), '--directory',
                        str(DEMO_ROOT / entry['source'])], check=True, cwd=ROOT)
    return datasets


def create_archive(destination, directory, files):
    """Generate a deterministic ZIP from already checked expanded source bytes.

    Preserve capture contents, sort member paths, and use fixed timestamps and
    permissions. Refuse existing output; never unzip or reuse a source archive.
    """
    destination.parent.mkdir(parents=True, exist_ok=True)
    with zipfile.ZipFile(destination, 'x', compression=zipfile.ZIP_DEFLATED, compresslevel=9) as archive:
        for item in sorted(files, key=lambda item: item['path']):
            path = directory / item['path']
            data = path.read_bytes()
            if hashlib.sha256(data).hexdigest() != item['sha256']:
                raise ValueError('Demo changed after hygiene validation')
            info = zipfile.ZipInfo(item['path'], date_time=(1980, 1, 1, 0, 0, 0))
            info.compress_type = zipfile.ZIP_DEFLATED
            info.create_system = 3
            info.external_attr = 0o100644 << 16
            archive.writestr(info, data, compresslevel=9)


def bundle(destination):
    """Check sources before building a fresh native viewer examples directory.

    Used by DEB, MSI and DMG builders. Sources are read-only; failed checks stop
    packaging before any ZIP is written. CLI-only archives never call this path.
    Stage the complete bundle privately and rename it into place on success.
    """
    if destination.exists():
        raise FileExistsError(destination)
    datasets = check_sources()
    source = ROOT / 'tests/fixtures/pypnm'
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='pnm-examples-', dir=destination.parent) as temporary:
        payload = Path(temporary) / 'examples'
        singles = payload / 'single'
        singles.mkdir(parents=True)
        for name in CAPTURE_FILES:
            shutil.copy2(source / 'tests/files' / name, singles / name)
        for name in ('LICENSE', 'NOTICE', 'provenance.json'):
            shutil.copy2(source / name, singles / name)
        (singles / 'README.md').write_text(
            '# Single captures\n\nSanitized PyPNM fixtures for offline exploration. '
            'CM/CMTS MAC addresses are 00:ca:00:00:00:01. '
            'LICENSE, NOTICE and provenance.json retain upstream terms and hashes.\n', encoding='utf-8')
        for entry in datasets:
            directory = DEMO_ROOT / entry['source']
            output = payload / entry['output']
            if entry['archive']:
                create_archive(output, directory, entry['files'])
            else:
                output.mkdir(parents=True)
                for item in entry['files']:
                    target = output / item['path']
                    target.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(directory / item['path'], target)
        (payload / 'README.md').write_text(
            '# Example PNM captures\n\n'
            '- single/: nine captures; open with the main Open action.\n'
            '- rxmer/: ZIP collections for Min-Avg-Max and Playback.\n'
            '- spectrum/: explicitly synthetic standard PNN9 temporal demo.\n'
            '- us-pre-eq/comparison/: original.bin and last-update.bin; use Compare.\n\n'
            'All captures are sanitized before ZIP generation. Distinct modems use '
            'sequential 00:ca:00 aliases. Installed examples are read-only; export '
            'to your own writable folder. No network or modem is needed.\n', encoding='utf-8')
        inventory = [{'path': path.relative_to(payload).as_posix(), 'sha256': digest(path)}
                     for path in sorted(payload.rglob('*')) if path.is_file()]
        (payload / 'manifest.json').write_text(json.dumps({'files': inventory}, indent=2) + '\n', encoding='utf-8')
        payload.rename(destination)


def installed_directory(gui):
    """Verify installed examples in Windows/Linux and macOS application layouts.

    Check single capture bytes, bundle hashes and generated ZIP members against
    expanded repository sources. Verification reads ZIPs without extracting or
    rewriting them. Return the examples root, raising on absent/corrupt resources.
    """
    candidates = (gui.parent / 'share/examples',
                  gui.parent.parent / 'Resources/share/examples')
    for directory in candidates:
        if not directory.is_dir():
            continue
        for name in CAPTURE_FILES:
            expected = ROOT / 'tests/fixtures/pypnm/tests/files' / name
            if digest(directory / 'single' / name) != digest(expected):
                raise ValueError(f'Bundled example missing or modified: {name}')
        inventory = json.loads((directory / 'manifest.json').read_text(encoding='utf-8'))['files']
        for item in inventory:
            if digest(directory / str(relative_path(item['path']))) != item['sha256']:
                raise ValueError('Bundled example inventory mismatch')
        for entry in json.loads(DEMO_MANIFEST.read_text(encoding='utf-8'))['datasets']:
            output = directory / entry['output']
            if entry['archive']:
                with zipfile.ZipFile(output) as archive:
                    names = archive.namelist()
                    if sorted(names) != sorted(item['path'] for item in entry['files']):
                        raise ValueError('Bundled demo ZIP member mismatch')
                    for item in entry['files']:
                        info = archive.getinfo(item['path'])
                        if info.file_size != item['size_bytes'] or info.file_size > MAX_DEMO_BYTES:
                            raise ValueError('Bundled demo ZIP member size mismatch')
                        if hashlib.sha256(archive.read(info)).hexdigest() != item['sha256']:
                            raise ValueError('Bundled demo ZIP checksum mismatch')
            else:
                for item in entry['files']:
                    if digest(output / item['path']) != item['sha256']:
                        raise ValueError('Bundled comparison checksum mismatch')
        return directory
    raise ValueError('Installed examples folder missing')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument('--check', action='store_true', help='Check expanded installer sources without generating ZIPs')
    modes.add_argument('--output', type=Path, help='Build a fresh examples directory after checks pass')
    args = parser.parse_args()
    if args.check:
        check_sources()
    else:
        bundle(args.output)
