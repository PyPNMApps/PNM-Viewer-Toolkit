#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check native Qt tool selection without downloading SDKs or invoking compilers."""
import importlib.util
import os
from pathlib import Path
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


if __name__ == '__main__':
    unittest.main()
