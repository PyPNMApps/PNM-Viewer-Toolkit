/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared header data and inheritance-like access without class inheritance.
//! Decoders select their own evidenced layout; this is not a universal decoder.
use crate::pnm::data_type::{CaptureTime, FileVersionComponent, PnmFileType};

/// Version of a PNM binary layout, distinct from the DOCSIS protocol version.
/// Named fields keep major and vendor-specific minor components explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PnmFormatVersion {
    /// Specification-assigned major file-format version.
    pub major: FileVersionComponent,
    /// Vendor-specific minor version; zero is the default.
    pub minor: FileVersionComponent,
}

/// Common metadata owned by a format-specific capture.
/// Capture time may be absent for formats that do not encode it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PnmHeader {
    pub file_type: PnmFileType,
    /// Encoded binary-layout version.
    pub format_version: PnmFormatVersion,
    pub capture_time: Option<CaptureTime>,
}

/// Implemented by captures that embed a common header.
/// Default methods provide the shared base-class API through composition.
pub trait HasPnmHeader {
    /// Borrows the header embedded in this capture.
    ///
    /// # Returns
    /// A shared reference; ownership stays with the capture.
    fn pnm_header(&self) -> &PnmHeader;
    /// Reads the encoded file-type identifier.
    ///
    /// # Returns
    /// Four identifier bytes without assuming protocol applicability.
    fn file_type(&self) -> PnmFileType {
        self.pnm_header().file_type
    }
    /// Reads optional capture time metadata.
    ///
    /// # Returns
    /// Epoch seconds when present; None remains distinct from zero.
    fn capture_time(&self) -> Option<CaptureTime> {
        self.pnm_header().capture_time
    }
    /// Reads the encoded format version.
    ///
    /// # Returns
    /// The named major/minor components, not the DOCSIS protocol version.
    fn format_version(&self) -> PnmFormatVersion {
        self.pnm_header().format_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Capture {
        header: PnmHeader,
    }
    impl HasPnmHeader for Capture {
        /// Borrows the header embedded in this capture.
        ///
        /// # Returns
        /// A shared reference; ownership stays with the capture.
        fn pnm_header(&self) -> &PnmHeader {
            &self.header
        }
    }
    #[test]
    fn shared_access_works_through_trait_object() {
        let capture = Capture {
            header: PnmHeader {
                file_type: *b"TEST",
                format_version: PnmFormatVersion { major: 1, minor: 0 },
                capture_time: None,
            },
        };
        let base: &dyn HasPnmHeader = &capture;
        assert_eq!(base.file_type(), *b"TEST");
        assert_eq!(
            base.format_version(),
            PnmFormatVersion { major: 1, minor: 0 }
        );
        assert_eq!(base.capture_time(), None);
    }
}
