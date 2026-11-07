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
    property var metadata: ({profiles: [], profile_index: 0})
    property var viewport: ({start: 0, count: 0, total: 0})
    readonly property var fec: viewport.fec_summary || ({points: []})
    // CaptureTime is an unsigned 32-bit epoch-second field.
    readonly property real maximumEpochSeconds: 4294967295
    readonly property color totalColor: theme.dark ? "#79a9ff" : "#2467c7"
    readonly property color correctedColor: theme.dark ? "#51d69a" : "#118749"
    readonly property color uncorrectableColor: theme.dark ? "#ff7885" : "#c73346"
    signal profileRequested(int index)
    signal timeRangeRequested(double first, double last)
    signal rangeRequested(int start, int count)
    onViewportChanged: { selection.dragging = false; plot.requestPaint() }
    onThemeChanged: plot.requestPaint()
    Connections { target: chart.theme; function onDarkChanged() { plot.requestPaint() } function onChartStyleChanged() { plot.requestPaint() } }
    function utc(timestamp) {
        return timestamp === undefined || timestamp === null ? "—" : new Date(timestamp * 1000).toISOString().replace("T", " ").replace(".000Z", " UTC")
    }
    function countLabel(value) { return value === undefined || value === null ? "—" : Number(value).toFixed(0) }
    function resetZoom() { rangeRequested(0, viewport.total) }
    function selectTime(first, last) {
        if (fec.first_timestamp === null || fec.last_timestamp === null) { return }
        const span = fec.last_timestamp - fec.first_timestamp
        const lower = Math.max(0, Math.min(1, Math.min(first, last)))
        const upper = Math.max(0, Math.min(1, Math.max(first, last)))
        const startTime = Math.ceil(fec.first_timestamp + lower * span)
        const endTime = Math.floor(fec.first_timestamp + upper * span)
        if (startTime > endTime) { return }
        timeRangeRequested(startTime, endTime)
    }
    RowLayout {
        Layout.fillWidth: true
        Label { text: "Profile"; color: chart.theme.muted }
        ComboBox {
            id: profiles
            objectName: "fecProfileSelector"
            Accessible.name: "FEC profile"
            model: (chart.metadata.profiles || []).map(profile => "Profile " + profile.profile_id)
            currentIndex: chart.metadata.profile_index || 0
            enabled: model.length > 0
            onActivated: index => chart.profileRequested(index)
            Layout.preferredWidth: 145
        }
        Label {
            text: chart.metadata.summary_type === 2 ? "10-minute summary · 1-second intervals" : chart.metadata.summary_type === 3 ? "24-hour summary · 1-minute intervals" : "Summary type " + (chart.metadata.summary_type === undefined ? "—" : chart.metadata.summary_type)
            color: chart.theme.muted
            Layout.fillWidth: true
            elide: Text.ElideRight
        }
        CheckBox { id: totals; objectName: "fecTotalToggle"; text: "Total CW"; checked: true; palette.windowText: chart.totalColor; onCheckedChanged: plot.requestPaint() }
        CheckBox { id: corrected; objectName: "fecCorrectedToggle"; text: "Corrected CW"; checked: true; palette.windowText: chart.correctedColor; onCheckedChanged: plot.requestPaint() }
        CheckBox { id: uncorrectable; objectName: "fecUncorrectableToggle"; text: "Uncorrectable CW"; checked: true; palette.windowText: chart.uncorrectableColor; onCheckedChanged: plot.requestPaint() }
    }
    Canvas {
        id: plot
        objectName: "fecPlot"
        Layout.fillWidth: true
        Layout.fillHeight: true
        Layout.minimumHeight: 180
        property real leftMargin: 85
        property real rightMargin: 85
        property real topMargin: 25
        property real bottomMargin: 43
        Accessible.role: Accessible.Graphic
        Accessible.name: "FEC interval counts by UTC time; left total codewords, right errors; drag to zoom and double-click to reset"
        onWidthChanged: requestPaint()
        onHeightChanged: requestPaint()
        onPaint: {
            const context = getContext("2d")
            context.clearRect(0, 0, width, height)
            const usableWidth = Math.max(1, width - leftMargin - rightMargin)
            const usableHeight = Math.max(1, height - topMargin - bottomMargin)
            const points = chart.fec.points || []
            context.font = "11px sans-serif"
            context.fillStyle = chart.theme.muted
            context.textAlign = "left"
            context.fillText("Total CW / interval", leftMargin, 13)
            context.textAlign = "right"
            context.fillText("Error CW / interval", width - rightMargin, 13)
            for (let tick = 0; tick <= 4; tick += 1) {
                const fraction = tick / 4
                const vertical = topMargin + usableHeight * (1 - fraction)
                context.strokeStyle = chart.theme.line
                context.beginPath(); context.moveTo(leftMargin, vertical); context.lineTo(width - rightMargin, vertical); context.stroke()
                context.fillStyle = chart.totalColor
                context.textAlign = "right"
                context.fillText(((chart.fec.total_maximum || 1) * fraction).toExponential(1), leftMargin - 10, vertical + 4)
                context.fillStyle = chart.theme.muted
                context.textAlign = "left"
                context.fillText(((chart.fec.error_maximum || 1) * fraction).toExponential(1), width - rightMargin + 10, vertical + 4)
            }
            if (!points.length) {
                context.fillStyle = chart.theme.muted; context.textAlign = "center"
                context.fillText("No FEC intervals in this selection", width / 2, height / 2)
                return
            }
            const first = chart.fec.first_timestamp
            const last = chart.fec.last_timestamp
            const span = Math.max(1, last - first)
            function horizontal(timestamp) { return leftMargin + (last === first ? 0.5 : (timestamp - first) / span) * usableWidth }
            function draw(field, maximum, color, enabled) {
                if (!enabled) { return }
                context.strokeStyle = color; context.lineWidth = chart.theme.chartLineWidth; context.beginPath()
                for (let index = 0; index < points.length; index += 1) {
                    const point = points[index]
                    const horizontalPosition = horizontal(point.timestamp)
                    const vertical = topMargin + usableHeight * (1 - point[field] / maximum)
                    if (index === 0 || point.break_before) { context.moveTo(horizontalPosition, vertical) }
                    else { context.lineTo(horizontalPosition, vertical) }
                }
                context.stroke()
                // Mark single records and gap endpoints; isolated readings stay visible.
                context.fillStyle = color
                for (let index = 0; index < points.length; index += 1) {
                    if (points.length === 1 || points[index].break_before || (index + 1 < points.length && points[index + 1].break_before)) {
                        context.beginPath(); context.arc(horizontal(points[index].timestamp), topMargin + usableHeight * (1 - points[index][field] / maximum), 2, 0, Math.PI * 2); context.fill()
                    }
                }
            }
            draw("total_codewords", chart.fec.total_maximum || 1, chart.totalColor, totals.checked)
            draw("corrected", chart.fec.error_maximum || 1, chart.correctedColor, corrected.checked)
            draw("uncorrectable", chart.fec.error_maximum || 1, chart.uncorrectableColor, uncorrectable.checked)
            context.fillStyle = chart.theme.muted; context.textAlign = "center"
            for (let tick = 0; tick <= 4; tick += 1) {
                const timestamp = first + (last - first) * tick / 4
                context.fillText(chart.utc(Math.round(timestamp)).slice(11, 19), leftMargin + usableWidth * tick / 4, height - 23)
            }
            context.fillText("Time · UTC", width / 2, height - 4)
        }
        Rectangle {
            visible: selection.dragging
            x: plot.leftMargin + Math.min(selection.first, selection.last) * (plot.width - plot.leftMargin - plot.rightMargin)
            y: plot.topMargin
            width: Math.abs(selection.last - selection.first) * (plot.width - plot.leftMargin - plot.rightMargin)
            height: plot.height - plot.topMargin - plot.bottomMargin
            color: chart.theme.dark ? "#3051d6bf" : "#30007e75"
            border.color: chart.theme.accent
        }
        MouseArea {
            id: selection
            anchors.fill: parent
            property bool dragging: false
            property real first: 0
            property real last: 0
            cursorShape: Qt.CrossCursor
            function fraction(horizontal) { return Math.max(0, Math.min(1, (horizontal - plot.leftMargin) / (plot.width - plot.leftMargin - plot.rightMargin))) }
            onPressed: mouse => {
                if (mouse.x < plot.leftMargin || mouse.x > plot.width - plot.rightMargin || mouse.y < plot.topMargin || mouse.y > plot.height - plot.bottomMargin) { mouse.accepted = false; return }
                first = fraction(mouse.x); last = first; dragging = true
            }
            onPositionChanged: mouse => { if (dragging) { last = fraction(mouse.x) } }
            onReleased: mouse => {
                if (!dragging) { return }
                last = fraction(mouse.x); dragging = false
                if (Math.abs(last - first) * (plot.width - plot.leftMargin - plot.rightMargin) >= 6) { chart.selectTime(first, last) }
            }
            onCanceled: dragging = false
            onDoubleClicked: { dragging = false; chart.resetZoom() }
            onWheel: wheel => {
                if (!chart.viewport.count || chart.fec.first_timestamp === chart.fec.last_timestamp) { return }
                const anchor = fraction(wheel.x)
                const span = Math.max(1, chart.fec.last_timestamp - chart.fec.first_timestamp)
                const nextSpan = span * (wheel.angleDelta.y > 0 ? 0.8 : 1.25)
                const first = Math.max(0, Math.floor(chart.fec.first_timestamp + anchor * (span - nextSpan)))
                const last = Math.min(chart.maximumEpochSeconds, Math.ceil(first + nextSpan))
                chart.timeRangeRequested(first, last)
                wheel.accepted = true
            }
        }
    }
    RowLayout {
        Layout.fillWidth: true
        spacing: 20
        Label { text: "Window sums"; color: chart.theme.muted }
        Label { text: "Total " + chart.countLabel(chart.fec.total_codewords); color: chart.totalColor }
        Label { text: "Corrected " + chart.countLabel(chart.fec.corrected); color: chart.correctedColor }
        Label { text: "Uncorrectable " + chart.countLabel(chart.fec.uncorrectable); color: chart.uncorrectableColor }
        Item { Layout.fillWidth: true }
    }
    Label {
        text: chart.fec.last ? "Last interval · Total " + chart.countLabel(chart.fec.last.total_codewords) + " · Corrected " + chart.countLabel(chart.fec.last.corrected) + " · Uncorrectable " + chart.countLabel(chart.fec.last.uncorrectable) : "Last interval · —"
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        elide: Text.ElideRight
    }
    Label {
        text: chart.viewport.count > 0 ? "Time selection: " + chart.utc(chart.fec.first_timestamp) + " – " + chart.utc(chart.fec.last_timestamp) + " · Intervals: " + chart.viewport.count + (chart.fec.decimated ? " · Overview" : "") : "No time selection · double-click to reset"
        color: chart.theme.muted
        font.pixelSize: 11
        Layout.fillWidth: true
        elide: Text.ElideRight
    }
}
