# Install on Ubuntu

[Installation overview](README.md) · [Windows](windows.md) · [macOS](macos.md)

PNM Viewer Toolkit is **under development**. Release validation targets Ubuntu
22.04, 24.04 and 26.04 on x64, using the same DEBs built on 22.04. Check the
release's assets and workflow results; older releases may cover only 22.04.

## Download and install

1. Open [GitHub Releases](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases).
2. Download both same-version `pnm-decode_<version>_amd64.deb` and
   `pnm-viewer-toolkit_<version>_amd64.deb`, plus `SHA256SUMS`, into Downloads.
3. Quit the viewer. Open a terminal in that download folder, verify the checksums,
   then install the two DEBs using their actual filenames:

```bash
# Replace VERSION with the version you downloaded; keep both packages identical.
sha256sum --ignore-missing --check SHA256SUMS &&
sudo apt install ./pnm-decode_VERSION_amd64.deb ./pnm-viewer-toolkit_VERSION_amd64.deb &&
pnm-decode --version
```

Launch **PNM Viewer Toolkit** from the application menu, or run `pnm-viewer`
from a graphical desktop terminal as your normal user. No repository checkout,
Rust, Python, or separate Qt SDK is needed to run packaged binaries. APT may
download system libraries. An `_apt` sandbox permission notice by itself is
nonfatal; confirm both packages installed successfully.

<details>
<summary>Optional: download and install the latest release entirely from a terminal</summary>

This method needs `curl` and `python3` for release lookup. It downloads into a
fresh APT-readable temporary directory, verifies hashes, then installs both DEBs.
Installation starts only after every download and checksum check succeeds.

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

The printed directory contains the downloads. If the latest release lacks the
Ubuntu package pair, the command stops before installation. It changes no
home-directory permissions.

</details>

## CLI only

Download the release's `pnm-decode_<version>_amd64.deb` and `SHA256SUMS`.
Verify its hash and install that single DEB with `sudo apt install ./<filename>`.
Run `pnm-decode --help` or `man pnm-decode`. The viewer package is not required.

## Update

No uninstall is needed: quit the viewer and install both new-version DEBs using the same steps. Installed
Ubuntu builds with the updater can also use About → Check for updates, then
Install update and restart. Export work before starting and approve the OS
authentication dialog. After success, the viewer restarts automatically once any
active capture/export work finishes. If launching fails, the existing window
remains open with a Restart viewer retry button. Older builds with the manual
restart flow require that button after installation. Versions 0.1.8/0.1.9 need one manual upgrade to fix their download
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
