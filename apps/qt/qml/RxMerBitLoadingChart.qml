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
    property var analysis: ({})
    onAnalysisChanged: plot.requestPaint()
    Connections { target: chart.theme; function onDarkChanged() { plot.requestPaint() } }
    RowLayout {
        MeasurementToggle { id: measured; theme: chart.theme; text: "Per carrier"; checked: true; onCheckedChanged: plot.requestPaint() }
        MeasurementToggle { id: uniform; theme: chart.theme; text: "Uniform"; traceColor: "#f46b75"; checked: true; onCheckedChanged: plot.requestPaint() }
        MeasurementToggle { id: ideal; theme: chart.theme; text: "Shannon"; traceColor: "#79a9ff"; checked: false; onCheckedChanged: plot.requestPaint() }
    }
    Canvas {
        id: plot
        objectName: "rxmerBitLoadingPlot"
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 150
        Accessible.role: Accessible.Graphic
        Accessible.name: "RxMER estimated bits per symbol by frequency"
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        onVisibleChanged: requestPaint()
        onPaint: {
            const context = getContext("2d")
            context.reset()
            const left = 58
            const right = width - 16
            const top = 16
            const bottom = height - 48
            if (right <= left || bottom <= top) {
                return
            }
            const selection = chart.analysis.selection || {}
            const model = chart.analysis.model || {}
            const first = selection.start_frequency_hz || 0
            const last = selection.stop_frequency_hz || first
            const span = Math.max(1, last - first)
            const maximum = model.maximum_bit_loading || 13
            context.font = "11px sans-serif"
            context.lineWidth = 1
            for (let tick = 0; tick <= 4; tick++) {
                const fraction = tick / 4
                const horizontal = left + fraction * (right - left)
                const vertical = bottom - fraction * (bottom - top)
                context.strokeStyle = chart.theme.line
                context.beginPath()
                context.moveTo(horizontal, top); context.lineTo(horizontal, bottom)
                context.moveTo(left, vertical); context.lineTo(right, vertical)
                context.stroke()
                context.fillStyle = chart.theme.muted
                context.textAlign = "center"
                context.fillText(((first + fraction * (last - first)) / 1000000).toFixed(3), horizontal, bottom + 18)
                context.textAlign = "right"
                context.fillText((maximum * fraction).toFixed(1), left - 8, vertical + 4)
            }
            const points = chart.analysis.subcarriers || []
            function horizontal(point) { return left + (point.frequency_hz - first) / span * (right - left) }
            function vertical(bits) { return bottom - bits / maximum * (bottom - top) }
            function trace(field, color, gaps) {
                context.strokeStyle = color
                context.fillStyle = color
                context.lineWidth = 1.4
                context.beginPath()
                let previous = null
                for (let index = 0; index < points.length; index++) {
                    const point = points[index]
                    const bits = point[field]
                    if (bits === null || bits === undefined) {
                        previous = null
                        continue
                    }
                    const x = horizontal(point)
                    const y = vertical(bits)
                    if (previous && gaps.indexOf(point.sample_index) < 0) {
                        context.lineTo(x, vertical(previous[field]))
                        context.lineTo(x, y)
                    } else {
                        context.moveTo(x, y)
                    }
                    previous = point
                }
                context.stroke()
                // Mark single carriers as well as disconnected source runs.
                for (let index = 0; index < points.length; index++) {
                    const point = points[index]
                    if (point[field] !== null && point[field] !== undefined) {
                        context.fillRect(horizontal(point) - 1, vertical(point[field]) - 1, 2, 2)
                    }
                }
            }
            if (ideal.checked) {
                trace("ideal_bit_loading", "#79a9ff", chart.analysis.ideal_plot_gaps || [])
            }
            if (measured.checked) {
                trace("bit_loading", chart.theme.accent, chart.analysis.plot_gaps || [])
            }
            const common = (chart.analysis.summary || {}).uniform_bit_loading
            if (uniform.checked && common !== undefined && common !== null) {
                context.strokeStyle = "#f46b75"
                context.lineWidth = 2
                context.setLineDash([7, 5])
                context.beginPath(); context.moveTo(left, vertical(common)); context.lineTo(right, vertical(common)); context.stroke()
                context.setLineDash([])
            }
            context.fillStyle = chart.theme.muted
            context.textAlign = "center"
            context.fillText("Frequency · MHz", (left + right) / 2, height - 4)
            context.save(); context.translate(13, (top + bottom) / 2); context.rotate(-Math.PI / 2)
            context.fillText("Bits per symbol", 0, 0); context.restore()
            if (points.length === 0) {
                context.fillText("No analysis available", (left + right) / 2, (top + bottom) / 2)
            }
        }
    }
}
