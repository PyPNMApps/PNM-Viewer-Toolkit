/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
ColumnLayout {
    id: chart
    required property var theme
    property var result: ({})
    property bool delay: false
    property bool aligned: true
    property bool showOriginal: true
    property bool showPredicted: true
    signal rangeRequested(int start, int count)
    signal resetRequested()
    onResultChanged: { overlay.requestPaint(); delta.requestPaint() }
    onDelayChanged: { overlay.requestPaint(); delta.requestPaint() }
    onAlignedChanged: { overlay.requestPaint(); delta.requestPaint() }
    onShowOriginalChanged: overlay.requestPaint()
    onShowPredictedChanged: overlay.requestPaint()
    Connections { target: chart.theme; function onDarkChanged() { overlay.requestPaint(); delta.requestPaint() } }
    function paint(canvas, deltaOnly) {
        const context=canvas.getContext("2d")
        context.reset()
        const points=result.points || []
        if (!points.length) return
        const left=64, right=canvas.width-18, top=12, bottom=canvas.height-40
        if (right<=left || bottom<=top) return
        const first=points[0].frequency_hz,last=points[points.length-1].frequency_hz,span=Math.max(1,last-first)
        const originalField=delay ? "original_delay_s" : aligned ? "original_aligned_db" : "original_db"
        const predictedField=delay ? "predicted_delay_s" : aligned ? "predicted_aligned_db" : "predicted_db"
        const deltaField=delay ? "delta_delay_s" : aligned ? "aligned_delta_db" : "delta_db"
        const fields=deltaOnly ? [deltaField] : [originalField,predictedField]
        const factor=delay ? 1000000 : 1
        let minimum=Infinity,maximum=-Infinity
        for (const point of points) for (const field of fields) {
            const value=point[field]
            if (value!==null && value!==undefined && isFinite(value)) {minimum=Math.min(minimum,value*factor);maximum=Math.max(maximum,value*factor)}
        }
        if (!isFinite(minimum)) { minimum=-1;maximum=1 }
        if (deltaOnly) {minimum=Math.min(0,minimum);maximum=Math.max(0,maximum)}
        const padding=Math.max((maximum-minimum)*.1,.001)
        minimum-=padding;maximum+=padding
        function x(point) {return left+(point.frequency_hz-first)/span*(right-left)}
        function y(value) {return bottom-(value*factor-minimum)/(maximum-minimum)*(bottom-top)}
        function valid(point,field) {return point[field]!==null && point[field]!==undefined && isFinite(point[field])}
        const gaps=delay ? result.delay_gaps || [] : result.magnitude_gaps || []
        context.font="11px sans-serif";context.lineWidth=1
        for (let tick=0;tick<=4;tick++) {
            const fraction=tick/4,horizontal=left+fraction*(right-left),vertical=bottom-fraction*(bottom-top)
            context.strokeStyle=chart.theme.line;context.beginPath();context.moveTo(left,vertical);context.lineTo(right,vertical);context.moveTo(horizontal,top);context.lineTo(horizontal,bottom);context.stroke()
            context.fillStyle=chart.theme.muted;context.textAlign="right";context.fillText((minimum+fraction*(maximum-minimum)).toFixed(2),left-8,vertical+4)
            context.textAlign="center";context.fillText(((first+fraction*(last-first))/1000000).toFixed(2),horizontal,bottom+18)
        }
        if (!deltaOnly && showOriginal && showPredicted) {
            for (let index=1;index<points.length;index++) {
                const previous=points[index-1],point=points[index]
                if (gaps.indexOf(point.sample_index)>=0 || !valid(previous,originalField) || !valid(previous,predictedField) || !valid(point,originalField) || !valid(point,predictedField)) continue
                context.fillStyle=point[predictedField]>=point[originalField] ? "#d99a32" : "#199c97"
                context.globalAlpha=.35
                context.beginPath();context.moveTo(x(previous),y(previous[originalField]));context.lineTo(x(point),y(point[originalField]));context.lineTo(x(point),y(point[predictedField]));context.lineTo(x(previous),y(previous[predictedField]));context.closePath();context.fill();context.globalAlpha=1
            }
        }
        function trace(field,color) {
            context.strokeStyle=color;context.lineWidth=1.3;context.beginPath();let connected=false
            for (const point of points) {
                if (!valid(point,field)) {connected=false;continue}
                if (connected && gaps.indexOf(point.sample_index)<0) context.lineTo(x(point),y(point[field]));else context.moveTo(x(point),y(point[field]))
                connected=true
            }
            context.stroke()
            context.fillStyle=color
            for (const point of points) {
                if (valid(point,field)) context.fillRect(x(point)-1,y(point[field])-1,2,2)
            }
        }
        const amber=chart.theme.dark ? "#ffbc66" : "#ac6500"
        if (deltaOnly) {
            context.strokeStyle=chart.theme.muted;context.setLineDash([4,4]);context.beginPath();context.moveTo(left,y(0));context.lineTo(right,y(0));context.stroke();context.setLineDash([])
            trace(deltaField,amber)
        } else {
            if (showOriginal) trace(originalField,chart.theme.accent)
            if (showPredicted) trace(predictedField,amber)
        }
        context.fillStyle=chart.theme.muted;context.textAlign="center"
        context.fillText("Frequency · MHz",(left+right)/2,canvas.height-3)
        context.save();context.translate(13,(top+bottom)/2);context.rotate(-Math.PI/2)
        context.fillText(deltaOnly ? (delay ? "Delay change · µs" : aligned ? "Flatness delta · dB" : "Magnitude change · dB") : (delay ? "Coefficient group delay · µs" : aligned ? "Level-aligned magnitude · dB" : "Coefficient magnitude · dB"),0,0);context.restore()
    }
    Canvas {
        id: overlay
        objectName: "preEqComparisonOverlay"
        Accessible.role: Accessible.Graphic
        Accessible.name: "Original and predicted Pre-EQ; drag to zoom, double-click to reset"
        Layout.fillWidth: true;Layout.fillHeight: true;Layout.minimumHeight: 140
        onWidthChanged: requestPaint();onHeightChanged: requestPaint();onVisibleChanged: requestPaint()
        onPaint: chart.paint(overlay,false)
        MouseArea {
            anchors.fill: parent
            property real first: 0
            property bool selecting: false
            function fraction(horizontal) {return Math.max(0,Math.min(1,(horizontal-64)/Math.max(1,width-82)))}
            onPressed: mouse => {if(mouse.x<64 || mouse.x>width-18 || mouse.y>height-40) {mouse.accepted=false;return} first=fraction(mouse.x);selecting=true}
            onReleased: mouse => {
                if (!selecting) return
                selecting=false
                const last=fraction(mouse.x)
                if (Math.abs(last-first)*(width-82)<6) return
                const count=chart.result.count || 0,start=chart.result.start || 0
                const lower=Math.floor(Math.min(first,last)*Math.max(0,count-1)),upper=Math.ceil(Math.max(first,last)*Math.max(0,count-1))
                chart.rangeRequested(start+lower,upper-lower+1)
            }
            onCanceled: selecting=false
            onDoubleClicked: {selecting=false;chart.resetRequested()}
        }
    }
    Canvas {
        id: delta
        objectName: "preEqComparisonDelta"
        Accessible.role: Accessible.Graphic
        Accessible.name: "Predicted minus original Pre-EQ delta"
        Layout.fillWidth: true;Layout.preferredHeight: 150
        onWidthChanged: requestPaint();onHeightChanged: requestPaint();onVisibleChanged: requestPaint()
        onPaint: chart.paint(delta,true)
    }
}
