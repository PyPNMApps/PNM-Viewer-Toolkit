# Packaging requirements

Targets: Windows x64 NSIS (MSI if offered), separate macOS Intel/Apple Silicon
DMGs, Ubuntu x64 deb (AppImage if offered). All offered primary packages must be
at most 50,000,000 bytes including bundled prerequisites and assets. Gate measured
artifacts; report installer/executable bytes, installed footprint and external
prerequisites separately. No package has been built or measured yet.

Pin native toolchains/lockfiles and a documented Ubuntu baseline. Check current
Tauri configuration documentation before implementation. Configure/test WebView2
policy explicitly; offline viewing after setup is required, but pristine-host
offline installation is a distinct promise. Do not hide runtime downloads or
exempt oversized formats. Report size attribution and resolve scope if exceeded.

Use native CI runners and installed-package smoke tests; cross-compilation does
not establish runtime support. Produce local artifacts/checksums, license inventory,
SBOM and source-build instructions. Reusable crates are Apache-2.0; desktop/UI
are GPL-3.0-only with corresponding-source obligations. The root [LICENSE](../../LICENSE)
records component scope; full texts are under `LICENSES/`. Include the applicable
texts and NOTICE content in distributables. Do not bundle PDFs, private
captures, Python or reference datasets. External signing/notarization, publishing
and system-wide installation require authorization.

## CLI-only and full-toolkit installation

- CLI-only installs `pnm-decode`, `pnm-decode.1`, usage documentation and notices;
  it requires no UI or WebView. The initial source installer installs the manual
  and notices; Markdown usage is available in the checkout.
- Future full-toolkit/UI packages must also install the CLI and its manual at
  the same project version. Both applications call core directly; the UI does
  not spawn the CLI for parsing.
- Package ownership must preserve a separately installed CLI when removing the
  UI. Prefer a CLI package dependency where supported; bundled installers need
  explicit component ownership/upgrade rules. Do not infer ownership from a
  filename alone. These UI installer rules are design requirements, not implemented.
- Initial host test: Ubuntu 22.04 x86_64, isolated prefix under `local/`, no
  system packages or PATH changes. Windows/macOS require native validation later.

See [CLI usage](../cli/usage.md) for the source installer and current limitations.
