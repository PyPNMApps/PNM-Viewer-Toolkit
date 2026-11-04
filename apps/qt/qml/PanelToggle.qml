/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

ToolButton {
    id: control
    required property Theme theme
    property bool rightSide: false
    implicitWidth: 34
    implicitHeight: 32
    ToolTip.visible: hovered
    ToolTip.delay: 500
    ToolTip.text: Accessible.name
    contentItem: Item {
        implicitWidth: 22
        implicitHeight: 18
        Rectangle {
            anchors.centerIn: parent
            width: 22
            height: 18
            radius: 3
            color: "transparent"
            border.color: control.checked ? control.theme.accent : control.theme.muted
            Rectangle {
                x: control.rightSide ? 14 : 3
                y: 3
                width: 5
                height: 12
                radius: 1
                color: control.checked ? control.theme.accent : control.theme.muted
            }
        }
    }
}
