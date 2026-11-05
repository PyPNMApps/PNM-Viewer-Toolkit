/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Paired Ubuntu update identity, download and authenticated installation.
mod download;
mod install;
#[cfg(test)]
mod tests;
use super::release::ReleaseVersion;
pub use download::download;
pub use install::{install, install_elevated, restart};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
const MAX_PACKAGE_BYTES: u64 = 50_000_000;
const INSTALLED_VIEWER: &str = "/opt/pnm-viewer-toolkit/pnm-viewer-qt";
const PACKAGES: [&str; 2] = ["pnm-decode", "pnm-viewer-toolkit"];

/// Relevant immutable fields from the GitHub release asset API.
#[derive(Debug, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    pub digest: Option<String>,
    pub browser_download_url: String,
}

/// Exact bytes and release identity accepted by the download/install boundary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub size: u64,
    pub sha256: String,
    pub url: String,
}

/// One newer version containing both the CLI and viewer DEBs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallPlan {
    pub version: String,
    pub packages: Vec<Package>,
}

impl InstallPlan {
    /// Validates the same-version Ubuntu pair using GitHub's asset digests.
    /// # Arguments
    /// * `version` - Stable numeric triple without a tag prefix.
    /// * `assets` - Assets from this project's published release metadata.
    /// # Returns
    /// A validated CLI-first pair, independent of release asset ordering.
    /// # Errors
    /// Rejects absent/duplicate assets, invalid identity, size or digest.
    pub fn from_assets(version: &str, assets: &[ReleaseAsset]) -> Result<Self, String> {
        let mut packages = Vec::new();
        for package in PACKAGES {
            let name = format!("{package}_{version}_amd64.deb");
            let mut matching = assets.iter().filter(|asset| asset.name == name);
            let asset = matching
                .next()
                .ok_or("Release is missing a matching Ubuntu package.")?;
            if matching.next().is_some() {
                return Err("Duplicate Ubuntu package metadata.".into());
            }
            packages.push(Package {
                name,
                size: asset.size,
                sha256: asset
                    .digest
                    .as_deref()
                    .and_then(|digest| digest.strip_prefix("sha256:"))
                    .ok_or("Release package has no SHA-256 digest.")?
                    .to_ascii_lowercase(),
                url: asset.browser_download_url.clone(),
            });
        }
        let plan = Self {
            version: version.into(),
            packages,
        };
        plan.validate()?;
        Ok(plan)
    }

    /// Checks a deserialized plan before any download or elevated staging.
    /// # Returns
    /// Success only for the exact same-version project-owned package pair.
    /// # Errors
    /// Rejects unexpected filenames, URLs, digest syntax, sizes or versions.
    pub fn validate(&self) -> Result<(), String> {
        ReleaseVersion::parse(&self.version)?;
        if self.version.starts_with('v') || self.packages.len() != PACKAGES.len() {
            return Err("Invalid Ubuntu update plan.".into());
        }
        for (package, expected) in self.packages.iter().zip(PACKAGES) {
            let name = format!("{expected}_{}_amd64.deb", self.version);
            let url = format!(
                "https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases/download/v{}/{name}",
                self.version
            );
            if package.name != name
                || package.url != url
                || package.size == 0
                || package.size > MAX_PACKAGE_BYTES
                || package.sha256.len() != 64
                || !package.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
            {
                return Err("Invalid Ubuntu package identity, size or SHA-256.".into());
            }
        }
        Ok(())
    }
}

/// Identifies supported installed Ubuntu x64 runs; source builds never elevate.
/// # Returns
/// True when this executable is the packaged viewer and Polkit is available.
pub fn supported() -> bool {
    cfg!(all(target_os = "linux", target_arch = "x86_64"))
        && std::env::current_exe().ok().as_deref() == Some(Path::new(INSTALLED_VIEWER))
        && Path::new("/usr/bin/pkexec").is_file()
        && fs::read_to_string("/etc/os-release").is_ok_and(|text| {
            text.lines()
                .any(|line| matches!(line, "ID=ubuntu" | "ID=\"ubuntu\""))
        })
}

/// Creates update storage with owner-only access before any content is written.
/// # Arguments
/// * `prefix` - Identifies download or elevated staging directories.
/// # Returns
/// An owned directory removed automatically when dropped; Unix mode is 0700.
/// # Errors
/// Reports temporary-directory creation failures.
fn private_directory(prefix: &str) -> Result<tempfile::TempDir, String> {
    let mut builder = tempfile::Builder::new();
    builder.prefix(prefix);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        builder.permissions(fs::Permissions::from_mode(0o700));
    }
    builder.tempdir().map_err(|error| error.to_string())
}
