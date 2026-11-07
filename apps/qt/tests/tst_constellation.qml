/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "helpers"
import "../qml"

RenderTestCase {
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
        chart.showSoft = true
        chart.showHard = true
        chart.showBoundaries = true
        chart.showOverlay = true
        chart.requested = []
        chart.resets = 0
        chart.width = 800
        chart.height = 650
        chart.constellation = {bounds: {minimum_i: -2, maximum_i: 2, minimum_q: -2, maximum_q: 2},
            soft_decisions: [[0.5,0.5],[-0.5,-0.5]],hard_decisions: [[1,1],[-1,-1]],
            soft_visible: 2,hard_visible: 2,reference_available: true, decision_boundaries: [[[0,-2],[0,2]],[[-2,0],[2,0]]],
            soft_label: "Soft decisions: 2 of 2 visible",hard_label: "Hard decisions: 2 of 2 visible",
            mer: {calculation_status: "available", mer_db: 40.24, rms_evm_percent: 0.9723, sample_count: 8192}}
        wait(20)
    }
    function test_mer_updates_with_capture_and_ignores_layer_toggles() {
        const badge = findChild(chart, "constellationMerOverlay")
        compare(badge.label, "MER 40.2 dB\nEVM 0.97 %")
        for (const name of ["constellationSoftToggle", "constellationHardToggle", "constellationBoundaryToggle"]) {
            mouseClick(findChild(chart, name))
            compare(badge.label, "MER 40.2 dB\nEVM 0.97 %")
            compare(chart.constellation.mer.sample_count, 8192)
        }
        chart.constellation = Object.assign({}, chart.constellation, {
            mer: {calculation_status: "available", mer_db: 30.04, rms_evm_percent: 3.1472, sample_count: 512}})
        compare(badge.label, "MER 30.0 dB\nEVM 3.15 %")
        for (const status of ["perfect_reference", "no_samples", "invalid_samples", "sample_count_mismatch",
            "zero_signal_power", "non_finite_result", "unsupported_modulation"]) {
            chart.constellation = Object.assign({}, chart.constellation, {
                mer: {calculation_status: status, mer_db: null}})
            compare(badge.label, "MER —\nEVM —")
        }
        for (const invalid of [NaN, Infinity, -Infinity, null]) {
            chart.constellation = Object.assign({}, chart.constellation, {
                mer: {calculation_status: "available", mer_db: invalid}})
            compare(badge.label, "MER —\nEVM —")
        }
    }
    function test_overlay_toggle_hides_badge_without_changing_measurement() {
        const badge = findChild(chart, "constellationMerOverlay")
        const toggle = findChild(chart, "constellationOverlayToggle")
        const original = JSON.stringify(chart.constellation.mer)
        compare(toggle.width, findChild(chart, "constellationSoftToggle").width)
        verify(badge.visible)
        mouseClick(toggle)
        compare(badge.visible, false)
        compare(JSON.stringify(chart.constellation.mer), original)
        mouseClick(toggle)
        compare(badge.visible, true)
        compare(badge.label, "MER 40.2 dB\nEVM 0.97 %")
        chart.constellation = Object.assign({}, chart.constellation, {
            mer: {calculation_status: "perfect_reference", mer_db: null, rms_evm_percent: 0}})
        compare(badge.label, "MER —\nEVM 0.00 %")
    }
    function test_mer_badge_tracks_plot_corner_and_does_not_capture_mouse() {
        const plot = findChild(chart, "constellationPlot")
        const badge = findChild(chart, "constellationMerOverlay")
        for (const dimensions of [[800, 650], [620, 480]]) {
            chart.width = dimensions[0]
            chart.height = dimensions[1]
            waitForPolish(chart)
            verify(waitForFrame(plot))
            const toggle = findChild(chart, "constellationOverlayToggle")
            const togglePosition = toggle.mapToItem(chart, 0, 0)
            verify(togglePosition.x >= 0 && togglePosition.x + toggle.width <= chart.width)
            fuzzyCompare(badge.x + badge.width, plot.width - plot.leftMargin - 6, 0.1)
            fuzzyCompare(badge.y + badge.height, plot.height - plot.topMargin - 6, 0.1)
            compare(plot.width, plot.height)
            const horizontal = badge.x + badge.width / 2
            const vertical = badge.y + badge.height / 2
            chart.requested = []
            mousePress(plot, horizontal, vertical)
            mouseMove(plot, plot.width / 2, plot.height / 2)
            mouseRelease(plot, plot.width / 2, plot.height / 2)
            compare(chart.requested.length, 4)
            chart.requested = []
            mouseWheel(plot, horizontal, vertical, 0, 120)
            compare(chart.requested.length, 4)
            compare(badge.label, "MER 40.2 dB\nEVM 0.97 %")
        }
    }
    function test_independent_layer_controls_and_unknown_reference() {
        const soft_decisions=findChild(chart,"constellationSoftToggle")
        const hard_decisions=findChild(chart,"constellationHardToggle")
        mouseClick(soft_decisions)
        compare(chart.showSoft,false)
        compare(chart.showHard,true)
        const boundary=findChild(chart,"constellationBoundaryToggle")
        compare(boundary.width,soft_decisions.width)
        mouseClick(boundary)
        compare(chart.showBoundaries,false)
        compare(chart.showHard,true)
        mouseClick(hard_decisions)
        compare(chart.showHard,false)
        mouseClick(soft_decisions)
        compare(chart.showSoft,true)
        chart.constellation = Object.assign({},chart.constellation,{reference_available:false,hard_decisions:[]})
        compare(hard_decisions.enabled,false)
        compare(boundary.enabled,false)
        compare(soft_decisions.enabled,true)
    }
    function test_boundaries_toggle_changes_plot_without_hiding_symbols() {
        const plot=findChild(chart,"constellationPlot")
        chart.showSoft=false
        chart.showHard=false
        chart.showBoundaries=true
        wait(40)
        const withBoundaries=grabImage(plot)
        const horizontal=Math.round(plot.leftMargin+plot.plotSize/2)
        const vertical=Math.round(plot.topMargin+plot.plotSize/4)
        chart.showBoundaries=false
        wait(40)
        const withoutBoundaries=grabImage(plot)
        let difference=0
        // Canvas strokes can cover either adjacent pixel at subpixel positions.
        for (let offset=-2;offset<=2;offset++) {
            const first=withBoundaries.pixel(horizontal+offset,vertical)
            const second=withoutBoundaries.pixel(horizontal+offset,vertical)
            difference=Math.max(difference,Math.abs(first.r-second.r)+Math.abs(first.g-second.g)+Math.abs(first.b-second.b)+Math.abs(first.a-second.a))
        }
        verify(difference>0.01)
        compare(chart.showSoft,false)
        compare(chart.showHard,false)
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
        chart.constellation=Object.assign({},chart.constellation,{soft_decisions:[],hard_decisions:[],soft_visible:0,hard_visible:0})
        wait(20)
        verify(grabImage(plot).width>100)
    }
}
