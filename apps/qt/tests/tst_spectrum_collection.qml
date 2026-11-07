/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml"
TestCase {
    id:testCase;name:"SpectrumCollections";width:1280;height:820;visible:true;when:windowShown
    Theme {id:appearance}
    QtObject {
        id:service
        property string catalog_json:"{}"
        property string view_json:"{}"
        property bool busy:false
        property int selected:-1
        property bool playback:false
        property int frame:-1
        property int window:0
        function select_group(group,frames){selected=group;playback=frames}
        function show_frame(index){frame=index}
        function cancel(){busy=false}
        function show_range(start,count){}
        function range_error(start,stop,unit){return ""}
        function average(points){window=points}
        function open_band(start,stop,unit){}
        function select_tab(index,close){}
        function calculate_power(first,last){}
    }
    SpectrumCollectionView {id:workspace;anchors.fill:parent;theme:appearance;service:service}
    function init(){workspace.pausePlayback();workspace.playbackMode=false;service.busy=false;service.catalog_json="{}";service.view_json="{}";service.selected=-1;service.frame=-1;appearance.preference=1}
    function cleanup(){const dialog=findChild(workspace,"spectrumDeviceDialog");dialog.close()}
    function test_mac_settings_selection_and_playback_routing(){
        service.catalog_json=JSON.stringify({groups:[{id:0,mac_address:"00:ca:00:00:00:01",label:"Settings 1",sample_count:3,sources:[{capture_time:100},{capture_time:200}]}]})
        const dialog=findChild(workspace,"spectrumDeviceDialog")
        tryCompare(dialog,"opened",true)
        dialog.accept()
        compare(workspace.selectedMac,"00:ca:00:00:00:01")
        compare(service.selected,0);compare(service.playback,false)
        workspace.playbackMode=true
        compare(service.playback,true)
        workspace.showFrame(1);compare(service.frame,1)
    }
    function test_standalone_spectrum_trace_and_average_control_both_themes(){
        for(const theme of [1,2]){
            appearance.preference=theme
            service.view_json=JSON.stringify({group:{mac_address:"00:ca:00:00:00:01",channel_id:0,window_function:1},resolution_bandwidth_hz:45000,total:3,start:0,count:3,capture_count:2,
                points:[{frequency_hz:100000000,sample_index:0,value:-30},{frequency_hz:101000000,sample_index:1,value:-20},{frequency_hz:102000000,sample_index:2,value:-30}],
                spectrum_statistics:{minimum:[],maximum:[]},workspace:{tabs:[],power:null},minimum:-50,maximum:0})
            const button=findChild(workspace,"spectrumWindowAverageButton")
            waitForPolish(button);verify(waitForRendering(button));mouseClick(button)
            compare(service.window,101)
            const chart=findChild(workspace,"spectrumCollectionChart")
            compare(chart.spectrumEnabled,true);compare(chart.rxmerEnabled,false);compare(chart.unit,"dBmV")
        }
    }
}
