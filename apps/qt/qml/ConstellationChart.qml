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
    property var constellation: ({})
    property bool showMeasured: true
    property bool showIdeal: true
    readonly property color referenceColor: theme.dark ? "#fb7185" : "#b42345"
    signal boundsRequested(real firstI, real lastI, real firstQ, real lastQ)
    signal resetRequested()
    spacing: 6
    onConstellationChanged: plot.requestPaint()
    onShowMeasuredChanged: plot.requestPaint()
    onShowIdealChanged: plot.requestPaint()
    Connections {
        target: chart.theme
        function onDarkChanged() { plot.requestPaint() }
    }
    RowLayout {
        spacing: 6
        MeasurementToggle {
            objectName: "constellationMeasuredToggle"
            text: "Measured"
            Accessible.name: "Show measured constellation"
            theme: chart.theme
            traceColor: chart.theme.accent
            checked: chart.showMeasured
            onClicked: chart.showMeasured = !chart.showMeasured
        }
        MeasurementToggle {
            objectName: "constellationIdealToggle"
            text: "Ideal"
            Accessible.name: "Show ideal constellation"
            theme: chart.theme
            traceColor: chart.referenceColor
            checked: chart.showIdeal
            enabled: chart.constellation.reference_available === true
            onClicked: chart.showIdeal = !chart.showIdeal
        }
        Item { Layout.fillWidth: true }
    }
    Item {
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 180
        Canvas {
            id: plot
            objectName: "constellationPlot"
            anchors.centerIn: parent
            width: Math.max(0, Math.min(parent.width, parent.height))
            height: width
            readonly property real leftMargin: 58
            readonly property real topMargin: 14
            readonly property real plotSize: Math.max(0, width - 80)
            Accessible.role: Accessible.Graphic
            Accessible.name: "Constellation I/Q; drag to zoom, double-click to reset"
            onWidthChanged: requestPaint()
            onHeightChanged: requestPaint()
            onVisibleChanged: requestPaint()
            function inside(horizontal, vertical) {
                return horizontal >= leftMargin && horizontal <= leftMargin + plotSize
                    && vertical >= topMargin && vertical <= topMargin + plotSize
            }
            function inPhase(horizontal) {
                const bounds = chart.constellation.bounds
                return bounds.minimum_i + (horizontal - leftMargin) / plotSize * (bounds.maximum_i - bounds.minimum_i)
            }
            function quadrature(vertical) {
                const bounds = chart.constellation.bounds
                return bounds.maximum_q - (vertical - topMargin) / plotSize * (bounds.maximum_q - bounds.minimum_q)
            }
            onPaint: {
                const context = getContext("2d")
                context.reset()
                const bounds = chart.constellation.bounds
                if (!bounds || plotSize <= 0) {
                    return
                }
                context.font = "11px sans-serif"
                context.lineWidth = 1
                for (let tick = 0; tick <= 4; tick++) {
                    const fraction = tick / 4
                    const horizontal = leftMargin + fraction * plotSize
                    const vertical = topMargin + fraction * plotSize
                    context.strokeStyle = chart.theme.line
                    context.beginPath()
                    context.moveTo(horizontal, topMargin)
                    context.lineTo(horizontal, topMargin + plotSize)
                    context.moveTo(leftMargin, vertical)
                    context.lineTo(leftMargin + plotSize, vertical)
                    context.stroke()
                    context.fillStyle = chart.theme.muted
                    context.textAlign = "center"
                    context.fillText((bounds.minimum_i + fraction * (bounds.maximum_i - bounds.minimum_i)).toFixed(3), horizontal, topMargin + plotSize + 20)
                    context.textAlign = "right"
                    context.fillText((bounds.maximum_q - fraction * (bounds.maximum_q - bounds.minimum_q)).toFixed(3), leftMargin - 8, vertical + 4)
                }
                const layers = [
                    {points: chart.constellation.measured || [], visible: chart.showMeasured, ideal: false, color: chart.theme.accent},
                    {points: chart.constellation.ideal || [], visible: chart.showIdeal, ideal: true, color: chart.referenceColor}
                ]
                for (const layer of layers) {
                    if (!layer.visible) { continue }
                    context.strokeStyle = layer.color
                    context.fillStyle = layer.color
                    context.globalAlpha = layer.ideal ? 1 : 0.75
                    for (const point of layer.points) {
                        const horizontal = leftMargin + (point[0] - bounds.minimum_i) / (bounds.maximum_i - bounds.minimum_i) * plotSize
                        const vertical = topMargin + plotSize - (point[1] - bounds.minimum_q) / (bounds.maximum_q - bounds.minimum_q) * plotSize
                        context.beginPath()
                        if (layer.ideal) {
                            context.moveTo(horizontal - 3, vertical)
                            context.lineTo(horizontal + 3, vertical)
                            context.moveTo(horizontal, vertical - 3)
                            context.lineTo(horizontal, vertical + 3)
                            context.stroke()
                        } else {
                            context.arc(horizontal, vertical, 1.8, 0, Math.PI * 2)
                            context.fill()
                        }
                    }
                }
                context.globalAlpha = 1
                context.fillStyle = chart.theme.muted
                context.textAlign = "center"
                context.fillText("In-phase (I)", leftMargin + plotSize / 2, topMargin + plotSize + 44)
                context.save()
                context.translate(12, topMargin + plotSize / 2)
                context.rotate(-Math.PI / 2)
                context.fillText("Quadrature (Q)", 0, 0)
                context.restore()
                if ((chart.constellation.measured_visible || 0) === 0 && (chart.constellation.ideal_visible || 0) === 0) {
                    context.fillText("No points in this region", leftMargin + plotSize / 2, topMargin + plotSize / 2)
                }
            }
            Rectangle {
                visible: selection.dragging
                x: Math.min(selection.startX, selection.endX)
                y: Math.min(selection.startY, selection.endY)
                width: Math.abs(selection.endX - selection.startX)
                height: Math.abs(selection.endY - selection.startY)
                color: Qt.rgba(chart.theme.accent.r, chart.theme.accent.g, chart.theme.accent.b, 0.12)
                border.color: chart.theme.accent
            }
            MouseArea {
                id: selection
                anchors.fill: parent
                cursorShape: Qt.CrossCursor
                property bool dragging: false
                property real startX: 0
                property real startY: 0
                property real endX: 0
                property real endY: 0
                onPressed: mouse => {
                    if (!chart.constellation.bounds || !plot.inside(mouse.x, mouse.y)) { return }
                    startX = endX = mouse.x
                    startY = endY = mouse.y
                    dragging = true
                }
                onPositionChanged: mouse => {
                    if (!dragging) { return }
                    endX = Math.max(plot.leftMargin, Math.min(plot.leftMargin + plot.plotSize, mouse.x))
                    endY = Math.max(plot.topMargin, Math.min(plot.topMargin + plot.plotSize, mouse.y))
                }
                onReleased: mouse => {
                    if (!dragging) { return }
                    dragging = false
                    if (Math.abs(endX - startX) >= 8 && Math.abs(endY - startY) >= 8) {
                        chart.boundsRequested(plot.inPhase(startX), plot.inPhase(endX), plot.quadrature(startY), plot.quadrature(endY))
                    }
                }
                onCanceled: dragging = false
                onDoubleClicked: chart.resetRequested()
                onWheel: wheel => {
                    if (!chart.constellation.bounds || !plot.inside(wheel.x, wheel.y)) { return }
                    const bounds = chart.constellation.bounds
                    const centerI = plot.inPhase(wheel.x)
                    const centerQ = plot.quadrature(wheel.y)
                    const factor = wheel.angleDelta.y > 0 ? 0.8 : 1.25
                    chart.boundsRequested(centerI + (bounds.minimum_i - centerI) * factor,
                        centerI + (bounds.maximum_i - centerI) * factor,
                        centerQ + (bounds.minimum_q - centerQ) * factor,
                        centerQ + (bounds.maximum_q - centerQ) * factor)
                    wheel.accepted = true
                }
            }
        }
    }
    Label {
        text: (chart.constellation.measured_label || "") + "    ·    " + (chart.constellation.ideal_label || "")
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        wrapMode: Text.Wrap
    }
}
