/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    name: "ConstellationPresentation"
    when: windowShown
    visible: true
    width: 850
    height: 700
    Theme { id: appearance }
    ConstellationChart {
        id: chart
        width: 800
        height: 650
        theme: appearance
        property var requested: []
        property int resets: 0
        onBoundsRequested: (firstI,lastI,firstQ,lastQ) => requested = [firstI,lastI,firstQ,lastQ]
        onResetRequested: resets++
    }
    function init() {
        chart.showMeasured = true
        chart.showIdeal = true
        chart.requested = []
        chart.resets = 0
        chart.constellation = {bounds: {minimum_i: -2, maximum_i: 2, minimum_q: -2, maximum_q: 2},
            measured: [[0.5,0.5],[-0.5,-0.5]],ideal: [[1,1],[-1,-1]],
            measured_visible: 2,ideal_visible: 2,reference_available: true,
            measured_label: "Measured: 2 of 2 visible",ideal_label: "Ideal: 2 of 2 visible"}
        wait(20)
    }
    function test_independent_layer_controls_and_unknown_reference() {
        const measured=findChild(chart,"constellationMeasuredToggle")
        const ideal=findChild(chart,"constellationIdealToggle")
        mouseClick(measured)
        compare(chart.showMeasured,false)
        compare(chart.showIdeal,true)
        mouseClick(ideal)
        compare(chart.showIdeal,false)
        mouseClick(measured)
        compare(chart.showMeasured,true)
        chart.constellation = Object.assign({},chart.constellation,{reference_available:false,ideal:[]})
        compare(ideal.enabled,false)
        compare(measured.enabled,true)
    }
    function test_drag_wheel_reset_and_outside_plot() {
        const plot=findChild(chart,"constellationPlot")
        const left=plot.leftMargin+plot.plotSize*0.25
        const right=plot.leftMargin+plot.plotSize*0.75
        const upper=plot.topMargin+plot.plotSize*0.25
        const lower=plot.topMargin+plot.plotSize*0.75
        mousePress(plot,left,upper)
        mouseMove(plot,right,lower)
        mouseRelease(plot,right,lower)
        compare(chart.requested.length,4)
        fuzzyCompare(chart.requested[0],-1,0.02)
        fuzzyCompare(chart.requested[1],1,0.02)
        fuzzyCompare(chart.requested[2],1,0.02)
        fuzzyCompare(chart.requested[3],-1,0.02)
        mouseWheel(plot,plot.leftMargin+plot.plotSize/2,plot.topMargin+plot.plotSize/2,0,120)
        fuzzyCompare(chart.requested[0],-1.6,0.02)
        fuzzyCompare(chart.requested[1],1.6,0.02)
        mouseDoubleClickSequence(plot,left,upper)
        compare(chart.resets,1)
        chart.requested=[]
        mousePress(plot,2,2)
        mouseMove(plot,right,lower)
        mouseRelease(plot,right,lower)
        compare(chart.requested.length,0)
    }
    function test_equal_aspect_render_in_both_themes_and_empty_region() {
        const plot=findChild(chart,"constellationPlot")
        compare(plot.width,plot.height)
        for (const preference of [1,2]) {
            appearance.preference=preference
            wait(40)
            const image=grabImage(plot)
            verify(image.width>100&&image.height>100)
        }
        chart.constellation=Object.assign({},chart.constellation,{measured:[],ideal:[],measured_visible:0,ideal_visible:0})
        wait(20)
        verify(grabImage(plot).width>100)
    }
}
