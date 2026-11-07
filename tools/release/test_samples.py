#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Generated installer demos must pass hygiene before any archive is created."""
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch
import zipfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts/release'))
import samples


class InstallerSampleTests(unittest.TestCase):
    """Exercise real sanitized sources and deliberately invalid build inputs."""

    def test_fixture_checkout_preserves_bytes_with_windows_line_endings(self):
        """Exercise Git's real checkout filters with core.autocrlf enabled."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            repository = root / 'repository'
            repository.mkdir()
            shutil.copy2(ROOT / '.gitattributes', repository / '.gitattributes')
            fixtures = repository / 'tests/fixtures/demos/example'
            fixtures.mkdir(parents=True)
            expected = {
                'README.md': b'# Demo\n\nKeep exact bytes.\n',
                'provenance.json': b'{"approved": true}\n',
                'capture.bin': b'PNN\x09\x00\n\r\n\xff',
                'LICENSE': b'License notice\n',
            }
            for name, content in expected.items():
                (fixtures / name).write_bytes(content)
            subprocess.run(['git', 'init', str(repository)], check=True, capture_output=True)
            command = ['git', '-C', str(repository), '-c', 'core.autocrlf=true']
            subprocess.run(command + ['add', '.'], check=True, capture_output=True)
            checkout = root / 'checkout'
            subprocess.run(command + ['checkout-index', '--all', '--prefix', checkout.as_posix() + '/'],
                           check=True, capture_output=True)
            for name, content in expected.items():
                self.assertEqual((checkout / 'tests/fixtures/demos/example' / name).read_bytes(), content)

    def test_system_symlink_ancestor_is_allowed_but_demo_redirects_fail(self):
        """Accept macOS-style temp roots without permitting redirected inputs."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            actual = root / 'actual'
            actual.mkdir()
            alias = root / 'alias'
            try:
                alias.symlink_to(actual, target_is_directory=True)
            except (OSError, NotImplementedError) as error:
                self.skipTest(f'Directory symlinks unavailable: {error}')
            demos = alias / 'demos'
            shutil.copytree(samples.DEMO_ROOT, demos)
            manifest_path = demos / 'installer-demos.json'
            with patch.object(samples, 'DEMO_ROOT', demos), patch.object(samples, 'DEMO_MANIFEST', manifest_path):
                entries = samples.check_sources()
                source = demos / entries[0]['source']
                capture = next(source.glob('*.bin'))
                outside = root / 'outside.bin'
                shutil.copy2(capture, outside)
                capture.unlink()
                capture.symlink_to(outside)
                with self.assertRaisesRegex(ValueError, 'Invalid or oversized'):
                    samples.check_sources()
                capture.unlink()
                shutil.copy2(outside, capture)
                source.rename(demos / 'redirected-source')
                source.symlink_to(demos / 'redirected-source', target_is_directory=True)
                with self.assertRaisesRegex(ValueError, 'Invalid expanded demo source'):
                    samples.check_sources()
                source.unlink()
                (demos / 'redirected-source').rename(source)
                demos.rename(actual / 'redirected-demos')
                demos.symlink_to(actual / 'redirected-demos', target_is_directory=True)
                with self.assertRaisesRegex(ValueError, 'parents must not be symlinks'):
                    samples.check_sources()

    def test_expanded_hygiene_never_opens_a_source_archive(self):
        """Check all registered sources without opening or generating a ZIP."""
        with patch.object(zipfile, 'ZipFile', side_effect=AssertionError('Unexpected ZIP access')):
            datasets = samples.check_sources()
        self.assertEqual(len(datasets), 5)
        self.assertEqual(sum(len([item for item in entry['files'] if item['path'].endswith('.bin')])
                             for entry in datasets), 381)

    def test_generated_zips_repeat_and_installed_resources_match(self):
        """Bundle deterministic ZIPs and verify deployed bytes on each OS layout."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first = root / 'first/share/examples'
            second = root / 'second/share/examples'
            samples.bundle(first)
            samples.bundle(second)
            self.assertEqual(samples.installed_directory(root / 'first/pnm-viewer.exe'), first)
            mac = root / 'Viewer.app/Contents/Resources/share/examples'
            mac.parent.mkdir(parents=True)
            shutil.copytree(first, mac)
            self.assertEqual(samples.installed_directory(root / 'Viewer.app/Contents/MacOS/pnm-viewer'), mac)
            for archive in first.glob('rxmer/*.zip'):
                self.assertEqual(archive.read_bytes(), (second / archive.relative_to(first)).read_bytes())
                with zipfile.ZipFile(archive) as source:
                    self.assertTrue(all(info.date_time == (1980, 1, 1, 0, 0, 0) for info in source.infolist()))
            alias_capture = first / 'single/rxmer.bin'
            alias_capture.write_bytes(alias_capture.read_bytes() + b'corrupt')
            with self.assertRaisesRegex(ValueError, 'missing or modified'):
                samples.installed_directory(root / 'first/pnm-viewer.exe')
            with self.assertRaises(FileExistsError):
                samples.bundle(second)

    def test_hygiene_failure_stops_before_zip_or_destination(self):
        """Even checksum-registered input with a private MAC cannot be packaged."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            demos = root / 'demos'
            shutil.copytree(samples.DEMO_ROOT, demos)
            manifest_path = demos / 'installer-demos.json'
            manifest = json.loads(manifest_path.read_text())
            entry = manifest['datasets'][0]
            item = next(item for item in entry['files'] if item['path'].endswith('.bin'))
            capture = demos / entry['source'] / item['path']
            data = bytearray(capture.read_bytes())
            data[11:17] = bytes.fromhex('102030405060')
            capture.write_bytes(data)
            item['sha256'] = hashlib.sha256(data).hexdigest()
            manifest_path.write_text(json.dumps(manifest))
            output = root / 'examples'
            with patch.object(samples, 'DEMO_ROOT', demos), patch.object(samples, 'DEMO_MANIFEST', manifest_path), \
                    patch.object(samples, 'create_archive') as create:
                with self.assertRaises(subprocess.CalledProcessError):
                    samples.bundle(output)
                create.assert_not_called()
            self.assertFalse(output.exists())
            self.assertEqual(capture.read_bytes(), data)

    def test_extra_archive_corrupt_source_and_path_escape_fail(self):
        """Reject unregistered/private inputs and unsafe output paths early."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            demos = root / 'demos'
            shutil.copytree(samples.DEMO_ROOT, demos)
            manifest_path = demos / 'installer-demos.json'
            manifest = json.loads(manifest_path.read_text())
            source = demos / manifest['datasets'][0]['source']
            extra = source / 'prebuilt.zip'
            extra.write_bytes(b'never unzip me')
            with patch.object(samples, 'DEMO_ROOT', demos), patch.object(samples, 'DEMO_MANIFEST', manifest_path):
                with self.assertRaisesRegex(ValueError, 'unregistered'):
                    samples.check_sources()
                extra.unlink()
                manifest['datasets'][0]['output'] = '../outside.zip'
                manifest_path.write_text(json.dumps(manifest))
                with self.assertRaisesRegex(ValueError, 'Unsafe'):
                    samples.check_sources()
                manifest['datasets'][0]['output'] = 'rxmer/single-channel.zip'
                manifest_path.write_text(json.dumps(manifest))
                capture = next(source.glob('*.bin'))
                capture.write_bytes(capture.read_bytes() + b'changed')
                with self.assertRaisesRegex(ValueError, 'checksum'):
                    samples.check_sources()


class SamplePackageTests(unittest.TestCase):
    """Build real sample resources for each supported installation layout."""

    def test_samples_are_complete_verified_and_read_only_to_the_builder(self):
        """Copy nine approved captures and notices without adding Python tests."""
        for relative, executable in (
                ('share/examples', 'pnm-viewer.exe'),
                ('Contents/Resources/share/examples', 'Contents/MacOS/pnm-viewer')):
            with self.subTest(layout=relative), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                destination = root / relative
                samples.bundle(destination)
                self.assertEqual(len(list((destination / 'single').glob('*.bin'))), 9)
                self.assertFalse(list(destination.rglob('*.py')))
                for name in ('LICENSE', 'NOTICE', 'provenance.json', 'README.md'):
                    self.assertTrue((destination / 'single' / name).is_file())
                self.assertEqual(samples.installed_directory(root / executable), destination)
                with self.assertRaises(FileExistsError):
                    samples.bundle(destination)
                (destination / 'single/rxmer.bin').write_bytes(b'corrupt')
                with self.assertRaisesRegex(ValueError, 'modified: rxmer.bin'):
                    samples.installed_directory(root / executable)

    def test_absent_installed_examples_fail_the_package_gate(self):
        """Never silently fall back to checkout fixtures in deployed smoke tests."""
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(ValueError, 'examples folder missing'):
                samples.installed_directory(Path(directory) / 'viewer')

    def test_native_icon_formats_include_shell_sizes(self):
        """Reject missing/corrupt native assets before invoking platform builders."""
        icon = (samples.ROOT / 'packaging/windows/pnm-viewer.ico').read_bytes()
        reserved, kind, count = struct.unpack_from('<HHH', icon)
        self.assertEqual((reserved, kind), (0, 1))
        sizes = set()
        for index in range(count):
            width, height, _, _, _, _, length, offset = struct.unpack_from('<BBBBHHII', icon, 6 + index * 16)
            self.assertEqual(width, height)
            sizes.add(width or 256)
            self.assertGreater(length, 0)
            self.assertLessEqual(offset + length, len(icon))
        self.assertEqual(sizes, {16, 24, 32, 48, 64, 128, 256})
        mac = (samples.ROOT / 'packaging/macos/pnm-viewer.icns').read_bytes()
        self.assertEqual(mac[:4], b'icns')
        self.assertEqual(struct.unpack_from('>I', mac, 4)[0], len(mac))
        self.assertIn(b'ic10', mac)
        for name, expected in (('dialog.bmp', (493, 312)), ('banner.bmp', (493, 58))):
            bitmap = (samples.ROOT / 'packaging/windows' / name).read_bytes()
            self.assertEqual(bitmap[:2], b'BM')
            self.assertEqual(struct.unpack_from('<ii', bitmap, 18), expected)


if __name__ == '__main__':
    unittest.main()
