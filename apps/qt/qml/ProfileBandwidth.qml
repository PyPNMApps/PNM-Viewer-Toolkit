/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

ToolButton {
    id: chip
    required property Theme theme
    property var metadata: ({profiles: [], selected_indices: [], bandwidth_summary: "None selected"})
    readonly property bool popupVisible: bandwidthDialog.visible
    objectName: "profileBandwidthChip"
    Accessible.name: "Show profile bandwidth"
    implicitWidth: chipContent.implicitWidth + 24
    implicitHeight: 30
    onClicked: bandwidthDialog.open()
    onMetadataChanged: bandwidthDialog.close()
    contentItem: Row {
        id: chipContent
        spacing: 8
        Label { text: "Profile BW"; color: chip.theme.accent; font.pixelSize: 12; font.bold: true }
        Label { text: chip.metadata.bandwidth_summary || "None selected"; color: chip.theme.text; font.pixelSize: 12 }
        Label { text: "⌄"; color: chip.theme.muted; font.pixelSize: 12 }
    }
    background: Rectangle {
        radius: height / 2
        color: chip.hovered ? chip.theme.accentSoft : (chip.theme.dark ? "#0affffff" : "#06000000")
        border.color: chip.activeFocus ? chip.theme.accent : chip.theme.line
    }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: bandwidthDialog; theme: chip.theme }
        id: bandwidthDialog
        objectName: "profileBandwidthDialog"
        parent: Overlay.overlay
        anchors.centerIn: parent
        title: "Profile bandwidth"
        width: Math.min(460, parent.width - 24)
        height: Math.min(400, parent.height - 24)
        modal: true
        focus: true
        standardButtons: Dialog.Close
        background: Rectangle { color: chip.theme.panel; border.color: chip.theme.line; radius: 10 }
        contentItem: ColumnLayout {
            spacing: 8
            RowLayout {
                Layout.fillWidth: true
                Label { text: "Profile"; color: chip.theme.muted; font.bold: true; Layout.fillWidth: true }
                Label { text: "Bandwidth"; color: chip.theme.muted; font.bold: true }
            }
            ScrollView {
                id: ratesScroll
                Layout.fillWidth: true
                Layout.fillHeight: true
                contentWidth: availableWidth
                clip: true
                ColumnLayout {
                    width: ratesScroll.availableWidth
                    spacing: 4
                    Repeater {
                        model: chip.metadata.profiles || []
                        Rectangle {
                            required property int index
                            required property var modelData
                            readonly property bool selected: (chip.metadata.selected_indices || []).indexOf(index) >= 0
                            Layout.fillWidth: true
                            implicitHeight: 34
                            radius: 4
                            color: selected ? chip.theme.accentSoft : "transparent"
                            RowLayout {
                                anchors.fill: parent
                                anchors.margins: 8
                                Label {
                                    text: "Profile " + modelData.profile_id
                                    color: chip.theme.text
                                    Layout.fillWidth: true
                                }
                                Label {
                                    objectName: "profileBandwidthRate" + index
                                    text: modelData.bandwidth_text
                                    Accessible.name: "Profile " + modelData.profile_id + " bandwidth " + text
                                    color: chip.theme.text
                                    font.family: "monospace"
                                    HoverHandler { id: rateHover }
                                    ToolTip.visible: rateHover.hovered
                                    ToolTip.text: modelData.bandwidth_bps === null ? "Unavailable" : modelData.bandwidth_bps.toString() + " bit/s"
                                }
                            }
                        }
                    }
                }
            }
            Label {
                text: "Data carriers only · ideal gross rate before overhead"
                color: chip.theme.muted
                font.pixelSize: 11
                wrapMode: Text.Wrap
                Layout.fillWidth: true
            }
        }
    }
}
