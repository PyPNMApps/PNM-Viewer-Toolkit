# Rust development

The reusable library lives in `crates/core/src/pnm/`. Its Cargo package and Rust
import name remain `pnm`; the directory name does not change those names.

The first core crate contains a checked byte reader, structured errors, and
PnmHeader/HasPnmHeader. Measurement decoding follows the RxMER pattern, using
the authorized versioned text extracts and pinned PyPNM fixtures. Rendered-PDF validation is not claimed.

The shared-header design uses composition: each capture owns a PnmHeader and
implements HasPnmHeader. Default trait methods expose common file type, version
and optional capture time. Trait objects support polymorphic access without
class inheritance, duplicated decoding, or treating every format as one header.

Rust 1.85.1 is pinned. Serde and serde_json provide output serialization;
exact direct dependency versions and Cargo.lock are committed. The locally installed
toolchain lives in ignored local/cargo and local/rustup; no shell profile was
changed. Run `./install.sh --development` to prepare the local toolchain and native
prerequisites; see [development setup](../install/development.md). The wrapper
requires that local installation; it does not auto-install.

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
scripts/cargo-local.sh test --workspace --locked
```

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
scripts/cargo-local.sh fmt --all -- --check &&
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
```

With an existing Rust installation, standard cargo commands can be run from the
repository root using rust-toolchain.toml. Cargo.lock is committed with source.
Validation is organized by parser, shared header/dispatch, numeric helpers and
CLI behavior. See the test organization below. Native Windows/macOS validation
is pending.

## RxMER behavior and evidence

See development/specs/field-provenance.csv and development/specs/coverage.csv. Layout: I27 Table 68,
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

## Parser implementation pattern

Use `cm_ds_ofdm_rx_mer.rs` as the first concrete parser example:

1. Read and validate the file identifier and supported format version.
2. Decode fields in their documented wire order with the shared `Reader`:
   `read_bytes` for fixed arrays, `read_u8`, `read_u16_be`, and `read_u32_be`
   for unsigned fields. The `be` suffix makes big-endian byte order explicit.
3. Annotate decoded fields with shared domain aliases such as `ChannelId`,
   `MacAddress` and `FrequencyHz`. Reader handles bytes, not PNM semantics.
4. Keep the encoded `DataLengthBytes` separate from `payload_length: usize`.
   Use a checked conversion before host-sized indexing and resource checks.
5. Validate format-specific lengths and limits, borrow the original payload,
   compose `PnmHeader`, and validate derived ranges before returning.

Reader failures preserve the cursor and report the original field offset and
label. Each format still needs its own evidence for layout, endianness, header
fields, sentinels and payload constraints; RxMER's 28-byte header and one-byte
samples are not universal. Add other primitive read methods only when needed.

### Two-tier file decoding

`PnmFile::from_bytes(bytes)` retains the original byte slice and parses the common
header once. `header()` returns the header or its diagnostic; unsupported and
truncated files still expose `bytes()` for raw inspection. `decode(limits)` routes
the parsed header to the supported second-tier parser. It clones only header
metadata and slice references, never the capture buffer.

`PnmHeader::from_bytes(bytes)` owns common metadata and retains `pnm_data()` plus
`payload_offset()`. The central `pnm_file_type.rs` registry supplies identity,
canonical tag, enabled version/layout and decoder route. All eleven listed 1.0
headers are enabled. FEC and latency omit capture time; symbol capture remains a
header-only stub whose semantic decode returns UnsupportedDecoder. Other routes
return their concrete typed captures. Unknown signatures and unsupported versions
return UnsupportedType and UnsupportedVersion respectively.

RxMER's `from_header(header, limits)` reads only the remaining payload through
`Reader::with_origin`, preserving original-file offsets. `parse(bytes, limits)`
remains a convenience entry point using the same header object. The previous
`parse_with_capture_time` helper is replaced. Header fields are private and accessed
through `HasPnmHeader`; callers no longer construct them with struct literals.
Generic header errors precede second-tier type validation (for example, a truncated
FEC header is a truncation error before RxMER rejects that type).

The caller owns the original bytes throughout. Parser objects borrow those bytes;
no self-referential owning buffer, unsafe code or Python sidecar is required.

```rust
use pnm::pnm::parser::{
    cm_ds_ofdm_rx_mer::ParseLimits,
    pnm_file::{PnmFile, DecodedPnm},
    model::DEFAULT_MAX_JSON_BYTES,
};

// `capture_bytes` is owned by the caller and outlives these objects.
let file = PnmFile::from_bytes(&capture_bytes);
let header = file.header()?;
let header_json = header.to_json(false, DEFAULT_MAX_JSON_BYTES)?;
let DecodedPnm::RxMer(rxmer) = file.decode(ParseLimits::default())?;
let decoded_json = rxmer.to_json(DEFAULT_MAX_JSON_BYTES)?;
```

