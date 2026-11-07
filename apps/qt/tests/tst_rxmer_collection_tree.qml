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
    name: "RxMerCollectionTree"
    width: 500; height: 700
    visible: true
    when: windowShown
    property int requestedGroup: -1
    property string requestedMac: ""
    Theme { id: appearance }
    RxMerCollectionTree {
        id: tree
        width: 340; height: 360
        theme: appearance
        onGroupRequested: (macAddress,groupId)=>{testCase.requestedMac=macAddress;testCase.requestedGroup=groupId}
    }
    function init() {
        tree.catalog=({});tree.selectedMac="";tree.working=false;tree.playbackEnabled=false
        requestedGroup=-1;requestedMac=""
    }
    function test_mac_channel_epoch_tree_and_channel_selection_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_mac_channel_epoch_tree_and_channel_selection(data) {
        appearance.preference=data.theme
        const groups=[]
        for (let channel=0;channel<5;channel++) groups.push({id:channel,mac_address:"aa:bb:cc:dd:ee:ff",tree_label:"Channel "+(193+channel)+" · 827.600 MHz",label:"Channel grid",capture_count:2,sources:[{capture_time:10,name:"late-filename.bin"},{capture_time:20,name:"early-filename.bin"}]})
        groups.push({id:5,mac_address:"ff:ee:dd:cc:bb:aa",tree_label:"Channel 193 · 900.000 MHz",label:"Different MAC",capture_count:1,sources:[{capture_time:30,name:"other.bin"}]})
        tree.catalog=({groups:groups});tree.selectedMac="aa:bb:cc:dd:ee:ff"
        compare(tree.rows().length,7)
        tree.toggleGroup(0)
        const rows=tree.rows()
        compare(rows[0].kind,"mac");compare(rows[1].kind,"channel")
        compare(rows[1].label,"Channel 193 · 827.600 MHz")
        compare(rows[2].label,"10");compare(rows[3].label,"20")
        const nodes=findChild(tree,"rxmerCaptureTree")
        verify(waitForPolish(nodes))
        const action=findChild(tree,"rxmerTreeChannel_0")
        verify(action!==null)
        mouseClick(action)
        compare(requestedMac,"aa:bb:cc:dd:ee:ff");compare(requestedGroup,0)
        tree.working=true;verify(!action.enabled)
        tree.toggleMac("aa:bb:cc:dd:ee:ff")
        compare(tree.rows().length,2)
    }
    function test_playback_epoch_leaf_selects_header_ordered_frame_data() {
        return [{tag:"dark",theme:1},{tag:"light",theme:2}]
    }
    function test_playback_epoch_leaf_selects_header_ordered_frame(data) {
        appearance.preference=data.theme
        tree.playbackEnabled=true
        tree.catalog=({groups:[{id:0,mac_address:"aa:bb:cc:dd:ee:ff",label:"Channel 34",capture_count:2,sources:[{capture_time:10,name:"z.bin"},{capture_time:20,name:"a.bin"}]}]})
        tree.selectedMac="aa:bb:cc:dd:ee:ff";tree.toggleGroup(0)
        const spy=createTemporaryObject(signalSpy,tree,{target:tree,signalName:"frameRequested"})
        const nodes=findChild(tree,"rxmerCaptureTree")
        verify(waitForPolish(nodes))
        nodes.forceLayout()
        nodes.positionViewAtIndex(3,ListView.Contain)
        tryVerify(()=>findChild(tree,"rxmerTreeFrame_0_1")!==null)
        const action=findChild(tree,"rxmerTreeFrame_0_1")
        verify(waitForPolish(action))
        // A static Cocoa window may have no new frame to await. Synchronize
        // the click with delegate layout and assert the actual routed signal.
        tryVerify(()=>action.visible && action.enabled && action.width>0 && action.height>0)
        mouseClick(action)
        tryCompare(spy,"count",1)
        compare(Array.from(spy.signalArguments[0]),["aa:bb:cc:dd:ee:ff",0,1])
        tree.playbackEnabled=false
    }
    Component { id: signalSpy; SignalSpy {} }
    function test_hundreds_of_epochs_use_virtualized_rows() {
        const sources=[]
        for (let epoch=0;epoch<250;epoch++) sources.push({capture_time:1000+epoch,name:"capture"+epoch+".bin"})
        tree.catalog=({groups:[{id:0,mac_address:"aa:bb:cc:dd:ee:ff",tree_label:"Channel 193 · 827.600 MHz",label:"Grid",capture_count:250,sources:sources}]})
        tree.selectedMac="aa:bb:cc:dd:ee:ff";tree.toggleGroup(0)
        compare(tree.rows().length,252)
        const nodes=findChild(tree,"rxmerCaptureTree")
        verify(waitForPolish(nodes))
        verify(nodes.contentItem.children.length<100)
    }
}
