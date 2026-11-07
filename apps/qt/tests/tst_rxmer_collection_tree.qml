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
        tree.catalog=({});tree.selectedMac="";tree.working=false
        requestedGroup=-1;requestedMac=""
    }
    function test_mac_channel_epoch_tree_and_channel_selection_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_mac_channel_epoch_tree_and_channel_selection(data) {
        appearance.preference=data.theme
        const groups=[]
        for (let channel=0;channel<5;channel++) groups.push({id:channel,mac_address:"aa:bb:cc:dd:ee:ff",tree_label:"Channel "+(193+channel)+" · 827.600 MHz (index 0)",label:"Channel grid",capture_count:2,sources:[{capture_time:10,name:"late-filename.bin"},{capture_time:20,name:"early-filename.bin"}]})
        groups.push({id:5,mac_address:"ff:ee:dd:cc:bb:aa",tree_label:"Channel 193 · 900.000 MHz (index 0)",label:"Different MAC",capture_count:1,sources:[{capture_time:30,name:"other.bin"}]})
        tree.catalog=({groups:groups});tree.selectedMac="aa:bb:cc:dd:ee:ff"
        compare(tree.rows().length,7)
        tree.toggleGroup(0)
        const rows=tree.rows()
        compare(rows[0].kind,"mac");compare(rows[1].kind,"channel")
        compare(rows[1].label,"Channel 193 · 827.600 MHz (index 0)")
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
    function test_hundreds_of_epochs_use_virtualized_rows() {
        const sources=[]
        for (let epoch=0;epoch<250;epoch++) sources.push({capture_time:1000+epoch,name:"capture"+epoch+".bin"})
        tree.catalog=({groups:[{id:0,mac_address:"aa:bb:cc:dd:ee:ff",tree_label:"Channel 193 · 827.600 MHz (index 0)",label:"Grid",capture_count:250,sources:sources}]})
        tree.selectedMac="aa:bb:cc:dd:ee:ff";tree.toggleGroup(0)
        compare(tree.rows().length,252)
        const nodes=findChild(tree,"rxmerCaptureTree")
        verify(waitForPolish(nodes))
        verify(nodes.contentItem.children.length<100)
    }
}
