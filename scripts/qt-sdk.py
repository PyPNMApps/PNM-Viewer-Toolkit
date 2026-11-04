#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Set up or check the pinned native Qt SDK on Linux, Windows and macOS."""
import argparse
import os
from pathlib import Path
import platform
import subprocess
import sys
import venv

ROOT = Path(__file__).resolve().parents[1]
QT_VERSION = '6.8.3'
AQT_VERSION = '3.3.0'


def sdk_layout():
    """Return native aqt host, architecture and SDK subdirectory.

    Reject unsupported platforms/architectures before downloads or writes.
    macOS's clang_64 SDK is universal; Cargo builds the runner's native target.
    """
    system, machine = platform.system(), platform.machine().lower()
    if system == 'Linux' and machine == 'x86_64':
        return 'linux', 'linux_gcc_64', 'gcc_64'
    if system == 'Windows' and machine in ('amd64', 'x86_64'):
        return 'windows', 'win64_msvc2022_64', 'msvc2022_64'
    if system == 'Darwin' and machine in ('arm64', 'x86_64'):
        return 'mac', 'clang_64', 'macos'
    raise ValueError(f'Unsupported Qt host: {system}/{machine}')


def sdk_path():
    """Resolve an explicit SDK override or the native repository-local SDK."""
    return Path(os.environ.get('QT_SDK_DIR', ROOT / 'local/qt' / QT_VERSION / sdk_layout()[2])).resolve()


def msvc_environment(environment):
    """Select the native x64 MSVC tools in a child-process environment.

    environment is a copied environment dictionary; only that dictionary is
    modified. Resolve VCToolsInstallDir case-insensitively because Windows Python
    uppercases environment keys. Put MSVC before Git Bash's unrelated link.exe
    and set Cargo's default x64 linker to its absolute path. Preserve an explicit
    Cargo linker override. Raise ValueError for an uninitialized/incomplete x64
    developer environment before starting any native builds.
    """
    tools = next((value for key, value in environment.items()
                  if key.upper() == 'VCTOOLSINSTALLDIR'), None)
    if not tools:
        raise ValueError('VCToolsInstallDir missing; initialize an x64 MSVC developer environment')
    binaries = Path(tools) / 'bin' / 'Hostx64' / 'x64'
    for name in ('cl.exe', 'link.exe', 'lib.exe'):
        if not (binaries / name).is_file():
            raise ValueError(f'MSVC x64 tool missing: {binaries / name}')
    environment['PATH'] = str(binaries) + os.pathsep + environment.get('PATH', '')
    environment.setdefault('CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER', str(binaries / 'link.exe'))


def export_msvc_linker():
    """Persist the validated Cargo linker for subsequent GitHub Actions steps.

    Read the initialized MSVC environment and append its absolute default linker
    (or an explicit override) to GITHUB_ENV. No Qt SDK is required. Only the job's
    environment file changes; the calling process and system PATH stay untouched.
    Raise ValueError for missing metadata, tools, or unsafe multiline values;
    propagate filesystem errors before subsequent Cargo commands can run.
    """
    environment = os.environ.copy()
    msvc_environment(environment)
    destination = environment.get('GITHUB_ENV')
    if not destination:
        raise ValueError('GITHUB_ENV missing; this action requires GitHub Actions')
    linker_key = 'CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER'
    linker = environment[linker_key]
    if any(character in linker for character in ('\r', '\n', '\0')):
        raise ValueError('Cargo linker must be a single-line environment value')
    with Path(destination).open('a', encoding='utf-8', newline='\n') as output:
        output.write(f'{linker_key}={linker}\n')


