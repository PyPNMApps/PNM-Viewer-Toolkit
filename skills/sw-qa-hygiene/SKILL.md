---
name: sw-qa-hygiene
description: Sanitize PNM example and demo captures before sharing or pushing, retaining multi-modem identities and validating binary and archive MAC fields.
---

# PNM example and demo hygiene

Use `tools/security/pnm-mac-updater.py`; do not search/replace arbitrary binary
byte sequences. Single examples use `00ca.0000.0001`. Collections assign distinct
CMs sequential aliases (`00ca.0000.0002`, etc.), keeping one identity across all
channels and times. Use binary headers, never filenames, to identify modems.
Include CMTS MAC fields. Preserve comparisons and original/update pairing.

For a collection, run `--directory PATH --dry-run`, then `--directory PATH
--write --decoder PATH`. The tool handles expanded captures, ZIP, TAR, TAR.GZ
and gzip captures, and refreshes matching names, metadata and checksums. Unknown
layouts, links, unsafe archives and allocation limits fail closed. Writes compare
non-MAC output using the Rust CLI. Do not weaken these checks to import new data.

Run `scripts/sw-qa-hygiene.sh` before saving/pushing. The save helper, pre-push
gate and CI enforce the policy read-only. New nonignored capture assets are
inventoried through Git, including archive members. Keep ignored raw/private
working data out of Git; sanitize the curated demo directories explicitly.
Preserve provenance, licenses and upstream hashes. Check screenshots separately;
changing captures does not change existing pixels or old Git history/releases.

See [tool details](../../tools/security/README.md) for commands and limits.

## Installer demo archives

Keep approved demo sources expanded under `tests/fixtures/demos/`, registered in
`installer-demos.json` with paths, byte sizes and current SHA-256 values. Do not
commit prebuilt installer demo ZIPs. Sanitize expanded captures explicitly during
preparation, then refresh the approved manifest; preserve original/source evidence.

All native installer builders must call `scripts/release/samples.py`'s shared
`bundle()` function. It runs read-only hygiene on expanded sources before creating
any ZIP, then writes deterministic archives into the generated payload. A failed
hash, extra file or unapproved MAC stops packaging; never silently rewrite source
captures during a release build. No unzip/rezip step belongs in this build path.
Standalone imported customer/demo archives still use the sanitizer's archive mode.

Run `python3 scripts/release/samples.py --check` for source-only validation, or
`--output NEW_DIRECTORY` to exercise bundle generation. Keep generated bundles
under ignored `local/` or `dist/`. Verify installed `single/`, RxMER ZIPs and the
pre-EQ comparison pair; keep standalone CLI packages capture-free and the 50 MB
installer gate active. See [packaging](../../docs/release/packaging.md).
