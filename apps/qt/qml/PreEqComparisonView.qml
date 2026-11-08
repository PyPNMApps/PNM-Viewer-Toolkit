/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ColumnLayout {
    id: view
    required property var theme
    property var result: ({})
    readonly property var summary: result.summary || ({})
    signal rangeRequested(int start, int count)
    signal resetRequested()
    spacing: 10
    function number(value) {return value===null || value===undefined ? "—" : value.toFixed(2)}
    Label { text: "Pre-EQ · Predicted correction & delta"; color: view.theme.text; font.pixelSize: 20; font.bold: true }
    Label { visible: !view.result.pair; text: "Open an original capture, attach its Last Update, then choose Adjust or Set and Compare."; Layout.fillWidth: true; wrapMode: Text.Wrap; color: view.theme.muted }
    Flow {
        visible: !!view.result.pair
        Layout.fillWidth: true
        spacing: 8
        SpectrumPowerChip { theme: view.theme; title: "Original full span"; value: view.number(view.summary.original_magnitude_span_db)+" dB p-p" }
        SpectrumPowerChip { theme: view.theme; title: "Predicted full span"; value: view.number(view.summary.predicted_magnitude_span_db)+" dB p-p" }
        SpectrumPowerChip { theme: view.theme; title: "Aligned delta"; value: view.number(view.summary.aligned_delta_min_db)+" to "+view.number(view.summary.aligned_delta_max_db)+" dB" }
        SpectrumPowerChip { theme: view.theme; title: "Mode"; value: view.result.mode || "—" }
    }
    Flow {
        Layout.fillWidth: true
        spacing: 8
        visible: !!view.result.pair
        ThemeComboBox { theme: view.theme; id: metric; objectName: "preEqComparisonMetric"; Accessible.name: "Pre-EQ comparison metric"; model:["Magnitude","Group delay","I/Q scatter"]; implicitWidth: 150 }
        ThemeComboBox { theme: view.theme; id: alignment; objectName: "preEqAlignment"; Accessible.name: "Pre-EQ magnitude alignment"; visible: metric.currentIndex===0; model:["Level-aligned flatness","Raw coefficient magnitude"]; implicitWidth: 220 }
        MeasurementToggle { id: original; theme: view.theme; text: "Original"; checked: true; visible: metric.currentIndex!==2 }
        MeasurementToggle { id: predicted; theme: view.theme; text: "Predicted"; checked: true; traceColor: view.theme.dark ? "#ffbc66" : "#ac6500"; visible: metric.currentIndex!==2 }
        ThemeComboBox { theme: view.theme; id: iqSet; objectName: "preEqIqDataset"; visible: metric.currentIndex===2; model:["Original","Predicted","Delta"]; currentIndex: 2 }
    }
    PreEqComparisonChart {
        visible: !!view.result.pair && metric.currentIndex!==2
        Layout.fillWidth: true;Layout.fillHeight: true
        theme: view.theme;result: view.result;delay: metric.currentIndex===1;aligned: alignment.currentIndex===0
        showOriginal: original.checked;showPredicted: predicted.checked
        onRangeRequested: (start,count)=>view.rangeRequested(start,count)
        onResetRequested: view.resetRequested()
    }
    IqScatterChart {
        visible: !!view.result.pair && metric.currentIndex===2
        Layout.fillWidth: true;Layout.fillHeight: true
        theme: view.theme
        density: iqSet.currentIndex===0 ? view.result.original_iq || ({}) : iqSet.currentIndex===1 ? view.result.predicted_iq || ({}) : view.result.delta_iq || ({})
    }
    Item { visible: !view.result.pair; Layout.fillHeight: true }
}
