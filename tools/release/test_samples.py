#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Packaged samples retain sanitized source bytes and upstream provenance."""
from pathlib import Path
import sys
import struct
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'scripts/release'))
import samples


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
                self.assertEqual(len(list(destination.glob('*.bin'))), 9)
                self.assertFalse(list(destination.rglob('*.py')))
                for name in ('LICENSE', 'NOTICE', 'provenance.json', 'README.md'):
                    self.assertTrue((destination / name).is_file())
                self.assertEqual(samples.installed_directory(root / executable), destination)
                with self.assertRaises(FileExistsError):
                    samples.bundle(destination)
                (destination / 'rxmer.bin').write_bytes(b'corrupt')
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
