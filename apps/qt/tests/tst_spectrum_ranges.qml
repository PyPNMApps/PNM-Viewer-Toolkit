/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "helpers"
import "../qml"

RenderTestCase {
    id: testCase
    name: "SpectrumRanges"
    when: windowShown
    visible: true
    width: 900
    height: 600
    Theme { id: appearance }
    SpectrumTools {
        id: tools
        width: 850
        theme: appearance
        property var request: []
        // Synthetic validation response; real unit/bounds math is tested in Rust.
        validateRange: (start, stop, unit) => start >= stop ? "Start must precede Stop" : (start < 100 || stop > 200 ? "Out of bounds" : "")
        onRangeRequested: (start, stop, unit) => request = [start, stop, unit]
    }
    MeasurementChart {
        id: chart
        y: 100
        width: 850
        height: 450
        theme: appearance
        spectrumEnabled: true
        property var powerRequest: []
        property int rangeCount: 0
        onPowerRequested: (first, last) => powerRequest = [first, last]
        onRangeRequested: rangeCount++
        onFrequencyRequested: tools.openFrequency()
    }
    function init() {
        chart.powerRequest = []
        chart.rangeCount = 0
        chart.selectingPower = false
        chart.powerSelected = false
        chart.viewport = {start: 0, count: 4, total: 4, points: [
            {frequency_hz: 100000000, sample_index: 0, value: 10},
            {frequency_hz: 200000000, sample_index: 3, value: 20}]}
        tools.workspace = {minimum_hz: 100, maximum_hz: 200, active: 0, tabs: [{title: "Spectrum"}]}
        tools.request = []
    }
    function cleanup() {
        findChild(tools, "spectrumFrequencyDialog").close()
        findChild(chart, "spectrumContextMenu").close()
    }
    function test_context_menu_and_power_drag_preserve_zoom_data() {
        return [{tag: "dark", preference: 1}, {tag: "light", preference: 2}]
    }
    function test_context_menu_and_power_drag_preserve_zoom(data) {
        appearance.preference = data.preference
        const plot = findChild(chart, "measurementPlot")
        verify(waitForPolish(chart))
        // Explicitly schedule the frame this wait observes, including when
        // the first data row keeps the already-rendered default dark theme.
        plot.requestPaint()
        verify(waitForFrame(plot))
        mouseClick(plot, plot.leftMargin + 100, plot.topMargin + 30, Qt.RightButton)
        const menu = findChild(chart, "spectrumContextMenu")
        tryCompare(menu, "visible", true)
        tryCompare(menu, "opened", true)
        waitForPolish(menu.contentItem)
        const calculate = findChild(chart, "calculateSpectrumPower")
        plot.requestPaint()
        verify(waitForFrame(plot))
        verify(calculate.width > 0 && calculate.height > 0)
        mouseClick(calculate)
        compare(chart.selectingPower, true)
        const width = plot.width - plot.leftMargin - plot.rightMargin
        mousePress(plot, plot.leftMargin + width * 0.2, plot.topMargin + 30)
        mouseMove(plot, plot.leftMargin + width * 0.8, plot.topMargin + 30)
        const selectedFrequency = chart.frequencySelection
        mouseRelease(plot, plot.leftMargin + width * 0.8, plot.topMargin + 30)
        compare(chart.powerRequest.length, 2)
        fuzzyCompare(chart.powerRequest[0], 0.2, 0.01)
        fuzzyCompare(chart.powerRequest[1], 0.8, 0.01)
        compare(chart.rangeCount, 0)
        compare(chart.powerSelected, true)
        compare(chart.selectingPower, false)
        compare(chart.frequencySelection, selectedFrequency)
        keyClick(Qt.Key_Escape)
        compare(chart.powerSelected, false)
        verify(chart.frequencySelection.indexOf("100.000 – 200.000") >= 0)
    }
    function test_frequency_dialog_validates_and_submits_selected_units_data() {
        return [{tag: "dark", preference: 1}, {tag: "light", preference: 2}]
    }
    function test_frequency_dialog_validates_and_submits_selected_units(data) {
        appearance.preference = data.preference
        tools.openFrequency()
        const dialog = findChild(tools, "spectrumFrequencyDialog")
        tryCompare(dialog, "visible", true)
        const start = findChild(tools, "spectrumStartFrequency")
        const stop = findChild(tools, "spectrumStopFrequency")
        const open = findChild(tools, "openSpectrumRange")
        compare(start.text, "100")
        compare(stop.text, "200")
        for (const invalid of ["", "-1", "201", "200", "abc"]) {
            start.text = invalid
            compare(open.enabled, false)
        }
        start.text = "120"
        stop.text = "210"
        compare(open.enabled, false)
        stop.text = "180"
        compare(open.enabled, true)
        const radio = findChild(tools, "spectrumUnitRepeater").itemAt(2)
        waitForPolish(dialog.contentItem)
        verify(radio !== null)
        mouseClick(radio)
        compare(dialog.unit, "MHz")
        waitForPolish(dialog.contentItem)
        verify(waitForFrame(dialog.contentItem))
        mouseClick(open)
        compare(tools.request, [120, 180, "MHz"])
        tryCompare(dialog, "visible", false)
    }
    function test_power_chips_use_dbmv_and_one_decimal() {
        tools.workspace = {tabs: [{title: "Spectrum"}], power: {
            start_hz: 100, stop_hz: 200, bandwidth_hz: 100, level_dbmv: 3.01029995664}}
        compare(tools.powerChips.length, 3)
        compare(tools.powerChips[0].title, "Integrated level")
        compare(tools.powerChips[0].value, "3.0 dBmV")
        compare(tools.powerChips[1].value, "100–200 Hz")
        compare(tools.powerChips[2].value, "100 Hz")
        tools.workspace = {tabs: []}
        compare(tools.powerChips.length, 0)
    }
}
