/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import "helpers"

RenderTestCase {
    name: "RenderSynchronization"
    width: 200
    height: 200
    visible: true
    when: windowShown

    Rectangle {
        id: unchangedItem
        width: 100
        height: 100
        color: "#123456"
    }

    function test_unchanged_scene_renders_consecutive_requested_frames() {
        verify(waitForFrame(unchangedItem))
        compare(grabImage(unchangedItem).pixel(50, 50), unchangedItem.color)
        verify(waitForFrame(unchangedItem))
        compare(grabImage(unchangedItem).pixel(50, 50), unchangedItem.color)
    }
}
