/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

GridLayout {
    id: controls
    required property Theme theme
    property var metadata: ({profiles: [], selected_indices: [], metric_index: 0, carrier_types: [], selected_carrier_types: []})
    columns: 2
    columnSpacing: 8
    rowSpacing: 6
    signal profileRequested(int index)
    signal allRequested()
    signal metricRequested(int index)
    signal carrierRequested(int index)
    signal allCarriersRequested()

    Label { text: "Y axis"; color: controls.theme.muted }
    ThemeComboBox {
        theme: controls.theme
        objectName: "modulationMetricSelector"
        Accessible.name: "Modulation metric"
        model: ["dB", "Bit loading"]
        currentIndex: controls.metadata.metric_index || 0
        onActivated: index => controls.metricRequested(index)
        Layout.preferredWidth: 160
        Layout.preferredHeight: 30
    }
    Label { text: "Profiles"; color: controls.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 7 }
    Flow {
        Layout.fillWidth: true
        Layout.preferredHeight: childrenRect.height
        spacing: 6
        Repeater {
            model: controls.metadata.profiles || []
            MeasurementToggle {
                required property int index
                required property var modelData
                objectName: "modulationProfileToggle" + index
                theme: controls.theme
                traceColor: theme.profileColor(modelData.profile_id)
                text: "Profile " + modelData.profile_id
                Accessible.name: text
                checked: (controls.metadata.selected_indices || []).indexOf(index) >= 0
                onClicked: controls.profileRequested(index)
            }
        }
        MeasurementToggle {
            objectName: "modulationAllProfiles"
            theme: controls.theme
            text: "All"
            Accessible.name: "Show all modulation profiles"
            checkable: false
            showIndicator: false
            enabled: (controls.metadata.profiles || []).length > 0
            highlighted: enabled && (controls.metadata.selected_indices || []).length === controls.metadata.profiles.length
            onClicked: controls.allRequested()
        }
    }
    Label { text: "Carriers"; color: controls.theme.muted; Layout.alignment: Qt.AlignTop; topPadding: 7 }
    Flow {
        Layout.fillWidth: true
        Layout.preferredHeight: childrenRect.height
        spacing: 6
        Repeater {
            model: controls.metadata.carrier_types || []
            MeasurementToggle {
                required property int index
                required property var modelData
                objectName: "modulationCarrierToggle" + index
                theme: controls.theme
                traceColor: theme.carrierColor(modelData.kind)
                text: modelData.label
                Accessible.name: "Carrier type: " + text
                checked: (controls.metadata.selected_carrier_types || []).indexOf(modelData.kind) >= 0
                enabled: modelData.count > 0
                onClicked: controls.carrierRequested(index)
                ToolTip.visible: hovered
                ToolTip.text: modelData.count + " assignments in selected profiles"
            }
        }
        MeasurementToggle {
            objectName: "modulationAllCarriers"
            theme: controls.theme
            text: "All"
            Accessible.name: "Show all carrier types"
            checkable: false
            showIndicator: false
            highlighted: (controls.metadata.carrier_types || []).length > 0 && (controls.metadata.selected_carrier_types || []).length === controls.metadata.carrier_types.length
            onClicked: controls.allCarriersRequested()
        }
    }
}
