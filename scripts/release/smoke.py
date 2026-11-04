#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check deployed CLI and Qt exports without using the development Qt SDK."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ('rxmer.bin', 'spectrum_analyzer.bin', 'channel_estimation.bin', 'histogram.bin', 'fec_summary.bin', 'modulation_profile.bin', 'const_display.bin', 'us_pre_equalizer_coef.bin', 'us_pre_equalizer_coef_last.bin')


def clean_environment():
    """Remove development Qt paths while preserving the host display/session.

    Return a child-only environment. Packaged Qt must resolve its own libraries
    and QML/plugins; this intentionally does not mask absent system prerequisites.
    """
    environment = os.environ.copy()
    for name in ('QT_SDK_DIR', 'QMAKE', 'QT_PLUGIN_PATH', 'QML_IMPORT_PATH',
                 'QML2_IMPORT_PATH', 'LD_LIBRARY_PATH', 'DYLD_LIBRARY_PATH',
                 'DYLD_FRAMEWORK_PATH', 'QT_QPA_PLATFORM_PLUGIN_PATH'):
        environment.pop(name, None)
    environment['PATH'] = os.pathsep.join(
        entry for entry in environment['PATH'].split(os.pathsep)
        if 'local/qt' not in entry.replace('\\', '/').lower())
    environment.update(QT_QUICK_BACKEND='software', QT_QUICK_CONTROLS_STYLE='Fusion')
    return environment


def smoke(gui, cli):
    """Open/render/export every supported fixture and compare raw CLI fields.

    gui/cli are installed executable paths. Use new temporary destinations,
    require a successful rendered-frame marker, and reject missing/mismatched
    exports. Each native GUI subprocess has a timeout and SDK-free environment.
    """
    environment = clean_environment()
    subprocess.run([str(cli), '--version'], check=True, env=environment, timeout=15)
    with tempfile.TemporaryDirectory(prefix='pnm-package-smoke-') as directory:
        output = Path(directory)
        for filename in FIXTURES:
            capture = ROOT / 'tests/fixtures/pypnm/tests/files' / filename
            raw = output / (filename + '.cli.json')
            decoded = output / (filename + '.ui.json')
            subprocess.run([str(cli), str(capture), '--output', str(raw)],
                           env=environment, check=True, timeout=30)
            try:
                result = subprocess.run([str(gui), '--smoke-test', capture.as_uri(), decoded.as_uri()],
                                        env=environment, capture_output=True, text=True, timeout=50)
            except subprocess.TimeoutExpired as error:
                print(error.stdout, error.stderr, flush=True)
                raise
            print(result.stdout, result.stderr, flush=True)
            if result.returncode != 0 or 'UI_SMOKE_OK' not in result.stdout + result.stderr:
                raise RuntimeError(f'Installed GUI smoke failed: {filename}')
            expected = json.loads(raw.read_text(encoding='utf-8'))
            actual = json.loads(decoded.read_text(encoding='utf-8'))
            for field, value in expected.items():
                if actual.get(field) != value:
                    raise RuntimeError(f'CLI/UI export mismatch: {filename}/{field}')
            if not actual:
                raise RuntimeError(f'Empty export: {filename}')
            print(f'PASS: installed open/render/export {filename}', flush=True)


def main():
    """Run installed-binary checks; failures return nonzero with diagnostics."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--gui', required=True, type=Path)
    parser.add_argument('--cli', required=True, type=Path)
    args = parser.parse_args()
    smoke(args.gui.resolve(), args.cli.resolve())


if __name__ == '__main__':
    main()
