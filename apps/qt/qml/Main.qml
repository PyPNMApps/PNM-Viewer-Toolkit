/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import QtCore
import Pnm.Viewer

ApplicationWindow {
    id: window
    width: 1280
    height: 820
    minimumWidth: 860
    minimumHeight: 680
    visible: true
    title: "PNM Viewer Toolkit — Qt prototype"
    color: appTheme.background
    font.family: "Sans Serif"
    font.pixelSize: 13
    palette.window: appTheme.background
    palette.windowText: appTheme.text
    palette.base: appTheme.panel
    palette.alternateBase: appTheme.button
    palette.text: appTheme.text
    palette.button: appTheme.button
    palette.buttonText: appTheme.text
    palette.highlight: appTheme.accent
    palette.highlightedText: appTheme.background
    Theme { id: appTheme }
    ViewerBackend { id: backend }
    Component.onCompleted: backend.set_dark_icon(appTheme.dark)
    Connections {
        target: appTheme
        function onDarkChanged() { backend.set_dark_icon(appTheme.dark) }
    }
    readonly property var metadata: JSON.parse(backend.metadata_json)
    readonly property var viewport: JSON.parse(backend.viewport_json)
    property bool navigationVisible: false
    readonly property bool blocked: backend.busy || openDialog.visible || saveDialog.visible || systemDialog.visible || aboutDialog.visible

    FileDialog {
        id: openDialog
        title: "Open PNM capture"
        fileMode: FileDialog.OpenFile
        nameFilters: ["PNM captures (*)"]
        currentFolder: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]
        onAccepted: backend.open_file(selectedFile)
    }
    FileDialog {
        id: saveDialog
        title: "Export full-resolution JSON (new file)"
        fileMode: FileDialog.SaveFile
        nameFilters: ["JSON files (*.json)"]
        defaultSuffix: "json"
        onAccepted: backend.export_json(selectedFile)
    }
    Shortcut { sequences: [StandardKey.Open]; enabled: !window.blocked; onActivated: openDialog.open() }
    Shortcut { sequences: [StandardKey.Close]; enabled: !window.blocked; onActivated: backend.close_capture() }


    header: ToolBar {
        height: 38
        background: Rectangle { color: appTheme.sidebar }
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 8
            anchors.rightMargin: 8
            spacing: 8
            ToolButton { text: "Open"; Accessible.name: "Open PNM file"; enabled: !window.blocked; onClicked: openDialog.open() }
            ToolButton {
                text: "Settings"
                enabled: !window.blocked
                onClicked: settingsMenu.open()
                Menu { id: settingsMenu; y: parent.height; MenuItem { text: "System…"; onTriggered: systemDialog.open() } }
            }
            ToolButton { text: "About"; enabled: !window.blocked; onClicked: aboutDialog.open() }
            Item { Layout.fillWidth: true }
            BusyIndicator { running: backend.busy; visible: running; Layout.preferredWidth: 24; Layout.preferredHeight: 24; Accessible.name: "Processing capture" }
            PanelToggle {
                theme: appTheme
                checked: window.navigationVisible
                Accessible.name: checked ? "Hide navigation" : "Show navigation"
                onClicked: window.navigationVisible = !window.navigationVisible
            }
        }
    }
    Dialog {
        id: systemDialog
        title: "System settings"
        anchors.centerIn: parent
        width: 360
        modal: true
        standardButtons: Dialog.Close
        ColumnLayout {
            anchors.fill: parent
            spacing: 16
            Label { text: "Appearance"; color: appTheme.text }
            ComboBox {
                objectName: "themeSelector"
                Accessible.name: "Appearance"
                model: ["System", "Dark", "Light"]
                currentIndex: appTheme.preference
                Layout.fillWidth: true
                onActivated: appTheme.preference = currentIndex
            }
            Label { text: "System follows your desktop appearance.\nPreferences apply to this session."; color: appTheme.muted }
        }
    }
    Dialog {
        id: aboutDialog
        title: "About PNM Viewer Toolkit"
        anchors.centerIn: parent
        width: 420
        modal: true
        standardButtons: Dialog.Close
        ColumnLayout {
            anchors.fill: parent
            spacing: 12
            Image { source: appTheme.dark ? "qrc:/branding/icon-dark.png" : "qrc:/branding/icon-light.png"; sourceSize.width: 64; sourceSize.height: 64; Layout.alignment: Qt.AlignHCenter }
            Label { text: "PNM Viewer Toolkit"; color: appTheme.text; font.pixelSize: 20; font.bold: true }
            Label { text: "Version " + backend.app_version + " · In development"; color: appTheme.muted }
            Label { text: "Offline inspection of DOCSIS PNM captures."; color: appTheme.text }
            Label { text: "Desktop: GPL-3.0-only\nCore libraries: Apache-2.0\nAcknowledgment: PyPNM Toolkit"; color: appTheme.muted }
            Button { text: "GitHub project"; onClicked: Qt.openUrlExternally("https://github.com/PyPNMApps/PNM-Viewer-Toolkit") }
        }
    }
    RowLayout {
        anchors.fill: parent
        spacing: 0
        Rectangle {
            visible: window.navigationVisible
            Layout.preferredWidth: 180
            Layout.fillHeight: true
            color: appTheme.sidebar
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 16
                spacing: 16
                Image {
                    id: brandLogo
                    Layout.topMargin: 8
                    Layout.fillWidth: true
                    Layout.preferredHeight: 100
                    Layout.preferredWidth: 148
                    source: appTheme.dark ? "qrc:/branding/logo-dark.png" : "qrc:/branding/logo-light.png"
                    fillMode: Image.PreserveAspectFit
                    mipmap: true
                    Accessible.role: Accessible.Graphic
                    Accessible.name: "PyPNMApps PNM Viewer Toolkit"
                    // Blend only the artwork's outer padding into the live theme.
                    // Keep the original README image and logo lettering intact.
                    readonly property color clearSidebar: Qt.rgba(appTheme.sidebar.r, appTheme.sidebar.g, appTheme.sidebar.b, 0)
                    Rectangle {
                        anchors { left: parent.left; right: parent.right; top: parent.top }
                        height: parent.height * 0.20
                        gradient: Gradient {
                            GradientStop { position: 0; color: appTheme.sidebar }
                            GradientStop { position: 1; color: brandLogo.clearSidebar }
                        }
                    }
                    Rectangle {
                        anchors { left: parent.left; right: parent.right; bottom: parent.bottom }
                        height: parent.height * 0.20
                        gradient: Gradient {
                            GradientStop { position: 0; color: brandLogo.clearSidebar }
                            GradientStop { position: 1; color: appTheme.sidebar }
                        }
                    }
                    Rectangle {
                        anchors { left: parent.left; top: parent.top; bottom: parent.bottom }
                        width: parent.width * 0.05
                        gradient: Gradient {
                            orientation: Gradient.Horizontal
                            GradientStop { position: 0; color: appTheme.sidebar }
                            GradientStop { position: 1; color: brandLogo.clearSidebar }
                        }
                    }
                    Rectangle {
                        anchors { right: parent.right; top: parent.top; bottom: parent.bottom }
                        width: parent.width * 0.05
                        gradient: Gradient {
                            orientation: Gradient.Horizontal
                            GradientStop { position: 0; color: brandLogo.clearSidebar }
                            GradientStop { position: 1; color: appTheme.sidebar }
                        }
                    }
                }

                Label { text: "WORKSPACE"; color: appTheme.muted; font.pixelSize: 10 }
                Label { text: backend.loaded ? "Downstream RxMER" : "Home"; color: appTheme.accent }
                Item { Layout.fillHeight: true }
            }
        }
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 4
            spacing: 10
            Label {
                visible: backend.error_message.length > 0
                text: backend.error_message
                color: appTheme.dark ? "#ffb4ab" : "#a92b26"
                wrapMode: Text.Wrap
                Layout.fillWidth: true
                Accessible.role: Accessible.AlertMessage
            }
                Rectangle {
                    visible: !backend.loaded
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: appTheme.panel
                    border.color: appTheme.line
                    radius: 14
                    ColumnLayout {
                        anchors.centerIn: parent
                        spacing: 20
                        Label { text: "∿"; color: appTheme.accent; font.pixelSize: 64; Layout.alignment: Qt.AlignHCenter }
                        Label { text: "YOUR CAPTURE, ON YOUR DESKTOP"; color: appTheme.accent; font.pixelSize: 10; font.letterSpacing: 1.6; Layout.alignment: Qt.AlignHCenter }
                        Label { text: "Explore a PNM capture"; color: appTheme.text; font.pixelSize: 26; font.bold: true; Layout.alignment: Qt.AlignHCenter }
                        Label { text: "Choose a local capture to view measurement details,\ninspect its graph, and export decoded JSON."; color: appTheme.muted; horizontalAlignment: Text.AlignHCenter; Layout.alignment: Qt.AlignHCenter }
                        Button { text: "Open capture"; enabled: !window.blocked; highlighted: true; Layout.alignment: Qt.AlignHCenter; onClicked: openDialog.open() }
                        Label { text: "Currently supported: Downstream RxMER (format 1.0)"; color: appTheme.muted; font.pixelSize: 11; Layout.alignment: Qt.AlignHCenter }
                    }
                }
                Rectangle {
                    visible: backend.loaded
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.minimumHeight: 300
                    color: appTheme.panel
                    border.color: appTheme.line
                    radius: 12
                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 8
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 6
                                Label { text: "RxMER · " + (window.metadata.filename || ""); elide: Text.ElideMiddle; Layout.fillWidth: true; color: appTheme.text; font.pixelSize: 17; font.bold: true }
                                Label { text: "Channel " + window.metadata.channel_id + " · " + window.metadata.mac_address + " · " + window.metadata.spacing_khz + " kHz"; color: appTheme.muted; font.pixelSize: 11 }
                            }
                            Button { text: "Export JSON"; enabled: !window.blocked; onClicked: saveDialog.open() }
                            Button { text: "Close capture"; enabled: !window.blocked; onClicked: backend.close_capture() }
                        }
                        RxMerChart { theme: appTheme; viewport: window.viewport; Layout.fillWidth: true; Layout.fillHeight: true }
                        Rectangle { Layout.fillWidth: true; height: 1; color: appTheme.line }
                        RowLayout {
                            Layout.fillWidth: true
                            Label {
                                Layout.fillWidth: true
                                text: window.viewport.total === 0 ? "No samples" : "Samples " + window.viewport.start + "–" + (window.viewport.start + window.viewport.points.length - 1) + " of " + window.viewport.total
                                color: appTheme.muted
                                font.pixelSize: 11
                            }
                            Button { text: "← Previous"; enabled: !window.blocked && window.viewport.start > 0; onClicked: backend.show_page(Math.max(0, window.viewport.start - 1024)) }
                            Button { text: "Next →"; enabled: !window.blocked && window.viewport.start + window.viewport.points.length < window.viewport.total; onClicked: backend.show_page(window.viewport.start + window.viewport.points.length) }
                        }
                    }
                }

        }
    }
}
