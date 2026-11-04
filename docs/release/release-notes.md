PNM Viewer Toolkit is under development.

This draft contains native Qt viewer packages and standalone `pnm-decode`
archives. The viewer supports RxMER, Spectrum Analyzer, downstream channel
estimation, histogram, FEC summary, modulation profile and constellation files,
with JSON export and System/Dark/Light themes.

Modulation-profile JSON now uses textual modulation names and adds data
`bit_loading`, carrier roles and retained numeric assignment codes. JSON consumers
should read the former numeric value from `modulation_order_code`; skip schemes
use the same fields with `main_` and `skip_` prefixes.

Modulation controls now use equal-sized profile/carrier toggles and categorical
carrier lanes with hover details. Core and CLI JSON include ideal per-profile
data bandwidth and gross useful-symbol rate with explicit overhead exclusions.

Profiles now expose bandwidth_bps directly, shared by CLI/UI exports. A compact
Profile BW chip opens a per-profile bandwidth grid using Rust-supplied values.

Packages are built only after CodeQL, code checks and the complete native CI
matrix pass. Installed-package checks cover Ubuntu 22.04/24.04/26.04, Windows
x64, and macOS Apple Silicon/Intel. Inspect the workflow logs for this tag's
results. Checksums and package-size/dependency metadata accompany the assets.

Windows packages are unsigned; macOS apps use ad-hoc signing and are not
notarized. Linux uses declared distribution prerequisites. These are development
builds, not a release-readiness claim. Review the packaging and user guides in
the tagged source before installation.

PLC and continuous-pilot assignments now export physical constellation metadata:
16-QAM PLC data (4 bits) and BPSK pilots (1 bit), with a separate BPSK PLC
preamble. CLI and Qt share Rust-derived ideal Shannon references; both roles
plot on the numeric axis without contributing to data-only profile bandwidth.
See the [JSON contract](../reference/formats.md#modulation-profile-json-assignments).

Constellation Display is available in the Qt viewer with soft/hard decision overlays,
layer toggles and coordinate zoom. Preserved PyPNM QAM resources compile into
static Rust lookup tables; runtime viewing needs no table files or database.
Core/CLI JSON now includes shared reference analysis while preserving measured I/Q.

The constellation viewer now labels the layers Soft decisions and Hard decisions,
adds independently controlled nearest-symbol boundaries, and removes numeric
axis scales. Rust compiles square-QAM midpoint grids and cross-QAM diagonal
boundaries into the same reference data shared by CLI and desktop JSON.

Release preparation keeps root `VERSION` as the shared three-component version
source. Bump previews validate metadata and list affected files. Developer guides
now separate Qt setup from measurement internals and include first-tag and
screenshot checklists. The dark/light gallery includes FEC Summary and the
constellation view without its sample-count footer.

Qt Quick is now the sole desktop UI. The Tauri/React prototype, Node tooling,
WebKit prerequisites and associated CI checks have been removed. CLI/parser
behavior and the Qt packaging matrix are unchanged. Version helpers synchronize
only the Rust workspace and standalone Qt manifests/lockfiles.

The Git push helper now supports OSCAR-UI-style --tag-patch, --tag-minor and
--tag-major releases: synchronize VERSION/Cargo, validate the bumped candidate,
commit, annotate and atomically push master plus the selected tag. Existing-tag
mode supports first-tag/retry workflows without another bump. Plain push remains
branch-only and tag CI still creates draft releases rather than publishing.
