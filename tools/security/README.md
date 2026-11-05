# Fixture MAC hygiene

All imported PNM fixture MACs use the user-selected alias **aabb.ccdd.eeff**,
serialized as **aa:bb:cc:dd:ee:ff**. This applies to both CM and CMTS fields;
this dataset intentionally does not preserve distinct device identities.

Check (read-only, default):

```bash
python3 tools/security/pnm-mac-updater.py
```

Explicit rewrite, with Rust decode comparison before any write:

```bash
./scripts/cargo-local.sh build --release --locked -p pnm-decode
python3 tools/security/pnm-mac-updater.py --dry-run
python3 tools/security/pnm-mac-updater.py --write --mac-address aabb.ccdd.eeff
```

A different Cargo target directory or Windows build can select its decoder with
`--decoder PATH`. The tool accepts dotted, colon, hyphen and compact MAC input.
It never prints the previous addresses. It only processes the registered binary
fixtures, not personal captures or arbitrary repository text. CI, save and push
checks never rewrite files. Failed checks block staging/pushing.

Field positions come from the project's format-1.0 header/measurement parsers.
FEC has a shorter common header; Histogram has no channel byte before its MAC;
both pre-EQ formats have a second CMTS MAC. Unknown versions, missing fixtures,
checksum drift and symlinks fail the check. Original hashes remain in fixture
provenance; current hashes identify sanitized derivatives. The write command
compares every decoded non-MAC value and changes only declared six-byte ranges.
Per-file replacement is atomic; inspect and rerun the check after a batch I/O
failure. Regenerate linked JSON and real UI screenshots from sanitized inputs.

## Upstream attribution

Adapted from PyPNM `tools/pnm/pnm-mac-updater.py`, commit
`82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`, original SHA-256
`c3c9e19918721e80a45d31a8b268d6fd09434b7195c18fcc0e306ef6a0d5af3a`.
Preserve Maurice Garcia's copyright, the
[upstream Apache license](../../tests/fixtures/pypnm/LICENSE) and
[NOTICE](../../tests/fixtures/pypnm/NOTICE).

Local changes remove PyPNM imports, replace first-match byte searches with
verified field positions, cover the CMTS field, fail closed, mask identifiers,
and add checksum/provenance checks, explicit writes and Rust decode validation.
PyPNM's text `--clear-oui` rewriting is deliberately not used on binary files.
This is development-only tooling; production packages need no Python runtime.
Existing Git history, tags and releases are unchanged.
