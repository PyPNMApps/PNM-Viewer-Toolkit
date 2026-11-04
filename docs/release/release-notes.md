PNM Viewer Toolkit is under development.

This draft contains native Qt viewer packages and standalone `pnm-decode`
archives. The viewer supports RxMER, Spectrum Analyzer, downstream channel
estimation, histogram and FEC summary files, with JSON export and System/Dark/Light themes.

Packages are built only after CodeQL, code checks and the complete native CI
matrix pass. Installed-package checks cover Ubuntu 22.04/24.04/26.04, Windows
x64, and macOS Apple Silicon/Intel. Inspect the workflow logs for this tag's
results. Checksums and package-size/dependency metadata accompany the assets.

Windows packages are unsigned; macOS apps use ad-hoc signing and are not
notarized. Linux uses declared distribution prerequisites. These are development
builds, not a release-readiness claim. Review the packaging and user guides in
the tagged source before installation.
