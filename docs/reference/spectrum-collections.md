# Standard spectrum temporal analysis

Spectrum collections accept the existing standard PNN9 format and use the same
Rust decoder/frequency convention as single-file viewing. PyPNM-specific SNMP
amplitude payloads are excluded. Metadata comes exclusively from binary headers.

Compatibility compares every encoded acquisition field: MAC, channel, format
major/minor, first/last segment centers, segment span, bins per segment, sample
count, encoded equivalent-noise-bandwidth factor and window function. Identical
rounded range/RBW labels do not establish compatibility. Bins are never stretched,
interpolated or stitched across incompatible captures. Groups sort by MAC/settings,
then captures sort by numeric epoch and source label for ties. Exact file-byte
copies are deduplicated; distinct captures at one epoch are retained.

At each bin, minimum and maximum are extrema of finite readings. The temporal
average is equal-weight mean-square voltage expressed in dBmV:

```text
average_dbmv = 10 log10(sum(10^(level_dbmv / 10)) / valid_capture_count)
```

For example, 0 and 10 dBmV average to approximately **7.4036268949 dBmV**.
The implementation subtracts the maximum before exponentiation for stability.
The shared 75-ohm reference cancels; there is no added impedance or dBm conversion.
Invalid readings remain gaps, counts are explicit, and zero valid readings produce
null values. RxMER's separate arithmetic-dB temporal average is unchanged.

`SpectrumAggregate` in `crates/core/src/pnm/analysis/spectrum/` owns the unrounded
result. JSON contains `analysis_type: spectrum_min_avg_max`, `average_method:
linear_power`, `value_units: dBmV`, `capture_count`, `first_epoch`,
`last_epoch`, the exact `group`, ordered `sources`, and
`points` with `sample_index`, `frequency_hz`, `minimum_dbmv`, `average_dbmv`,
`maximum_dbmv`, and `valid_count`. Playback uses the same representation for one
original frame; minimum/average/maximum coincide. No synthetic PNM header is
created for an aggregate and no averaged samples are quantized back into i16.

The desktop collection service lives under
`apps/viewer/src/measurements/spectrum_analysis/collection/`. Shared archive
traversal, joint-extrema decimation and no-clobber export live under
`apps/viewer/src/capture/collection/`. The Qt bridge owns only orchestration and
bounded presentation state. Import limits/cancellation and stale-worker rejection
follow the existing collection contract. Defaults remain 4096 entries, nesting
8, 64 MiB/file, 256 MiB expanded reads and four million decoded samples.

WindowAvg is a second, independent linear-power average across frequency, applied
to the temporal mean or current frame. Acquisition RBW stays unchanged. Derived
range tabs retain source-bin bounds and independent zoom/smoothing state. Power
integrates the unrounded underlying trace with the existing
[ENBW-corrected dBmV formula](../development/spectrum-power.md), not the smoothed
or reduced display data. Min/mean/max share decimation positions that preserve
extrema/gaps; JSON exports retain every original bin.

Tests cover independently known averages, metadata mismatches, invalid grids,
missing data, duplicate content, header chronology, nonstandard-format exclusion,
archive limits/cancellation, tabs, power, playback and full-resolution exports.
The supplied real expanded inputs contained one unique standard spectrum capture;
the included temporal demo is explicitly synthetic.
