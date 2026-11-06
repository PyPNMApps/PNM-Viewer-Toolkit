#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Clean project Rust outputs manually or before builds at 95% disk usage."""
import argparse
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
DISK_USAGE_THRESHOLD_PERCENT = 95
CACHE_SIGNATURE = 'Signature: 8a477f597d28d172789f06886806bc55'
BUILD_TARGETS = (('Cargo.toml', 'target'), ('apps/qt/Cargo.toml', 'apps/qt/target'))


def clean(root=ROOT, dry_run=False):
    """Call Cargo clean for validated checkout targets, or preview that plan.

    root is a checkout; dry_run leaves outputs unchanged. Reject tracked files,
    symlink redirects and unmarked directories before cleaning anything. Cargo
    owns its build locks. Preserve SDKs, downloads, toolchains and other repos.
    Git, Cargo and validation failures propagate to stop the build.
    """
    root = root.resolve()
    if subprocess.check_output(['git', '-C', str(root), 'ls-files', '-z', '--',
                                *(target for _, target in BUILD_TARGETS)]):
        raise ValueError('Build outputs contain tracked files; cleanup refused.')
    candidates = []
    for manifest, relative in BUILD_TARGETS:
        target = root / relative
        if not target.exists() and not target.is_symlink():
            continue
        marker = target / 'CACHEDIR.TAG'
        if (target.is_symlink() or target.resolve() != target or not target.is_dir()
                or marker.is_symlink() or not marker.is_file()
                or not marker.read_text().startswith(CACHE_SIGNATURE)
                or not (root / manifest).is_file()):
            raise ValueError(f'Refusing to clean unmarked or redirected Cargo output: {target}')
        candidates.append((root / manifest, target))
    local_cargo = root / 'scripts/cargo-local.sh'
    cargo = str(local_cargo) if local_cargo.is_file() and (root / 'local/cargo/bin/cargo').is_file() else 'cargo'
    for manifest, target in candidates:
        if dry_run:
            print(f'Would clean: {target}')
        else:
            subprocess.run([cargo, 'clean', '--manifest-path', str(manifest),
                            '--target-dir', str(target)], cwd=root, check=True)


def ensure_space(root=ROOT, dry_run=False):
    """At 95% filesystem usage, invoke the shared cleaner and recheck space.

    root identifies this checkout. A dry run previews without cleaning. Stop
    before compilation if real cleanup cannot lower disk usage below 95%.
    """
    root = root.resolve()
    usage = shutil.disk_usage(root)
    if usage.used * 100 < (usage.used + usage.free) * DISK_USAGE_THRESHOLD_PERCENT:
        return
    print('Filesystem usage is at least 95%; calling cargo clean for project build caches.', flush=True)
    clean(root, dry_run)
    if dry_run:
        return
    usage = shutil.disk_usage(root)
    if usage.used * 100 >= (usage.used + usage.free) * DISK_USAGE_THRESHOLD_PERCENT:
        raise ValueError('Filesystem remains at least 95% full after project Cargo cleanup; free space before building.')


def main():
    """Run a manual clean, preview or threshold check for this checkout.

    CLI flags select policy only, never arbitrary deletion paths. Validation,
    Git and Cargo failures propagate as nonzero exit status before a build.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--if-disk-full', action='store_true', help='clean only at 95% filesystem usage or higher')
    parser.add_argument('--dry-run', action='store_true', help='preview without deleting build outputs')
    args = parser.parse_args()
    if args.if_disk_full:
        ensure_space(dry_run=args.dry_run)
    else:
        clean(dry_run=args.dry_run)


if __name__ == '__main__':
    main()
