#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Attach verified assets to a draft, verify uploaded bytes, then publish it."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile


def github(*arguments):
    """Run GitHub CLI without a shell; propagate authentication/API failures.

    Arguments are discrete CLI arguments. Return stdout on success; never print
    credentials or signed artifact URLs. The caller owns release authorization.
    """
    try:
        return subprocess.run(['gh', *map(str, arguments)], check=True,
                              capture_output=True, text=True).stdout
    except subprocess.CalledProcessError as error:
        # Captured CLI diagnostics contain the actionable HTTP/flag error;
        # without them CI prints only a generic CalledProcessError traceback.
        if error.stderr:
            print(error.stderr, file=sys.stderr, flush=True)
        raise


def publish(assets, notes, tag, repository, title, partial=False):
    """Publish a tested tag only after exact remote asset bytes are verified.

    Args:
        assets: Directory already verified by the upstream artifact gate. Its
            release-index.json is local renderer input, never a public asset.
        notes: Rendered version-specific installation instructions.
        tag: Validated tag from the workflow's identity gate.
        repository: GitHub owner/repository, passed explicitly to every gh call.
        title: Public development-release title stating its platform scope.
        partial: Publish as prerelease without replacing the latest stable release.
    Returns:
        None after publication. Complete releases are latest; partial releases
        are prereleases and preserve the previous stable release as latest.
    Raises:
        ValueError: Existing public release, invalid/empty local asset directory,
            mismatched upload or missing GitHub SHA-256 package digests.
        subprocess.CalledProcessError: Failed API, upload, download or publication.
    """
    entries = sorted(assets.iterdir())
    if any(not path.is_file() or path.is_symlink() for path in entries):
        raise ValueError('Expected regular verified release assets')
    paths = [path for path in entries if path.name != 'release-index.json']
    if not paths:
        raise ValueError('Expected regular verified release assets')
    if any(path.name != 'SHA256SUMS' and path.suffix not in ('.deb', '.msi', '.dmg')
           for path in paths):
        raise ValueError('Public releases contain only installers and SHA256SUMS')
    # Listing includes drafts and distinguishes API failure from an absent tag.
    releases = json.loads(github('api', '--paginate', '--slurp',
                                 f'repos/{repository}/releases?per_page=100'))
    matching = [release for page in releases for release in page if release['tag_name'] == tag]
    if len(matching) > 1:
        raise ValueError('Duplicate release identity')
    if matching:
        if not matching[0]['draft']:
            raise ValueError('Refusing to modify a public release')
        github('release', 'edit', tag, '--repo', repository, '--title', title, '--notes-file', notes)
    else:
        github('release', 'create', tag, '--repo', repository, '--verify-tag', '--draft',
               '--title', title, '--notes-file', notes)
    github('release', 'upload', tag, *paths, '--repo', repository, '--clobber')
    pages = json.loads(github('api', '--paginate', '--slurp',
                              f'repos/{repository}/releases?per_page=100'))
    matching = [release for page in pages for release in page if release['tag_name'] == tag]
    if len(matching) != 1:
        raise ValueError('Expected exactly one staged release')
    metadata = matching[0]
    remote = metadata['assets']
    expected = {path.name: path.stat().st_size for path in paths}
    if not metadata['draft'] or len(remote) != len(expected) or {
        asset['name']: asset['size'] for asset in remote if asset['state'] == 'uploaded'
    } != expected:
        raise ValueError('Uploaded release asset set does not match verified artifacts')
    for asset in remote:
        if asset['name'].endswith(('.deb', '.msi', '.dmg')):
            digest = 'sha256:' + hashlib.sha256((assets / asset['name']).read_bytes()).hexdigest()
            if asset.get('digest') != digest:
                raise ValueError('GitHub package digest does not match verified bytes')
    with tempfile.TemporaryDirectory(prefix='pnm-release-check-') as directory:
        github('release', 'download', tag, '--repo', repository, '--dir', directory)
        for source in paths:
            downloaded = Path(directory) / source.name
            if not downloaded.is_file() or hashlib.sha256(source.read_bytes()).digest() != hashlib.sha256(downloaded.read_bytes()).digest():
                raise ValueError('Downloaded release bytes do not match verified artifacts')
    visibility = ['--prerelease', '--latest=false'] if partial else ['--latest']
    github('release', 'edit', tag, '--repo', repository, '--draft=false', *visibility)


def main():
    """Publish only the workflow-authorized tag and explicitly supplied artifacts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--assets', type=Path, required=True)
    parser.add_argument('--notes', type=Path, required=True)
    parser.add_argument('--tag', required=True)
    parser.add_argument('--repository', required=True)
    parser.add_argument('--title', required=True)
    parser.add_argument('--partial', action='store_true')
    args = parser.parse_args()
    publish(args.assets, args.notes, args.tag, args.repository, args.title, args.partial)


if __name__ == '__main__':
    main()
