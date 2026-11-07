/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "helpers"
import "../qml"

RenderTestCase {
    id: testCase
    name: "RxMerBitLoading"
    width: 1280; height: 820
    visible: true
    when: windowShown
    Theme { id: appearance }
    property var requests: []
    property int exports: 0
    property int cancels: 0
    property int contextRequests: 0
    MeasurementChart {
        id: original
        theme: appearance
        width: 1200; height: 700
        rxmerEnabled: true
        viewport: ({start: 400, count: 1001, total: 7480, points: [
            {sample_index:400,frequency_hz:650000000,value:41.75},
            {sample_index:1400,frequency_hz:675000000,value:42}]})
        onBitLoadingRequested: testCase.contextRequests++
    }
    RxMerBitLoadingDialog {
        id: insights
        theme: appearance
        analysis: ({model:{maximum_bit_loading:13},selection:{start_frequency_hz:650000000,stop_frequency_hz:675000000},
            coverage:{measured_count:1001,unavailable_count:0},summary:{minimum_rxmer_db:39.25,uniform_bit_loading:10,uniform_modulation:"qam_1024",limiting_frequency_hz:651000000,remaining_margin_db:0.25,status:"meets_measured_target"},
            rate:{estimated_gross_rate_bps:250250000},modulation_caps:[{bits:10,label:"1024-QAM"},{bits:13,label:"8192-QAM"}],
            uniform_scenarios:[{bit_loading:10,modulation:"qam_1024",estimated_gross_rate_bps:250250000,status:"meets_target"},{bit_loading:13,modulation:"qam_8192",estimated_gross_rate_bps:325325000,status:"not_covered_by_model"}],
            subcarriers:[{sample_index:400,frequency_hz:650000000,bit_loading:10,ideal_bit_loading:13},{sample_index:1400,frequency_hz:675000000,bit_loading:11,ideal_bit_loading:13}]})
        onAnalysisRequested: (start,count,model,margin,cap) => testCase.requests = testCase.requests.concat([{start:start,count:count,model:model,margin:margin,cap:cap}])
        onExportRequested: testCase.exports++
        onCancelRequested: testCase.cancels++
    }
    function init() {
        insights.close()
        requests = []
        appearance.preference = 1
        original.visible = true
        original.rxmerEnabled = true
        original.spectrumEnabled = false
        insights.working = false
        insights.selectedCap = 13
        findChild(insights.contentItem,"rxmerBitLoadingModel").currentIndex = 1
    }
    function cleanup() { insights.close() }
    function test_full_capture_below_model_target_is_a_completed_result_data() {
        return [{tag: "dark", preference: 1}, {tag: "light", preference: 2}]
    }
    function test_full_capture_below_model_target_is_a_completed_result(data) {
        const zoomAnalysis = insights.analysis
        try {
            appearance.preference = data.preference
            insights.analysis = Object.assign({}, zoomAnalysis, {
                selection: {start_frequency_hz:640000000,stop_frequency_hz:826975000},
                coverage: {measured_count:7480,unavailable_count:0},
                summary: {minimum_rxmer_db:28.25,uniform_bit_loading:null,uniform_modulation:null,
                    limiting_frequency_hz:750000000,status:"below_model_minimum"},
                rate: {estimated_gross_rate_bps:null}
            })
            insights.openRange(0,7480)
            tryCompare(insights,"opened",true)
            compare(insights.title,"RxMER · Modulation & Capacity")
            compare(requests[0].start,0)
            compare(requests[0].count,7480)
            const status = findChild(insights.contentItem,"rxmerModulationStatus")
            verify(status.visible)
            verify(status.text.indexOf("Analysis complete") >= 0)
            verify(status.text.indexOf("weakest carrier") >= 0)
            compare(findChild(insights.contentItem,"rxmerGrossRateValue").text,"Unavailable")
            verify(findChild(insights.contentItem,"rxmerGrossRateChip").enabled)
        } finally {
            insights.close()
            insights.analysis = zoomAnalysis
        }
    }
    function test_changed_controls_emphasize_recalculate_until_result_arrives_data() {
        return [{tag: "dark", preference: 1}, {tag: "light", preference: 2}]
    }
    function test_changed_controls_emphasize_recalculate_until_result_arrives(data) {
        const savedAnalysis = insights.analysis
        const margin = findChild(insights.contentItem,"rxmerBitLoadingMargin")
        const model = findChild(insights.contentItem,"rxmerBitLoadingModel")
        const recalculate = findChild(insights.contentItem,"recalculateRxmerBitLoading")
        try {
            appearance.preference = data.preference
            margin.text = "0.0"
            insights.analysis = Object.assign({}, savedAnalysis, {
                model: {id:"ossi_candidate_defaults",extra_margin_db:0,maximum_bit_loading:13}
            })
            insights.openRange(0,7480)
            tryCompare(insights,"opened",true)
            compare(recalculate.borderHighlighted,false)
            margin.text = "1.0"
            compare(recalculate.borderHighlighted,true)
            compare(recalculate.background.border.color,appearance.accent)
            compare(recalculate.background.border.width,2)
            margin.text = "0.0"
            compare(recalculate.borderHighlighted,false)
            model.currentIndex = 0
            compare(recalculate.borderHighlighted,true)
            model.currentIndex = 1
            compare(recalculate.borderHighlighted,false)
            insights.selectedCap = 10
            compare(recalculate.borderHighlighted,true)
            mouseClick(recalculate)
            compare(requests[requests.length-1].cap,10)
            // A submitted request is not yet a successful result.
            compare(recalculate.borderHighlighted,true)
            insights.working = true
            compare(recalculate.borderHighlighted,true)
            insights.working = false
            insights.analysis = Object.assign({}, savedAnalysis, {
                model: {id:"ossi_candidate_defaults",extra_margin_db:0,maximum_bit_loading:10}
            })
            compare(recalculate.borderHighlighted,false)
            insights.selectedCap = 13
            compare(recalculate.borderHighlighted,true)
        } finally {
            insights.close()
            insights.analysis = savedAnalysis
            insights.selectedCap = 13
            margin.text = "0.0"
        }
    }
    function test_popup_uses_original_zoom_model_margin_and_uniform_rate() {
        insights.openRange(400,1001)
        tryCompare(insights,"opened",true)
        compare(requests.length,1)
        compare(requests[0].start,400)
        compare(requests[0].count,1001)
        compare(requests[0].model,"ossi_candidate_defaults")
        compare(requests[0].cap,13)
        compare(findChild(insights.contentItem,"rxmerGrossRateValue").text,"250.25 Mb/s")
        const margin = findChild(insights.contentItem,"rxmerBitLoadingMargin")
        const model = findChild(insights.contentItem,"rxmerBitLoadingModel")
        const recalculate = findChild(insights.contentItem,"recalculateRxmerBitLoading")
        margin.text = "1.5"
        model.currentIndex = 0
        verify(waitForFrame(recalculate))
        mouseClick(recalculate)
        compare(requests.length,2)
        compare(requests[1].margin,1.5)
        compare(requests[1].model,"webui_mer_heuristic")
        margin.text = "-1"
        compare(recalculate.enabled,false)
        margin.text = "0.0"
        insights.working = true
        compare(recalculate.enabled,false)
        insights.working = false
        for (const preference of [1,2]) {
            appearance.preference = preference
            const chip = findChild(insights.contentItem,"rxmerGrossRateChip")
            verify(waitForFrame(chip))
            mouseClick(chip)
            const grid = findChild(insights,"rxmerUniformRatesDialog")
            tryCompare(grid,"opened",true)
            compare(findChild(grid.contentItem,"rxmerScenarioRate1").text,"325.32 Mb/s")
            keyClick(Qt.Key_Escape)
            tryCompare(grid,"visible",false)
        }
        compare(original.viewport.start,400)
        compare(original.viewport.count,1001)
    }
    function test_right_click_offers_rxmer_action_without_spectrum_actions() {
        const plot = findChild(original,"measurementPlot")
        verify(waitForFrame(plot))
        mouseClick(plot,200,150,Qt.RightButton)
        const action = findChild(original,"analyzeRxmerBitLoading")
        verify(action.visible)
        verify(!findChild(original,"calculateSpectrumPower").visible)
        verify(waitForFrame(action))
        mouseClick(action)
        compare(contextRequests,1)
        original.rxmerEnabled = false
        original.spectrumEnabled = true
        mouseClick(plot,200,150,Qt.RightButton)
        verify(!action.visible)
        verify(findChild(original,"calculateSpectrumPower").visible)
        keyClick(Qt.Key_Escape)
    }
}