### Output models and JSON

Header `to_model()` returns `pnm_header` with PyPNM's file_type,
file_type_version, major_version, minor_version and capture_time keys. The historic
file_type_version key holds the numeric file discriminator, not the format version.
Missing capture time is null. `to_json(false, limit)` produces header-only JSON;
`to_json(true, limit)` explicitly adds remaining bytes as hexadecimal `data`.

RxMER implements Serde Serialize directly on the parser object. The separate
RxMER output model and to_model() method have been removed. Serialization keeps
colon-separated MAC, spacing/frequency in Hz, dB values and null for unavailable
samples. A private adapter streams values_db() without allocating a decoded vector.
`to_json(limit)` uses the shared bounded writer to produce pretty JSON.

The limit includes indentation; exceeding it returns an error and discards partial
output. Payload hex allocation is separately checked before construction. The
provided default budget is 16 MiB; callers may supply another limit.

JSON is an output representation, never the handoff between parsing tiers. Typed
parser fields and the original binary data remain independently accessible. Statistics and
occupied bandwidth are not part of this decoder output, so it is not full PyPNM
analysis JSON parity. No desktop/UI implementation is included in this change.

### Shared header evidence

FEC Summary: I27 Table 71 p181 and I13 Table 104 p220 show the four-byte signature,
two version bytes and then channel ID; there is no capture-time field. Both use
1.0 for the supported header. The fixture and PyPNM pnm_header.py corroborate this
six-byte split. This does not imply a byte order for multi-byte FEC payload fields.
Central identities are from the file-format tables and pinned PyPNM vocabulary;
PNN10 is the display tag for a fourth byte of 0x0a. LLD01 is recognized as
4c4c4401 with decoding enabled and synthetic test coverage only. PXX9 is internal PyPNM SNMP data and is
not recognized as an on-disk DOCSIS format here.

### Generate JSON from the RxMER fixture

Run the actual parser against the preserved PyPNM fixture:

```bash
mkdir -p local
scripts/cargo-local.sh run --quiet --locked -p pnm \
  --example rxmer_json > local/rxmer.json
```

The example embeds `tests/fixtures/pypnm/tests/files/rxmer.bin` and writes JSON to
stdout. Generated output stays in ignored `local/`. This fixture produces 7,480
samples; the first five values are 42.75, 43.0, 43.0, 40.5 and 41.25 dB.
The generated metadata and every JSON sample were checked against the binary
fixture, including quarter-dB conversion and unavailable-value handling.

### Logging

`tracing` provides level-based events, structured fields and spans grouping a
parse operation. The separate `logging` crate configures the text subscriber;
applications call `logging::init()` once during startup. `pnm` only emits
events and never initializes process-global logging. The example uses the logging
crate as a development dependency; core library consumers choose their subscriber.

Logs go to stderr, keeping JSON on stdout. The default filter is info; parser
success events and spans use debug. Header/payload/dispatch failures emit error
events and still return structured ParseError values. Nested failure events may
appear at both the failing parser and dispatch boundary. Logging fields include
byte counts, file type and sample count, not capture bytes or MAC addresses.

```bash
RUST_LOG=pnm=debug scripts/cargo-local.sh run --quiet --locked \
  -p pnm --example rxmer_json > local/rxmer.json
```

Use `RUST_LOG=off` to silence logging or a standard filter such as
`warn,pnm=debug`. Invalid filters and repeated global initialization return
errors rather than panicking or replacing an existing subscriber. File sinks,
rotation and remote telemetry are not configured.

The simpler `log` crate offers message levels and targets; `tracing` additionally
provides structured fields and nested operation context. Both are third-party
crates. See the [tracing subscriber documentation](https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/).

## Public API examples

See [the examples guide](../../crates/core/examples/README.md) for one runnable
example per file type, concrete parser imports, fixture defaults, and direct
typed sample access. `pnm_json` demonstrates automatic file dispatch.
Latency requires an input path; symbol capture exports header metadata only.

## Additional measurement decoders

See [format support](../reference/formats.md) for the full matrix and JSON differences.
The same RxMER flow applies to every implemented family:
`PnmFile -> PnmHeader -> from_header -> typed capture -> bounded JSON`.
`parse_limits::ParseLimits` is shared; the old RxMER import remains available.
Its default 65,536-item budget includes nested profile/flow containers and records,
and latency edges/bins. It is application policy, not a protocol maximum.

