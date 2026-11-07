#!/usr/bin/env python3
#
# Copyright (c) 2025 Maurice Garcia
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Sanitize fixture MAC fields; adapted from PyPNM's pnm-mac-updater.py.

See README.md for upstream provenance and the binary-safety changes. This is
standalone development tooling, never a production Python dependency.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path
import subprocess
import sys
import tempfile
import tarfile
import zipfile

# Direct invocation from any working directory and unittest discovery share this module.
sys.path.insert(0, str(Path(__file__).resolve().parent))
from dataset_mac import Dataset, is_alias

ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / 'tests/fixtures/pypnm'
DEFAULT_MAC = '00ca.0000.0001'
MAC_BYTES = 6
# Format 1.0 layout evidence: pnm_file_type.rs selects 10-byte common headers
# except FEC (6 bytes); each cm_* parser reads channel before MAC except hist.
# Pre-EQ reads both CM and CMTS. These are field locations, never byte searches.
MAC_FIELDS = {
    b'PNN\x02': (('mac_address', 11),),
    b'PNN\x03': (('mac_address', 11),),
    b'PNN\x04': (('mac_address', 11),),
    b'PNN\x05': (('mac_address', 10),),
    b'PNN\x06': (('mac_address', 11), ('cmts_mac_address', 17)),
    b'PNN\x07': (('mac_address', 11), ('cmts_mac_address', 17)),
    b'PNN\x08': (('mac_address', 7),),
    b'PNN\x09': (('mac_address', 11),),
    b'PNN\x0a': (('mac_address', 11),),
    b'LLD\x01': (('mac_address', 6),),
}


def normalize_mac(value):
    """Accept dotted, colon, hyphen or compact hex and return exactly six bytes.

    Reject malformed separator groups rather than removing arbitrary characters.
    This explicit fixture alias may be multicast; do not alter the user's value.
    """
    groups = value.split('.') if '.' in value else value.replace('-', ':').split(':')
    widths = [len(group) for group in groups]
    if widths not in ([4, 4, 4], [2] * 6, [12]):
        raise ValueError('Expected a six-byte MAC in dotted, octet or compact form')
    compact = ''.join(groups)
    if any(character not in '0123456789abcdefABCDEF' for character in compact):
        raise ValueError('MAC must contain hexadecimal digits only')
    return bytes.fromhex(compact)


def field_ranges(data):
    """Return evidenced MAC ranges; fail closed for unknown/truncated layouts."""
    fields = MAC_FIELDS.get(data[:4])
    if fields is None or data[4:6] != bytes((1, 0)):
        raise ValueError('Unsupported PNM identity or format version')
    if any(offset + MAC_BYTES > len(data) for _, offset in fields):
        raise ValueError('Truncated PNM MAC field')
    return fields


def rewrite(data, address):
    """Return a derived capture changing only declared MAC fields, preserving size."""
    if len(address) != MAC_BYTES:
        raise ValueError("Replacement MAC must be exactly six bytes")
    updated = bytearray(data)
    for _, offset in field_ranges(data):
        updated[offset:offset + MAC_BYTES] = address
    return bytes(updated)


def decode(decoder, data):
    """Validate bytes through the real Rust decoder without printing identifiers."""
    with tempfile.TemporaryDirectory(prefix='pnm-sanitize-') as directory:
        capture = Path(directory) / 'capture.bin'
        capture.write_bytes(data)
        result = subprocess.run([str(decoder), str(capture), '--stdout'],
                                capture_output=True, text=True, check=False)
        if result.returncode:
            raise ValueError('Rust decoder rejected a fixture; no files written')
        return json.loads(result.stdout)


def without_identity(value):
    """Mask MAC keys for exact before/after decoded measurement comparison."""
    if isinstance(value, dict):
        return {key: without_identity(item) for key, item in value.items()
                if key not in ('mac_address', 'cmts_mac_address')}
    if isinstance(value, list):
        return [without_identity(item) for item in value]
    return value


