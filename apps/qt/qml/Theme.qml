/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

QtObject {
    // Chart preferences share the existing session lifetime of Appearance.
    readonly property real defaultChartLineWidth: 0.75
    property real chartLineWidth: defaultChartLineWidth
    property string traceColorOverride: ""
    property string minimumColorOverride: ""
    property string averageColorOverride: ""
    property string maximumColorOverride: ""
    property string windowAverageColorOverride: ""
    readonly property color chartLineColor: traceColorOverride.length ? traceColorOverride : accent
    signal chartStyleChanged()
    onChartLineWidthChanged: chartStyleChanged()
    onChartLineColorChanged: chartStyleChanged()
    onRxmerMinimumChanged: chartStyleChanged()
    onRxmerAverageChanged: chartStyleChanged()
    onRxmerMaximumChanged: chartStyleChanged()
    onSpectrumAverageChanged: chartStyleChanged()
    // Restore theme-aware colors and the requested default trace width.
    function resetChartStyle() {
        chartLineWidth = defaultChartLineWidth
        traceColorOverride = ""
        minimumColorOverride = ""
        averageColorOverride = ""
        maximumColorOverride = ""
        windowAverageColorOverride = ""
    }
    function carrierLabel(kind) {
        const labels = {data: "Data", continuous_pilot: "Pilot", plc: "PLC", exclusion: "Excluded",
            zero_bit_loaded: "Zero-loaded", reserved: "Reserved", unknown: "Unknown"}
        return labels[kind] || kind
    }
    function carrierColor(kind) {
        const colors = dark
            ? {data: "#51d6bf", continuous_pilot: "#f5c66a", plc: "#79b8ff", exclusion: "#ff9b70", zero_bit_loaded: "#a9d66c", reserved: "#ce9bff", unknown: "#ff92bb"}
            : {data: "#007e75", continuous_pilot: "#996000", plc: "#2563b9", exclusion: "#b64b20", zero_bit_loaded: "#517c13", reserved: "#8544bb", unknown: "#b82c68"}
        return colors[kind] || muted
    }
    // Stable profile colors are shared by toggle legends and chart traces.
    function profileColor(profileId) {
        const palette = dark
            ? ["#51d6bf", "#79b8ff", "#f5c66a", "#ce9bff", "#ff92bb", "#a9d66c", "#ff9b70", "#92d7e8"]
            : ["#007e75", "#2563b9", "#996000", "#8544bb", "#b82c68", "#517c13", "#b64b20", "#197d99"]
        return palette[profileId % palette.length]
    }
    // 0 follows the platform, 1 is Dark, 2 is Light. Session-only preference.
    property int preference: 0
    property int systemScheme: Qt.styleHints.colorScheme
    readonly property bool dark: preference === 1 || (preference === 0 && systemScheme === Qt.Dark)
    readonly property color background: dark ? "#0c1422" : "#f3f6f9"
    readonly property color panel: dark ? "#111e30" : "#ffffff"
    readonly property color sidebar: dark ? "#0a1220" : "#edf2f6"
    readonly property color text: dark ? "#e8f0f7" : "#142b3e"
    readonly property color muted: dark ? "#93a8be" : "#566c7e"
    readonly property color line: dark ? "#24354b" : "#d8e2ea"
    readonly property color accent: dark ? "#51d6bf" : "#007e75"
    readonly property color rxmerMinimum: minimumColorOverride.length ? minimumColorOverride : (dark ? "#64d98b" : "#237c3f")
    readonly property color rxmerAverage: averageColorOverride.length ? averageColorOverride : (dark ? "#69aaff" : "#2563a6")
    readonly property color rxmerMaximum: maximumColorOverride.length ? maximumColorOverride : (dark ? "#ff7777" : "#b42318")
    readonly property color spectrumAverage: windowAverageColorOverride.length ? windowAverageColorOverride : (dark ? "#ff6b6b" : "#c62828")
    readonly property color accentSoft: dark ? "#163a3d" : "#e0f4ef"
    readonly property color button: dark ? "#19293e" : "#f6f9fb"
}
