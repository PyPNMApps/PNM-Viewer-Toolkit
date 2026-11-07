/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest

TestCase {
    // Observe a newly requested frame, including on an unchanged Cocoa scene.
    // Flush layout first and request Canvas painting when applicable. Preserve
    // the normal render timeout/failure; never retry or bypass an assertion.
    function waitForFrame(item) {
        verify(item !== null && item !== undefined, "Missing render target")
        const targetWindow = item.Window.window
        verify(targetWindow !== null, "Render target has no window")
        verify(waitForPolish(targetWindow), "Window layout did not settle")
        if (typeof item.requestPaint === "function") {
            item.requestPaint()
        }
        targetWindow.update()
        return waitForRendering(item)
    }
}
