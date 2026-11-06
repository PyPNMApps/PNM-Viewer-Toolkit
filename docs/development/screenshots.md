# Refresh UI screenshots

[Developer index](README.md) · [Measurement gallery](../ui/measurements.md)

Capture the actual native Qt application from the current build. Do not use
mockups, screenshots from PyPNM, or image edits that invent controls. Use only
the preserved repository fixtures, never customer captures.

## Build and capture

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
./scripts/qt-desktop.sh build &&
./scripts/qt-desktop.sh
```

Use a 1280 × 820 application content area for consistent images. A desktop
screenshot tool or an isolated X desktop can capture the real window. Exclude
OS decorations and unrelated windows. Move the pointer away from controls and
wait for file loading, chart rendering and tooltips to settle before capturing.
Do not change the graph data to improve its appearance.

For each theme, choose Settings → System… → Appearance → Dark or Light. Show
the left navigation pane on Home with the top-right panel button. Hide it for
measurement captures to give the charts room. Keep the full reset viewport,
except Spectrum's documented range-tab and power-selection example.

Store PNG files under `docs/images/user-guide/` using the pairs below:

| Filename stem (append `-dark.png` and `-light.png`) | Fixture and view |
| --- | --- |
| `home` | Close capture; show navigation |
| `rxmer` | `rxmer.bin` |
| `spectrum` | `spectrum_analyzer.bin`, 300–350 MHz range tab, WindowAvg N=101 and a selected power band |
| `spectrum-frequency` | Same capture, Frequency dialog with 300000000–350000000 Hz inputs |
| `channel-magnitude` | `channel_estimation.bin`, Magnitude |
| `channel-delay` | Same capture, Group delay |
| `channel-iq` | Same capture, I/Q scatter |
| `histogram` | `histogram.bin` |
| `fec-summary` | `fec_summary.bin`, initial profile |
| `modulation-profile` | `modulation_profile.bin`, All profiles, dB |
| `modulation-bits` | Same capture, All profiles, Bit loading |
| `profile-bandwidth` | Same capture, Profile BW grid open |
| `constellation` | `const_display.bin`, all three layers and lower-right MER/EVM overlay enabled |

Fixtures live in `tests/fixtures/pypnm/tests/files/`. Reload each capture before
capturing if previous selections or zoom could persist. Do not include native
file dialogs in the gallery.

## Review before a tag

- Open both images of every pair and check labels, clipping, theme and graph data.
- Confirm Home shows the navigation button and expanded pane.
- Confirm constellation has no numeric axes or sample-count footer; show MER/EVM
  at lower right and its visibility toggle.
- Confirm Spectrum shows the original and 300–350 MHz tabs, compact range-close
  button, N=101, red average, highlighted power band and dBmV result. The lower
  frequency-selection readout must match the selected power band.
- Confirm modulation images show all four profiles and the correct metric.
- Keep README's Home picture and the gallery's `<picture>` paths consistent.
- Update the user guide when visible controls or supported measurements change.
- Run `./scripts/qt-desktop.sh check` after any UI code changes. A screenshot
  verifies appearance; it does not replace parser, numeric or platform tests.

These images show a local Ubuntu native build. They do not demonstrate Windows,
macOS or packaged installation support. Keep hosted CI evidence separate.
