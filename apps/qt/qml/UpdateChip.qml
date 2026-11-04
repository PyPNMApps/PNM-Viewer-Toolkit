/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Button {
    id: control
    required property var updates
    objectName: "updateChip"
    visible: updates.update_state === "available"
    text: "Update"
    implicitHeight: 26
    leftPadding: 12
    rightPadding: 12
    topPadding: 3
    bottomPadding: 3
    Accessible.name: "Update PNM Viewer Toolkit"
    ToolTip.visible: hovered
    ToolTip.text: "A newer release is available. View update options."
    contentItem: Text {
        text: control.text
        color: "white"
        font.bold: true
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
    background: Rectangle {
        radius: 4
        color: control.hovered ? "#2ea043" : "#238636"
        border.color: control.visualFocus ? "white" : "#2ea043"
        border.width: control.visualFocus ? 2 : 1
    }
}
