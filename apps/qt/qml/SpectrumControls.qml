/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Basic as Basic
import QtQuick.Layouts

RowLayout {
    id: controls
    required property Theme theme
    property int sampleCount: 0
    property int averageWindow: 0
    property bool showAverage: true
    function presentation() { return {input: input.text, showAverage: showAverage} }
    function restorePresentation(state) {
        input.text = state ? state.input : "101"
        showAverage = state ? state.showAverage : true
    }
    signal averageRequested(int windowPoints)
    spacing: 8
    Label { text: "N-point"; color: controls.theme.muted }
    Basic.TextField {
        id: input
        objectName: "spectrumAverageInput"
        text: "101"
        Layout.preferredWidth: 90
        validator: IntValidator { bottom: 1; top: 2147483647 }
        inputMethodHints: Qt.ImhDigitsOnly
        Accessible.name: "Window average points"
        color: controls.theme.text
        selectionColor: controls.theme.accent
        selectedTextColor: controls.theme.background
        background: Rectangle {
            radius: 4
            color: controls.theme.button
            border.color: input.activeFocus ? controls.theme.accent : controls.theme.line
        }
    }
    ThemeButton {
        objectName: "spectrumWindowAverageButton"
        theme: controls.theme
        text: "WindowAvg"
        enabled: input.acceptableInput && controls.sampleCount > 0
        onClicked: {
            controls.showAverage = true
            controls.averageRequested(Number.fromLocaleString(Qt.locale(), input.text))
        }
    }
    MeasurementToggle {
        objectName: "spectrumAverageVisibilityButton"
        theme: controls.theme
        text: "Show average"
        Accessible.name: "Show spectrum window average"
        implicitWidth: 148
        traceColor: controls.theme.spectrumAverage
        enabled: controls.averageWindow > 0
        checked: enabled && controls.showAverage
        onClicked: controls.showAverage = !controls.showAverage
    }
    Item { Layout.fillWidth: true }
}
