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
    property int selectedIndex: 0
    readonly property int pointCount: viewport.points.length
    readonly property real hzPerMhz: 1000000
    readonly property var selectedPoint: pointCount > 0 ? viewport.points[Math.min(selectedIndex, pointCount - 1)] : null
    readonly property string reading: {
        if (!selectedPoint) {
            return "No samples in this capture"
        }
        const value = selectedPoint.rxmer_db === null ? "Unavailable" : selectedPoint.rxmer_db.toFixed(2) + " dB"
        return "#" + (viewport.start + Math.min(selectedIndex, pointCount - 1)) + "  ·  "
            + (selectedPoint.frequency_hz / hzPerMhz).toFixed(3) + " MHz  ·  " + value
    }
    onViewportChanged: { selectedIndex = 0; plot.requestPaint() }
    onSelectedIndexChanged: plot.requestPaint()
    spacing: 10

    // Qt Quick Canvas is rendered by Qt; no HTML document or WebView is used.
    Canvas {
        id: plot
        objectName: "rxmerPlot"
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 180
        Layout.preferredHeight: 280
        Accessible.role: Accessible.Graphic
        Accessible.name: "RxMER in dB versus frequency in MHz; missing readings appear as gaps"
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
                const level = tick * 16
                const vertical = topMargin + (64 - level) / 64 * plotHeight
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
            context.fillText("RxMER · dB", leftMargin, 12)
            context.textAlign = "center"
            context.fillText("Frequency · MHz", width / 2, height - 3)
            context.strokeStyle = chart.theme.accent
            context.lineWidth = 1.5
            context.beginPath()
            let connected = false
            for (let index = 0; index < points.length; index++) {
                const point = points[index]
                if (point.rxmer_db === null) {
                    connected = false
                    continue
                }
                const horizontal = lastHz === firstHz ? leftMargin + plotWidth / 2
                    : leftMargin + (point.frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                const vertical = topMargin + (64 - point.rxmer_db) / 64 * plotHeight
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
                const isolated = (index === 0 || points[index - 1].rxmer_db === null)
                    && (index === points.length - 1 || points[index + 1].rxmer_db === null)
                if (point.rxmer_db !== null && (isolated || index === chart.selectedIndex)) {
                    const horizontal = lastHz === firstHz ? leftMargin + plotWidth / 2
                        : leftMargin + (point.frequency_hz - firstHz) / (lastHz - firstHz) * plotWidth
                    const vertical = topMargin + (64 - point.rxmer_db) / 64 * plotHeight
                    context.beginPath()
                    context.arc(horizontal, vertical, 3, 0, Math.PI * 2)
                    context.fill()
                }
            }
        }
        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onPositionChanged: mouse => {
                const fraction = (mouse.x - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)
                chart.selectedIndex = Math.max(0, Math.min(chart.pointCount - 1, Math.round(fraction * (chart.pointCount - 1))))
            }
        }
    }
    RowLayout {
        Layout.fillWidth: true
        spacing: 12
        Label { text: "Inspect sample"; color: chart.theme.muted; font.pixelSize: 11 }
        Slider {
            objectName: "sampleSlider"
            Layout.fillWidth: true
            from: 0
            to: Math.max(0, chart.pointCount - 1)
            stepSize: 1
            value: chart.selectedIndex
            enabled: chart.pointCount > 0
            Accessible.name: "Inspect sample"
            onMoved: chart.selectedIndex = Math.round(value)
        }
        Label {
            objectName: "sampleReading"
            text: chart.reading
            color: chart.theme.muted
            font.family: "monospace"
            font.pixelSize: 11
            Layout.preferredWidth: 295
        }
    }
}
