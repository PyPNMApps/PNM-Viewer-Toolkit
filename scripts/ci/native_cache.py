#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Carry validated native build outputs between successful tag-only workflows."""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import platform
import shutil
import subprocess
import tarfile
import tempfile

ROOT = Path(__file__).resolve().parents[2]
TREES = ('target', 'apps/qt/target', 'local/ccache')
MAX_BYTES = 6_000_000_000
MAX_ARCHIVE_BYTES = 1_500_000_000
MAX_ENTRIES = 200_000
ARTIFACT_NAME = 'ubuntu-22.04-native-build-cache'


def run(*arguments):
    """Run discrete trusted executable arguments; propagate command failures."""
    return subprocess.run(list(map(str, arguments)), check=True, capture_output=True, text=True).stdout


def identity():
    """Fingerprint the pinned compiler/Qt setup and runner ABI, not app versions."""
    fields = ['native-cache-v1', platform.system(), platform.machine(),
              os.environ.get('ImageOS', ''), os.environ.get('ImageVersion', ''), run('rustc', '-Vv'),
              run('c++', '--version'), (ROOT / 'scripts/qt-sdk.py').read_text()]
    return hashlib.sha256('\n'.join(fields).encode()).hexdigest()


def allowed(name):
    """Require a relative normalized member confined to the three cache trees."""
    path = PurePosixPath(name)
    return (not path.is_absolute() and '..' not in path.parts and
            any(path == PurePosixPath(tree) or PurePosixPath(tree) in path.parents for tree in TREES))


def workspace_artifacts():
    """Find workspace crate/target names so test/app outputs are always rebuilt."""
    names = set()
    for manifest in ('Cargo.toml', 'apps/qt/Cargo.toml'):
        metadata = json.loads(run('cargo', 'metadata', '--manifest-path', ROOT / manifest,
                                  '--format-version', '1', '--no-deps', '--locked', '--offline'))
        for package in metadata['packages']:
            names.add(package['name'])
            names.update(target['name'] for target in package['targets'])
    return names


def reusable(path, names):
    """Keep dependency outputs, excluding incremental state and workspace targets."""
    parts = PurePosixPath(path).parts
    if path.startswith('local/ccache'):
        return True
    prefix = 3 if path.startswith('apps/qt/target') else 1
    remaining = parts[prefix:]
    if not remaining:
        return True
    if remaining[0] == 'tmp' or 'incremental' in remaining:
        return False
    if remaining[0] not in ('debug', 'release') or len(remaining) < 2:
        return True
    if remaining[1] == 'examples':
        return False
    candidate = remaining[2] if remaining[1] in ('deps', 'build', '.fingerprint') and len(remaining) > 2 else remaining[1]
    for name in names:
        for spelling in (name, name.replace('-', '_')):
            for stem in (spelling, 'lib' + spelling):
                if candidate == stem or candidate.startswith(stem + '-') or candidate.startswith(stem + '.'):
                    return False
    return True


def pack(output, workspace_names=()):
    """Archive build outputs with executable modes and normalized internal links.

    Reject outside links, devices, excessive expanded size or entry count. Never
    include Cargo credentials, checkout source, captures or package outputs.
    """
    output.mkdir(parents=True, exist_ok=False)
    total = count = 0
    with tarfile.open(output / 'native.tar.gz', 'w:gz', compresslevel=1) as archive:
        for tree in TREES:
            directory = ROOT / tree
            if not directory.is_dir():
                continue
            for source in [directory, *directory.rglob('*')]:
                relative = source.relative_to(ROOT).as_posix()
                if not reusable(relative, workspace_names):
                    continue
                count += 1
                if count > MAX_ENTRIES:
                    raise ValueError('Native cache has too many entries')
                info = archive.gettarinfo(str(source), arcname=relative)
                info.mode &= 0o777
                if info.issym():
                    target = source.resolve().relative_to(ROOT).as_posix()
                    if not allowed(target):
                        raise ValueError('Native cache link leaves build outputs')
                    info.linkname = os.path.relpath(ROOT / target, source.parent)
                    archive.addfile(info)
                elif info.islnk():
                    if not allowed(info.linkname):
                        raise ValueError('Native cache hard link leaves build outputs')
                    archive.addfile(info)
                elif info.isfile():
                    total += info.size
                    if total > MAX_BYTES:
                        raise ValueError('Native cache exceeds expanded size limit')
                    with source.open('rb') as stream:
                        archive.addfile(info, stream)
                elif info.isdir():
                    archive.addfile(info)
                else:
                    raise ValueError('Native cache contains a special file')
    if (output / 'native.tar.gz').stat().st_size > MAX_ARCHIVE_BYTES:
        raise ValueError('Native cache exceeds archive size limit')
    (output / 'identity.json').write_text(json.dumps({'fingerprint': identity()}))
    print(f'Native cache: {count} entries, {total} expanded bytes')


