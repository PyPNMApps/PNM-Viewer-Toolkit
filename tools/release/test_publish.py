#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Publication is last, gated by uploaded metadata and downloaded bytes."""
import hashlib
import importlib.util
import json
import io
from contextlib import redirect_stderr
from pathlib import Path
import tempfile
import subprocess
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location('publish_release', Path(__file__).resolve().parents[2] / 'scripts/release/publish.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class PublishTests(unittest.TestCase):
    """Use an entirely synthetic GitHub service; no network/publication occurs."""

    def test_cli_failure_reports_stderr_and_preserves_original_error(self):
        """Expose the API error instead of losing it behind a generic traceback."""
        failure = subprocess.CalledProcessError(
            1, ['gh', 'release', 'create'], stderr='HTTP 403: synthetic permission failure')
        diagnostics = io.StringIO()
        with patch.object(MODULE.subprocess, 'run', side_effect=failure), \
                redirect_stderr(diagnostics):
            with self.assertRaises(subprocess.CalledProcessError) as caught:
                MODULE.github('release', 'create', 'v0.2.2')
        self.assertIs(caught.exception, failure)
        self.assertIn('HTTP 403: synthetic permission failure', diagnostics.getvalue())

    def exercise(self, mode, filename="test.deb", partial=False):
        """Execute the real coordinator with success, API failure or corrupt assets."""
        with tempfile.TemporaryDirectory() as directory:
            assets = Path(directory) / 'assets'
            assets.mkdir()
            (assets / filename).write_bytes(b'package')
            calls = []
            def github(*arguments):
                calls.append(arguments)
                if arguments[0] == 'api':
                    if mode == 'api-error':
                        raise RuntimeError('API failed')
                    if not any(call[:2] == ('release', 'upload') for call in calls):
                        return json.dumps([[]] if mode == 'new' else [[{'tag_name': 'v0.2.0', 'draft': mode != 'public'}]])
                    digest = 'sha256:' + hashlib.sha256(b'package').hexdigest()
                    return json.dumps([[{'tag_name': 'v0.2.0', 'draft': True, 'assets': [{
                        'name': filename, 'state': 'uploaded',
                        'size': 8 if mode == 'size' else 7,
                        'digest': None if mode == 'digest' else digest}]}]])
                if arguments[:2] == ('release', 'download'):
                    destination = Path(arguments[arguments.index('--dir') + 1])
                    (destination / filename).write_bytes(b'corrupt' if mode == 'bytes' else b'package')
                return ''
            with patch.object(MODULE, 'github', side_effect=github):
                if mode in ('ok', 'new'):
                    MODULE.publish(assets, Path('notes'), 'v0.2.0', 'owner/repo', 'Title', partial)
                    self.assertIn('--draft=false', calls[-1])
                    self.assertIn('--latest=false' if partial else '--latest', calls[-1])
                    if partial:
                        self.assertIn('--prerelease', calls[-1])
                else:
                    with self.assertRaises((ValueError, RuntimeError)):
                        MODULE.publish(assets, Path('notes'), 'v0.2.0', 'owner/repo', 'Title', partial)
                    self.assertFalse(any('--draft=false' in call for call in calls))
                    if mode in ('public', 'api-error'):
                        self.assertFalse(any(call[:2] == ('release', 'upload') for call in calls))

    def test_publishes_only_verified_bytes(self):
        """Publish last after successful remote metadata and download verification."""
        self.exercise('ok')
        self.exercise('new')

    def test_partial_release_is_verified_prerelease_and_not_latest(self):
        for mode in ('new', 'ok', 'public', 'size', 'digest', 'bytes'):
            self.exercise(mode, partial=True)

    def test_msi_and_dmg_digests_are_verified_before_publication(self):
        """Both new platform installer types require matching GitHub SHA-256."""
        for filename in ('test.msi', 'test.dmg'):
            with self.subTest(filename=filename):
                self.exercise('ok', filename)
                self.exercise('digest', filename)

    def test_blocks_public_api_failure_corrupt_or_missing_assets(self):
        """Every failure stops before publication; public assets remain untouched."""
        for mode in ('public', 'api-error', 'size', 'digest', 'bytes'):
            with self.subTest(mode=mode):
                self.exercise(mode)


if __name__ == '__main__':
    unittest.main()
