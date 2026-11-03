/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared header data and inheritance-like access without class inheritance.
//! Decoders select their own evidenced layout; this is not a universal decoder.
use super::reader::Reader;
use crate::diagnostic::{ErrorCode, ParseError};
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

impl PnmHeader {
    /// Decodes the identifier/version/capture-time layout used by RxMER.
    ///
    /// Only use this reader for formats confirmed to share this layout:
    /// four identifier bytes, two version bytes, then a big-endian u32 time.
    /// Each successful field advances the reader; errors do not roll back
    /// previously read fields. Type and version are checked before reading time.
    ///
    /// # Arguments
    /// * `reader` - Cursor positioned at the start of this header.
    /// * `expected_file_type` - Identifier accepted by the owning parser.
    /// * `expected_version` - Exact version supported by the owning parser.
    ///
    /// # Returns
    /// An owned header with capture time present, leaving the cursor after it.
    ///
    /// # Errors
    /// Reports Truncated for incomplete fields, UnsupportedType for a different
    /// identifier, or UnsupportedVersion for a different major/minor pair.
    /// Diagnostic offsets are relative to the reader's original input.
    pub fn parse_with_capture_time(
        reader: &mut Reader<'_>,
        expected_file_type: PnmFileType,
        expected_version: PnmFormatVersion,
    ) -> Result<Self, ParseError> {
        let file_type_offset = reader.position();
        let file_type: PnmFileType = reader.read_bytes("file_type")?;
        if file_type != expected_file_type {
            return Err(ParseError {
                code: ErrorCode::UnsupportedType,
                offset: file_type_offset,
                field: "file_type",
            });
        }
        let version_offset = reader.position();
        let format_version = PnmFormatVersion {
            major: reader.read_u8("major_version")?,
            minor: reader.read_u8("minor_version")?,
        };
        if format_version != expected_version {
            return Err(ParseError {
                code: ErrorCode::UnsupportedVersion,
                offset: version_offset,
                field: "format_version",
            });
        }
        let capture_time: CaptureTime = reader.read_u32_be("capture_time")?;
        Ok(Self {
            file_type,
            format_version,
            capture_time: Some(capture_time),
        })
    }
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
    fn header_reader_preserves_following_fields() {
        let mut reader = Reader::new(b"TEST\x01\x00\x12\x34\x56\x78\xab");
        let header = PnmHeader::parse_with_capture_time(
            &mut reader,
            *b"TEST",
            PnmFormatVersion { major: 1, minor: 0 },
        )
        .unwrap();
        assert_eq!(header.file_type, *b"TEST");
        assert_eq!(
            header.format_version,
            PnmFormatVersion { major: 1, minor: 0 }
        );
        assert_eq!(header.capture_time, Some(0x12345678));
        assert_eq!(reader.position(), 10);
        assert_eq!(reader.read_u8("next_field").unwrap(), 0xab);
    }

    #[test]
    fn header_validation_precedes_later_truncation() {
        for (bytes, expected_code, expected_offset, expected_field) in [
            (&b"!FAIL"[..], ErrorCode::UnsupportedType, 1, "file_type"),
            (
                &b"!TEST\x02\x00"[..],
                ErrorCode::UnsupportedVersion,
                5,
                "format_version",
            ),
            (
                &b"!TEST\x01\x00\x12"[..],
                ErrorCode::Truncated,
                7,
                "capture_time",
            ),
        ] {
            let mut reader = Reader::new(bytes);
            reader.read_u8("prefix").unwrap();
            let failure = PnmHeader::parse_with_capture_time(
                &mut reader,
                *b"TEST",
                PnmFormatVersion { major: 1, minor: 0 },
            )
            .unwrap_err();
            assert_eq!(failure.code, expected_code);
            assert_eq!(failure.offset, expected_offset);
            assert_eq!(failure.field, expected_field);
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
