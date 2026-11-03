/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Borrowed wire arrays and Serde views; exclusions are PNM interpretation.
use crate::pnm::{
    data_type::{ComplexValue, CounterValue},
    lib::fixed_point_decoder::{decode_complex, QFormat},
};
use serde::{Serialize, Serializer};

pub(crate) const COMPLEX_BYTES: usize = 4;
pub(crate) const COUNTER_BYTES: usize = 4;
pub(crate) const AMPLITUDE_BYTES: usize = 2;
const EXCLUDED_COMPONENT: i16 = i16::MIN;

/// Borrowed fixed-point pairs. Channel estimates alone recognize exclusion pairs.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ComplexValues<'input> {
    pub bytes: &'input [u8],
    pub format: QFormat,
    pub excluded_pair: bool,
}
impl ComplexValues<'_> {
    /// Lazily converts signed complex components, preserving exclusion markers.
    ///
    /// # Returns
    /// Fixed-point real/imaginary pairs or None for configured exclusions.
    pub(crate) fn values(&self) -> impl Iterator<Item = Option<ComplexValue>> + '_ {
        self.bytes.chunks_exact(COMPLEX_BYTES).map(|pair| {
            let real = i16::from_be_bytes([pair[0], pair[1]]);
            let imaginary = i16::from_be_bytes([pair[2], pair[3]]);
            if self.excluded_pair && real == EXCLUDED_COMPONENT && imaginary == EXCLUDED_COMPONENT {
                None
            } else {
                Some(decode_complex(real, imaginary, self.format))
            }
        })
    }
}
impl Serialize for ComplexValues<'_> {
    /// Streams decoded pairs without allocating a sample array.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// A sequence containing null for excluded channel-estimate samples.
    /// # Errors
    /// Propagates destination errors.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(self.values())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CounterValues<'input>(pub &'input [u8]);
impl CounterValues<'_> {
    /// Lazily reads validated unsigned counters.
    ///
    /// # Returns
    /// Counters in original capture order.
    pub(crate) fn values(&self) -> impl Iterator<Item = CounterValue> + '_ {
        self.0
            .chunks_exact(COUNTER_BYTES)
            .map(|bytes| u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}
impl Serialize for CounterValues<'_> {
    /// Streams unsigned counters from borrowed bytes.
    ///
    /// # Arguments
    /// * `serializer` - Output destination.
    /// # Returns
    /// A sequence of counters.
    /// # Errors
    /// Propagates destination errors.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        serializer.collect_seq(self.values())
    }
}
