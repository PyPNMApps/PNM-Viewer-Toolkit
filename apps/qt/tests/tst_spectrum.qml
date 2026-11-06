/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    id: testCase
    name: "SpectrumWindowAverage"
    when: windowShown
    visible: true
    width: 900
    height: 500
    Theme { id: appearance }
    SpectrumControls {
        id: controls
        width: 850
        theme: appearance
        sampleCount: 4
        averageWindow: chart.viewport.spectrum_average ? chart.viewport.spectrum_average.window_points : 0
        property int requested: 0
        onAverageRequested: windowPoints => {
            requested = windowPoints
            // Synthetic backend response; scientific averaging is tested in Rust.
            chart.viewport = Object.assign({}, chart.viewport, {spectrum_average: {
                window_points: windowPoints, points: [
                    {sample_index: 0, frequency_hz: 100000000, value: -25},
                    {sample_index: 3, frequency_hz: 103000000, value: -25}]}})
        }
    }
    MeasurementChart {
        id: chart
        y: 50
        width: 850
        height: 430
        theme: appearance
        minimum: -50
        maximum: 0
        axisLabel: "Amplitude"
        unit: "dBmV"
        showSpectrumAverage: controls.showAverage
    }
    function init() {
        controls.sampleCount = 4
        controls.requested = 0
        chart.viewport = {start: 0, count: 4, total: 4, points: [
            {sample_index: 0, frequency_hz: 100000000, value: -10},
            {sample_index: 1, frequency_hz: 101000000, value: -20},
            {sample_index: 2, frequency_hz: 102000000, value: -30},
            {sample_index: 3, frequency_hz: 103000000, value: -40}]}
        findChild(controls, "spectrumAverageInput").text = "101"
    }
    function test_positive_integer_validation_and_disabled_visibility_until_generated() {
        const input = findChild(controls, "spectrumAverageInput")
        const apply = findChild(controls, "spectrumWindowAverageButton")
        const toggle = findChild(controls, "spectrumAverageVisibilityButton")
        compare(toggle.enabled, false)
        for (const invalid of ["", "0", "-1", "1.5", "abc", "2147483648"]) {
            input.text = invalid
            compare(apply.enabled, false)
        }
        input.text = "3"
        compare(apply.enabled, true)
        mouseClick(apply)
        compare(controls.requested, 3)
        compare(toggle.enabled, true)
        compare(chart.showSpectrumAverage, true)
        const original = JSON.stringify(chart.viewport)
        mouseClick(toggle)
        compare(chart.showSpectrumAverage, false)
        compare(JSON.stringify(chart.viewport), original)
        mouseClick(toggle)
        compare(chart.showSpectrumAverage, true)
        controls.sampleCount = 0
        compare(apply.enabled, false)
    }
    function redAtAverage() {
        const plot = findChild(chart, "measurementPlot")
        const image = grabImage(testCase)
        const horizontal = Math.floor(plot.leftMargin + (plot.width - plot.leftMargin - plot.rightMargin) / 4)
        const vertical = Math.round((plot.topMargin + plot.dataBottom) / 2)
        for (let offset = -2; offset <= 2; offset++) {
            const position = plot.mapToItem(testCase, horizontal, vertical + offset)
            const color = image.pixel(Math.round(position.x), Math.round(position.y))
            if (color.r > color.g * 1.3 && color.r > color.b * 1.3 && color.r > 0.3) {
                return true
            }
        }
        return false
    }
    function test_red_average_trace_renders_and_mutes_in_both_themes() {
        waitForPolish(controls)
        verify(waitForRendering(controls))
        mouseClick(findChild(controls, "spectrumWindowAverageButton"))
        compare(controls.requested, 101)
        compare(chart.showSpectrumAverage, true)
        verify(chart.viewport.spectrum_average !== undefined)
        const toggle = findChild(controls, "spectrumAverageVisibilityButton")
        for (const preference of [1, 2]) {
            appearance.preference = preference
            controls.showAverage = true
            verify(waitForRendering(findChild(chart, "measurementPlot")))
            tryVerify(redAtAverage, 5000, "Red trace in theme " + preference)
            mouseClick(toggle)
            tryVerify(function() { return !redAtAverage() })
        }
        chart.viewport = {start: 0, count: 0, total: 0, points: []}
        compare(toggle.enabled, false)
    }
}
