/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls.Basic as Basic

Basic.Button {
    id: control
    required property var theme
    // Marks a pending action without changing the button fill or text contrast.
    property bool borderHighlighted: false
    property color outlineColor: theme.accent
    readonly property color foreground: !enabled ? theme.muted
        : highlighted ? (theme.dark ? theme.background : theme.panel) : theme.text
    leftPadding: 12
    rightPadding: 12
    topPadding: 6
    bottomPadding: 6
    contentItem: Text {
        text: control.text
        font: control.font
        color: control.foreground
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }
    // Paint both sides of the contrast pair; platform palette roles alone
    // produced pale Windows backgrounds with pale application text.
    background: Rectangle {
        implicitWidth: 90
        implicitHeight: 30
        radius: 4
        color: !control.enabled ? control.theme.button
            : control.highlighted ? control.theme.accent
            : control.down || control.hovered ? control.theme.accentSoft : control.theme.button
        border.color: control.visualFocus || control.borderHighlighted ? control.outlineColor : control.theme.line
        border.width: control.visualFocus || control.borderHighlighted ? 2 : 1
    }
}