def restore(directory):
    """Validate a downloaded artifact, stage safely, then replace cache trees.

    An incompatible toolchain/ABI is a normal cold-build fallback. Malformed or
    unsafe archives fail before existing build outputs are touched. Restore links
    only after regular files; restrict all links to selected build-output trees.
    """
    if json.loads((directory / 'identity.json').read_text()) != {'fingerprint': identity()}:
        print('Native cache incompatible; rebuilding normally')
        return
    source = directory / 'native.tar.gz'
    if source.stat().st_size > MAX_ARCHIVE_BYTES:
        raise ValueError('Native cache exceeds archive size limit')
    with tempfile.TemporaryDirectory(prefix='pnm-native-restore-', dir=ROOT / 'local') as temporary:
        staging = Path(temporary)
        links = []
        total = count = 0
        with tarfile.open(source, 'r:gz') as archive:
            for member in archive:
                count += 1
                if count > MAX_ENTRIES or not allowed(member.name):
                    raise ValueError('Unsafe native cache member')
                destination = staging / member.name
                if destination.exists() or destination.is_symlink():
                    raise ValueError('Duplicate native cache member')
                if member.isdir():
                    destination.mkdir(parents=True, exist_ok=True)
                elif member.isfile():
                    total += member.size
                    if total > MAX_BYTES:
                        raise ValueError('Native cache exceeds expanded size limit')
                    destination.parent.mkdir(parents=True, exist_ok=True)
                    with archive.extractfile(member) as stream, destination.open('xb') as output:
                        shutil.copyfileobj(stream, output)
                    destination.chmod(member.mode & 0o777)
                    os.utime(destination, (member.mtime, member.mtime))
                elif member.issym() or member.islnk():
                    target = (destination.parent / member.linkname) if member.issym() else staging / member.linkname
                    try:
                        relative = target.resolve().relative_to(staging).as_posix()
                    except ValueError as error:
                        raise ValueError('Unsafe native cache link') from error
                    if not allowed(relative):
                        raise ValueError('Native cache link leaves build outputs')
                    links.append((member, destination, staging / relative))
                else:
                    raise ValueError('Special native cache file')
        for member, destination, target in links:
            destination.parent.mkdir(parents=True, exist_ok=True)
            if destination.exists() or destination.is_symlink():
                raise ValueError('Native cache link overlaps another member')
            if member.issym():
                destination.symlink_to(os.path.relpath(target, destination.parent))
            else:
                if not target.is_file() or target.is_symlink():
                    raise ValueError('Invalid native cache hard link')
                os.link(target, destination)
        for tree in TREES:
            candidate = staging / tree
            if candidate.is_dir():
                destination = ROOT / tree
                if destination.is_symlink():
                    raise ValueError('Refusing to replace a symlink cache root')
                if destination.exists():
                    shutil.rmtree(destination)
                destination.parent.mkdir(parents=True, exist_ok=True)
                candidate.rename(destination)
    print(f'Restored native build outputs: {count} entries; all checks still run')


def select_run(repository, output):
    """Select only successful exact-workflow push artifacts from ancestor commits."""
    runs = []
    for workflow in ('release.yml', 'ubuntu-deb-temp.yml'):
        endpoint = f'repos/{repository}/actions/workflows/{workflow}/runs?event=push&status=success&per_page=30'
        runs.extend(json.loads(run('gh', 'api', endpoint))['workflow_runs'])
    artifact_name = ARTIFACT_NAME + '-' + identity()[:20]
    selected = None
    for candidate in runs:
        tag = candidate.get('head_branch', '')
        parts = tag.removeprefix('v').split('.')
        if not tag.startswith('v') or len(parts) != 3 or not all(part.isascii() and part.isdigit() for part in parts):
            continue
        commit = candidate['head_sha']
        if len(commit) != 40 or any(character not in '0123456789abcdef' for character in commit):
            raise ValueError('Invalid cache source commit')
        ancestor = subprocess.run(['git', 'merge-base', '--is-ancestor', commit, 'HEAD'], cwd=ROOT, capture_output=True)
        if ancestor.returncode:
            continue
        artifacts = json.loads(run('gh', 'api', f'repos/{repository}/actions/runs/{candidate["id"]}/artifacts'))['artifacts']
        if any(artifact['name'] == artifact_name and not artifact['expired'] and 0 < artifact['size_in_bytes'] <= MAX_ARCHIVE_BYTES + 20_000 for artifact in artifacts):
            selected = candidate['id']
            break
    with output.open('a') as stream:
        stream.write(f'run_id={selected or ""}\nartifact_name={artifact_name}\n')
    print(f'Native cache source run: {selected or "none; first compatible run seeds cache"}')


def main():
    """Dispatch artifact selection/export/restore without installing or publishing."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=('select', 'pack', 'restore'))
    parser.add_argument('--directory', type=Path, default=ROOT / 'local/native-cache-artifact')
    args = parser.parse_args()
    if args.action == 'select':
        select_run(os.environ['GITHUB_REPOSITORY'], Path(os.environ['GITHUB_OUTPUT']))
    elif args.action == 'pack':
        ready = False
        try:
            pack(args.directory, workspace_artifacts())
            ready = True
        except (OSError, ValueError, tarfile.TarError) as error:
            # Cache export is optional; required build/test gates already passed.
            # Never upload partial/oversized output or fail a valid package for it.
            print(f'::warning::Native cache export unavailable: {error}')
        if os.environ.get('GITHUB_OUTPUT'):
            with Path(os.environ['GITHUB_OUTPUT']).open('a') as output:
                output.write(f'ready={str(ready).lower()}\n')
    else:
        restore(args.directory)


if __name__ == '__main__':
    main()
