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
    name: "ChartSettings"
    width: 900
    height: 700
    visible: true
    when: windowShown
    Theme { id: appearance }
    ChartSettingsDialog { id: settings; theme: appearance }
    MeasurementChart {
        id: chart
        width: 850
        height: 600
        theme: appearance
        minimum: 0
        maximum: 80
        viewport: ({start:0,count:2,total:2,points:[
            {frequency_hz:100000000,value:40},{frequency_hz:101000000,value:40}],
            rxmer_statistics:{minimum:[{frequency_hz:100000000,value:20},{frequency_hz:101000000,value:20}],
                maximum:[{frequency_hz:100000000,value:60},{frequency_hz:101000000,value:60}]}})
    }
    function init() { appearance.preference=1;appearance.resetChartStyle() }
    function cleanup() { settings.close();appearance.resetChartStyle() }
    function test_default_width_and_theme_aware_palette() {
        compare(appearance.chartLineWidth,0.75)
        compare(appearance.rxmerMinimum.toString(),"#64d98b")
        compare(appearance.rxmerAverage.toString(),"#69aaff")
        compare(appearance.rxmerMaximum.toString(),"#ff7777")
        appearance.preference=2
        compare(appearance.rxmerMinimum.toString(),"#237c3f")
        compare(appearance.rxmerAverage.toString(),"#2563a6")
        compare(appearance.rxmerMaximum.toString(),"#b42318")
    }
    function test_width_control_repaints_fractional_traces() {
        const plot=findChild(chart,"measurementPlot")
        verify(waitForFrame(plot))
        const thin=grabImage(plot)
        const selector=findChild(settings,"chartLineWidthSelector")
        selector.currentIndex=5;selector.activated(5)
        compare(appearance.chartLineWidth,2)
        verify(waitForFrame(plot))
        const thick=grabImage(plot)
        const horizontal=Math.round((plot.leftMargin+plot.width-plot.rightMargin)/2)
        let differentPixels=0
        for(let vertical=plot.topMargin+2;vertical<plot.height-plot.bottomMargin-2;vertical++) {
            if(!Qt.colorEqual(thin.pixel(horizontal,vertical),thick.pixel(horizontal,vertical))) {
                differentPixels++
            }
        }
        verify(differentPixels>=3,"All three traces must respond to the global width change")
    }
    function test_color_picker_commit_and_reset_both_themes() {
        const picker=findChild(settings,"chartColorPicker")
        const plot=findChild(chart,"measurementPlot")
        for(const preference of [1,2]) {
            appearance.preference=preference
            appearance.chartLineWidth=2
            verify(waitForFrame(plot))
            const before=grabImage(plot)
            settings.editingColor="minimumColorOverride"
            picker.selectedColor="#e040fb";picker.accepted()
            compare(appearance.rxmerMinimum.toString(),"#e040fb")
            verify(waitForFrame(plot))
            const after=grabImage(plot)
            const horizontal=Math.round((plot.leftMargin+plot.width-plot.rightMargin)/2)
            const vertical=Math.round(plot.topMargin+0.75*(plot.height-plot.topMargin-plot.bottomMargin))
            let changed=false
            for(let offset=-2;offset<=2;offset++) {
                if(!Qt.colorEqual(before.pixel(horizontal,vertical+offset),after.pixel(horizontal,vertical+offset))) {
                    changed=true
                }
            }
            verify(changed,"Color changes must repaint the minimum trace immediately")
            settings.open()
            tryCompare(settings,"opened",true)
            const reset=findChild(settings,"resetChartStyleButton")
            verify(waitForFrame(reset))
            mouseClick(reset)
            compare(appearance.chartLineWidth,0.75)
            compare(appearance.minimumColorOverride,"")
            compare(appearance.rxmerMinimum.toString(),preference===1 ? "#64d98b" : "#237c3f")
            settings.close()
            tryCompare(settings,"opened",false)
        }
    }
}
