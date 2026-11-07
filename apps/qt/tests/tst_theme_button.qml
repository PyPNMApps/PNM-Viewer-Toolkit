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
    name: "ThemeButtonContrast"
    when: windowShown
    visible: true
    width: 300
    height: 100
    Theme { id: appearance }
    ThemeButton {
        id: button
        anchors.centerIn: parent
        theme: appearance
        text: "Open capture"
    }
    // WCAG relative luminance evaluates the rendered background and actual
    // label color, rather than assuming a palette binding controls the pixels.
    function luminance(color) {
        const channels = [color.r, color.g, color.b].map(function(channel) {
            if (channel <= 0.04045) {
                return channel / 12.92
            }
            return Math.pow((channel + 0.055) / 1.055, 2.4)
        })
        return channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722
    }
    function test_rendered_contrast_data() {
        const rows = []
        for (const mode of [
            {name: "dark", preference: 1, systemScheme: Qt.Light},
            {name: "light", preference: 2, systemScheme: Qt.Dark},
            {name: "system-dark", preference: 0, systemScheme: Qt.Dark},
            {name: "system-light", preference: 0, systemScheme: Qt.Light}
        ]) {
            for (const highlighted of [false, true]) {
                for (const enabled of [false, true]) {
                    rows.push({tag: mode.name + "-highlighted-" + highlighted + "-enabled-" + enabled,
                        preference: mode.preference, systemScheme: mode.systemScheme,
                        highlighted: highlighted, enabled: enabled})
                }
            }
        }
        return rows
    }
    function test_rendered_contrast(data) {
        appearance.preference = data.preference
        appearance.systemScheme = data.systemScheme
        button.highlighted = data.highlighted
        button.enabled = data.enabled
        verify(waitForFrame(button))
        const image = grabImage(button)
        const background = image.pixel(Math.floor(button.width / 2), 3)
        const foreground = button.contentItem.color
        const backgroundLuminance = luminance(background)
        const foregroundLuminance = luminance(foreground)
        const contrast = (Math.max(backgroundLuminance, foregroundLuminance) + 0.05)
            / (Math.min(backgroundLuminance, foregroundLuminance) + 0.05)
        verify(contrast >= 4.5, "Rendered button contrast: " + contrast)
    }
}
