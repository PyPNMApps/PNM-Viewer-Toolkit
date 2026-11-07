#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build Ubuntu-baseline DEBs with private Qt libraries and separate CLI ownership."""
import json
import os
from pathlib import Path
import shutil

from common import ROOT, binary_path, inventory, notices, qt, run
from samples import DEMO_DIRECTORY, bundle

SYSTEM_DEPENDENCIES = (
    'libc6 (>= 2.35), libstdc++6, libgcc-s1, libgl1, libegl1, libopengl0, '
    'libdbus-1-3, libfontconfig1, libfreetype6, libx11-6, libx11-xcb1, '
    'libxkbcommon0, libxkbcommon-x11-0, libxcb1, libxcb-cursor0, libxcb-icccm4, '
    'libxcb-image0, libxcb-keysyms1, libxcb-render-util0, libxcb-shape0, '
    'libxcb-randr0, libxcb-xfixes0, libxcb-sync1, libxcb-render0, libxcb-shm0, '
    'libsm6, libice6, libglib2.0-0 | libglib2.0-0t64, '
    'pkexec, libgtk-3-0 | libgtk-3-0t64, libzstd1, zlib1g, libpcre2-16-0, libb2-1'
)


def deploy_qt(payload):
    """Collect QML imports, Qt plugins and their SDK ELF dependencies.

    payload is a fresh private app directory. Use Qt's scanner and ldd recursively;
    missing ELF libraries fail packaging. System libraries remain declared DEB
    prerequisites. Runtime search paths are rewritten relative to each file.
    """
    sdk = qt.sdk_path()
    environment = qt.qt_environment()
    qml = payload / 'qml'
    qml.mkdir()
    result = run([sdk / 'libexec/qmlimportscanner', '-rootPath', ROOT / 'apps/qt/qml',
                  '-importPath', sdk / 'qml'], capture_output=True, text=True, env=environment)
    for module in json.loads(result.stdout):
        source = Path(module.get('path', ''))
        if not source.is_dir() or not source.is_relative_to(sdk / 'qml'):
            continue
        destination = qml / source.relative_to(sdk / 'qml')
        shutil.copytree(source, destination, dirs_exist_ok=True,
                        ignore=shutil.ignore_patterns('*.debug', '*.a', '*.prl'))
    for category in ('platforms', 'platformthemes', 'platforminputcontexts',
                     'imageformats', 'iconengines', 'xcbglintegrations',
                     'networkinformation', 'tls'):
        source = sdk / 'plugins' / category
        if source.exists():
            destination = payload / 'plugins' / category
            destination.mkdir(parents=True)
            for plugin in source.glob('*.so'):
                if category == 'platforms' and plugin.name not in ('libqxcb.so', 'libqoffscreen.so', 'libqminimal.so'):
                    continue
                shutil.copy2(plugin, destination / plugin.name)
    libraries = payload / 'lib'
    libraries.mkdir()
    pending = [path for path in payload.rglob('*') if path.is_file()
               and path.read_bytes()[:4] == b'\x7fELF']
    examined = set()
    while pending:
        path = pending.pop()
        if path in examined:
            continue
        examined.add(path)
        dependencies = run(['ldd', path], env=environment, capture_output=True, text=True).stdout
        if 'not found' in dependencies:
            raise ValueError(f'Unresolved library in {path}: {dependencies}')
        for line in dependencies.splitlines():
            fields = line.split()
            if len(fields) < 3 or fields[1] != '=>':
                continue
            dependency = Path(fields[2])
            if not dependency.is_relative_to(sdk):
                continue
            destination = libraries / fields[0]
            if not destination.exists():
                shutil.copy2(dependency.resolve(), destination)
                pending.append(destination)
        relative_lib = os.path.relpath(libraries, path.parent)
        run(['patchelf', '--set-rpath', '$ORIGIN/' + relative_lib, path])
    (payload / 'qt.conf').write_text('[Paths]\nPrefix=.\nLibraries=lib\nPlugins=plugins\nQmlImports=qml\n', encoding='utf-8')


def control(stage, name, version, dependencies, description):
    """Write DEB control metadata for a fresh stage; no maintainer scripts run."""
    directory = stage / 'DEBIAN'
    directory.mkdir()
    size = sum(path.stat().st_size for path in stage.rglob('*') if path.is_file()) // 1024
    (directory / 'control').write_text(
        f'Package: {name}\nVersion: {version}\nArchitecture: amd64\n'
        'Maintainer: PyPNMApps contributors\nSection: science\nPriority: optional\n'
        f'Installed-Size: {size}\nDepends: {dependencies}\n'
        f'Description: {description}\n', encoding='utf-8')


def build(stage, output, version):
    """Build CLI and viewer DEBs from native release binaries.

    The viewer depends on the same-version CLI package; removing it never owns
    the separately installed CLI files. All Qt files live under a private prefix.
    """
    cli_stage = stage / 'cli-deb'
    cli_bin = cli_stage / 'usr/bin'
    cli_bin.mkdir(parents=True)
    shutil.copy2(binary_path(), cli_bin / 'pnm-decode')
    doc = cli_stage / 'usr/share/doc/pnm-decode'
    notices(doc)
    manual = cli_stage / 'usr/share/man/man1'
    manual.mkdir(parents=True)
    shutil.copy2(ROOT / 'crates/cli/man/pnm-decode.1', manual / 'pnm-decode.1')
    control(cli_stage, 'pnm-decode', version, 'libc6 (>= 2.35), libgcc-s1',
            'Offline DOCSIS PNM decoder (development build)')
    run(['dpkg-deb', '--root-owner-group', '-Zxz', '--build', cli_stage,
         output / f'pnm-decode_{version}_amd64.deb'])
    gui_stage = stage / 'viewer-deb'
    payload = gui_stage / 'opt/pnm-viewer-toolkit'
    payload.mkdir(parents=True)
    shutil.copy2(binary_path(gui=True), payload / 'pnm-viewer-qt')
    deploy_qt(payload)
    notices(payload / 'share')
    bundle(payload / DEMO_DIRECTORY)
    launcher = gui_stage / 'usr/bin'
    launcher.mkdir(parents=True)
    (launcher / 'pnm-viewer').symlink_to('/opt/pnm-viewer-toolkit/pnm-viewer-qt')
    applications = gui_stage / 'usr/share/applications'
    applications.mkdir(parents=True)
    (applications / 'pnm-viewer-toolkit.desktop').write_text(
        '[Desktop Entry]\nType=Application\nName=PNM Viewer Toolkit\n'
        'Exec=pnm-viewer\nIcon=pnm-viewer-toolkit\nTerminal=false\n'
        'StartupWMClass=pnm-viewer-toolkit\nCategories=Science;\n', encoding='utf-8')
    icons = gui_stage / 'usr/share/icons/hicolor/256x256/apps'
    icons.mkdir(parents=True)
    shutil.copy2(ROOT / 'docs/images/desktop-icon-dark.png', icons / 'pnm-viewer-toolkit.png')
    control(gui_stage, 'pnm-viewer-toolkit', version,
            f'pnm-decode (= {version}), {SYSTEM_DEPENDENCIES}',
            'Native Qt DOCSIS PNM viewer (development build)')
    inventory(payload, output / 'linux-installed-files.json')
    run(['dpkg-deb', '--root-owner-group', '-Zxz', '--build', gui_stage,
         output / f'pnm-viewer-toolkit_{version}_amd64.deb'])
