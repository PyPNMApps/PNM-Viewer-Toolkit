/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtTest
import "../qml"

TestCase {
    name: "RxMerPresentation"
    when: windowShown
    width: 900
    height: 450
    Theme { id: appearance }
    RxMerChart { id: chart; theme: appearance; width: 850; height: 400 }
    function init() {
        appearance.preference = 0
        appearance.systemScheme = Qt.Light
        chart.viewport = {start: 0, total: 0, points: []}
    }
    function test_system_and_explicit_themes() {
        compare(appearance.dark, false)
        appearance.systemScheme = Qt.Dark
        compare(appearance.dark, true)
        appearance.preference = 2
        compare(appearance.dark, false)
        appearance.preference = 1
        appearance.systemScheme = Qt.Light
        compare(appearance.dark, true)
        appearance.preference = 0
        compare(appearance.dark, false)
    }
    function test_missing_and_page_readings() {
        chart.viewport = {start: 1024, total: 7480, points: [
            {frequency_hz: 665600000, rxmer_db: 42.75},
            {frequency_hz: 665625000, rxmer_db: null}
        ]}
        verify(chart.reading.indexOf("#1024") >= 0)
        verify(chart.reading.indexOf("665.600 MHz") >= 0)
        verify(chart.reading.indexOf("42.75 dB") >= 0)
        chart.selectedIndex = 1
        verify(chart.reading.indexOf("Unavailable") >= 0)
        chart.viewport = {start: 0, total: 0, points: []}
        compare(chart.selectedIndex, 0)
        compare(chart.reading, "No samples in this capture")
    }
    function test_keyboard_sample_inspection() {
        chart.viewport = {start: 0, total: 2, points: [
            {frequency_hz: 640000000, rxmer_db: 42.75},
            {frequency_hz: 640025000, rxmer_db: 43}
        ]}
        const slider = findChild(chart, "sampleSlider")
        slider.forceActiveFocus()
        keyClick(Qt.Key_Right)
        compare(chart.selectedIndex, 1)
        verify(chart.reading.indexOf("43.00 dB") >= 0)
    }
}
