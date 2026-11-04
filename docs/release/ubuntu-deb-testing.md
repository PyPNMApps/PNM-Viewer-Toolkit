# Temporary Ubuntu 22.04 DEB testing

[CI scope](../development/ci.md#temporary-ubuntu-only-delivery) ·
[Packaging](packaging.md) · [Version/tag commands](../development/releasing.md)

The project is in development. This temporary workflow runs only when a `v*`
tag is pushed and builds/tests **Ubuntu 22.04 x64** packages. It preserves the
full multi-OS workflows for explicit manual runs; it does not make their current
Windows/macOS failures pass or certify Ubuntu 24.04/26.04 packages.

## Get the packages

After pushing a normal version-matched tag, open **Actions → Ubuntu 22.04 DEB
(temporary)** and select its successful run. Download the
`ubuntu-22.04-deb-vMAJOR.MINOR.PATCH` artifact, available for 30 days.
Extract it into a fresh directory, keeping one version per directory. It includes:

- `pnm-decode_VERSION_amd64.deb` — CLI/manual.
- `pnm-viewer-toolkit_VERSION_amd64.deb` — viewer, with a matching CLI dependency.
- Existing CLI archive, SHA256SUMS, SBOM and build/dependency metadata.

The job enforces version/master ancestry, CodeQL/static checks, Rust and Qt/QML
checks, package sizes/checksums and installed viewing/export/removal checks on
Ubuntu 22.04. Failed builds retain diagnostics rather than a successful package
artifact. The workflow does not publish or modify a GitHub release.

## Install or manually upgrade

On a disposable Ubuntu 22.04 x64 test VM, open a terminal in the extracted
artifact directory. Verify its bytes, then install both packages together:

```bash
sha256sum --check SHA256SUMS
sudo apt install ./pnm-decode_*_amd64.deb ./pnm-viewer-toolkit_*_amd64.deb
pnm-decode --version
pnm-viewer
```

For upgrade testing, first install an older successful artifact, then repeat
these commands from a separate directory containing the newer successful tag's
artifact. Check the CLI version and **About** version, open a known capture,
and export JSON. Confirm existing captures and exported files are preserved.
A newer candidate alone is not proof of an older-to-newer upgrade: the workflow's
current lifecycle test installs and removes one version, without a previous
baseline package. Record the actual installed versions and results separately.

The app's update check reads published stable GitHub releases. Actions artifacts
are not releases, so these builds will not activate the green Update chip merely
because a tag succeeded. Download/install manually during this temporary phase.

## Restore full delivery

1. Resolve and validate the Windows/macOS QML failures.
2. Restore the `push: tags: ['v*']` trigger in release.yml.
3. Restore automatic full-native CI scheduling and strict all-platform aggregation.
4. Remove ubuntu-deb-temp.yml so one tag does not trigger two delivery paths.
5. Update the README badge and temporary notices in documentation/AGENTS.md.

Until then, a green automatic **Required checks** status covers CodeQL/static
checks only; the temporary tag job separately requires the Ubuntu 22.04 gate and
package lifecycle. Full CI and full packages remain available through manual
workflow runs, with the native matrix enabled and a matching tag for packaging.
