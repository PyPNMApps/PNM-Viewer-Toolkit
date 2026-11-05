/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared desktop diagnostics without losing parser field/offset information.
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Diagnostic {
    pub code: String,
    pub message: String,
    pub field: Option<String>,
    pub offset: Option<usize>,
}
impl Diagnostic {
    /// Creates an application diagnostic without a binary field location.
    /// # Arguments
    /// * `code` - Stable error category.
    /// * `message` - User-facing explanation.
    /// # Returns
    /// An error suitable for IPC serialization.
    pub fn new(code: &str, message: impl ToString) -> Self {
        Self {
            code: code.into(),
            message: message.to_string(),
            field: None,
            offset: None,
        }
    }
}
impl From<pnm::diagnostic::ParseError> for Diagnostic {
    /// Preserves a parser diagnostic's field and original byte offset.
    fn from(error: pnm::diagnostic::ParseError) -> Self {
        Self {
            code: format!("{:?}", error.code),
            message: error.to_string(),
            field: Some(error.field.into()),
            offset: Some(error.offset),
        }
    }
}
