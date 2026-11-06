#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Partial releases must exclude every platform without successful checks."""
from pathlib import Path
import json
import subprocess
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'scripts/release'))
import targets


class EligibleTargetTests(unittest.TestCase):
    """Model mixed native outcomes without contacting GitHub."""

    def setUp(self):
        self.jobs = [{'name': 'validate / ' + name, 'conclusion': 'success'}
                     for name in ('CodeQL', 'Code checks')]
        self.jobs += [{'name': 'validate / native / Native / ' + runner, 'conclusion': 'success'}
                      for runner in (*targets.RUNNERS.values(), 'ubuntu-24.04', 'ubuntu-26.04')]
        self.jobs += [{'name': 'validate / native-baseline / DEB lifecycle / ' + runner,
                       'conclusion': 'success'}
                      for runner in ('ubuntu-22.04', 'ubuntu-24.04', 'ubuntu-26.04')]
        self.artifacts = [{'name': name, 'expired': False} for name in targets.RUNNERS]

    def test_failed_mac_keeps_successful_windows_and_ubuntu(self):
        self.jobs[5]['conclusion'] = 'failure'
        selected = targets.eligible(self.jobs, self.artifacts, True)
        self.assertIn('native-windows-2022', selected)
        self.assertIn('native-ubuntu-22.04', selected)
        self.assertNotIn('native-macos-15-intel', selected)

    def test_ubuntu_tests_continue_but_failed_lifecycle_excludes_linux(self):
        self.assertIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts))
        for result in ('failure', 'cancelled', 'skipped'):
            self.jobs[-1]['conclusion'] = result
            self.assertNotIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts, True))

    def test_every_ubuntu_native_check_required_for_linux_publication(self):
        self.jobs[7]['conclusion'] = 'failure'
        self.assertIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts))
        self.assertNotIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts, True))

    def test_global_check_failure_blocks_all_publication(self):
        self.jobs[0]['conclusion'] = 'failure'
        self.assertEqual(targets.eligible(self.jobs, self.artifacts, True), [])

    def test_missing_expired_or_duplicate_native_evidence_is_rejected(self):
        self.artifacts[1]['expired'] = True
        self.jobs.append(dict(self.jobs[4]))
        selected = targets.eligible(self.jobs, self.artifacts, True)
        self.assertNotIn('native-windows-2022', selected)
        self.assertNotIn('native-macos-15', selected)
        self.assertEqual(targets.eligible(self.jobs, [], True), [])

    def test_missing_duplicate_or_pending_lifecycle_excludes_linux(self):
        for result in (None, 'failure', 'cancelled', 'skipped'):
            with self.subTest(result=result):
                jobs = [dict(job) for job in self.jobs]
                jobs[-2]['conclusion'] = result
                selected = targets.eligible(jobs, self.artifacts, True)
                self.assertNotIn('native-ubuntu-22.04', selected)
                self.assertIn('native-windows-2022', selected)
        self.assertNotIn('native-ubuntu-22.04',
                         targets.eligible(self.jobs[:-1], self.artifacts, True))
        self.assertNotIn('native-ubuntu-22.04',
                         targets.eligible(self.jobs + [self.jobs[-1]], self.artifacts, True))

    def test_baseline_workflow_name_preserves_native_evidence(self):
        self.jobs[2]['name'] = 'validate / native-baseline / Native / ubuntu-22.04'
        self.assertEqual(set(targets.eligible(self.jobs, self.artifacts, True)),
                         set(targets.RUNNERS))

    def test_cli_reads_paginated_evidence_and_requires_lifecycle_results(self):
        """Exercise the publish invocation with the API's paginated JSON shape."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            jobs = root / 'jobs.json'
            artifacts = root / 'artifacts.json'
            output = root / 'output'
            jobs.write_text(json.dumps([{'jobs': self.jobs[:8]}, {'jobs': self.jobs[8:]}]))
            artifacts.write_text(json.dumps([{'artifacts': self.artifacts}]))
            command = [sys.executable, str(Path(targets.__file__)), '--jobs', str(jobs),
                       '--artifacts', str(artifacts), '--output', str(output),
                       '--require-ubuntu-lifecycle']
            subprocess.run(command, check=True, capture_output=True, text=True)
            fields = dict(line.split('=', 1) for line in output.read_text().splitlines())
            self.assertEqual(set(json.loads(fields['names'])), set(targets.RUNNERS))
            jobs.write_text(json.dumps([{'jobs': self.jobs[:-1]}]))
            output.unlink()
            subprocess.run(command, check=True, capture_output=True, text=True)
            fields = dict(line.split('=', 1) for line in output.read_text().splitlines())
            self.assertNotIn('native-ubuntu-22.04', json.loads(fields['names']))
            self.assertIn('native-windows-2022', json.loads(fields['names']))
