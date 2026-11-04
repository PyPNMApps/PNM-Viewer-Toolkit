/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtTest
import "../qml"

TestCase {
    id: testCase
    name: "ApplicationUpdates"
    when: windowShown
    visible: true
    width: 700
    height: 700
    Theme { id: appearance }
    QtObject {
        id: service
        property string update_state: "idle"
        property string update_status: "Updates have not been checked."
        property string latest_version: ""
        property string release_url: "https://github.com/PyPNMApps/PNM-Viewer-Toolkit/releases"
        property int requests: 0
        function check_updates() { requests++; update_state = "checking"; update_status = "Checking for updates…" }
    }
    UpdateChip {
        id: chip
        updates: service
        onClicked: about.open()
    }
    AboutDialog {
        id: about
        parent: testCase
        theme: appearance
        updates: service
        appVersion: "0.1.5"
        iconSource: ""
    }
    SignalSpy { id: releaseOpened; target: about; signalName: "openReleaseRequested" }
    function init() {
        about.close()
        service.update_state = "idle"
        service.update_status = "Updates have not been checked."
        service.latest_version = ""
        service.requests = 0
        appearance.preference = 1
        releaseOpened.clear()
    }
    function cleanup() { about.close() }
    function test_chip_only_shows_newer_release_and_opens_about() {
        for (const state of ["idle", "checking", "current", "ahead", "no_release", "error"]) {
            service.update_state = state
            compare(chip.visible, false)
        }
        service.update_state = "available"
        service.latest_version = "0.2.0"
        verify(chip.visible)
        mouseClick(chip)
        tryCompare(about, "opened", true)
        compare(findChild(about, "latestReleaseLabel").text, "Latest release: 0.2.0")
    }
    function test_manual_check_is_disabled_until_finished_and_can_retry() {
        about.open()
        tryCompare(about, "opened", true)
        const button = findChild(about, "checkUpdatesButton")
        mouseClick(button)
        compare(service.requests, 1)
        compare(button.enabled, false)
        service.update_state = "error"
        service.update_status = "Unable to check for updates."
        compare(findChild(about, "updateStatusLabel").text, service.update_status)
        compare(button.enabled, true)
        mouseClick(button)
        compare(service.requests, 2)
        service.update_state = "no_release"
        service.update_status = "No published release is available yet."
        compare(findChild(about, "updateStatusLabel").text, service.update_status)
        compare(chip.visible, false)
    }
    function test_release_action_and_both_themes_use_shared_state() {
        service.update_state = "available"
        service.latest_version = "0.2.0"
        for (const preference of [1, 2]) {
            appearance.preference = preference
            about.open()
            tryCompare(about, "opened", true)
            const downloads = findChild(about, "releaseDownloadsButton")
            compare(downloads.text, "Release notes and downloads")
            mouseClick(downloads)
            compare(releaseOpened.count, preference)
            compare(releaseOpened.signalArguments[preference - 1][0].toString(), service.release_url)
            verify(about.width > 400)
            about.close()
            tryCompare(about, "opened", false)
        }
    }
}
