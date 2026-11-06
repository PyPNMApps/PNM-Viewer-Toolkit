/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ColumnLayout {
    id: workspace
    required property Theme theme
    property var catalog: ({})
    property var viewport: ({})
    property bool working: false
    property string selectedMac: ""
    property var devices: []
    readonly property var groups: (catalog.groups || []).filter(group => group.mac_address === selectedMac)
    readonly property bool hasResult: !!viewport.group
    signal chooseDirectory()
    signal chooseArchive()
    signal cancelImport()
    signal groupRequested(int groupId)
    signal rangeRequested(int start, int count)
    signal bitLoadingRequested(int start, int count)
    signal exportRequested()
    readonly property bool popupVisible: deviceDialog.visible || detailsDialog.visible
    onCatalogChanged: {
        devices = Array.from(new Set((catalog.groups || []).map(group => group.mac_address)))
        selectedMac = ""
        if (devices.length > 0) {
            deviceChoice.currentIndex = 0
            deviceDialog.open()
        }
    }
    spacing: 10
    Label { text: "RxMER · Min-Avg-Max"; color: workspace.theme.text; font.pixelSize: 20; font.bold: true }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        ThemeButton { objectName: "chooseRxmerDirectory"; theme: workspace.theme; text: "Choose folder…"; enabled: !workspace.working; onClicked: workspace.chooseDirectory() }
        ThemeButton { objectName: "chooseRxmerArchive"; theme: workspace.theme; text: "Open archive…"; enabled: !workspace.working; onClicked: workspace.chooseArchive() }
        ThemeButton { objectName: "chooseRxmerDevice"; theme: workspace.theme; text: "Choose MAC…"; enabled: !workspace.working && workspace.devices.length>0; onClicked: deviceDialog.open() }
        ThemeButton { theme: workspace.theme; text: "Import details…"; visible: !!workspace.catalog.groups; onClicked: detailsDialog.open() }
        ThemeButton { theme: workspace.theme; text: "Export JSON"; enabled: workspace.hasResult && !workspace.working; onClicked: workspace.exportRequested() }
        ThemeButton { theme: workspace.theme; text: "Cancel"; visible: workspace.working; onClicked: workspace.cancelImport() }
        BusyIndicator { running: workspace.working; visible: running; implicitWidth: 28; implicitHeight: 28 }
    }
    RowLayout {
        Layout.fillWidth: true
        visible: workspace.selectedMac.length>0
        Label { text: workspace.selectedMac; color: workspace.theme.accent; font.bold: true }
        ComboBox {
            id: channelChoice
            objectName: "rxmerCollectionChannel"
            Accessible.name: "RxMER channel and frequency grid"
            Layout.fillWidth: true
            enabled: !workspace.working
            model: workspace.groups
            textRole: "label"
            onActivated: if (currentIndex>=0) workspace.groupRequested(workspace.groups[currentIndex].id)
        }
    }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        visible: workspace.hasResult
        SpectrumPowerChip { theme: workspace.theme; title: "Captures"; value: workspace.viewport.capture_count || 0 }
        SpectrumPowerChip { theme: workspace.theme; title: "First UTC"; value: workspace.viewport.first_epoch===undefined ? "—" : new Date(workspace.viewport.first_epoch*1000).toISOString().replace("T"," ").replace(".000Z","") }
        SpectrumPowerChip { theme: workspace.theme; title: "Last UTC"; value: workspace.viewport.last_epoch===undefined ? "—" : new Date(workspace.viewport.last_epoch*1000).toISOString().replace("T"," ").replace(".000Z","") }
    }
    RowLayout {
        visible: workspace.hasResult
        ThemeButton { id: minimum; theme: workspace.theme; text: "Min"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerMinimum=checked }
        Rectangle { implicitWidth: 16; implicitHeight: 3; color: workspace.theme.dark ? "#69aaff" : "#2563a6" }
        ThemeButton { id: average; theme: workspace.theme; text: "Avg"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerAverage=checked }
        Rectangle { implicitWidth: 16; implicitHeight: 3; color: workspace.theme.accent }
        ThemeButton { id: maximum; theme: workspace.theme; text: "Max"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerMaximum=checked }
        Rectangle { implicitWidth: 16; implicitHeight: 3; color: workspace.theme.dark ? "#f6bc63" : "#9a5700" }
    }
    MeasurementChart {
        id: plot
        objectName: "rxmerCollectionChart"
        visible: workspace.hasResult
        Layout.fillWidth: true
        Layout.fillHeight: true
        theme: workspace.theme
        viewport: workspace.hasResult ? workspace.viewport : ({points:[],total:0,start:0,count:0})
        rxmerEnabled: true
        onRangeRequested: (start,count)=>workspace.rangeRequested(start,count)
        onBitLoadingRequested: workspace.bitLoadingRequested(visibleStart,visibleCount)
    }
    Item { visible: !workspace.hasResult; Layout.fillHeight: true }
    Dialog {
        id: deviceDialog
        objectName: "rxmerDeviceDialog"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 390
        modal: true
        title: "Select RxMER MAC address"
        background: Rectangle { color: workspace.theme.panel; border.color: workspace.theme.line; radius: 10 }
        header: Label { text: deviceDialog.title; padding: 16; color: workspace.theme.text; font.bold: true }
        contentItem: ComboBox { id: deviceChoice; objectName: "rxmerDeviceChoice"; Accessible.name: "RxMER MAC address"; model: workspace.devices }
        footer: DialogButtonBox {
            ThemeButton { theme: workspace.theme; text: "Select"; enabled: deviceChoice.currentIndex>=0; DialogButtonBox.buttonRole: DialogButtonBox.AcceptRole }
            ThemeButton { theme: workspace.theme; text: "Cancel"; DialogButtonBox.buttonRole: DialogButtonBox.RejectRole }
            onAccepted: deviceDialog.accept()
            onRejected: deviceDialog.reject()
        }
        onAccepted: {
            workspace.selectedMac=workspace.devices[deviceChoice.currentIndex]
            channelChoice.currentIndex=0
            if (workspace.groups.length>0) workspace.groupRequested(workspace.groups[0].id)
        }
    }
    Dialog {
        id: detailsDialog
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: Math.min(640,parent.width-32)
        height: Math.min(480,parent.height-32)
        modal: true
        title: "RxMER import details"
        background: Rectangle { color: workspace.theme.panel; border.color: workspace.theme.line; radius: 10 }
        contentItem: ScrollView {
            clip: true
            TextArea {
                readOnly: true
                color: workspace.theme.text
                wrapMode: TextEdit.Wrap
                text: "Accepted: " + (workspace.catalog.accepted_count || 0) + "\nDuplicates: " + (workspace.catalog.duplicate_count || 0)
                    + "\nSkipped: " + (workspace.catalog.skipped_count || 0) + "\n\n" + (workspace.catalog.issues || []).join("\n")
            }
        }
        footer: DialogButtonBox {
            ThemeButton { theme: workspace.theme; text: "Close"; DialogButtonBox.buttonRole: DialogButtonBox.RejectRole }
            onRejected: detailsDialog.close()
        }
    }
}
