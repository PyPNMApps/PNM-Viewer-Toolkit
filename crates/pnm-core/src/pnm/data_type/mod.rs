/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared PNM data types and unit definitions.
pub mod units;

/// Four-byte PNM file identifier in its encoded byte order.
/// This alias provides a domain name; it does not validate supported identifiers.
pub type PnmFileType = [u8; 4];

/// Six octets of a MAC address in encoded order.
/// This alias names the representation; it does not validate address semantics.
pub type MacAddress = [u8; 6];

/// Capture timestamp encoded as unsigned epoch seconds; no calendar validation.
pub type CaptureTime = u32;
/// Encoded channel identifier; range semantics belong to the measurement format.
pub type ChannelId = u8;
/// Encoded subcarrier index, matching the RxMER header's 16-bit field.
pub type SubcarrierIdx = u16;
/// Frequency in Hz; arithmetic must be checked against the u32 range.
pub type FrequencyHz = u32;
/// Encoded subcarrier spacing in kHz; no PHY spacing validation is implied.
pub type SubcarrierSpacingKhz = u8;
/// One encoded file-version component; not a DOCSIS protocol version.
pub type FileVersionComponent = u8;
/// Length in bytes as encoded in a 32-bit file field.
pub type DataLengthBytes = u32;
/// Decoder/application sample budget or in-memory sample count.
pub type SampleCount = usize;
/// Decoded RxMER in decibels; unavailable measurements are represented by Option.
pub type RxMerDb = f64;
