/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "helpers"
import "../qml"

RenderTestCase {
    id: testCase
    name: "TextControlContrast"
    visible: true
    when: windowShown
    width: 1100
    height: 800
    Theme { id: appearance }
    // Deliberately contradictory host colors model a light OS palette while
    // the application uses dark mode. Controls must own both contrast colors.
    Pane {
        width: 600; height: 260
        palette.base: "white"
        palette.text: "white"
        palette.button: "white"
        palette.buttonText: "white"
        Column {
            spacing: 10
            ThemeTextArea { id: report; theme: appearance; width: 400; height: 100; readOnly: true; text: "Accepted: 311\nDuplicates: 56\nSkipped: 3" }
            ThemeTextField { id: input; theme: appearance; width: 300; placeholderText: "Frequency"; text: "827.600" }
            ThemeComboBox { id: choice; theme: appearance; width: 300; model: ["Channel 193", "Channel 194"] }
        }
    }
    RxMerCollectionView { id: rxmer; theme: appearance; visible: false; width: 1000; height: 700 }
    QtObject {
        id: service
        property string catalog_json: "{}"
        property string view_json: "{}"
        property bool busy: false
        function cancel() {}
        function range_error(start, stop, unit) { return "" }
    }
    SpectrumCollectionView { id: spectrum; theme: appearance; service: service; visible: false; width: 1000; height: 700 }

    function modes() {
        return [
            {tag: "dark", preference: 1, scheme: Qt.Light},
            {tag: "light", preference: 2, scheme: Qt.Dark},
            {tag: "system-dark", preference: 0, scheme: Qt.Dark},
            {tag: "system-light", preference: 0, scheme: Qt.Light}
        ]
    }
    function setMode(data) {
        appearance.preference = data.preference
        appearance.systemScheme = data.scheme
    }
    function luminance(color) {
        const channels = [color.r, color.g, color.b].map(channel => {
            if (channel <= 0.04045) {
                return channel / 12.92
            }
            return Math.pow((channel + 0.055) / 1.055, 2.4)
        })
        return channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722
    }
    function contrast(first, second) {
        const firstLight = luminance(first)
        const secondLight = luminance(second)
        return (Math.max(firstLight, secondLight) + 0.05) / (Math.min(firstLight, secondLight) + 0.05)
    }
    function verifyTextSurface(control) {
        verify(waitForFrame(control))
        // TextArea inside ScrollView uses a viewport transform, and Popup
        // reparents into Overlay. Sample mapped window pixels for both cases.
        const root = control.Window.window.contentItem
        const image = grabImage(root)
        const origin = control.mapToItem(root, 0, 0)
        const background = image.pixel(Math.floor(origin.x + control.width - 12), Math.floor(origin.y + 3))
        verify(contrast(control.color, background) >= 4.5,
               control.objectName + " text " + control.color + " background " + background)
        let visibleTextPixels = 0
        for (let row = 8; row < Math.min(control.height - 4, 70); row++) {
            for (let column = 8; column < Math.min(control.width - 20, 220); column++) {
                if (contrast(image.pixel(Math.floor(origin.x + column), Math.floor(origin.y + row)), background) >= 4.5) {
                    visibleTextPixels++
                }
            }
        }
        verify(visibleTextPixels > 20, "Unselected text must render visibly")
    }
    function init() {
        report.enabled = true
        input.enabled = true
        choice.enabled = true
        report.deselect()
        input.deselect()
    }
    function cleanup() {
        choice.popup.close()
        findChild(rxmer, "rxmerImportDetails").close()
        findChild(spectrum, "spectrumImportDetails").close()
        rxmer.visible = false
        spectrum.visible = false
    }
    function test_controls_render_without_selection_data() { return modes() }
    function test_controls_render_without_selection(data) {
        setMode(data)
        for (const enabled of [true, false]) {
            report.enabled = enabled
            input.enabled = enabled
            choice.enabled = enabled
            verifyTextSurface(report)
            verifyTextSurface(input)
            verify(waitForFrame(choice))
            const image = grabImage(choice)
            verify(contrast(choice.contentItem.color, image.pixel(10, 3)) >= 4.5,
                   "Dropdown text must contrast with its rendered background")
        }
    }
    function test_selection_and_placeholder_colors_data() { return modes() }
    function test_selection_and_placeholder_colors(data) {
        setMode(data)
        for (const control of [report, input]) {
            control.forceActiveFocus()
            control.select(0, 8)
            verify(waitForFrame(control))
            verify(contrast(control.selectedTextColor, control.selectionColor) >= 4.5)
            compare(control.selectedTextColor, appearance.background)
            compare(control.selectionColor, appearance.accent)
        }
        input.text = ""
        verify(contrast(input.placeholderTextColor, input.background.color) >= 4.5)
        verify(waitForFrame(input))
        input.text = "827.600"
    }
    function test_dropdown_popup_palette_data() { return modes() }
    function test_dropdown_popup_palette(data) {
        setMode(data)
        choice.popup.open()
        tryCompare(choice.popup, "opened", true)
        verify(waitForFrame(choice.popup.contentItem))
        compare(choice.popup.palette.window, appearance.background)
        compare(choice.popup.palette.text, appearance.text)
        const image = grabImage(choice.popup.background)
        const background = image.pixel(image.width - 12, image.height - 4)
        verify(contrast(choice.palette.text, background) >= 4.5,
               "Popup text " + choice.palette.text + " background " + background)
    }
    function test_both_collection_reports_data() { return modes() }
    function test_both_collection_reports(data) {
        setMode(data)
        for (const entry of [
            {view: rxmer, dialog: "rxmerImportDetails", report: "rxmerImportReport"},
            {view: spectrum, dialog: "spectrumImportDetails", report: "spectrumImportReport"}
        ]) {
            entry.view.visible = true
            const catalog = {accepted_count: 311, duplicate_count: 56, skipped_count: 3,
                issues: ["README.md: Not a supported capture for this collection."]}
            if (entry.view === rxmer) {
                rxmer.catalog = catalog
            } else {
                service.catalog_json = JSON.stringify(catalog)
            }
            const dialog = findChild(entry.view, entry.dialog)
            const text = findChild(entry.view, entry.report)
            dialog.open()
            tryCompare(dialog, "opened", true)
            compare(dialog.palette.windowText, appearance.text)
            compare(text.selectionStart, text.selectionEnd)
            verifyTextSurface(text)
            dialog.close()
            entry.view.visible = false
        }
    }
}
