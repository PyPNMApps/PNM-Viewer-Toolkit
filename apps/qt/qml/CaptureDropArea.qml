/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls

DropArea {
    id: target
    required property Theme theme
    property bool blocked: false
    enabled: !blocked
    signal fileDropped(url captureUrl)

    // Filter drag metadata only. Rust validates the file and its contents.
    function accepts(event) {
        return !blocked && event.hasUrls && event.urls.length === 1
            && event.urls[0].toString().toLowerCase().startsWith("file:")
            && (event.supportedActions & Qt.CopyAction) !== 0
    }
    function handleDrop(event) {
        event.accepted = false
        if (!accepts(event)) {
            return
        }
        const captureUrl = event.urls[0]
        // Never accept a proposed move: file managers may remove the source.
        event.accept(Qt.CopyAction)
        fileDropped(captureUrl)
    }
    onEntered: drag => { drag.accepted = accepts(drag) }
    onPositionChanged: drag => { drag.accepted = accepts(drag) }
    onDropped: drop => handleDrop(drop)

    Rectangle {
        anchors.fill: parent
        anchors.margins: 5
        visible: target.containsDrag && !target.blocked
        radius: 12
        color: target.theme.dark ? "#24163a3d" : "#24e0f4ef"
        border.width: 2
        border.color: target.theme.accent
        Rectangle {
            anchors.centerIn: parent
            width: hint.implicitWidth + 48
            height: hint.implicitHeight + 28
            radius: 12
            color: target.theme.panel
            border.color: target.theme.accent
            Label {
                id: hint
                anchors.centerIn: parent
                text: "Drop to open PNM capture"
                color: target.theme.text
                font.pixelSize: 18
            }
        }
    }
}
