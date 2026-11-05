#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Test native package installation, rendered JSON export, and removal in CI."""
import argparse
import hashlib
import json
import os
import uuid
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
        desktop_entry = Path('/usr/share/applications/pnm-viewer-toolkit.desktop')
        if not desktop_entry.is_file():
            raise ValueError('Desktop integration missing')
        launcher = desktop_entry.read_text(encoding='utf-8').splitlines()
        for field in ('Exec=pnm-viewer', 'Icon=pnm-viewer-toolkit',
                      'StartupWMClass=pnm-viewer-toolkit'):
            if launcher.count(field) != 1:
                raise ValueError(f'Desktop integration missing or duplicated: {field}')
        smoke(installed_gui, Path('/usr/bin/pnm-decode'))
        linux_upgrade(cli, gui, installed_gui)
    finally:
        run(['sudo', 'apt-get', 'remove', '-y', 'pnm-viewer-toolkit'])
        if installed_gui.exists() or Path('/usr/bin/pnm-viewer').exists():
            raise ValueError('Viewer removal left installed executable')
        # Separate CLI ownership must survive removing the viewer.
        run(['/usr/bin/pnm-decode', '--version'])
        run(['sudo', 'apt-get', 'remove', '-y', 'pnm-decode'])
        if Path('/usr/bin/pnm-decode').exists():
            raise ValueError('CLI removal left installed executable')


def linux_upgrade(cli, gui, installed_gui):
    """Exercise the real root installer from a synthetic older control version.

    Runs only inside the existing disposable-host lifecycle gate. The baseline
    uses this build's payload with an older DEB control version, not a claimed
    real historical binary. Test both-package upgrade and same-version refusal.
    No desktop password prompt is automated; VM Polkit testing is separate.
    """
    version = json.loads((cli.parent / 'release-manifest.json').read_text())['version']
    with tempfile.TemporaryDirectory(prefix='pnm-upgrade-fixture-') as workspace:
        root = Path(workspace)
        older = []
        for package in (cli, gui):
            stage = root / package.stem
            run(['dpkg-deb', '-R', package, stage])
            control = stage / 'DEBIAN/control'
            control.write_text(control.read_text().replace(version, version + '~upgrade-test'))
            output = root / package.name
            run(['dpkg-deb', '--root-owner-group', '-Znone', '--build', stage, output])
            older.append(output)
        run(['sudo', 'apt-get', 'install', '-y', '--allow-downgrades', *older])
        with tempfile.TemporaryDirectory(prefix='pnm-viewer-update-') as directory:
            downloads = Path(directory)
            plan = {'version': version, 'packages': []}
            for package in (cli, gui):
                shutil.copy2(package, downloads / package.name)
                plan['packages'].append({
                    'name': package.name, 'size': package.stat().st_size,
                    'sha256': hashlib.sha256(package.read_bytes()).hexdigest(),
                    'url': f'https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases/download/v{version}/{package.name}'})
            (downloads / 'plan.json').write_text(json.dumps(plan))
            command = ['sudo', 'env', f'PKEXEC_UID={os.getuid()}', installed_gui,
                       '--install-update', downloads]
            # Reject tampered bytes through the real privileged entry point.
            first = downloads / cli.name
            original = first.read_bytes()
            first.write_bytes(original[:-1] + bytes([original[-1] ^ 1]))
            rejected = subprocess.run([str(item) for item in command], capture_output=True, text=True)
            if rejected.returncode == 0 or 'SHA-256 verification failed' not in rejected.stderr:
                raise ValueError('Updater did not reject a corrupted package by hash')
            first.write_bytes(original)
            run(command)
            for package in ('pnm-decode', 'pnm-viewer-toolkit'):
                installed = run(['dpkg-query', '-W', '-f=${Version}', package],
                                capture_output=True, text=True).stdout
                if installed != version:
                    raise ValueError('Paired upgrade did not install the expected version')
            rejected = subprocess.run([str(item) for item in command], capture_output=True, text=True)
            if rejected.returncode == 0 or 'newer than both installed packages' not in rejected.stderr:
                raise ValueError('Updater did not reject a same-version installation')
            smoke(installed_gui, Path('/usr/bin/pnm-decode'))


