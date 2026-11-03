# Rust development

The first core crate contains a checked byte reader, structured errors, and
PnmHeader/HasPnmHeader. RxMER 1.0 decoding is implemented from the versioned text extracts and pinned
PyPNM fixture, as authorized by the user. Rendered-PDF validation is not claimed.

The shared-header design uses composition: each capture owns a PnmHeader and
implements HasPnmHeader. Default trait methods expose common file type, version
and optional capture time. Trait objects support polymorphic access without
class inheritance, duplicated decoding, or treating every format as one header.

Rust 1.85.1 is pinned for this initial dependency-free crate. The locally installed
toolchain lives in ignored local/cargo and local/rustup; no shell profile was
changed. The wrapper requires that local installation; it does not auto-install.

```bash
cd /home/dev01/Projects/pnm-viewer &&
scripts/cargo-local.sh test --workspace --locked
```

```bash
cd /home/dev01/Projects/pnm-viewer &&
scripts/cargo-local.sh fmt --all -- --check &&
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
```

With an existing Rust installation, standard cargo commands can be run from the
repository root using rust-toolchain.toml. Cargo.lock is committed with source.
Validation: two unit tests and five RxMER integration tests passed on Linux,
including every truncated fixture prefix. Formatting and Clippy passed. No
application build or other platform validation is claimed.

## RxMER behavior and evidence

See specs/field-provenance.csv and specs/coverage.csv. Layout: I27 Table 68,
pages 176-177; I13 Table 101, pages 215-216. Value semantics: both D.2.1.3.1.
Big-endian decoding is corroborated by pinned PyPNM's ! struct formats and fixture;
this note does not claim an independently located normative byte-order rule.
The user authorized using clear text extracts; request the PDF for ambiguities.

CmDsOfdmRxMer borrows input bytes without allocating a sample array. It embeds
PnmHeader and implements HasPnmHeader, exposing the shared base-like API through
trait objects. Decoded values and frequencies are lazy iterators. 0xFF is None,
not PyPNM's clamped 63.5 dB. Raw bytes remain accessible. No scientific statistics
or remote/file I/O occur inside the parser.

Application policies: accept only version 1.0, reject trailing bytes and require
exact declared length, default maximum 65,536 samples (caller configurable).
These policies are not specification limits. Nonzero vendor minor versions are
unsupported, not necessarily invalid. Encoded spacing/indices are preserved;
no PHY-range compliance is claimed. Parse errors include field and byte offset.

Future format-specific decoders must select their own header layout; the shared
header data model is not a universal assumption about capture-time presence.

## Common domain aliases

Shared types in pnm/data_type follow PyPNM src/pypnm/lib/types.py vocabulary:
CaptureTime, ChannelId, FrequencyHz and SubcarrierIdx. FrequencyHz uses u32 for stored and calculated Hz. Other aliases include
SubcarrierSpacingKhz, FileVersionComponent, DataLengthBytes, SampleCount and
RxMerDb. These aliases improve readability without validation or type separation;
Python NewType's static distinction is not reproduced by a Rust type alias.
Raw byte buffers and cursor offsets retain primitive types.

FrequencyHz is u32 throughout. frequencies_hz now returns Result<iterator,
ParseError>; parsing rejects overflowing frequency ranges, and later calls
revalidate public metadata. This is numeric safety, not a 3 GHz PHY limit.
