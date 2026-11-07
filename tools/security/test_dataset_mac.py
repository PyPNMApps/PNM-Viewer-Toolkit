#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check collection identity preservation, archive safety and push inventory."""
import contextlib
import gzip
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import sys
import tarfile
import tempfile
import unittest
from unittest.mock import patch
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parent))
from dataset_mac import Dataset, aliases, is_alias

SPEC = importlib.util.spec_from_file_location('mac_tool', Path(__file__).with_name('pnm-mac-updater.py'))
TOOL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TOOL)
FIRST = bytes.fromhex('00ca00000001')
SECOND = bytes.fromhex('00ca00000002')


def capture(address, kind=4):
    """Build distinct header fields with unchanged synthetic measurement bytes."""
    return b'PNN' + bytes((kind, 1, 0)) + bytes(5) + address + bytes(range(40))


class DatasetMacTests(unittest.TestCase):
    """Ensure anonymization retains time-series grouping and measured content."""

    def test_alias_namespace_and_increment_beyond_one_octet(self):
        addresses = {number.to_bytes(6, 'big') for number in range(300)}
        mapping = aliases(addresses)
        self.assertEqual(len(set(mapping.values())), 300)
        self.assertEqual(mapping[bytes(6)], FIRST)
        self.assertEqual(mapping[(299).to_bytes(6, 'big')], bytes.fromhex('00ca0000012c'))
        self.assertEqual(aliases(set(mapping.values())), {value: value for value in mapping.values()})
        self.assertFalse(is_alias(bytes.fromhex('00ca00000000')))
        self.assertFalse(is_alias(bytes.fromhex('aabbccddeeff')))

    def test_multi_cm_archives_names_metadata_and_repeat_runs(self):
        original_macs = [bytes.fromhex('102030405060'), bytes.fromhex('203040506070')]
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            for index, address in enumerate(original_macs):
                data = capture(address)
                folder = root / address.hex()
                folder.mkdir()
                (folder / 'capture.bin').write_bytes(data)
                (folder / 'capture.bin.gz').write_bytes(gzip.compress(data))
                with zipfile.ZipFile(root / f'{index}.zip', 'w') as archive:
                    archive.writestr(f'{address.hex()}/capture.bin', data)
                with tarfile.open(root / f'{index}.tar.gz', 'w:gz') as archive:
                    info = tarfile.TarInfo(f'{address.hex()}/capture.bin')
                    info.size = len(data)
                    archive.addfile(info, io.BytesIO(data))
            (root / 'selection.json').write_text(json.dumps({
                'mac_address': original_macs[0].hex(':'),
                'sha256': hashlib.sha256(capture(original_macs[0])).hexdigest(),
                'original_sha256': hashlib.sha256(capture(original_macs[0])).hexdigest()}))
            with contextlib.redirect_stdout(io.StringIO()), patch.object(TOOL, 'validate_rewrite') as validate:
                TOOL.process_directory(root, write=True, decoder=Path('synthetic-decoder'))
                self.assertGreater(validate.call_count, 0)
                TOOL.process_directory(root)
                TOOL.process_directory(root, write=True, decoder=Path('synthetic-decoder'))
            for index, replacement in enumerate((FIRST, SECOND)):
                self.assertEqual((root / replacement.hex() / 'capture.bin').read_bytes(), capture(replacement))
                with zipfile.ZipFile(root / f'{index}.zip') as archive:
                    self.assertEqual(archive.namelist(), [f'{replacement.hex()}/capture.bin'])
                    self.assertEqual(archive.read(archive.namelist()[0]), capture(replacement))
                with tarfile.open(root / f'{index}.tar.gz') as archive:
                    self.assertEqual(archive.extractfile(archive.getmembers()[0]).read(), capture(replacement))
            metadata = json.loads((root / 'selection.json').read_text())
            self.assertEqual(metadata['mac_address'], '00:ca:00:00:00:01')
            self.assertEqual(metadata['sha256'], hashlib.sha256(capture(FIRST)).hexdigest())
            self.assertEqual(metadata['original_sha256'], hashlib.sha256(capture(original_macs[0])).hexdigest())

    def test_pre_eq_cmts_identity_is_preserved_across_pair(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            cm = bytes.fromhex('102030405060')
            cmts = bytes.fromhex('203040506070')
            for kind in (6, 7):
                (root / f'{kind}.bin').write_bytes(capture(cm, kind)[:17] + cmts + bytes(range(40)))
            plan = Dataset(root, TOOL.field_ranges).plan()
            for _, _, original, updated in plan:
                self.assertEqual(updated[11:17], FIRST)
                self.assertEqual(updated[17:23], SECOND)
                self.assertEqual(updated[:11], original[:11])
                self.assertEqual(updated[23:], original[23:])

    def test_unknown_symlink_traversal_and_limits_fail_before_writes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            original = capture(bytes.fromhex('102030405060'))
            (root / 'good.bin').write_bytes(original)
            (root / 'unknown.bin').write_bytes(b'unknown')
            with self.assertRaises(ValueError), contextlib.redirect_stdout(io.StringIO()):
                TOOL.process_directory(root, write=True)
            self.assertEqual((root / 'good.bin').read_bytes(), original)
            (root / 'unknown.bin').unlink()
            with zipfile.ZipFile(root / 'bad.zip', 'w') as archive:
                archive.writestr('../escape.bin', original)
            with self.assertRaisesRegex(ValueError, 'Unsafe'):
                Dataset(root, TOOL.field_ranges).plan()
            (root / 'bad.zip').unlink()
            with patch('dataset_mac.MAX_EXPANDED_BYTES', 1):
                with self.assertRaisesRegex(ValueError, 'size limit'):
                    Dataset(root, TOOL.field_ranges).plan()
            (root / 'link.bin').symlink_to(root / 'good.bin')
            with self.assertRaisesRegex(ValueError, 'symlink'):
                Dataset(root, TOOL.field_ranges).plan()

    def test_git_inventory_catches_new_demo_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'new-demo.capture').write_bytes(capture(bytes.fromhex('102030405060')))
            result = type('Result', (), {'stdout': b'new-demo.capture\0'})()
            with patch.object(TOOL, 'ROOT', root), patch.object(TOOL.subprocess, 'run', return_value=result):
                with self.assertRaisesRegex(ValueError, 'outside approved'):
                    TOOL.check_tracked_captures()
                (root / 'new-demo.capture').write_bytes(capture(SECOND))
                TOOL.check_tracked_captures()


if __name__ == '__main__':
    unittest.main()
