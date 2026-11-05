/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Unprivileged TLS downloads and shared bounded hash verification.
use super::{private_directory, InstallPlan, Package};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

/// Downloads and verifies both DEBs without privileges into a private directory.
/// # Arguments
/// * `plan` - Validated newer release package pair.
/// * `canceled` - User cancellation, checked between reads/requests.
/// * `progress` - Worker-thread percentage callback across both downloads.
/// # Returns
/// An owned temporary directory, removed on failure or after installation.
/// # Errors
/// Reports identity, TLS/HTTP, redirect, timeout, cancellation, hash or I/O errors.
pub fn download(
    plan: &InstallPlan,
    canceled: &AtomicBool,
    progress: impl Fn(u32),
) -> Result<tempfile::TempDir, String> {
    plan.validate()?;
    let directory = private_directory("pnm-viewer-update-")?;
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(20))
        .build();
    let total: u64 = plan.packages.iter().map(|package| package.size).sum();
    let mut completed = 0;
    for package in &plan.packages {
        let mut address = url::Url::parse(&package.url).map_err(|error| error.to_string())?;
        let deadline = Instant::now() + Duration::from_secs(15 * 60);
        let mut response = None;
        for _ in 0..=5 {
            check_canceled(canceled)?;
            if !trusted_download_address(&address) {
                return Err("Untrusted update download redirect.".into());
            }
            let received = agent
                .get(address.as_str())
                .set("User-Agent", "PNM-Viewer-Toolkit")
                .call()
                .map_err(|_| "Unable to download the update. Check your connection.")?;
            match received.status() {
                200 => {
                    response = Some(received);
                    break;
                }
                301 | 302 | 303 | 307 | 308 => {
                    address = address
                        .join(
                            received
                                .header("Location")
                                .ok_or("Missing update redirect.")?,
                        )
                        .map_err(|error| error.to_string())?;
                }
                _ => return Err("Unexpected update download response.".into()),
            }
        }
        let response = response.ok_or("Too many update download redirects.")?;
        let mut output = File::create(directory.path().join(&package.name))
            .map_err(|error| error.to_string())?;
        copy_verified(
            response.into_reader(),
            &mut output,
            package,
            canceled,
            deadline,
            |received| progress(((completed + received) * 100 / total) as u32),
        )?;
        output.sync_all().map_err(|error| error.to_string())?;
        completed += package.size;
    }
    fs::write(
        directory.path().join("plan.json"),
        serde_json::to_vec(plan).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    Ok(directory)
}

/// Restricts each request and redirect to TLS on the GitHub release hosts.
/// # Arguments
/// * `address` - Parsed URL including any signed GitHub query string.
/// # Returns
/// True only for the allowlisted hosts, standard TLS port and no credentials.
pub(super) fn trusted_download_address(address: &url::Url) -> bool {
    address.scheme() == "https"
        && address.username().is_empty()
        && address.password().is_none()
        && address.port().is_none()
        && matches!(
            address.host_str(),
            Some(
                "github.com"
                    | "release-assets.githubusercontent.com"
                    | "objects.githubusercontent.com"
            )
        )
}

fn check_canceled(canceled: &AtomicBool) -> Result<(), String> {
    if canceled.load(Ordering::Relaxed) {
        return Err("Update canceled. The installed version is unchanged.".into());
    }
    Ok(())
}

/// Copies bounded bytes while verifying size and hash, aborting on cancellation.
/// # Arguments
/// * `input` - Untrusted stream; EOF must match the package's declared size.
/// * `output` - Destination owned by the caller; incomplete copies are discarded.
/// * `package` - Expected byte count and SHA-256 digest.
/// * `canceled` - Cancellation signal checked before every read.
/// * `deadline` - Absolute copy deadline; caller also bounds blocking I/O.
/// * `progress` - Receives cumulative bytes on the calling thread.
/// # Errors
/// Reports I/O, cancellation, timeout, size or hash mismatch.
pub(super) fn copy_verified(
    mut input: impl Read,
    mut output: impl Write,
    package: &Package,
    canceled: &AtomicBool,
    deadline: Instant,
    progress: impl Fn(u64),
) -> Result<(), String> {
    let mut digest = Sha256::new();
    let mut received = 0_u64;
    let mut buffer = [0; 65_536];
    loop {
        check_canceled(canceled)?;
        if Instant::now() >= deadline {
            return Err("Update download timed out.".into());
        }
        let count = input.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        received += count as u64;
        if received > package.size {
            return Err("Update exceeds its declared size.".into());
        }
        digest.update(&buffer[..count]);
        output
            .write_all(&buffer[..count])
            .map_err(|error| error.to_string())?;
        progress(received);
    }
    if received != package.size
        || format!("{:x}", digest.finalize()) != package.sha256.to_ascii_lowercase()
    {
        return Err("Update size or SHA-256 verification failed.".into());
    }
    Ok(())
}
