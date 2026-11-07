/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml"
TestCase {
    id:testCase;name:"PreEqWorkspace";width:1280;height:820;visible:true;when:windowShown
    Theme {id:appearance}
    QtObject {
        id:service
        property bool busy:false
        property bool pre_eq_busy:false
        property bool pre_eq_current:false
        property string pre_eq_attachment_json:"{}"
        property string pre_eq_json:"{}"
        function compare_pre_eq(mode){}
        function remove_pre_eq_update(){}
        function show_pre_eq_range(start,count){}
    }
    PreEqWorkspace {id:workspace;anchors.fill:parent;theme:appearance;service:service}
    SignalSpy {id:choose;target:workspace;signalName:"chooseOriginal"}
    function test_comparison_can_start_without_single_capture(){
        compare(service.pre_eq_current,false)
        const button=findChild(workspace,"choosePreEqOriginal")
        verify(button.enabled)
        mouseClick(button);compare(choose.count,1)
        const update=findChild(workspace,"choosePreEqUpdate")
        compare(update.enabled,false)
        service.pre_eq_current=true
        tryCompare(update,"enabled",true)
    }
}
