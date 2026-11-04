/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: chart
    required property Theme theme
    property var viewport: ({start: 0, total: 0, points: []})
    property string unit: "dB"
    property string axisLabel: "RxMER"
    property real minimum: 0
    property real maximum: 64
    property int selectedIndex: 0
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
        const lower = selection.dragging ? Math.min(selection.startFraction, selection.endFraction) : 0
        const upper = selection.dragging ? Math.max(selection.startFraction, selection.endFraction) : 1
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
    onViewportChanged: { selection.dragging = false; selectedIndex = 0; plot.requestPaint() }
    onSelectedIndexChanged: plot.requestPaint()
    onMinimumChanged: plot.requestPaint()
    onMaximumChanged: plot.requestPaint()
    onUnitChanged: plot.requestPaint()
    onAxisLabelChanged: plot.requestPaint()
    spacing: 10

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
        property real leftMargin: 48
        property real rightMargin: 18
        property real topMargin: 22
        property real bottomMargin: 42
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
            const plotHeight = height - topMargin - bottomMargin
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
                context.fillText(level.toString(), leftMargin - 12, vertical + 4)
                const fraction = tick / 4
                const frequency = firstHz + fraction * (lastHz - firstHz)
                context.textAlign = "center"
                context.fillText((frequency / chart.hzPerMhz).toFixed(2), leftMargin + fraction * plotWidth, height - 23)
            }
            context.textAlign = "left"
            context.fillText(chart.axisLabel + " · " + chart.unit, leftMargin, 12)
            context.textAlign = "center"
            context.fillText("Frequency · MHz", width / 2, height - 3)
            context.strokeStyle = chart.theme.accent
            context.lineWidth = 1.5
            context.beginPath()
            let connected = false
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
                    context.lineTo(horizontal, vertical)
                } else {
                    context.moveTo(horizontal, vertical)
                }
                connected = true
            }
            context.stroke()
            context.fillStyle = chart.theme.accent
            for (let index = 0; index < points.length; index++) {
                const point = points[index]
                const isolated = (index === 0 || points[index - 1].value === null)
                    && (index === points.length - 1 || points[index + 1].value === null)
                if (point.value !== null && (isolated || index === chart.selectedIndex)) {
                    const horizontal = lastHz === firstHz ? leftMargin + plotWidth / 2
                        : leftMargin + (point.frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                    const vertical = topMargin + (chart.maximum - point.value) / (chart.maximum - chart.minimum) * plotHeight
                    context.beginPath()
                    context.arc(horizontal, vertical, 3, 0, Math.PI * 2)
                    context.fill()
                }
            }
        }
        Rectangle {
            visible: selection.dragging
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
            property bool dragging: false
            property real startFraction: 0
            property real endFraction: 0
            cursorShape: Qt.CrossCursor
            function fractionAt(horizontal) {
                return Math.max(0, Math.min(1, (horizontal - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)))
            }
            onPressed: mouse => {
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
                if (distance >= 6) { chart.selectRange(startFraction, endFraction) }
            }
            onCanceled: dragging = false
            hoverEnabled: true
            onWheel: wheel => {
                const fraction = (wheel.x - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)
                chart.zoomAt(fraction, wheel.angleDelta.y > 0)
                wheel.accepted = true
            }
            onDoubleClicked: { dragging = false; chart.resetZoom() }
            onPositionChanged: mouse => {
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
