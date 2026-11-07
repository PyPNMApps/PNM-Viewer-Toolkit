# Install on Windows

[Installation overview](README.md) · [Ubuntu](ubuntu.md) · [macOS](macos.md)

**Under development.** Use the **Windows x64 MSI** from a published release with
Windows assets. Windows 11 x64 is the desktop target; hosted lifecycle tests run
on Windows Server 2022, which does not prove interactive Windows 11 approval UI.

## Install and launch

1. Open [GitHub Releases](https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases).
   Download `PNM-Viewer-Toolkit-<version>-windows-x64.msi` and `SHA256SUMS`.
2. Run `Get-FileHash .\<msi-filename> -Algorithm SHA256` in PowerShell and compare
   the matching checksum entry. Use the project release assets, not source ZIPs.
3. Double-click the MSI and follow Windows Installer's prompts, including UAC.
   On **Desktop shortcut**, select **Create a desktop shortcut** if wanted.
   The checkbox starts unchecked; Start-menu integration is always installed.
4. Launch **PNM Viewer Toolkit** from Start or the selected desktop shortcut
   as your normal user. The installer and shortcuts use the navy/teal app icon.

The default installation directory is `C:\Program Files\PNM Viewer Toolkit`.
Qt, the MSVC runtime DLLs, CLI, manual and notices are bundled app-local.
The application does not require an administrator account to run or modify PATH.
The first Open dialog starts in the bundled [example captures](../ui/examples.md)
under `share\examples` in the application directory. Navigation is retained for
the current viewer session; export JSON to your own writable folder.

For an unattended install, `CREATE_DESKTOP_SHORTCUT=1` opts in; omit it or use
`CREATE_DESKTOP_SHORTCUT=0` to opt out. Choose again during an interactive upgrade.
The installer removes its selected shortcut on uninstall.

## Windows 11 security prompts or an MSI that does not open

These development MSIs are unsigned. A matching SHA-256 checks downloaded bytes;
it does not provide a verified Authenticode publisher. Confirm the project and
release before approving OS prompts. If double-click does nothing, try
right-click → Install, or start Windows Installer with logging from PowerShell:

```powershell
$msi = Get-ChildItem "$env:USERPROFILE\Downloads\PNM-Viewer-Toolkit-*-windows-x64.msi" |
    Sort-Object LastWriteTime -Descending | Select-Object -First 1
if (!$msi) { throw 'Download the Windows x64 MSI first.' }
$log = "$env:TEMP\pnm-viewer-install.log"
Write-Host "Installing: $($msi.FullName)"
$result = Start-Process "$env:SystemRoot\System32\msiexec.exe" -ArgumentList "/i `"$($msi.FullName)`" /norestart /L*V `"$log`"" -Wait -PassThru
Write-Host "Installer exit code: $($result.ExitCode); log: $log"
```

Selection is by download modification time; check the printed filename and its
checksum. Exit code 0 means success; 3010 means success with an OS reboot needed.
The command keeps Windows security checks enabled and does not reboot the PC.
If SmartScreen, Smart App Control, Defender, or company policy blocks installation,
use the applicable OS approval process or contact your administrator. Do not
disable protections, change execution policy, or blanket-unblock downloads.
Read logs around `Return value 3` on failure; remove personal information before
sharing them. See [Microsoft's installer options](https://learn.microsoft.com/en-us/windows/win32/msi/standard-installer-command-line-options).

The viewer's Open and Export actions use native Windows Open/Save dialogs.
The package includes `qwindows.dll`; smoke tests reject forcing QML fallback.
Interactive Explorer/dialog and security-prompt behavior still needs Windows 11 testing.

## Update

Quit the viewer and install the new MSI. Its major-upgrade rules replace an older
MSI from this product family and refuse downgrades. Windows in-app installation
and MSP patches are deferred; About links to release downloads. A legacy EXE/NSIS
installation must be uninstalled through Installed apps before the first MSI.

The **Update** button opens About; **Check for updates** checks the available
version. Neither installs a Windows update. Choose **Release notes and downloads**,
download the Windows MSI, close every running viewer window, then run the MSI
and relaunch. Uninstalling a previous MSI first is normally unnecessary. About
shows the version of the running process, which stays unchanged until relaunch.

## CLI only

Extract the Windows CLI ZIP and run `pnm-decode.exe --help` from its versioned
folder. Or use the copy bundled with the viewer:

```powershell
& "$env:ProgramFiles\PNM Viewer Toolkit\pnm-decode.exe" --help
```

## Uninstall

In Windows builds, **About → Uninstall…** opens Windows **Installed apps**.
Close the viewer, select **PNM Viewer Toolkit**, then choose **Uninstall**.
You can also open this page directly:

Settings → Apps → Installed apps → PNM Viewer Toolkit → Uninstall.
Windows Installer owns removal; no separate `uninstall.exe` is installed.
A separately extracted CLI is independent. Keep captures outside installation
folders; removing the viewer does not remove captures in your own folders.
