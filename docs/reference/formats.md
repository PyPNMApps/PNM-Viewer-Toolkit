# Format support and evidence

The Rust core implements the following file-format version **1.0** decoders.
The project remains in the dev phase. See the [desktop measurements](../ui/measurements.md)
for the subset currently supported by the Qt viewer.

| Capture | Route/module | Verification |
| --- | --- | --- |
| RxMER (PNN4) | `cm_ds_ofdm_rx_mer` | PyPNM fixture and boundary tests |
| Channel estimate (PNN2) | `cm_ds_ofdm_chan_estimate_coef` | PyPNM fixture; signed s2.13 and exclusion pairs |
| Constellation (PNN3) | `cm_ds_const_disp_meas` | PyPNM fixture; signed s2.13 |
| Histogram (PNN5) | `cm_ds_hist` | PyPNM fixture; separate dwell/hit lengths |
| Current upstream pre-EQ (PNN6) | `cm_us_ofdma_pre_eq` | PyPNM fixture; s2.13 |
| Last upstream pre-EQ update (PNN7) | `cm_us_ofdma_pre_eq` | Separate PyPNM fixture; s1.14 |
| FEC summary (PNN8) | `cm_ds_ofdm_fec_summary` | PyPNM fixture; profile/interval counts |
| Spectrum analysis (PNN9) | `cm_spectrum_analysis` | PyPNM fixture; signed hundredths dB; partial final segment |
| Modulation profiles (PNN10) | `cm_ds_ofdm_modulation_profile` | PyPNM fixture; synthetic skip assignments |
| Latency summary (LLD01) | `cm_latency_rpt` | **Synthetic tests only; real capture still needed** |
| Symbol capture (PNN1) | `cm_symbol_capture` | **Header-only stub; semantic dispatch returns UnsupportedDecoder** |

Each concrete decoder composes `PnmHeader`, implements `HasPnmHeader`, accepts
`parse(bytes, limits)` or `from_header(header, limits)`, and serializes directly.
The common header retains the original payload. Samples are borrowed and decoded
lazily for complex, histogram, FEC and spectrum measurements. Modulation keeps
bounded compact schemes; latency keeps bounded typed records. Neither duplicates
the input file. Symbol capture intentionally has no measurement fields.

All decoders reject truncated fields, inconsistent declared lengths and trailing
bytes. Nested work/allocations count against `ParseLimits.max_samples` as well as
wire bounds. Unknown identities, schemas and versions return structured errors;
`PnmFile.bytes()` still exposes the original input after a decoding failure.
Only minor version zero is enabled; this is an implementation support policy,
not a claim that vendor minor versions are invalid.

