/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Two-input Pre-EQ analysis delegates precheck and prediction to core Rust.
use super::{Failure, DECODE_ERROR, UNSUPPORTED_ERROR, USAGE_ERROR};
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::{
        analysis::upstream_pre_equalization::comparison::{
            precheck, CorrectionMode, PreEqComparison,
        },
        parser::{
            parse_limits::ParseLimits,
            pnm_file::{DecodedPnm, PnmFile},
        },
    },
};
use std::{ffi::OsString, path::PathBuf};
#[derive(Default)]
pub(super) struct Options {
    pub correction: Option<PathBuf>,
    mode: Option<CorrectionMode>,
    pub precheck_only: bool,
}
impl Options {
    /// Consumes only Pre-EQ switches, preserving OS-native paths.
    /// # Arguments
    /// * `argument` - Current option or unconsumed filename.
    /// * `arguments` - Remaining OS arguments; recognized value options consume one.
    /// # Returns
    /// Whether the option was consumed.
    /// # Errors
    /// Reports absent, invalid or repeated option values as usage failures.
    pub(super) fn parse(
        &mut self,
        argument: &OsString,
        arguments: &mut impl Iterator<Item = OsString>,
    ) -> Result<bool, Failure> {
        if argument == "--pre-eq-update" {
            if self.correction.is_some() {
                return Err(Failure(USAGE_ERROR, "Repeated --pre-eq-update.".into()));
            }
            self.correction = Some(
                arguments
                    .next()
                    .ok_or_else(|| Failure(USAGE_ERROR, "--pre-eq-update requires a path.".into()))?
                    .into(),
            );
        } else if argument == "--update-mode" {
            if self.mode.is_some() {
                return Err(Failure(USAGE_ERROR, "Repeated --update-mode.".into()));
            }
            self.mode = Some(
                arguments
                    .next()
                    .and_then(|value| value.into_string().ok())
                    .and_then(|value| CorrectionMode::from_name(&value))
                    .ok_or_else(|| {
                        Failure(USAGE_ERROR, "--update-mode requires adjust or set.".into())
                    })?,
            );
        } else if argument == "--precheck-only" {
            if self.precheck_only {
                return Err(Failure(USAGE_ERROR, "Repeated --precheck-only.".into()));
            }
            self.precheck_only = true;
        } else {
            return Ok(false);
        }
        Ok(true)
    }
    /// Enforces explicit mode or precheck-only, and excludes conflicting analyses.
    /// # Arguments
    /// * `header` - Whether header-only export is active.
    /// * `rxmer` - Whether RxMER analysis is active.
    /// # Returns
    /// Success for consistent options, before any file I/O.
    /// # Errors
    /// Rejects missing modes, orphan options and conflicting analysis modes.
    pub(super) fn validate(&self, header: bool, rxmer: bool) -> Result<(), Failure> {
        if self.correction.is_some() {
            if header
                || rxmer
                || (!self.precheck_only && self.mode.is_none())
                || (self.precheck_only && self.mode.is_some())
            {
                return Err(Failure(USAGE_ERROR,"Use --pre-eq-update with either --update-mode adjust|set or --precheck-only, without other analysis modes.".into()));
            }
        } else if self.mode.is_some() || self.precheck_only {
            return Err(Failure(
                USAGE_ERROR,
                "Pre-EQ options require --pre-eq-update.".into(),
            ));
        }
        Ok(())
    }
    /// Reads the second bounded input and delegates validation/prediction to core.
    /// # Arguments
    /// * `original` - Decoded baseline backed by the retained first input buffer.
    /// # Returns
    /// Full-resolution analysis or precheck JSON according to validated options.
    /// # Errors
    /// Reports I/O, wrong-type, compatibility and output-budget failures.
    pub(super) fn json(&self, original: DecodedPnm<'_>) -> Result<String, Failure> {
        let bytes = super::read_input(self.correction.as_ref().unwrap())?;
        let correction = PnmFile::from_bytes(&bytes).decode(ParseLimits::default())?;
        let (
            DecodedPnm::UpstreamPreEqualization(original),
            DecodedPnm::UpstreamPreEqualization(correction),
        ) = (original, correction)
        else {
            return Err(Failure(
                UNSUPPORTED_ERROR,
                "Both inputs must be OFDMA Pre-EQ captures.".into(),
            ));
        };
        if self.precheck_only {
            let pair = precheck(&original, &correction)
                .map_err(|error| Failure(DECODE_ERROR, error.to_string()))?;
            pnm::common::json::to_json(&pair, DEFAULT_MAX_JSON_BYTES)
                .map_err(|error| Failure(DECODE_ERROR, error.to_string()))
        } else {
            PreEqComparison::calculate(&original, &correction, self.mode.unwrap())
                .map_err(|error| Failure(DECODE_ERROR, error.to_string()))?
                .to_json(DEFAULT_MAX_JSON_BYTES)
                .map_err(|error| Failure(DECODE_ERROR, error.to_string()))
        }
    }
}
