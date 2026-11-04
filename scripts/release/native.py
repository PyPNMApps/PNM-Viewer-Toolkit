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


def nsis_escape(value):
    """Quote generated NSIS string content, rejecting command delimiters.

    Input is a trusted local path or release version; reject quotes/newlines
    rather than allowing generated source injection. Escape literal dollar signs.
    """
    text = str(value)
    if any(character in text for character in ('"', '\n', '\r')):
        raise ValueError('Unsupported quote/newline in NSIS path')
    return text.replace('$', '$$')


def windows(stage, output, version):
    """Deploy a private Qt runtime and build a per-user NSIS installer.

    Bundle app-local MSVC runtime DLLs, CLI and manual. Do not mutate PATH or
    claim ownership of any independently installed CLI. Missing deploy/compiler
    tools fail before installer creation. Native installation is a separate test.
    """
    payload = stage / 'windows-payload'
    payload.mkdir()
    gui = payload / 'pnm-viewer.exe'
    shutil.copy2(binary_path(gui=True), gui)
    shutil.copy2(binary_path(), payload / 'pnm-decode.exe')
    notices(payload / 'share')
    run([qt.sdk_path() / 'bin/windeployqt.exe', '--release', '--no-translations',
         '--no-compiler-runtime', '--qmldir', ROOT / 'apps/qt/qml', gui], env=qt.qt_environment())
    redist = os.environ.get('VCToolsRedistDir')
    if not redist:
        raise ValueError('VCToolsRedistDir missing; initialize the MSVC environment')
    candidates = sorted((Path(redist) / 'x64').glob('Microsoft.VC*.CRT'))
    if not candidates:
        raise ValueError('MSVC app-local redistributable DLLs missing')
    for dll in candidates[-1].glob('*.dll'):
        shutil.copy2(dll, payload / dll.name)
    inventory(payload, output / 'windows-installed-files.json')
    artifact = output / f'PNM-Viewer-Toolkit-{version}-windows-x64.exe'
    install_files = []
    delete_files = []
    for path in sorted(payload.rglob('*')):
        if path.is_file():
            relative = path.relative_to(payload)
            parent = str(relative.parent).replace('/', '\\')
            install_files.extend([f'SetOutPath "$INSTDIR\\{nsis_escape(parent)}"',
                                  f'File "{nsis_escape(path)}"'])
            delete_files.append(f'Delete "$INSTDIR\\{nsis_escape(str(relative).replace(chr(47), chr(92)))}"')
    directories = sorted((path.relative_to(payload) for path in payload.rglob('*') if path.is_dir()),
                         key=lambda path: len(path.parts), reverse=True)
    delete_files += [f'RMDir "$INSTDIR\\{nsis_escape(str(path).replace(chr(47), chr(92)))}"' for path in directories]
    script = stage / 'installer.nsi'
    script.write_text(f'''Unicode true
Name "PNM Viewer Toolkit"
OutFile "{nsis_escape(artifact)}"
InstallDir "$LOCALAPPDATA\\Programs\\PNM Viewer Toolkit"
RequestExecutionLevel user
SetCompressor /SOLID lzma
Page directory
Page instfiles
UninstPage uninstConfirm
UninstPage instfiles
Section
{chr(10).join(install_files)}
WriteUninstaller "$INSTDIR\\uninstall.exe"
CreateShortcut "$SMPROGRAMS\\PNM Viewer Toolkit.lnk" "$INSTDIR\\pnm-viewer.exe"
WriteRegStr HKCU "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\PNMViewerToolkit" "DisplayName" "PNM Viewer Toolkit"
WriteRegStr HKCU "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\PNMViewerToolkit" "DisplayVersion" "{version}"
WriteRegStr HKCU "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\PNMViewerToolkit" "UninstallString" '"$INSTDIR\\uninstall.exe"'
SectionEnd
Section "Uninstall"
{chr(10).join(delete_files)}
Delete "$SMPROGRAMS\\PNM Viewer Toolkit.lnk"
DeleteRegKey HKCU "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\PNMViewerToolkit"
Delete "$INSTDIR\\uninstall.exe"
RMDir "$INSTDIR"
SectionEnd
''', encoding='utf-8')
    compiler = shutil.which('makensis') or r'C:\Program Files (x86)\NSIS\makensis.exe'
    run([compiler, '/V3', script])


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
    shutil.copy2(binary_path(), binaries / 'pnm-decode')
    notices(resources / 'share')
    info = {'CFBundleExecutable': 'pnm-viewer', 'CFBundleIdentifier': 'org.pypnm.pnm-viewer-toolkit',
            'CFBundleName': 'PNM Viewer Toolkit', 'CFBundleDisplayName': 'PNM Viewer Toolkit',
            'CFBundlePackageType': 'APPL', 'CFBundleShortVersionString': version,
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
