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
    objectName: "aboutDialog"
    title: "About PNM Viewer Toolkit"
    width: 500
    modal: true
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
                enabled: dialog.updates.update_state !== "checking"
                onClicked: dialog.updates.check_updates()
            }
            Button {
                objectName: "releaseDownloadsButton"
                text: dialog.updates.update_state === "available" ? "Release notes and downloads" : "View releases"
                onClicked: dialog.openReleaseRequested(dialog.updates.release_url)
            }
        }
        Label {
            text: "Updates are installed manually from the release downloads."
            color: dialog.theme.muted
            wrapMode: Text.WordWrap
            Layout.fillWidth: true
        }
    }
}
