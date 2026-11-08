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
    property bool playbackMode: false
    property bool playbackBlocked: false
    property int selectedGroup: -1
    signal frameRequested(int groupId, int frameIndex, int start, int count)
    function pausePlayback() { playback.playing=false }
    function showFrame(frameIndex) {
        if (selectedGroup<0 || working) {
            return
        }
        const group=(catalog.groups || []).find(candidate => candidate.id===selectedGroup)
        if (!group) {
            return
        }
        frameRequested(selectedGroup,frameIndex,hasResult ? viewport.start || 0 : 0,hasResult ? viewport.count || group.sample_count : group.sample_count)
    }
    function requestGroup(groupId) {
        playback.playing=false
        selectedGroup=groupId
        if (playbackMode) {
            const group=(catalog.groups || []).find(candidate => candidate.id===groupId)
            if (group) {
                frameRequested(groupId,0,0,group.sample_count)
            }
        } else {
            groupRequested(groupId)
        }
    }
    onPlaybackModeChanged: {
        playback.playing=false
        if (selectedGroup>=0) {
            requestGroup(selectedGroup)
        }
    }
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
    readonly property bool popupVisible: deviceDialog.visible || detailsDialog.visible || channelDialog.visible
    onCatalogChanged: {
        playback.playing=false
        selectedGroup=-1
        devices = Array.from(new Set((catalog.groups || []).map(group => group.mac_address)))
        selectedMac = ""
        if (devices.length > 0) {
            deviceChoice.currentIndex = 0
            deviceDialog.open()
        }
    }
    // Select a tree channel through the same path as the MAC/channel dialogs.
    // Unknown IDs and busy requests retain the currently displayed result.
    function selectGroup(macAddress, groupId) {
        if (working) return
        const group=(catalog.groups || []).find(group => group.id===groupId && group.mac_address===macAddress)
        if (!group) return
        selectedMac=macAddress
        channelChoice.currentIndex=groups.findIndex(candidate => candidate.id===groupId)
        requestGroup(groupId)
    }
    spacing: 10
    Label { text: workspace.playbackMode ? "RxMER · Playback" : "RxMER · Min-Avg-Max"; color: workspace.theme.text; font.pixelSize: 20; font.bold: true }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        ThemeButton { objectName: "chooseRxmerDirectory"; theme: workspace.theme; text: "Choose folder…"; enabled: !workspace.working; onClicked: workspace.chooseDirectory() }
        ThemeButton { objectName: "chooseRxmerArchive"; theme: workspace.theme; text: "Open archive…"; enabled: !workspace.working; onClicked: workspace.chooseArchive() }
        ThemeButton { objectName: "chooseRxmerDevice"; theme: workspace.theme; text: "Choose MAC…"; enabled: !workspace.working && workspace.devices.length>0; onClicked: deviceDialog.open() }
        ThemeButton { theme: workspace.theme; text: "Change channel…"; enabled: !workspace.working && workspace.groups.length>1; onClicked: channelDialog.open() }
        ThemeButton { theme: workspace.theme; text: "Import details…"; visible: !!workspace.catalog.groups; onClicked: detailsDialog.open() }
        ThemeButton { theme: workspace.theme; text: "Export JSON"; visible: !workspace.playbackMode; enabled: workspace.hasResult && !workspace.working; onClicked: workspace.exportRequested() }
        ThemeButton { theme: workspace.theme; text: "Cancel"; visible: workspace.working; onClicked: workspace.cancelImport() }
        BusyIndicator { running: workspace.working; visible: running; implicitWidth: 28; implicitHeight: 28 }
    }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        visible: workspace.hasResult
        MetadataChip { theme: workspace.theme; title: "MAC"; value: workspace.hasResult ? workspace.viewport.group.mac_address || "—" : "—" }
        MetadataChip { theme: workspace.theme; title: "Channel"; value: workspace.hasResult ? workspace.viewport.group.channel_id : "—" }
        MetadataChip { theme: workspace.theme; title: "Zero frequency"; value: workspace.hasResult ? (workspace.viewport.group.zero_frequency_hz || 0) + " Hz" : "—" }
        MetadataChip { theme: workspace.theme; title: "First carrier"; value: workspace.hasResult ? workspace.viewport.group.first_active_subcarrier_index || 0 : "—" }
        MetadataChip { theme: workspace.theme; title: "Spacing"; value: workspace.hasResult ? (workspace.viewport.group.spacing_hz || 0) + " Hz" : "—" }
        MetadataChip { theme: workspace.theme; title: "Carriers"; value: workspace.hasResult ? workspace.viewport.group.sample_count || 0 : "—" }
    }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        visible: workspace.hasResult && !workspace.playbackMode
        SpectrumPowerChip { theme: workspace.theme; title: "Captures"; value: workspace.viewport.capture_count || 0 }
        SpectrumPowerChip { theme: workspace.theme; title: "First UTC"; value: workspace.viewport.first_epoch===undefined ? "—" : new Date(workspace.viewport.first_epoch*1000).toISOString().replace("T"," ").replace(".000Z","") }
        SpectrumPowerChip { theme: workspace.theme; title: "Last UTC"; value: workspace.viewport.last_epoch===undefined ? "—" : new Date(workspace.viewport.last_epoch*1000).toISOString().replace("T"," ").replace(".000Z","") }
    }
    Flow {
        visible: workspace.hasResult && workspace.playbackMode
        Layout.fillWidth: true
        spacing: 8
        SpectrumPowerChip { objectName: "rxmerPlaybackAverage"; theme: workspace.theme; title: "Avg MER"; value: workspace.viewport.average_mer_db===undefined || workspace.viewport.average_mer_db===null ? "—" : workspace.viewport.average_mer_db.toFixed(1)+" dB" }
        SpectrumPowerChip { theme: workspace.theme; title: "Capture UTC"; value: workspace.viewport.capture_epoch===undefined ? "—" : new Date(workspace.viewport.capture_epoch*1000).toISOString().replace("T"," ").replace(".000Z","") }
    }
    RxMerPlaybackControls {
        id: playback
        objectName: "rxmerPlaybackControls"
        theme: workspace.theme
        visible: workspace.playbackMode && workspace.hasResult
        frameIndex: workspace.viewport.frame_index || 0
        frameCount: workspace.viewport.frame_count || 0
        suspended: !workspace.visible || !workspace.playbackMode || workspace.working || workspace.playbackBlocked || workspace.popupVisible
        onFrameRequested: frameIndex=>workspace.showFrame(frameIndex)
    }
    RowLayout {
        visible: workspace.hasResult && !workspace.playbackMode
        ThemeButton { id: minimum; theme: workspace.theme; outlineColor: workspace.theme.rxmerMinimum; text: "Min"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerMinimum=checked }
        ThemeButton { id: average; theme: workspace.theme; outlineColor: workspace.theme.rxmerAverage; text: "Avg"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerAverage=checked }
        ThemeButton { id: maximum; theme: workspace.theme; outlineColor: workspace.theme.rxmerMaximum; text: "Max"; checkable: true; checked: true; borderHighlighted: checked; onClicked: plot.showRxmerMaximum=checked }
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
        onRangeRequested: (start,count)=> {
            if (workspace.playbackMode) {
                workspace.frameRequested(workspace.selectedGroup,workspace.viewport.frame_index || 0,start,count)
            } else {
                workspace.rangeRequested(start,count)
            }
        }
        onBitLoadingRequested: workspace.bitLoadingRequested(visibleStart,visibleCount)
    }
    Item { visible: !workspace.hasResult; Layout.fillHeight: true }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: deviceDialog; theme: workspace.theme }
        id: deviceDialog
        objectName: "rxmerDeviceDialog"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: 390
        modal: true
        title: "Select RxMER MAC address"
        background: Rectangle { color: workspace.theme.panel; border.color: workspace.theme.line; radius: 10 }
        header: Label { text: deviceDialog.title; padding: 16; color: workspace.theme.text; font.bold: true }
        contentItem: ThemeComboBox { theme: workspace.theme; id: deviceChoice; objectName: "rxmerDeviceChoice"; Accessible.name: "RxMER MAC address"; model: workspace.devices }
        footer: DialogButtonBox {
            ThemeButton { theme: workspace.theme; text: "Select"; enabled: deviceChoice.currentIndex>=0; DialogButtonBox.buttonRole: DialogButtonBox.AcceptRole }
            ThemeButton { theme: workspace.theme; text: "Cancel"; DialogButtonBox.buttonRole: DialogButtonBox.RejectRole }
            onAccepted: deviceDialog.accept()
            onRejected: deviceDialog.reject()
        }
        onAccepted: {
            workspace.selectedMac=workspace.devices[deviceChoice.currentIndex]
            channelChoice.currentIndex=0
            if (workspace.groups.length>0) workspace.requestGroup(workspace.groups[0].id)
        }
    }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: channelDialog; theme: workspace.theme }
        id: channelDialog
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: Math.min(760,parent.width-32)
        modal: true
        title: "Select channel and frequency grid"
        background: Rectangle { color: workspace.theme.panel; border.color: workspace.theme.line; radius: 10 }
        header: Label { text: channelDialog.title; padding: 16; color: workspace.theme.text; font.bold: true }
        contentItem: ThemeComboBox {
            theme: workspace.theme
            id: channelChoice
            objectName: "rxmerCollectionChannel"
            Accessible.name: "RxMER channel and frequency grid"
            enabled: !workspace.working
            model: workspace.groups
            textRole: "label"
            onActivated: if (currentIndex>=0) {
                workspace.requestGroup(workspace.groups[currentIndex].id)
                channelDialog.close()
            }
        }
        footer: DialogButtonBox {
            ThemeButton { theme: workspace.theme; text: "Close"; DialogButtonBox.buttonRole: DialogButtonBox.RejectRole }
            onRejected: channelDialog.close()
        }
    }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: detailsDialog; theme: workspace.theme }
        id: detailsDialog
        objectName: "rxmerImportDetails"
        parent: Overlay.overlay
        anchors.centerIn: parent
        width: Math.min(640,parent.width-32)
        height: Math.min(480,parent.height-32)
        modal: true
        title: "RxMER import details"
        background: Rectangle { color: workspace.theme.panel; border.color: workspace.theme.line; radius: 10 }
        contentItem: ScrollView {
            clip: true
            ThemeTextArea {
                theme: workspace.theme
                objectName: "rxmerImportReport"
                readOnly: true
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
