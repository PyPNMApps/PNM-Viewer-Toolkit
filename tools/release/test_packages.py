#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Artifact integrity, size-budget and lifecycle guard regression tests."""
import importlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
import zipfile
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts/release'))
import common
import collect
import native
import verify


class PackageContractTests(unittest.TestCase):
    """Reject corrupt artifacts and unsafe execution without native installers."""

    def test_windows_zip_preserves_legacy_notices(self):
        """Clamp old ZIP dates while preserving bytes, paths and source metadata."""
        builder = importlib.import_module('build')
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'pnm-decode-synthetic-windows-x64'
            share = source / 'share'
            share.mkdir(parents=True)
            files = {share / 'legacy-LICENSE': (b'legacy license bytes', 157809600),
                     source / 'pnm-decode.exe': (b'synthetic executable', 946728000)}
            for path, (content, timestamp) in files.items():
                path.write_bytes(content)
                os.utime(path, (timestamp, timestamp))
            original_times = {path: path.stat().st_mtime_ns for path in files}
            destination = builder.cli_archive(source, root, 'Windows')
            with zipfile.ZipFile(destination) as archive:
                self.assertIsNone(archive.testzip())
                for path, (content, _) in files.items():
                    name = path.relative_to(root).as_posix()
                    self.assertEqual(archive.read(name), content)
                    self.assertEqual(path.read_bytes(), content)
                    self.assertEqual(path.stat().st_mtime_ns, original_times[path])
                legacy = archive.getinfo(f'{source.name}/share/legacy-LICENSE')
                self.assertEqual(legacy.date_time, (1980, 1, 1, 0, 0, 0))
                modern = archive.getinfo(f'{source.name}/pnm-decode.exe')
                self.assertEqual(modern.date_time[0], 2000)
            original_archive = destination.read_bytes()
            with self.assertRaises(FileExistsError):
                builder.cli_archive(source, root, 'Windows')
            self.assertEqual(destination.read_bytes(), original_archive)

    def test_size_budget_and_checksum_corruption(self):
        """Reject empty/oversized packages and changed bytes after upload."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            artifact = output / 'synthetic.deb'
            artifact.write_bytes(b'valid fixture bytes')
            common.verify_artifacts(output)
            (output / 'release-manifest.json').write_text(json.dumps({
                'version': common.release_version(),
                'artifacts': [{'name': artifact.name, 'bytes': artifact.stat().st_size}]}))
            verify.verify(output)
            artifact.write_bytes(b'changed fixture bytes')
            with self.assertRaisesRegex(ValueError, 'Checksum mismatch'):
                verify.verify(output)
            with patch.object(common, 'MAX_PACKAGE_BYTES', 1):
                with self.assertRaisesRegex(ValueError, 'budget'):
                    common.verify_artifacts(output)
            artifact.write_bytes(b'')
            with self.assertRaisesRegex(ValueError, 'budget'):
                common.verify_artifacts(output)

    def test_unsafe_checksum_paths(self):
        """Reject traversal names before reading any package outside its directory."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory)
            (output / 'release-manifest.json').write_text(json.dumps({
                'version': common.release_version(), 'artifacts': []}))
            (output / 'SHA256SUMS').write_text('abc  ../capture.bin\n')
            with self.assertRaisesRegex(ValueError, 'Unsafe'):
                verify.verify(output)

    def test_install_requires_disposable_runner(self):
        """Local invocation cannot install packages without the explicit CI guard."""
        result = subprocess.run([sys.executable, str(ROOT / 'scripts/release/test-installed.py'),
                                 '--packages', 'missing'], capture_output=True, text=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('disposable', result.stderr)

    def test_release_assembly_requires_every_target(self):
        """A partial artifact set cannot become a release asset directory."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'artifacts'
            source.mkdir()
            with self.assertRaisesRegex(ValueError, 'Incomplete'):
                collect.collect(source, root / 'assets')
            self.assertFalse((root / 'assets').exists())

    def test_native_target_and_existing_output(self):
        """Reject unsupported hosts and preserve existing output without builds."""
        with patch('common.platform.system', return_value='Linux'), patch('common.platform.machine', return_value='aarch64'):
            with self.assertRaises(ValueError):
                common.native_target()
        builder = importlib.import_module('build')
        with tempfile.TemporaryDirectory() as directory, patch.object(builder, 'run') as command:
            with self.assertRaisesRegex(ValueError, 'Output exists'):
                builder.build(Path(directory))
            command.assert_not_called()


if __name__ == '__main__':
    unittest.main()
