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
    property bool showSoft: true
    property bool showHard: true
    property bool showBoundaries: true
    readonly property color referenceColor: theme.dark ? "#fb7185" : "#b42345"
    signal boundsRequested(real firstI, real lastI, real firstQ, real lastQ)
    signal resetRequested()
    spacing: 6
    onConstellationChanged: plot.requestPaint()
    onShowSoftChanged: plot.requestPaint()
    onShowHardChanged: plot.requestPaint()
    onShowBoundariesChanged: plot.requestPaint()
    Connections {
        target: chart.theme
        function onDarkChanged() { plot.requestPaint() }
    }
    RowLayout {
        spacing: 6
        MeasurementToggle {
            objectName: "constellationSoftToggle"
            text: "Soft decisions"
            Accessible.name: "Show soft decisions"
            theme: chart.theme
            implicitWidth: 164
            traceColor: chart.theme.accent
            checked: chart.showSoft
            onClicked: chart.showSoft = !chart.showSoft
        }
        MeasurementToggle {
            objectName: "constellationHardToggle"
            text: "Hard decisions"
            Accessible.name: "Show hard decisions"
            theme: chart.theme
            implicitWidth: 164
            traceColor: chart.referenceColor
            checked: chart.showHard
            enabled: chart.constellation.reference_available === true
            onClicked: chart.showHard = !chart.showHard
        }
        MeasurementToggle {
            objectName: "constellationBoundaryToggle"
            text: "Decision boundaries"
            Accessible.name: "Show decision boundaries"
            theme: chart.theme
            implicitWidth: 164
            traceColor: chart.theme.muted
            checked: chart.showBoundaries
            enabled: chart.constellation.reference_available === true
            onClicked: chart.showBoundaries = !chart.showBoundaries
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
            readonly property real leftMargin: 12
            readonly property real topMargin: 12
            readonly property real plotSize: Math.max(0, width - 24)
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
                // Boundary geometry is computed and clipped in Rust; QML only maps it to pixels.
                if (chart.showBoundaries) {
                    context.strokeStyle = chart.theme.muted
                    context.globalAlpha = 0.4
                    context.lineWidth = 1
                    for (const segment of (chart.constellation.decision_boundaries || [])) {
                        context.beginPath()
                        for (let endpoint = 0; endpoint < 2; endpoint++) {
                            const point = segment[endpoint]
                            const horizontal = leftMargin + (point[0] - bounds.minimum_i) / (bounds.maximum_i - bounds.minimum_i) * plotSize
                            const vertical = topMargin + plotSize - (point[1] - bounds.minimum_q) / (bounds.maximum_q - bounds.minimum_q) * plotSize
                            if (endpoint === 0) {
                                context.moveTo(horizontal, vertical)
                            } else {
                                context.lineTo(horizontal, vertical)
                            }
                        }
                        context.stroke()
                    }
                    context.globalAlpha = 1
                }
                const layers = [
                    {points: chart.constellation.soft_decisions || [], visible: chart.showSoft, hard: false, color: chart.theme.accent},
                    {points: chart.constellation.hard_decisions || [], visible: chart.showHard, hard: true, color: chart.referenceColor}
                ]
                for (const layer of layers) {
                    if (!layer.visible) { continue }
                    context.strokeStyle = layer.color
                    context.fillStyle = layer.color
                    context.globalAlpha = layer.hard ? 1 : 0.75
                    for (const point of layer.points) {
                        const horizontal = leftMargin + (point[0] - bounds.minimum_i) / (bounds.maximum_i - bounds.minimum_i) * plotSize
                        const vertical = topMargin + plotSize - (point[1] - bounds.minimum_q) / (bounds.maximum_q - bounds.minimum_q) * plotSize
                        context.beginPath()
                        if (layer.hard) {
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
                if ((chart.constellation.soft_visible || 0) === 0 && (chart.constellation.hard_visible || 0) === 0) {
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
        text: (chart.constellation.soft_label || "") + "    ·    " + (chart.constellation.hard_label || "")
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        wrapMode: Text.Wrap
    }
}
