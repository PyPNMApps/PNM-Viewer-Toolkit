#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Fixture sanitization must never rewrite measurement payloads or skip failures."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('fixture_mac', Path(__file__).with_name('pnm-mac-updater.py'))
TOOL = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TOOL)


class FixtureMacTests(unittest.TestCase):
    """Exercise real fixture layouts and deliberately malformed synthetic inputs."""

    def test_user_alias_formats_and_invalid_input(self):
        """Accept equivalent formats; reject malformed groups and wrong lengths."""
        expected = bytes.fromhex('aabbccddeeff')
        for text in ('aabb.ccdd.eeff', 'AA:BB:CC:DD:EE:FF', 'aa-bb-cc-dd-ee-ff', 'aabbccddeeff'):
            self.assertEqual(TOOL.normalize_mac(text), expected)
        for text in ('abcd', 'aabbcc.dd.eeff', 'aa:bb:cc:dd:ee:gg', 'aa:bb-cc:dd:ee:ff:00'):
            with self.assertRaises(ValueError):
                TOOL.normalize_mac(text)
        with self.assertRaises(ValueError):
            TOOL.rewrite(b'PNN\x04\x01\x00' + bytes(20), b'short')

    def test_only_mac_fields_change_even_if_payload_repeats_mac_bytes(self):
        """A matching sequence elsewhere in a file must never be rewritten."""
        address = bytes.fromhex('aabbccddeeff')
        for signature, fields in TOOL.MAC_FIELDS.items():
            original = signature + bytes((1, 0)) + bytes(range(80))
            for _, offset in fields:
                original = original[:offset] + b'ABCDEF' + original[offset + 6:]
            original += b'ABCDEF'
            updated = TOOL.rewrite(original, address)
            changed = {index for _, offset in fields for index in range(offset, offset + 6)}
            self.assertEqual(len(original), len(updated))
            self.assertEqual(updated[-6:], b'ABCDEF')
            for index in range(len(original)):
                if index not in changed:
                    self.assertEqual(original[index], updated[index])
            self.assertEqual(TOOL.rewrite(updated, address), updated)
        for data in (b'PNN\x01\x01\x00', b'PNN\x04\x02\x00' + bytes(80), b'PNN\x06\x01\x00'):
            with self.assertRaises(ValueError):
                TOOL.field_ranges(data)

    def test_real_fixtures_and_documented_examples_use_one_alias(self):
        """Check the shipped nine captures/eleven fields and all JSON examples."""
        address = TOOL.normalize_mac(TOOL.DEFAULT_MAC)
        with contextlib.redirect_stdout(io.StringIO()):
            TOOL.process(address)
        TOOL.check_examples(address)
        files = list((TOOL.FIXTURES / 'tests/files').glob('*.bin'))
        self.assertEqual(len(files), 9)
        self.assertEqual(sum(len(TOOL.field_ranges(path.read_bytes())) for path in files), 11)

    def test_checksum_unknown_fixture_and_dry_run_fail_closed(self):
        """Checks are read-only and fail rather than silently skipping bad files."""
        with tempfile.TemporaryDirectory() as directory:
            copied = Path(directory) / 'fixtures'
            shutil.copytree(TOOL.FIXTURES, copied)
            fixture = copied / 'tests/files/rxmer.bin'
            original = fixture.read_bytes()
            with patch.object(TOOL, 'FIXTURES', copied), contextlib.redirect_stdout(io.StringIO()):
                TOOL.process(bytes(6), dry_run=True)
                self.assertEqual(fixture.read_bytes(), original)
                with self.assertRaises(ValueError):
                    TOOL.process(bytes(6))
                fixture.write_bytes(original + b'x')
                with self.assertRaisesRegex(ValueError, 'checksum'):
                    TOOL.process(TOOL.normalize_mac(TOOL.DEFAULT_MAC))
                fixture.write_bytes(original)
                (copied / 'tests/files/unknown.bin').write_bytes(b'unknown')
                with self.assertRaisesRegex(ValueError, 'provenance'):
                    TOOL.process(TOOL.normalize_mac(TOOL.DEFAULT_MAC))


if __name__ == '__main__':
    unittest.main()
