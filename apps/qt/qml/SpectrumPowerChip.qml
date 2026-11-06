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
    implicitWidth: content.implicitWidth + 28
    implicitHeight: 54
    radius: 10
    color: theme.accentSoft
    border.color: theme.line
    Accessible.role: Accessible.StaticText
    Accessible.name: title + ": " + value
    Column {
        id: content
        anchors.centerIn: parent
        spacing: 3
        Label { text: chip.title; color: chip.theme.accent; font.pixelSize: 11; font.bold: true }
        Label { text: chip.value; color: chip.theme.text; font.pixelSize: 16; font.bold: true }
    }
}
