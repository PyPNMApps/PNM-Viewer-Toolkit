#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check native Qt tool selection without downloading SDKs or invoking compilers."""
import importlib.util
import io
import os
from pathlib import Path
from contextlib import redirect_stdout
from functools import partial
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SCRIPT = Path(__file__).resolve().parents[1] / 'qt-sdk.py'
SPEC = importlib.util.spec_from_file_location('qt_sdk', SCRIPT)
QT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(QT)
LINKER_KEY = 'CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER'


class QtEnvironmentTests(unittest.TestCase):
    """Reproduce Git Bash linker shadowing using disposable tool directories."""

    def setUp(self):
        """Create synthetic MSVC, Git and Qt paths, including spaces."""
        temporary = tempfile.TemporaryDirectory(prefix='pnm tool paths ')
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.tools = self.root / 'Visual Studio' / 'MSVC'
        self.binaries = self.tools / 'bin' / 'Hostx64' / 'x64'
        self.binaries.mkdir(parents=True)
        for name in ('cl.exe', 'link.exe', 'lib.exe'):
            (self.binaries / name).touch()
        self.git = self.root / 'Git' / 'usr' / 'bin'
        self.git.mkdir(parents=True)
        (self.git / 'link.exe').touch()
        self.sdk = self.root / 'Qt SDK'
        (self.sdk / 'bin').mkdir(parents=True)
        (self.sdk / 'bin' / ('qmake.exe' if os.name == 'nt' else 'qmake')).touch()

    def test_msvc_wins_over_git_bash_link_executable(self):
        """Resolve the first link.exe to MSVC despite Git starting first in PATH."""
        environment = {'PATH': str(self.git), 'VCToolsInstallDir': str(self.tools)}
        QT.msvc_environment(environment)
        first_link = next(Path(part) / 'link.exe' for part in environment['PATH'].split(os.pathsep)
                          if (Path(part) / 'link.exe').is_file())
        self.assertEqual(first_link, self.binaries / 'link.exe')
        self.assertEqual(environment[LINKER_KEY], str(self.binaries / 'link.exe'))

    def test_windows_uppercase_environment_keys(self):
        """Windows os.environ.copy uses uppercase keys for MSVC metadata."""
        environment = {'PATH': str(self.git), 'VCTOOLSINSTALLDIR': str(self.tools)}
        QT.msvc_environment(environment)
        self.assertEqual(environment[LINKER_KEY], str(self.binaries / 'link.exe'))

    def test_explicit_cargo_linker_override_is_preserved(self):
        """Keep an intentional caller-provided linker while fixing PATH order."""
        environment = {'PATH': str(self.git), 'VCTOOLSINSTALLDIR': str(self.tools),
                       LINKER_KEY: 'custom-linker.exe'}
        QT.msvc_environment(environment)
        self.assertEqual(environment[LINKER_KEY], 'custom-linker.exe')
        self.assertEqual(environment['PATH'].split(os.pathsep)[0], str(self.binaries))

    def test_uninitialized_developer_environment_fails_clearly(self):
        """Do not silently fall back to Git's executable when MSVC is unavailable."""
        with self.assertRaisesRegex(ValueError, 'initialize an x64 MSVC'):
            QT.msvc_environment({'PATH': str(self.git)})

    def test_incomplete_toolchain_fails_before_build(self):
        """Reject each missing tool instead of allowing a later unrelated error."""
        for name in ('cl.exe', 'link.exe', 'lib.exe'):
            with self.subTest(name=name):
                path = self.binaries / name
                path.unlink()
                try:
                    with self.assertRaisesRegex(ValueError, 'MSVC x64 tool missing'):
                        QT.msvc_environment({'PATH': str(self.git),
                                             'VCTOOLSINSTALLDIR': str(self.tools)})
                finally:
                    path.touch()

    def test_ci_export_survives_later_git_bash_path_changes(self):
        """Persist the absolute linker without requiring Qt or changing the caller."""
        destination = self.root / 'github environment'
        destination.write_text('EXISTING=value\n', encoding='utf-8')
        original = {'PATH': str(self.git), 'VCTOOLSINSTALLDIR': str(self.tools),
                    'GITHUB_ENV': str(destination)}
        with patch.dict(os.environ, original, clear=True), \
                patch.object(QT, 'sdk_path', side_effect=AssertionError('Qt not required')):
            QT.export_msvc_linker()
            self.assertEqual(dict(os.environ), original)
        entries = dict(line.split('=', 1) for line in destination.read_text().splitlines())
        self.assertEqual(entries['EXISTING'], 'value')
        self.assertEqual(entries[LINKER_KEY], str(self.binaries / 'link.exe'))
        # A later Bash step may prepend Git again; Cargo keeps its explicit linker.
        child = {'PATH': str(self.git), **entries}
        self.assertTrue(Path(child[LINKER_KEY]).is_file())
        self.assertNotEqual(child[LINKER_KEY], str(self.git / 'link.exe'))

    def test_ci_export_preserves_override(self):
        """Persist an intentional linker override including spaces and equals signs."""
        destination = self.root / 'github environment'
        override = str(self.root / 'custom tools' / 'link=custom.exe')
        with patch.dict(os.environ, {'VCTOOLSINSTALLDIR': str(self.tools),
                                     'GITHUB_ENV': str(destination), LINKER_KEY: override}, clear=True):
            QT.export_msvc_linker()
        self.assertEqual(destination.read_text(), f'{LINKER_KEY}={override}\n')

    def test_ci_export_rejects_invalid_input_without_writing(self):
        """Fail before touching the job file for invalid MSVC or environment values."""
        destination = self.root / 'github environment'
        cases = ({'GITHUB_ENV': str(destination)},
                 {'VCTOOLSINSTALLDIR': str(self.tools)},
                 {'VCTOOLSINSTALLDIR': str(self.tools), 'GITHUB_ENV': str(destination),
                  LINKER_KEY: 'link.exe\nINJECTED=value'})
        for environment in cases:
            with self.subTest(environment=environment), \
                    patch.dict(os.environ, environment, clear=True):
                with self.assertRaises(ValueError):
                    QT.export_msvc_linker()
                self.assertFalse(destination.exists())

    def test_qt_environment_applies_selection_without_global_changes(self):
        """Qt checks and package builds receive the corrected child environment."""
        original = {'PATH': str(self.git), 'VCTOOLSINSTALLDIR': str(self.tools)}
        with patch.dict(os.environ, original, clear=True), \
                patch.object(QT, 'sdk_path', return_value=self.sdk), \
                patch.object(QT.platform, 'system', return_value='Windows'), \
                patch.object(QT.subprocess, 'run', return_value=subprocess.CompletedProcess(
                    [], 0, stdout=QT.QT_VERSION + '\n')) as query:
            environment = QT.qt_environment()
            self.assertEqual(environment[LINKER_KEY], str(self.binaries / 'link.exe'))
            self.assertEqual(environment['PATH'].split(os.pathsep)[:2],
                             [str(self.binaries), str(self.sdk / 'bin')])
            self.assertEqual(query.call_args.kwargs['env'], environment)
            self.assertEqual(dict(os.environ), original)

    def test_linux_environment_does_not_require_msvc(self):
        """Preserve Qt library resolution and ordinary tool lookup on Linux."""
        with patch.dict(os.environ, {'PATH': str(self.git)}, clear=True), \
                patch.object(QT, 'sdk_path', return_value=self.sdk), \
                patch.object(QT.platform, 'system', return_value='Linux'), \
                patch.object(QT.subprocess, 'run', return_value=subprocess.CompletedProcess(
                    [], 0, stdout=QT.QT_VERSION + '\n')):
            environment = QT.qt_environment()
            self.assertNotIn(LINKER_KEY, environment)
            self.assertEqual(environment['PATH'].split(os.pathsep)[0], str(self.sdk / 'bin'))
            self.assertEqual(environment['LD_LIBRARY_PATH'].split(os.pathsep)[0], str(self.sdk / 'lib'))


