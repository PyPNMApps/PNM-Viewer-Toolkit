#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise release-page links, version agreement and safe local rendering."""
import copy
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts/release'))
import notes
from common import release_version


def manifests(version):
    """Build synthetic manifests without reading native packages.

    Args:
        version: Version to use in every manifest and package basename.
    Returns:
        Four platform manifests with deliberately synthetic download sizes.
    """
    names = {
        ('Linux', 'x64'): [f'pnm-viewer-toolkit_{version}_amd64.deb',
                           f'pnm-decode_{version}_amd64.deb', f'pnm-decode-{version}-linux-x64.tar.gz'],
        ('Windows', 'x64'): [f'PNM-Viewer-Toolkit-{version}-windows-x64.msi',
                             f'pnm-decode-{version}-windows-x64.zip'],
        ('Darwin', 'arm64'): [f'PNM-Viewer-Toolkit-{version}-macos-arm64.dmg',
                              f'pnm-decode-{version}-darwin-arm64.tar.gz'],
        ('Darwin', 'x64'): [f'PNM-Viewer-Toolkit-{version}-macos-x64.dmg',
                            f'pnm-decode-{version}-darwin-x64.tar.gz'],
    }
    return [{'version': version, 'system': system, 'architecture': architecture,
             'artifacts': [{'name': name, 'bytes': 2_200_000} for name in filenames]}
            for (system, architecture), filenames in names.items()]