def atomic_write(path, content):
    """Replace one reviewed file through a same-directory temporary file."""
    temporary = None
    try:
        with tempfile.NamedTemporaryFile(dir=path.parent, delete=False) as stream:
            temporary = Path(stream.name)
            stream.write(content)
        temporary.chmod(path.stat().st_mode & 0o777)
        temporary.replace(path)
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def process(address, write=False, dry_run=False, decoder=None):
    """Check or explicitly sanitize the repository's complete imported fixture set.

    Validate every binary/manifest before writes. In write mode the Rust decoder
    proves all non-MAC output unchanged. Per-file replacement is atomic; a batch
    I/O failure must be reviewed and the check rerun before staging.
    """
    manifest_path = FIXTURES / 'provenance.json'
    if any(path.is_symlink() for path in (FIXTURES, FIXTURES / 'tests', FIXTURES / 'tests/files', manifest_path)):
        raise ValueError('Fixture roots and provenance must not be symlinks')
    manifest = json.loads(manifest_path.read_text())
    files = sorted((FIXTURES / 'tests/files').glob('*.bin'))
    entries = {entry['path']: entry for entry in manifest['files']}
    planned = []
    for path in files:
        if path.is_symlink() or not path.is_file():
            raise ValueError('Fixture must be a regular non-symlink file')
        relative = path.relative_to(FIXTURES).as_posix()
        if relative not in entries:
            raise ValueError(f'Missing fixture provenance: {relative}')
        if path.stat().st_size > 16 * 1024 * 1024:
            raise ValueError(f'Fixture exceeds the 16 MiB limit: {relative}')
        original = path.read_bytes()
        digest = hashlib.sha256(original).hexdigest()
        if digest != entries[relative]['sha256']:
            raise ValueError(f'Fixture checksum mismatch: {relative}')
        updated = rewrite(original, address)
        if write:
            if without_identity(decode(decoder, original)) != without_identity(decode(decoder, updated)):
                raise ValueError(f'Measurement comparison failed: {relative}')
        planned.append((path, original, updated))
    expected = {entry['path'] for entry in manifest['files'] if entry['path'].endswith('.bin')}
    if expected != {path.relative_to(FIXTURES).as_posix() for path in files}:
        raise ValueError('Missing or unexpected fixture binaries')
    changed = [item for item in planned if item[1] != item[2]]
    if write:
        for path, original, updated in planned:
            entry = entries[path.relative_to(FIXTURES).as_posix()]
            entry.setdefault('original_sha256', entry['sha256'])
            entry['sha256'] = hashlib.sha256(updated).hexdigest()
            entry['modified'] = True
            entry['sanitized_fields'] = [name for name, _ in field_ranges(original)]
            if original != updated:
                atomic_write(path, updated)
        manifest['modified'] = True
        manifest['sanitization'] = {'mac_address': address.hex(':'),
            'tool': 'tools/security/pnm-mac-updater.py',
            'scope': 'All CM and CMTS MAC fields; measurement bytes unchanged'}
        atomic_write(manifest_path, (json.dumps(manifest, indent=2) + '\n').encode())
    elif changed and not dry_run:
        raise ValueError(f'{len(changed)} fixture(s) need MAC sanitization; run this tool with --write')
    print(f'Fixtures: {len(files)}; MAC fields: {sum(len(field_ranges(data)) for _, data, _ in planned)}; changes: {len(changed)}')


def check_examples(address):
    """Check every MAC key in linked JSON and the synthetic latency source."""
    def visit(value):
        if isinstance(value, dict):
            for key, item in value.items():
                if key in ('mac_address', 'cmts_mac_address'):
                    if not isinstance(item, str) or normalize_mac(item) != address:
                        raise ValueError('JSON example has a non-approved MAC field; regenerate examples')
                else:
                    visit(item)
        elif isinstance(value, list):
            for item in value:
                visit(item)
    for path in (ROOT / 'docs/cli/examples').glob('*.json'):
        visit(json.loads(path.read_text()))
    for path in (ROOT / 'docs/cli/examples').glob('*.hex'):
        data = bytes.fromhex(path.read_text())
        if data[:4] == b'PNN\x01':
            continue  # Header-only Symbol stub has no evidenced MAC layout.
        if rewrite(data, address) != data:
            raise ValueError('Synthetic source needs the approved MAC alias')



def validate_rewrite(decoder, original, updated):
    """Require identical decoded non-MAC measurements before replacing a capture."""
    if without_identity(decode(decoder, original)) != without_identity(decode(decoder, updated)):
        raise ValueError('Demo measurement comparison failed; no files written')


def process_directory(directory, write=False, dry_run=False, decoder=None):
    """Sanitize a demo collection with stable aliases, archives and metadata.

    Repeated CM identities retain one alias across channels and timestamps.
    CMTS fields share the same mapping when identities overlap. Plan and validate the
    entire collection before writing; never print original addresses.
    """
    validator = None
    if write:
        validator = lambda original, updated: validate_rewrite(decoder, original, updated)
    dataset = Dataset(directory, field_ranges, validator)
    plan = dataset.plan()
    changes = [(source, destination, original, updated)
               for source, destination, original, updated in plan
               if source != destination or original != updated]
    if changes and not write and not dry_run:
        raise ValueError('Demo collection needs MAC sanitization; run with --directory PATH --write')
    if write:
        for source, destination, _, updated in changes:
            destination.parent.mkdir(parents=True, exist_ok=True)
            if source == destination:
                atomic_write(source, updated)
            else:
                # The plan has checked collisions; retain permissions for renamed files.
                atomic_write(source, updated)
                source.replace(destination)
        for path in sorted(Path(directory).rglob('*'), reverse=True):
            if path.is_dir() and not any(path.iterdir()):
                path.rmdir()
    print(f'Demo captures: {dataset.capture_count}; modems: '
          f'{len(dataset.identities["mac_address"])}; changed files: {len(changes)}')


