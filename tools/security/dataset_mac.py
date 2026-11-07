#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Bounded demo-dataset MAC rewriting shared by the standalone hygiene tool."""
import copy
import gzip
import hashlib
import io
from pathlib import Path, PurePosixPath
import re
import tarfile
import zipfile

ALIAS_PREFIX = bytes.fromhex('00ca00')
MAX_FILE_BYTES = 256 * 1024 * 1024
MAX_EXPANDED_BYTES = 512 * 1024 * 1024
MAX_ENTRIES = 20_000
MAX_DEPTH = 4
# Match complete MAC tokens in dotted, octet-separated or compact form, including
# filenames such as capture_aabbccddeeff_123.bin. Groups are not consumed; case
# is ignored. Hex boundaries exclude partial matches inside hashes/longer IDs.
MAC_TOKEN = re.compile(
    r'(?<![0-9a-f])(?:[0-9a-f]{2}(?::[0-9a-f]{2}){5}|'
    r'[0-9a-f]{2}(?:-[0-9a-f]{2}){5}|[0-9a-f]{4}(?:\.[0-9a-f]{4}){2}|'
    r'[0-9a-f]{12})(?![0-9a-f])', re.IGNORECASE)


def is_alias(address):
    """Test the approved fixed-prefix, nonzero-counter demo namespace.

    Returns False for malformed lengths and the reserved zero counter.
    """
    return len(address) == 6 and address[:3] == ALIAS_PREFIX and int.from_bytes(address[3:], 'big') > 0


def aliases(addresses, reserved=()):
    """Preserve existing aliases; assign other identities unused sequential IDs.

    Sorting makes a fresh dataset deterministic. Existing aliases keep their
    identities when new captures are appended. No original identifier is logged.
    """
    mapping = {address: address for address in addresses if is_alias(address)}
    used = set(mapping.values()) | set(reserved)
    ordinal = 1
    for address in sorted(addresses - mapping.keys()):
        while ALIAS_PREFIX + ordinal.to_bytes(3, 'big') in used:
            ordinal += 1
        if ordinal > 0xffffff:
            raise ValueError('Demo MAC namespace exhausted')
        replacement = ALIAS_PREFIX + ordinal.to_bytes(3, 'big')
        mapping[address] = replacement
        used.add(replacement)
    return mapping


def replace_tokens(text, mapping):
    """Replace known identities in demo names/metadata, preserving MAC notation.

    Unknown tokens and longer hexadecimal identifiers remain unchanged. The
    mapping contains only header-derived identities; no replacements are guessed.
    """
    def replace(match):
        token = match.group()
        compact = token.replace(':', '').replace('-', '').replace('.', '')
        replacement = mapping.get(bytes.fromhex(compact))
        if replacement is None:
            return token
        value = replacement.hex()
        if '.' in token:
            return '.'.join(value[index:index + 4] for index in range(0, 12, 4))
        if ':' in token or '-' in token:
            separator = ':' if ':' in token else '-'
            return separator.join(value[index:index + 2] for index in range(0, 12, 2))
        return value
    return MAC_TOKEN.sub(replace, text)


