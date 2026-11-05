/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Privileged staging and apt handoff; no network access or Qt initialization.
#[cfg(target_os = "linux")]
use super::{download::copy_verified, private_directory, InstallPlan, PACKAGES};
use super::{supported, INSTALLED_VIEWER};
#[cfg(target_os = "linux")]
use std::{
    fs::{self, File},
    io::Read,
    sync::atomic::AtomicBool,
    time::{Duration, Instant},
};
use std::{path::Path, process::Command};
#[cfg(target_os = "linux")]
const MAX_PLAN_BYTES: u64 = 16_384;

/// Requests administrator authentication and waits for paired package installation.
/// # Arguments
/// * `directory` - Private verified download directory retained by the caller.
/// # Returns
/// Success after the privileged installer reports successful apt completion.
/// # Errors
/// Rejects source/unsupported runs, canceled authentication or install failures.
pub fn install(directory: &Path) -> Result<(), String> {
    if !supported() {
        return Err("Installation requires the installed Ubuntu x64 viewer.".into());
    }
    let status = Command::new("/usr/bin/pkexec")
        .arg(INSTALLED_VIEWER)
        .arg("--install-update")
        .arg(directory)
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        return Err("Administrator approval was canceled or installation failed. Check installed package versions before retrying.".into());
    }
    Ok(())
}

/// Handles elevated installation before Qt initialization; never downloads code.
/// # Arguments
/// * `directory` - Private download directory owned by the authenticated user.
/// # Returns
/// Success after root-private copies pass identity/hash checks and apt succeeds.
/// # Errors
/// Rejects non-root/unsupported calls, unsafe files, mismatched or older packages.
pub fn install_elevated(directory: &Path) -> Result<(), String> {
    #[cfg(target_os = "linux")]
    {
        use std::os::unix::fs::MetadataExt;
        if !supported() {
            return Err("Installation requires the installed Ubuntu x64 viewer.".into());
        }
        let identity = Command::new("/usr/bin/id")
            .arg("-u")
            .output()
            .map_err(|error| error.to_string())?;
        if !identity.status.success() || identity.stdout != b"0\n" {
            return Err("The update installer requires administrator authentication.".into());
        }
        let owner: u32 = std::env::var("PKEXEC_UID")
            .map_err(|_| "Missing authenticated user identity.")?
            .parse()
            .map_err(|_| "Invalid authenticated user identity.")?;
        let metadata = fs::symlink_metadata(directory).map_err(|error| error.to_string())?;
        if !metadata.is_dir() || metadata.uid() != owner || metadata.mode() & 0o077 != 0 {
            return Err(
                "Update directory must be private and owned by the authenticated user.".into(),
            );
        }
        let plan_file =
            File::open(directory.join("plan.json")).map_err(|error| error.to_string())?;
        let mut bytes = Vec::new();
        plan_file
            .take(MAX_PLAN_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| error.to_string())?;
        if bytes.len() as u64 > MAX_PLAN_BYTES {
            return Err("Update plan is too large.".into());
        }
        let plan: InstallPlan =
            serde_json::from_slice(&bytes).map_err(|_| "Invalid update plan.")?;
        plan.validate()?;
        let staging = private_directory("pnm-install-")?;
        let mut destinations = Vec::new();
        for (package, expected) in plan.packages.iter().zip(PACKAGES) {
            let source = directory.join(&package.name);
            let metadata = fs::symlink_metadata(&source).map_err(|error| error.to_string())?;
            if !metadata.is_file() || metadata.uid() != owner {
                return Err("Unsafe update package file.".into());
            }
            let input = File::open(source).map_err(|error| error.to_string())?;
            if !input
                .metadata()
                .map_err(|error| error.to_string())?
                .is_file()
            {
                return Err("Update must be a regular file.".into());
            }
            let destination = staging.path().join(&package.name);
            let output = File::create(&destination).map_err(|error| error.to_string())?;
            copy_verified(
                input,
                output,
                package,
                &AtomicBool::new(false),
                Instant::now() + Duration::from_secs(60),
                |_| {},
            )?;
            validate_deb(&destination, expected, &plan.version)?;
            let installed = Command::new("/usr/bin/dpkg-query")
                .args(["-W", "-f=${Version}", expected])
                .output()
                .map_err(|error| error.to_string())?;
            if !installed.status.success() {
                return Err("Both existing packages must be installed.".into());
            }
            let installed =
                String::from_utf8(installed.stdout).map_err(|_| "Invalid installed version.")?;
            if !Command::new("/usr/bin/dpkg")
                .args(["--compare-versions", &plan.version, "gt", &installed])
                .status()
                .map_err(|error| error.to_string())?
                .success()
            {
                return Err("The update must be newer than both installed packages.".into());
            }
            destinations.push(destination);
        }
        let status = Command::new("/usr/bin/apt-get")
            .env("DEBIAN_FRONTEND", "noninteractive")
            .args(["install", "-y", "--"])
            .args(destinations)
            .status()
            .map_err(|error| error.to_string())?;
        if !status.success() {
            return Err(
                "Package installation failed. Check apt output and installed versions.".into(),
            );
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = directory;
        Err("In-app installation is supported only on Ubuntu x64.".into())
    }
}

/// Checks the staged archive's package name, version and architecture.
/// # Arguments
/// * `path` - Root-staged regular DEB archive already verified by hash.
/// * `expected` - Exact CLI or viewer package name.
/// * `version` - Requested stable numeric version.
/// # Errors
/// Rejects unreadable archives or any mismatched control field.
#[cfg(target_os = "linux")]
pub(super) fn validate_deb(path: &Path, expected: &str, version: &str) -> Result<(), String> {
    for (field, value) in [
        ("Package", expected),
        ("Version", version),
        ("Architecture", "amd64"),
    ] {
        let output = Command::new("/usr/bin/dpkg-deb")
            .args(["-f"])
            .arg(path)
            .arg(field)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() || String::from_utf8_lossy(&output.stdout).trim() != value {
            return Err(format!("Update package has an unexpected {field}."));
        }
    }
    Ok(())
}

/// Starts the newly installed viewer without a shell; call only after success.
/// # Returns
/// A child handle whose process lifetime is independent of the old viewer.
/// # Errors
/// Reports failure to start the fixed installed executable.
pub fn restart() -> Result<std::process::Child, String> {
    Command::new(INSTALLED_VIEWER)
        .spawn()
        .map_err(|error| error.to_string())
}
