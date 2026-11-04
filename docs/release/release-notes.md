PNM Viewer Toolkit is under development.

This draft contains native Qt viewer packages and standalone `pnm-decode`
archives. The viewer supports RxMER, Spectrum Analyzer, downstream channel
estimation, histogram, FEC summary and modulation profile files, with JSON export and System/Dark/Light themes.

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
