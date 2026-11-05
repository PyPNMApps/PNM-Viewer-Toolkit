# Install on Windows

[Installation overview](README.md) · [Ubuntu](ubuntu.md) · [macOS](macos.md)

**Under development.** Windows x64 packaging is configured but automatic releases
currently contain Ubuntu packages only. These instructions apply when a release
actually includes a Windows installer; they are not proof of Windows validation.

## Install and launch

1. Open [GitHub Releases](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases)
   and download `PNM-Viewer-Toolkit-<version>-windows-x64.exe` and `SHA256SUMS`.
2. In PowerShell, run `Get-FileHash .\<installer-filename> -Algorithm SHA256`.
   Compare the result with the matching entry in `SHA256SUMS`.
3. Run the EXE installer and launch PNM Viewer Toolkit from Start.

The default per-user folder is `%LOCALAPPDATA%\Programs\PNM Viewer Toolkit`.
Qt, runtime DLLs, and the CLI are app-local; no Rust or Qt SDK is needed.
The installer is unsigned. Verify the download before approving OS prompts;
do not disable Windows protections. GitHub source archives are not installers.

## CLI only

Extract the release's Windows CLI ZIP. Run `pnm-decode.exe --help` from its
versioned folder. The desktop installer includes a CLI too:

```powershell
& "$env:LOCALAPPDATA\Programs\PNM Viewer Toolkit\pnm-decode.exe" --help
```

The installer does not modify PATH. Add an independently extracted CLI folder
to your user PATH only if desired.

## Update and uninstall

Quit the viewer and run the new EXE using the same installation directory.
Windows updates use manual downloads; Ubuntu's in-app installer does not apply.

To remove it, use Settings → Apps → Installed apps → PNM Viewer Toolkit →
Uninstall, or run `uninstall.exe` in its installation folder. A separately
extracted CLI is independent; delete its extracted folder to remove it.
Keep captures outside installation directories.
