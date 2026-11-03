/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use super::super::cm_ds_ofdm_rx_mer::decode_rxmer_value;
use super::pnm_header::PnmHeaderParameters;
use crate::pnm::data_type::{ChannelId, FrequencyHz, SampleCount, SubcarrierIdx};
use serde::{ser::SerializeSeq, Serialize, Serializer};

/// Lazy output view of RxMER samples, borrowing unchanged capture bytes.
#[derive(Debug)]
pub struct RxMerValues<'input>(pub(crate) &'input [u8]);
impl Serialize for RxMerValues<'_> {
    /// Serializes quarter-dB readings while preserving unavailable samples.
    ///
    /// # Arguments
    /// * `serializer` - Destination serializer.
    /// # Returns
    /// A sequence of dB numbers and nulls, without allocating a sample vector.
    /// # Errors
    /// Propagates destination serialization failures.
    fn serialize<Output: Serializer>(
        &self,
        serializer: Output,
    ) -> Result<Output::Ok, Output::Error> {
        let mut sequence = serializer.serialize_seq(Some(self.0.len()))?;
        for &raw in self.0 {
            let value = decode_rxmer_value(raw);
            sequence.serialize_element(&value)?;
        }
        sequence.end()
    }
}
/// PyPNM-aligned decoding output. Statistics remain an analysis responsibility.
#[derive(Debug, Serialize)]
pub struct CmDsOfdmRxMerModel<'input> {
    pub pnm_header: PnmHeaderParameters,
    pub channel_id: ChannelId,
    pub mac_address: String,
    pub subcarrier_zero_frequency: FrequencyHz,
    pub first_active_subcarrier_index: SubcarrierIdx,
    /// Hertz in the output model, matching PyPNM.
    pub subcarrier_spacing: FrequencyHz,
    pub data_length: SampleCount,
    pub value_units: &'static str,
    pub values: RxMerValues<'input>,
}