class Dataset:
    """Plan a complete directory/archives rewrite before any file is replaced.

    The caller supplies evidenced PNM field ranges and optional Rust validation.
    Archives are read in memory, never extracted. Symlinks, unsafe names, unknown
    binary layouts and resource-limit breaches stop the entire planning phase.
    """

    def __init__(self, root, field_ranges, validate=None):
        """Create a planner without reading or modifying the selected directory.

        field_ranges returns evidenced MAC offsets; validate optionally compares
        original/rewritten captures before writes. No reverse map is persisted.
        """
        self.root = Path(root)
        self.field_ranges = field_ranges
        self.validate = validate
        self.identities = {'mac_address': set(), 'cmts_mac_address': set()}
        self.hashes = {}
        self.capture_count = 0
        self.validated = set()

    def walk(self, name, data, transform, budget, depth=0):
        """Visit capture bytes recursively with shared expansion and entry limits.

        transform receives evidenced capture bytes. Returns rebuilt archive or
        capture bytes; unsafe paths, unknown layouts and resource breaches raise.
        No archive member is extracted to the filesystem.
        """
        budget[0] += 1
        budget[1] += len(data)
        if depth > MAX_DEPTH or budget[0] > MAX_ENTRIES or budget[1] > MAX_EXPANDED_BYTES:
            raise ValueError('Demo archive exceeds depth, entry or expansion limits')
        lower = name.lower()
        if lower.endswith(('.zst', '.zts')):
            raise ValueError('Expand Zstandard captures before MAC sanitization')
        if lower.endswith('.zip') or data.startswith(b'PK\x03\x04'):
            output = io.BytesIO()
            with zipfile.ZipFile(io.BytesIO(data)) as source, zipfile.ZipFile(output, 'w') as target:
                names = set()
                for info in source.infolist():
                    self.safe_name(info.filename)
                    if info.external_attr >> 16 & 0o170000 == 0o120000:
                        raise ValueError('Archive symlinks are not allowed')
                    if info.file_size > MAX_FILE_BYTES or info.flag_bits & 1:
                        raise ValueError('Oversized or encrypted archive member')
                    with source.open(info) as stream:
                        member = stream.read(MAX_FILE_BYTES + 1)
                    if len(member) > MAX_FILE_BYTES:
                        raise ValueError('Oversized archive member')
                    updated = self.walk(info.filename, member, transform, budget, depth + 1)
                    new_info = copy.copy(info)
                    new_info.filename = self.rename(info.filename)
                    if new_info.filename in names:
                        raise ValueError('Duplicate archive member after MAC renaming')
                    names.add(new_info.filename)
                    target.writestr(new_info, updated)
            return output.getvalue()
        if lower.endswith(('.tar', '.tar.gz', '.tgz')) or data[257:262] == b'ustar':
            output = io.BytesIO()
            mode = 'w:gz' if lower.endswith(('.gz', '.tgz')) else 'w'
            with tarfile.open(fileobj=io.BytesIO(data), mode='r:*') as source, tarfile.open(fileobj=output, mode=mode) as target:
                names = set()
                for info in source:
                    self.safe_name(info.name)
                    if not info.isfile() and not info.isdir():
                        raise ValueError('Archive links and special files are not allowed')
                    if info.size > MAX_FILE_BYTES:
                        raise ValueError('Oversized archive member')
                    member = b''
                    if info.isfile():
                        with source.extractfile(info) as stream:
                            member = stream.read(MAX_FILE_BYTES + 1)
                    updated = self.walk(info.name, member, transform, budget, depth + 1)
                    new_info = copy.copy(info)
                    new_info.name = self.rename(info.name)
                    if new_info.name in names:
                        raise ValueError('Duplicate archive member after MAC renaming')
                    names.add(new_info.name)
                    new_info.size = len(updated)
                    # Do not carry the archive author's local login into demos.
                    new_info.uname = new_info.gname = ''
                    new_info.pax_headers = {}
                    target.addfile(new_info, io.BytesIO(updated) if info.isfile() else None)
            return output.getvalue()
        if lower.endswith('.gz') or data.startswith(b'\x1f\x8b'):
            with gzip.GzipFile(fileobj=io.BytesIO(data)) as stream:
                member = stream.read(MAX_FILE_BYTES + 1)
            if len(member) > MAX_FILE_BYTES:
                raise ValueError('Oversized gzip capture')
            updated = self.walk(name[:-3], member, transform, budget, depth + 1)
            return gzip.compress(updated, mtime=0)
        if lower.endswith('.bin') or data[:3] in (b'PNN', b'LLD', b'PNM'):
            return transform(data)
        if hasattr(self, 'text_mapping') and lower.endswith(('.json', '.md', '.txt')):
            return self.refresh_text(name, data)
        return data

    @staticmethod
    def safe_name(name):
        """Reject names which could escape a later archive extraction directory.

        Raises ValueError for absolute paths, parent traversal, drive prefixes
        and backslashes; callers never extract the accepted members here.
        """
        if '\\' in name or PurePosixPath(name).is_absolute() or '..' in PurePosixPath(name).parts or ':' in name:
            raise ValueError('Unsafe archive member name')

    def rename(self, name):
        """Translate known MAC tokens using the collection's established map.

        During inventory the map is absent and the name remains unchanged.
        """
        return replace_tokens(name, getattr(self, 'text_mapping', {}))

    def collect(self, data):
        """Inventory only declared identity fields, returning unchanged bytes.

        Counts capture occurrences, including archive copies. Unsupported or
        truncated layouts raise through the supplied field-range validator.
        """
        for key, offset in self.field_ranges(data):
            self.identities[key].add(data[offset:offset + 6])
        self.capture_count += 1
        return data

    def rewrite(self, data):
        """Change MAC fields while preserving every other capture byte.

        Records the current digest and validates each distinct changed capture
        once. Validation failure aborts planning before any file is written.
        """
        updated = bytearray(data)
        for key, offset in self.field_ranges(data):
            updated[offset:offset + 6] = self.mappings[key][data[offset:offset + 6]]
        updated = bytes(updated)
        original_hash = hashlib.sha256(data).hexdigest()
        self.hashes[original_hash] = hashlib.sha256(updated).hexdigest()
        if data != updated and self.validate is not None and original_hash not in self.validated:
            self.validate(data, updated)
            self.validated.add(original_hash)
        return updated

    def refresh_text(self, name, data):
        """Refresh current hashes while retaining original/source hash evidence.

        JSON sha256 fields are updated without reserializing numeric values.
        Plain-text manifests use exact digest replacements. Known MAC tokens
        use the same aliases as the captures.
        """
        text = self.rename(data.decode('utf-8'))
        if name.lower().endswith('.json'):
            # Match only a complete JSON "sha256" value. Group 1 preserves the
            # key/spacing; source_sha256 and original_sha256 are excluded.
            current_hash = re.compile(r'("sha256"\s*:\s*")([0-9a-f]{64})(")')
            text = current_hash.sub(lambda match: match[1] + self.hashes.get(match[2], match[2]) + match[3], text)
        else:
            for old, new in self.hashes.items():
                text = text.replace(old, new)
        return text.encode('utf-8')

    def plan(self):
        """Return path/content replacements without changing the input directory.

        Reads bounded inputs, inventories identities, validates changed captures
        and checks naming collisions. Raises on unsupported data, unsafe paths,
        exceeded limits or failed decoder comparisons before returning a plan.
        """
        if self.root.is_symlink() or not self.root.is_dir():
            raise ValueError('Demo root must be a regular directory')
        files = sorted(self.root.rglob('*'))
        if len(files) > MAX_ENTRIES:
            raise ValueError('Demo directory exceeds entry limit')
        contents = {}
        total_bytes = 0
        for path in files:
            if path.is_symlink():
                raise ValueError('Demo symlinks are not allowed')
            if path.is_file():
                if path.stat().st_size > MAX_FILE_BYTES:
                    raise ValueError('Demo file exceeds size limit')
                total_bytes += path.stat().st_size
                if total_bytes > MAX_EXPANDED_BYTES:
                    raise ValueError('Demo directory exceeds total size limit')
                contents[path] = path.read_bytes()
        budget = [0, 0]
        for path, data in contents.items():
            self.walk(path.name, data, self.collect, budget)
        if not self.capture_count:
            raise ValueError('No supported PNM captures in demo directory')
        reserved = {address for address in self.identities['cmts_mac_address'] if is_alias(address)}
        self.text_mapping = aliases(self.identities['mac_address'], reserved)
        # Preserve cross-role identity: one physical address has one alias.
        used = set(self.text_mapping.values())
        for address in sorted(self.identities['cmts_mac_address'] - self.text_mapping.keys()):
            replacement = address
            if not is_alias(address):
                ordinal = 1
                while ALIAS_PREFIX + ordinal.to_bytes(3, 'big') in used:
                    ordinal += 1
                replacement = ALIAS_PREFIX + ordinal.to_bytes(3, 'big')
            self.text_mapping[address] = replacement
            used.add(replacement)
        self.mappings = {key: {address: self.text_mapping[address] for address in values}
                         for key, values in self.identities.items()}
        budget = [0, 0]
        for path, data in contents.items():
            self.walk(path.name, data, self.rewrite, budget)
        self.validate = None  # The planning pass has already compared every capture.
        planned = []
        budget = [0, 0]
        for path, original in contents.items():
            updated = self.walk(path.name, original, self.rewrite, budget)
            destination = self.root / self.rename(path.relative_to(self.root).as_posix())
            planned.append((path, destination, original, updated))
        # Refresh hashes and identity tokens in text metadata after all binaries.
        result = []
        for path, destination, original, updated in planned:
            if path.suffix.lower() in ('.json', '.md', '.txt'):
                updated = self.refresh_text(path.name, original)
            # Archive compression timestamps must not cause changes on every run.
            if path.name.lower().endswith(('.zip', '.gz', '.tar', '.tgz')):
                if all(old == new for mapping in self.mappings.values() for old, new in mapping.items()):
                    updated = original
            result.append((path, destination, original, updated))
        destinations = [destination for _, destination, _, _ in result]
        if len(set(destinations)) != len(destinations):
            raise ValueError('MAC renaming would collide with another demo file')
        for path, destination, _, _ in result:
            if path != destination and destination.exists():
                raise ValueError('MAC renaming destination already exists')
        return result
