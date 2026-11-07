/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

Dialog {
    id: dialog
    required property Theme theme
    title: "Chart settings"
    width: 440
    modal: true
    standardButtons: Dialog.Close
    palette.window: theme.panel
    palette.windowText: theme.text
    palette.base: theme.background
    palette.text: theme.text
    palette.button: theme.button
    palette.buttonText: theme.text
    readonly property var widths: [0.25, 0.5, 0.75, 1, 1.5, 2, 3, 4]
    readonly property var colorChoices: [
        {label: "Main trace", setting: "traceColorOverride", effective: "chartLineColor"},
        {label: "Min", setting: "minimumColorOverride", effective: "rxmerMinimum"},
        {label: "Avg", setting: "averageColorOverride", effective: "rxmerAverage"},
        {label: "Max", setting: "maximumColorOverride", effective: "rxmerMaximum"},
        {label: "WindowAvg", setting: "windowAverageColorOverride", effective: "spectrumAverage"}
    ]
    property string editingColor: ""
    ColorDialog {
        id: colorPicker
        objectName: "chartColorPicker"
        title: "Choose chart color"
        onAccepted: {
            if (dialog.editingColor.length > 0) {
                dialog.theme[dialog.editingColor] = selectedColor.toString()
            }
        }
    }
    contentItem: ColumnLayout {
        spacing: 12
        RowLayout {
            Label { text: "Line thickness"; color: dialog.theme.text; Layout.fillWidth: true }
            ComboBox {
                objectName: "chartLineWidthSelector"
                Accessible.name: "Chart line thickness"
                model: dialog.widths.map(value => value + " px")
                currentIndex: dialog.widths.indexOf(dialog.theme.chartLineWidth)
                onActivated: dialog.theme.chartLineWidth = dialog.widths[currentIndex]
            }
        }
        Repeater {
            model: dialog.colorChoices
            delegate: RowLayout {
                required property var modelData
                Layout.fillWidth: true
                Label { text: modelData.label; color: dialog.theme.text; Layout.fillWidth: true }
                Rectangle {
                    Layout.preferredWidth: 28
                    Layout.preferredHeight: 24
                    radius: 4
                    color: dialog.theme[modelData.effective]
                    border.color: dialog.theme.line
                }
                ThemeButton {
                    objectName: "chartColor_" + modelData.setting
                    theme: dialog.theme
                    Accessible.name: "Choose " + modelData.label + " color"
                    text: dialog.theme[modelData.effective].toString()
                    Layout.preferredWidth: 110
                    onClicked: {
                        dialog.editingColor = modelData.setting
                        colorPicker.selectedColor = dialog.theme[modelData.effective]
                        colorPicker.open()
                    }
                }
            }
        }
        ThemeButton {
            objectName: "resetChartStyleButton"
            theme: dialog.theme
            text: "Restore defaults"
            onClicked: dialog.theme.resetChartStyle()
        }
    }
}
