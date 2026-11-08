/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls.Basic as Basic

Basic.TextArea {
    id: control
    required property Theme theme
    property ThemePalette themePalette: ThemePalette { targetControl: control; theme: control.theme }
    color: enabled ? theme.text : theme.muted
    placeholderTextColor: theme.muted
    selectionColor: theme.accent
    selectedTextColor: theme.background
    selectByMouse: true
    textFormat: TextEdit.PlainText
    background: Rectangle { color: control.theme.panel }
}
