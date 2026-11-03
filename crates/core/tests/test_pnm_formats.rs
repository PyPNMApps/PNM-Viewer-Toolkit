/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Real PyPNM fixtures plus explicitly synthetic malformed and boundary captures.
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef,
        cm_symbol_capture::CmSymbolCapture,
        cm_us_ofdma_pre_eq::CmUsOfdmaPreEq,
        parse_limits::ParseLimits,
        pnm_file::{DecodedPnm, PnmFile},
        pnm_file_type::{identify_tag, HeaderLayout},
        pnm_header::HasPnmHeader,
    },
};
use serde_json::{json, Value};

const CHANNEL: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/channel_estimation.bin");
const CONSTELLATION: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/const_display.bin");
const HISTOGRAM: &[u8] = include_bytes!("../../../tests/fixtures/pypnm/tests/files/histogram.bin");
const UPSTREAM: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/us_pre_equalizer_coef.bin");
const LAST_UPDATE: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/us_pre_equalizer_coef_last.bin");
const FEC: &[u8] = include_bytes!("../../../tests/fixtures/pypnm/tests/files/fec_summary.bin");
const SPECTRUM: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/spectrum_analyzer.bin");
const MODULATION: &[u8] =
    include_bytes!("../../../tests/fixtures/pypnm/tests/files/modulation_profile.bin");
const FIXTURES: &[&[u8]] = &[
    CHANNEL,
    CONSTELLATION,
    HISTOGRAM,
    UPSTREAM,
    LAST_UPDATE,
    FEC,
    SPECTRUM,
    MODULATION,
];
const JSON_BUDGET: usize = 4_000_000;

fn decode(bytes: &[u8]) -> DecodedPnm<'_> {
    PnmFile::from_bytes(bytes)
        .decode(ParseLimits::default())
        .unwrap()
}
fn json_capture(bytes: &[u8]) -> Value {
    serde_json::from_str(&decode(bytes).to_json(JSON_BUDGET).unwrap()).unwrap()
}
fn header(tag: &str) -> Vec<u8> {
    let descriptor = identify_tag(tag).unwrap();
    let mut bytes = descriptor.signature.to_vec();
    bytes.extend_from_slice(&[1, 0]);
    if descriptor.layout == Some(HeaderLayout::WithCaptureTime) {
        bytes.extend_from_slice(&0_u32.to_be_bytes());
    }
    bytes
}
fn failure(bytes: &[u8]) -> ErrorCode {
    PnmFile::from_bytes(bytes)
        .decode(ParseLimits::default())
        .unwrap_err()
        .code
}

#[test]
fn golden_complex_fixtures_and_borrowed_samples() {
    // Expected signed fixed-point pairs independently decoded from fixture bytes.
    for (bytes, key, count, first, last) in [
        (
            CHANNEL,
            "iq",
            7480,
            [-0.216552734375, -1.1671142578125],
            [-0.587890625, 0.593994140625],
        ),
        (
            CONSTELLATION,
            "iq",
            8192,
            [0.376953125, -1.15625],
            [1.146484375, 0.0859375],
        ),
        (
            UPSTREAM,
            "iq",
            1776,
            [0.642822265625, -0.6092529296875],
            [-0.8643798828125, 0.8048095703125],
        ),
        (
            LAST_UPDATE,
            "iq",
            1776,
            [0.03173828125, -0.169921875],
            [-0.17144775390625, 0.01422119140625],
        ),
    ] {
        let output = json_capture(bytes);
        let values = output[key].as_array().unwrap();
        assert_eq!(values.len(), count);
        assert_eq!(values[0], json!(first));
        assert_eq!(values[count - 1], json!(last));
        assert_eq!(output["subcarrier_spacing"], 25000);
    }
    let channel = CmDsOfdmChanEstimateCoef::parse(CHANNEL, ParseLimits::default()).unwrap();
    assert_eq!(channel.raw_iq().as_ptr(), CHANNEL[28..].as_ptr());
    assert_eq!(
        channel.pnm_header().pnm_data().as_ptr(),
        CHANNEL[10..].as_ptr()
    );
    let upstream = CmUsOfdmaPreEq::parse(UPSTREAM, ParseLimits::default()).unwrap();
    assert_eq!(upstream.raw_iq().as_ptr(), UPSTREAM[34..].as_ptr());
    assert_eq!(
        CmDsOfdmChanEstimateCoef::parse(UPSTREAM, ParseLimits::default())
            .unwrap_err()
            .code,
        ErrorCode::UnsupportedType
    );
}