Export any supported local capture with the generic example:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
scripts/cargo-local.sh run --quiet --locked -p pnm --example pnm_json --   tests/fixtures/pypnm/tests/files/channel_estimation.bin > local/channel-estimate.json
```

The example caps input at 16 MiB and uses the existing bounded JSON writer.
It writes JSON to stdout and diagnostics to stderr. Core parsers perform no I/O.
Historical parser validation: 24 Rust tests before the per-parser test split,
formatting and Clippy with warnings denied.
Nine real fixture JSON exports passed; eight additional fixtures were compared
value-by-value with independent `struct` decoding. Latency has synthetic tests
only. No GUI, packaging or other-OS runtime validation is implied.

## Numeric, DSP and parser organization

```text
crates/core/src/
├── lib.rs                         # Rust crate entry point
├── common/                        # PyPNM top-level lib equivalent
│   ├── reader.rs                  # Checked binary cursor
│   ├── json.rs                    # Bounded JSON writer
│   └── signal_processing/         # Future DSP modules; scaffold only
├── pnm/
│   ├── data_type/                 # Domain types and shared unit constants
│   ├── lib/
│   │   └── fixed_point_decoder.rs # QFormat and pure signed conversion
│   └── parser/
│       ├── pnm_file.rs            # Central dispatch
│       ├── pnm_header.rs          # Shared header and borrowed payload
│       ├── pnm_file_type.rs       # Identity/layout registry
│       ├── parse_limits.rs       # Application decode budgets
│       ├── cm_*.rs               # Measurement-specific parsers
│       ├── model/                # Header JSON representation
│       └── support/
│           ├── payload.rs        # Length, route and budget validation
│           ├── serialization.rs  # Shared JSON field adapters
│           └── values.rs         # Borrowed wire arrays and lazy JSON views
└── diagnostic.rs                  # Shared structured failures
```

This follows the inspected PyPNM checkout: `pypnm/pnm/lib/fixed_point_decoder.py`
owns fixed-point math, while `pypnm/lib/signal_processing/` holds separate
algorithms such as IFFT, group delay, windows and complex-array operations.
Rust uses `common` for that top-level utility package to distinguish it from
`src/lib.rs`, the crate entry point. `pnm/lib` is an ordinary nested Rust module.

`pnm::lib::fixed_point_decoder::{QFormat, decode_fixed_point, decode_iq}`
accepts already-decoded signed components. It has no Serde, reader, file-routing
or sentinel logic. `QFormat::S2_13` and `QFormat::S1_14` select the existing
central scale constants in `pnm::data_type::units`. Wire adapters handle byte
order and channel-estimate exclusions before calling the numeric helper.

Future DSP algorithms belong in focused modules under `common/signal_processing`.
That directory currently contains documentation only; no FIR filter, FFT/IFFT,
group-delay analysis or other DSP algorithm was added by this organization change.
PNM-specific analysis can compose those algorithms later without making generic
DSP depend on capture parsers. The mixed `binary_values.rs` module was removed.

The existing `pnm::parser::reader` and `pnm::parser::model::to_json` import paths
remain re-exports of the single implementations in `common`. New general-purpose
code should use `common::reader` and `common::json` directly. Concrete parser
APIs and JSON contracts are unchanged.

Historical reorganization validation: all 24 tests, rustfmt and Clippy passed. JSON exports from all nine
real fixtures were byte-for-byte identical before and after the reorganization.

Complex captures expose `iq()` and `raw_iq()` in Rust and `iq` in JSON.
The shared `IqPair` type is ordered `[I, Q]`: in-phase/real then
quadrature/imaginary. `IqSamples` owns the borrowed wire view; `decode_iq`
performs fixed-point conversion. RxMER keeps its scalar dB naming.

## Test organization

Each concrete parser has a `crates/core/tests/test_<parser_module>.rs` integration
test target. Current and last-update pre-EQ share `test_cm_us_ofdma_pre_eq.rs`,
matching their shared implementation. Header/identity/dispatch tests remain in
`test_pnm_file.rs`; reusable assertions live under `tests/common/`. Reader unit
tests remain beside their implementation.

`test_fixed_point_decoder.rs` verifies s2.13 and s1.14 known values, signed
boundaries, I/Q ordering, and all 65,536 signed input words for each format.
Parser tests independently verify on-wire scale selection and exclusion rules.
See the [testing skill](../../skills/pnm-testing/SKILL.md) for maintenance rules.

Run a single parser's suite with:

```bash
scripts/cargo-local.sh test -p pnm --test test_cm_ds_ofdm_rx_mer --locked
```

### Shared fixture catalog

Tests and examples use named embedded bytes such as `pnm_fixtures::RXMER`.
The development-only package at `tests/fixtures/` owns the fixture directory
and each filename in one place, anchored with `CARGO_MANIFEST_DIR`. It is a
`dev-dependency` of `pnm`, so production library builds do not pull in captures.
The existing binary files and their provenance remain unchanged.

## Shannon capacity helper

`pnm::common::signal_processing::shannon::minimum_snr_db(bits_per_symbol)` returns
an ideal dB SNR bound for positive bit counts and `None` for zero. It does not
interpret wire assignment codes or apply DOCSIS margins. Modulation presentation
maps valid QAM assignments to `BitsPerSymbol` before calling the helper. Known
values and the full u8 domain have independent numeric tests.


Modulation capacity analysis lives in
`crates/core/src/pnm/analysis/modulation_profile.rs`; reusable useful-symbol
rate and bandwidth arithmetic lives in `common/signal_processing/ofdm.rs`.
Exports include per-profile `analysis` through the core capture serializer, so
`pnm-decode INPUT --stdout` and desktop JSON agree without UI post-processing.
See [the capacity contract](../reference/formats.md#per-profile-ideal-capacity-analysis).

## Pre-EQ pair precheck and prediction

Use `pnm::pnm::analysis::upstream_pre_equalization::comparison::precheck` with
current and last-update parsed captures to get `PairMetadata` or a field-specific
`ComparisonError`. It checks both CM and CMTS MAC addresses and strictly newer
encoded capture time, in addition to exact grid/version/count matching.

`PreEqComparison::calculate(original, correction, CorrectionMode::Adjust)`
(or explicit `Set`) rechecks the pair and produces full raw prediction/delta,
level-aligned magnitude, delay and summary data. Use `to_json(max_bytes)` for
bounded output. The common DSP helper owns complex multiply/replace and the
coefficient-power reference. Shared MAC formatting has one owner in data_type.

The desktop adapter’s `Capture::pre_eq_precheck` / `pre_eq_compare` use the same
core implementation. `bounded_range` prepares joint graph points for an exact
source range while retaining full summaries and full-channel I/Q maps. Only
`write_json_new` exports the full snapshot; existing destinations are rejected.
See the [user flow](../ui/measurements/upstream-pre-eq/README.md#correction--delta-from-the-left-gutter)
and [CLI options](../cli/usage.md#pre-eq-precheck-and-correction-comparison).

## RxMER temporal collections

`crates/core/src/pnm/analysis/rxmer/aggregate.rs` owns compatible per-carrier
statistics and average-line model analysis.
`apps/viewer/src/measurements/rxmer/collection/` owns bounded container I/O,
SHA-256 deduplication, catalogs, viewport selection and no-clobber export.
Qt uses a generation/cancellation-guarded worker adapter.
See [equations and JSON](../reference/rxmer-min-avg-max.md).

Exercise the public batch API without Qt (PATH is a directory or archive):

```bash
./scripts/cargo-local.sh run -p viewer --example rxmer_collection -- PATH
./scripts/cargo-local.sh run -p viewer --example rxmer_collection -- PATH GROUP_ID
```

The first command prints a catalog; the second prints full aggregate JSON for
one catalog group. This is a Rust API example, not a new pnm-decode CLI flag.
The importer uses bundled Rust archive/decompression crates; no external tar,
unzip, zstd command or new system development package is required. Existing
C/C++ build prerequisites also build the bundled Zstandard decoder.

### RxMER Playback

`collection/playback.rs` adds `Collection::frame(group_id, frame_index)`. Indices
follow the catalog's header-epoch/source-name order. It projects only the chosen
capture into the existing single-source result, preserving missing samples and
full resolution; it does not duplicate the whole collection. The shared viewport
bounds each plot to 1024 source positions. Invalid groups, frames and ranges fail
without replacing the current plot.

The Qt adapter performs no file I/O or collection-wide aggregation on timer ticks.
It retains the selected frame for shared Modulation & Capacity analysis. Each
successful frame selection invalidates pending analysis; QML pauses before modal
analysis and keeps the exact carrier range between frames. Play speed is UI
cadence, independent of the capture timestamp interval. No new packages are
required. See the [Playback user guide](../ui/measurements/rxmer/playback.md).

Playback viewport `average_mer_db` is the full displayed capture's arithmetic
dB mean over valid carriers, not the decimated or zoomed plot. All unavailable
returns null. QML displays Avg MER to one decimal and retains Capture UTC.
