# CLI JSON examples

These are complete outputs from `pnm-decode --stdout`, including every sample
and profile. Arrays have not been shortened or rounded for documentation.
Together the JSON files occupy a few MB. The
[CLI user guide](../usage.md#json-examples-by-file-type) lists all supported types.

## MAC-sanitized capture sources

Inputs come from [the MAC-sanitized PyPNM fixtures](../../../tests/fixtures/pypnm/README.md).
Their [provenance](../../../tests/fixtures/pypnm/provenance.json),
[license](../../../tests/fixtures/pypnm/LICENSE) and
[notice](../../../tests/fixtures/pypnm/NOTICE) apply to the source material.
Generated JSON changes the representation, not ownership of that material.

| Output | Input capture |
| --- | --- |
| [rxmer.json](rxmer.json) | [rxmer.bin](../../../tests/fixtures/pypnm/tests/files/rxmer.bin) |
| [channel-estimate.json](channel-estimate.json) | [channel_estimation.bin](../../../tests/fixtures/pypnm/tests/files/channel_estimation.bin) |
| [constellation.json](constellation.json) | [const_display.bin](../../../tests/fixtures/pypnm/tests/files/const_display.bin) |
| [histogram.json](histogram.json) | [histogram.bin](../../../tests/fixtures/pypnm/tests/files/histogram.bin) |
| [upstream-pre-eq.json](upstream-pre-eq.json) | [us_pre_equalizer_coef.bin](../../../tests/fixtures/pypnm/tests/files/us_pre_equalizer_coef.bin) |
| [upstream-pre-eq-last.json](upstream-pre-eq-last.json) | [us_pre_equalizer_coef_last.bin](../../../tests/fixtures/pypnm/tests/files/us_pre_equalizer_coef_last.bin) |
| [fec-summary.json](fec-summary.json) | [fec_summary.bin](../../../tests/fixtures/pypnm/tests/files/fec_summary.bin) |
| [spectrum.json](spectrum.json) | [spectrum_analyzer.bin](../../../tests/fixtures/pypnm/tests/files/spectrum_analyzer.bin) |
| [modulation-profile.json](modulation-profile.json) | [modulation_profile.bin](../../../tests/fixtures/pypnm/tests/files/modulation_profile.bin) |

To reproduce any row, build the current CLI, then use that row's input:

```bash
./scripts/cargo-local.sh build --release --locked -p pnm-decode
./target/release/pnm-decode tests/fixtures/pypnm/tests/files/modulation_profile.bin --stdout
```

To save a comparison without overwriting checked-in examples, choose a new
output under the ignored `local/` directory:

```bash
./target/release/pnm-decode tests/fixtures/pypnm/tests/files/modulation_profile.bin --output local/modulation-profile-comparison.json
```

## Synthetic examples

| Output | Synthetic input | Meaning |
| --- | --- | --- |
| [latency-synthetic.json](latency-synthetic.json) | [latency-synthetic.hex](latency-synthetic.hex) | One flow, one edge and one snapshot, matching [the latency parser test](../../../crates/core/tests/test_cm_latency_rpt.rs) |
| [symbol-header-synthetic.json](symbol-header-synthetic.json) | [symbol-header-synthetic.hex](symbol-header-synthetic.hex) | PNN1 v1.0 header plus two uninterpreted bytes, matching [the symbol stub test](../../../crates/core/tests/test_cm_symbol_capture.rs) |

The hex files are project-owned synthetic test data, not modem captures.
Latency has no real-capture fixture coverage. Symbol payload decoding remains
unsupported: its example was generated with `--header-only`, so its payload
bytes are intentionally absent from JSON.

From the repository root, convert either hex input using Python's standard
library (development-only), then run the actual CLI:

```bash
python3 -c 'from pathlib import Path; Path("local/latency-synthetic.bin").write_bytes(bytes.fromhex(Path("docs/cli/examples/latency-synthetic.hex").read_text()))'
./target/release/pnm-decode local/latency-synthetic.bin --stdout

python3 -c 'from pathlib import Path; Path("local/symbol-header-synthetic.bin").write_bytes(bytes.fromhex(Path("docs/cli/examples/symbol-header-synthetic.hex").read_text()))'
./target/release/pnm-decode local/symbol-header-synthetic.bin --header-only --stdout
```

Use an existing `local/` directory; all paths above are relative to the checkout
root. The JSON examples come from the same serializer used by normal CLI and
desktop exports. Modulation capacity is an ideal gross useful-symbol estimate
with explicit overhead exclusions; see [the contract](../../reference/formats.md#per-profile-ideal-capacity-analysis).

All example MAC fields, including the synthetic latency example, use
`aa:bb:cc:dd:ee:ff` (`aabb.ccdd.eeff`). Measurement values are unchanged.

Constellation JSON also includes `constellation_analysis.mer`: the full-capture,
Rust-calculated decision-directed MER result and availability status. The desktop
uses the same result. See the [MER contract](../../reference/formats.md#estimated-constellation-mer)
for power sums, null/perfect-reference handling and interpretation limits.
