/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick

Rectangle {
    id: overlay
    required property Theme theme
    property var result: null
    readonly property string label: "Estimated MER " + (
        result && result.calculation_status === "available"
        && typeof result.mer_db === "number" && isFinite(result.mer_db)
            ? result.mer_db.toFixed(1) + " dB" : "—")
    objectName: "constellationMerOverlay"
    width: caption.implicitWidth + 16
    height: caption.implicitHeight + 10
    radius: 5
    color: Qt.rgba(theme.background.r, theme.background.g, theme.background.b, 0.9)
    border.color: theme.line
    // No input handler: drags/wheel events reach the chart beneath this badge.
    Text {
        id: caption
        anchors.centerIn: parent
        text: overlay.label
        color: overlay.theme.text
        font.pixelSize: 12
        Accessible.role: Accessible.StaticText
        Accessible.name: text
    }
}
