/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ColumnLayout {
    id: chart
    required property var theme
    property var density: ({})
    spacing: 6
    onDensityChanged: plot.requestPaint()
    Connections {
        target: chart.theme
        function onDarkChanged() { plot.requestPaint() }
    }
    Item {
        Layout.fillWidth: true
        Layout.fillHeight: true
        Canvas {
            id: plot
            objectName: "iqDensityPlot"
            anchors.centerIn: parent
            width: Math.min(parent.width, parent.height)
            height: width
            Accessible.role: Accessible.Graphic
            Accessible.name: "I/Q density scatter plot"
            onWidthChanged: requestPaint()
            onHeightChanged: requestPaint()
            onVisibleChanged: requestPaint()
            onPaint: {
                const context = getContext("2d")
                context.reset()
                const margin = 60
                const size = width - margin - 20
                if (size <= 0 || !chart.density.grid_side) {
                    return
                }
                const lower = chart.density.minimum
                const upper = chart.density.maximum
                context.font = "11px sans-serif"
                context.lineWidth = 1
                for (let tick = 0; tick <= 4; tick++) {
                    const fraction = tick / 4
                    const position = margin + fraction * size
                    context.strokeStyle = chart.theme.line
                    context.beginPath()
                    context.moveTo(position, 20)
                    context.lineTo(position, size + 20)
                    context.moveTo(margin, 20 + fraction * size)
                    context.lineTo(margin + size, 20 + fraction * size)
                    context.stroke()
                    context.fillStyle = chart.theme.muted
                    context.textAlign = "center"
                    context.fillText((lower + fraction * (upper - lower)).toFixed(2), position, size + 38)
                    context.textAlign = "right"
                    context.fillText((upper - fraction * (upper - lower)).toFixed(2), margin - 8, 24 + fraction * size)
                }
                const cellSize = size / chart.density.grid_side
                const cells = chart.density.cells || []
                context.fillStyle = chart.theme.accent
                for (let index = 0; index < cells.length; index++) {
                    const cell = cells[index]
                    context.globalAlpha = 0.25 + 0.75 * Math.sqrt(cell.count / chart.density.maximum_cell_count)
                    context.fillRect(margin + cell.column * cellSize, 20 + size - (cell.row + 1) * cellSize, cellSize, cellSize)
                }
                context.globalAlpha = 1
                context.fillStyle = chart.theme.muted
                context.textAlign = "center"
                context.fillText("In-phase (I)", margin + size / 2, size + 58)
                context.save()
                context.translate(14, 20 + size / 2)
                context.rotate(-Math.PI / 2)
                context.fillText("Quadrature (Q)", 0, 0)
                context.restore()
                if (cells.length === 0) {
                    context.fillText("No valid I/Q samples", margin + size / 2, 20 + size / 2)
                }
            }
        }
    }
    Label {
        text: "I/Q density · " + (chart.density.valid_count || 0) + " valid · " + (chart.density.excluded_count || 0) + " excluded · Color intensity indicates sample count"
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        wrapMode: Text.Wrap
    }
}
