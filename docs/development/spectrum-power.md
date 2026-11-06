# Spectrum range integration in dBmV

The native Spectrum Analyzer has two different operations: **WindowAvg** smooths
across frequency; **Calculate power…** integrates a selected frequency band.
All numerical work runs in Rust. The result is an integrated RMS voltage level
in **dBmV**, displayed to one decimal place. The viewer does not convert to dBm.

## Voltage, mean-square voltage and 75 Ω

A bin level is defined by RMS voltage:

\[
L_i = 20\log_{10}(V_i / 1\text{ mV}),\qquad
V_i = 10^{L_i/20}\text{ mV}.
\]

Power at a fixed resistance is proportional to squared RMS voltage:

\[
P_i=V_i^2/R,\qquad (V_i/1\text{ mV})^2=10^{L_i/10}.
\]

Consequently, integration adds **mean-square voltage**, not coherent amplitudes.
Use a factor of 10 with the logarithm of that sum. Equivalently, take its square
root and use a factor of 20. The fixed resistance cancels when the result is
expressed in dBmV, including a 75 Ω cable system. There is no impedance offset
in this calculation. A dBmV result is a voltage level corresponding to integrated
power at a fixed impedance; it is not a dBm value.

Reference: [Analog Devices: CATV dBm, dBmV and dBµV conversions](https://www.analog.com/en/resources/technical-articles/2022/07/16/07/23/guide-to-rf-tuners-and-dbm-to-dbuv-conversion.html).

## ENBW-corrected integration

Let Δf be the original source-bin spacing and B_ENBW the acquisition filter's
equivalent noise bandwidth, both in Hz. The existing spectrum helper derives:

\[
\Delta f=\frac{\text{segment span}}{\text{bins per segment}},\qquad
B_{\mathrm{ENBW}}=\Delta f\frac{\text{encoded ENBW}}{100}.
\]

Treat each bin as a constant mean-square spectral-density cell centered at f_i.
For the requested band [f_start, f_stop], the contributing width is:

\[
w_i=\max\left(0,
\min(f_{\mathrm{stop}},f_i+\Delta f/2)
-\max(f_{\mathrm{start}},f_i-\Delta f/2)\right).
\]

The integrated result is:

\[
L_{\mathrm{band,dBmV}}=
10\log_{10}\left(\sum_i10^{L_i/10}\frac{w_i}{B_{\mathrm{ENBW}}}\right).
\]

For complete bins, w_i = Δf. The overlap handles fractional boundary bins without
snapping the integration band to bin centers. This is a bin-cell integration
approximation, not recovery of spectral detail finer than the acquisition ENBW.
It assumes the supplied levels represent calibrated mean-square bin measurements;
unknown detector/calibration behavior cannot be corrected from this file alone.

Reference: [Rohde & Schwarz FSQ operating manual, channel-power integration](https://scdn.rohde-schwarz.com/ur/pws/dl_downloads/dl_common_library/dl_manuals/dl_user_manual/FSQ_OperatingManual_en_02.pdf).
The integration method is an analysis convention, not a new DOCSIS wire field.

## Numerical behavior and examples

`common::signal_processing::spectrum_power::integrated_level_dbmv` uses the exact
fractional-Hz source grid derived from the capture header, rather than the rounded
integer-Hz display positions. Integration uses all original bins, independently
of viewport decimation and the optional averaged trace. Missing/nonfinite bins
with nonzero overlap invalidate the result; gaps outside the selection do not.

Internally define a_i = L_i + 10 log10(w_i / B_ENBW), then calculate
`max(a) + 10 log10(sum(10^((a_i - max(a))/10)))`. The implementation separates
logarithms of the bandwidths to avoid forming an overflowing ratio. This prevents
large positive/negative dBmV levels from overflowing/underflowing linear powers.

Independent synthetic references:

| Spectrum and selected width | ENBW | Integrated level |
| --- | --- | --- |
| Flat 0 dBmV, 20 Hz | 10 Hz | 3.0103 dBmV |
| Flat 0 dBmV, 15 Hz | 10 Hz | 1.7609 dBmV |
| Flat 0 dBmV, 20 Hz | 20 Hz | 0 dBmV |
| Flat 0 dBmV, 6 Hz | 10 Hz | −2.2185 dBmV |

The reusable helper accepts ranges inside the source bin-cell coverage. The viewer
uses the stricter displayed first/last center bounds for user input; truncated
captures use their actual last center rather than the configured last segment.
No silent clamping or out-of-capture extrapolation is applied.

## Range tabs and ownership

- `apps/viewer/src/measurements/spectrum_analysis/range.rs` owns unit conversion,
  bounds validation, source selection and the integrated result DTO.
- `apps/qt/src/backend/spectrum.rs` owns bounded tab state over the existing
  `Arc<Capture>`; no capture bytes are duplicated. Maximum 16 tabs including the
  original. Each stores a bounded viewport, range, averaging mode and result.
- `SpectrumTools.qml` renders tabs and the Frequency dialog and supplies formatted
  result-chip values. `SpectrumPowerChip.qml` presents the integrated level,
  frequency range and bandwidth as larger two-line chips in the metadata Flow,
  beside MAC/RBW. The Flow wraps chips at narrow window widths.
  `MeasurementChart.qml` handles the context menu and drag highlight.
- Frequency units are Hz, kHz, MHz or GHz; Rust validates again after the dialog.
  Start must precede Stop. A new graph requires at least two source bin centers;
  integration can use a narrower nonzero band inside a source bin.
- New tabs select original centers within the requested band. Zoom/reset stays
  within those centers. Each tab restores its zoom, averaging window, N input
  and visibility choice. Averaging uses the full capture before selecting the
  range, retaining the existing full-capture reflection convention.
- Closing/replacing the capture clears every tab. Invalid requests retain the
  previous state. The original spectrum tab cannot be closed independently.
- Original capture JSON exports remain unchanged. Band results are transient
  viewer analysis; no new CLI option or persisted capture field is introduced.

The Rust `SpectrumBand` result contains `start_hz`, `stop_hz`, `bandwidth_hz`,
`level_dbmv`, `first_sample` and `sample_count`. Frequency-tab validation leaves
`level_dbmv` null; an integration request supplies a finite numeric result or a
structured diagnostic. The Qt adapter publishes the active result as the small
workspace `power` object, without rewriting the capture's exported JSON.

## Validation

Independent integration tests cover linear-power addition versus coherent
amplitude addition, fractional overlap, ENBW scaling, extreme values and invalid
or missing inputs. Viewer tests cover all four units, actual truncated bounds,
source selection, sub-bin integration and unchanged capture JSON. QML tests cover
the actual right-click menu, power drag versus zoom, dialog input validation and
dBmV presentation. Native Qt checks supplement these component tests.
