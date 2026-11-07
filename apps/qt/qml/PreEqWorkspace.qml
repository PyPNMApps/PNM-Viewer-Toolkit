/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ColumnLayout {
    id:workspace
    required property Theme theme
    required property var service
    readonly property var attachment:JSON.parse(service.pre_eq_attachment_json)
    readonly property var result:JSON.parse(service.pre_eq_json)
    property alias updateTarget:controls.dropTarget
    signal chooseOriginal()
    signal chooseUpdate()
    signal exportRequested()
    spacing:10
    RowLayout {
        Label {text:"US Pre-EQ · Compare";color:workspace.theme.text;font.pixelSize:20;font.bold:true}
        Item {Layout.fillWidth:true}
        ThemeButton {objectName:"choosePreEqOriginal";theme:workspace.theme;text:"Open original…";enabled:!workspace.service.busy && !workspace.service.pre_eq_busy;onClicked:workspace.chooseOriginal()}
    }
    RowLayout {
        Layout.fillWidth:true;Layout.fillHeight:true
        PreEqComparisonSidebar {
            id:controls
            Layout.preferredWidth:220;Layout.maximumWidth:220;Layout.alignment:Qt.AlignTop
            theme:workspace.theme;comparisonSelected:true;standalone:true
            attachment:workspace.attachment;resultMode:workspace.result.mode || ""
            working:workspace.service.busy || workspace.service.pre_eq_busy
            enabled:workspace.service.pre_eq_current
            onChooseFile:workspace.chooseUpdate()
            onCalculate:mode=>workspace.service.compare_pre_eq(mode)
            onRemoveAttachment:workspace.service.remove_pre_eq_update()
            onExportResult:workspace.exportRequested()
        }
        PreEqComparisonView {
            Layout.fillWidth:true;Layout.fillHeight:true
            theme:workspace.theme;result:workspace.result
            onRangeRequested:(start,count)=>workspace.service.show_pre_eq_range(start,count)
            onResetRequested:workspace.service.show_pre_eq_range(0,workspace.result.pair ? workspace.result.pair.sample_count : 0)
        }
    }
}
