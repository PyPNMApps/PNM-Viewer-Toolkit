/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmUsOfdmaPreEq.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, parse_limits::ParseLimits},
};
use serde_json::json;
const UPSTREAM: &[u8] = pnm_fixtures::UPSTREAM_PRE_EQ;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmUsOfdmaPreEq::parse(UPSTREAM, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(UPSTREAM)
    );
    assert_fixture_contract(UPSTREAM);
}

#[test]
fn golden_upstream() {
    let output = json_capture(UPSTREAM);
    let values = output["iq"].as_array().unwrap();
    assert_eq!(values.len(), 1776);
    assert_eq!(values[0], json!([0.642822265625, -0.6092529296875]));
    assert_eq!(values[1776 - 1], json!([-0.8643798828125, 0.8048095703125]));
    assert_eq!(output["subcarrier_spacing"], 25000);
}

#[test]
fn golden_last_update() {
    let output = json_capture(LAST_UPDATE);
    let values = output["iq"].as_array().unwrap();
    assert_eq!(values.len(), 1776);
    assert_eq!(values[0], json!([0.03173828125, -0.169921875]));
    assert_eq!(
        values[1776 - 1],
        json!([-0.17144775390625, 0.01422119140625])
    );
    assert_eq!(output["subcarrier_spacing"], 25000);
}

use pnm::pnm::parser::pnm_file_type::identify_tag;
const LAST_UPDATE: &[u8] = pnm_fixtures::UPSTREAM_PRE_EQ_LAST;
#[test]
fn last_update_contract_and_borrowing() {
    assert_fixture_contract(LAST_UPDATE);
    let capture = CmUsOfdmaPreEq::parse(UPSTREAM, ParseLimits::default()).unwrap();
    assert_eq!(capture.raw_iq().as_ptr(), UPSTREAM[34..].as_ptr());
    let capture = CmUsOfdmaPreEq::parse(LAST_UPDATE, ParseLimits::default()).unwrap();
    assert_eq!(capture.raw_iq().as_ptr(), LAST_UPDATE[34..].as_ptr());
    let mut invalid = UPSTREAM.to_vec();
    invalid.push(0);
    assert_eq!(failure(&invalid), ErrorCode::LengthMismatch);
}
#[test]
fn current_and_last_update_select_distinct_scales() {
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
}