#[test]
fn golden_histogram_fec_spectrum_and_profiles() {
    let histogram = json_capture(HISTOGRAM);
    assert_eq!(histogram["symmetry"], 2); // Preserve observed value rather than coercing to bool.
    assert_eq!(histogram["dwell_count_values"], json!([16777216]));
    assert_eq!(histogram["hit_count_values"].as_array().unwrap().len(), 256);
    let fec = json_capture(FEC);
    assert!(fec["pnm_header"]["capture_time"].is_null());
    let profiles = fec["fec_summary_data"].as_array().unwrap();
    assert_eq!(profiles.len(), 5);
    assert_eq!(profiles[0]["profile_id"], 255);
    assert_eq!(profiles[0]["number_of_sets"], 600);
    assert_eq!(
        profiles[0]["codeword_entries"][0],
        json!({"timestamp":1762636604,"total_codewords":44444,"corrected":0,"uncorrectable":0})
    );
    assert_eq!(profiles[4]["codeword_entries"][599]["corrected"], 1);
    let spectrum = json_capture(SPECTRUM);
    let segments = spectrum["amplitude_bin_segments_float"].as_array().unwrap();
    assert_eq!(segments.len(), 81);
    assert_eq!(segments[0].as_array().unwrap().len(), 256);
    assert_eq!(segments[0][0], -22.8);
    assert_eq!(segments[80][255], -77.3);
    let modulation = json_capture(MODULATION);
    let profiles = modulation["profiles"].as_array().unwrap();
    assert_eq!(profiles.len(), 4);
    assert_eq!(profiles[0]["profile_id"], 3);
    assert_eq!(
        profiles[0]["schemes"][0],
        json!({"schema_type":"range","modulation_order":12,"num_subcarriers":110})
    );
}

#[test]
fn all_fixtures_reject_truncation_trailing_bytes_versions_and_limits() {
    for bytes in FIXTURES {
        // Every fixed-header boundary plus representative nested-record boundaries.
        for end in (0..40.min(bytes.len())).chain([bytes.len() / 2, bytes.len() - 1]) {
            assert!(
                PnmFile::from_bytes(&bytes[..end])
                    .decode(ParseLimits::default())
                    .is_err(),
                "accepted truncation at {end}"
            );
        }
        let mut extra = bytes.to_vec();
        extra.push(0);
        assert_eq!(failure(&extra), ErrorCode::LengthMismatch);
        let mut version = bytes.to_vec();
        version[5] = 1;
        assert_eq!(failure(&version), ErrorCode::UnsupportedVersion);
        assert_eq!(
            PnmFile::from_bytes(bytes)
                .decode(ParseLimits { max_samples: 0 })
                .unwrap_err()
                .code,
            ErrorCode::LimitExceeded
        );
        let capture = decode(bytes);
        assert!(capture.to_json(1).is_err());
        let output = capture.to_json(JSON_BUDGET).unwrap();
        assert!(capture.to_json(output.len() - 1).is_err());
        assert_eq!(capture.to_json(output.len()).unwrap(), output);
    }
}

#[test]
fn complex_signed_boundaries_exclusion_and_distinct_scales() {
    let mut channel = CHANNEL[..28].to_vec();
    channel[24..28].copy_from_slice(&12_u32.to_be_bytes());
    channel.extend_from_slice(&[0x80, 0, 0x80, 0, 0x80, 0, 0x7f, 0xff, 0x24, 0, 0xf8, 0]);
    assert_eq!(
        json_capture(&channel)["iq"],
        json!([null, [-4.0, 3.9998779296875], [1.125, -0.25]])
    );
    for (tag, expected) in [
        ("PNN6", json!([[1.0, -1.0]])),
        ("PNN7", json!([[0.5, -0.5]])),
    ] {
        let mut bytes = UPSTREAM[..34].to_vec();
        bytes[..4].copy_from_slice(&identify_tag(tag).unwrap().signature);
        bytes[30..34].copy_from_slice(&4_u32.to_be_bytes());
        bytes.extend_from_slice(&[0x20, 0, 0xe0, 0]);
        assert_eq!(json_capture(&bytes)["iq"], expected);
    }
    channel[24..28].copy_from_slice(&11_u32.to_be_bytes());
    assert_eq!(failure(&channel), ErrorCode::LengthMismatch);
}

