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
artifact. A final job stages an Ubuntu-only draft, verifies uploaded metadata
and downloads the uploaded bytes, then automatically publishes with installation
notes. It refuses to change an already public release.

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
are not releases, and drafts are ignored, so a successful tag alone will not
activate the green Update chip. Automatic publication happens only after all
workflow gates and remote asset checks pass. Installed Ubuntu x64 builds with
the installer feature can install both matching packages after OS authentication.
Older builds need a manual upgrade once to gain that feature.

### Update-directory compatibility

Versions 0.1.8 and 0.1.9 create download directories using tempfile's default
permissions (normally 0755 with umask 022). The elevated installer correctly
rejects those directories with "Update directory must be private and owned by
the authenticated user." New code explicitly creates both download and staging
directories with mode 0700. Do not weaken the ownership/permission check.

An affected installed viewer needs one manual upgrade to a release containing
this fix: download and verify both same-version DEBs, install them together with
apt, and relaunch. Publishing a fixed release cannot change the downloader in an
already installed older viewer. Test in-app installation again against the next
newer release. Wayland/accessibility warnings are separate from this rejection.

### Test from the installed 0.1.6 baseline

1. Keep 0.1.6 installed while the next patch tag builds and publishes automatically.
2. After publication, use **About → Check for updates** in
   0.1.6. Confirm the newer version and green Update button appear.
3. Open **Release notes and downloads**, download the matching CLI and viewer
   DEBs plus checksums, verify them, and install both DEBs together as above.
4. Relaunch normally, without `QT_QPA_PLATFORMTHEME=gtk3`. Verify Open and
   Export use the OS file chooser, About shows the new version, and the update
   check reports the installed version is current. Open a fixture and export JSON.

This baseline test is manual because 0.1.6 predates the in-app installer. For the
next release after the installer is installed, use **Download and install update**
in About. Test download cancellation, canceled administrator approval and
successful paired installation. After success, export any unsaved work and use
**Restart viewer**. Verify both versions and viewing/export after restart.

## Existing successful tags

A rerun of an old tag uses its old workflow, so v0.1.6 will not acquire the
new draft-creation step by rerunning it. Its successful run is 41938409457.
The original package artifact can be downloaded, checksum-verified and attached
to an Ubuntu-only draft without rebuilding or moving that tag. Render notes
with `scripts/release/notes.py --ubuntu-only` while VERSION matches the packages.
Prepared v0.1.6 assets/notes in this checkout are under ignored
`local/release-backfill-v0.1.6/`; they are not distributed in fresh clones.
Creating a draft and publishing it remain distinct actions. Do not overwrite
an already public release to apply the workflow correction.

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
