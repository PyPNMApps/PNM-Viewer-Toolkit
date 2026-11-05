#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Native cache reuse preserves executables and cannot write outside build trees."""
import importlib.util
import io
from contextlib import redirect_stdout
import os
import sys
import json
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('native_cache', Path(__file__).resolve().parents[2] / 'scripts/ci/native_cache.py')
CACHE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CACHE)


class NativeCacheTests(unittest.TestCase):
    """Exercise real archives in isolated directories without Cargo or GitHub."""

    def test_round_trip_preserves_mode_and_relative_internal_link(self):
        """Restore runnable build scripts and CXX-Qt header links across tag builds."""
        with tempfile.TemporaryDirectory() as directory, patch.object(CACHE, 'identity', return_value='fixture'):
            root = Path(directory)
            (root / 'local').mkdir()
            executable = root / 'apps/qt/target/debug/build/helper'
            executable.parent.mkdir(parents=True)
            executable.write_bytes(b'synthetic executable')
            executable.chmod(0o755)
            link = root / 'apps/qt/target/debug/alias'
            link.symlink_to(executable)
            with patch.object(CACHE, 'ROOT', root):
                CACHE.pack(root / 'archive')
                executable.write_bytes(b'old')
                CACHE.restore(root / 'archive')
                self.assertEqual(executable.read_bytes(), b'synthetic executable')
                self.assertEqual(executable.stat().st_mode & 0o777, 0o755)
                self.assertTrue(link.is_symlink())
                self.assertFalse(link.readlink().is_absolute())
                self.assertEqual(link.read_bytes(), executable.read_bytes())

    def test_unsafe_members_and_links_do_not_change_existing_outputs(self):
        """Reject traversal, outside links, devices, overlap and excessive sizes."""
        variants = [('../outside', 'file', ''), ('/absolute', 'file', ''),
                    ('docs/source.rs', 'file', ''), ('target/link', 'link', '/etc/passwd'),
                    ('target/link', 'link', '../../outside'), ('target/pipe', 'fifo', ''),
                    ('target/repeat', 'duplicate', ''), ('target/large', 'large', '')]
        for name, kind, target in variants:
            with self.subTest(kind=kind, name=name), tempfile.TemporaryDirectory() as directory:
                root = Path(directory)
                (root / 'local').mkdir()
                original = root / 'target/original'
                original.parent.mkdir()
                original.write_text('preserved')
                source = root / 'archive'
                source.mkdir()
                (source / 'identity.json').write_text(json.dumps({'fingerprint': 'fixture'}))
                with tarfile.open(source / 'native.tar.gz', 'w:gz') as archive:
                    member = tarfile.TarInfo(name)
                    if kind == 'link':
                        member.type = tarfile.SYMTYPE
                        member.linkname = target
                        archive.addfile(member)
                    elif kind == 'fifo':
                        member.type = tarfile.FIFOTYPE
                        archive.addfile(member)
                    else:
                        member.size = 2
                        archive.addfile(member, io.BytesIO(b'ab'))
                        if kind == 'duplicate':
                            archive.addfile(member, io.BytesIO(b'ab'))
                limit = 1 if kind == 'large' else CACHE.MAX_BYTES
                with patch.object(CACHE, 'ROOT', root), patch.object(CACHE, 'identity', return_value='fixture'), patch.object(CACHE, 'MAX_BYTES', limit):
                    with self.assertRaises(ValueError):
                        CACHE.restore(source)
                self.assertEqual(original.read_text(), 'preserved')
                self.assertFalse((root / 'outside').exists())

    def test_only_dependency_outputs_are_reused(self):
        """Do not carry workspace binaries/tests or incremental caches across tags."""
        names = {'pnm', 'pnm-viewer-qt', 'test_rxmer'}
        for path in ('target/debug/deps/test_rxmer-abc', 'target/debug/deps/libpnm-abc.rlib',
                     'apps/qt/target/debug/build/pnm-viewer-qt-abc/out/file',
                     'target/debug/incremental/session/file', 'target/debug/examples/example'):
            self.assertFalse(CACHE.reusable(path, names), path)
        for path in ('apps/qt/target/debug/deps/libcxx_qt_lib-abc.rlib',
                     'apps/qt/target/release/build/cxx-qt-lib-abc/out/file',
                     'local/ccache/abc/file'):
            self.assertTrue(CACHE.reusable(path, names), path)

    def test_fingerprint_mismatch_keeps_current_outputs(self):
        """A toolchain/ABI mismatch is a normal cold fallback, never a test skip."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'identity.json').write_text(json.dumps({'fingerprint': 'other'}))
            with patch.object(CACHE, 'identity', return_value='fixture'):
                CACHE.restore(root)

    def test_failed_cache_export_is_not_uploaded_or_a_test_bypass(self):
        """Oversized/broken optional exports do not invalidate tested packages."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / 'github-output'
            diagnostics = io.StringIO()
            with patch.dict(os.environ, {'GITHUB_OUTPUT': str(output)}), \
                    patch.object(sys, 'argv', ['native_cache.py', 'pack']), \
                    patch.object(CACHE, 'workspace_artifacts', return_value=set()), \
                    patch.object(CACHE, 'pack', side_effect=ValueError('synthetic size limit')), \
                    redirect_stdout(diagnostics):
                CACHE.main()
            self.assertEqual(output.read_text(), 'ready=false\n')
            self.assertIn('::warning::Native cache export unavailable: synthetic size limit',
                          diagnostics.getvalue())

    def test_source_selection_requires_successful_workflow_ancestor_tag(self):
        """Ignore branch runs, non-ancestor commits and expired/missing artifacts."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / 'output'
            candidates = [
                {'id': 1, 'head_branch': 'master', 'head_sha': 'a' * 40},
                {'id': 2, 'head_branch': 'v0.1.0', 'head_sha': 'b' * 40},
                {'id': 3, 'head_branch': 'v0.2.0', 'head_sha': 'c' * 40},
                {'id': 4, 'head_branch': 'v0.3.0', 'head_sha': 'd' * 40},
            ]
            def run(*arguments):
                if 'workflows/' in arguments[-1]:
                    self.assertIn('event=push&status=success', arguments[-1])
                    return json.dumps({'workflow_runs': candidates})
                expired = '/3/' in arguments[-1]
                return json.dumps({'artifacts': [{'name': CACHE.ARTIFACT_NAME + '-fixture', 'expired': expired, 'size_in_bytes': 100}]})
            with patch.object(CACHE, 'identity', return_value='fixture'), patch.object(CACHE, 'run', side_effect=run), patch.object(CACHE.subprocess, 'run') as git:
                git.side_effect = [type('Result', (), {'returncode': value})() for value in (1, 0, 0)]
                CACHE.select_run('owner/repo', output)
            self.assertEqual(output.read_text(), 'run_id=4\nartifact_name=' + CACHE.ARTIFACT_NAME + '-fixture\n')


if __name__ == '__main__':
    unittest.main()
