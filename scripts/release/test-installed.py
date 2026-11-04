#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Test native package installation, rendered JSON export, and removal in CI."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tarfile
import zipfile

from common import native_target, run
from smoke import smoke


def one(directory, pattern):
    """Require exactly one artifact matching pattern; reject missing/extra files."""
    paths = list(directory.glob(pattern))
    if len(paths) != 1:
        raise ValueError(f'Expected one {pattern}, found {len(paths)}')
    return paths[0].resolve()


def linux(packages):
    """Install both DEBs, exercise GUI/CLI, remove UI, then remove CLI.

    Intended only for disposable CI hosts with explicit --ci-disposable-host.
    Require same artifact checksums in the workflow before invoking this script.
    """
    gui = one(packages, 'pnm-viewer-toolkit_*.deb')
    cli = one(packages, 'pnm-decode_*.deb')
    installed_gui = Path('/opt/pnm-viewer-toolkit/pnm-viewer-qt')
    try:
        run(['sudo', 'apt-get', 'install', '-y', cli, gui])
        if not Path('/usr/share/applications/pnm-viewer-toolkit.desktop').is_file():
            raise ValueError('Desktop integration missing')
        smoke(installed_gui, Path('/usr/bin/pnm-decode'))
    finally:
        run(['sudo', 'apt-get', 'remove', '-y', 'pnm-viewer-toolkit'])
        if installed_gui.exists() or Path('/usr/bin/pnm-viewer').exists():
            raise ValueError('Viewer removal left installed executable')
        # Separate CLI ownership must survive removing the viewer.
        run(['/usr/bin/pnm-decode', '--version'])
        run(['sudo', 'apt-get', 'remove', '-y', 'pnm-decode'])
        if Path('/usr/bin/pnm-decode').exists():
            raise ValueError('CLI removal left installed executable')


def windows(packages):
    """Silently install NSIS to a fresh temp prefix, smoke-test and uninstall.

    Never touch a user's installed viewer. This script requires the disposable
    CI guard and refuses a preexisting product registration.
    """
    import winreg
    key = r'Software\Microsoft\Windows\CurrentVersion\Uninstall\PNMViewerToolkit'
    try:
        with winreg.OpenKey(winreg.HKEY_CURRENT_USER, key):
            raise ValueError('Existing viewer registration; refusing lifecycle test')
    except FileNotFoundError:
        pass
    installer = one(packages, '*-windows-x64.exe')
    with tempfile.TemporaryDirectory(prefix='pnm-install-') as directory:
        prefix = Path(directory) / 'viewer'
        try:
            run([installer, '/S', f'/D={prefix}'], timeout=120)
            smoke(prefix / 'pnm-viewer.exe', prefix / 'pnm-decode.exe')
            if not (prefix / 'share/pnm-decode.1').is_file():
                raise ValueError('Bundled CLI manual missing')
        finally:
            uninstaller = prefix / 'uninstall.exe'
            if uninstaller.exists():
                run([uninstaller, '/S', f'_?={prefix}'], timeout=120)
            if (prefix / 'pnm-viewer.exe').exists() or (prefix / 'pnm-decode.exe').exists():
                raise ValueError('Uninstall left application executables')
        try:
            with winreg.OpenKey(winreg.HKEY_CURRENT_USER, key):
                raise ValueError('Uninstall left product registration')
        except FileNotFoundError:
            pass


def macos(packages, architecture):
    """Mount a DMG read-only and test a copied app without development Qt paths.

    Copy into a fresh temporary Applications directory, then remove it. No
    developer signing, notarization, global installation or LaunchServices change.
    """
    dmg = one(packages, f'*-macos-{architecture}.dmg')
    with tempfile.TemporaryDirectory(prefix='pnm-dmg-') as directory:
        mount = Path(directory) / 'mounted'
        mount.mkdir()
        run(['hdiutil', 'attach', '-readonly', '-nobrowse', '-mountpoint', mount, dmg])
        app = Path(directory) / 'Applications/PNM Viewer Toolkit.app'
        try:
            run(['ditto', mount / 'PNM Viewer Toolkit.app', app])
        finally:
            run(['hdiutil', 'detach', mount])
        run(['codesign', '--verify', '--deep', '--strict', app])
        smoke(app / 'Contents/MacOS/pnm-viewer', app / 'Contents/MacOS/pnm-decode')
        if not (app / 'Contents/Resources/share/pnm-decode.1').is_file():
            raise ValueError('Bundled CLI manual missing')
        shutil.rmtree(app)
        if app.exists():
            raise ValueError('App removal failed')


def standalone_cli(packages):
    """Extract the standalone archive safely and test its decoder/manual.

    Reject traversal and links even though CI produced the artifact. Extraction
    uses a temporary prefix and never modifies a globally installed CLI.
    """
    archives = list(packages.glob('pnm-decode-*.zip')) + list(packages.glob('pnm-decode-*.tar.gz'))
    if len(archives) != 1:
        raise ValueError('Expected one standalone CLI archive')
    with tempfile.TemporaryDirectory(prefix='pnm-cli-archive-') as directory:
        root = Path(directory)
        archive = archives[0]
        if archive.suffix == '.zip':
            with zipfile.ZipFile(archive) as source:
                members = source.infolist()
                if sum(member.file_size for member in members) > 200_000_000:
                    raise ValueError('CLI archive expansion exceeds limit')
                entries = [(member.filename, source.read(member)) for member in members if not member.is_dir()]
        else:
            with tarfile.open(archive) as source:
                entries = []
                expanded_bytes = 0
                for member in source:
                    if member.isdir():
                        continue
                    if not member.isfile():
                        raise ValueError('Unexpected link in CLI archive')
                    expanded_bytes += member.size
                    if expanded_bytes > 200_000_000:
                        raise ValueError('CLI archive expansion exceeds limit')
                    entries.append((member.name, source.extractfile(member).read()))
        for name, data in entries:
            relative = Path(name)
            if relative.is_absolute() or '..' in relative.parts:
                raise ValueError('Unsafe CLI archive path')
            target = root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            if target.name == 'pnm-decode':
                target.chmod(0o755)
        suffix = '.exe' if os.name == 'nt' else ''
        binary = one(root, '*/pnm-decode' + suffix)
        if not (binary.parent / 'share/pnm-decode.1').is_file():
            raise ValueError('Standalone CLI manual missing')
        output = root / 'rxmer.json'
        from common import ROOT
        from smoke import clean_environment
        environment = clean_environment()
        run([binary, '--version'], env=environment)
        run([binary, ROOT / 'tests/fixtures/pypnm/tests/files/rxmer.bin', '--output', output], env=environment)
        import json
        if not json.loads(output.read_text(encoding="utf-8")):
            raise ValueError('Standalone CLI produced empty JSON')


def main():
    """Require a disposable GitHub runner before system-changing package tests."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--packages', required=True, type=Path)
    parser.add_argument('--ci-disposable-host', action='store_true')
    args = parser.parse_args()
    if not args.ci_disposable_host or os.environ.get('GITHUB_ACTIONS') != 'true':
        parser.error('Installation requires a disposable GitHub Actions runner')
    standalone_cli(args.packages)
    system, architecture = native_target()
    if system == 'Linux':
        linux(args.packages)
    elif system == 'Windows':
        windows(args.packages)
    else:
        macos(args.packages, architecture)


if __name__ == '__main__':
    main()
