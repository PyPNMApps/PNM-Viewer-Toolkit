# Bundled example captures

Desktop installers include single captures, RxMER collections and an upstream
pre-EQ comparison pair. No download, modem or server is required.

Choose **Open** to start in the examples folder. A dialog remembers navigation
for the current application session. Source builds without packaged resources
fall back to the home directory.

```text
examples/
├── README.md
├── manifest.json
├── single/
│   ├── rxmer.bin
│   ├── spectrum_analyzer.bin
│   ├── channel_estimation.bin
│   ├── histogram.bin
│   ├── fec_summary.bin
│   ├── modulation_profile.bin
│   ├── const_display.bin
│   ├── us_pre_equalizer_coef.bin
│   ├── us_pre_equalizer_coef_last.bin
│   └── README.md, LICENSE, NOTICE, provenance.json
├── rxmer/
│   ├── single-channel.zip
│   ├── multiple-channels.zip
│   └── multiple-modems.zip
└── us-pre-eq/
    └── comparison/
        ├── original.bin
        ├── last-update.bin
        └── README.md, LICENSE, NOTICE, provenance.json
```

| Platform | Installed folder |
| --- | --- |
| Windows | `C:\Program Files\PNM Viewer Toolkit\share\examples` |
| macOS | `/Applications/PNM Viewer Toolkit.app/Contents/Resources/share/examples` |
| Ubuntu | `/opt/pnm-viewer-toolkit/share/examples` |

Treat installed examples as read-only. Export JSON into Documents or another
user-selected writable folder. CLI-only archives remain capture-free.

## Single captures

Open a `.bin` from `single/` using the main **Open** action. These nine sanitized
PyPNM captures cover each supported measurement view. Single CM/CMTS MACs use
`00:ca:00:00:00:01`. LICENSE/NOTICE and provenance retain upstream attribution
and hashes. See the [source provenance](../../tests/fixtures/pypnm/README.md).

## RxMER collections

Select **RxMER → Min-Avg-Max** or **Playback** in the left navigation pane, then
open a ZIP directly. The viewer reads the archive; manual extraction is optional.

| ZIP | Captures | What to try |
| --- | --- | --- |
| `single-channel.zip` | 56 | One modem/channel over time |
| `multiple-channels.zip` | 279 | One modem across five channels |
| `multiple-modems.zip` | 32 | Select between two modems, then choose a channel |

These are sanitized, user-provided lab captures. Distinct modems keep sequential
aliases beginning with `00:ca:00:00:00:01`. The viewer groups by binary headers
and orders capture times, independently of filenames. The same collections work
for [Min-Avg-Max](measurements/rxmer/README.md#min-avg-max) and
[Playback](measurements/rxmer/playback.md).

## US Pre-EQ comparison

Open `us-pre-eq/comparison/original.bin`, choose **Compare** in the left gutter,
then attach `last-update.bin`. The pair's update timestamp is explicitly synthetic
and eight seconds later; normal prechecks remain enabled. The included README
provides a walkthrough. See the [comparison guide](measurements/upstream-pre-eq/README.md)
and [source pair](../../tests/fixtures/demos/us-pre-eq-comparison/README.md).

Demo ZIPs are generated at installer build time after expanded-source hygiene
and checksum checks pass. No prebuilt demo ZIP is used as a packaging source.
