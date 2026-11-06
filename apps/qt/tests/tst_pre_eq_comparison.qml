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
    name: "PreEqComparison"
    width: 1280; height: 820; visible: true; when: windowShown
    Theme { id: appearance }
    property string calculated: ""
    property int attached: 0
    property int opened: 0
    property int removed: 0
    property int selectedStart: -1
    property int selectedCount: -1
    property int resets: 0
    property var pair: ({original_header:{capture_time:100},correction_header:{capture_time:108},file_time_difference_s:8,sample_count:3,mac_address:"aa:bb:cc:dd:ee:ff",cmts_mac_address:"00:01:02:03:04:05"})
    FileErrorDialog { id: fileError; theme: appearance }
    PreEqComparisonSidebar {
        id: sidebar
        width:244
        theme: appearance
        onShowComparison: comparisonSelected=true
        onShowOriginal: comparisonSelected=false
        onCalculate: mode=>testCase.calculated=mode
        onRemoveAttachment: testCase.removed++
    }
    PreEqComparisonView {
        id: comparison
        x:250; width:1000; height:760
        theme: appearance
        result: ({pair:testCase.pair,mode:"adjust",start:0,count:3,summary:{original_magnitude_span_db:1,predicted_magnitude_span_db:0.8,aligned_delta_min_db:-.1,aligned_delta_max_db:.2},points:[
            {sample_index:0,frequency_hz:40000000,original_db:0,predicted_db:0.2,delta_db:0.2,original_aligned_db:0,predicted_aligned_db:.2,aligned_delta_db:.2,original_delay_s:0,predicted_delay_s:0,delta_delay_s:0},
            {sample_index:1,frequency_hz:45000000,original_db:1,predicted_db:.9,delta_db:-.1,original_aligned_db:1,predicted_aligned_db:.9,aligned_delta_db:-.1,original_delay_s:1e-8,predicted_delay_s:2e-8,delta_delay_s:1e-8},
            {sample_index:2,frequency_hz:50000000,original_db:null,predicted_db:null,delta_db:null,original_aligned_db:null,predicted_aligned_db:null,aligned_delta_db:null,original_delay_s:null,predicted_delay_s:null,delta_delay_s:null}]})
        onRangeRequested: (start,count)=>{testCase.selectedStart=start;testCase.selectedCount=count}
        onResetRequested: testCase.resets++
    }
    CaptureDropArea {
        id: drop
        anchors.fill: parent
        theme: appearance
        attachmentTarget: sidebar.dropTarget
        onAttachmentDropped: testCase.attached++
        onFileDropped: testCase.opened++
    }
    function init() {
        fileError.close()
        calculated="";removed=0;attached=0;opened=0;resets=0;selectedStart=-1;selectedCount=-1
        sidebar.comparisonSelected=true
        sidebar.attachment=({});sidebar.resultMode="";sidebar.working=false
        appearance.preference=1
        drop.enabled=false
    }
    function test_gutter_precheck_mode_required_and_recalculate_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_gutter_precheck_mode_required_and_recalculate(data) {
        appearance.preference=data.theme
        const action=findChild(sidebar,"calculatePreEqComparison")
        const mode=findChild(sidebar,"preEqUpdateMode")
        verify(!action.enabled)
        sidebar.attachment=({pair:testCase.pair})
        compare(findChild(sidebar,"preEqPrecheckStatus"),null)
        verify(!action.enabled)
        verify(!sidebar.children.some(child=>typeof child.text==="string" && child.text.indexOf("CMTS  ")>=0))
        mode.currentIndex=1
        verify(action.enabled)
        verify(waitForPolish(sidebar))
        mouseClick(action)
        compare(calculated,"adjust")
        sidebar.resultMode="adjust"
        compare(action.borderHighlighted,false)
        mode.currentIndex=2
        verify(action.borderHighlighted)
        mouseClick(action)
        compare(calculated,"set")
        sidebar.working=true
        verify(!action.enabled)
        sidebar.working=false
        sidebar.attachment=({pair:testCase.pair})
        compare(mode.currentIndex,0)
        verify(!action.enabled)
        mouseClick(findChild(sidebar,"removePreEqUpdate"))
        compare(removed,1)
    }
    function test_compare_navigation_keeps_controls_with_the_tool() {
        const controls=findChild(sidebar,"preEqCompareControls")
        mouseClick(findChild(sidebar,"showOriginalPreEq"))
        verify(!sidebar.comparisonSelected)
        verify(!controls.visible)
        mouseClick(findChild(sidebar,"showPreEqCompare"))
        verify(sidebar.comparisonSelected)
        verify(controls.visible)
    }
    function test_file_error_popup_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_file_error_popup(data) {
        appearance.preference=data.theme
        fileError.showError("")
        verify(!fileError.visible)
        const message="cmts_mac_address: Expected matching CMTS MAC."
        fileError.showError(message)
        tryCompare(fileError,"visible",true)
        compare(findChild(fileError,"fileErrorMessage").text,message)
        verify(fileError.modal)
        const dismiss=findChild(fileError,"dismissFileError")
        verify(waitForPolish(fileError.contentItem))
        mouseClick(dismiss)
        tryCompare(fileError,"visible",false)
        fileError.showError(message)
        tryCompare(fileError,"visible",true)
        fileError.close()
    }
    function test_attachment_drop_is_consumed_without_opening_a_new_capture() {
        drop.enabled=true
        verify(waitForPolish(sidebar))
        const position=sidebar.dropTarget.mapToItem(drop,10,10)
        let accepted=false
        const event={x:position.x,y:position.y,hasUrls:true,urls:["file:///synthetic/update.bin"],supportedActions:Qt.CopyAction,accepted:false,accept:function(action){accepted=action===Qt.CopyAction}}
        drop.handleDrop(event)
        verify(accepted);compare(attached,1);compare(opened,0)
        event.x=1000;event.y=100
        drop.handleDrop(event)
        compare(attached,1);compare(opened,1)
        event.urls=["file:///one","file:///two"]
        drop.handleDrop(event)
        compare(attached,1);compare(opened,1)
    }
    function test_paired_graph_metric_alignment_iq_and_mouse_zoom_data() {return [{tag:"dark",theme:1},{tag:"light",theme:2}]}
    function test_paired_graph_metric_alignment_iq_and_mouse_zoom(data) {
        appearance.preference=data.theme
        const plot=findChild(comparison,"preEqComparisonOverlay")
        plot.requestPaint();verify(waitForRendering(plot))
        const image=grabImage(plot)
        verify(image.width>500 && image.height>100)
        mousePress(plot,100,60);mouseMove(plot,500,60);mouseRelease(plot,500,60)
        verify(selectedStart>=0);verify(selectedCount>0)
        mouseDoubleClickSequence(plot,200,60)
        compare(resets,1)
        const metric=findChild(comparison,"preEqComparisonMetric")
        metric.currentIndex=1
        plot.requestPaint();verify(waitForRendering(plot))
        metric.currentIndex=2
        verify(!plot.visible)
        metric.currentIndex=0
        findChild(comparison,"preEqAlignment").currentIndex=1
        plot.requestPaint();verify(waitForRendering(plot))
    }
}
