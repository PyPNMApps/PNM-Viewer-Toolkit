/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "helpers"
import "../qml"
RenderTestCase {
    id: testCase
    name: "RxMerPlayback"
    width: 1100; height: 800
    visible: true
    when: windowShown
    property int requested: -1
    Theme { id: appearance }
    RxMerPlaybackControls {
        id: controls
        theme: appearance
        onFrameRequested: frameIndex=> { testCase.requested=frameIndex; controls.frameIndex=frameIndex }
    }
    SignalSpy {
        id: frameRequests
        target: controls
        signalName: "frameRequested"
    }
    RxMerCollectionView {
        id: workspace
        y: 80; width: 1080; height: 700
        theme: appearance
        visible: false
        property var lastRequest: []
        onFrameRequested: (groupId,frameIndex,start,count)=>lastRequest=[groupId,frameIndex,start,count]
    }
    function init() {
        controls.playing=false;controls.suspended=false;controls.frameCount=3;controls.frameIndex=0
        requested=-1
        frameRequests.clear()
        workspace.visible=false;workspace.catalog=({});workspace.viewport=({});workspace.playbackMode=false
    }
    function cleanup() {
        controls.playing=false
    }
    function test_play_pause_step_restart_and_end_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_play_pause_step_restart_and_end(data) {
        appearance.preference=data.theme
        const play=findChild(controls,"rxmerPlayPause")
        verify(waitForPolish(controls));verify(waitForFrame(play))
        // Keep Pause independent of reaching the last frame on a busy host.
        controls.frameCount=100
        mouseClick(play);verify(controls.playing)
        mouseClick(play);verify(!controls.playing)
        controls.jump(0)
        controls.frameCount=3
        frameRequests.clear()
        mouseClick(play);verify(controls.playing)
        tryCompare(controls,"playing",false,2500)
        compare(controls.frameIndex,2)
        // Inspect every emitted frame after completion, rather than polling
        // frame 1 while the timer is already advancing toward frame 2.
        compare(frameRequests.signalArguments.map(arguments=>arguments[0]),[1,2])
        mouseClick(findChild(controls,"rxmerPreviousFrame"));compare(controls.frameIndex,1)
        mouseClick(findChild(controls,"rxmerNextFrame"));compare(controls.frameIndex,2)
        verify(!findChild(controls,"rxmerNextFrame").enabled)
        frameRequests.clear()
        mouseClick(play);compare(controls.frameIndex,0);verify(controls.playing)
        tryCompare(controls,"playing",false,2500)
        compare(controls.frameIndex,2)
        compare(frameRequests.signalArguments.map(arguments=>arguments[0]),[0,1,2])
        mouseClick(findChild(controls,"rxmerPreviousFrame"));compare(controls.frameIndex,1)
    }
    function test_blocking_and_one_capture_disable_playback() {
        controls.playing=true;controls.suspended=true;verify(!controls.playing)
        controls.jump(1);compare(requested,-1)
        controls.suspended=false;controls.frameCount=1
        verify(!findChild(controls,"rxmerPlayPause").enabled)
        controls.jump(-1);controls.jump(1);compare(requested,-1)
    }
    function test_average_mer_chip_uses_rust_value_with_one_decimal() {
        workspace.visible=true;workspace.playbackMode=true
        workspace.viewport=({group:{channel_id:34},frame_index:0,frame_count:1,average_mer_db:39.875,capture_epoch:10,points:[]})
        const average=findChild(workspace,"rxmerPlaybackAverage")
        compare(average.title,"Avg MER");compare(average.value,"39.9 dB")
        workspace.viewport=({group:{channel_id:34},average_mer_db:null,points:[]})
        compare(average.value,"—")
    }
    function test_workspace_frame_and_zoom_requests_reuse_header_grid() {
        workspace.visible=true;workspace.playbackMode=true
        workspace.catalog=({groups:[{id:4,mac_address:"aa:bb:cc:dd:ee:ff",sample_count:7480,label:"Channel 34"}]})
        findChild(workspace,"rxmerDeviceDialog").accept()
        compare(workspace.lastRequest,[4,0,0,7480])
        workspace.viewport=({group:{channel_id:34},frame_index:2,frame_count:10,start:100,count:200,total:7480,points:[]})
        workspace.showFrame(3);compare(workspace.lastRequest,[4,3,100,200])
        const chart=findChild(workspace,"rxmerCollectionChart")
        chart.rangeRequested(120,50);compare(workspace.lastRequest,[4,2,120,50])
        workspace.showFrame(4);compare(workspace.lastRequest,[4,4,100,200])
        const playback=findChild(workspace,"rxmerPlaybackControls")
        playback.playing=true;workspace.visible=false;verify(!playback.playing)
    }
}