class QtDownloadTests(unittest.TestCase):
    """Keep download retries bounded and independent of test/build failures."""

    def test_timeout_retries_full_install_after_partial_extraction(self):
        """Retry aqt despite an already extracted qmake, then retain its output."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            qmake = root / 'local/qt/bin/qmake'
            command = ['python', '-m', 'aqt', 'install-qt', 'mac']
            calls = []

            def download(arguments, **options):
                calls.append(arguments)
                if len(calls) == 1:
                    qmake.parent.mkdir(parents=True)
                    qmake.touch()
                    raise subprocess.CalledProcessError(
                        1, arguments, output='ArchiveDownloadError: Read timed out.')
                self.assertTrue(qmake.exists())
                return subprocess.CompletedProcess(arguments, 0, stdout='Installed all archives')

            output = io.StringIO()
            with patch.object(QT, 'ROOT', root), \
                    patch.object(QT.subprocess, 'run', side_effect=download), \
                    patch.object(QT.time, 'sleep') as pause, redirect_stdout(output):
                QT._download_qt(command, 3)
            self.assertEqual(calls, [command, command])
            pause.assert_called_once_with(10)
            self.assertIn('Read timed out', output.getvalue())
            self.assertIn('Installed all archives', output.getvalue())

    def test_repeated_timeout_preserves_final_failure(self):
        """Stop after exactly three downloads with the final original status."""
        failure = subprocess.CalledProcessError(7, ['aqt'], output='Read timed out.')
        with patch.object(QT.subprocess, 'run', side_effect=failure) as download, \
                patch.object(QT.time, 'sleep') as pause, redirect_stdout(io.StringIO()):
            with self.assertRaises(subprocess.CalledProcessError) as caught:
                QT._download_qt(['aqt'], 3)
        self.assertIs(caught.exception, failure)
        self.assertEqual(download.call_count, 3)
        self.assertEqual([call.args[0] for call in pause.call_args_list], [10, 20])

    def test_non_network_failures_are_not_retried(self):
        """Bad archives, invalid arguments and unknown errors fail immediately."""
        for diagnostic in ('ArchiveChecksumError', 'Unknown architecture', 'Disk full', ''):
            with self.subTest(diagnostic=diagnostic), \
                    patch.object(QT.subprocess, 'run', side_effect=subprocess.CalledProcessError(
                        1, ['aqt'], output=diagnostic)) as download, \
                    patch.object(QT.time, 'sleep') as pause, redirect_stdout(io.StringIO()):
                with self.assertRaises(subprocess.CalledProcessError):
                    QT._download_qt(['aqt'], 3)
                self.assertEqual(download.call_count, 1)
                pause.assert_not_called()

    def test_single_attempt_does_not_retry_timeout(self):
        """Keep ordinary local setup at one download attempt by default."""
        with patch.object(QT.subprocess, 'run', side_effect=subprocess.CalledProcessError(
                1, ['aqt'], output='Read timed out.')) as download, \
                patch.object(QT.time, 'sleep') as pause, redirect_stdout(io.StringIO()):
            with self.assertRaises(subprocess.CalledProcessError):
                QT._download_qt(['aqt'], 1)
        self.assertEqual(download.call_count, 1)
        pause.assert_not_called()

    def test_invalid_attempt_count_fails_before_setup(self):
        """Reject unbounded attempts before tool selection or filesystem writes."""
        for attempts in (0, 4):
            with self.subTest(attempts=attempts), patch.object(QT, 'sdk_layout') as layout:
                with self.assertRaises(ValueError):
                    QT.setup(attempts)
                layout.assert_not_called()


class QmlReportTests(unittest.TestCase):
    """Keep otherwise invisible Windows test failures observable and fatal."""

    def setUp(self):
        """Isolate reports and fake SDK paths without downloading or compiling."""
        temporary = tempfile.TemporaryDirectory(prefix='pnm qml reports ')
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.output = io.StringIO()
        self.addCleanup(patch.stopall)
        patch.object(QT, 'ROOT', self.root).start()
        patch.object(QT, 'sdk_path', return_value=self.root / 'Qt SDK').start()
        self.environment = {'PATH': 'caller tools'}

    def fake_run(self, command, *, expected_platform, **options):
        """Write test reports as the native runner would, including UTF-8 text."""
        outputs = [command[index + 1] for index, argument in enumerate(command)
                   if argument == '-o']
        self.assertEqual(len(outputs), 2)
        text, xml = [Path(value.rsplit(',', 1)[0]) for value in outputs]
        text.write_text('FAIL! simulated assertion with café\n', encoding='utf-8')
        xml.write_text('<testsuite failures="1"/>', encoding='utf-8')
        options['stdout'].write(b'Native runner diagnostic\n')
        self.assertEqual(options['env']['QT_QPA_PLATFORM'], expected_platform)
        self.assertEqual(options['env']['QT_QUICK_BACKEND'], 'software')
        self.assertTrue(options['check'])
        raise subprocess.CalledProcessError(1, command)

    def test_failure_prints_reports_and_preserves_exit_status(self):
        """Never hide a failed assertion behind the final subprocess exception."""
        for system, expected_platform in (('Linux', 'offscreen'),
                                          ('Windows', 'offscreen'),
                                          ('Darwin', 'cocoa')):
            output = io.StringIO()
            report_root = self.root / system
            with self.subTest(system=system), \
                    patch.object(QT, 'ROOT', report_root), \
                    patch.object(QT.platform, 'system', return_value=system), \
                    patch.object(QT.subprocess, 'run', side_effect=partial(
                        self.fake_run, expected_platform=expected_platform)), \
                    redirect_stdout(output):
                with self.assertRaises(subprocess.CalledProcessError) as failure:
                    QT.qml_check(self.environment)
                self.assertEqual(failure.exception.returncode, 1)
                self.assertIn('FAIL! simulated assertion with café', output.getvalue())
                self.assertIn('Native runner diagnostic', output.getvalue())
                self.assertEqual(len(list(report_root.rglob('qml-tests.xml'))), 1)
                self.assertEqual(self.environment, {'PATH': 'caller tools'})

    def test_startup_failure_keeps_available_process_diagnostics(self):
        """Print startup errors even when no Qt Test report could be written."""
        def fail_before_report(command, **options):
            options['stdout'].write(b'QML module not installed\n')
            raise subprocess.CalledProcessError(2, command)
        with patch.object(QT.subprocess, 'run', side_effect=fail_before_report), \
                redirect_stdout(self.output):
            with self.assertRaises(subprocess.CalledProcessError):
                QT.qml_check(self.environment)
        self.assertIn('QML module not installed', self.output.getvalue())
        self.assertIn('no text report', self.output.getvalue())

    def test_qml_failure_stops_before_native_compilation(self):
        """Avoid expensive Rust builds after a known presentation-suite failure."""
        failure = subprocess.CalledProcessError(1, ['qmltestrunner'])
        with patch.object(QT, 'qt_environment', return_value=self.environment), \
                patch.object(QT, 'ensure_space') as space_check, \
                patch.object(QT, 'qml_check', side_effect=failure), \
                patch.object(QT, 'run') as native:
            with self.assertRaises(subprocess.CalledProcessError):
                QT.check()
            native.assert_not_called()
            space_check.assert_called_once_with(QT.ROOT)

    def test_mac_uses_native_cocoa_for_interaction_tests(self):
        """Use WindowServer event delivery without skipping any QML test input."""
        def succeed(command, **options):
            self.assertEqual(options['env']['QT_QPA_PLATFORM'], 'cocoa')
            self.assertEqual(options['env']['QT_QUICK_BACKEND'], 'software')
            self.assertIn(str(QT.ROOT / 'apps/qt/tests'), command)
            destination = Path(command[command.index('-o') + 1].rsplit(',', 1)[0])
            destination.write_text('PASS fixture\n', encoding='utf-8')
        with patch.object(QT.platform, 'system', return_value='Darwin'), \
                patch.object(QT.subprocess, 'run', side_effect=succeed), redirect_stdout(self.output):
            QT.qml_check(self.environment)

    def test_repeated_runs_have_separate_reports(self):
        """A later startup failure cannot reuse an earlier passing result file."""
        def succeed(command, **options):
            destination = Path(command[command.index('-o') + 1].rsplit(',', 1)[0])
            destination.write_text('PASS fresh result\n', encoding='utf-8')
        with patch.object(QT.subprocess, 'run', side_effect=succeed), \
                redirect_stdout(self.output):
            first = QT.qml_check(self.environment)
            second = QT.qml_check(self.environment)
        self.assertNotEqual(first, second)
        self.assertTrue((first / 'qml-tests.txt').exists())
        self.assertTrue((second / 'qml-tests.txt').exists())


if __name__ == '__main__':
    unittest.main()
