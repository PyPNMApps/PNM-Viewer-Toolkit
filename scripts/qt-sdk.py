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
import tempfile
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


def qml_check(environment=None):
    """Run the complete QML suite with persistent, console-independent reports.

    environment optionally supplies an already validated Qt subprocess environment;
    otherwise validate the SDK here. Copy it before selecting Cocoa/offscreen
    event delivery and software rendering. Write fresh text, JUnit XML and process logs in a unique directory
    under local/qt-test-results, then print available diagnostics even on failure.
    Return the report directory. SDK, filesystem and subprocess errors propagate;
    a nonzero test exit always fails the check, never triggers a passing retry.
    """
    environment = dict(qt_environment() if environment is None else environment)
    # Cocoa runners have a logged-in WindowServer. Its native event delivery
    # avoids offscreen-plugin mouse/Canvas failures observed on both Mac CPUs.
    environment.update(QT_QPA_PLATFORM='cocoa' if platform.system() == 'Darwin' else 'offscreen',
                       QT_QUICK_BACKEND='software')
    reports = ROOT / 'local/qt-test-results'
    reports.mkdir(parents=True, exist_ok=True)
    destination = Path(tempfile.mkdtemp(prefix='run-', dir=reports))
    text_report = destination / 'qml-tests.txt'
    xml_report = destination / 'qml-tests.xml'
    process_log = destination / 'process.log'
    suffix = '.exe' if os.name == 'nt' else ''
    command = [str(sdk_path() / 'bin' / ('qmltestrunner' + suffix)),
               '-input', str(ROOT / 'apps/qt/tests'),
               '-import', str(ROOT / 'apps/qt/qml'),
               '-o', f'{text_report},txt', '-o', f'{xml_report},junitxml']
    print(f'QML test reports: {destination}', flush=True)
    try:
        with process_log.open('wb') as output:
            subprocess.run(command, cwd=ROOT, env=environment, stdout=output,
                           stderr=subprocess.STDOUT, check=True)
    finally:
        for report in (process_log, text_report):
            if report.is_file():
                print(report.read_text(encoding='utf-8', errors='replace'), flush=True)
        if not text_report.is_file():
            print('QML runner produced no text report; inspect process.log for startup errors.',
                  flush=True)
    return destination


def check():
    """Check QML first, then native Qt format, Rust tests/lints and build.

    A failed presentation test stops before expensive native compilation; all
    existing checks still run on success. Propagate the original failure.
    """
    environment = qt_environment()
    qml_check(environment)
    cargo = [str(ROOT / 'scripts/cargo-local.sh')] if os.name != 'nt' and (ROOT / 'local/cargo/bin/cargo').exists() else ['cargo']
    manifest = ['--manifest-path', 'apps/qt/Cargo.toml']
    run([*cargo, 'fmt', *manifest, '--', '--check'], environment)
    run([*cargo, 'test', *manifest, '--locked'], environment)
    run([*cargo, 'clippy', *manifest, '--all-targets', '--locked', '--', '-D', 'warnings'], environment)
    run([*cargo, 'build', *manifest, '--locked'], environment)


def main():
    """Dispatch SDK checks or CI linker export and report failures clearly."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('setup', 'check', 'qml-test', 'export-msvc-linker'))
    args = parser.parse_args()
    try:
        if args.action == 'setup':
            setup()
        elif args.action == 'check':
            check()
        elif args.action == 'qml-test':
            qml_check()
        else:
            export_msvc_linker()
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f'ERROR: {error}\n')


if __name__ == '__main__':
    main()
