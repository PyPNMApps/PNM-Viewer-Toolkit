/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    name: "CaptureFileDrop"
    when: windowShown
    visible: true
    width: 600
    height: 400
    Theme { id: appearance }
    CaptureDropArea {
        id: target
        theme: appearance
        anchors.fill: parent
    }
    SignalSpy { id: opened; target: target; signalName: "fileDropped" }
    function init() {
        target.blocked = false
        appearance.preference = 1
        opened.clear()
    }
    function dragEvent(urls, actions) {
        return {hasUrls: urls.length > 0, urls: urls, supportedActions: actions,
            accepted: false, acceptedAction: Qt.IgnoreAction,
            accept: function(action) { this.accepted = true; this.acceptedAction = action }}
    }
    function test_single_file_routes_with_copy_action() {
        const event = dragEvent(["file:///tmp/capture%20with%20spaces.bin"], Qt.CopyAction | Qt.MoveAction)
        target.handleDrop(event)
        compare(opened.count, 1)
        compare(event.accepted, true)
        compare(event.acceptedAction, Qt.CopyAction)
        verify(opened.signalArguments[0][0].toString().startsWith("file:///tmp/capture"))
        const windowsFile = dragEvent(["file:///C:/Captures/rxmer.bin"], Qt.CopyAction)
        target.handleDrop(windowsFile)
        compare(opened.count, 2)
    }
    function test_reject_non_files_multiple_and_move_only() {
        for (const event of [dragEvent([], Qt.CopyAction),
                             dragEvent(["https://example.invalid/capture.bin"], Qt.CopyAction),
                             dragEvent(["file:///tmp/a", "file:///tmp/b"], Qt.CopyAction),
                             dragEvent(["file:///tmp/a"], Qt.MoveAction)]) {
            target.handleDrop(event)
            compare(event.accepted, false)
            compare(event.acceptedAction, Qt.IgnoreAction)
        }
        compare(opened.count, 0)
    }
    function test_blocked_and_theme_changes() {
        const event = dragEvent(["file:///tmp/capture.bin"], Qt.CopyAction)
        verify(target.accepts(event))
        target.blocked = true
        compare(target.enabled, false)
        target.handleDrop(event)
        compare(opened.count, 0)
        compare(event.accepted, false)
        target.blocked = false
        appearance.preference = 2
        target.handleDrop(event)
        compare(opened.count, 1)
        compare(event.acceptedAction, Qt.CopyAction)
    }
}
