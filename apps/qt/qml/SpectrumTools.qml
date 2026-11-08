/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Basic as Basic
import QtQuick.Layouts

ColumnLayout {
    id: tools
    required property Theme theme
    readonly property bool popupVisible: frequencyDialog.visible
    property var workspace: ({tabs: []})
    property var validateRange: function(start, stop, unit) { return "No spectrum capture" }
    readonly property var powerChips: {
        const power = workspace.power
        if (power === undefined || power === null) {
            return []
        }
        return [
            {title: "Integrated level", value: power.level_dbmv.toFixed(1) + " dBmV"},
            {title: "Frequency range", value: power.start_hz.toFixed(0) + "–" + power.stop_hz.toFixed(0) + " Hz"},
            {title: "Bandwidth", value: power.bandwidth_hz.toFixed(0) + " Hz"}
        ]
    }
    signal rangeRequested(real start, real stop, string unit)
    signal tabRequested(int index)
    signal closeRequested(int index)
    function openFrequency() { frequencyDialog.open() }
    spacing: 4
    ScrollView {
        Layout.fillWidth: true
        Layout.preferredHeight: 34
        clip: true
        RowLayout {
            Repeater {
                model: tools.workspace.tabs || []
                RowLayout {
                    required property var modelData
                    required property int index
                    ThemeButton {
                        theme: tools.theme
                        text: modelData.title
                        highlighted: tools.workspace.active === index
                        onClicked: tools.tabRequested(index)
                    }
                    ThemeButton {
                        visible: index > 0
                        Layout.preferredWidth: 32
                        Layout.minimumWidth: 32
                        Layout.maximumWidth: 32
                        leftPadding: 0
                        rightPadding: 0
                        theme: tools.theme
                        text: "×"
                        Accessible.name: "Close frequency range " + modelData.title
                        onClicked: tools.closeRequested(index)
                    }
                }
            }
        }
    }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: frequencyDialog; theme: tools.theme }
        id: frequencyDialog
        objectName: "spectrumFrequencyDialog"
        title: "Frequency"
        modal: true
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: Math.min(480, parent ? parent.width - 24 : 480)
        property string unit: "Hz"
        readonly property string validationMessage: {
            if (!startInput.acceptableInput || !stopInput.acceptableInput) {
                return "Enter a valid Start and Stop frequency."
            }
            return tools.validateRange(Number.fromLocaleString(Qt.locale(), startInput.text),
                Number.fromLocaleString(Qt.locale(), stopInput.text), unit)
        }
        onOpened: {
            unit = "Hz"
            const unitButtons = units.buttons
            if (unitButtons.length) { unitButtons[0].checked = true }
            startInput.text = String(tools.workspace.minimum_hz || 0)
            stopInput.text = String(tools.workspace.maximum_hz || 0)
        }
        background: Rectangle { color: tools.theme.panel; border.color: tools.theme.line; radius: 8 }
        header: Label { text: frequencyDialog.title; padding: 14; color: tools.theme.text; font.bold: true }
        contentItem: ColumnLayout {
            spacing: 10
            Label {
                text: "Available: " + tools.workspace.minimum_hz + "–" + tools.workspace.maximum_hz + " Hz"
                color: tools.theme.muted
                wrapMode: Text.Wrap
                Layout.fillWidth: true
            }
            ButtonGroup { id: units }
            RowLayout {
                Repeater {
                    objectName: "spectrumUnitRepeater"
                    model: ["Hz", "kHz", "MHz", "GHz"]
                    Basic.RadioButton {
                        id: unitButton
                        required property string modelData
                        objectName: "spectrumUnit_" + modelData
                        text: modelData
                        ButtonGroup.group: units
                        indicator: Rectangle {
                            implicitWidth: 18
                            implicitHeight: 18
                            x: unitButton.leftPadding
                            y: (unitButton.height - height) / 2
                            radius: 9
                            color: tools.theme.panel
                            border.color: unitButton.checked ? tools.theme.accent : tools.theme.muted
                            border.width: 1
                            Rectangle {
                                anchors.centerIn: parent
                                width: 10
                                height: 10
                                radius: 5
                                visible: unitButton.checked
                                color: tools.theme.accent
                            }
                        }
                        onClicked: frequencyDialog.unit = modelData
                        contentItem: Label { text: unitButton.text; leftPadding: unitButton.indicator.width + 6; color: tools.theme.text; verticalAlignment: Text.AlignVCenter }
                    }
                }
            }
            Label { text: "Start"; color: tools.theme.text }
            ThemeTextField {
                theme: tools.theme
                id: startInput
                objectName: "spectrumStartFrequency"
                Layout.fillWidth: true
                validator: DoubleValidator { bottom: 0; top: 4294967295; decimals: 9; notation: DoubleValidator.StandardNotation }
                Accessible.name: "Start frequency"
            }
            Label { text: "Stop"; color: tools.theme.text }
            ThemeTextField {
                theme: tools.theme
                id: stopInput
                objectName: "spectrumStopFrequency"
                Layout.fillWidth: true
                validator: DoubleValidator { bottom: 0; top: 4294967295; decimals: 9; notation: DoubleValidator.StandardNotation }
                Accessible.name: "Stop frequency"
            }
            Label { text: frequencyDialog.validationMessage; color: tools.theme.muted; wrapMode: Text.Wrap; Layout.fillWidth: true }
            RowLayout {
                Item { Layout.fillWidth: true }
                ThemeButton { theme: tools.theme; text: "Cancel"; onClicked: frequencyDialog.reject() }
                ThemeButton {
                    theme: tools.theme
                    objectName: "openSpectrumRange"
                    text: "Open range"
                    enabled: frequencyDialog.validationMessage === ""
                    onClicked: {
                        tools.rangeRequested(Number.fromLocaleString(Qt.locale(), startInput.text),
                            Number.fromLocaleString(Qt.locale(), stopInput.text), frequencyDialog.unit)
                        frequencyDialog.accept()
                    }
                }
            }
        }
    }
}
