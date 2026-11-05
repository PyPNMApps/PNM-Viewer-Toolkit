# PNM Viewer Toolkit {{TAG}} — Ubuntu 22.04 development build

This temporary release contains **Ubuntu 22.04 x64 packages only**. The project
is under development. Other platforms are not validated by this workflow; their
full workflows are preserved for manual execution. The workflow uploads to a temporary draft, verifies the uploaded metadata and
downloaded bytes, then automatically publishes the release for update discovery.

## Downloads

{{ASSET_TABLE}}

`SHA256SUMS`, license/SBOM and build/dependency metadata are attached alongside
the packages. Verify downloaded package hashes before installation.

## Ubuntu 22.04 x64 — install or upgrade

[Ubuntu installation and uninstall guide]({{SOURCE_URL}}/docs/install/ubuntu.md)

Download both same-version DEBs and `SHA256SUMS` from the table above. Quit the
viewer and use the APT commands below from your Downloads folder.

<details>
<summary>Optional: download and install this release from a terminal</summary>

```bash
(
  set -euo pipefail
  umask 022
  download_dir="$(mktemp -d /tmp/pnm-viewer-install.XXXXXX)"
  chmod 755 "$download_dir"
  cd "$download_dir"
  release_url="https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases/download/{{TAG}}"
  for asset in SHA256SUMS pnm-decode_{{VERSION}}_amd64.deb pnm-viewer-toolkit_{{VERSION}}_amd64.deb; do
    curl -fL --retry 3 "$release_url/$asset" -o "$asset"
  done
  sha256sum --ignore-missing --check SHA256SUMS
  sudo apt install ./pnm-decode_{{VERSION}}_amd64.deb ./pnm-viewer-toolkit_{{VERSION}}_amd64.deb
  pnm-decode --version
)
```

Launch `pnm-viewer` from a graphical desktop terminal or the application menu.
</details>

The download command uses `curl`; downloaded packages remain under the generated
`/tmp/pnm-viewer-install.*` directory. To install files already downloaded using
your browser, open a terminal in their directory and run:

```bash
sha256sum --ignore-missing --check SHA256SUMS &&
sudo apt install ./pnm-decode_{{VERSION}}_amd64.deb ./pnm-viewer-toolkit_{{VERSION}}_amd64.deb
pnm-decode --version
pnm-viewer
```

The viewer requires the matching CLI package. `apt install` also performs a
manual upgrade when an older package is installed. Confirm the new version in
About, open a known capture and export JSON. Do not treat an install/remove smoke
test as proof of an upgrade between two different package versions.

Ubuntu system-library dependencies may require internet access during setup;
local capture viewing works offline afterward. Installation on a pristine host
without network access is not claimed.

## Uninstall

```bash
sudo apt remove pnm-viewer-toolkit pnm-decode
```

Remove only `pnm-viewer-toolkit` to keep the CLI. Capture files remain intact.

## Windows and macOS

This Ubuntu-only release does not include Windows or macOS installers. See the
[Windows guide]({{SOURCE_URL}}/docs/install/windows.md) and
[macOS guide]({{SOURCE_URL}}/docs/install/macos.md) for configured installation
steps when those assets are available in a future multi-OS release.

## Validation and use

This build passed its Ubuntu 22.04 Rust/CLI/tooling/Qt/QML checks, package
size/checksum gates, and installed open/render/export/removal checks. It does not
claim validation on Ubuntu 24.04/26.04, Windows or macOS.

- [User guide]({{SOURCE_URL}}/docs/ui/README.md)
- [Measurement gallery]({{SOURCE_URL}}/docs/ui/measurements.md)
- [CLI usage]({{SOURCE_URL}}/docs/cli/usage.md)
- [Build and test runs]({{RUNS_URL}})

The application's Update chip checks published stable releases. Drafts are not
announced. Installed Ubuntu x64 builds with the installer feature download and
verify both DEBs, request OS administrator approval, then restart automatically
after installation and any active capture/export work finish. Export work before
starting. Older installed versions may still require clicking Restart viewer.
Older builds such as 0.1.6/0.1.7 need one manual upgrade to gain this feature. Reusable core
libraries are Apache-2.0; the desktop application is GPL-3.0-only.
