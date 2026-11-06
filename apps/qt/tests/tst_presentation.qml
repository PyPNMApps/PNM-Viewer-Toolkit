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
    TextMetrics {
        id: negativeAxisLabel
        font.family: "sans-serif"
        font.pixelSize: 11
        text: "-15.396"
    }
    function test_negative_pre_eq_axis_label_fits_margin() {
        const plot = findChild(chart, "measurementPlot")
        verify(plot !== null)
        // Canvas labels are right aligned, 12 px before the plot origin.
        verify(plot.leftMargin - 12 >= negativeAxisLabel.width)
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
    ModulationControls {
        id: modulationControls
        theme: appearance
        width: 850
        visible: false
        property int profileRequest: -1
        property int metricRequest: -1
        property int allRequests: 0
        property int carrierRequest: -1
        property int allCarrierRequests: 0
        onProfileRequested: index => {
            profileRequest = index
            const selected = (metadata.selected_indices || []).slice()
            const position = selected.indexOf(index)
            if (position < 0) { selected.push(index) } else { selected.splice(position, 1) }
            metadata = Object.assign({}, metadata, {selected_indices: selected})
        }
        onAllRequested: {
            allRequests++
            metadata = Object.assign({}, metadata, {selected_indices: metadata.profiles.map((profile, index) => index)})
        }
        onCarrierRequested: index => {
            carrierRequest = index
            const selected = (metadata.selected_carrier_types || []).slice()
            const kind = metadata.carrier_types[index].kind
            const position = selected.indexOf(kind)
            if (position < 0) { selected.push(kind) } else { selected.splice(position, 1) }
            metadata = Object.assign({}, metadata, {selected_carrier_types: selected})
        }
        onAllCarriersRequested: {
            allCarrierRequests++
            metadata = Object.assign({}, metadata, {selected_carrier_types: metadata.carrier_types.map(choice => choice.kind)})
        }
        onMetricRequested: index => metricRequest = index
    }
    function clickProfileToggle(button, expectedIndex) {
        // Model changes queue both GridLayout and Flow positioning. Wait for
        // real geometry before Cocoa maps the click into window coordinates.
        verify(waitForPolish(modulationControls))
        verify(waitForPolish(button.parent))
        verify(waitForRendering(button))
        modulationControls.profileRequest = -1
        mouseClick(button)
        tryCompare(modulationControls, "profileRequest", expectedIndex)
    }
    function test_modulation_profile_metric_controls_and_step_chart() {
        chart.visible = false
        modulationControls.visible = true
        modulationControls.metadata = {profiles: [{profile_id: 3}, {profile_id: 0}], selected_indices: [1], metric_index: 0,
            assignment_summary: "256-QAM: 7408 · Pilots: 56 · PLC: 16"}
        const first = findChild(modulationControls, "modulationProfileToggle0")
        const second = findChild(modulationControls, "modulationProfileToggle1")
        const all = findChild(modulationControls, "modulationAllProfiles")
        verify(waitForPolish(modulationControls))
        verify(waitForPolish(first.parent))
        const firstPosition = first.mapToItem(modulationControls, 0, 0)
        const secondPosition = second.mapToItem(modulationControls, 0, 0)
        verify(firstPosition.x + first.width <= secondPosition.x,
            "Profile buttons must finish layout without overlapping")
        verify(!first.checked)
        verify(second.checked)
        clickProfileToggle(first, 0)
        verify(first.checked && second.checked)
        clickProfileToggle(second, 1)
        verify(first.checked && !second.checked)
        clickProfileToggle(first, 0)
        verify(!first.checked && !second.checked)
        all.forceActiveFocus()
        keyClick(Qt.Key_Space)
        tryCompare(modulationControls, "allRequests", 1)
        verify(first.checked && second.checked)
        verify(all.highlighted)
        compare(first.width, all.width)
        compare(first.height, all.height)
        const metric = findChild(modulationControls, "modulationMetricSelector")
        compare(Math.round(metric.mapToItem(modulationControls, 0, 0).x), Math.round(first.mapToItem(modulationControls, 0, 0).x))
        metric.forceActiveFocus()
        keyClick(Qt.Key_End)
        keyClick(Qt.Key_Return)
        tryCompare(modulationControls, "metricRequest", 1)
        modulationControls.visible = false
        chart.visible = true
        chart.stepPlot = true
        chart.unit = "bits/symbol"
        chart.axisLabel = "Bit loading"
        chart.minimum = 0
        chart.maximum = 15
        chart.viewport = {start: 0, count: 4, total: 4, points: [
            {sample_index: 0, frequency_hz: 640000000, value: 8},
            {sample_index: 1, frequency_hz: 640025000, value: 10},
            {sample_index: 2, frequency_hz: 640050000, value: null},
            {sample_index: 3, frequency_hz: 640075000, value: 0}]}
        verify(chart.reading.indexOf("bits/symbol") >= 0)
        appearance.preference = 2
        wait(50)
        chart.stepPlot = false
        chart.unit = "dB"
        chart.axisLabel = "RxMER"
        chart.maximum = 64
        appearance.preference = 1
    }
    function test_profile_model_changes_route_each_mouse_click() {
        chart.visible = false
        modulationControls.visible = true
        // Replace differently sized models without a fixed delay, exercising
        // the queued Flow positioning that can otherwise route a click wrongly.
        for (const profileIds of [[3], [3, 2, 0], [0, 3]]) {
            modulationControls.metadata = {profiles: profileIds.map(profileId => ({profile_id: profileId})),
                selected_indices: [], metric_index: 0, carrier_types: [], selected_carrier_types: []}
            for (let index = 0; index < profileIds.length; index++) {
                const button = findChild(modulationControls, "modulationProfileToggle" + index)
                verify(button !== null)
                clickProfileToggle(button, index)
                verify(button.checked)
                clickProfileToggle(button, index)
                verify(!button.checked)
            }
        }
        modulationControls.visible = false
        chart.visible = true
    }
    function test_modulation_overlay_rendering_and_empty_selection() {
        chart.visible = true
        chart.stepPlot = true
        chart.maximum = 15
        chart.unit = "bits/symbol"
        chart.axisLabel = "Bit loading"
        const anchors = [{sample_index: 0, frequency_hz: 640000000, value: null},
                         {sample_index: 3, frequency_hz: 640075000, value: null}]
        const series = [{profile_index: 0, profile_id: 3, points: [
            {sample_index: 0, frequency_hz: 640000000, value: 12},
            {sample_index: 3, frequency_hz: 640075000, value: 12}]},
            {profile_index: 3, profile_id: 0, points: [
            {sample_index: 0, frequency_hz: 640000000, value: 8},
            {sample_index: 3, frequency_hz: 640075000, value: 8}]}]
        chart.viewport = {start: 0, count: 4, total: 4, points: anchors, modulation_profiles: {series: series}}
        for (const preference of [1, 2]) {
            appearance.preference = preference
            wait(50)
            const plot = findChild(chart, "measurementPlot")
            plot.requestPaint()
            verify(waitForRendering(plot))
            const image = grabImage(plot)
            const horizontal = Math.round((plot.width + plot.leftMargin - plot.rightMargin) / 2)
            for (const trace of series) {
                const vertical = Math.round(plot.topMargin + (15 - trace.points[0].value) / 15 * (plot.height - plot.topMargin - plot.bottomMargin))
                // Canvas antialiasing blends subpixel strokes with the surface.
                const hex = appearance.profileColor(trace.profile_id)
                const expected = Qt.rgba(parseInt(hex.slice(1, 3), 16) / 255,
                    parseInt(hex.slice(3, 5), 16) / 255, parseInt(hex.slice(5, 7), 16) / 255, 1)
                let distance = 3
                for (let offset = -1; offset <= 1; offset++) {
                    const pixel = image.pixel(horizontal, vertical + offset)
                    distance = Math.min(distance, Math.abs(pixel.r - expected.r)
                        + Math.abs(pixel.g - expected.g) + Math.abs(pixel.b - expected.b))
                }
                verify(distance < 0.6, "Expected profile trace color " + hex + " at its bit-loading level")
            }
        }
        chart.viewport = {start: 0, count: 4, total: 4, points: anchors, modulation_profiles: {series: []}}
        wait(50)
        verify(chart.frequencySelection.indexOf("640.075") >= 0)
        appearance.preference = 1
        chart.stepPlot = false
        chart.maximum = 64
        chart.unit = "dB"
        chart.axisLabel = "RxMER"
    }
    function test_carrier_buttons_lanes_and_hover() {
        modulationControls.visible = true
        modulationControls.metadata = {profiles: [{profile_id: 0}], selected_indices: [0], metric_index: 0,
            carrier_types: [{kind: "data", label: "Data", count: 10}, {kind: "continuous_pilot", label: "Pilots", count: 2},
                            {kind: "exclusion", label: "Excluded", count: 0}],
            selected_carrier_types: ["data", "continuous_pilot", "exclusion"]}
        wait(50)
        const pilot = findChild(modulationControls, "modulationCarrierToggle1")
        const data = findChild(modulationControls, "modulationCarrierToggle0")
        const all = findChild(modulationControls, "modulationAllCarriers")
        const profile = findChild(modulationControls, "modulationProfileToggle0")
        compare(pilot.width, profile.width)
        compare(pilot.height, all.height)
        compare(pilot.width, all.width)
        verify(!findChild(modulationControls, "modulationCarrierToggle2").enabled)
        verify(waitForPolish(modulationControls))
        mouseClick(pilot)
        tryCompare(modulationControls, "carrierRequest", 1)
        verify(!pilot.checked && data.checked)
        mouseClick(all)
        tryCompare(modulationControls, "allCarrierRequests", 1)
        verify(pilot.checked && data.checked)
        modulationControls.visible = false
        chart.visible = true
        chart.unit = "dB"
        chart.stepPlot = true
        chart.viewport = {start: 0, count: 3, total: 3,
            points: [{sample_index: 0, frequency_hz: 640000000, value: null}, {sample_index: 2, frequency_hz: 640050000, value: null}],
            modulation_profiles: {series: [], carrier_lanes: ["continuous_pilot"], markers: [
                {profile_id: 0, profile_index: 0, carrier_type: "continuous_pilot", first_frequency_hz: 640025000, last_frequency_hz: 640025000, count: 1}]}}
        compare(chart.verticalTitle, "dB")
        const plot = findChild(chart, "measurementPlot")
        wait(50)
        const horizontal = (plot.width + plot.leftMargin - plot.rightMargin) / 2
        mouseMove(plot, horizontal, plot.laneCenter("continuous_pilot"))
        tryCompare(chart, "carrierHoverText", "Profile 0 · Pilot · 640.025 MHz")
        const tooltip = findChild(chart, "carrierTypeTooltip")
        tryCompare(tooltip, "visible", true)
        chart.unit = "bits/symbol"
        compare(chart.verticalTitle, "Bit loading")
        chart.viewport = Object.assign({}, chart.viewport, {modulation_profiles: {series: [], carrier_lanes: ["plc"], markers: [
            {profile_id: 3, profile_index: 0, carrier_type: "plc", first_frequency_hz: 640000000, last_frequency_hz: 640050000, count: 2}]}})
        wait(50)
        mouseMove(plot, horizontal + 2, plot.laneCenter("plc"))
        tryCompare(chart, "carrierHoverText", "Profile 3 · PLC · 640.000–640.050 MHz · 2 carriers (grouped)")
        chart.viewport = {start: 0, total: 0, points: []}
        compare(chart.carrierHoverText, "")
        chart.stepPlot = false
        chart.unit = "dB"
    }
    ProfileBandwidth {
        id: bandwidthChip
        theme: appearance
        visible: false
    }
    function test_profile_bandwidth_chip_grid_and_refresh() {
        bandwidthChip.visible = true
        chart.visible = false
        bandwidthChip.metadata = {bandwidth_summary: "0 · 1481.600 Mbit/s", selected_indices: [1], profiles: [
            {profile_id: 3, bandwidth_bps: 2222400000, bandwidth_text: "2222.400 Mbit/s"},
            {profile_id: 0, bandwidth_bps: 1481600000, bandwidth_text: "1481.600 Mbit/s"}]}
        for (const preference of [1, 2]) {
            appearance.preference = preference
            wait(50)
            mouseClick(bandwidthChip)
            tryCompare(bandwidthChip, "popupVisible", true)
            const dialog = findChild(bandwidthChip, "profileBandwidthDialog")
            verify(dialog.modal)
            wait(50)
            compare(findChild(dialog.contentItem, "profileBandwidthRate0").text, "2222.400 Mbit/s")
            compare(findChild(dialog.contentItem, "profileBandwidthRate1").text, "1481.600 Mbit/s")
            keyClick(Qt.Key_Escape)
            tryCompare(bandwidthChip, "popupVisible", false)
        }
        bandwidthChip.forceActiveFocus()
        keyClick(Qt.Key_Space)
        tryCompare(bandwidthChip, "popupVisible", true)
        bandwidthChip.metadata = {bandwidth_summary: "3 · Unavailable", selected_indices: [0], profiles: [
            {profile_id: 3, bandwidth_bps: null, bandwidth_text: "Unavailable"}]}
        tryCompare(bandwidthChip, "popupVisible", false)
        mouseClick(bandwidthChip)
        tryCompare(bandwidthChip, "popupVisible", true)
        wait(50)
        compare(findChild(findChild(bandwidthChip, "profileBandwidthDialog").contentItem, "profileBandwidthRate0").text, "Unavailable")
        keyClick(Qt.Key_Escape)
        tryCompare(bandwidthChip, "popupVisible", false)
        bandwidthChip.visible = false
        chart.visible = true
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
        fecChart.visible = false
        modulationControls.visible = false
        bandwidthChip.visible = false
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
        verify(waitForRendering(chart))
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
