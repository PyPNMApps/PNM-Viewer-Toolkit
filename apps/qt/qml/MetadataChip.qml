/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
Rectangle {
    id: chip
    required property Theme theme
    required property string title
    required property string value
    implicitWidth: content.implicitWidth + 24
    implicitHeight: 30
    radius: height / 2
    color: theme.dark ? "#0affffff" : "#06000000"
    border.color: theme.line
    Accessible.role: Accessible.StaticText
    Accessible.name: title + ": " + value
    Row {
        id: content
        anchors.centerIn: parent
        spacing: 8
        Label { text: chip.title; color: chip.theme.accent; font.pixelSize: 12; font.bold: true }
        Label { text: chip.value; color: chip.theme.text; font.pixelSize: 12 }
    }
}
