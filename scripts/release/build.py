#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build native PNM Viewer Toolkit packages without installing or publishing."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess

from common import (ROOT, binary_path, inventory, native_target, notices, qt,
                    release_version, run, verify_artifacts)


def build(output):
    """Build native CLI archive and Qt installer into a fresh output directory.

    Reject unsupported hosts, stale versions and existing output first. Build
    dependencies must already be installed. Preserve all output on failure for
    diagnosis; do not install, upload, tag or publish. Enforce package budgets.
    """
    system, architecture = native_target()
    version = release_version()
    output = output.resolve()
    if output.exists():
        raise ValueError(f'Output exists; move it aside before retrying: {output}')
    environment = qt.qt_environment()
    # Match Qt 6.8's supported macOS baseline; do not mislabel older compatibility.
    if system == 'Darwin':
        environment['MACOSX_DEPLOYMENT_TARGET'] = '13.0'
    run(['cargo', 'build', '--release', '--locked', '-p', 'pnm-decode'], env=environment)
    run(['cargo', 'build', '--release', '--locked', '--manifest-path', 'apps/qt/Cargo.toml'], env=environment)
    output.mkdir(parents=True)
    stage = output.parent / (output.name + '-stage')
    stage.mkdir(exist_ok=False)
    import common
    import licenses
    common.DEPENDENCY_NOTICES = stage / 'dependency-notices'
    licenses.collect(common.DEPENDENCY_NOTICES)
    shutil.copy2(common.DEPENDENCY_NOTICES / 'bom.cdx.json', output / 'bom.cdx.json')
    cli = stage / f'pnm-decode-{version}-{system.lower()}-{architecture}'
    cli.mkdir()
    shutil.copy2(binary_path(), cli / binary_path().name)
    notices(cli / 'share')
    inventory(cli, output / 'cli-installed-files.json')
    archive = shutil.make_archive(str(output / cli.name), 'zip' if system == 'Windows' else 'gztar',
                                  root_dir=cli.parent, base_dir=cli.name)
    print(f'CLI archive: {archive}', flush=True)
    if system == 'Linux':
        import linux
        linux.build(stage, output, version)
    else:
        import native
        if system == 'Windows':
            native.windows(stage, output, version)
        else:
            native.macos(stage, output, version, architecture)
    # Record exact locked project dependencies alongside deployed-file inventories.
    for label, manifest in (('core', 'Cargo.toml'), ('qt', 'apps/qt/Cargo.toml')):
        data = run(['cargo', 'metadata', '--locked', '--format-version', '1',
                    '--manifest-path', ROOT / manifest], capture_output=True, text=True, env=environment)
        (output / f'{label}-dependency-graph.json').write_text(data.stdout, encoding='utf-8')
    artifacts = verify_artifacts(output)
    (output / 'release-manifest.json').write_text(json.dumps({
        'version': version, 'system': system, 'architecture': architecture,
        'artifacts': [{'name': path.name, 'bytes': path.stat().st_size} for path in artifacts],
        'pristine_host_offline_install': False,
        'external_prerequisites': 'See docs/release/packaging.md; Linux uses declared system libraries.'
    }, indent=2) + '\n', encoding='utf-8')


def main():
    """Build native artifacts at --output; report failure without deleting output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, default=ROOT / 'dist/release')
    args = parser.parse_args()
    try:
        build(args.output)
    except (ValueError, OSError, subprocess.CalledProcessError) as error:
        parser.exit(1, f'ERROR: {error}\n')


if __name__ == '__main__':
    main()
