# Bundled example captures

Desktop installers include nine sanitized PyPNM captures, covering RxMER,
spectrum, channel estimation, histogram, FEC summary, modulation profiles,
constellation, and current/last-update upstream pre-equalization.

Choose **Open** to start in the examples folder. A dialog remembers navigation
for the current application session. Source builds without packaged resources
fall back to the home directory. No download, modem or server is required.

| Platform | Installed folder |
| --- | --- |
| Windows | `C:\Program Files\PNM Viewer Toolkit\share\examples` |
| macOS | `/Applications/PNM Viewer Toolkit.app/Contents/Resources/share/examples` |
| Ubuntu | `/opt/pnm-viewer-toolkit/share/examples` |

Treat installed examples as read-only. Export JSON into Documents or another
user-selected writable folder. CLI-only archives remain small and capture-free.

All example CM/CMTS MAC addresses are `aa:bb:cc:dd:ee:ff`. The examples folder
includes upstream LICENSE/NOTICE, provenance hashes and a file list. These are
modified demonstration fixtures, not live customer measurements. The source
[fixture provenance](../../tests/fixtures/pypnm/README.md) explains their origin.

## US Pre-EQ comparison demo (source checkout)

The [comparison demo](../../tests/fixtures/demos/us-pre-eq-comparison/README.md)
sets aside `original.bin` and `last-update.bin` with a step-by-step walkthrough.
Open the original, then attach the update from **Compare** in the
left gutter. This is the sanitized pair used for the comparison screenshots;
the update timestamp is explicitly synthetic and eight seconds later. Normal
prechecks remain enabled. This additional pair is available in the repository;
it is not yet included in desktop installers.
