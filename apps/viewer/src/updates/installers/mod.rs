/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Select published installer links without downloading or installing anything.
use super::{release::RELEASES_URL, ubuntu::ReleaseAsset};

/// A platform-specific browser download action from validated release metadata.
#[derive(Debug, PartialEq, Eq)]
pub struct InstallerDownload {
    /// Plain-text action identifying the native installer or Ubuntu package pair.
    pub label: &'static str,
    /// Exact project-owned asset URL, or the release page for paired DEBs.
    pub url: String,
}

/// Finds a native installer for an already validated three-component version.
///
/// # Arguments
/// * `version` - Validated numeric release version without a leading `v`.
/// * `assets` - Published assets returned by this project's release endpoint.
/// * `operating_system` - Rust target OS name: windows, macos or linux.
/// * `architecture` - Rust target architecture: x86_64 or aarch64.
///
/// # Returns
/// Returns no action for unsupported targets, missing/duplicate assets, empty
/// files or redirected URLs. Ubuntu uses its existing validated package pair
/// and opens the version's release page rather than offering only one DEB.
pub(super) fn select(
    version: &str,
    assets: &[ReleaseAsset],
    operating_system: &str,
    architecture: &str,
) -> Option<InstallerDownload> {
    let (suffix, label) = match (operating_system, architecture) {
        ("windows", "x86_64") => ("windows-x64.msi", "Download MSI"),
        ("macos", "aarch64") => ("macos-arm64.dmg", "Download DMG"),
        ("macos", "x86_64") => ("macos-x64.dmg", "Download DMG"),
        ("linux", "x86_64") => {
            super::ubuntu::InstallPlan::from_assets(version, assets).ok()?;
            return Some(InstallerDownload {
                label: "Download packages",
                url: format!("{RELEASES_URL}/tag/v{version}"),
            });
        }
        _ => return None,
    };
    let name = format!("PNM-Viewer-Toolkit-{version}-{suffix}");
    let mut matching = assets.iter().filter(|asset| asset.name == name);
    let asset = matching.next()?;
    let url = format!("{RELEASES_URL}/download/v{version}/{name}");
    if matching.next().is_some() || asset.size == 0 || asset.browser_download_url != url {
        return None;
    }
    Some(InstallerDownload { label, url })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn asset(name: &str) -> ReleaseAsset {
        ReleaseAsset {
            name: name.into(),
            size: 100,
            digest: Some(format!("sha256:{}", "a".repeat(64))),
            browser_download_url: format!("{RELEASES_URL}/download/v0.3.3/{name}"),
        }
    }

    #[test]
    fn selects_exact_installer_for_each_native_architecture() {
        let assets = [
            asset("PNM-Viewer-Toolkit-0.3.3-windows-x64.msi"),
            asset("PNM-Viewer-Toolkit-0.3.3-macos-arm64.dmg"),
            asset("PNM-Viewer-Toolkit-0.3.3-macos-x64.dmg"),
        ];
        for (system, architecture, suffix, label) in [
            ("windows", "x86_64", "windows-x64.msi", "Download MSI"),
            ("macos", "aarch64", "macos-arm64.dmg", "Download DMG"),
            ("macos", "x86_64", "macos-x64.dmg", "Download DMG"),
        ] {
            let download = select("0.3.3", &assets, system, architecture).unwrap();
            assert_eq!(download.label, label);
            assert_eq!(
                download.url,
                format!("{RELEASES_URL}/download/v0.3.3/PNM-Viewer-Toolkit-0.3.3-{suffix}")
            );
        }
        assert!(select("0.3.3", &assets, "windows", "aarch64").is_none());
        assert!(select("0.3.4", &assets, "macos", "aarch64").is_none());
    }

    #[test]
    fn rejects_absent_duplicate_empty_or_external_installer_assets() {
        let name = "PNM-Viewer-Toolkit-0.3.3-windows-x64.msi";
        assert!(select("0.3.3", &[], "windows", "x86_64").is_none());
        assert!(select("0.3.3", &[asset(name), asset(name)], "windows", "x86_64").is_none());
        let mut empty = asset(name);
        empty.size = 0;
        assert!(select("0.3.3", &[empty], "windows", "x86_64").is_none());
        let mut redirected = asset(name);
        redirected.browser_download_url = "https://example.invalid/installer.msi".into();
        assert!(select("0.3.3", &[redirected], "windows", "x86_64").is_none());
    }

    #[test]
    fn ubuntu_manual_download_requires_both_verified_packages() {
        let assets = [
            asset("pnm-decode_0.3.3_amd64.deb"),
            asset("pnm-viewer-toolkit_0.3.3_amd64.deb"),
        ];
        let download = select("0.3.3", &assets, "linux", "x86_64").unwrap();
        assert_eq!(download.label, "Download packages");
        assert_eq!(download.url, format!("{RELEASES_URL}/tag/v0.3.3"));
        assert!(select("0.3.3", &assets[..1], "linux", "x86_64").is_none());
    }
}