def qt_environment():
    """Return a subprocess environment for native Qt/CXX-Qt builds.

    SDK paths affect child processes only; user profiles and global tools stay
    unchanged. Missing SDK files cause a clear error before launching Cargo.
    """
    sdk = sdk_path()
    suffix = '.exe' if os.name == 'nt' else ''
    qmake = sdk / 'bin' / ('qmake' + suffix)
    if not qmake.is_file():
        raise ValueError(f'Qt SDK missing: {qmake}; run scripts/qt-sdk.py setup')
    environment = os.environ.copy()
    environment.update(QMAKE=str(qmake), QT_VERSION_MAJOR='6', QT_QUICK_CONTROLS_STYLE='Fusion')
    environment['PATH'] = str(sdk / 'bin') + os.pathsep + environment['PATH']
    if platform.system() == 'Windows':
        msvc_environment(environment)
    if platform.system() == 'Linux':
        environment['LD_LIBRARY_PATH'] = str(sdk / 'lib') + os.pathsep + environment.get('LD_LIBRARY_PATH', '')
    actual = subprocess.run([str(qmake), '-query', 'QT_VERSION'], env=environment,
                            capture_output=True, text=True, check=True).stdout.strip()
    if actual != QT_VERSION:
        raise ValueError(f'Expected Qt {QT_VERSION}, found {actual}')
    return environment


def run(command, environment=None):
    """Run an argument list at the repository root and propagate failure."""
    subprocess.run([str(argument) for argument in command], cwd=ROOT,
                   env=environment, check=True)


def setup():
    """Install aqt and the native Qt SDK locally, or validate an existing SDK.

    Reject unsupported hosts first. Downloads require network; no system package
    installation or global environment mutation occurs. Subprocess errors propagate.
    """
    host, architecture, _ = sdk_layout()
    sdk = sdk_path()
    suffix = '.exe' if os.name == 'nt' else ''
    qmake = sdk / 'bin' / ('qmake' + suffix)
    if not qmake.is_file():
        if 'QT_SDK_DIR' in os.environ:
            raise ValueError('Explicit QT_SDK_DIR is incomplete; refusing to populate it')
        tools = ROOT / 'local/qt-tools'
        venv.EnvBuilder(with_pip=True).create(tools)
        python = tools / ('Scripts/python.exe' if os.name == 'nt' else 'bin/python')
        run([python, '-m', 'pip', 'install', f'aqtinstall=={AQT_VERSION}'])
        archives = ['qtbase', 'qtdeclarative', 'qtshadertools', 'qtsvg', 'qttools']
        if host == 'linux':
            archives.append('icu')
        run([python, '-m', 'aqt', 'install-qt', host, 'desktop', QT_VERSION,
             architecture, '-O', ROOT / 'local/qt', '--archives', *archives])
    run([qmake, '-query', 'QT_VERSION'], qt_environment())
    if not (sdk / 'bin' / ('qmltestrunner' + suffix)).is_file():
        raise ValueError('Qt SDK lacks qmltestrunner; move the incomplete local SDK aside')


def check():
    """Run native Qt format, Rust tests/lints/build, and QML presentation tests."""
    environment = qt_environment()
    cargo = [str(ROOT / 'scripts/cargo-local.sh')] if os.name != 'nt' and (ROOT / 'local/cargo/bin/cargo').exists() else ['cargo']
    manifest = ['--manifest-path', 'apps/qt/Cargo.toml']
    run([*cargo, 'fmt', *manifest, '--', '--check'], environment)
    run([*cargo, 'test', *manifest, '--locked'], environment)
    run([*cargo, 'clippy', *manifest, '--all-targets', '--locked', '--', '-D', 'warnings'], environment)
    run([*cargo, 'build', *manifest, '--locked'], environment)
    environment.update(QT_QPA_PLATFORM='offscreen', QT_QUICK_BACKEND='software')
    suffix = '.exe' if os.name == 'nt' else ''
    run([sdk_path() / 'bin' / ('qmltestrunner' + suffix), '-input',
         ROOT / 'apps/qt/tests', '-import', ROOT / 'apps/qt/qml'], environment)


def main():
    """Dispatch SDK checks or CI linker export and report failures clearly."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('setup', 'check', 'export-msvc-linker'))
    args = parser.parse_args()
    try:
        if args.action == 'setup':
            setup()
        elif args.action == 'check':
            check()
        else:
            export_msvc_linker()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f'ERROR: {error}\n')


if __name__ == '__main__':
    main()
