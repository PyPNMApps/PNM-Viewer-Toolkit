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
    property var attachment: ({})
    property string resultMode: ""
    property bool working: false
    property string errorMessage: ""
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
    function utc(epoch) { return epoch === undefined ? "—" : new Date(epoch*1000).toISOString().replace("T"," ").replace(".000Z"," UTC") }
    function mac(address) { if (typeof address === "string") return address; return (address || []).map(value => value.toString(16).padStart(2,"0")).join(":") }
    Label { text: "PRE-EQ COMPARISON"; color: panel.theme.muted; font.pixelSize: 10 }
    ThemeButton { theme: panel.theme; text: "Original capture"; Layout.fillWidth: true; onClicked: panel.showOriginal() }
    ThemeButton { theme: panel.theme; text: "Correction & Delta"; Layout.fillWidth: true; onClicked: panel.showComparison() }
    Rectangle {
        id: dropZone
        objectName: "preEqAttachmentTarget"
        Layout.fillWidth: true
        implicitHeight: 76
        radius: 8
        color: panel.theme.button
        border.color: panel.theme.line
        Label { anchors.fill: parent; anchors.margins: 10; text: "Drop Last Update here\nor choose a file"; horizontalAlignment: Text.AlignHCenter; verticalAlignment: Text.AlignVCenter; color: panel.theme.text; wrapMode: Text.Wrap }
    }
    ThemeButton { objectName: "choosePreEqUpdate"; theme: panel.theme; text: panel.checkedPair ? "Replace correction…" : "Choose Last Update…"; Layout.fillWidth: true; enabled: !panel.working; onClicked: panel.chooseFile() }
    Label { objectName: "preEqPrecheckStatus"; text: panel.checkedPair ? "Precheck passed · newer by " + panel.attachment.pair.file_time_difference_s + " s" : "Attach a compatible newer PNN7 file."; color: panel.checkedPair ? panel.theme.accent : panel.theme.muted; Layout.fillWidth: true; wrapMode: Text.Wrap }
    Label { visible: panel.checkedPair; Layout.fillWidth: true; wrapMode: Text.Wrap; font.pixelSize: 11; color: panel.theme.muted; text: panel.checkedPair ? "CM  " + panel.mac(panel.attachment.pair.mac_address) + "\nCMTS  " + panel.mac(panel.attachment.pair.cmts_mac_address) + "\nOriginal  " + panel.utc(panel.attachment.pair.original_header.capture_time) + "\nCorrection  " + panel.utc(panel.attachment.pair.correction_header.capture_time) : "" }
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
    Label { objectName: "preEqComparisonError"; text: panel.errorMessage; visible: text.length>0; Layout.fillWidth: true; wrapMode: Text.Wrap; color: panel.theme.text }
    Label { text: "Mode is not stored in the file. Results are predicted, not confirmed modem changes."; Layout.fillWidth: true; wrapMode: Text.Wrap; color: panel.theme.muted; font.pixelSize: 11 }
}
