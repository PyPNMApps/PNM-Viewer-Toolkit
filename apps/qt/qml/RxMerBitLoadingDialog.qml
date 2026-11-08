/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Dialog {
    property ThemePalette themePalette: ThemePalette { targetControl: dialog; theme: dialog.theme }
    id: dialog
    objectName: "rxmerBitLoadingDialog"
    required property var theme
    property var analysis: ({})
    property bool working: false
    property string errorMessage: ""
    property int rangeStart: 0
    property int rangeCount: 0
    property int selectedCap: 13
    readonly property var summary: analysis.summary || ({})
    readonly property var selection: analysis.selection || ({})
    readonly property var coverage: analysis.coverage || ({})
    readonly property bool settingsChanged: !!analysis.model && (
        (model.currentValue || "ossi_candidate_defaults") !== analysis.model.id
        || Number(margin.text) !== analysis.model.extra_margin_db
        || selectedCap !== analysis.model.maximum_bit_loading)
    signal analysisRequested(int start, int count, string model, real marginDb, int cap)
    signal cancelRequested()
    signal exportRequested()
    parent: Overlay.overlay
    anchors.centerIn: parent
    width: Math.min(1120, parent ? parent.width - 32 : 1120)
    height: Math.min(730, parent ? parent.height - 32 : 730)
    modal: true
    focus: true
    title: "RxMER · Modulation & Capacity"
    background: Rectangle { color: dialog.theme.panel; border.color: dialog.theme.line; radius: 12 }
    onClosed: { ratesDialog.close(); cancelRequested() }
    /*
     * Opens a new immutable source-range request using the current model controls.
     * start/count are original sample indices, never display point indices.
     * Reopening keeps the user's model, margin and modulation cap.
     */
    function openRange(start, count) {
        rangeStart = start
        rangeCount = count
        open()
        requestAnalysis()
    }
    function requestAnalysis() {
        analysisRequested(rangeStart, rangeCount, model.currentValue || "ossi_candidate_defaults", Number(margin.text), selectedCap)
    }
    function rateText(value) {
        return value === null || value === undefined ? "Unavailable" : (value / 1000000).toFixed(2) + " Mb/s"
    }
    function modulationText(value) {
        return !value ? "Below model minimum" : value.indexOf("qam_") === 0 ? value.slice(4) + "-QAM" : value.toUpperCase()
    }
    function frequencyText(value) {
        return value === null || value === undefined ? "—" : (value / 1000000).toFixed(3) + " MHz"
    }
    contentItem: ColumnLayout {
        spacing: 10
        Flow {
            Layout.fillWidth: true
            spacing: 8
            SpectrumPowerChip { theme: dialog.theme; title: "Carrier centers"; value: dialog.frequencyText(dialog.selection.start_frequency_hz) + "–" + dialog.frequencyText(dialog.selection.stop_frequency_hz) }
            SpectrumPowerChip { theme: dialog.theme; title: "Measured / unavailable"; value: (dialog.coverage.measured_count || 0) + " / " + (dialog.coverage.unavailable_count || 0) }
            SpectrumPowerChip { theme: dialog.theme; title: "Minimum RxMER"; value: dialog.summary.minimum_rxmer_db === null || dialog.summary.minimum_rxmer_db === undefined ? "Unavailable" : dialog.summary.minimum_rxmer_db.toFixed(1) + " dB" }
            SpectrumPowerChip { theme: dialog.theme; title: "Uniform modulation"; value: dialog.modulationText(dialog.summary.uniform_modulation) + (dialog.summary.uniform_bit_loading ? " · " + dialog.summary.uniform_bit_loading + " bits/symbol" : "") }
        }
        RowLayout {
            Layout.fillWidth: true
            enabled: !dialog.working
            Label { text: "Model"; color: dialog.theme.muted }
            ThemeComboBox {
                theme: dialog.theme
                id: model
                objectName: "rxmerBitLoadingModel"
                Accessible.name: "RxMER modulation model"
                currentIndex: 1
                textRole: "label"; valueRole: "id"
                model: [{label: "WebUI heuristic", id: "webui_mer_heuristic"}, {label: "OSSI candidate defaults", id: "ossi_candidate_defaults"}, {label: "Ideal Shannon", id: "ideal_shannon"}]
                Layout.preferredWidth: 215
            }
            Label { text: "Margin · dB"; color: dialog.theme.muted }
            ThemeTextField {
                theme: dialog.theme
                id: margin
                objectName: "rxmerBitLoadingMargin"
                Accessible.name: "Additional RxMER margin in dB"
                text: "0.0"
                Layout.preferredWidth: 72
                validator: DoubleValidator { bottom: 0; decimals: 2; notation: DoubleValidator.StandardNotation }
            }
            Label { text: "Maximum"; color: dialog.theme.muted }
            ThemeComboBox {
                theme: dialog.theme
                id: cap
                objectName: "rxmerBitLoadingCap"
                Accessible.name: "Maximum RxMER bit loading"
                model: dialog.analysis.modulation_caps || []
                textRole: "label"; valueRole: "bits"
                currentIndex: (dialog.analysis.modulation_caps || []).findIndex(entry => entry.bits === dialog.selectedCap)
                onActivated: dialog.selectedCap = currentValue
                Layout.preferredWidth: 130
            }
            ThemeButton {
                theme: dialog.theme
                text: "Recalculate"
                objectName: "recalculateRxmerBitLoading"
                enabled: margin.acceptableInput
                borderHighlighted: dialog.settingsChanged
                ToolTip.visible: hovered && dialog.settingsChanged
                ToolTip.text: "Settings changed. Recalculate to update the result."
                onClicked: dialog.requestAnalysis()
            }
            Item { Layout.fillWidth: true }
            ThemeButton { theme: dialog.theme; text: "Export JSON"; enabled: !!dialog.analysis.summary; onClicked: dialog.exportRequested() }
        }
        RxMerBitLoadingChart { theme: dialog.theme; analysis: dialog.analysis; Layout.fillWidth: true; Layout.fillHeight: true }
        RowLayout {
            ThemeButton {
                theme: dialog.theme
                objectName: "rxmerGrossRateChip"
                Accessible.name: "Show rates by modulation"
                implicitWidth: rateContent.implicitWidth + 32
                implicitHeight: 64
                enabled: !!dialog.analysis.rate
                contentItem: Column {
                    id: rateContent
                    spacing: 4
                    Label { text: "Estimated gross throughput"; color: dialog.theme.accent; font.pixelSize: 11; font.bold: true }
                    Label { objectName: "rxmerGrossRateValue"; text: dialog.rateText((dialog.analysis.rate || {}).estimated_gross_rate_bps); color: dialog.theme.text; font.pixelSize: 25; font.bold: true }
                }
                onClicked: ratesDialog.open()
            }
            ColumnLayout {
                Label { text: "Uniform loading across measured carriers · rates by modulation ⌄"; color: dialog.theme.text }
                Label {
                    text: "Result model: " + ((dialog.analysis.model || {}).id || "—") + " · Extra margin: " + ((dialog.analysis.model || {}).extra_margin_db || 0) + " dB"
                    color: dialog.theme.muted
                    font.pixelSize: 11
                }
                Label {
                    text: "Limiting carrier: " + dialog.frequencyText(dialog.summary.limiting_frequency_hz)
                        + " · Remaining model margin: " + (dialog.summary.remaining_margin_db === null || dialog.summary.remaining_margin_db === undefined ? "—" : dialog.summary.remaining_margin_db.toFixed(2) + " dB")
                    color: dialog.theme.muted
                }
            }
            Item { Layout.fillWidth: true }
            BusyIndicator { running: dialog.working; visible: running; implicitWidth: 30; implicitHeight: 30 }
        }
        Label {
            objectName: "rxmerModulationStatus"
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            visible: !!dialog.analysis.summary && dialog.summary.status !== "meets_measured_target"
            text: dialog.summary.status === "no_measurements"
                ? "No measured RxMER values are available in this range."
                : "Analysis complete: no uniform modulation meets this model across the selected range. The weakest carrier limits the result. Try OSSI candidate defaults for lower QAM coverage, or inspect a smaller range."
            color: dialog.theme.text
            font.bold: true
        }
        Label {
            Layout.fillWidth: true
            wrapMode: Text.Wrap
            text: "Snapshot estimate; measured carriers assumed data. Gross rate excludes LDPC/FEC, cyclic prefix, pilots/PLC and MAC overhead. "
                + (dialog.coverage.unavailable_count > 0 ? "Incomplete coverage: unavailable carriers are not evaluated." : "Carrier roles are unverified.")
                + (dialog.summary.status === "below_model_minimum" ? " No covered modulation meets the selected model at this margin." : "")
            color: dialog.theme.muted
            font.pixelSize: 11
        }
        Label { Layout.fillWidth: true; wrapMode: Text.Wrap; text: dialog.errorMessage; visible: text.length > 0; color: dialog.theme.text }
        RowLayout { Item { Layout.fillWidth: true } ThemeButton { theme: dialog.theme; text: "Close"; onClicked: dialog.close() } }
    }
    Dialog {
        property ThemePalette themePalette: ThemePalette { targetControl: ratesDialog; theme: dialog.theme }
        id: ratesDialog
        objectName: "rxmerUniformRatesDialog"
        parent: Overlay.overlay
        anchors.centerIn: parent
        modal: true; focus: true
        title: "Uniform rates by modulation"
        width: Math.min(620, parent.width - 32)
        height: Math.min(490, parent.height - 32)
        background: Rectangle { color: dialog.theme.panel; border.color: dialog.theme.line; radius: 10 }
        contentItem: ColumnLayout {
            Label { text: "Each row assumes the entire measured range uses one modulation."; wrapMode: Text.Wrap; Layout.fillWidth: true; color: dialog.theme.muted }
            ScrollView {
                id: rateScroll
                Layout.fillWidth: true; Layout.fillHeight: true
                contentWidth: availableWidth; clip: true
                ColumnLayout {
                    width: rateScroll.availableWidth
                    Repeater {
                        model: dialog.analysis.uniform_scenarios || []
                        Rectangle {
                            required property var modelData
                            required property int index
                            Layout.fillWidth: true; implicitHeight: 32; radius: 4
                            color: modelData.bit_loading === dialog.summary.uniform_bit_loading ? dialog.theme.accentSoft : "transparent"
                            RowLayout {
                                anchors.fill: parent; anchors.margins: 5
                                Label { text: dialog.modulationText(modelData.modulation) + " · " + modelData.bit_loading + " bits"; color: dialog.theme.text; Layout.preferredWidth: 145 }
                                Label { objectName: "rxmerScenarioRate" + index; text: dialog.rateText(modelData.estimated_gross_rate_bps); color: dialog.theme.text; Layout.preferredWidth: 110 }
                                Label { text: modelData.status === "meets_target" ? "Meets model target" : modelData.status === "below_target" ? "Below target" : modelData.status === "no_measurements" ? "No measurements" : "Not covered by model"; color: dialog.theme.muted; Layout.fillWidth: true }
                            }
                        }
                    }
                }
            }
            Label { text: "Hypothetical gross rates before overhead; a rate alone does not imply support."; wrapMode: Text.Wrap; Layout.fillWidth: true; color: dialog.theme.muted; font.pixelSize: 11 }
            ThemeButton { theme: dialog.theme; text: "Close"; Layout.alignment: Qt.AlignRight; onClicked: ratesDialog.close() }
        }
    }
}
