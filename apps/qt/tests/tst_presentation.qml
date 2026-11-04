/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    name: "RxMerPresentation"
    when: windowShown
    visible: true
    width: 900
    height: 450
    Theme { id: appearance }
    property var sourcePoints: []
    RxMerChart {
        id: chart
        theme: appearance
        width: 850
        height: 400
        onRangeRequested: (start, count) => {
            chart.viewport = {start: start, count: count, total: sourcePoints.length, points: sourcePoints.slice(start, start + count)}
        }
    }
    function init() {
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
            {frequency_hz: 665600000, rxmer_db: 42.75},
            {frequency_hz: 665625000, rxmer_db: null}
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
    function test_drag_frequency_selection() {
        sourcePoints = []
        for (let index = 0; index < 100; index++) {
            sourcePoints.push({frequency_hz: 640000000 + index * 25000, rxmer_db: 43})
        }
        chart.viewport = {start: 0, count: 100, total: 100, points: sourcePoints}
        const plot = findChild(chart, "rxmerPlot")
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
            points.push({frequency_hz: 640000000 + index * 25000, rxmer_db: index === 50 ? null : 43})
        }
        sourcePoints = points
        chart.viewport = {start: 0, count: 100, total: 100, points: points}
        const plot = findChild(chart, "rxmerPlot")
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
