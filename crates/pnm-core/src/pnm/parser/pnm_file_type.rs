/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Central identities and routing; only evidenced header layouts are enabled.
use super::pnm_header::PnmFormatVersion;
use crate::pnm::data_type::PnmFileType;

/// Recognized PyPNM identities, independent of decoder availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnmFileKind {
    SymbolCapture,
    ChannelEstimate,
    ConstellationDisplay,
    RxMer,
    Histogram,
    UpstreamPreEqualization,
    UpstreamPreEqualizationLastUpdate,
    FecSummary,
    SpectrumAnalysis,
    ModulationProfile,
    LatencyReport,
}
/// Common header layout, selected only after identity/version recognition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeaderLayout {
    WithCaptureTime,
    WithoutCaptureTime,
}
/// Routing identity; shared routes do not erase distinct file identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PnmDecoderKind {
    RxMer,
    UpstreamPreEqualization,
    Unsupported,
}
/// Single identity descriptor consumed by header, dispatch and output code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDescriptor {
    pub signature: PnmFileType,
    pub kind: PnmFileKind,
    pub tag: &'static str,
    pub prefix: &'static str,
    pub header_version: Option<PnmFormatVersion>,
    /// Enabled for version 1.0 only; None means layout not yet verified here.
    pub layout: Option<HeaderLayout>,
    pub decoder: PnmDecoderKind,
}

/// PyPNM PNN identity vocabulary; recognition alone does not enable decoding.
/// RxMER: I27 Table 68 / I13 Table 101. FEC: I27 Table 71 / I13 Table 104.
pub const FILE_TYPES: &[FileDescriptor] = &[
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 1],
        kind: PnmFileKind::SymbolCapture,
        tag: "PNN1",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 2],
        kind: PnmFileKind::ChannelEstimate,
        tag: "PNN2",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 3],
        kind: PnmFileKind::ConstellationDisplay,
        tag: "PNN3",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 4],
        kind: PnmFileKind::RxMer,
        tag: "PNN4",
        prefix: "PNN",
        header_version: Some(PnmFormatVersion { major: 1, minor: 0 }),
        layout: Some(HeaderLayout::WithCaptureTime),
        decoder: PnmDecoderKind::RxMer,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 5],
        kind: PnmFileKind::Histogram,
        tag: "PNN5",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 6],
        kind: PnmFileKind::UpstreamPreEqualization,
        tag: "PNN6",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::UpstreamPreEqualization,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 7],
        kind: PnmFileKind::UpstreamPreEqualizationLastUpdate,
        tag: "PNN7",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::UpstreamPreEqualization,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 8],
        kind: PnmFileKind::FecSummary,
        tag: "PNN8",
        prefix: "PNN",
        header_version: Some(PnmFormatVersion { major: 1, minor: 0 }),
        layout: Some(HeaderLayout::WithoutCaptureTime),
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 9],
        kind: PnmFileKind::SpectrumAnalysis,
        tag: "PNN9",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    FileDescriptor {
        signature: [0x50, 0x4e, 0x4e, 10],
        kind: PnmFileKind::ModulationProfile,
        tag: "PNN10",
        prefix: "PNN",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
    // I27 Table 83 identifies LLD01; its layout/payload are not enabled here.
    FileDescriptor {
        signature: [0x4c, 0x4c, 0x44, 1],
        kind: PnmFileKind::LatencyReport,
        tag: "LLD01",
        prefix: "LLD",
        header_version: None,
        layout: None,
        decoder: PnmDecoderKind::Unsupported,
    },
];

/// Resolves an exact binary signature without text normalization.
///
/// # Arguments
/// * `signature` - Original four-byte identifier, including the numeric last byte.
/// # Returns
/// The shared descriptor, or None for unrecognized identities.
pub fn identify(signature: PnmFileType) -> Option<&'static FileDescriptor> {
    FILE_TYPES
        .iter()
        .find(|descriptor| descriptor.signature == signature)
}

/// Resolves a human-readable tag using PyPNM-style case and numeric normalization.
///
/// # Arguments
/// * `tag` - Text such as PNN4, PNN10 or LLD1, with optional surrounding whitespace.
/// # Returns
/// The central descriptor; unknown prefixes, nondecimal or oversized suffixes fail.
/// This normalization never applies to binary signatures.
pub fn identify_tag(tag: &str) -> Option<&'static FileDescriptor> {
    let tag = tag.trim();
    let prefix = tag.get(..3)?;
    let suffix = tag.get(3..)?;
    if suffix.is_empty() || !suffix.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let discriminator = suffix.parse::<u8>().ok()?;
    FILE_TYPES.iter().find(|descriptor| {
        descriptor.prefix.eq_ignore_ascii_case(prefix) && descriptor.signature[3] == discriminator
    })
}
