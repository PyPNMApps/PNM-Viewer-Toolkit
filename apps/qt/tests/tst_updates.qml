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
        property string issue_report_url: "https://github.com/PyPNMApps/PNM-Viewer-Toolkit/issues/new?title=test&body=Environment"
        property bool install_available: false
        property string install_state: "idle"
        property string install_status: ""
        property int install_progress: 0
        property int installs: 0
        property int cancels: 0
        function install_update() { installs++; install_state = "downloading" }
        function cancel_install() { cancels++ }
        property bool restartSucceeds: false
        property int restarts: 0
        function restart_updated() {
            restarts++
            if (restartSucceeds) {
                install_state = "restarting"
            }
            return restartSucceeds
        }
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
    SignalSpy { id: restartRequested; target: about; signalName: "restartRequested" }
    SignalSpy { id: issueOpened; target: about; signalName: "openIssueRequested" }
    SignalSpy { id: uninstallOpened; target: about; signalName: "openUninstallRequested" }
    SignalSpy { id: releaseOpened; target: about; signalName: "openReleaseRequested" }
    function init() {
        about.close()
        service.update_state = "idle"
        service.update_status = "Updates have not been checked."
        service.latest_version = ""
        service.requests = 0
        service.install_available = false
        service.install_state = "idle"
        service.install_status = ""
        service.installs = 0
        service.cancels = 0
        service.restarts = 0
        service.restartSucceeds = false
        about.captureBusy = false
        about.platformOs = Qt.platform.os
        uninstallOpened.clear()
        issueOpened.clear()
        restartRequested.clear()
        appearance.preference = 1
        releaseOpened.clear()
    }
    function cleanup() { about.close() }
    function test_issue_draft_action_both_themes() {
        for (const preference of [1, 2]) {
            appearance.preference = preference
            about.open()
            tryCompare(about, "opened", true)
            const button = findChild(about, "reportIssueButton")
            verify(button.visible)
            compare(button.palette.buttonText, appearance.text)
            waitForPolish(button)
            verify(waitForRendering(button))
            mouseClick(button)
            compare(issueOpened.count, preference)
            compare(issueOpened.signalArguments[preference - 1][0], service.issue_report_url)
            about.close()
        }
        compare(service.requests, 0)
        compare(service.installs, 0)
    }
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
    function test_windows_uninstall_link_is_platform_specific_data() {
        const cases=[]
        for (const platform of ["windows", "linux", "osx"]) {
            for (const preference of [1, 2]) {
                cases.push({tag:platform+"-"+preference, platform:platform, preference:preference})
            }
        }
        return cases
    }
    function test_windows_uninstall_link_is_platform_specific(data) {
        about.platformOs=data.platform
        appearance.preference=data.preference
        about.open()
        tryCompare(about,"opened",true)
        const button=findChild(about,"windowsUninstallButton")
        compare(button.visible,data.platform==="windows")
        if (data.platform==="windows") {
            verify(waitForPolish(button))
            verify(waitForRendering(button))
            compare(button.contentItem.color,appearance.text)
            mouseClick(button)
            compare(uninstallOpened.count,1)
            verify(about.opened)
        } else {
            compare(uninstallOpened.count,0)
        }
    }
    function test_install_success_automatically_requests_restart_once() {
        service.restartSucceeds = true
        service.install_state = "installing"
        service.install_state = "installed"
        tryCompare(restartRequested, "count", 1)
        compare(service.restarts, 1)
        compare(service.install_state, "restarting")
        wait(100)
        compare(service.restarts, 1)
    }
    function test_about_buttons_follow_dark_and_light_palette() {
        for (const preference of [1, 2]) {
            appearance.preference = preference
            about.open()
            tryCompare(about, "opened", true)
            for (const name of ["projectButton", "checkUpdatesButton", "releaseDownloadsButton"]) {
                const button = findChild(about, name)
                compare(button.contentItem.color, appearance.text)
                verify(waitForRendering(button))
                const image = grabImage(button)
                const background = image.pixel(Math.floor(button.width / 2), 3)
                // Check rendered background, not only palette metadata: pale
                // native backgrounds with custom pale text caused this defect.
                const brightness = (background.r + background.g + background.b) / 3
                verify(preference === 1 ? brightness < 0.4 : brightness > 0.6,
                    "Theme " + preference + " button background " + background)
            }
            about.close()
            tryCompare(about, "opened", false)
        }
    }
    function test_restart_failure_keeps_old_window_and_allows_retry() {
        about.open()
        tryCompare(about, "opened", true)
        service.install_state = "installed"
        tryCompare(service, "restarts", 1)
        compare(restartRequested.count, 0)
        verify(about.opened)
        service.restartSucceeds = true
        mouseClick(findChild(about, "restartUpdateButton"))
        tryCompare(restartRequested, "count", 1)
        compare(service.restarts, 2)
    }
    function test_restart_waits_for_capture_work_and_ignores_install_errors() {
        about.captureBusy = true
        service.restartSucceeds = true
        service.install_state = "installed"
        wait(100)
        compare(service.restarts, 0)
        about.captureBusy = false
        about.platformOs = Qt.platform.os
        uninstallOpened.clear()
        issueOpened.clear()
        tryCompare(restartRequested, "count", 1)
        compare(service.restarts, 1)
        service.install_state = "error"
        wait(100)
        compare(service.restarts, 1)
    }
    function test_installer_progress_cancel_failure_and_restart_controls() {
        about.open()
        tryCompare(about, "opened", true)
        const install = findChild(about, "installUpdateButton")
        verify(!install.visible)
        service.install_available = true
        verify(install.visible)
        mouseClick(install)
        compare(service.installs, 1)
        verify(!install.enabled)
        verify(!findChild(about, "checkUpdatesButton").enabled)
        mouseClick(findChild(about, "cancelUpdateButton"))
        compare(service.cancels, 1)
        service.install_state = "installing"
        verify(!findChild(about, "cancelUpdateButton").visible)
        service.install_state = "error"
        service.install_status = "Administrator approval canceled."
        verify(install.enabled)
        compare(findChild(about, "installStatusLabel").text, service.install_status)
        service.install_state = "installed"
        service.install_available = false
        verify(findChild(about, "restartUpdateButton").visible)
        verify(!findChild(about, "checkUpdatesButton").enabled)
    }
}