Complete [CLI JSON examples](../cli/usage.md#json-examples-by-file-type) are
linked for every decoded family, plus synthetic latency and symbol-header cases.

## Evidence and compatibility

Layouts were reviewed in the authorized text extracts of
CM-SP-CM-OSSIv3.1-I27-250219 and CM-SP-CM-OSSIv4.0-I13-260429. See
[coverage](../../development/specs/coverage.csv), [field provenance](../../development/specs/field-provenance.csv)
and [format evidence](../../development/specs/measurement-formats.md). These evidence
files are local-only under ignored `development/` and are absent from fresh clones.
No rendered-PDF validation,
latest-issue verification or blanket DOCSIS compliance is claimed.

PyPNM is pinned at `82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`.
[Unmodified fixtures and licensing](../../tests/fixtures/pypnm/README.md) are retained.
PyPNM's latency implementation is a stub; its symbol parser is not used.
Internal SNMP spectrum `PXX9` remains excluded from this offline file viewer.

JSON follows PyPNM vocabulary where practical, with these deliberate differences:

- Channel-estimate `(0x8000, 0x8000)` pairs become `null`; raw bytes remain intact.
- Complex fixed-point values retain exact binary fractions rather than rounding
  to six decimals. Derived sections are identified separately from raw fields.
- Complex capture byte lengths use `data_length`, including constellation and
  upstream coefficients. Constellation samples and coefficient pairs use `iq`, ordered
  `[I, Q]` (in-phase/real then quadrature/imaginary). RxMER retains `values` in dB.
- FEC `codeword_entries` is an array of records, each with timestamp and counters.
  Its common header capture time stays `null`, rather than being synthesized from
  the first interval. Latency also has no common capture timestamp.
- Modulation schemes are tagged `range`/`skip`. Assignments use textual
  `modulation_order`, nullable `bit_loading` and `carrier_type`; original numeric
  assignments remain in `modulation_order_code`. Skip schemes prefix each field
  with `main_` or `skip_`. Unknown scheme layouts are rejected.
- Histogram symmetry is retained as the encoded number: the real fixture uses 2.
  This reader does not guess a boolean mapping or invent bin coordinates.
- Spectrum retains raw ENBW/window parameters without assigning unverified units.
  A partial final segment is valid; missing declared bytes are not.
- Latency labels remain exact 16-byte arrays, bin edges use hundredths of a
  millisecond, and maximum latency uses microseconds. Reserved words are retained.
  JSON emits exact u64 bin counts; future JavaScript IPC must avoid narrowing them
  to JavaScript Number when they exceed its safe integer range.

See [Rust development](../development/rust.md) for export commands.

## Modulation-profile JSON assignments

`modulation_order` follows PyPNM names (`qpsk`, `qam_16` through `qam_16384`,
`zero_bit_loaded`, `continuous_pilot`, `exclusion`, `plc`, `reserved_3`,
`reserved_5`). Unassigned codes use `unknown`; the original byte is always
retained as `modulation_order_code`. This changes the former numeric
`modulation_order` JSON field; Rust parser fields remain encoded numeric values.

```json
{
  "schema_type": "range",
  "modulation_order": "qam_4096",
  "modulation_order_code": 12,
  "bit_loading": 12,
  "carrier_type": "data",
  "num_subcarriers": 110
}
```

| `carrier_type` | `bit_loading` (bits per symbol) |
| --- | --- |
| `data` | 2 for QPSK, or 4 and 6–14 for the specified QAM orders |
| `zero_bit_loaded` | 0 |
| `continuous_pilot` | 1 (BPSK) |
| `exclusion` | `null` |
| `plc` | 4 (16-QAM data segment; BPSK preamble is separate) |
| `reserved` | `null` |
| `unknown` | `null` |

The roles and assignments are identified in **CM-SP-CM-OSSIv3.1-I27-250219,
D.2.1.7, PDF pages 153–154**, and independently in
**CM-SP-CM-OSSIv4.0-I13-260429, D.2.1.7, PDF pages 192–193**, using the authorized
text extracts. `carrier_type` is this application's classification of those
roles, not an additional binary field. PLC remains overhead, not a data carrier.

`constellation` identifies the physical modulation separately from the OSSI
assignment name. `shannon_minimum_mer_db` is the Rust-calculated ideal reference
`10 log10(2^bit_loading − 1)`, not measured MER or a receiver requirement.
Continuous pilots export `bpsk`, 1 bit and **0 dB**; PLC exports `qam_16`, 4 bits
and **11.760912590556813 dB** for its data segment. There is no arbitrary 2 dB
floor. Pilot boosting is a transmit-power rule, not an additive MER threshold.
These fields are null for unknown, reserved, excluded and zero-loaded assignments.
Zero-loaded `bit_loading` remains 0.

PLC also exports `preamble: {"constellation":"bpsk", "bit_loading":1,
"shannon_minimum_mer_db":0.0}`; other assignments have null `preamble`.
The profile does not identify the instantaneous preamble/data symbol. These
parameters describe its structure, not observed timing or decoded PLC payload.

Constellation evidence: **CM-SP-PHYv3.1-I20-230419, sections 7.5.13.1–3,
PDF pages 148–151**, and **7.5.15.3–4, PDF page 171** (authorized text).
The PLC has 8 preamble symbols followed by 120 data symbols. Continuous pilots
are BPSK, boosted about 6 dB. DOCSIS 4.0 FDD delegates these definitions through
**CM-SP-PHYv4.0-I07-250627, sections 7.3.5.13 and 7.3.5.15, PDF page 146**.
This is text verification, not rendered-PDF validation.

Skip schemes expose `main_modulation_order`, `main_modulation_order_code`,
`main_bit_loading`, `main_carrier_type` and the corresponding `skip_` fields.
They also expose `main_constellation`, `main_shannon_minimum_mer_db`,
`main_preamble` and corresponding `skip_` fields. They retain one
`num_subcarriers` for the total alternating sequence, starting
with the main assignment. Export keeps compressed schemes and all profiles;
it does not substitute expanded chart arrays for the original assignments.

### Per-profile ideal capacity analysis

Every modulation-profile capture export includes `profiles[].bandwidth_bps`
and `profiles[].analysis`, shared
by the core parser export, `pnm-decode` and the Qt viewer. Calculations operate
on compressed range/skip counts; odd skip runs start and end with main.
`bandwidth_bps` is directly beside `profile_id` and `schemes`. It reuses
`analysis.theoretical_throughput_bps`: sum of data-carrier bit loading × spacing
in Hz, with the same null handling and overhead exclusions. It is a bit rate;
`analysis.data_bandwidth_hz` separately describes nominal RF data-carrier width.

```json
{
  "data_subcarrier_count": 7408,
  "data_bits_per_symbol": 88896,
  "data_bandwidth_hz": 185200000,
  "theoretical_throughput_bps": 2222400000,
  "useful_symbol_duration_s": 0.00004,
  "unclassified_subcarrier_count": 0,
  "complete": true,
  "basis": "ideal_gross_useful_symbol",
  "excluded_overheads": ["cyclic_prefix", "fec", "mac_and_other_phy_overhead"]
}
```

- Data bandwidth is the sum of nominal data-carrier widths: count × spacing.
  Disjoint data regions are summed; this is not the full channel frequency span.
- Data bits per useful OFDM symbol sum only QPSK/QAM data assignments. Pilots,
  PLC, exclusion and zero-loaded carriers contribute no data capacity.
- Ideal gross rate is data bits per useful symbol × spacing in Hz. This uses
  `Tu = 1 / spacing`: 25 kHz gives 40 µs and 50 kHz gives 20 µs.
- The file does not supply cyclic-prefix duration, coding rate or other PHY/MAC
  overhead. The result is an ideal gross upper bound, not measured throughput
  or usable application capacity. No CP or FEC defaults are guessed.
- Reserved/unassigned carriers make `complete` false and the rate `null`.
  Known-data counts and bandwidth remain available. Zero spacing or bandwidth
  overflow also makes the analysis incomplete, with unavailable values `null`.

Timing evidence: **CM-SP-PHYv3.1-I20-230419, section 7.3.1, Table 6, PDF page 47**,
reviewed using its clear text extraction. The capacity formula and summed data
bandwidth are engineering calculations using those timings and the OSSI
assignments cited above; they are not additional fields specified by OSSI.
The pinned PyPNM modulation parser supplies textual assignment vocabulary;
no PyPNM net-throughput calculation is assumed. Capture exports retain every
profile regardless of the UI's selected profiles or carrier filters.


## Constellation Display reference analysis

The original `actual_modulation_order` is an OSSI `DsOfdmModulationType` enum,
not a literal QAM size or a modulation-profile assignment code. For example, 7
means 256-QAM; 3 means QPSK. Raw s2.13 `iq` pairs remain unchanged.

`constellation_analysis` is emitted by the core parser for CLI and desktop:

```json
{
  "decision_rule": "nearest_reference_euclidean",
  "normalization": "unit_average_symbol_power",
  "soft_samples_rescaled": false,
  "reference": {
    "order": 256,
    "bit_loading": 8,
    "scale_factor": 0.07669649888473704,
    "iq": [[1.1504474832710556, 1.1504474832710556]]
  }
}
```

This excerpt shows only the first ideal point. Full exports contain every ideal
point (256 here), plus every original measured pair in the top-level `iq` array.
`reference` is null for other, zero-valued or unknown modulation codes. The
viewer still displays their measured points; it does not infer a QAM order.

The copied [resources](../../crates/core/resources/qam-table/README.md) retain
PyPNM hashes and licensing. Rust validates supported point counts, uniqueness
and scaling energy during compilation, then embeds normalized static tables.
References use `1/sqrt(Es)`. Captured samples are already normalized by the CM
and are never rescaled. No database or runtime resource-file access is needed.
The preserved 8-QAM resource has inconsistent energy/scaling and is excluded;
8-QAM is not one of the downstream `DsOfdmModulationType` capture choices.

Evidence, using authorized text extracts:

- CM-SP-CM-OSSIv3.1-I27-250219, D.2.7.7.1.11, PDF 172:
  s2.13 measured samples with approximately unit constellation average power.
- CM-SP-CM-OSSIv4.0-I13-260429, D.2.7, PDF 209: s2.13 and unit-power scaling.
- The independent enums appear in F.3.2, PDF 231 (I27) and PDF 276 (I13).
- PyPNM `analysis.py` and `auto_gen_qam_lut.py` provide the compatibility
  pattern: normalize the hard-reference table, preserve measured soft samples.

This does not establish rendered-PDF verification or decode codewords.

### Estimated constellation MER

`constellation_analysis.mer` is calculated in Rust from the complete decoded
capture and exported identically by the CLI and desktop. It contains:

| Field | Meaning |
| --- | --- |
| `mer_db` | `10 log10(signal_power / error_power)`, or null |
| `sample_count` | Original received sample count, including invalid samples |
| `signal_power` | Sum of squared paired reference I/Q amplitudes |
| `error_power` | Sum of squared I/Q differences between received and paired reference symbols |
| `rms_evm_percent` | `100 sqrt(error_power / signal_power)`, or null |
| `calculation_status` | `available`, `perfect_reference`, `no_samples`, `invalid_samples`, `sample_count_mismatch`, `zero_signal_power`, `non_finite_result`, or `unsupported_modulation` |

The source contains only soft decisions. Each received point selects its own
nearest Euclidean symbol from the existing normalized reference table. This
does not pair the two plotted arrays by index. Exact distance ties choose lower
Q, then lower I; this deterministic convention is not modem slicer telemetry.
The signed s2.13 decoder already divides raw amplitudes by 8192; no second
normalization, fitted gain/phase correction, clipping or outlier rejection occurs.
Power sums and calculations use f64. References cover every currently supported
viewer modulation, including cross-shaped QAM tables.

An exact zero error sum has `perfect_reference`, `mer_db: null` and
`rms_evm_percent: 0`; it represents infinite MER without invalid JSON or an
arbitrary finite ceiling. Invalid/unavailable calculations never serialize
non-finite values. Valid power sums remain exposed for zero-signal and
perfect-reference conditions. The header's sample count must agree with the
decoded payload for a numeric MER. This marks the analysis unavailable without
changing the parser's input policy.

This is **decision-directed estimated constellation MER**, not DOCSIS OFDM
per-subcarrier RxMER. Incorrect symbol decisions under strong noise/distortion
can make it optimistic. The file lacks transmitted-symbol identities, so this
estimate cannot validate modem hard decisions or recover true transmitted-symbol
MER. Toggle state, zoom and bounded display sampling never select the samples
used by this calculation. See the [complete CLI JSON](../cli/examples/constellation.json).


### Constellation decision-boundary JSON

`constellation_analysis.decision_rule` is `nearest_reference_euclidean`.
Each known `reference` also contains `decision_boundaries` calculated in the
Rust build. Their coordinates share the normalized units of `reference.iq`:

```json
{
  "origin": [0.0, 0.0],
  "direction": [0.0, 1.0],
  "minimum": null,
  "maximum": null
}
```

This example is the vertical boundary through zero. Position is
`origin + direction × parameter`. Null `minimum` or `maximum` means the line
extends indefinitely in that parameter direction; finite values restrict it to
a ray or segment. Direction is a parameter vector, not necessarily unit length.
The complete model includes cross-QAM diagonals and unbounded outer regions.
The viewer clips these boundaries in Rust for display; no viewport coordinates
replace these descriptors in CLI/desktop capture JSON. Unknown modulation keeps
`reference: null`. Raw captured `iq` remains unchanged. See the
[calculation design](../development/qam-resources.md) and the regenerated
[complete example](../cli/examples/constellation.json).

<a id="current-upstream-pre-equalization-analysis"></a>

## Upstream pre-equalization analysis

PNN6 and PNN7 JSON include `pre_equalization_analysis` through the shared Rust
parser
serializer and `pnm::analysis::upstream_pre_equalization` helper. CLI and desktop
exports are identical. Last-update exports now add this analysis object; their
original fields and s1.14 I/Q values remain unchanged.
The existing borrowed parser and original fields/IQ remain available; requesting
analysis or serializing either variant rejects zero spacing and frequency
overflow.

```text
pre_equalization_analysis: {
  frequencies_hz: [u32, ...],
  magnitude_db: [number | null, ...],
  group_delay: [{frequency_hz: u32, delay_s: number | null}, ...]
}
```

All arrays align with raw `iq` (ordered `[I, Q]`) and retain full resolution.
Frequency is `zero_frequency + (first_active_subcarrier_index + ordinal) × spacing_hz`,
using checked u32 arithmetic. Magnitude is `20 log10(hypot(I, Q))`; no per-file
renormalization is applied. Group delay is `−d(unwrap(arg(I+jQ)))/df / (2π)` in
seconds. Interior differences are central; runs of at least three use
second-order one-sided edges, and two-point runs use their single slope. Isolated
or zero coefficients have null delay. Zero coefficients also have null magnitude.
Phase unwrapping restarts after undefined coefficients. No smoothing, absolute
delay recovery, inferred plant response or echo distance is implied.

Coefficient format and metadata evidence: CM-OSSI 3.1 I27, D.2.13,
Tables 77/78 and D.2.13.13.1–14.1 (pp. 189–192); CM-OSSI 4.0 I13, D.2.13,
Tables 110/111 and D.2.13.13.1–14.1. Current values use s2.13; last updates use
s1.14. The frequency grid follows the pinned PyPNM parser convention. Magnitude,
group-delay calculation and JSON structure are application analysis choices,
not fields stored in the OSSI file. Evidence was checked against versioned text
extracts, without rendered-PDF verification.

[Current output](../cli/examples/upstream-pre-eq.json) ·
[Last-update output](../cli/examples/upstream-pre-eq-last.json) ·
[Viewer controls](../ui/measurements/upstream-pre-eq/README.md)

## RxMER bit-loading analysis

`--rxmer-bit-loading` emits the shared Rust analysis object instead of changing
the raw RxMER parser fields. Qt popup exports use the same serializer. The
[full example](../cli/examples/rxmer-bit-loading.json) retains every selected
original carrier; Qt plotting alone is bounded.

- `model`: policy identifier/revision/source, additional margin and maximum bits.
- `selection`: actual inclusive carrier-center frequencies, original first index,
  count and spacing in Hz. User bounds between carriers select the centers inside.
- `coverage`: measured/unavailable counts, coverage status, and false
  `carrier_roles_verified` (this analysis does not import a profile capture).
- `summary`: minimum RxMER, limiting carrier frequency, uniform bits/modulation,
  model threshold, remaining margin and result status. No measurements or a
  measured carrier below the model minimum produces null uniform order/rate.
- `rate`: `estimated_gross_rate_bps`, `ideal_gross_useful_symbol` basis, explicit
  assumed-data-carrier basis and excluded overheads. Count × bits × spacing uses
  checked arithmetic; it is independent of the center-to-center span.
- `uniform_scenarios`: hypothetical fixed-loading rates for each registry data
  order up to the cap, with meets/below/not-covered/no-measurements statuses.
- `subcarriers`: source index, Hz, nullable measured RxMER, estimated bit loading
  and the separately computed ideal Shannon bit loading.

Policy sources: pinned PyPNM-CMTS-WebUI `25f40bda` selection-insights heuristic;
CM-OSSIv3.1 I27 D.2.11 Table 72 (PDF pp.182–183, versioned text) configurable
candidate-profile defaults; ideal Shannon inverse SNR thresholds. These do not
guarantee receiver BER/FEC performance. Values clipped at 63.5 dB remain measured
lower bounds, and 0xFF remains unavailable. Original capture exports are unchanged.

## Pre-EQ correction comparison

The standalone current/last-update parser schema is unchanged. Optional two-input
analysis exports `analysis_type: upstream_pre_eq_comparison`, `pair`, `mode`,
`predicted: true`, `assumptions`, `alignment_basis`, `summary` and full `points`.
`pair` contains both typed PNM headers, CM and CMTS MAC strings, shared grid
and count, positive `file_time_difference_s`, `precheck_passed: true`,
`update_mode_encoded: false` and `application_status_known: false`.

Each point retains source index/frequency, original/correction/predicted/delta
I/Q, raw original/predicted/delta dB, separately labeled aligned dB, and
original/predicted/delta delay in seconds. Delta I/Q is predicted minus original;
delta dB is the difference of coefficient magnitudes in dB, not the magnitude of
the complex delta. Undefined logs and delay stay null. Full-channel magnitude
spans include tilt; alignment references use mean coefficient power over all
matched captured positions. Neither is a claim of measured RF power.

Validation rejects wrong roles, unsupported/mismatched headers and counts,
nonmatching device/channel/grid, empty data, and equal/older correction times.
Precheck can be exported without a prediction mode. Apply-then-delta and all
summaries are shared Rust code; UI sampling never changes full exports.
