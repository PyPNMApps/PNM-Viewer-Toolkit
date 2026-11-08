# Install on macOS

[Installation overview](README.md) · [Ubuntu](ubuntu.md) · [Windows](windows.md)

**Under development.** Choose a release containing the matching Apple Silicon
or Intel DMG. The release is published only after both native architecture jobs
pass; older Ubuntu-only releases have no macOS installer. Signing is ad-hoc,
not Apple notarization or a verified Developer ID.

## Install and launch

1. Open [GitHub Releases](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases).
   Choose `PNM-Viewer-Toolkit-<version>-macos-arm64.dmg` for Apple Silicon or
   `PNM-Viewer-Toolkit-<version>-macos-x64.dmg` for Intel, plus `SHA256SUMS`.
2. Run `shasum -a 256 <dmg-filename>` and compare the matching checksum entry.
3. Open the DMG, drag PNM Viewer Toolkit onto Applications, and eject the image.
4. Launch the app from Applications.

The app uses the branded PNM icon in Finder and the Dock. For a desktop shortcut,
select the installed app in Finder, choose **File → Make Alias**, and move the
alias to Desktop. The DMG does not silently modify your desktop.

Capture, archive and folder selection starts in the bundled [demo files](../ui/examples.md)
at `PNM Viewer Toolkit.app/Contents/Resources/files/demo`. Navigation is
retained for the current viewer session. Export JSON to your own writable folder.

The deployment target is macOS 13 or newer; native CI uses macOS 15 runners.
Qt and the CLI are included. The app is ad-hoc signed and not notarized.
If macOS blocks launch, verify its source/checksum and use the operating system's
application-approval controls. Do not disable system protections globally.

## CLI only

Extract the matching Darwin arm64 or x64 CLI `.tar.gz` and run `pnm-decode --help`
from its versioned folder. The desktop application's CLI is also available:

```bash
"/Applications/PNM Viewer Toolkit.app/Contents/MacOS/pnm-decode" --help
```

The standalone archive includes the manual and license notices.

## Update and uninstall

In **About → Software**, choose **Download DMG** for the latest published stable
release. The link selects Intel or Apple Silicon for the running build. Choose
**Release notes** to view the release page. Quit the app and replace it in
Applications using the downloaded DMG. macOS updates remain manual.

To uninstall, quit the app and move it from Applications to Trash. Delete the
extracted folder of any independently installed CLI separately. Keep capture
files outside the application bundle.
