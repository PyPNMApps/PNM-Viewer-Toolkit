#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Artifact integrity, size-budget and lifecycle guard regression tests."""
import importlib
import hashlib
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
import smoke
import verify


class PackageContractTests(unittest.TestCase):
    """Reject corrupt artifacts and unsafe execution without native installers."""

    def test_smoke_captures_gui_diagnostics_without_sdk_paths(self):
        """Route Qt GUI messages to stderr without altering the caller's session."""
        original = {'PATH': os.pathsep.join(('/tools', '/checkout/local/qt/bin')),
                    'QT_SDK_DIR': '/checkout/local/qt', 'QT_PLUGIN_PATH': '/sdk/plugins',
                    'QMAKE': '/sdk/qmake', 'QT_FORCE_STDERR_LOGGING': '0',
                    'QT_QUICK_CONTROLS_STYLE': 'Windows',
                    'QT_QUICK_CONTROLS_FALLBACK_STYLE': 'Basic',
                    'QT_QUICK_CONTROLS_CONF': '/sdk/development.conf',
                    'DISPLAY': ':42'}
        with patch.dict(os.environ, original, clear=True):
            environment = smoke.clean_environment()
            self.assertEqual(dict(os.environ), original)
        self.assertEqual(environment['QT_FORCE_STDERR_LOGGING'], '1')
        self.assertEqual(environment['DISPLAY'], ':42')
        self.assertEqual(environment['PATH'], '/tools')
        for name in ('QT_QUICK_CONTROLS_STYLE', 'QT_QUICK_CONTROLS_FALLBACK_STYLE',
                     'QT_QUICK_CONTROLS_CONF'):
            self.assertNotIn(name, environment)
        for name in ('QT_SDK_DIR', 'QT_PLUGIN_PATH', 'QMAKE'):
            self.assertNotIn(name, environment)

    def test_smoke_rejects_missing_render_marker(self):
        """An exit-zero GUI still fails without positive rendering evidence."""
        with patch.object(smoke.subprocess, 'run', return_value=subprocess.CompletedProcess(
                [], 0, stdout='', stderr='')), \
                patch.object(smoke, 'installed_directory', return_value=ROOT / 'tests/fixtures/pypnm/tests/files'):
            with self.assertRaisesRegex(RuntimeError, 'Installed GUI smoke failed: rxmer.bin'):
                smoke.smoke(Path('viewer'), Path('decoder'))

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

    def test_explicit_partial_assembly_omits_failed_platform_and_verifies_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'artifacts'
            windows = source / 'native-windows-2022'
            windows.mkdir(parents=True)
            payload = b'synthetic verified MSI'
            (windows / 'test.msi').write_bytes(payload)
            (windows / 'SHA256SUMS').write_text(hashlib.sha256(payload).hexdigest() + '  test.msi\n')
            (windows / 'release-manifest.json').write_text(json.dumps({
                'version': common.release_version(), 'system': 'Windows', 'architecture': 'x64',
                'artifacts': [{'name': 'test.msi', 'bytes': len(payload)}]}))
            (source / 'native-macos-15-intel').mkdir()
            with self.assertRaisesRegex(ValueError, 'Incomplete'):
                collect.collect(source, root / 'full', include_artifacts=['native-windows-2022'])
            collect.collect(source, root / 'partial', allow_partial=True,
                            include_artifacts=['native-windows-2022'])
            self.assertEqual((root / 'partial/test.msi').read_bytes(), payload)
            self.assertEqual(len(json.loads((root / 'partial/release-index.json').read_text())), 1)
            (windows / 'test.msi').write_bytes(b'corrupt')
            with self.assertRaisesRegex(ValueError, 'Checksum'):
                collect.collect(source, root / 'corrupt', allow_partial=True,
                                include_artifacts=['native-windows-2022'])
            self.assertFalse((root / 'corrupt').exists())

    def test_public_collection_has_five_installers_and_matching_checksums(self):
        """Keep reports/archives internal while verifying all original bytes."""
        version = common.release_version()
        packages = {
            ('Linux', 'x64'): [f'pnm-viewer-toolkit_{version}_amd64.deb', f'pnm-decode_{version}_amd64.deb'],
            ('Windows', 'x64'): [f'PNM-Viewer-Toolkit-{version}-windows-x64.msi'],
            ('Darwin', 'arm64'): [f'PNM-Viewer-Toolkit-{version}-macos-arm64.dmg'],
            ('Darwin', 'x64'): [f'PNM-Viewer-Toolkit-{version}-macos-x64.dmg'],
        }
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            source = root / 'native'
            installers = set()
            for (system, architecture), filenames in packages.items():
                platform = source / f'{system}-{architecture}'
                platform.mkdir(parents=True)
                installers.update(filenames)
                archive = f'pnm-decode-{version}-{system}-{architecture}.zip'
                entries, checksums = [], []
                for name in [*filenames, archive]:
                    content = name.encode()
                    (platform / name).write_bytes(content)
                    entries.append({'name': name, 'bytes': len(content)})
                    checksums.append(hashlib.sha256(content).hexdigest() + '  ' + name)
                (platform / 'SHA256SUMS').write_text('\n'.join(checksums) + '\n')
                (platform / 'release-manifest.json').write_text(json.dumps({
                    'version': version, 'system': system, 'architecture': architecture,
                    'artifacts': entries}))
                (platform / 'bom.cdx.json').write_text('{}')
            output = root / 'public'
            collect.collect(source, output)
            self.assertEqual({path.name for path in output.iterdir()},
                             installers | {'SHA256SUMS', 'release-index.json'})
            checksums = (output / 'SHA256SUMS').read_text().splitlines()
            self.assertEqual({line.split('  ', 1)[1] for line in checksums}, installers)
            for line in checksums:
                digest, name = line.split('  ', 1)
                self.assertEqual(hashlib.sha256((output / name).read_bytes()).hexdigest(), digest)
            index = json.loads((output / 'release-index.json').read_text())
            self.assertEqual({item['name'] for manifest in index for item in manifest['artifacts']}, installers)
            # Excluded archives still pass the upstream integrity gate.
            next(source.glob('*/*.zip')).write_bytes(b'corrupted archive')
            with self.assertRaisesRegex(ValueError, 'Checksum'):
                collect.collect(source, root / 'corrupt')

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
