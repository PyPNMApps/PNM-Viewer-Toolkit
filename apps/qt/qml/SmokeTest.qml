/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Dialogs

// Explicit CI-only command: --smoke-test INPUT_FILE_URL NEW_OUTPUT_FILE_URL.
// Exercise the real asynchronous backend and one rendered frame, then exit.
Timer {
    required property var viewerBackend
    required property var applicationWindow
    required property Item surface
    required property var openCaptureDialog
    required property var exportCaptureDialog
    readonly property int argumentIndex: Qt.application.arguments.indexOf("--smoke-test")
    property int stage: 0
    property int ticks: 0
    interval: 150
    repeat: true
    running: argumentIndex >= 0
    onTriggered: {
        ticks += 1
        if (ticks > 260 || viewerBackend.error_message.length > 0) {
            console.error("UI_SMOKE_FAILED: " + viewerBackend.error_message)
            stop()
            Qt.exit(1)
            return
        }
        if (stage === 0) {
            const examples = viewerBackend.examples_folder().toString()
            const currentFolder = openCaptureDialog.currentFolder.toString()
            if (examples.length > 0 && currentFolder !== examples &&
                currentFolder !== examples + "/" && currentFolder + "/" !== examples) {
                console.error("UI_SMOKE_FAILED: initial Open folder is not the bundled examples")
                stop()
                Qt.exit(1)
                return
            }
            if (Qt.platform.os === "windows" &&
                ((openCaptureDialog.options & FileDialog.DontUseNativeDialog) !== 0 ||
                 (exportCaptureDialog.options & FileDialog.DontUseNativeDialog) !== 0)) {
                console.error("UI_SMOKE_FAILED: Windows native file dialogs disabled")
                stop()
                Qt.exit(1)
                return
            }
            if (Qt.platform.os === "linux" && Qt.application.name !== "pnm-viewer-toolkit") {
                console.error("UI_SMOKE_FAILED: desktop application identity mismatch")
                stop()
                Qt.exit(1)
                return
            }
            if (Qt.application.arguments.length !== argumentIndex + 3) {
                console.error("UI_SMOKE_FAILED: expected input and new output file URLs")
                stop()
                Qt.exit(2)
                return
            }
            viewerBackend.open_file(Qt.application.arguments[argumentIndex + 1])
            stage = 1
        } else if (stage === 1 && viewerBackend.loaded && !viewerBackend.busy) {
            const viewport = JSON.parse(viewerBackend.viewport_json)
            if (Object.keys(viewport).length === 0 || !applicationWindow.visible) {
                console.error("UI_SMOKE_FAILED: no visible capture")
                stop()
                Qt.exit(1)
                return
            }
            viewerBackend.export_json(Qt.application.arguments[argumentIndex + 2])
            stage = 2
        } else if (stage === 2 && !viewerBackend.busy && viewerBackend.status === "Full-resolution JSON exported") {
            stage = 3
            stop()
            const queued = surface.grabToImage(function(result) {
                if (result.url.toString().length === 0) {
                    console.error("UI_SMOKE_FAILED: rendering failed")
                    Qt.exit(1)
                    return
                }
                console.log("UI_SMOKE_OK")
                Qt.quit()
            })
            if (!queued) {
                console.error("UI_SMOKE_FAILED: frame request rejected")
                Qt.exit(1)
            }
        }
    }
}