#[test]
fn nested_counts_lengths_and_spectrum_partial_segment() {
    let mut histogram = HISTOGRAM.to_vec();
    histogram[17..21].copy_from_slice(&3_u32.to_be_bytes());
    assert_eq!(failure(&histogram), ErrorCode::LengthMismatch);
    let mut fec = FEC.to_vec();
    fec[14] += 1;
    assert_eq!(failure(&fec), ErrorCode::Truncated);
    let mut spectrum = SPECTRUM[..39].to_vec();
    spectrum[35..39].copy_from_slice(&6_u32.to_be_bytes());
    spectrum.extend_from_slice(&[0x80, 0, 0, 0, 0x7f, 0xff]);
    assert_eq!(
        json_capture(&spectrum)["amplitude_bin_segments_float"],
        json!([[-327.68, 0.0, 327.67]])
    );
    spectrum[29..31].copy_from_slice(&0_u16.to_be_bytes());
    assert_eq!(failure(&spectrum), ErrorCode::LengthMismatch);
    let mut profiles = MODULATION.to_vec();
    profiles[17] += 1;
    assert_eq!(failure(&profiles), ErrorCode::Truncated);
    profiles = MODULATION.to_vec();
    profiles[32] = 255;
    assert_eq!(failure(&profiles), ErrorCode::UnsupportedLayout);
}

#[test]
fn synthetic_skip_modulation_preserves_unknown_assignments() {
    let mut bytes = MODULATION[..29].to_vec();
    bytes[17] = 1;
    bytes[25..29].copy_from_slice(&8_u32.to_be_bytes());
    bytes.extend_from_slice(&[7, 0, 5, 1, 255, 2, 0, 9]);
    let output = json_capture(&bytes);
    assert_eq!(
        output["profiles"][0]["schemes"][0],
        json!({"schema_type":"skip","main_modulation_order":255,"skip_modulation_order":2,"num_subcarriers":9})
    );
    for end in 29..bytes.len() {
        assert!(PnmFile::from_bytes(&bytes[..end])
            .decode(ParseLimits::default())
            .is_err());
    }
}

fn latency_fixture() -> Vec<u8> {
    // Synthetic LLD1 v1.0 per I27 Tables 83/84 and I13 Tables 116/117.
    let mut bytes = header("LLD01");
    bytes.extend_from_slice(&[1, 2, 3, 4, 5, 6, 1]); // MAC and one flow.
    bytes.extend_from_slice(&1050_u32.to_be_bytes());
    bytes.extend_from_slice(b"LLService1\0\0\0\0\0\0");
    bytes.push(1); // One edge, two histogram bins.
    bytes.extend_from_slice(&100_u16.to_be_bytes());
    bytes.extend_from_slice(&1_u16.to_be_bytes()); // One snapshot.
    bytes.extend_from_slice(&1456252718_u32.to_be_bytes());
    bytes.extend_from_slice(&1456252719_u32.to_be_bytes());
    bytes.extend_from_slice(&u64::MAX.to_be_bytes());
    bytes.extend_from_slice(&10_u64.to_be_bytes());
    for value in [57_u32, 2575, 9, 3, 4, 0, 5, 6] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    bytes
}

#[test]
fn latency_synthetic_layout_u64_counts_units_and_all_truncations() {
    let bytes = latency_fixture();
    let output = json_capture(&bytes);
    assert!(output["pnm_header"]["capture_time"].is_null());
    let flow = &output["latency_data"][0];
    assert_eq!(flow["sf_id"], 1050);
    assert_eq!(flow["bin_edges_hundredths_ms"], json!([100]));
    assert_eq!(flow["snapshots"][0]["bin_counts"], json!([u64::MAX, 10]));
    assert_eq!(flow["snapshots"][0]["max_latency_us"], 57);
    assert_eq!(flow["snapshots"][0]["ce_marked_ect1_pkts"], 6);
    for end in 0..bytes.len() {
        assert!(PnmFile::from_bytes(&bytes[..end])
            .decode(ParseLimits::default())
            .is_err());
    }
    let mut extra = bytes.clone();
    extra.push(0);
    assert_eq!(failure(&extra), ErrorCode::LengthMismatch);
    assert_eq!(
        PnmFile::from_bytes(&bytes)
            .decode(ParseLimits { max_samples: 4 })
            .unwrap_err()
            .code,
        ErrorCode::LimitExceeded
    );
}

#[test]
fn symbol_capture_is_only_a_header_stub() {
    let mut bytes = header("PNN1");
    bytes.extend_from_slice(&[42, 43]);
    let stub = CmSymbolCapture::parse(&bytes).unwrap();
    assert_eq!(stub.pnm_header().pnm_data(), &[42, 43]);
    assert_eq!(failure(&bytes), ErrorCode::UnsupportedDecoder);
    assert_eq!(
        CmSymbolCapture::parse(CHANNEL).unwrap_err().code,
        ErrorCode::UnsupportedType
    );
}
