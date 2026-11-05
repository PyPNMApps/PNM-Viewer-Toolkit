#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Bundle the explicitly approved, sanitized PyPNM example captures."""
import hashlib
from pathlib import Path
import shutil

from common import ROOT

CAPTURE_FILES = ('rxmer.bin', 'spectrum_analyzer.bin', 'channel_estimation.bin',
                 'histogram.bin', 'fec_summary.bin', 'modulation_profile.bin',
                 'const_display.bin', 'us_pre_equalizer_coef.bin',
                 'us_pre_equalizer_coef_last.bin')


def bundle(destination):
    """Copy only approved captures and their licensing/provenance to a new folder.

    destination belongs to a fresh viewer payload, not the standalone CLI.
    Refuse existing output and preserve fixture bytes. Filesystem errors propagate.
    """
    source = ROOT / 'tests/fixtures/pypnm'
    destination.mkdir(parents=True, exist_ok=False)
    for name in CAPTURE_FILES:
        shutil.copy2(source / 'tests/files' / name, destination / name)
    for name in ('LICENSE', 'NOTICE', 'provenance.json'):
        shutil.copy2(source / name, destination / name)
    (destination / 'README.md').write_text(
        '# Example PNM captures\n\n'
        'Sanitized PyPNM fixtures for offline exploration. CM/CMTS MAC addresses '
        'are aa:bb:cc:dd:ee:ff. Original capture paths and hashes are recorded in '
        'provenance.json; LICENSE and NOTICE retain upstream terms.\n\n'
        'Treat these files as read-only installation resources. Export JSON to your '
        'own writable folder. No modem or network connection is needed.\n\n'
        + ''.join(f'- {name}\n' for name in CAPTURE_FILES), encoding='utf-8')


def installed_directory(gui):
    """Find the examples folder for a deployed Windows/Linux executable or Mac app.

    Raise ValueError when packaged captures are absent or differ from the approved
    fixtures. This is a CI contract check, not general user capture validation.
    """
    candidates = (gui.parent / 'share/examples',
                  gui.parent.parent / 'Resources/share/examples')
    for directory in candidates:
        if directory.is_dir():
            for name in CAPTURE_FILES:
                expected = ROOT / 'tests/fixtures/pypnm/tests/files' / name
                actual = directory / name
                if not actual.is_file() or hashlib.sha256(actual.read_bytes()).digest() != hashlib.sha256(expected.read_bytes()).digest():
                    raise ValueError(f'Bundled example missing or modified: {name}')
            return directory
    raise ValueError('Installed examples folder missing')
