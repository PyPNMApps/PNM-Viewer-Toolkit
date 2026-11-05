#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Render an installation-focused draft page from a verified release index."""
import argparse
import json
from pathlib import Path
import re
from urllib.parse import quote

from common import MAX_PACKAGE_BYTES, ROOT, release_version
from collect import EXPECTED_TARGETS
from verify import verify

PLATFORMS = {
    ('Linux', 'x64'): 'Ubuntu 22.04 / 24.04 / 26.04 x64',
    ('Windows', 'x64'): 'Windows x64',
    ('Darwin', 'arm64'): 'macOS Apple Silicon (arm64)',
    ('Darwin', 'x64'): 'macOS Intel (x64)',
}


def render(template, manifests, version, tag, repository, ubuntu_only=False):
    """Render a tag-pinned page without uploading or changing release state.

    Args:
        template: Markdown with the documented uppercase template tokens.
        manifests: Complete platform manifests produced by collect.py.
        version: Validated canonical VERSION string.
        tag: Stable release tag, which must equal v plus version.
        repository: GitHub owner/name used for download and source links.
        ubuntu_only: Explicit temporary scope; require the paired Ubuntu DEBs.
    Returns:
        Markdown with measured download sizes and links to the verified assets.
    Raises:
        ValueError: Tag mismatch, unsafe names, incomplete/mixed targets,
            invalid artifact sizes, missing installers/CLI or unknown tokens.
    """
    if tag != 'v' + version:
        raise ValueError('Release tag does not match VERSION')
    # Whole owner/name validation; PyPNMApps/PNM-Viewer-Toolkit passes.
    # Two name segments exclude URLs, whitespace and extra slashes.
    if not re.fullmatch(r'[A-Za-z0-9_-]+/[A-Za-z0-9_.-]+', repository):
        raise ValueError('Expected a GitHub owner/repository')
    platforms = PLATFORMS
    expected_targets = EXPECTED_TARGETS
    if ubuntu_only:
        platforms = {('Linux', 'x64'): 'Ubuntu 22.04 x64 only'}
        expected_targets = set(platforms)
    required_packages = {
        ('Linux', 'x64'): {f'pnm-viewer-toolkit_{version}_amd64.deb', f'pnm-decode_{version}_amd64.deb'},
        ('Windows', 'x64'): {f'PNM-Viewer-Toolkit-{version}-windows-x64.msi', f'pnm-decode-{version}-windows-x64.zip'},
        ('Darwin', 'arm64'): {f'PNM-Viewer-Toolkit-{version}-macos-arm64.dmg', f'pnm-decode-{version}-darwin-arm64.tar.gz'},
        ('Darwin', 'x64'): {f'PNM-Viewer-Toolkit-{version}-macos-x64.dmg', f'pnm-decode-{version}-darwin-x64.tar.gz'},
    }
    base = 'https://github.com/' + repository
    download = base + '/releases/download/' + tag + '/'
    targets = {}
    names = set()
    for manifest in manifests:
        target = (manifest['system'], manifest['architecture'])
        if target in targets or target not in expected_targets:
            raise ValueError('Duplicate or unexpected release target')
        if manifest['version'] != version:
            raise ValueError('Mixed release versions')
        viewer, cli = [], []
        for artifact in manifest['artifacts']:
            name, size = artifact['name'], artifact['bytes']
            # Whole asset basename, e.g. pnm-decode_1.2.3_amd64.deb;
            # exclude separators, Markdown punctuation and whitespace.
            if not re.fullmatch(r'[A-Za-z0-9_.-]+', name) or name in ('.', '..') or name in names:
                raise ValueError('Unsafe or duplicate release asset name')
            if type(size) is not int or not 0 < size <= MAX_PACKAGE_BYTES:
                raise ValueError('Invalid package download size')
            names.add(name)
            link = f'[{name}]({download}{quote(name)}) ({size / 1_000_000:.2f} MB)'
            if name.startswith('pnm-decode'):
                cli.append(link)
            elif name.startswith(('PNM-Viewer-Toolkit-', 'pnm-viewer-toolkit_')):
                viewer.append(link)
            else:
                raise ValueError('Unrecognized release package')
        if ubuntu_only and not {
            f'pnm-viewer-toolkit_{version}_amd64.deb',
            f'pnm-decode_{version}_amd64.deb',
        }.issubset(names):
            raise ValueError('Ubuntu draft requires matching viewer and CLI DEBs')
        if len(viewer) != 1 or not cli:
            raise ValueError('Each target needs one viewer installer and CLI packages')
        target_names = {artifact['name'] for artifact in manifest['artifacts']}
        if not required_packages[target].issubset(target_names):
            raise ValueError('Target installer or CLI package missing; require MSI/DMG/DEB for the correct architecture')
        targets[target] = ' | '.join((platforms[target], '<br>'.join(viewer), '<br>'.join(cli)))
    if set(targets) != expected_targets:
        raise ValueError('Incomplete release platform set')
    table = ['| Platform | Desktop installer | CLI only |', '| --- | --- | --- |']
    table.extend('| ' + targets[target] + ' |' for target in platforms)
    replacements = {
        '{{TAG}}': tag,
        '{{VERSION}}': version,
        '{{ASSET_TABLE}}': '\n'.join(table),
        '{{SOURCE_URL}}': base + '/blob/' + tag,
        '{{RUNS_URL}}': base + '/actions?query=' + quote('event:push branch:' + tag),
    }
    for token, value in replacements.items():
        template = template.replace(token, value)
    if '{{' in template or '}}' in template:
        raise ValueError('Unresolved release-page template token')
    return template


def main():
    """Write a fresh release-page Markdown file from validated local metadata.

    Resolve source files from ROOT; never overwrite output, publish, install,
    create tags or send GitHub requests. Return nonzero on invalid metadata.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--assets', type=Path, required=True, help='Verified collected asset directory')
    parser.add_argument('--tag', required=True)
    parser.add_argument('--repository', default='PyPNMApps/PNM-Viewer-Toolkit')
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--ubuntu-only', action='store_true',
                        help='Render a temporary Ubuntu-only draft from verified packages')
    args = parser.parse_args()
    try:
        if args.ubuntu_only:
            manifests = [verify(args.assets)]
            template_path = ROOT / 'docs/release/ubuntu-deb-notes.md'
        else:
            manifests = json.loads((args.assets / 'release-index.json').read_text(encoding='utf-8'))
            template_path = ROOT / 'docs/release/release-notes.md'
        template = template_path.read_text(encoding='utf-8')
        notes = render(template, manifests, release_version(), args.tag, args.repository,
                       ubuntu_only=args.ubuntu_only)
        with args.output.open('x', encoding='utf-8') as output:
            output.write(notes)
    except (ValueError, KeyError, TypeError, OSError) as error:
        parser.exit(1, f'ERROR: {error}\n')


if __name__ == '__main__':
    main()
