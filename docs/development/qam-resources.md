# QAM resources, scaling and decision geometry

[Documentation index](../README.md) · [Constellation user guide](../ui/measurements.md#constellation-display)

## Source resources

The repository preserves PyPNM's text tables in
[`crates/core/resources/qam-table/`](../../crates/core/resources/qam-table/README.md).
`provenance.json` records their source revision and SHA-256 hashes; LICENSE and
NOTICE retain attribution. Non-square QAM uses the supplied cross-shaped tables.
The inconsistent 8-QAM table remains preserved but is not compiled or offered
as a DOCSIS data-capture modulation.

## Compilation and scaling

Cargo runs [`crates/core/build.rs`](../../crates/core/build.rs) when building the
core library. The build step:

1. Reads each supported `<order>QAM_table.txt` as integer `[I, Q]` reference points.
2. Reads `ConstellationScalingFactors.txt`, keyed by bits per symbol.
3. Validates point counts, uniqueness and mean symbol energy against the supplied
   energy `Es`: average of `I² + Q²` over the equally probable reference symbols.
4. Multiplies each reference coordinate by `1 / sqrt(Es)`, producing unit mean
   symbol power. For 256-QAM, `Es = 170` and the multiplier is approximately
   `0.07669649888473704`; raw reference coordinate 15 becomes `1.1504474832710556`.
5. Generates normalized point arrays and decision-boundary descriptors as Rust
   source in Cargo's generated `OUT_DIR/qam_references.rs`.
6. Compiles those static arrays into the core library used by the CLI and Qt app.

There is no database or runtime text conversion. Opening a file resolves its
`DsOfdmModulationType` code to a borrowed static table. The generated file stays
under Cargo's ignored build output; users do not install it separately.

Received soft-decision I/Q uses signed s2.13. The existing parser converts encoded
integers to amplitude by their fixed-point scale. The CM has already normalized
the constellation, so the reference-table multiplier is **never applied again**
to these samples. Neither zoom nor layer visibility changes exported samples.

## Decision boundaries

The hard-decision overlay shows ideal reference symbols, not decoded codewords.
Boundaries divide the plane by the nearest reference point in Euclidean distance.
For neighboring square-QAM levels, a boundary lies halfway between them:
`boundary = (first_level + second_level) / 2`. The spacing between adjacent
256-QAM reference levels is approximately `0.15339299776947408`; the boundary
is therefore approximately `0.07669649888473704` from either neighboring level.

Interior square-QAM regions are squares. Outer regions are unbounded; the plot
clips them at its viewport rather than inventing an outer decision threshold.
Cross-QAM regions are not all square: missing corners require diagonal edges.
See [Zhang, Yu and Wei, 2010](https://link.springer.com/article/10.1155/2010/917954).

The Rust build helper uses pinned, **build-only** `delaunator` triangulation to
identify neighboring symbols. It constructs the equidistance bisectors, clips
those lines against each symbol's nearest-neighbor half-planes, removes
zero-length edges, and merges collinear contiguous intervals. References retain
finite segments, rays and infinite lines using nullable parameter limits.
The rule is exported as `nearest_reference_euclidean`; it is an ideal reference
model, not a claim about a particular modem's internal slicer.

During zoom Rust clips the compiled descriptors to the requested I/Q viewport.
QML only maps these endpoints and symbol coordinates to pixels. It does not
compute normalization, nearest neighbors or decision boundaries. All boundaries
are retained independently of display sampling of the soft/hard points.

For this resource set, 16-QAM yields six grid lines and 256-QAM yields 30;
128-QAM yields 22 axis-aligned segments/lines plus four diagonal rays. The
largest supported constellation needs only 254 merged boundaries.

## Validation

Dedicated tests check independent known thresholds, all square/cross geometries,
unit average energy, finite-window clipping and unknown modulation. A separate
brute-force oracle checks boundary positions against every reference symbol;
it does not reuse the triangulation algorithm. CLI and Qt exports use the same
core reference metadata. Native UI checks cover toggles, zoom, reset and themes.
