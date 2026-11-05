# Install on Ubuntu

[Installation overview](README.md) · [Windows](windows.md) · [macOS](macos.md)

PNM Viewer Toolkit is **under development**. The current temporary release
workflow builds and tests **Ubuntu 22.04 x64 only**. Ubuntu 24.04/26.04 packaging
and lifecycle checks are preserved for manual runs; this release does not claim
those platforms passed. Download packages from
[GitHub Releases](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases).

## Download and install the latest release

Quit the viewer first. This block downloads both same-version DEBs and their
checksums, verifies the downloads, and installs or upgrades them through APT.
It uses `curl` and `python3` for release lookup; neither Python nor Rust is needed
to run the packaged application. If these download tools are missing, install
with `sudo apt install curl python3`, or download the assets using your browser.

```bash
(
  set -euo pipefail
  umask 022
  release_version="$(curl -fsSL https://api.github.com/repos/PyPNMApps/PNM-Viewer-Toolkit/releases/latest |
    python3 -c 'import json,sys; print(json.load(sys.stdin)["tag_name"].removeprefix("v"))')"
  download_dir="$(mktemp -d /tmp/pnm-viewer-install.XXXXXX)"
  chmod 755 "$download_dir"
  cd "$download_dir"
  release_url="https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases/download/v${release_version}"
  for asset in SHA256SUMS "pnm-decode_${release_version}_amd64.deb" "pnm-viewer-toolkit_${release_version}_amd64.deb"; do
    curl -fL --retry 3 "$release_url/$asset" -o "$asset"
  done
  sha256sum --ignore-missing --check SHA256SUMS
  sudo apt install ./pnm-decode_"${release_version}"_amd64.deb ./pnm-viewer-toolkit_"${release_version}"_amd64.deb
  pnm-decode --version
  printf 'Downloaded packages: %s\n' "$download_dir"
)
```

Downloads remain in the printed temporary directory. The directory is readable
by APT's `_apt` user; no home-directory permissions are changed. Installation
needs administrator approval, and APT may download required system libraries.
No repository checkout or separate Qt SDK is needed. If a newer release lacks
the Ubuntu package pair, the download stops before installation.

Launch from the application menu, or from a terminal in the graphical desktop:

```bash
pnm-viewer
```

Do not run the viewer with sudo. A terminal without DISPLAY or a working display
connection cannot launch the desktop; see [Qt troubleshooting](../development/qt.md).

## CLI only

Download the release's `pnm-decode_<version>_amd64.deb` and `SHA256SUMS`.
Verify its hash and install that single DEB with `sudo apt install ./<filename>`.
Run `pnm-decode --help` or `man pnm-decode`. The viewer package is not required.

## Update

No uninstall is needed: quit the viewer and repeat the install block. Installed
Ubuntu builds with the updater can also use About → Check for updates, then
Download and install update. Approve the OS authentication dialog and restart
after success. Versions 0.1.8/0.1.9 need one manual upgrade to fix their download
directory permissions before testing future in-app updates.

## Uninstall

Remove both the viewer and CLI:

```bash
sudo apt remove pnm-viewer-toolkit pnm-decode
```

To keep the CLI, remove only `pnm-viewer-toolkit`. Your capture files and source
checkout are separate from the installed packages and remain intact.

## Add to Favorites

Open Ubuntu's application grid, search for **PNM Viewer Toolkit**, right-click
its launcher and select **Add to Favorites** (or **Pin to Dash**, depending on
your GNOME version). Dragging an icon from a terminal is not required.

The installed desktop entry is `pnm-viewer-toolkit.desktop`. The viewer sets the
same Qt desktop ID and X11 window class so GNOME can associate a running window
with its launcher. After installing a release with this fix, close all older
viewer windows and reopen it from the application grid before pinning. Remove
any old duplicate favorite through its context menu first. A source-only build
has no installed desktop entry unless you install the desktop package.
