/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use pnm::pnm::parser::{cm_ds_hist::CmDsHist, parse_limits::ParseLimits};
use viewer::capture::{chart::MeasurementView, Capture, MAX_VIEW_SAMPLES};

#[test]
fn histogram_fixture_has_exact_bars_and_unchanged_json() {
    let capture =
        Capture::from_bytes(pnm_fixtures::HISTOGRAM.to_vec(), "histogram.bin".into()).unwrap();
    let metadata = capture.chart_metadata().unwrap();
    assert!(metadata.histogram);
    assert_eq!(metadata.title, "Downstream Histogram");
    assert_eq!(metadata.sample_count, 256);
    assert_eq!(metadata.chips.len(), 1);
    assert_eq!(metadata.chips[0].title, "MAC");
    let view = capture.chart_range(0, 256).unwrap();
    assert!(view.points.is_empty());
    let histogram = view.histogram.unwrap();
    assert_eq!(histogram.bars.len(), 256);
    assert!(!histogram.grouped);
    assert_eq!(histogram.total_hits, 3_103_784_960);
    assert_eq!(histogram.maximum, 97_294_729);
    assert_eq!(histogram.bars[127].hit_count, 97_294_729);
    let raw = CmDsHist::parse(pnm_fixtures::HISTOGRAM, ParseLimits::default()).unwrap();
    let exported: serde_json::Value = serde_json::from_str(&capture.json().unwrap()).unwrap();
    assert_eq!(exported, serde_json::to_value(&raw).unwrap());
    assert_eq!(exported["symmetry"], 2);
    assert_eq!(exported["dwell_count_values"][0], 16_777_216);
    for (index, (bar, count)) in histogram.bars.iter().zip(raw.hit_counts()).enumerate() {
        assert_eq!(bar.first_bin, index);
        assert_eq!(bar.bin_count, 1);
        assert_eq!(bar.hit_count, u64::from(count));
    }
    assert!(capture
        .chart_metadata_for(MeasurementView::GroupDelay)
        .is_err());
    assert!(capture
        .chart_range_for(MeasurementView::IqScatter, 0, 256)
        .is_err());
    assert!(capture.metadata().is_err());
}
fn synthetic(counts: &[u32]) -> Capture {
    let mut bytes = pnm_fixtures::HISTOGRAM[..17].to_vec();
    bytes.extend_from_slice(&0u32.to_be_bytes()); // No dwell records in this synthetic file.
    bytes.extend_from_slice(&((counts.len() * 4) as u32).to_be_bytes());
    for count in counts {
        bytes.extend_from_slice(&count.to_be_bytes());
    }
    Capture::from_bytes(bytes, "synthetic.bin".into()).unwrap()
}
#[test]
fn grouping_preserves_hits_and_zoom_exposes_original_bins() {
    let capture = synthetic(&vec![u32::MAX; 4097]);
    let histogram = capture.chart_range(0, 4097).unwrap().histogram.unwrap();
    assert!(histogram.bars.len() <= MAX_VIEW_SAMPLES);
    assert!(histogram.grouped);
    assert_eq!(histogram.total_hits, 17_596_481_007_615);
    assert_eq!(
        histogram
            .bars
            .iter()
            .map(|bar| bar.bin_count)
            .sum::<usize>(),
        4097
    );
    assert_eq!(histogram.bars[0].hit_count, 21_474_836_475);
    let detail = capture.chart_range(4095, 2).unwrap().histogram.unwrap();
    assert!(!detail.grouped);
    assert_eq!(detail.bars[0].first_bin, 4095);
    assert_eq!(detail.bars[1].hit_count, 4_294_967_295);
    assert_eq!(detail.total_hits, 8_589_934_590);
    assert!(capture.chart_range(4097, 1).is_err());
    assert!(capture.chart_range(usize::MAX, 1).is_err());
}
#[test]
fn zero_single_and_empty_histograms_have_safe_bounds() {
    for counts in [vec![], vec![0], vec![0, 0, 0]] {
        let capture = synthetic(&counts);
        let histogram = capture
            .chart_range(0, counts.len())
            .unwrap()
            .histogram
            .unwrap();
        assert_eq!(histogram.maximum, 1);
        assert_eq!(histogram.total_hits, 0);
        assert_eq!(histogram.bars.len(), counts.len());
    }
    let histogram = synthetic(&[42])
        .chart_range(0, 1)
        .unwrap()
        .histogram
        .unwrap();
    assert_eq!(histogram.maximum, 42);
    assert_eq!(histogram.total_hits, 42);
}
