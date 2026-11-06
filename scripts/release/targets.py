#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Select same-run packages only after their platform's native checks pass."""
import argparse
import json
from pathlib import Path

RUNNERS = {'native-ubuntu-22.04': 'ubuntu-22.04', 'native-windows-2022': 'windows-2022',
           'native-macos-15': 'macos-15', 'native-macos-15-intel': 'macos-15-intel'}


def eligible(jobs, artifacts, ubuntu_result=None):
    """Return allowed artifact names from this run's completed successful jobs.

    jobs/artifacts are GitHub API records, not arbitrary paths. Require global
    CodeQL/code checks. Without ubuntu_result, identify baseline DEBs to test;
    with it, require all Ubuntu native and lifecycle checks before publishing
    Linux. Failed, canceled, duplicate or absent jobs never qualify.
    """
    def passed(suffix):
        matches = [job for job in jobs if job['name'].endswith(' / ' + suffix)]
        return len(matches) == 1 and matches[0]['conclusion'] == 'success'
    if not passed('CodeQL') or not passed('Code checks'):
        return []
    available = {item['name'] for item in artifacts if not item['expired']}
    selected = []
    for name, runner in RUNNERS.items():
        if name not in available or not passed('Native / ' + runner):
            continue
        if runner == 'ubuntu-22.04' and ubuntu_result is not None:
            if ubuntu_result != 'success' or not all(passed('Native / ' + ubuntu)
                    for ubuntu in ('ubuntu-24.04', 'ubuntu-26.04')):
                continue
        selected.append(name)
    return selected


def main():
    """Read paginated same-run API snapshots and append bounded workflow outputs.

    No network, installation or publication occurs. Reject malformed snapshots
    rather than assuming a failed platform passed.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--jobs', type=Path, required=True)
    parser.add_argument('--artifacts', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--ubuntu-result')
    args = parser.parse_args()
    jobs = [job for page in json.loads(args.jobs.read_text()) for job in page['jobs']]
    artifacts = [item for page in json.loads(args.artifacts.read_text()) for item in page['artifacts']]
    names = eligible(jobs, artifacts, args.ubuntu_result)
    with args.output.open('a') as output:
        output.write('ubuntu=' + str('native-ubuntu-22.04' in names).lower() + '\n')
        output.write('names=' + json.dumps(names) + '\n')


if __name__ == '__main__':
    main()
