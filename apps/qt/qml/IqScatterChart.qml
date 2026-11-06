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
    property bool densityMode: false
    readonly property color sparseColor: "#3b82f6"
    readonly property color denseColor: "#ef4444"
    /*
     * Maps a cell count to the blue-to-red display palette.
     * count is a valid cell's sample count; the maximum comes from Rust.
     * Returns a display color only; Rust owns binning and sample counts.
     */
    function densityColor(count) {
        const fraction = count / Math.max(1, density.maximum_cell_count || 0)
        return Qt.rgba(sparseColor.r + fraction * (denseColor.r - sparseColor.r),
            sparseColor.g + fraction * (denseColor.g - sparseColor.g),
            sparseColor.b + fraction * (denseColor.b - sparseColor.b), 1)
    }
    spacing: 6
    onDensityModeChanged: plot.requestPaint()
    onDensityChanged: plot.requestPaint()
    Connections {
        target: chart.theme
        function onDarkChanged() { plot.requestPaint() }
    }
    RowLayout {
        Layout.fillWidth: true
        Label { text: "Display"; color: chart.theme.muted }
        ButtonGroup { id: displayModes }
        ThemeButton {
            objectName: "iqDotsButton"
            theme: chart.theme
            text: "Dots"
            checkable: true
            checked: !chart.densityMode
            highlighted: checked
            ButtonGroup.group: displayModes
            Layout.preferredWidth: 90
            onClicked: chart.densityMode = false
        }
        ThemeButton {
            objectName: "iqDensityButton"
            theme: chart.theme
            text: "Density"
            checkable: true
            checked: chart.densityMode
            highlighted: checked
            ButtonGroup.group: displayModes
            Layout.preferredWidth: 90
            onClicked: chart.densityMode = true
        }
        Item { Layout.fillWidth: true }
        Label { visible: chart.densityMode; text: "Sparse"; color: chart.theme.muted }
        Rectangle {
            visible: chart.densityMode
            Layout.preferredWidth: 110
            Layout.preferredHeight: 8
            radius: 4
            gradient: Gradient {
                orientation: Gradient.Horizontal
                GradientStop { position: 0; color: chart.sparseColor }
                GradientStop { position: 1; color: chart.denseColor }
            }
        }
        Label { visible: chart.densityMode; text: "Dense"; color: chart.theme.muted }
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
            Accessible.name: chart.densityMode ? "I/Q density map" : "I/Q dot scatter plot"
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
                if (chart.densityMode) {
                    for (let index = 0; index < cells.length; index++) {
                        const cell = cells[index]
                        context.fillStyle = chart.densityColor(cell.count)
                        context.fillRect(margin + cell.column * cellSize, 20 + size - (cell.row + 1) * cellSize, cellSize, cellSize)
                    }
                } else {
                    const points = chart.density.points || []
                    context.globalAlpha = 0.8
                    for (let index = 0; index < points.length; index++) {
                        const point = points[index]
                        const horizontal = margin + (point[0] - lower) / (upper - lower) * size
                        const vertical = 20 + size - (point[1] - lower) / (upper - lower) * size
                        context.beginPath()
                        context.arc(horizontal, vertical, 1.8, 0, 2 * Math.PI)
                        context.fill()
                    }
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
        text: (chart.densityMode ? "Density · Blue = sparse, red = dense"
            : "Dots · " + (chart.density.points || []).length + " displayed")
            + " · " + (chart.density.valid_count || 0) + " valid · " + (chart.density.excluded_count || 0) + " excluded"
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        wrapMode: Text.Wrap
    }
}
