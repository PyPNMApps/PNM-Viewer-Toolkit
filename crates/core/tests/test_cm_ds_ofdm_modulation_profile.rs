/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsOfdmModulationProfile.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile, parse_limits::ParseLimits,
    },
};
use serde_json::json;
const MODULATION: &[u8] = pnm_fixtures::MODULATION_PROFILE;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsOfdmModulationProfile::parse(MODULATION, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(MODULATION)
    );
    assert_fixture_contract(MODULATION);
}
#[test]
fn golden_fixture() {
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
fn malformed_lengths_and_boundaries() {
    let mut profiles = MODULATION.to_vec();
    profiles[17] += 1;
    assert_eq!(failure(&profiles), ErrorCode::Truncated);
    profiles = MODULATION.to_vec();
    profiles[32] = 255;
    assert_eq!(failure(&profiles), ErrorCode::UnsupportedLayout);
}
use pnm::pnm::parser::pnm_file::PnmFile;
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
