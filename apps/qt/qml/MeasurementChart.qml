/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Basic as Basic
import QtQuick.Layouts

ColumnLayout {
    id: chart
    required property Theme theme
    property var viewport: ({start: 0, total: 0, points: []})
    property bool spectrumEnabled: false
    property bool rxmerEnabled: false
    property bool showRxmerMinimum: true
    property bool showRxmerAverage: true
    property bool showRxmerMaximum: true
    onShowRxmerMinimumChanged: plot.requestPaint()
    onShowRxmerAverageChanged: plot.requestPaint()
    onShowRxmerMaximumChanged: plot.requestPaint()
    signal bitLoadingRequested()
    property bool selectingPower: false
    property bool powerSelected: false
    signal frequencyRequested()
    signal powerRequested(real first, real last)
    property bool stepPlot: false
    property bool showSpectrumAverage: true
    onShowSpectrumAverageChanged: plot.requestPaint()
    property string unit: "dB"
    property string axisLabel: "RxMER"
    property real minimum: 0
    property real maximum: 64
    property int selectedIndex: 0
    property string carrierHoverText: ""
    readonly property var carrierLanes: viewport.modulation_profiles ? (viewport.modulation_profiles.carrier_lanes || []) : []
    readonly property string verticalTitle: viewport.modulation_profiles ? (unit === "dB" ? "dB" : "Bit loading") : axisLabel + " · " + unit
    function carrierAt(horizontal, vertical) {
        const points = viewport.points
        const overlays = viewport.modulation_profiles
        if (!overlays || !points.length || horizontal < plot.leftMargin || horizontal > plot.width - plot.rightMargin) { return "" }
        const first = points[0].frequency_hz
        const span = points[points.length - 1].frequency_hz - first
        const plotWidth = plot.width - plot.leftMargin - plot.rightMargin
        const matches = []
        for (const marker of (overlays.markers || [])) {
            if (Math.abs(vertical - plot.laneCenter(marker.carrier_type)) > 6) { continue }
            const start = span === 0 ? plot.leftMargin + plotWidth / 2 : plot.leftMargin + (marker.first_frequency_hz - first) / span * plotWidth
            const end = span === 0 ? start : plot.leftMargin + (marker.last_frequency_hz - first) / span * plotWidth
            if (horizontal < start - 4 || horizontal > end + 4) { continue }
            const range = (marker.first_frequency_hz / hzPerMhz).toFixed(3)
                + (marker.last_frequency_hz !== marker.first_frequency_hz ? "–" + (marker.last_frequency_hz / hzPerMhz).toFixed(3) : "") + " MHz"
            matches.push("Profile " + marker.profile_id + " · " + theme.carrierLabel(marker.carrier_type)
                + " · " + range + (marker.count > 1 ? " · " + marker.count + " carriers (grouped)" : ""))
        }
        return matches.slice(0, 4).join("\n") + (matches.length > 4 ? "\n+" + (matches.length - 4) + " more profiles" : "")
    }
    readonly property int pointCount: viewport.points.length
    readonly property real hzPerMhz: 1000000
    readonly property var selectedPoint: pointCount > 0 ? viewport.points[Math.min(selectedIndex, pointCount - 1)] : null
    readonly property string reading: {
        if (!selectedPoint) {
            return "No samples in this capture"
        }
        const value = selectedPoint.value === null ? "Unavailable" : selectedPoint.value.toFixed(2) + " " + unit
        return "#" + ((selectedPoint.sample_index === undefined ? viewport.start + Math.min(selectedIndex, pointCount - 1) : selectedPoint.sample_index)) + "  ·  "
            + (selectedPoint.frequency_hz / hzPerMhz).toFixed(3) + " MHz  ·  " + value
    }
    readonly property int visibleStart: viewport.start || 0
    readonly property int visibleCount: viewport.count === undefined ? pointCount : viewport.count
    signal rangeRequested(int start, int count)
    function resetZoom() {
        selectingPower = false
        powerSelected = false
        rangeRequested(0, viewport.total)
    }
    function zoomAt(fraction, zoomIn) {
        if (viewport.total < 2) { return }
        const anchor = Math.max(0, Math.min(1, fraction))
        const nextCount = Math.max(2, Math.min(viewport.total, Math.round(visibleCount * (zoomIn ? 0.8 : 1.25))))
        const nextStart = Math.max(0, Math.min(viewport.total - nextCount, Math.round(visibleStart + anchor * (visibleCount - nextCount))))
        rangeRequested(nextStart, nextCount)
    }
    readonly property string frequencySelection: {
        const points = viewport.points
        if (!points.length) { return "No frequency selection" }
        const first = points[0].frequency_hz
        const span = points[points.length - 1].frequency_hz - first
        const lower = selection.dragging || chart.powerSelected ? Math.min(selection.startFraction, selection.endFraction) : 0
        const upper = selection.dragging || chart.powerSelected ? Math.max(selection.startFraction, selection.endFraction) : 1
        return "Frequency selection: " + ((first + lower * span) / hzPerMhz).toFixed(3)
            + " – " + ((first + upper * span) / hzPerMhz).toFixed(3) + " MHz"
    }
    function selectRange(firstFraction, lastFraction) {
        if (visibleCount < 2) { return }
        const lower = Math.max(0, Math.min(1, Math.min(firstFraction, lastFraction)))
        const upper = Math.max(0, Math.min(1, Math.max(firstFraction, lastFraction)))
        const first = Math.floor(lower * (visibleCount - 1))
        const last = Math.ceil(upper * (visibleCount - 1))
        rangeRequested(visibleStart + first, Math.max(2, last - first + 1))
    }
    onViewportChanged: { powerSelected = false; selectingPower = false; carrierHoverText = ""; selection.dragging = false; selectedIndex = 0; plot.requestPaint() }
    onSelectedIndexChanged: plot.requestPaint()
    onMinimumChanged: plot.requestPaint()
    onMaximumChanged: plot.requestPaint()
    onUnitChanged: plot.requestPaint()
    onAxisLabelChanged: plot.requestPaint()
    onStepPlotChanged: plot.requestPaint()
    Keys.onEscapePressed: {
        selectingPower = false
        powerSelected = false
    }
    spacing: 10

    Basic.Menu {
        id: contextMenu
        parent: Overlay.overlay
        popupType: Popup.Item
        width: 220
        objectName: "spectrumContextMenu"
        background: Rectangle { color: chart.theme.panel; border.color: chart.theme.line; radius: 4 }
        Basic.MenuItem {
            id: calculatePowerItem
            objectName: "calculateSpectrumPower"
            text: "Calculate power…"
            visible: chart.spectrumEnabled
            height: visible ? implicitHeight : 0
            contentItem: Label { text: calculatePowerItem.text; color: chart.theme.text }
            background: Rectangle { color: parent.highlighted ? chart.theme.accentSoft : "transparent" }
            onTriggered: {
                chart.selectingPower = true
                chart.powerSelected = false
            }
        }
        Basic.MenuItem {
            id: frequencyItem
            objectName: "selectSpectrumFrequency"
            text: "Frequency…"
            visible: chart.spectrumEnabled
            height: visible ? implicitHeight : 0
            contentItem: Label { text: frequencyItem.text; color: chart.theme.text }
            background: Rectangle { color: parent.highlighted ? chart.theme.accentSoft : "transparent" }
            onTriggered: chart.frequencyRequested()
        }
        Basic.MenuItem {
            id: bitLoadingItem
            objectName: "analyzeRxmerBitLoading"
            text: "Modulation & Capacity…"
            visible: chart.rxmerEnabled
            height: visible ? implicitHeight : 0
            contentItem: Label { text: bitLoadingItem.text; color: chart.theme.text }
            background: Rectangle { color: parent.highlighted ? chart.theme.accentSoft : "transparent" }
            onTriggered: chart.bitLoadingRequested()
        }
    }
    Label {
        visible: chart.selectingPower
        text: "Drag across the graph to calculate integrated dBmV"
        color: chart.theme.accent
    }
    TextMetrics {
        id: axisLabelMetrics
        font.family: "sans-serif"
        font.pixelSize: 11
        text: "-000.000"
    }
    // Qt Quick Canvas is rendered by Qt; no HTML document or WebView is used.
    Canvas {
        id: plot
        objectName: "measurementPlot"
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 180
        Layout.preferredHeight: 280
        Accessible.role: Accessible.Graphic
        Accessible.name: chart.axisLabel + " in " + chart.unit + " versus frequency in MHz; drag to zoom, double-click to reset"
        // Leave room for negative dB labels with three decimal places.
        property real leftMargin: Math.max(64, Math.ceil(axisLabelMetrics.width) + 16)
        onLeftMarginChanged: requestPaint()
        property real rightMargin: Math.max(18, Math.ceil(axisLabelMetrics.width) / 2 + 8)
        property real topMargin: 22
        property real bottomMargin: 42
        readonly property real laneHeight: 16
        readonly property real dataBottom: height - bottomMargin - chart.carrierLanes.length * laneHeight - (chart.carrierLanes.length ? 8 : 0)
        function laneCenter(kind) { return dataBottom + 8 + (chart.carrierLanes.indexOf(kind) + 0.5) * laneHeight }
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        Connections {
            target: chart.theme
            function onDarkChanged() { plot.requestPaint() }
        }
        onPaint: {
            const context = getContext("2d")
            context.reset()
            const points = chart.viewport.points
            const plotWidth = width - leftMargin - rightMargin
            const plotHeight = dataBottom - topMargin
            const firstHz = points.length ? points[0].frequency_hz : 0
            const lastHz = points.length ? points[points.length - 1].frequency_hz : firstHz
            context.font = "11px sans-serif"
            context.strokeStyle = chart.theme.line
            context.fillStyle = chart.theme.muted
            context.lineWidth = 1
            for (let tick = 0; tick <= 4; tick++) {
                const level = chart.minimum + tick / 4 * (chart.maximum - chart.minimum)
                const vertical = topMargin + (chart.maximum - level) / (chart.maximum - chart.minimum) * plotHeight
                context.beginPath()
                context.moveTo(leftMargin, vertical)
                context.lineTo(width - rightMargin, vertical)
                context.stroke()
                context.textAlign = "right"
                context.fillText(Number(level.toFixed(3)).toString(), leftMargin - 12, vertical + 4)
                const fraction = tick / 4
                const frequency = firstHz + fraction * (lastHz - firstHz)
                context.textAlign = "center"
                context.fillText((frequency / chart.hzPerMhz).toFixed(2), leftMargin + fraction * plotWidth, height - 23)
            }
            context.textAlign = "left"
            context.fillText(chart.verticalTitle, leftMargin, 12)
            context.textAlign = "center"
            context.fillText("Frequency · MHz", width / 2, height - 3)
            const overlays = chart.viewport.modulation_profiles
            const statistics = chart.viewport.rxmer_statistics || chart.viewport.spectrum_statistics
            const traces = overlays ? overlays.series : (statistics && !chart.showRxmerAverage ? [] : [{points: chart.viewport.points, color: statistics ? chart.theme.rxmerAverage : chart.theme.accent}])
            if (statistics) {
                if (chart.showRxmerMinimum) traces.push({points: statistics.minimum, color: chart.theme.rxmerMinimum})
                if (chart.showRxmerMaximum) traces.push({points: statistics.maximum, color: chart.theme.rxmerMaximum})
            }
            if (!overlays && chart.showSpectrumAverage && chart.viewport.spectrum_average) {
                traces.push({points: chart.viewport.spectrum_average.points, color: chart.theme.spectrumAverage, dashed: !!statistics})
            }
            for (let traceIndex = 0; traceIndex < traces.length; traceIndex++) {
                const trace = traces[traceIndex]
                const points = trace.points
                const color = overlays ? chart.theme.profileColor(trace.profile_id) : (trace.color || chart.theme.accent)
                context.strokeStyle = color
                context.lineWidth = 1.5
                if (context.setLineDash) context.setLineDash(trace.dashed ? [6,4] : [])
                context.beginPath()
                let connected = false
                let previousVertical = 0
                for (let index = 0; index < points.length; index++) {
                    const point = points[index]
                    if (point.value === null) {
                        connected = false
                        continue
                    }
                    const horizontal = lastHz === firstHz ? leftMargin + plotWidth / 2
                        : leftMargin + (point.frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                    const vertical = topMargin + (chart.maximum - point.value) / (chart.maximum - chart.minimum) * plotHeight
                    if (connected) {
                        if (chart.stepPlot) { context.lineTo(horizontal, previousVertical) }
                        context.lineTo(horizontal, vertical)
                    } else {
                        context.moveTo(horizontal, vertical)
                    }
                    previousVertical = vertical
                    connected = true
                }
                context.stroke()
                context.fillStyle = color
                for (let index = 0; index < points.length; index++) {
                    const point = points[index]
                    const isolated = (index === 0 || points[index - 1].value === null)
                        && (index === points.length - 1 || points[index + 1].value === null)
                    if (point.value !== null && (isolated || (!overlays && traceIndex === 0 && index === chart.selectedIndex))) {
                        const horizontal = lastHz === firstHz ? leftMargin + plotWidth / 2
                            : leftMargin + (point.frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                        const vertical = topMargin + (chart.maximum - point.value) / (chart.maximum - chart.minimum) * plotHeight
                        context.beginPath()
                        context.arc(horizontal, vertical, 3, 0, Math.PI * 2)
                        context.fill()
                    }
                }
            }
            if (overlays) {
                for (const kind of chart.carrierLanes) {
                    const vertical = laneCenter(kind)
                    context.strokeStyle = chart.theme.line
                    context.lineWidth = 1
                    context.beginPath(); context.moveTo(leftMargin, vertical); context.lineTo(width - rightMargin, vertical); context.stroke()
                    context.fillStyle = chart.theme.muted
                    context.textAlign = "right"
                    const labels = {continuous_pilot: "Pilot", plc: "PLC", exclusion: "Excl.", zero_bit_loaded: "Zero", reserved: "Res.", unknown: "Unk."}
                    context.fillText(labels[kind] || kind, leftMargin - 8, vertical + 3)
                }
                for (const marker of (overlays.markers || [])) {
                    const first = lastHz === firstHz ? leftMargin + plotWidth / 2 : leftMargin + (marker.first_frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                    const last = lastHz === firstHz ? first : leftMargin + (marker.last_frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                    context.strokeStyle = chart.theme.carrierColor(marker.carrier_type)
                    context.lineWidth = 5
                    context.globalAlpha = marker.count > 1 ? 0.65 : 1
                    context.beginPath(); context.moveTo(first - 1, laneCenter(marker.carrier_type)); context.lineTo(last + 1, laneCenter(marker.carrier_type)); context.stroke()
                }
                context.globalAlpha = 1
            }
        }
        Rectangle {
            visible: selection.dragging || chart.powerSelected
            x: plot.leftMargin + Math.min(selection.startFraction, selection.endFraction) * (plot.width - plot.leftMargin - plot.rightMargin)
            y: plot.topMargin
            width: Math.abs(selection.endFraction - selection.startFraction) * (plot.width - plot.leftMargin - plot.rightMargin)
            height: plot.height - plot.topMargin - plot.bottomMargin
            color: chart.theme.accent
            opacity: 0.2
            border.color: chart.theme.text
        }
        MouseArea {
            id: selection
            anchors.fill: parent
            acceptedButtons: chart.spectrumEnabled || chart.rxmerEnabled ? Qt.LeftButton | Qt.RightButton : Qt.LeftButton
            property bool dragging: false
            property real startFraction: 0
            property real endFraction: 0
            cursorShape: Qt.CrossCursor
            function fractionAt(horizontal) {
                return Math.max(0, Math.min(1, (horizontal - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)))
            }
            onPressed: mouse => {
                if (mouse.button === Qt.RightButton) {
                    const position = plot.mapToItem(contextMenu.parent, mouse.x, mouse.y)
                    contextMenu.popup(position.x, position.y)
                    return
                }
                chart.forceActiveFocus()
                chart.powerSelected = false
                chart.carrierHoverText = "";
                if (mouse.x < plot.leftMargin || mouse.x > plot.width - plot.rightMargin
                    || mouse.y < plot.topMargin || mouse.y > plot.height - plot.bottomMargin) {
                    mouse.accepted = false
                    return
                }
                startFraction = fractionAt(mouse.x)
                endFraction = startFraction
                dragging = true
            }
            onReleased: mouse => {
                if (!dragging) { return }
                endFraction = fractionAt(mouse.x)
                const distance = Math.abs(endFraction - startFraction) * (plot.width - plot.leftMargin - plot.rightMargin)
                dragging = false
                if (distance >= 6) {
                    if (chart.selectingPower) {
                        chart.powerSelected = true
                        chart.selectingPower = false
                        chart.powerRequested(startFraction, endFraction)
                    } else {
                        chart.selectRange(startFraction, endFraction)
                    }
                }
            }
            onCanceled: dragging = false
            hoverEnabled: true
            onExited: chart.carrierHoverText = ""
            ToolTip {
                objectName: "carrierTypeTooltip"
                visible: selection.containsMouse && !selection.dragging && chart.carrierHoverText.length > 0
                text: chart.carrierHoverText
                x: Math.max(0, Math.min(selection.width - width - 8, selection.mouseX + 12))
                y: Math.max(0, selection.mouseY - height - 8)
            }
            onWheel: wheel => {
                const fraction = (wheel.x - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)
                chart.zoomAt(fraction, wheel.angleDelta.y > 0)
                wheel.accepted = true
            }
            onDoubleClicked: { dragging = false; chart.resetZoom() }
            onPositionChanged: mouse => {
                chart.carrierHoverText = dragging ? "" : chart.carrierAt(mouse.x, mouse.y)
                if (dragging) { endFraction = fractionAt(mouse.x) }
                const fraction = (mouse.x - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)
                const points = chart.viewport.points
                if (!points.length) { return }
                const frequency = points[0].frequency_hz + fraction * (points[points.length - 1].frequency_hz - points[0].frequency_hz)
                let nearest = 0
                for (let index = 1; index < points.length; index++) {
                    if (Math.abs(points[index].frequency_hz - frequency) < Math.abs(points[nearest].frequency_hz - frequency)) {
                        nearest = index
                    }
                }
                chart.selectedIndex = nearest
            }
        }
    }
    Label {
        objectName: "sampleReading"
        text: chart.frequencySelection
        color: chart.theme.muted
        font.family: "monospace"
        font.pixelSize: 11
        Layout.fillWidth: true
        elide: Text.ElideRight
        Accessible.name: chart.frequencySelection
    }
}
