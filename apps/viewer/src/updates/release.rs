/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Read-only stable-release discovery, independent of capture state and Qt.
use serde::Deserialize;
use std::{io::Read, time::Duration};

pub const RELEASES_URL: &str = "https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases";
const RELEASE_API_URL: &str =
    "https://api.github.com/repos/PyPNMApps/PNM-Viewer-Toolkit/releases/latest";
const MAX_RESPONSE_BYTES: usize = 1024 * 1024;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);
const MAX_VERSION_BYTES: usize = 64;

/// A completed release lookup; no installer action is implied by availability.
#[derive(Debug, PartialEq, Eq)]
pub struct ReleaseCheck {
    /// Stable UI state: available, current, ahead, or no_release.
    pub state: &'static str,
    /// Human-readable plain text generated locally, never remote HTML.
    pub status: String,
    /// Published numeric version, absent when there is no published release.
    pub latest_version: Option<String>,
    /// Project-owned release page or fixed releases index.
    pub release_url: String,
}

#[derive(Deserialize)]
struct ReleaseMetadata {
    tag_name: String,
    draft: bool,
    prerelease: bool,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct ReleaseVersion([u64; 3]);
impl ReleaseVersion {
    fn parse(text: &str) -> Result<Self, String> {
        if text.len() > MAX_VERSION_BYTES {
            return Err("Release version is too long.".into());
        }
        let components = text.strip_prefix('v').unwrap_or(text).split('.');
        let mut numbers = [0; 3];
        let mut count = 0;
        for (index, component) in components.enumerate() {
            if index >= numbers.len()
                || component.is_empty()
                || !component.bytes().all(|byte| byte.is_ascii_digit())
                || (component.len() > 1 && component.starts_with('0'))
            {
                return Err("Expected a stable MAJOR.MINOR.PATCH version.".into());
            }
            numbers[index] = component
                .parse()
                .map_err(|_| "Release version component is too large.")?;
            count += 1;
        }
        if count != numbers.len() {
            return Err("Expected a stable MAJOR.MINOR.PATCH version.".into());
        }
        Ok(Self(numbers))
    }
}

/// Checks this project's latest published stable release on the calling worker.
///
/// # Arguments
/// * `installed_version` - Version embedded in the running application.
/// # Returns
/// A release comparison or normal no-published-release result.
/// # Errors
/// Reports bounded network, HTTP, JSON and version-validation failures. The call
/// uses TLS, no credentials, no redirects, and a ten-second request timeout.
pub fn check_release(installed_version: &str) -> Result<ReleaseCheck, String> {
    fetch_release(installed_version, RELEASE_API_URL, REQUEST_TIMEOUT)
}

fn fetch_release(
    installed_version: &str,
    address: &str,
    timeout: Duration,
) -> Result<ReleaseCheck, String> {
    ReleaseVersion::parse(installed_version)?;
    let agent = ureq::AgentBuilder::new()
        .timeout(timeout)
        .redirects(0)
        .build();
    let response = match agent
        .get(address)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "PNM-Viewer-Toolkit")
        .call()
    {
        Ok(response) if response.status() == 200 => response,
        Err(ureq::Error::Status(404, _)) => {
            return Ok(ReleaseCheck {
                state: "no_release",
                status: "No published release is available yet.".into(),
                latest_version: None,
                release_url: RELEASES_URL.into(),
            });
        }
        Err(ureq::Error::Status(code, _)) => {
            return Err(format!("GitHub returned HTTP {code}. Try again later."));
        }
        Ok(response) => {
            return Err(format!(
                "Unexpected release response: HTTP {}.",
                response.status()
            ));
        }
        Err(_) => {
            return Err(
                "Unable to check for updates. Check your connection or try again later.".into(),
            );
        }
    };
    if !response.header("Content-Type").is_some_and(|value| {
        value.starts_with("application/json") || value.starts_with("application/vnd.github+json")
    }) {
        return Err("GitHub returned an unexpected release content type.".into());
    }
    let mut bytes = Vec::new();
    response
        .into_reader()
        .take((MAX_RESPONSE_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "Unable to read release metadata. Try again later.")?;
    parse_release(installed_version, &bytes)
}

fn parse_release(installed_version: &str, bytes: &[u8]) -> Result<ReleaseCheck, String> {
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err("GitHub returned oversized release metadata.".into());
    }
    let release: ReleaseMetadata = serde_json::from_slice(bytes)
        .map_err(|_| "GitHub returned incomplete release metadata.")?;
    if release.draft || release.prerelease || !release.tag_name.starts_with('v') {
        return Err("GitHub did not return a published stable release tag.".into());
    }
    let latest = ReleaseVersion::parse(&release.tag_name)?;
    let installed = ReleaseVersion::parse(installed_version)?;
    let latest_version = release.tag_name.trim_start_matches('v').to_string();
    let (state, status) = match latest.cmp(&installed) {
        std::cmp::Ordering::Greater => (
            "available",
            format!(
                "Update available: {}. Installed: {installed_version}.",
                release.tag_name
            ),
        ),
        std::cmp::Ordering::Equal => ("current", "You are up to date.".into()),
        std::cmp::Ordering::Less => (
            "ahead",
            "This build is newer than the latest published release.".into(),
        ),
    };
    Ok(ReleaseCheck {
        state,
        status,
        latest_version: Some(latest_version),
        release_url: format!("{RELEASES_URL}/tag/{}", release.tag_name),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Write, net::TcpListener, thread};

    fn metadata(tag: &str) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "tag_name": tag, "draft": false, "prerelease": false
        }))
        .unwrap()
    }

    #[test]
    fn compares_three_component_versions_numerically() {
        for (installed, published, state) in [
            ("0.1.5", "v0.1.6", "available"),
            ("0.9.0", "v0.10.0", "available"),
            ("0.1.5", "v0.1.5", "current"),
            ("1.0.0", "v0.99.99", "ahead"),
        ] {
            let result = parse_release(installed, &metadata(published)).unwrap();
            assert_eq!(result.state, state);
            assert_eq!(
                result.release_url,
                format!("{RELEASES_URL}/tag/{published}")
            );
        }
    }

    #[test]
    fn rejects_unsafe_versions_drafts_prereleases_and_incomplete_json() {
        for tag in [
            "v1.0",
            "v01.0.0",
            "v1.0.0-beta",
            "v1.0.0/path",
            "1.0.0",
            "vv1.0.0",
            "v18446744073709551616.0.0",
            "v1.0.0+build",
        ] {
            assert!(parse_release("0.1.5", &metadata(tag)).is_err(), "{tag}");
        }
        for body in [
            b"{}".as_slice(),
            b"<html>error</html>",
            br#"{"tag_name":"v1.0.0","draft":true,"prerelease":false}"#,
            br#"{"tag_name":"v1.0.0","draft":false,"prerelease":true}"#,
        ] {
            assert!(parse_release("0.1.5", body).is_err());
        }
        assert!(parse_release("unknown", &metadata("v1.0.0")).is_err());
        assert!(parse_release("0.1.5", &vec![b' '; MAX_RESPONSE_BYTES + 1]).is_err());
    }

    fn http_result(code: &str, content_type: &str, body: &[u8]) -> Result<ReleaseCheck, String> {
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = format!("http://{}", server.local_addr().unwrap());
        let header = format!("HTTP/1.1 {code}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len());
        let body = body.to_vec();
        let worker = thread::spawn(move || {
            let (mut client, _) = server.accept().unwrap();
            let mut request = [0; 4096];
            let _read = client.read(&mut request).unwrap();
            let _written = client
                .write_all(header.as_bytes())
                .and_then(|()| client.write_all(&body));
        });
        let result = fetch_release("0.1.5", &address, REQUEST_TIMEOUT);
        worker.join().unwrap();
        result
    }

    #[test]
    fn network_timeout_is_bounded_and_does_not_require_github() {
        let server = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = format!("http://{}", server.local_addr().unwrap());
        let worker = thread::spawn(move || {
            let (_client, _) = server.accept().unwrap();
            thread::sleep(Duration::from_millis(200));
        });
        let result = fetch_release("0.1.5", &address, Duration::from_millis(50));
        worker.join().unwrap();
        assert!(result.unwrap_err().contains("Unable to check"));
    }

    #[test]
    fn handles_missing_release_and_http_failures_without_github() {
        assert_eq!(
            http_result("404 Not Found", "application/json", b"{}")
                .unwrap()
                .state,
            "no_release"
        );
        for code in [
            "403 Forbidden",
            "429 Too Many Requests",
            "500 Server Error",
            "302 Found",
        ] {
            assert!(http_result(code, "application/json", b"{}").is_err());
        }
        assert!(http_result("200 OK", "text/html", b"error").is_err());
        assert!(http_result("200 OK", "application/json", b"broken").is_err());
        assert!(http_result(
            "200 OK",
            "application/json",
            &vec![b' '; MAX_RESPONSE_BYTES + 1]
        )
        .is_err());
        assert_eq!(
            http_result("200 OK", "application/json", &metadata("v0.2.0"))
                .unwrap()
                .state,
            "available"
        );
    }
}
