#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Partial releases must exclude every platform without successful checks."""
from pathlib import Path
import sys
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
        self.artifacts = [{'name': name, 'expired': False} for name in targets.RUNNERS]

    def test_failed_mac_keeps_successful_windows_and_ubuntu(self):
        self.jobs[5]['conclusion'] = 'failure'
        selected = targets.eligible(self.jobs, self.artifacts, 'success')
        self.assertIn('native-windows-2022', selected)
        self.assertIn('native-ubuntu-22.04', selected)
        self.assertNotIn('native-macos-15-intel', selected)

    def test_ubuntu_tests_continue_but_failed_lifecycle_excludes_linux(self):
        self.assertIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts))
        for result in ('failure', 'cancelled', 'skipped'):
            self.assertNotIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts, result))

    def test_every_ubuntu_native_check_required_for_linux_publication(self):
        self.jobs[-1]['conclusion'] = 'failure'
        self.assertIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts))
        self.assertNotIn('native-ubuntu-22.04', targets.eligible(self.jobs, self.artifacts, 'success'))

    def test_global_check_failure_blocks_all_publication(self):
        self.jobs[0]['conclusion'] = 'failure'
        self.assertEqual(targets.eligible(self.jobs, self.artifacts, 'success'), [])

    def test_missing_expired_or_duplicate_native_evidence_is_rejected(self):
        self.artifacts[1]['expired'] = True
        self.jobs.append(dict(self.jobs[4]))
        selected = targets.eligible(self.jobs, self.artifacts, 'success')
        self.assertNotIn('native-windows-2022', selected)
        self.assertNotIn('native-macos-15', selected)
        self.assertEqual(targets.eligible(self.jobs, [], 'success'), [])
