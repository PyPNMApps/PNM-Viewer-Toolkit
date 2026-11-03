# Format support and evidence

The Rust core implements the following file-format version **1.0** decoders.
The project remains in the dev phase; no desktop viewer exists yet.

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

## Evidence and compatibility

Layouts were reviewed in the authorized text extracts of
CM-SP-CM-OSSIv3.1-I27-250219 and CM-SP-CM-OSSIv4.0-I13-260429. See
[coverage](../development/specs/coverage.csv), [field provenance](../development/specs/field-provenance.csv)
and [format evidence](../development/specs/measurement-formats.md). These evidence
files are local-only under ignored `development/` and are absent from fresh clones.
No rendered-PDF validation,
latest-issue verification or blanket DOCSIS compliance is claimed.

PyPNM is pinned at `82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9`.
[Unmodified fixtures and licensing](../tests/fixtures/pypnm/README.md) are retained.
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
- Modulation schemes are tagged `range`/`skip` and preserve numeric assignment
  codes, including reserved/unknown values. Unknown scheme layouts are rejected.
- Histogram symmetry is retained as the encoded number: the real fixture uses 2.
  This reader does not guess a boolean mapping or invent bin coordinates.
- Spectrum retains raw ENBW/window parameters without assigning unverified units.
  A partial final segment is valid; missing declared bytes are not.
- Latency labels remain exact 16-byte arrays, bin edges use hundredths of a
  millisecond, and maximum latency uses microseconds. Reserved words are retained.
  JSON emits exact u64 bin counts; future JavaScript IPC must avoid narrowing them
  to JavaScript Number when they exceed its safe integer range.

See [Rust development](rust-development.md) for export commands.
