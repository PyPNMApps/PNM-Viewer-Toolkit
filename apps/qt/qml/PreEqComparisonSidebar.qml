/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ColumnLayout {
    id: panel
    required property var theme
    property bool comparisonSelected: false
    property var attachment: ({})
    property string resultMode: ""
    property bool working: false
    readonly property bool checkedPair: !!attachment.pair
    readonly property string selectedMode: mode.currentValue || ""
    property alias dropTarget: dropZone
    signal chooseFile()
    signal calculate(string mode)
    signal removeAttachment()
    signal showComparison()
    signal showOriginal()
    signal exportResult()
    spacing: 8
    onAttachmentChanged: mode.currentIndex = 0
    Label { text: "TOOLS"; color: panel.theme.muted; font.pixelSize: 10 }
    ThemeButton { objectName: "showOriginalPreEq"; theme: panel.theme; text: "Original capture"; borderHighlighted: !panel.comparisonSelected; Layout.fillWidth: true; onClicked: panel.showOriginal() }
    ThemeButton { objectName: "showPreEqCompare"; theme: panel.theme; text: "Compare"; borderHighlighted: panel.comparisonSelected; Layout.fillWidth: true; onClicked: panel.showComparison() }
    ColumnLayout {
        objectName: "preEqCompareControls"
        visible: panel.comparisonSelected
        Layout.fillWidth: true
        spacing: 8
        Rectangle {
            id: dropZone
            objectName: "preEqAttachmentTarget"
            Layout.fillWidth: true
            implicitHeight: 112
            radius: 8
            color: panel.theme.button
            border.color: panel.theme.line
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 10
                Label {
                    text: "Drop Last Update here"
                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    color: panel.theme.text
                    wrapMode: Text.Wrap
                }
                ThemeButton {
                    objectName: "choosePreEqUpdate"
                    theme: panel.theme
                    text: "Browse…"
                    Accessible.name: "Browse for a Last Update file"
                    Layout.fillWidth: true
                    enabled: !panel.working
                    onClicked: panel.chooseFile()
                }
            }
        }
        ComboBox {
            id: mode
            objectName: "preEqUpdateMode"
            Accessible.name: "Pre-EQ correction mode"
            Layout.fillWidth: true
            enabled: panel.checkedPair && !panel.working
            textRole: "label"; valueRole: "value"
            model: [{label:"Choose update mode…",value:""},{label:"Adjust (multiply)",value:"adjust"},{label:"Set (replace)",value:"set"}]
        }
        ThemeButton { objectName: "calculatePreEqComparison"; theme: panel.theme; text: panel.resultMode ? "Recalculate" : "Compare"; Layout.fillWidth: true; enabled: panel.checkedPair && panel.selectedMode.length>0 && !panel.working; borderHighlighted: panel.resultMode.length>0 && panel.selectedMode!==panel.resultMode; onClicked: panel.calculate(panel.selectedMode) }
        RowLayout {
            ThemeButton { objectName: "removePreEqUpdate"; theme: panel.theme; text: "Remove"; enabled: panel.checkedPair || panel.working; onClicked: panel.removeAttachment() }
            ThemeButton { theme: panel.theme; text: "Export JSON"; enabled: panel.resultMode.length>0 && !panel.working; onClicked: panel.exportResult() }
        }
        BusyIndicator { running: panel.working; visible: running; Layout.alignment: Qt.AlignHCenter; implicitWidth: 30; implicitHeight: 30 }
    }
}