class ReleasePageTests(unittest.TestCase):
    """Keep the page consistent with the tagged source and collected packages."""

    def setUp(self):
        """Use a synthetic future version so tests detect hard-coded release values."""
        self.version = '12.34.56'
        self.tag = 'v' + self.version
        self.index = manifests(self.version)
        self.template = (ROOT / 'docs/release/release-notes.md').read_text(encoding='utf-8')

    def render(self, **overrides):
        """Render the actual template with isolated synthetic package metadata."""
        arguments = dict(template=self.template, manifests=self.index, version=self.version,
                         tag=self.tag, repository='PyPNMApps/PNM-Viewer-Toolkit')
        arguments.update(overrides)
        return notes.render(**arguments)

    def test_complete_page_has_real_asset_links_and_tagged_guides(self):
        """Link every collected artifact and retain platform install/uninstall steps."""
        page = self.render()
        base = 'https://github.com/PyPNMApps/PNM-Viewer-Toolkit'
        for manifest in self.index:
            for artifact in manifest['artifacts']:
                link = base + '/releases/download/' + self.tag + '/' + artifact['name']
                if artifact['name'].endswith(('.deb', '.msi', '.dmg')):
                    self.assertIn(link, page)
                else:
                    self.assertNotIn(link, page)
        self.assertIn('(2.20 MB)', page)
        self.assertIn(base + '/blob/' + self.tag + '/docs/ui/README.md', page)
        self.assertIn('sudo apt install ./pnm-decode_12.34.56_amd64.deb', page)
        self.assertIn('pnm-viewer-toolkit_12.34.56_amd64.deb', page)
        self.assertIn('pnm-decode.exe', page)
        self.assertIn('Applications', page)
        self.assertIn('SHA256SUMS', page)
        self.assertIn('Ctrl+O', page)
        self.assertIn('Under development', page)
        self.assertNotIn('{{', page)
        self.assertNotIn('](../', page)

    def test_installer_only_index_needs_no_cli_archives(self):
        """Render the five installer downloads without broken archive links."""
        index = copy.deepcopy(self.index)
        for manifest in index:
            manifest['artifacts'] = [item for item in manifest['artifacts']
                                     if item['name'].endswith(('.deb', '.msi', '.dmg'))]
        page = self.render(manifests=index)
        self.assertEqual(page.count('Included in desktop installer'), 3)
        self.assertNotIn('/releases/download/' + self.tag + '/pnm-decode-', page)

    def test_windows_requires_msi_and_mac_requires_matching_architecture(self):
        """Do not publish legacy EXEs or mislabeled DMGs behind new OS instructions."""
        for index, suffix in ((1, 'windows-x64.exe'), (2, 'macos-x64.dmg')):
            changed = copy.deepcopy(self.index)
            changed[index]['artifacts'][0]['name'] = f'PNM-Viewer-Toolkit-{self.version}-{suffix}'
            with self.assertRaisesRegex(ValueError, 'Target installer|duplicate'):
                self.render(manifests=changed)

    def test_partial_page_lists_only_verified_downloads_and_missing_platforms(self):
        template = (ROOT / 'docs/release/partial-notes.md').read_text()
        page = self.render(template=template, manifests=self.index[:2], partial=True)
        self.assertIn('Partial release', page)
        self.assertIn('CI failed', page)
        self.assertIn('prerelease', page)
        self.assertIn('macOS Intel', page)
        self.assertIn('windows-x64.msi', page)
        self.assertNotIn('macos-arm64.dmg', page)
        with self.assertRaisesRegex(ValueError, 'Incomplete'):
            self.render(template=template, manifests=[], partial=True)

    def test_mismatched_tag_or_mixed_version_is_rejected(self):
        """Prevent a release page from pointing at another version's assets."""
        with self.assertRaisesRegex(ValueError, 'tag'):
            self.render(tag='v1.0.0')
        modified = copy.deepcopy(self.index)
        modified[0]['version'] = '1.0.0'
        with self.assertRaisesRegex(ValueError, 'Mixed'):
            self.render(manifests=modified)

    def test_missing_duplicate_targets_or_packages_are_rejected(self):
        """Do not describe an incomplete or ambiguous platform asset set."""
        with self.assertRaisesRegex(ValueError, 'Incomplete'):
            self.render(manifests=self.index[:-1])
        with self.assertRaisesRegex(ValueError, 'target'):
            self.render(manifests=self.index + [self.index[0]])
        modified = copy.deepcopy(self.index)
        modified[0]['artifacts'] = modified[0]['artifacts'][1:]
        with self.assertRaisesRegex(ValueError, 'viewer'):
            self.render(manifests=modified)
        modified = copy.deepcopy(self.index)
        modified[0]['artifacts'].append(modified[0]['artifacts'][0])
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            self.render(manifests=modified)

    def test_unsafe_links_sizes_and_unresolved_tokens_are_rejected(self):
        """Reject path/Markdown injection, invalid byte counts and incomplete templates."""
        for name in ('../capture.deb', '[capture].deb', 'line\nbreak.exe', '..'):
            modified = copy.deepcopy(self.index)
            modified[0]['artifacts'][0]['name'] = name
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'asset name'):
                self.render(manifests=modified)
        for size in (0, True, -1, 50_000_001):
            modified = copy.deepcopy(self.index)
            modified[0]['artifacts'][0]['bytes'] = size
            with self.subTest(size=size), self.assertRaisesRegex(ValueError, 'size'):
                self.render(manifests=modified)
        with self.assertRaisesRegex(ValueError, 'owner/repository'):
            self.render(repository='https://example.test/repository')
        with self.assertRaisesRegex(ValueError, 'template token'):
            self.render(template=self.template + '\n{{UNKNOWN}}')

    def test_ubuntu_scope_is_explicit_and_does_not_weaken_full_release(self):
        """Render paired DEBs in temporary mode; retain full target requirements."""
        template = (ROOT / 'docs/release/ubuntu-deb-notes.md').read_text(encoding='utf-8')
        page = self.render(template=template, manifests=self.index[:1], ubuntu_only=True)
        self.assertIn('Ubuntu 22.04 x64 only', page)
        self.assertIn(f'pnm-decode_{self.version}_amd64.deb', page)
        self.assertIn('automatically publishes', ' '.join(page.split()))
        with self.assertRaisesRegex(ValueError, 'Incomplete'):
            self.render(manifests=self.index[:1])
        with self.assertRaisesRegex(ValueError, 'unexpected'):
            self.render(template=template, ubuntu_only=True)

    def test_ubuntu_draft_requires_both_debs(self):
        """An archive alone cannot stand in for the CLI's DEB dependency."""
        modified = copy.deepcopy(self.index[:1])
        modified[0]['artifacts'] = [artifact for artifact in modified[0]['artifacts']
                                    if not artifact['name'].startswith('pnm-decode_')]
        with self.assertRaisesRegex(ValueError, 'matching viewer and CLI DEBs'):
            self.render(manifests=modified, ubuntu_only=True)

    def test_ubuntu_cli_verifies_downloaded_bytes_before_rendering(self):
        """Exercise the real CLI with synthetic hashes, then reject tampered bytes."""
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            version = release_version()
            manifest = manifests(version)[0]
            checksums = []
            for artifact in manifest['artifacts']:
                payload = b'synthetic package for verification tests'
                (folder / artifact['name']).write_bytes(payload)
                artifact['bytes'] = len(payload)
                checksums.append(hashlib.sha256(payload).hexdigest() + '  ' + artifact['name'])
            (folder / 'release-manifest.json').write_text(json.dumps(manifest))
            (folder / 'SHA256SUMS').write_text('\n'.join(checksums) + '\n')
            output = folder / 'page.md'
            command = [sys.executable, str(ROOT / 'scripts/release/notes.py'), '--ubuntu-only',
                       '--assets', str(folder), '--tag', 'v' + version, '--output', str(output)]
            result = subprocess.run(command, cwd=folder, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('Ubuntu 22.04 development build', output.read_text())
            (folder / manifest['artifacts'][0]['name']).write_bytes(b'tampered')
            output.unlink()
            result = subprocess.run(command, cwd=folder, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertFalse(output.exists())

    def test_cli_is_cwd_independent_and_preserves_existing_output(self):
        """Run the real renderer without network access or release publication."""
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)
            version = release_version()
            (folder / 'release-index.json').write_text(json.dumps(manifests(version)), encoding='utf-8')
            output = folder / 'page.md'
            command = [sys.executable, str(ROOT / 'scripts/release/notes.py'), '--assets', str(folder),
                       '--tag', 'v' + version, '--output', str(output)]
            result = subprocess.run(command, cwd=folder, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            before = output.read_bytes()
            self.assertIn(('PNM Viewer Toolkit v' + version).encode(), before)
            result = subprocess.run(command, cwd=folder, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(output.read_bytes(), before)


if __name__ == '__main__':
    unittest.main()
