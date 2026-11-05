# PNM Viewer Toolkit {{TAG}} — Ubuntu 22.04 development build

This temporary release contains **Ubuntu 22.04 x64 packages only**. The project
is under development. Other platforms are not validated by this workflow; their
full workflows are preserved for manual execution. The workflow uploads to a temporary draft, verifies the uploaded metadata and
downloaded bytes, then automatically publishes the release for update discovery.

## Downloads

{{ASSET_TABLE}}

`SHA256SUMS`, license/SBOM and build/dependency metadata are attached alongside
the packages. Verify downloaded package hashes before installation.

## Install or manually upgrade

[Installation and uninstall guide]({{SOURCE_URL}}/docs/install/README.md)

Download both DEBs into a fresh directory, then run there:

```bash
sha256sum --ignore-missing --check SHA256SUMS
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
verify both DEBs, request OS administrator approval, and offer an explicit restart.
Older builds such as 0.1.6/0.1.7 need one manual upgrade to gain this feature. Reusable core
libraries are Apache-2.0; the desktop application is GPL-3.0-only.
