/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    id: dialog
    required property var theme
    required property var updates
    required property string appVersion
    property url iconSource: theme.dark ? "qrc:/branding/icon-dark.png" : "qrc:/branding/icon-light.png"
    signal openReleaseRequested(url address)
    signal restartRequested()
    property bool captureBusy: false
    // Emit the close request only after Rust starts the updated executable.
    // Active capture/export work finishes first; failed launches remain retryable.
    function restartWhenReady() {
        if (!captureBusy && updates.install_state === "installed" && updates.restart_updated()) {
            restartRequested()
        }
    }
    onCaptureBusyChanged: {
        if (!captureBusy && updates.install_state === "installed") {
            Qt.callLater(restartWhenReady)
        }
    }
    Connections {
        target: dialog.updates
        function onInstall_stateChanged() {
            if (dialog.updates.install_state === "installed") {
                Qt.callLater(dialog.restartWhenReady)
            }
        }
    }
    objectName: "aboutDialog"
    title: "About PNM Viewer Toolkit"
    width: 500
    modal: true
    readonly property bool installBusy: updates.install_state === "downloading" || updates.install_state === "installing" || updates.install_state === "restarting"
    standardButtons: Dialog.Close
    contentItem: ColumnLayout {
        spacing: 12
        Image {
            source: dialog.iconSource
            sourceSize.width: 64
            sourceSize.height: 64
            Layout.alignment: Qt.AlignHCenter
        }
        Label { text: "PNM Viewer Toolkit"; color: dialog.theme.text; font.pixelSize: 20; font.bold: true }
        Label { text: "Version " + dialog.appVersion + " · In development"; color: dialog.theme.muted }
        Label { text: "Offline inspection of DOCSIS PNM captures."; color: dialog.theme.text }
        Label {
            text: "Desktop: GPL-3.0-only\nCore libraries: Apache-2.0\nAcknowledgment: PyPNM Toolkit"
            color: dialog.theme.muted
        }
        Button {
            text: "GitHub project"
            onClicked: Qt.openUrlExternally("https://github.com/PyPNMApps/PNM-Viewer-Toolkit")
        }
        Rectangle { Layout.fillWidth: true; implicitHeight: 1; color: dialog.theme.line }
        Label { text: "Software updates"; color: dialog.theme.text; font.bold: true }
        Label {
            objectName: "latestReleaseLabel"
            text: dialog.updates.latest_version.length > 0
                ? "Latest release: " + dialog.updates.latest_version : "Latest release: —"
            color: dialog.theme.muted
        }
        Label {
            objectName: "updateStatusLabel"
            text: dialog.updates.update_status
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
            color: dialog.theme.text
            Accessible.name: text
        }
        RowLayout {
            Button {
                objectName: "checkUpdatesButton"
                text: dialog.updates.update_state === "checking" ? "Checking…" : "Check for updates"
                enabled: dialog.updates.update_state !== "checking" && !dialog.installBusy && dialog.updates.install_state !== "installed"
                onClicked: dialog.updates.check_updates()
            }
            Button {
                objectName: "releaseDownloadsButton"
                text: dialog.updates.update_state === "available" ? "Release notes and downloads" : "View releases"
                onClicked: dialog.openReleaseRequested(dialog.updates.release_url)
            }
        }
        RowLayout {
            Button {
                objectName: "installUpdateButton"
                text: "Install update and restart"
                visible: dialog.updates.install_available
                enabled: !dialog.installBusy && !dialog.captureBusy
                onClicked: dialog.updates.install_update()
            }
            Button {
                objectName: "cancelUpdateButton"
                text: "Cancel download"
                visible: dialog.updates.install_state === "downloading"
                onClicked: dialog.updates.cancel_install()
            }
            Button {
                objectName: "restartUpdateButton"
                text: "Restart viewer"
                visible: dialog.updates.install_state === "installed"
                enabled: !dialog.captureBusy
                onClicked: dialog.restartWhenReady()
            }
        }
        ProgressBar {
            objectName: "installProgress"
            Layout.fillWidth: true
            visible: dialog.installBusy
            from: 0
            to: 100
            value: dialog.updates.install_progress
            indeterminate: dialog.updates.install_state === "installing" || dialog.updates.install_state === "restarting"
        }
        Label {
            objectName: "installStatusLabel"
            text: dialog.updates.install_state === "installed" && dialog.captureBusy
                ? "Update installed. Waiting for the active operation before restarting…"
                : dialog.updates.install_status
            visible: text.length > 0
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
            color: dialog.theme.text
        }
        Label {
            text: dialog.updates.install_available || dialog.installBusy
                ? "The viewer restarts after installation. Export your work before starting. Administrator approval is required."
                : "In-app installation requires an installed Ubuntu x64 build and matching release packages. Other builds use release downloads."
            color: dialog.theme.muted
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
    }
}
