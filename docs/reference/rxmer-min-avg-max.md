# RxMER temporal statistics

The public Rust analysis lives in
`pnm::pnm::analysis::rxmer::aggregate`; desktop collection import lives in
`viewer::measurements::rxmer::collection`. Binary parser JSON remains unchanged.

## Grouping and order

One group has an exact decoded CM MAC, channel, major/minor format, subcarrier
zero frequency, first active subcarrier index, spacing and sample count.
All carrier centers therefore match; there is no interpolation or grid union.
Sources sort by ascending encoded epoch then source name. Content SHA-256
removes byte-identical captures before aggregation; different bytes at the same
epoch remain distinct observations. A single source is valid.

## Equations

For carrier k, let V_k contain the valid decoded dB readings from the selected
captures and n_k = |V_k|:

```text
min_k = min(V_k)
avg_k = sum(V_k) / n_k
max_k = max(V_k)
frequency_k = zero_frequency_hz + (first_active_subcarrier_index + k) * spacing_hz
```

If n_k is zero, all three statistics are null. The unavailable wire sentinel
never becomes a zero-dB reading. The arithmetic dB mean summarizes logged MER
readings, not a ratio-of-total-signal-to-error-energy MER estimate. No per-file
signal/error energy is available to reconstruct that different statistic.

## JSON

```json
{
  "analysis_type": "rxmer_min_avg_max",
  "average_method": "arithmetic_db_valid_readings",
  "group": {
    "mac_address": "00:ca:00:00:00:01",
    "channel_id": 34,
    "major_version": 1,
    "minor_version": 0,
    "zero_frequency_hz": 600000000,
    "first_active_subcarrier_index": 1600,
    "spacing_hz": 25000,
    "sample_count": 1
  },
  "sources": [
    {"name": "capture-a.bin", "capture_time": 100},
    {"name": "capture-b.bin", "capture_time": 108}
  ],
  "points": [
    {"sample_index": 0, "frequency_hz": 640000000,
     "minimum_db": 40.0, "average_db": 40.125,
     "maximum_db": 40.25, "valid_count": 2}
  ]
}
```

This is an illustrative synthetic schema example, not a lab export.

`RxMerAggregate::analyze_average` passes unrounded f64 readings to
`RxMerBitLoadingAnalysis::calculate_series`, sharing the single-capture model,
threshold, coverage and throughput calculations. The resulting analysis includes
`temporal_average` with method, group, capture count and first/last epochs. This
optional field is absent from ordinary single-capture analysis JSON.

Desktop viewport queries preserve joint extrema and gaps for the three traces
while bounding IPC to at most 1024 carrier positions. Range analysis and export
always use retained full-resolution data. Sources are not rewritten or copied
into a synthetic quarter-dB capture.

See the [user walkthrough](../ui/measurements/rxmer/README.md#min-avg-max).

## Capture catalog hierarchy

Each desktop catalog group also carries a Rust-formatted `tree_label`, such as
`Channel 193 · 827.600 MHz`, and `sources` in ascending encoded epoch
order. Source records contain `name` and `capture_time` only; coefficient arrays
are never copied into tree rows. MACs/channels retain the existing catalog/group
order. Expanded/collapsed state is transient QML navigation state, and the
flattened visible tree uses ListView virtualization. The index-zero frequency is
`zero_frequency_hz / 1e6` for display; it is independent of the first active
index. There is no limit of five channels baked into the collection model.
Archive source names are bounded to 4096 UTF-8 bytes to limit catalog metadata.
