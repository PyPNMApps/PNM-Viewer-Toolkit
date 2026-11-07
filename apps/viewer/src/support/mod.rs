/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! User-reviewed support drafts with an explicit, non-sensitive metadata list.
use crate::updates::release::RELEASES_URL;

/// Builds a GitHub issue draft locally without submitting or contacting GitHub.
///
/// # Arguments
/// * `app_version` - Installed viewer version.
/// * `platform` - Operating-system family, excluding host or user identifiers.
/// * `architecture` - Executable architecture.
///
/// # Returns
/// An encoded browser URL containing version/platform metadata and empty report
/// sections. It contains no capture data, file paths, logs or environment dump.
pub fn issue_report_url(app_version: &str, platform: &str, architecture: &str) -> String {
    let body = format!(
        "## Environment\n- PNM Viewer Toolkit: {app_version}\n- OS: {platform}\n\
         - Architecture: {architecture}\n\n## Steps to reproduce\n\n1. \n\n\
         ## Expected behavior\n\n## Actual behavior\n\n## Screenshots (optional)\n\n\
         Please remove secrets and customer data before adding screenshots or files.\n"
    );
    let query = url::form_urlencoded::Serializer::new(String::new())
        .append_pair("title", "[Bug] ")
        .append_pair("body", &body)
        .finish();
    let project_url = RELEASES_URL.trim_end_matches("/releases");
    format!("{project_url}/issues/new?{query}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draft_preserves_encoded_metadata_and_empty_report_sections() {
        let draft = issue_report_url("0.2.12+test & check", "Windows", "x86_64");
        let parsed = url::Url::parse(&draft).unwrap();
        assert_eq!(parsed.scheme(), "https");
        assert_eq!(parsed.host_str(), Some("github.com"));
        assert_eq!(parsed.path(), "/PyPNMApps/PNM-Viewer-Toolkit/issues/new");
        let parameters: Vec<_> = parsed.query_pairs().collect();
        assert_eq!(parameters.len(), 2);
        assert_eq!(parameters[0].1, "[Bug] ");
        let body = &parameters[1].1;
        for expected in [
            "0.2.12+test & check",
            "Windows",
            "x86_64",
            "Steps to reproduce",
            "Expected behavior",
            "Actual behavior",
            "Screenshots (optional)",
        ] {
            assert!(body.contains(expected));
        }
        assert!(!body.contains("mac_address"));
        assert!(!body.contains("/home/"));
    }
}
