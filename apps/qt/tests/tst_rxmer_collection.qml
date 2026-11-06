/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml"
TestCase {
    id: testCase
    name: "RxMerCollection"
    width: 1280; height: 820
    visible: true
    when: windowShown
    property int requestedGroup: -1
    property int requestedStart: -1
    property int requestedCount: -1
    property bool archiveRequested: false
    property bool folderRequested: false
    Theme { id: appearance }
    RxMerCollectionView {
        id: workspace
        width: 1200; height: 790
        theme: appearance
        onChooseArchive: testCase.archiveRequested=true
        onChooseDirectory: testCase.folderRequested=true
        onGroupRequested: groupId=>testCase.requestedGroup=groupId
        onBitLoadingRequested: (start,count)=>{testCase.requestedStart=start;testCase.requestedCount=count}
    }
    function init() {
        findChild(workspace,"rxmerDeviceDialog").close()
        workspace.catalog=({});workspace.viewport=({})
        workspace.working=false
        requestedGroup=-1;requestedStart=-1;requestedCount=-1;archiveRequested=false;folderRequested=false
    }
    function test_device_popup_channel_groups_and_native_picker_signals_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_device_popup_channel_groups_and_native_picker_signals(data) {
        appearance.preference=data.theme
        mouseClick(findChild(workspace,"chooseRxmerArchive"));verify(archiveRequested)
        mouseClick(findChild(workspace,"chooseRxmerDirectory"));verify(folderRequested)
        workspace.catalog=({groups:[{id:0,mac_address:"aa:bb:cc:dd:ee:ff",label:"Channel 34"},{id:1,mac_address:"aa:bb:cc:dd:ee:ff",label:"Channel 35"},{id:2,mac_address:"00:11:22:33:44:55",label:"Channel 34"}]})
        const dialog=findChild(workspace,"rxmerDeviceDialog")
        tryCompare(dialog,"visible",true)
        const choice=findChild(workspace,"rxmerDeviceChoice")
        compare(choice.count,2)
        choice.currentIndex=1;dialog.accept()
        compare(workspace.selectedMac,"00:11:22:33:44:55")
        compare(requestedGroup,2);compare(workspace.groups.length,1)
        workspace.working=true
        verify(!findChild(workspace,"chooseRxmerArchive").enabled)
    }
    function test_three_trace_plot_zoom_and_average_context_analysis() {
        const points=[{sample_index:2,frequency_hz:640050000,value:40},{sample_index:3,frequency_hz:640075000,value:41},{sample_index:4,frequency_hz:640100000,value:null},{sample_index:5,frequency_hz:640125000,value:39}]
        workspace.viewport=({group:{channel_id:34},start:2,count:4,total:10,capture_count:3,first_epoch:10,last_epoch:20,points:points,rxmer_statistics:{minimum:points.map(point=>({sample_index:point.sample_index,frequency_hz:point.frequency_hz,value:point.value===null ? null : point.value-2})),maximum:points.map(point=>({sample_index:point.sample_index,frequency_hz:point.frequency_hz,value:point.value===null ? null : point.value+2}))}})
        const chart=findChild(workspace,"rxmerCollectionChart")
        const plot=findChild(chart,"measurementPlot")
        plot.requestPaint();verify(waitForRendering(plot))
        verify(grabImage(plot).width>500)
        verify(chart.rxmerEnabled)
        const action=findChild(chart,"analyzeRxmerBitLoading")
        mouseClick(plot,250,100,Qt.RightButton)
        tryCompare(findChild(chart,"spectrumContextMenu"),"opened",true)
        verify(waitForRendering(action))
        mouseClick(action)
        compare(requestedStart,2);compare(requestedCount,4)
        chart.showRxmerMinimum=false;plot.requestPaint();verify(waitForRendering(plot))
        chart.showRxmerMaximum=false;plot.requestPaint();verify(waitForRendering(plot))
    }
}
