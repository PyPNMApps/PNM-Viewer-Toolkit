#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native Windows installer and macOS app/DMG deployment."""
import os
from pathlib import Path
import plistlib
import shutil

from common import ROOT, binary_path, inventory, notices, qt, run
from samples import bundle


def windows(stage, output, version):
    """Deploy app-local Qt/MSVC and build a native per-machine Windows MSI.

    Bundle the CLI/manual without changing PATH. Windows Installer owns upgrade,
    repair and uninstall; no elevated application/custom action is installed.
    """
    from windows_msi import build
    payload = stage / 'windows-payload'
    payload.mkdir()
    gui = payload / 'pnm-viewer.exe'
    shutil.copy2(binary_path(gui=True), gui)
    shutil.copy2(binary_path(), payload / 'pnm-decode.exe')
    notices(payload / 'share')
    bundle(payload / 'share/examples')
    run([qt.sdk_path() / 'bin/windeployqt.exe', '--release', '--no-translations',
         '--no-compiler-runtime', '--qmldir', ROOT / 'apps/qt/qml', gui], env=qt.qt_environment())
    if not (payload / 'platforms/qwindows.dll').is_file():
        raise ValueError('Windows native platform/dialog plugin missing')
    redist = os.environ.get('VCToolsRedistDir')
    if not redist:
        raise ValueError('VCToolsRedistDir missing; initialize the MSVC environment')
    candidates = sorted((Path(redist) / 'x64').glob('Microsoft.VC*.CRT'))
    if not candidates:
        raise ValueError('MSVC app-local redistributable DLLs missing')
    for dll in candidates[-1].glob('*.dll'):
        shutil.copy2(dll, payload / dll.name)
    inventory(payload, output / 'windows-installed-files.json')
    build(payload, stage, output / f'PNM-Viewer-Toolkit-{version}-windows-x64.msi', version)


def macos(stage, output, version, architecture):
    """Deploy/sign an app bundle with Qt's tool and create a compressed DMG.

    Ad-hoc signing is local integrity only, not Apple Developer signing or
    notarization. Include a private CLI/manual and keep separate CLI installs
    independent. Native mounted-image and copied-app checks run separately.
    """
    volume = stage / 'macos-volume'
    app = volume / 'PNM Viewer Toolkit.app'
    contents = app / 'Contents'
    binaries = contents / 'MacOS'
    binaries.mkdir(parents=True)
    gui = binaries / 'pnm-viewer'
    shutil.copy2(binary_path(gui=True), gui)
    resources = contents / 'Resources'
    resources.mkdir()
    shutil.copy2(ROOT / 'packaging/macos/pnm-viewer.icns', resources / 'pnm-viewer.icns')
    shutil.copy2(binary_path(), binaries / 'pnm-decode')
    notices(resources / 'share')
    bundle(resources / 'share/examples')
    info = {'CFBundleExecutable': 'pnm-viewer', 'CFBundleIdentifier': 'org.pypnm.pnm-viewer-toolkit',
            'CFBundleName': 'PNM Viewer Toolkit', 'CFBundleDisplayName': 'PNM Viewer Toolkit',
            'CFBundlePackageType': 'APPL', 'CFBundleIconFile': 'pnm-viewer.icns',
            'CFBundleShortVersionString': version,
            'CFBundleVersion': version, 'NSHighResolutionCapable': True,
            'LSMinimumSystemVersion': os.environ.get('MACOSX_DEPLOYMENT_TARGET', '13.0')}
    (contents / 'Info.plist').write_bytes(plistlib.dumps(info))
    run([qt.sdk_path() / 'bin/macdeployqt', app, f'-qmldir={ROOT / "apps/qt/qml"}',
         '-always-overwrite'], env=qt.qt_environment())
    # The Qt SDK is universal; keep only the offered native architecture before
    # ad-hoc signing so each DMG is genuinely architecture-specific.
    slice_name = 'arm64' if architecture == 'arm64' else 'x86_64'
    for path in app.rglob('*'):
        if not path.is_file() or path.is_symlink():
            continue
        with path.open('rb') as binary:
            magic = binary.read(4)
        if magic not in (bytes.fromhex('cafebabe'), bytes.fromhex('cafebabf'),
                         bytes.fromhex('bebafeca'), bytes.fromhex('bfbafeca')):
            continue
        slices = run(['lipo', '-archs', path], capture_output=True, text=True).stdout.split()
        if slice_name not in slices:
            raise ValueError(f'Deployed runtime lacks {slice_name}: {path}')
        if len(slices) > 1:
            temporary = path.with_name(path.name + '.thin')
            run(['lipo', path, '-thin', slice_name, '-output', temporary])
            shutil.copystat(path, temporary)
            temporary.replace(path)
    run(['codesign', '--force', '--deep', '--sign', '-', app])
    run(['codesign', '--verify', '--deep', '--strict', app])
    inventory(app, output / f'macos-{architecture}-installed-files.json')
    (volume / 'Applications').symlink_to('/Applications')
    run(['hdiutil', 'create', '-volname', 'PNM Viewer Toolkit', '-srcfolder', volume,
         '-format', 'UDZO', output / f'PNM-Viewer-Toolkit-{version}-macos-{architecture}.dmg'])