def windows(packages, payload):
    """Exercise MSI install/major-upgrade/downgrade refusal/removal on disposable CI.

    payload is the freshly packaged image used to author a synthetic 0.0.0
    baseline with the current binaries. This verifies installer transitions,
    not a historical application's behavior or interactive Windows 11 prompts.
    """
    import winreg
    from windows_msi import build
    registry_path = r'Software\Microsoft\Windows\CurrentVersion\Uninstall'

    def registrations():
        with winreg.OpenKey(winreg.HKEY_LOCAL_MACHINE, registry_path, 0,
                            winreg.KEY_READ | winreg.KEY_WOW64_64KEY) as registry:
            found = []
            for index in range(winreg.QueryInfoKey(registry)[0]):
                name = winreg.EnumKey(registry, index)
                with winreg.OpenKey(registry, name) as product:
                    try:
                        title = winreg.QueryValueEx(product, 'DisplayName')[0]
                    except FileNotFoundError:
                        continue
                    if title == 'PNM Viewer Toolkit':
                        uuid.UUID(name.strip('{}'))
                        found.append((name, winreg.QueryValueEx(product, 'DisplayVersion')[0]))
            return found

    def execute(arguments, logfile, success=True):
        result = subprocess.run(['msiexec.exe', *map(str, arguments), '/qn', '/norestart',
                                 '/L*V', str(logfile)], cwd=packages, capture_output=True,
                                text=True, timeout=180)
        accepted = result.returncode in (0, 3010)
        if not success:
            accepted = result.returncode == 1603
        if not accepted:
            if logfile.is_file():
                print(logfile.read_text(encoding='utf-16', errors='replace'), flush=True)
            raise ValueError(f'Windows Installer exit {result.returncode}; log: {logfile}')

    if registrations():
        raise ValueError('Existing viewer MSI registration; refusing lifecycle test')
    if payload is None or not payload.is_dir():
        raise ValueError('Windows lifecycle requires the fresh packaging payload')
    installer = one(packages, '*-windows-x64.msi')
    version = json.loads((packages / 'release-manifest.json').read_text())['version']
    logs = Path(__file__).resolve().parents[2] / 'local/windows-installer-logs'
    logs.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='pnm-msi-') as directory:
        work = Path(directory)
        prefix = work / 'viewer'
        baseline = work / 'baseline.msi'
        build(payload, work, baseline, '0.0.0')
        try:
            execute(['/i', baseline, f'INSTALLFOLDER={prefix}'], logs / 'baseline.log')
            if len(registrations()) != 1 or registrations()[0][1] != '0.0.0':
                raise ValueError('MSI baseline registration missing')
            execute(['/i', installer, f'INSTALLFOLDER={prefix}'], logs / 'upgrade.log')
            if len(registrations()) != 1 or registrations()[0][1] != version:
                raise ValueError('MSI major upgrade did not replace baseline registration')
            smoke(prefix / 'pnm-viewer.exe', prefix / 'pnm-decode.exe')
            if not (prefix / 'share/pnm-decode.1').is_file():
                raise ValueError('Bundled CLI manual missing')
            execute(['/i', baseline, f'INSTALLFOLDER={prefix}'], logs / 'downgrade.log', success=False)
            if len(registrations()) != 1 or registrations()[0][1] != version:
                raise ValueError('Rejected downgrade changed installed version')
        finally:
            for product_code, _version in registrations():
                execute(['/x', product_code], logs / 'uninstall.log')
        if registrations() or (prefix / 'pnm-viewer.exe').exists() or (prefix / 'pnm-decode.exe').exists():
            raise ValueError('MSI uninstall left application files or registration')


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
    parser.add_argument('--windows-payload', type=Path, help='Fresh image for synthetic MSI upgrade tests')
    parser.add_argument('--ci-disposable-host', action='store_true')
    args = parser.parse_args()
    if not args.ci_disposable_host or os.environ.get('GITHUB_ACTIONS') != 'true':
        parser.error('Installation requires a disposable GitHub Actions runner')
    standalone_cli(args.packages)
    system, architecture = native_target()
    if system == 'Linux':
        linux(args.packages)
    elif system == 'Windows':
        windows(args.packages.resolve(), args.windows_payload.resolve() if args.windows_payload else None)
    else:
        macos(args.packages, architecture)


if __name__ == '__main__':
    main()
