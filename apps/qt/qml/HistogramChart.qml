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
    property var viewport: ({start: 0, count: 0, total: 0})
    readonly property var histogram: viewport.histogram || ({bars: [], maximum: 1, total_hits: 0, grouped: false})
    signal rangeRequested(int start, int count)
    onViewportChanged: { selection.dragging = false; plot.requestPaint() }
    function selectRange(first, last) {
        if (viewport.count < 2) { return }
        const lower = Math.max(0, Math.min(1, Math.min(first, last)))
        const upper = Math.max(0, Math.min(1, Math.max(first, last)))
        const start = Math.min(viewport.count - 1, Math.floor(lower * viewport.count))
        const end = Math.max(start + 1, Math.ceil(upper * viewport.count))
        rangeRequested(viewport.start + start, end - start)
    }
    function resetZoom() { rangeRequested(0, viewport.total) }
    Canvas {
        id: plot
        objectName: "histogramPlot"
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 180
        property real leftMargin: 85
        property real rightMargin: 20
        property real topMargin: 24
        property real bottomMargin: 45
        Accessible.role: Accessible.Graphic
        Accessible.name: "Histogram hit counts by bin index; drag to zoom, double-click to reset"
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        onVisibleChanged: requestPaint()
        Connections {
            target: chart.theme
            function onDarkChanged() { plot.requestPaint() }
        }
        onPaint: {
            const context = getContext("2d")
            context.reset()
            const plotWidth = width - leftMargin - rightMargin
            const plotHeight = height - topMargin - bottomMargin
            if (plotWidth <= 0 || plotHeight <= 0) { return }
            context.font = "11px sans-serif"
            context.fillStyle = chart.theme.muted
            context.strokeStyle = chart.theme.line
            for (let tick = 0; tick <= 4; tick++) {
                const fraction = tick / 4
                const vertical = topMargin + (1 - fraction) * plotHeight
                context.beginPath()
                context.moveTo(leftMargin, vertical)
                context.lineTo(width - rightMargin, vertical)
                context.stroke()
                context.textAlign = "right"
                const level = Math.round(chart.histogram.maximum * fraction)
                const label = level >= 1000000 ? level.toExponential(2) : level.toLocaleString()
                context.fillText(label, leftMargin - 10, vertical + 4)
                context.textAlign = "center"
                if (chart.viewport.count > 0) {
                    context.fillText(Math.round(chart.viewport.start + fraction * (chart.viewport.count - 1)).toString(), leftMargin + fraction * plotWidth, height - 23)
                }
            }
            context.textAlign = "left"
            context.fillText(chart.histogram.grouped ? "Hit count · summed bins" : "Hit count", leftMargin, 13)
            context.textAlign = "center"
            context.fillText("Bin index", leftMargin + plotWidth / 2, height - 3)
            const bars = chart.histogram.bars
            context.fillStyle = chart.theme.accent
            for (let index = 0; index < bars.length; index++) {
                const bar = bars[index]
                const horizontal = leftMargin + (bar.first_bin - chart.viewport.start) / chart.viewport.count * plotWidth
                const barWidth = bar.bin_count / chart.viewport.count * plotWidth
                const barHeight = bar.hit_count / chart.histogram.maximum * plotHeight
                context.fillRect(horizontal, topMargin + plotHeight - barHeight, Math.max(0.5, barWidth - Math.min(1, barWidth * 0.1)), barHeight)
            }
            if (!bars.length) {
                context.fillStyle = chart.theme.muted
                context.fillText("No histogram bins", leftMargin + plotWidth / 2, topMargin + plotHeight / 2)
            }
        }
        Rectangle {
            visible: selection.dragging
            x: plot.leftMargin + Math.min(selection.first, selection.last) * (plot.width - plot.leftMargin - plot.rightMargin)
            y: plot.topMargin
            width: Math.abs(selection.last - selection.first) * (plot.width - plot.leftMargin - plot.rightMargin)
            height: plot.height - plot.topMargin - plot.bottomMargin
            color: chart.theme.accent
            opacity: 0.2
        }
        MouseArea {
            id: selection
            anchors.fill: parent
            property bool dragging: false
            property real first: 0
            property real last: 0
            cursorShape: Qt.CrossCursor
            function fraction(horizontal) {
                return Math.max(0, Math.min(1, (horizontal - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin)))
            }
            onPressed: mouse => {
                if (mouse.x < plot.leftMargin || mouse.x > plot.width - plot.rightMargin || mouse.y < plot.topMargin || mouse.y > plot.height - plot.bottomMargin) {
                    mouse.accepted = false
                    return
                }
                first = fraction(mouse.x)
                last = first
                dragging = true
            }
            onPositionChanged: mouse => { if (dragging) { last = fraction(mouse.x) } }
            onReleased: mouse => {
                if (!dragging) { return }
                last = fraction(mouse.x)
                dragging = false
                if (Math.abs(last - first) * (plot.width - plot.leftMargin - plot.rightMargin) >= 6) {
                    chart.selectRange(first, last)
                }
            }
            onCanceled: dragging = false
            onDoubleClicked: { dragging = false; chart.resetZoom() }
            onWheel: wheel => {
                if (chart.viewport.total < 2) { return }
                const anchor = fraction(wheel.x)
                const count = Math.max(1, Math.min(chart.viewport.total, Math.round(chart.viewport.count * (wheel.angleDelta.y > 0 ? 0.8 : 1.25))))
                const start = Math.max(0, Math.min(chart.viewport.total - count, Math.round(chart.viewport.start + anchor * (chart.viewport.count - count))))
                chart.rangeRequested(start, count)
                wheel.accepted = true
            }
        }
    }
    Label {
        text: chart.viewport.count > 0 ? "Bin selection: " + chart.viewport.start + " – " + (chart.viewport.start + chart.viewport.count - 1) + " · Hits: " + chart.histogram.total_hits.toFixed(0) : "No bin selection"
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        elide: Text.ElideRight
    }
}
