/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

Button {
    id: button
    required property Theme theme
    property color traceColor: theme.chartLineColor
    property bool showIndicator: true
    readonly property bool selected: checked || highlighted
    implicitWidth: 108
    implicitHeight: 30
    width: implicitWidth
    height: implicitHeight
    checkable: true
    padding: 8
    contentItem: Row {
        spacing: 6
        Rectangle {
            visible: button.showIndicator
            width: 10; height: 3
            anchors.verticalCenter: parent.verticalCenter
            color: button.traceColor
        }
        Label { text: button.text; color: button.theme.text; font.pixelSize: 12 }
    }
    background: Rectangle {
        radius: 5
        color: button.selected ? button.theme.accentSoft : button.theme.button
        border.color: button.selected ? button.traceColor : button.theme.line
        border.width: button.activeFocus ? 2 : 1
        opacity: button.enabled ? (button.hovered || button.selected ? 1 : 0.8) : 0.4
    }
}
