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
