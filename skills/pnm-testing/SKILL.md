---
name: pnm-testing
description: Add or reorganize PNM Viewer Rust parser and numeric tests, keeping one integration-test file per concrete parser and independent math and Q-format coverage. Use for parser or numeric changes and testing reviews, not unrelated documentation edits.
---

# PNM testing

## Ownership and layout

- Give every concrete `cm_*.rs` parser its own
  `crates/core/tests/test_<parser_module>.rs`, including header-only stubs.
  Current and last-update upstream pre-EQ share a parser and therefore a test
  file, with separate cases for both identities and scales.
- Keep common-header/identity/dispatch tests in `test_pnm_file.rs`. Keep reader
  unit tests with the reader. Public-API tests under `tests/` are Rust integration
  tests even when they test one component.
- Keep shared test plumbing under `tests/common/`; assertions for a measurement
  belong in its dedicated test file. Do not rebuild an aggregate parser suite.
- Keep independent fixed-point tests in `test_fixed_point_decoder.rs`. Give new
  math or DSP components focused tests as they are implemented; no placeholder
  tests for unimplemented algorithms.
- When splitting tests, account for every existing assertion and fixture before
  removing the old suite. Check that every concrete parser has a nonempty test
  target and that Cargo actually runs it. Do not rely on test counts alone.

## Behavior to verify

Exercise each parser's public API and its JSON representation. Preserve known
fixture metadata and sample expectations, borrowing behavior where applicable,
truncation, declared lengths, trailing bytes, unsupported types/versions, decode
limits and bounded JSON behavior. Reuse common checks without hiding ownership.
Use named captures from the development-only `pnm-fixtures` crate (for example
`pnm_fixtures::RXMER`). Declare fixture paths only in `tests/fixtures/lib.rs`;
do not repeat relative paths or add fixture dependencies to production code.
Preserve PyPNM provenance and licenses; never run its capture services.
Label synthetic inputs clearly. Symbol tests must assert header preservation and
unsupported semantic decoding, not imply a complete symbol parser.

For math, use independent expected values rather than calculating expectations
with production helpers/constants. Cover zero, positive and negative fractions,
one least-significant step, signed minima/maxima, I/Q ordering, and each supported
Q-format. Binary fractions can be compared exactly; use a documented tolerance
for inexact calculations. An exhaustive 16-bit resolution/round-trip check is
cheap, but supplements rather than replaces known-value cases.

Test byte order, scaling selection and missing/excluded markers through the
owning parser as well. A numeric helper must not interpret a channel-estimate
sentinel as missing; identical bits may be valid in constellation/upstream data.
Keep RxMER quarter-dB, spectrum hundredth-dB and frequency overflow checks in
those parsers' files. Do not invent protocol values to fill coverage gaps.

## Validation

Run from the repository root using the local toolchain:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
```

For a focused iteration, use `test -p pnm --test test_<parser_module> --locked`.
Report actual checks and evidence limits. A passing decoder test does not prove
DOCSIS compliance, DSP accuracy outside tested cases, or desktop functionality.
Follow the coding-style skill for banners and descriptive identifiers.

## Installed desktop resources

When changing packaging resources, test executable-relative discovery for
Windows/Linux and macOS bundle layouts. Installed smoke tests must verify and
open packaged example bytes, preserve the render marker and CLI/JSON comparison,
and check the initial Open location. Test missing/corrupt sample failure and
retain upstream notices/provenance. Source fixtures alone do not prove inclusion.

Windows native lifecycle checks cover shortcut opt-out, opt-in and removal;
macOS checks its Info.plist icon reference and packaged ICNS. XML/asset checks
on Linux supplement native MSI/DMG tests; they do not prove interactive wizard,
Finder or Windows Explorer appearance.
