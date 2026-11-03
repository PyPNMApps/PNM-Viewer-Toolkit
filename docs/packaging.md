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
are GPL-3.0-only with corresponding-source obligations. The root [LICENSE](../LICENSE)
records component scope; full texts are under `LICENSES/`. Include the applicable
texts and NOTICE content in distributables. Do not bundle PDFs, private
captures, Python or reference datasets. External signing/notarization, publishing
and system-wide installation require authorization.
