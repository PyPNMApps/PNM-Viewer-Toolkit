# US Pre-EQ comparison demo

A ready-to-open, sanitized pair used for the comparison screenshots. No modem,
network connection or archive extraction is needed. Both CM and CMTS addresses
are `aa:bb:cc:dd:ee:ff`. Each file contains 1776 coefficients.

1. Open **original.bin** in the Qt viewer.
2. Show the left gutter and select **Correction & Delta**.
3. Drop **last-update.bin** into **Drop Last Update here**, or use
   **Choose Last Update…**. Wait for the mode selector to become available.
4. Select **Adjust (multiply)** and **Compare**.
5. Select **Level-aligned flatness** to see the small shape change. Try
   Magnitude, Group delay and I/Q; drag on the line graph to zoom.

The original span is about **2.51 dB**, predicted span **2.48 dB**. The
level-aligned magnitude delta is approximately **−0.04 to +0.05 dB**.
These are coefficient-derived predictions, not measured transmit power.

The update timestamp is intentionally changed from the source fixture to be
**eight seconds later** than the original. All other source bytes are unchanged.
Adjust is a demonstration assumption; these files do not prove a real update
sequence or encode the modem's application mode. Normal strict prechecks apply;
there is no demo bypass. The untouched PyPNM pair has equal timestamps and will
fail the newer-timestamp requirement.

From the repository root, with `pnm-decode` installed:

```bash
pnm-decode tests/fixtures/demos/us-pre-eq-comparison/original.bin \
  --pre-eq-update tests/fixtures/demos/us-pre-eq-comparison/last-update.bin \
  --precheck-only --stdout
pnm-decode tests/fixtures/demos/us-pre-eq-comparison/original.bin \
  --pre-eq-update tests/fixtures/demos/us-pre-eq-comparison/last-update.bin \
  --update-mode adjust --stdout
```

A source build can use `./target/debug/pnm-decode` instead. Export to a new file
in your own writable folder when desired. This demo is available in the source
checkout; current installers still include only their existing nine captures.

See [the user guide](../../../../docs/ui/measurements/upstream-pre-eq/README.md#correction--delta-from-the-left-gutter).
[provenance.json](provenance.json) records source/current hashes and the exact
modification. [Upstream provenance](../../pypnm/provenance.json), LICENSE and
NOTICE retain PyPNM attribution and terms.