def update_examples(address):
    """Update structured JSON identity keys and evidenced synthetic hex sources."""
    # Match only JSON identity string values (for example "mac_address": "aa:bb:cc:dd:ee:ff").
    # Group 1 retains the key/spacing; whole documents keep their numeric spelling.
    identity_field = re.compile(r'("(?:mac_address|cmts_mac_address)"\s*:\s*)"[^"]*"')
    for path in (ROOT / 'docs/cli/examples').glob('*.json'):
        original = path.read_text()
        json.loads(original)  # Reject malformed source before replacing anything.
        updated = identity_field.sub(lambda match: match[1] + '"' + address.hex(':') + '"', original)
        atomic_write(path, updated.encode())
    for path in (ROOT / 'docs/cli/examples').glob('*.hex'):
        data = bytes.fromhex(path.read_text())
        if data[:4] != b'PNN\x01':
            atomic_write(path, (rewrite(data, address).hex() + '\n').encode())


def check_tracked_captures():
    """Check tracked and new nonignored capture assets, including archive members.

    Inventory comes from Git, so adding a new demo folder cannot bypass the
    gate. Ignored private working datasets are inspected only on explicit request.
    """
    result = subprocess.run(['git', '-C', str(ROOT), 'ls-files', '--cached',
                             '--others', '--exclude-standard', '-z'],
                            capture_output=True, check=True)
    dataset = Dataset(ROOT, field_ranges)
    budget = [0, 0]
    for name in sorted(set(result.stdout.decode().split('\0')) - {''}):
        path = ROOT / name
        candidate = name.lower().endswith(('.bin', '.zip', '.tar', '.tar.gz', '.tgz', '.bin.gz', '.zst', '.zts'))
        if not candidate and path.is_file() and not path.is_symlink():
            with path.open('rb') as stream:
                prefix = stream.read(4)
            candidate = (len(prefix) == 4 and prefix[:3] in (b'PNN', b'LLD', b'PNM')
                         and prefix[3] < 0x20) or prefix.startswith((b'PK\x03\x04', b'\x1f\x8b'))
        if not candidate:
            continue
        if path.is_symlink():
            raise ValueError('Tracked capture assets must not be symlinks')
        if path.stat().st_size > 256 * 1024 * 1024:
            raise ValueError('Tracked capture asset exceeds size limit')
        dataset.walk(name, path.read_bytes(), dataset.collect, budget)
    if any(not is_alias(address) for values in dataset.identities.values() for address in values):
        raise ValueError('Tracked demo MAC outside approved 00ca.00 namespace; sanitize before push')


def main():
    """Check by default; dry-run never writes and --write explicitly sanitizes."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mac-address', default=DEFAULT_MAC)
    parser.add_argument('--directory', type=Path, help='Demo directory; preserve distinct CM identities with sequential aliases')
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument('--write', action='store_true')
    modes.add_argument('--dry-run', action='store_true')
    parser.add_argument('--decoder', type=Path, default=ROOT / 'target/release/pnm-decode')
    args = parser.parse_args()
    try:
        address = normalize_mac(args.mac_address)
        if not is_alias(address):
            raise ValueError('Use the approved 00ca.00 MAC namespace with a nonzero suffix')
        if args.directory is not None:
            process_directory(args.directory, args.write, args.dry_run, args.decoder.resolve())
        else:
            process(address, args.write, args.dry_run, args.decoder.resolve())
            if args.write:
                update_examples(address)
                demos = ROOT / 'tests/fixtures/demos'
                if demos.exists():
                    for directory in sorted(demos.iterdir()):
                        if directory.is_dir():
                            process_directory(directory, write=True, decoder=args.decoder.resolve())
            if not args.dry_run:
                check_examples(address)
                check_tracked_captures()
    except (ValueError, OSError, KeyError, subprocess.SubprocessError,
            tarfile.TarError, zipfile.BadZipFile, EOFError) as error:
        print(f'ERROR: {error}', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main())
