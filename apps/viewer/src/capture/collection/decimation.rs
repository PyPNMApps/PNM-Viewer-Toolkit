/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared joint extrema selection for temporal measurement curves.
use crate::capture::MAX_VIEW_SAMPLES;
/// Chooses common source positions preserving all three curves' extrema and gaps.
/// # Arguments
/// * `source` - Full-resolution points in the selected range.
/// * `value` - Accessor for minimum, average and maximum (field indices 0..3).
/// # Returns
/// Sorted indices bounded by MAX_VIEW_SAMPLES; endpoints and missing intervals
/// are retained without moving measurement calculations into the UI.
pub fn joint_indices<Point>(
    source: &[Point],
    value: impl Fn(&Point, usize) -> Option<f64>,
) -> Vec<usize> {
    let mut indices = Vec::new();
    let width = source.len().div_ceil(MAX_VIEW_SAMPLES / 9).max(1);
    for (bucket_index, bucket) in source.chunks(width).enumerate() {
        let offset = bucket_index * width;
        indices.extend([offset, offset + bucket.len() - 1]);
        for field in 0..3 {
            for minimum in [true, false] {
                if let Some(index) = (0..bucket.len())
                    .filter(|index| value(&bucket[*index], field).is_some())
                    .min_by(|first, last| {
                        let order = value(&bucket[*first], field)
                            .unwrap()
                            .total_cmp(&value(&bucket[*last], field).unwrap());
                        if minimum {
                            order
                        } else {
                            order.reverse()
                        }
                    })
                {
                    indices.push(offset + index);
                }
            }
        }
        if let Some(index) = bucket.iter().position(|point| value(point, 1).is_none()) {
            indices.push(offset + index);
        }
    }
    indices.sort_unstable();
    indices.dedup();
    indices
}
