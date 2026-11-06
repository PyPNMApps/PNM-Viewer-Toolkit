#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Disk cleanup is threshold-bound, checkout-scoped and Cargo-lock-aware."""
import importlib.util
from pathlib import Path
import tempfile
import unittest
from collections import namedtuple
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('disk_guard', Path(__file__).resolve().parents[2] / 'tools/maintenance/rust_clean.py')
GUARD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GUARD)
Usage = namedtuple('Usage', 'total used free')


class DiskGuardTests(unittest.TestCase):
    """Never delete real build outputs while testing cleanup decisions."""

    def test_below_threshold_does_not_clean(self):
        with patch.object(GUARD.shutil, 'disk_usage', return_value=Usage(100, 94, 6)), \
                patch.object(GUARD.subprocess, 'run') as clean:
            GUARD.ensure_space()
            clean.assert_not_called()

    def test_threshold_calls_cargo_clean_and_rechecks_space(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            for manifest, target in GUARD.BUILD_TARGETS:
                (root / target).mkdir(parents=True)
                (root / target / 'CACHEDIR.TAG').write_text(GUARD.CACHE_SIGNATURE)
                (root / manifest).write_text('[workspace]')
            with patch.object(GUARD.shutil, 'disk_usage', side_effect=[Usage(100, 95, 5), Usage(100, 70, 30)]), \
                    patch.object(GUARD.subprocess, 'check_output', return_value=b''), \
                    patch.object(GUARD.subprocess, 'run') as clean:
                GUARD.ensure_space(root)
            self.assertEqual(clean.call_count, 2)
            for call, (manifest, target) in zip(clean.call_args_list, GUARD.BUILD_TARGETS):
                self.assertEqual(call.args[0], ['cargo', 'clean', '--manifest-path', str(root / manifest),
                                               '--target-dir', str(root / target)])
                self.assertTrue(call.kwargs['check'])

    def test_unmarked_or_symlink_outputs_are_preserved(self):
        for redirected in (False, True):
            with self.subTest(redirected=redirected), tempfile.TemporaryDirectory() as directory:
                root = Path(directory).resolve()
                (root / 'Cargo.toml').write_text('[workspace]')
                if redirected:
                    (root / 'outside').mkdir()
                    (root / 'target').symlink_to(root / 'outside', target_is_directory=True)
                else:
                    (root / 'target').mkdir()
                with patch.object(GUARD.shutil, 'disk_usage', return_value=Usage(100, 99, 1)), \
                        patch.object(GUARD.subprocess, 'check_output', return_value=b''), \
                        patch.object(GUARD.subprocess, 'run') as clean:
                    with self.assertRaisesRegex(ValueError, 'Refusing'):
                        GUARD.ensure_space(root)
                    clean.assert_not_called()

    def test_still_full_fails_before_build(self):
        with tempfile.TemporaryDirectory() as directory, \
                patch.object(GUARD.shutil, 'disk_usage', return_value=Usage(100, 99, 1)), \
                patch.object(GUARD.subprocess, 'check_output', return_value=b''), \
                patch.object(GUARD.subprocess, 'run'):
            with self.assertRaisesRegex(ValueError, 'remains'):
                GUARD.ensure_space(Path(directory))

    def test_tracked_content_is_never_cleaned(self):
        with patch.object(GUARD.subprocess, 'check_output', return_value=b'target/source.rs\0'), \
                patch.object(GUARD.subprocess, 'run') as clean:
            with self.assertRaisesRegex(ValueError, 'tracked'):
                GUARD.clean()
            clean.assert_not_called()

    def test_manual_dry_run_preserves_marked_outputs(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory).resolve()
            (root / 'target').mkdir()
            (root / 'target/CACHEDIR.TAG').write_text(GUARD.CACHE_SIGNATURE)
            (root / 'Cargo.toml').write_text('[workspace]')
            with patch.object(GUARD.subprocess, 'check_output', return_value=b''), \
                    patch.object(GUARD.subprocess, 'run') as clean:
                GUARD.clean(root, dry_run=True)
                clean.assert_not_called()
            self.assertTrue((root / 'target/CACHEDIR.TAG').exists())


if __name__ == '__main__':
    unittest.main()
