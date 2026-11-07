/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
RowLayout {
    id: controls
    required property Theme theme
    property string namePrefix:"rxmer"
    property int frameIndex: 0
    property int frameCount: 0
    property bool suspended: false
    property bool playing: false
    readonly property int framesPerSecond: Number(speed.currentText) || 2
    signal frameRequested(int frameIndex)
    onSuspendedChanged: {
        if (suspended) {
            playing=false
        }
    }
    onFrameCountChanged: playing=false
    onFrameIndexChanged: {
        if (frameIndex>=frameCount-1) {
            playing=false
        }
    }
    function jump(index) {
        playing=false
        if (!suspended && index>=0 && index<frameCount) {
            frameRequested(index)
        }
    }
    Timer {
        interval: 1000/controls.framesPerSecond
        repeat: true
        running: controls.playing && !controls.suspended && controls.frameCount>1
        onTriggered: {
            if (controls.frameIndex<controls.frameCount-1) {
                controls.frameRequested(controls.frameIndex+1)
            } else {
                controls.playing=false
            }
        }
    }
    ThemeButton {
        objectName: controls.namePrefix+"PreviousFrame"
        Accessible.name: controls.namePrefix==="spectrum" ? "Spectrum Previous" : "Previous"
        theme: controls.theme
        text: "Previous"
        enabled: !controls.suspended && controls.frameIndex>0
        onClicked: controls.jump(controls.frameIndex-1)
    }
    ThemeButton {
        objectName: controls.namePrefix+"PlayPause"
        Accessible.name: controls.namePrefix==="spectrum" ? "Spectrum "+text : text
        theme: controls.theme
        text: controls.playing ? "Pause" : "Play"
        enabled: !controls.suspended && controls.frameCount>1
        borderHighlighted: controls.playing
        onClicked: {
            if (!controls.playing && controls.frameIndex===controls.frameCount-1) {
                controls.frameRequested(0)
            }
            controls.playing=!controls.playing
        }
    }
    ThemeButton {
        objectName: controls.namePrefix+"NextFrame"
        Accessible.name: controls.namePrefix==="spectrum" ? "Spectrum Next" : "Next"
        theme: controls.theme
        text: "Next"
        enabled: !controls.suspended && controls.frameIndex<controls.frameCount-1
        onClicked: controls.jump(controls.frameIndex+1)
    }
    Label { text: "Frames/s"; color: controls.theme.muted }
    ComboBox { id: speed; objectName: controls.namePrefix+"PlaybackSpeed"; Accessible.name: "Playback frames per second"; model: ["1","2","4","8"]; currentIndex: 1 }
    Label { text: controls.frameCount>0 ? (controls.frameIndex+1)+" / "+controls.frameCount : "—"; color: controls.theme.text }
}
