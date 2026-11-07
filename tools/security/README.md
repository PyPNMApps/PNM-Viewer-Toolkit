# SW-QA capture hygiene

Single examples use **00ca.0000.0001** (`00:ca:00:00:00:01`). A collection
keeps distinct modems with sequential aliases: `…:01`, `…:02`, `…:03`, etc.
Repeated captures keep the same alias across channels and timestamps. CMTS
fields are sanitized too; an address shared across roles retains one identity.
Aliases use the fixed `00:ca:00` prefix and a nonzero 24-bit counter.

## Check before sharing or pushing

```bash
./scripts/sw-qa-hygiene.sh
```

This read-only check runs in the standard pre-push gate and GitHub CI. The save
helper also checks MACs before staging. It checks fixture provenance, all linked
JSON/hex examples, and Git's tracked/new nonignored binary and archive inventory.
A new demo folder therefore cannot silently bypass the check. Failures stop the
gate; checks never silently rewrite files. Ignored private collections are
checked only when explicitly selected with `--directory`.

## Rewrite shipped fixtures and examples

```bash
./scripts/cargo-local.sh build --release --locked -p pnm-decode
python3 tools/security/pnm-mac-updater.py --dry-run
python3 tools/security/pnm-mac-updater.py --write --decoder target/release/pnm-decode
```

This updates the nine imported captures, registered demo pairs, JSON/hex
examples and current checksums. Original upstream hashes and attribution remain.
The normal check requires the fixed alias for these single examples.

## Rewrite a demo collection

```bash
python3 tools/security/pnm-mac-updater.py --directory /path/to/demo --dry-run
python3 tools/security/pnm-mac-updater.py --directory /path/to/demo \
  --write --decoder /path/to/pnm-decode
python3 tools/security/pnm-mac-updater.py --directory /path/to/demo
```

Select a curated demo directory. The tool handles `.bin`, ZIP, TAR, TAR.GZ/TGZ
and gzip captures without extracting archives. It changes declared MAC fields,
known MAC tokens in names/text metadata, and matching SHA-256 references.
CM aliases are assigned from sorted original identities; existing aliases retain
their numbers when captures are appended. Separate collections are independent.
No reverse mapping or original MAC addresses are logged.

Every changed capture is decoded before and after with the real Rust CLI;
non-MAC decoded output must match exactly. Only evidenced six-byte header fields
are rewritten; measurement bytes, timestamps, channel IDs and grids are retained.
Unknown/truncated layouts, symlinks, unsafe archive paths and naming collisions
stop planning before writes. Limits: 256 MiB per file/member, 512 MiB total
input/expanded traversal, 20,000 entries and four nested archive levels.
Per-file replacement is atomic; a batch I/O error can leave a partially updated
collection, so review and rerun the check before staging or sharing.

Use `--decoder PATH` for a different Cargo target directory or Windows executable.
The default is `target/release/pnm-decode`. The script needs only Python's standard
library; it is development tooling, not a production Python dependency.
Refresh screenshots separately: existing images and published Git history,
tags/releases do not change when example binaries are rewritten.

## Upstream attribution

Adapted from PyPNM `tools/pnm/pnm-mac-updater.py`, commit
`82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`, original SHA-256
`c3c9e19918721e80a45d31a8b268d6fd09434b7195c18fcc0e306ef6a0d5af3a`.
Preserve Maurice Garcia's copyright, the
[upstream Apache license](../../tests/fixtures/pypnm/LICENSE) and
[NOTICE](../../tests/fixtures/pypnm/NOTICE).

Field positions follow this project's format-1.0 parsers: FEC has a shorter
common header, Histogram has no channel byte before its MAC, and pre-EQ has CM
and CMTS fields. Symbol Capture remains unsupported for MAC rewriting.
PyPNM's text `--clear-oui` replacement is never used on arbitrary binary bytes.

## Installer build order

Installer demo ZIPs are generated from expanded, approved repository captures.
`tests/fixtures/demos/installer-demos.json` records their paths, sizes and hashes.
`python3 scripts/release/samples.py --check` verifies that manifest and runs
read-only MAC hygiene before any compression; `scripts/sw-qa-hygiene.sh` also
runs this source gate. Native DEB/MSI/DMG builders all use the shared bundler.

Source preparation uses the standalone sanitizer; release builds never silently
rewrite sources. Refresh and review manifest hashes after an intentional source
change. Prebuilt demo archives are rejected as packaging inputs. ZIP generation
uses fixed metadata and sorted entries. See [packaging](../../docs/release/packaging.md)
for the standalone bundle command. External archive sanitation remains available
through `--directory` for separately supplied data.
