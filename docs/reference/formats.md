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
  to six decimals. No derived bandwidth, statistics or analysis is added.
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

This does not establish rendered-PDF verification, decode codewords or estimate MER.
