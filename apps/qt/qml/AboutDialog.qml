/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    property ThemePalette themePalette: ThemePalette { targetControl: dialog; theme: dialog.theme }
    id: dialog
    required property var theme
    required property var updates
    required property string appVersion
    property url iconSource: theme.dark ? "qrc:/branding/icon-dark.png" : "qrc:/branding/icon-light.png"
    signal openReleaseRequested(url address)
    signal restartRequested()
    signal openUninstallRequested()
    signal openIssueRequested(url address)
    property string platformOs: Qt.platform.os
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
        Label { objectName: "installedVersionLabel"; text: "Version " + dialog.appVersion; color: dialog.theme.muted }
        RowLayout {
            ThemeButton {
                theme: dialog.theme
                objectName: "projectButton"
                text: "GitHub project"
                onClicked: Qt.openUrlExternally("https://github.com/PyPNMApps/PNM-Viewer-Toolkit")
            }
            ThemeButton {
                theme: dialog.theme
                objectName: "reportIssueButton"
                text: "Report issue"
                ToolTip.visible: hovered
                ToolTip.text: "Open a GitHub issue draft to review before submitting"
                onClicked: dialog.openIssueRequested(dialog.updates.issue_report_url)
            }
        }
        Rectangle { Layout.fillWidth: true; implicitHeight: 1; color: dialog.theme.line }
        Label { objectName: "softwareSectionLabel"; text: "Software"; color: dialog.theme.text; font.bold: true }
        Label {
            objectName: "updateStatusLabel"
            text: dialog.updates.update_status
            textFormat: Text.PlainText
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
            color: dialog.theme.text
            Accessible.name: text
        }
        Flow {
            objectName: "softwareActions"
            Layout.fillWidth: true
            spacing: 6
            ThemeButton {
                theme: dialog.theme
                objectName: "checkUpdatesButton"
                text: dialog.updates.update_state === "checking" ? "Checking…" : "Check for updates"
                enabled: dialog.updates.update_state !== "checking" && !dialog.installBusy && dialog.updates.install_state !== "installed"
                onClicked: dialog.updates.check_updates()
            }
            ThemeButton {
                theme: dialog.theme
                objectName: "downloadInstallerButton"
                text: dialog.updates.download_label
                visible: dialog.updates.download_url.length > 0
                enabled: !dialog.installBusy && dialog.updates.update_state !== "checking"
                onClicked: dialog.openReleaseRequested(dialog.updates.download_url)
            }
            ThemeButton {
                theme: dialog.theme
                objectName: "releaseDownloadsButton"
                text: "Release notes"
                onClicked: dialog.openReleaseRequested(dialog.updates.release_url)
            }
            ThemeButton {
                theme: dialog.theme
                objectName: "windowsUninstallButton"
                text: "Uninstall…"
                visible: dialog.platformOs === "windows"
                onClicked: dialog.openUninstallRequested()
            }
        }
        RowLayout {
            ThemeButton {
                theme: dialog.theme
                objectName: "installUpdateButton"
                text: "Install update and restart"
                visible: dialog.updates.install_available
                enabled: !dialog.installBusy && !dialog.captureBusy
                onClicked: dialog.updates.install_update()
            }
            ThemeButton {
                theme: dialog.theme
                objectName: "cancelUpdateButton"
                text: "Cancel download"
                visible: dialog.updates.install_state === "downloading"
                onClicked: dialog.updates.cancel_install()
            }
            ThemeButton {
                theme: dialog.theme
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
    }
}
