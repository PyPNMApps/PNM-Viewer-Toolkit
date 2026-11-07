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
    required property var service
    property bool playbackMode: false
    property bool playbackBlocked: false
    property int selectedGroup: -1
    property int pendingFrame:-1
    property string selectedMac: ""
    readonly property var catalog: JSON.parse(service.catalog_json)
    readonly property var viewport: JSON.parse(service.view_json)
    readonly property bool hasResult: !!viewport.group
    readonly property var devices: Array.from(new Set((catalog.groups || []).map(group=>group.mac_address)))
    readonly property var groups: (catalog.groups || []).filter(group=>group.mac_address===selectedMac)
    readonly property bool popupVisible: devicesDialog.visible || settingsDialog.visible || detailsDialog.visible || tools.popupVisible
    signal chooseDirectory()
    signal chooseArchive()
    signal exportRequested()
    function pausePlayback() { playback.playing=false }
    function selectGroup(macAddress,groupId) {
        if (service.busy) return
        pausePlayback()
        selectedMac=macAddress
        selectedGroup=groupId
        service.select_group(groupId,playbackMode)
    }
    function selectFrame(mac,group,index) {
        pausePlayback()
        if (selectedGroup===group) showFrame(index)
        else {selectGroup(mac,group);pendingFrame=index}
    }
    Connections {target:workspace.service;function onView_jsonChanged(){
        if (workspace.pendingFrame>=0 && workspace.viewport.group_id===workspace.selectedGroup && !workspace.service.busy) {
            const frame=workspace.pendingFrame;workspace.pendingFrame=-1
            Qt.callLater(function(){workspace.showFrame(frame)})
        }
    }}
    function showFrame(index) { if (!service.busy && selectedGroup>=0) service.show_frame(index) }
    onPlaybackModeChanged: { pausePlayback(); if (selectedGroup>=0) service.select_group(selectedGroup,playbackMode) }
    onCatalogChanged: {
        pausePlayback();selectedGroup=-1;selectedMac=""
        Qt.callLater(function(){if(workspace.visible && (workspace.devices || []).length>0){deviceChoice.currentIndex=0;devicesDialog.open()}})
    }
    spacing: 8
    Label { text: "Spectrum Analyzer · "+(workspace.playbackMode ? "Playback" : "Min-Avg-Max");color:workspace.theme.text;font.pixelSize:20;font.bold:true }
    Flow {
        Layout.fillWidth:true;spacing:8
        ThemeButton {objectName:"chooseSpectrumDirectory";theme:workspace.theme;text:"Choose folder…";enabled:!workspace.service.busy;onClicked:workspace.chooseDirectory()}
        ThemeButton {objectName:"chooseSpectrumArchive";theme:workspace.theme;text:"Open archive…";enabled:!workspace.service.busy;onClicked:workspace.chooseArchive()}
        ThemeButton {theme:workspace.theme;text:"Choose MAC…";enabled:workspace.devices.length>0 && !workspace.service.busy;onClicked:devicesDialog.open()}
        ThemeButton {theme:workspace.theme;text:"Capture settings…";enabled:workspace.groups.length>0 && !workspace.service.busy;onClicked:settingsDialog.open()}
        ThemeButton {theme:workspace.theme;text:"Import details…";visible:!!workspace.catalog.groups;onClicked:detailsDialog.open()}
        ThemeButton {theme:workspace.theme;text:"Export JSON";enabled:workspace.hasResult && !workspace.service.busy;onClicked:workspace.exportRequested()}
        ThemeButton {theme:workspace.theme;text:"Cancel";visible:workspace.service.busy;onClicked:workspace.service.cancel()}
        BusyIndicator {running:workspace.service.busy;visible:running;implicitWidth:28;implicitHeight:28}
    }
    Flow {
        visible:workspace.hasResult;Layout.fillWidth:true;spacing:8
        MetadataChip {theme:workspace.theme;title:"MAC";value:workspace.hasResult ? workspace.viewport.group.mac_address : "—"}
        MetadataChip {theme:workspace.theme;title:"RBW";value:workspace.hasResult ? (workspace.viewport.resolution_bandwidth_hz/1000).toFixed(3)+" kHz" : "—"}
        MetadataChip {theme:workspace.theme;title:"Channel";value:workspace.hasResult ? workspace.viewport.group.channel_id : "—"}
        MetadataChip {theme:workspace.theme;title:"Window";value:workspace.hasResult ? workspace.viewport.group.window_function : "—"}
        SpectrumPowerChip {theme:workspace.theme;title:workspace.playbackMode ? "Capture UTC" : "Captures";value:workspace.playbackMode ? (workspace.viewport.capture_epoch===undefined ? "—" : new Date(workspace.viewport.capture_epoch*1000).toISOString().replace("T"," ").replace(".000Z","")) : workspace.viewport.capture_count || 0}
    }
    RxMerPlaybackControls {
        id:playback
        namePrefix:"spectrum"
        objectName:"spectrumPlaybackControls"
        theme:workspace.theme;visible:workspace.hasResult && workspace.playbackMode
        frameIndex:workspace.viewport.frame_index || 0;frameCount:workspace.viewport.frame_count || 0
        suspended:!workspace.visible || !workspace.playbackMode || workspace.service.busy || workspace.playbackBlocked || workspace.popupVisible
        onFrameRequested: index=>workspace.showFrame(index)
    }
    SpectrumTools {
        id:tools
        visible:workspace.hasResult;Layout.fillWidth:true;theme:workspace.theme
        workspace:workspace.viewport.workspace || ({tabs:[]})
        validateRange:(start,stop,unit)=>workspace.service.range_error(start,stop,unit)
        onRangeRequested:(start,stop,unit)=>workspace.service.open_band(start,stop,unit)
        onTabRequested:index=>workspace.service.select_tab(index,false)
        onCloseRequested:index=>workspace.service.select_tab(index,true)
    }
    Flow {
        Layout.fillWidth:true;spacing:8
        Repeater {model:tools.powerChips;SpectrumPowerChip {required property var modelData;theme:workspace.theme;title:modelData.title;value:modelData.value}}
    }
    RowLayout {
        visible:workspace.hasResult && !workspace.playbackMode
        ThemeButton {theme:workspace.theme;text:"Min";checkable:true;checked:true;outlineColor:workspace.theme.rxmerMinimum;borderHighlighted:checked;onClicked:plot.showRxmerMinimum=checked}
        ThemeButton {theme:workspace.theme;text:"Avg";checkable:true;checked:true;outlineColor:workspace.theme.rxmerAverage;borderHighlighted:checked;onClicked:plot.showRxmerAverage=checked}
        ThemeButton {theme:workspace.theme;text:"Max";checkable:true;checked:true;outlineColor:workspace.theme.rxmerMaximum;borderHighlighted:checked;onClicked:plot.showRxmerMaximum=checked}
    }
    SpectrumControls {
        id:smoothing
        theme:workspace.theme;visible:workspace.hasResult;Layout.fillWidth:true
        sampleCount:workspace.viewport.total || 0
        averageWindow:workspace.viewport.spectrum_average ? workspace.viewport.spectrum_average.window_points : 0
        onAverageRequested:points=>workspace.service.average(points)
    }
    MeasurementChart {
        id:plot
        objectName:"spectrumCollectionChart"
        theme:workspace.theme;visible:workspace.hasResult;Layout.fillWidth:true;Layout.fillHeight:true
        viewport:workspace.hasResult ? workspace.viewport : ({points:[],total:0,start:0,count:0})
        spectrumEnabled:true;axisLabel:"Level";unit:"dBmV"
        minimum:workspace.viewport.minimum===undefined ? -100 : workspace.viewport.minimum
        maximum:workspace.viewport.maximum===undefined ? 0 : workspace.viewport.maximum
        showSpectrumAverage:smoothing.showAverage
        onRangeRequested:(start,count)=>workspace.service.show_range(start,count)
        onFrequencyRequested:tools.openFrequency()
        onPowerRequested:(first,last)=>workspace.service.calculate_power(first,last)
    }
    Item {visible:!workspace.hasResult;Layout.fillHeight:true}
    Dialog {
        id:devicesDialog;objectName:"spectrumDeviceDialog";parent:Overlay.overlay;anchors.centerIn:parent;width:390;modal:true;title:"Select spectrum MAC address"
        background:Rectangle {color:workspace.theme.panel;border.color:workspace.theme.line;radius:10}
        header:Label {text:devicesDialog.title;padding:16;color:workspace.theme.text;font.bold:true}
        contentItem:ComboBox {id:deviceChoice;objectName:"spectrumDeviceChoice";Accessible.name:"Spectrum MAC address";model:workspace.devices}
        footer:DialogButtonBox {
            ThemeButton {theme:workspace.theme;text:"Select";enabled:deviceChoice.currentIndex>=0;DialogButtonBox.buttonRole:DialogButtonBox.AcceptRole}
            ThemeButton {theme:workspace.theme;text:"Cancel";DialogButtonBox.buttonRole:DialogButtonBox.RejectRole}
            onAccepted:devicesDialog.accept();onRejected:devicesDialog.reject()
        }
        onAccepted: {
            workspace.selectedMac=workspace.devices[deviceChoice.currentIndex]
            if (workspace.groups.length>0) workspace.selectGroup(workspace.selectedMac,workspace.groups[0].id)
        }
    }
    Dialog {
        id:settingsDialog;parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(900,parent.width-24);modal:true;title:"Select capture settings"
        background:Rectangle {color:workspace.theme.panel;border.color:workspace.theme.line;radius:10}
        header:Label {text:settingsDialog.title;padding:16;color:workspace.theme.text;font.bold:true}
        contentItem:ComboBox {objectName:"spectrumCaptureSettings";Accessible.name:"Spectrum capture settings";model:workspace.groups;textRole:"label";onActivated:if(currentIndex>=0){workspace.selectGroup(workspace.selectedMac,workspace.groups[currentIndex].id);settingsDialog.close()}}
        footer:DialogButtonBox {ThemeButton {theme:workspace.theme;text:"Close";DialogButtonBox.buttonRole:DialogButtonBox.RejectRole}onRejected:settingsDialog.close()}
    }
    Dialog {
        id:detailsDialog;parent:Overlay.overlay;anchors.centerIn:parent;width:Math.min(700,parent.width-24);height:Math.min(500,parent.height-24);modal:true;title:"Spectrum import details"
        background:Rectangle {color:workspace.theme.panel;border.color:workspace.theme.line;radius:10}
        contentItem:ScrollView {clip:true;TextArea {readOnly:true;color:workspace.theme.text;wrapMode:TextEdit.Wrap;text:"Accepted: "+(workspace.catalog.accepted_count || 0)+"\nDuplicates: "+(workspace.catalog.duplicate_count || 0)+"\nSkipped: "+(workspace.catalog.skipped_count || 0)+"\n\n"+(workspace.catalog.issues || []).join("\n")}}
        footer:DialogButtonBox {ThemeButton {theme:workspace.theme;text:"Close";DialogButtonBox.buttonRole:DialogButtonBox.RejectRole}onRejected:detailsDialog.close()}
    }
}
