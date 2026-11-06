# RxMER

[All measurements](../../measurements.md) · [User guide](../../README.md) · [Project README](../../../../README.md)


RxMER in dB against frequency in MHz. Drag to investigate a frequency range; unavailable samples remain gaps.

Example: `rxmer.bin`.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="images/rxmer-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="images/rxmer-light.png">
  <img alt="RxMER chart with channel and MAC chips" src="images/rxmer-dark.png" width="1000">
</picture>


## Bit loading and estimated gross throughput

Zoom the RxMER chart to the frequency range you want, then right-click inside
it and select **Bit loading…**. With no zoom, the popup uses the whole capture.
It analyzes the original subcarriers, even when the main graph shows a sampled
overview. Closing it leaves your capture and zoom unchanged.

The teal stepped line estimates each measured carrier's supported bits per
symbol. The red horizontal line shows the highest **single modulation** that
meets the selected model across all measured carriers in the range. The weakest
carrier limits that line; averaging the good carriers cannot hide a bad one.
Toggle **Shannon** to show the separate ideal upper bound in blue.

Choose a model, add a nonnegative margin in dB if needed, select a maximum QAM,
and press **Recalculate**. The initial cap is 8192-QAM (13 bits/symbol), an
analysis choice rather than proof of modem capability. Results are estimates
from one capture, not a promise of error-free operation or a configured CMTS
profile. Missing readings remain gaps and the popup reports incomplete coverage.

| Model | What it means |
| --- | --- |
| WebUI heuristic (default) | Same 256–4096-QAM MER targets as PyPNM-CMTS-WebUI selection insights |
| OSSI candidate defaults | DOCSIS 3.1 CM-OSSI I27 D.2.11 candidate-profile default MER targets; actual configured targets may differ |
| Ideal Shannon | Ideal AWGN bound using MER as an SNR proxy; not a practical receiver threshold |

The large **Estimated gross throughput** chip assumes the measured carriers
carry data and use the uniform bit loading shown above:

`gross bit/s = measured carriers × uniform bits/symbol × subcarrier spacing (Hz)`

For example, 1,001 carriers at 10 bits/symbol and 25 kHz give **250.25 Mb/s**.
This excludes LDPC/BCH and other FEC redundancy, cyclic-prefix time, pilot/PLC
occupation, and MAC/other PHY overhead. It is not download or application speed.
RxMER alone does not identify every carrier's data/pilot role, so the assumption
stays visible. The range includes both carrier-center endpoints: 650–675 MHz at
25 kHz has 1,001 centers, not 1,000.

Click that chip to open **Uniform rates by modulation**. Each row assumes the
entire measured range uses that one modulation. A hypothetical rate can be
shown even when the model target is not met; the status column distinguishes
**Meets model target**, **Below target**, and **Not covered by model**. The large
chip uses the highest covered order meeting the model and cap, not the largest
hypothetical rate.

**Export JSON** saves the full-resolution popup result, its model/margin/cap,
coverage, per-carrier values and rate scenarios to a new file. This is separate
from the main capture's raw JSON export. The CLI can reproduce the same analysis;
see [CLI options](../../../cli/usage.md#rxmer-bit-loading-analysis).

The native screenshots below use a dragged 649.925–674.900 MHz window with
1,000 measured carriers. Its default-model estimate is 250.00 Mb/s.

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="images/rxmer-bit-loading-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="images/rxmer-bit-loading-light.png">
  <img alt="RxMER visible-range bit-loading popup with per-carrier and uniform traces, model controls and gross throughput" src="images/rxmer-bit-loading-dark.png" width="1000">
</picture>

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="images/rxmer-uniform-rates-dark.png">
  <source media="(prefers-color-scheme: light)" srcset="images/rxmer-uniform-rates-light.png">
  <img alt="Uniform rates by modulation grid, distinguishing modeled support from hypothetical gross rates" src="images/rxmer-uniform-rates-dark.png" width="1000">
</picture>
