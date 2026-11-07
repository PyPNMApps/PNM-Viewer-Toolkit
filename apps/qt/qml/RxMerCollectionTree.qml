/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
Item {
    id: tree
    required property Theme theme
    property var catalog: ({})
    property string selectedMac: ""
    property bool working: false
    property bool playbackEnabled: false
    property int selectedGroup: -1
    property int selectedFrame: -1
    signal frameRequested(string macAddress, int groupId, int frameIndex)
    property var expandedMacs: ({})
    property var expandedGroups: ({})
    signal groupRequested(string macAddress, int groupId)
    implicitHeight: 360
    onCatalogChanged: { expandedMacs=({}); expandedGroups=({}); expandSelected() }
    onSelectedMacChanged: expandSelected()
    // Expand the selected device without allocating or sorting measurement arrays.
    function expandSelected() {
        if (selectedMac.length>0) {
            const next=Object.assign({},expandedMacs)
            next[selectedMac]=true
            expandedMacs=next
        }
    }
    function toggleMac(mac) {
        const next=Object.assign({},expandedMacs)
        next[mac]=!next[mac]
        expandedMacs=next
    }
    function toggleGroup(groupId) {
        const next=Object.assign({},expandedGroups)
        next[groupId]=!next[groupId]
        expandedGroups=next
    }
    // Flatten only expanded navigation nodes; Rust has already ordered epochs.
    // ListView instantiates visible rows, never one control per imported capture.
    function rows() {
        const result=[]
        let previousMac=""
        for (const group of (catalog.groups || [])) {
            const mac=group.mac_address
            if (mac!==previousMac) {
                result.push({kind:"mac",depth:0,label:mac,key:mac,expanded:!!expandedMacs[mac]})
                previousMac=mac
            }
            if (!expandedMacs[mac]) continue
            result.push({kind:"channel",depth:1,label:group.tree_label || group.label,key:group.id,mac:mac,expanded:!!expandedGroups[group.id],detail:group.capture_count+" captures · "+group.label})
            if (!expandedGroups[group.id]) continue
            for (let frameIndex=0;frameIndex<(group.sources || []).length;frameIndex++) {
                const source=group.sources[frameIndex]
                result.push({kind:"epoch",depth:2,label:String(source.capture_time),detail:source.name,mac:mac,key:group.id,frame:frameIndex})
            }
        }
        return result
    }
    ListView {
        id: nodes
        objectName: "rxmerCaptureTree"
        anchors.fill: parent
        clip: true
        model: tree.rows()
        ScrollBar.vertical: ScrollBar {}
        delegate: Item {
            required property var modelData
            width: nodes.width
            height: 32
            RowLayout {
                anchors.fill: parent
                anchors.leftMargin: modelData.depth*14
                spacing: 2
                ToolButton {
                    objectName: "rxmerTreeExpand_"+modelData.kind+"_"+(modelData.key===undefined ? "" : modelData.key)
                    visible: modelData.kind!=="epoch"
                    Layout.preferredWidth: 24
                    text: modelData.expanded ? "▾" : "▸"
                    Accessible.name: (modelData.expanded ? "Collapse " : "Expand ")+modelData.label
                    onClicked: {
                        if (modelData.kind==="mac") tree.toggleMac(modelData.key)
                        else tree.toggleGroup(modelData.key)
                    }
                }
                ToolButton {
                    objectName: "rxmerTreeChannel_"+modelData.key
                    visible: modelData.kind==="channel"
                    Layout.fillWidth: true
                    enabled: !tree.working
                    text: modelData.label
                    contentItem: Label { text: parent.text; color: tree.theme.text; elide: Text.ElideRight; font.pixelSize: 11; verticalAlignment: Text.AlignVCenter }
                    onClicked: tree.groupRequested(modelData.mac,modelData.key)
                    ToolTip.visible: hovered
                    ToolTip.text: modelData.detail || ""
                }
                ToolButton {
                    objectName: "rxmerTreeFrame_"+modelData.key+"_"+modelData.frame
                    visible: modelData.kind==="epoch" && tree.playbackEnabled
                    Layout.fillWidth: true
                    enabled: !tree.working
                    text: modelData.label
                    contentItem: Label {
                        text: parent.text
                        color: tree.selectedGroup===modelData.key && tree.selectedFrame===modelData.frame ? tree.theme.accent : tree.theme.muted
                        font.family: "monospace"
                        font.pixelSize: 12
                        verticalAlignment: Text.AlignVCenter
                    }
                    onClicked: tree.frameRequested(modelData.mac,modelData.key,modelData.frame)
                }
                Label {
                    visible: modelData.kind==="mac" || (modelData.kind==="epoch" && !tree.playbackEnabled)
                    Layout.fillWidth: true
                    text: modelData.label
                    color: modelData.kind==="mac" ? tree.theme.accent : tree.theme.muted
                    font.family: modelData.kind==="epoch" ? "monospace" : "Sans Serif"
                    font.pixelSize: 12
                    elide: Text.ElideRight
                    HoverHandler { id: hover }
                    ToolTip.visible: hover.hovered && modelData.kind==="epoch"
                    ToolTip.text: modelData.detail || ""
                }
            }
        }
    }
}
