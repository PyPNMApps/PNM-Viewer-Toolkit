/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    name: "MeasurementPresentation"
    when: windowShown
    visible: true
    width: 900
    height: 450
    Theme { id: appearance }
    property var sourcePoints: []
    MeasurementChart {
        id: chart
        theme: appearance
        width: 850
        height: 400
        onRangeRequested: (start, count) => {
            chart.viewport = {start: start, count: count, total: sourcePoints.length, points: sourcePoints.slice(start, start + count)}
        }
    }
    IqScatterChart {
        id: scatter
        theme: appearance
        visible: false
        width: 850
        height: 400
    }
    FecSummaryChart {
        id: fecChart
        theme: appearance
        width: 900
        height: 430
        visible: false
        property real selectedFirst: -1
        property real selectedLast: -1
        property int selectedProfile: -1
        property int requestedCount: -1
        onTimeRangeRequested: (first, last) => { selectedFirst = first; selectedLast = last }
        onProfileRequested: index => selectedProfile = index
        onRangeRequested: (start, count) => requestedCount = count
    }
    function test_fec_profiles_utc_selection_toggles_and_empty() {
        chart.visible = false
        fecChart.visible = true
        fecChart.metadata = {profiles: [{profile_id: 255}, {profile_id: 0}], profile_index: 1, summary_type: 2}
        fecChart.viewport = {start: 0, count: 3, total: 3, fec_summary: {
            first_timestamp: 100, last_timestamp: 200, total_maximum: 100, error_maximum: 5,
            total_codewords: 200, corrected: 4, uncorrectable: 1,
            last: {timestamp: 200, total_codewords: 100, corrected: 3, uncorrectable: 1},
            points: [{timestamp: 100, total_codewords: 100, corrected: 1, uncorrectable: 0},
                     {timestamp: 200, total_codewords: 100, corrected: 3, uncorrectable: 1, break_before: true}]}}
        compare(findChild(fecChart, "fecProfileSelector").currentIndex, 1)
        compare(fecChart.utc(0), "1970-01-01 00:00:00 UTC")
        compare(fecChart.countLabel(17592186040321), "17592186040321")
        const plot = findChild(fecChart, "fecPlot")
        const usableWidth = plot.width - plot.leftMargin - plot.rightMargin
        const left = plot.leftMargin + usableWidth * 0.25
        const right = plot.leftMargin + usableWidth * 0.75
        mousePress(plot, left, 100)
        mouseMove(plot, right, 100)
        mouseRelease(plot, right, 100)
        fuzzyCompare(fecChart.selectedFirst, 125, 1)
        fuzzyCompare(fecChart.selectedLast, 175, 1)
        fecChart.resetZoom()
        compare(fecChart.requestedCount, 3)
        const toggle = findChild(fecChart, "fecCorrectedToggle")
        mouseClick(toggle)
        compare(toggle.checked, false)
        mouseClick(toggle)
        compare(toggle.checked, true)
        const selector = findChild(fecChart, "fecProfileSelector")
        selector.forceActiveFocus()
        keyClick(Qt.Key_Home)
        keyClick(Qt.Key_Return)
        tryCompare(fecChart, "selectedProfile", 0)
        appearance.preference = 2
        wait(50)
        fecChart.viewport = {start: 0, count: 0, total: 0, fec_summary: {points: [], last: null, first_timestamp: null, last_timestamp: null}}
        wait(50)
        fecChart.visible = false
        appearance.preference = 1
    }
    function test_channel_delay_and_density_presentation() {
        chart.unit = "µs"
        chart.axisLabel = "Group delay"
        chart.minimum = -0.1
        chart.maximum = 0.2
        chart.viewport = {start: 0, count: 2, total: 2, points: [
            {sample_index: 0, frequency_hz: 640000000, value: -0.025},
            {sample_index: 1, frequency_hz: 640025000, value: null}
        ]}
        verify(chart.reading.indexOf("µs") >= 0)
        chart.selectedIndex = 1
        verify(chart.reading.indexOf("Unavailable") >= 0)
        scatter.density = {minimum: -1, maximum: 1, grid_side: 32, valid_count: 5,
            excluded_count: 1, maximum_cell_count: 5, cells: [{column: 16, row: 16, count: 5}]}
        scatter.visible = true
        wait(50)
        verify(findChild(scatter, "iqDensityPlot").width > 0)
        appearance.preference = 1
        wait(50)
        scatter.density = {minimum: -1, maximum: 1, grid_side: 32, valid_count: 0,
            excluded_count: 0, maximum_cell_count: 0, cells: []}
        wait(50)
        scatter.visible = false
    }
    HistogramChart {
        id: histogramChart
        theme: appearance
        // Keep the drawable width divisible by four for integer mouse coordinates.
        width: 849
        height: 400
        visible: false
        onRangeRequested: (start, count) => {
            histogramChart.viewport = {start: start, count: count, total: 256,
                histogram: {bars: [{first_bin: start, bin_count: count, hit_count: 42}], maximum: 42, total_hits: 42, grouped: count > 1}}
        }
    }
    function test_histogram_drag_zoom_and_empty() {
        chart.visible = false
        histogramChart.visible = true
        histogramChart.viewport = {start: 0, count: 256, total: 256}
        histogramChart.resetZoom()
        const plot = findChild(histogramChart, "histogramPlot")
        const usableWidth = plot.width - plot.leftMargin - plot.rightMargin
        const left = plot.leftMargin + usableWidth * 0.25
        const right = plot.leftMargin + usableWidth * 0.75
        mousePress(plot, left, 100)
        mouseMove(plot, right, 100)
        mouseRelease(plot, right, 100)
        compare(histogramChart.viewport.start, 64)
        compare(histogramChart.viewport.count, 128)
        mouseDoubleClickSequence(plot, plot.width / 2, 100)
        compare(histogramChart.viewport.count, 256)
        mouseWheel(plot, plot.width / 2, 100, 0, 120)
        compare(histogramChart.viewport.count, 205)
        appearance.preference = 1
        histogramChart.viewport = {start: 0, count: 0, total: 0, histogram: {bars: [], maximum: 1, total_hits: 0, grouped: false}}
        wait(50)
        histogramChart.visible = false
        chart.visible = true
    }
    function init() {
        chart.visible = true
        histogramChart.visible = false
        scatter.visible = false
        chart.unit = "dB"
        chart.axisLabel = "RxMER"
        chart.minimum = 0
        chart.maximum = 64
        appearance.preference = 0
        appearance.systemScheme = Qt.Light
        chart.viewport = {start: 0, total: 0, points: []}
    }
    function test_system_and_explicit_themes() {
        compare(appearance.dark, false)
        appearance.systemScheme = Qt.Dark
        compare(appearance.dark, true)
        appearance.preference = 2
        compare(appearance.dark, false)
        appearance.preference = 1
        appearance.systemScheme = Qt.Light
        compare(appearance.dark, true)
        appearance.preference = 0
        compare(appearance.dark, false)
    }
    function test_missing_and_page_readings() {
        chart.viewport = {start: 1024, total: 7480, points: [
            {frequency_hz: 665600000, value: 42.75},
            {frequency_hz: 665625000, value: null}
        ]}
        verify(chart.reading.indexOf("#1024") >= 0)
        verify(chart.reading.indexOf("665.600 MHz") >= 0)
        verify(chart.reading.indexOf("42.75 dB") >= 0)
        chart.selectedIndex = 1
        verify(chart.reading.indexOf("Unavailable") >= 0)
        chart.viewport = {start: 0, total: 0, points: []}
        compare(chart.selectedIndex, 0)
        compare(chart.reading, "No samples in this capture")
    }
    function test_spectrum_units_and_negative_values() {
        chart.unit = "dBmV"
        chart.axisLabel = "Amplitude"
        chart.minimum = -100
        chart.maximum = 10
        chart.viewport = {start: 0, count: 2, total: 2, points: [
            {sample_index: 0, frequency_hz: 296264648, value: -22.8},
            {sample_index: 1, frequency_hz: 296293945, value: -77.3}
        ]}
        verify(chart.reading.indexOf("-22.80 dBmV") >= 0)
        chart.selectedIndex = 1
        verify(chart.reading.indexOf("-77.30 dBmV") >= 0)
    }
    function test_drag_frequency_selection() {
        sourcePoints = []
        for (let index = 0; index < 100; index++) {
            sourcePoints.push({frequency_hz: 640000000 + index * 25000, value: 43})
        }
        chart.viewport = {start: 0, count: 100, total: 100, points: sourcePoints}
        const plot = findChild(chart, "measurementPlot")
        const plotWidth = plot.width - plot.leftMargin - plot.rightMargin
        const left = plot.leftMargin + plotWidth * 0.25
        const right = plot.leftMargin + plotWidth * 0.75
        mousePress(plot, left, 100)
        mouseMove(plot, right, 100)
        verify(chart.frequencySelection.indexOf("Frequency selection:") >= 0)
        mouseRelease(plot, right, 100)
        compare(chart.visibleStart, 24)
        compare(chart.visibleCount, 52)
        compare(chart.frequencySelection, "Frequency selection: 640.600 – 641.875 MHz")
        chart.resetZoom()
        mousePress(plot, right, 100)
        mouseMove(plot, left, 100)
        mouseRelease(plot, left, 100)
        compare(chart.visibleStart, 24)
        compare(chart.visibleCount, 52)
        chart.resetZoom()
        mouseClick(plot, left, 100)
        compare(chart.visibleCount, 100)
    }
    function test_mouse_zoom_and_reset() {
        const points = []
        for (let index = 0; index < 100; index++) {
            points.push({frequency_hz: 640000000 + index * 25000, value: index === 50 ? null : 43})
        }
        sourcePoints = points
        chart.viewport = {start: 0, count: 100, total: 100, points: points}
        const plot = findChild(chart, "measurementPlot")
        mouseWheel(plot, plot.width / 2, plot.height / 2, 0, 120)
        tryCompare(chart, "visibleCount", 80)
        verify(chart.visibleStart > 0)
        verify(chart.visibleStart + chart.visibleCount <= 100)
        mouseDoubleClickSequence(plot, plot.width / 2, plot.height / 2)
        compare(chart.visibleCount, 100)
        compare(chart.visibleStart, 0)
        chart.zoomAt(1, true)
        compare(chart.visibleStart, 20)
        chart.zoomAt(1, false)
        compare(chart.visibleCount, 100)
        compare(chart.visibleStart, 0)
        chart.viewport = {start: 0, count: 0, total: 0, points: []}
        chart.zoomAt(0.5, true)
        compare(chart.visibleCount, 0)
    }
}
