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
    readonly property bool blocked: backend.busy || openDialog.visible || saveDialog.visible

    FileDialog {
        id: openDialog
        title: "Open RxMER capture"
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

    RowLayout {
        anchors.fill: parent
        spacing: 0
        Rectangle {
            Layout.preferredWidth: window.width < 1050 ? 164 : 208
            Layout.fillHeight: true
            color: appTheme.sidebar
            ColumnLayout {
                anchors.fill: parent
                anchors.margins: 20
                spacing: 16
                Image {
                    id: brandLogo
                    Layout.topMargin: 8
                    Layout.fillWidth: true
                    Layout.preferredHeight: width * 2 / 3
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
                Label { Layout.topMargin: 32; text: "WORKSPACE"; font.pixelSize: 10; font.letterSpacing: 1.5; color: appTheme.muted }
                Rectangle {
                    Layout.fillWidth: true
                    height: 42
                    radius: 8
                    color: appTheme.accentSoft
                    Label { anchors.centerIn: parent; text: "⌁  RxMER viewer"; color: appTheme.accent }
                }
                Item { Layout.fillHeight: true }
            }
        }
        Rectangle { Layout.preferredWidth: 1; Layout.fillHeight: true; color: appTheme.line }
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            spacing: 0
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 64
                color: appTheme.background
                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 28
                    anchors.rightMargin: 28
                    Label { text: "Measurements  /  Downstream RxMER"; color: appTheme.text; font.pixelSize: 12; Layout.fillWidth: true }
                    Label { text: "Appearance"; color: appTheme.muted; font.pixelSize: 12 }
                    ComboBox {
                        objectName: "themeSelector"
                        model: ["System", "Dark", "Light"]
                        Accessible.name: "Appearance"
                        Layout.preferredWidth: 112
                        onActivated: appTheme.preference = currentIndex
                    }
                }
                Rectangle { anchors.bottom: parent.bottom; width: parent.width; height: 1; color: appTheme.line }
            }
            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.margins: 24
                spacing: 14
                RowLayout {
                    Layout.fillWidth: true
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 7
                        Label { text: "SIGNAL EXPLORER"; color: appTheme.accent; font.pixelSize: 10; font.letterSpacing: 2; font.bold: true }
                        Label { text: "Downstream RxMER"; color: appTheme.text; font.pixelSize: 27; font.bold: true }
                        Label { text: "Receive modulation error ratio across OFDM subcarriers."; color: appTheme.muted; font.pixelSize: 12 }
                    }
                    Button {
                        text: "＋ Open PNM file"
                        Accessible.name: "Open PNM file"
                        enabled: !window.blocked
                        highlighted: true
                        onClicked: openDialog.open()
                    }
                }
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
                        Label { text: "A clearer view of your signal."; color: appTheme.text; font.pixelSize: 26; font.bold: true; Layout.alignment: Qt.AlignHCenter }
                        Label { text: "Open a local RxMER PNM file to inspect its channel,\nexplore exact readings, and export decoded JSON."; color: appTheme.muted; horizontalAlignment: Text.AlignHCenter; Layout.alignment: Qt.AlignHCenter }
                        Button { text: "Open RxMER capture"; enabled: !window.blocked; highlighted: true; Layout.alignment: Qt.AlignHCenter; onClicked: openDialog.open() }
                        Label { text: "Local processing  ·  RxMER format 1.0  ·  No server required"; color: appTheme.muted; font.pixelSize: 11; Layout.alignment: Qt.AlignHCenter }
                    }
                }
                Rectangle {
                    visible: backend.loaded
                    Layout.fillWidth: true
                    Layout.preferredHeight: 52
                    color: appTheme.panel
                    border.color: appTheme.line
                    radius: 10
                    RowLayout {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 12
                        Label { text: "▤"; color: appTheme.accent; font.pixelSize: 23 }
                        Label { text: window.metadata.filename || ""; color: appTheme.text; font.bold: true; elide: Text.ElideMiddle; Layout.fillWidth: true }
                        Label { text: "RxMER 1.0"; color: appTheme.accent; font.pixelSize: 10 }
                        Button { text: "Close"; enabled: !window.blocked; onClicked: backend.close_capture() }
                    }
                }
                RowLayout {
                    visible: backend.loaded
                    Layout.fillWidth: true
                    spacing: 12
                    Repeater {
                        model: [
                            {label: "CHANNEL", value: window.metadata.channel_id},
                            {label: "MAC ADDRESS", value: window.metadata.mac_address},
                            {label: "SUBCARRIERS", value: window.metadata.sample_count},
                            {label: "SPACING", value: (window.metadata.spacing_khz || 0) + " kHz"}
                        ]
                        Rectangle {
                            required property var modelData
                            required property int index
                            Layout.fillWidth: true
                            Layout.preferredWidth: index === 1 ? 230 : 150
                            Layout.preferredHeight: 72
                            color: appTheme.panel
                            border.color: appTheme.line
                            radius: 10
                            Column {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 8
                                Label { text: modelData.label; color: appTheme.muted; font.pixelSize: 9; font.letterSpacing: 1.3 }
                                Label { text: modelData.value === undefined ? "" : modelData.value.toString(); color: appTheme.text; font.pixelSize: index === 1 ? 14 : 22; font.bold: true }
                            }
                        }
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
                        anchors.margins: 18
                        spacing: 8
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 6
                                Label { text: "RxMER by frequency"; color: appTheme.text; font.pixelSize: 17; font.bold: true }
                                Label { text: "●  Exact samples · missing readings remain gaps"; color: appTheme.muted; font.pixelSize: 11 }
                            }
                            Button { text: "Export JSON ↗"; enabled: !window.blocked; onClicked: saveDialog.open() }
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
                Label { visible: backend.loaded; text: "Up to 1,024 original samples per window. JSON export includes every sample."; color: appTheme.muted; font.pixelSize: 10 }
            }
            Rectangle {
                Layout.fillWidth: true
                Layout.preferredHeight: 32
                color: appTheme.sidebar
                RowLayout {
                    anchors.fill: parent
                    anchors.leftMargin: 24
                    anchors.rightMargin: 24
                    BusyIndicator { running: backend.busy; visible: running; Layout.preferredWidth: 22; Layout.preferredHeight: 22 }
                    Label { text: backend.status; color: appTheme.muted; font.pixelSize: 10; Layout.fillWidth: true }
                    Label { text: "PNM Viewer Toolkit · Qt prototype"; color: appTheme.muted; font.pixelSize: 10 }
                }
            }
        }
    }
}
