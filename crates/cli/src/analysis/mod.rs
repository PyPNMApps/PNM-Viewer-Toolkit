/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Thin command-line options for the core RxMER analysis serializer.
use super::{Failure, DECODE_ERROR, UNSUPPORTED_ERROR, USAGE_ERROR};
use pnm::{
    common::{json::DEFAULT_MAX_JSON_BYTES, signal_processing::bit_loading::BitLoadingModel},
    pnm::{
        analysis::rxmer::{RxMerAnalysisRequest, RxMerBitLoadingAnalysis},
        parser::pnm_file::DecodedPnm,
    },
};
use std::{collections::BTreeSet, ffi::OsString};

#[derive(Default)]
pub(super) struct Options {
    pub enabled: bool,
    request: RxMerAnalysisRequest,
    seen: BTreeSet<String>,
}
impl Options {
    /// Consumes recognized analysis switches, leaving input/output options to main.
    /// # Arguments
    /// * `argument` - Current OS argument, preserved for filename handling otherwise.
    /// * `arguments` - Remaining arguments; value options consume one item.
    /// # Returns
    /// True for an analysis option, false for an unrelated argument.
    /// # Errors
    /// Reports repeated options, absent/non-UTF-8 values or unsupported models/numbers.
    pub(super) fn parse(
        &mut self,
        argument: &OsString,
        arguments: &mut impl Iterator<Item = OsString>,
    ) -> Result<bool, Failure> {
        let Some(name) = argument.to_str() else {
            return Ok(false);
        };
        if ![
            "--rxmer-bit-loading",
            "--model",
            "--margin-db",
            "--max-bit-loading",
            "--start-hz",
            "--stop-hz",
        ]
        .contains(&name)
        {
            return Ok(false);
        }
        if !self.seen.insert(name.to_owned()) {
            return Err(Failure(
                USAGE_ERROR,
                format!("{name} specified more than once"),
            ));
        }
        if name == "--rxmer-bit-loading" {
            self.enabled = true;
            return Ok(true);
        }
        let value = arguments
            .next()
            .and_then(|value| value.into_string().ok())
            .ok_or_else(|| Failure(USAGE_ERROR, format!("{name} requires a value")))?;
        let invalid = || Failure(USAGE_ERROR, format!("invalid value for {name}; see --help"));
        match name {
            "--model" => {
                self.request.model = BitLoadingModel::from_name(&value).ok_or_else(invalid)?
            }
            "--margin-db" => {
                self.request.extra_margin_db = value.parse().map_err(|_| invalid())?;
                if !self.request.extra_margin_db.is_finite() || self.request.extra_margin_db < 0.0 {
                    return Err(invalid());
                }
            }
            "--max-bit-loading" => {
                self.request.maximum_bit_loading = value.parse().map_err(|_| invalid())?;
                if !pnm::common::signal_processing::bit_loading::is_data_bit_loading(
                    self.request.maximum_bit_loading,
                ) {
                    return Err(invalid());
                }
            }
            "--start-hz" => {
                self.request.start_frequency_hz = Some(value.parse().map_err(|_| invalid())?)
            }
            "--stop-hz" => {
                self.request.stop_frequency_hz = Some(value.parse().map_err(|_| invalid())?)
            }
            _ => unreachable!(),
        }
        Ok(true)
    }
    /// Checks analysis-mode conflicts before opening the input or writing output.
    /// # Arguments
    /// * `header_only` - Whether the mutually exclusive common-header mode is active.
    /// # Returns
    /// Success for an independent enabled analysis or ordinary decoding request.
    /// # Errors
    /// Rejects unused analysis settings or a header/analysis combination.
    pub(super) fn validate(&self, header_only: bool) -> Result<(), Failure> {
        if (header_only && self.enabled)
            || (!self.enabled && !self.seen.is_empty())
            || (self.request.start_frequency_hz.is_some()
                != self.request.stop_frequency_hz.is_some())
        {
            return Err(Failure(
                USAGE_ERROR,
                "Use --rxmer-bit-loading with paired start/stop Hz, without --header-only.".into(),
            ));
        }
        Ok(())
    }
    /// Delegates full-resolution output to the shared core implementation.
    /// # Arguments
    /// * `decoded` - Centrally identified and validated input capture.
    /// # Returns
    /// Core analysis JSON, unchanged by the CLI.
    /// # Errors
    /// Rejects non-RxMER captures, invalid ranges or output exceeding the JSON limit.
    pub(super) fn json(&self, decoded: DecodedPnm<'_>) -> Result<String, Failure> {
        let DecodedPnm::RxMer(capture) = decoded else {
            return Err(Failure(
                UNSUPPORTED_ERROR,
                "Bit-loading analysis requires an RxMER capture.".into(),
            ));
        };
        RxMerBitLoadingAnalysis::calculate(&capture, self.request)
            .map_err(|error| Failure(DECODE_ERROR, error.to_string()))?
            .to_json(DEFAULT_MAX_JSON_BYTES)
            .map_err(|error| Failure(DECODE_ERROR, error.to_string()))
    }
}
