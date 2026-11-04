/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

QtObject {
    // 0 follows the platform, 1 is Dark, 2 is Light. Session-only preference.
    property int preference: 0
    property int systemScheme: Qt.styleHints.colorScheme
    readonly property bool dark: preference === 1 || (preference === 0 && systemScheme === Qt.Dark)
    readonly property color background: dark ? "#0c1422" : "#f3f6f9"
    readonly property color panel: dark ? "#111e30" : "#ffffff"
    readonly property color sidebar: dark ? "#0a1220" : "#edf2f6"
    readonly property color text: dark ? "#e8f0f7" : "#142b3e"
    readonly property color muted: dark ? "#93a8be" : "#566c7e"
    readonly property color line: dark ? "#24354b" : "#d8e2ea"
    readonly property color accent: dark ? "#51d6bf" : "#007e75"
    readonly property color accentSoft: dark ? "#163a3d" : "#e0f4ef"
    readonly property color button: dark ? "#19293e" : "#f6f9fb"
}
