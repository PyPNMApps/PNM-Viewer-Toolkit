/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs
import QtCore
import Pnm.Viewer

ApplicationWindow {
    id: window
    width: 1280
    height: 820
    minimumWidth: 860
    minimumHeight: 680
    visible: true
    title: "PNM Viewer Toolkit"
    color: appTheme.background
    font.family: "Sans Serif"
    font.pixelSize: 13
    palette.window: appTheme.background
    palette.windowText: appTheme.text
    palette.base: appTheme.panel
    palette.alternateBase: appTheme.button
    palette.text: appTheme.text
    palette.button: appTheme.button
    palette.buttonText: appTheme.text
    palette.highlight: appTheme.accent
    palette.highlightedText: appTheme.background
    Theme { id: appTheme }
    ViewerBackend { id: backend }
    UpdateBackend { id: updateBackend }
    SmokeTest {
        viewerBackend: backend
        applicationWindow: window
        surface: captureSurface
        openCaptureDialog: openDialog
        exportCaptureDialog: saveDialog
    }
    Component.onCompleted: {
        backend.set_dark_icon(appTheme.dark)
        // Packaged smoke checks stay offline; normal launch checks once, asynchronously.
        if (Qt.application.arguments.indexOf("--smoke-test") < 0) {
            Qt.callLater(function() { updateBackend.check_updates() })
        }
    }
    Connections {
        target: appTheme
        function onDarkChanged() { backend.set_dark_icon(appTheme.dark) }
    }
    readonly property var metadata: JSON.parse(backend.metadata_json)
    readonly property var viewport: JSON.parse(backend.viewport_json)
    property var spectrumPresentations: ({})
    function saveSpectrumPresentation() {
        spectrumPresentations[spectrumTools.workspace.active || 0] = spectrumControls.presentation()
    }
    function restoreSpectrumPresentation() {
        spectrumControls.restorePresentation(spectrumPresentations[spectrumTools.workspace.active || 0])
    }
    Connections {
        target: backend
        function onRxmer_collection_errorChanged() { fileError.showError(backend.rxmer_collection_error) }
        function onError_messageChanged() { fileError.showError(backend.error_message) }
        function onPre_eq_errorChanged() { fileError.showError(backend.pre_eq_error) }
        function onPre_eq_currentChanged() { if (!backend.pre_eq_current) window.comparisonSelected=false }
        function onPre_eq_jsonChanged() { if (!JSON.parse(backend.pre_eq_json).pair) window.comparisonSelected=false }
        function onMetadata_jsonChanged() {
            window.rxmerCollectionSelected = false
            window.spectrumPresentations = ({})
            spectrumControls.restorePresentation(null)
            bitLoadingDialog.close()
        }
    }
    property bool comparisonSelected: false
    property bool rxmerCollectionSelected: false
    property bool rxmerPlaybackSelected: false
    property bool averageAnalysisSelected: false
    readonly property var preEqAttachment: JSON.parse(backend.pre_eq_attachment_json)
    readonly property var preEqResult: JSON.parse(backend.pre_eq_json)
    property bool navigationVisible: false
    readonly property bool blocked: backend.busy || backend.pre_eq_busy || backend.rxmer_collection_busy || rxmerCollection.popupVisible || collectionArchiveDialog.visible || collectionFolderDialog.visible || collectionSaveDialog.visible || fileError.visible || preEqOpenDialog.visible || preEqSaveDialog.visible || openDialog.visible || saveDialog.visible || systemDialog.visible || aboutDialog.visible || profileBandwidth.popupVisible || bitLoadingDialog.visible || analysisSaveDialog.visible

    CaptureDropArea {
        parent: Overlay.overlay
        anchors.fill: parent
        z: 1
        theme: appTheme
        blocked: window.blocked || settingsMenu.visible || spectrumTools.popupVisible
        attachmentTarget: backend.pre_eq_current && window.navigationVisible && window.comparisonSelected ? preEqSidebar.dropTarget : null
        onAttachmentDropped: captureUrl => { window.comparisonSelected=true; backend.attach_pre_eq_update(captureUrl) }
        onFileDropped: captureUrl => backend.open_file(captureUrl)
    }

    FileErrorDialog { id: fileError; theme: appTheme }

    RxMerBitLoadingDialog {
        id: bitLoadingDialog
        theme: appTheme
        analysis: JSON.parse(backend.rxmer_analysis_json)
        working: backend.rxmer_analysis_busy
        errorMessage: backend.rxmer_analysis_error
        onAnalysisRequested: (start, count, model, margin, cap) => {
            if (window.averageAnalysisSelected) backend.analyze_rxmer_average(start, count, model, margin, cap)
            else backend.analyze_rxmer(start, count, model, margin, cap)
        }
        onCancelRequested: backend.cancel_rxmer_analysis()
        onExportRequested: analysisSaveDialog.open()
    }
    FolderDialog {
        id: collectionFolderDialog
        title: "Choose RxMER capture folder"
        onAccepted: backend.import_rxmer_collection(selectedFolder)
    }
    FileDialog {
        id: collectionArchiveDialog
        title: "Open RxMER capture archive"
        nameFilters: ["Capture archives (*.tar *.zip *.tar.gz *.tgz)", "All files (*)"]
        onAccepted: backend.import_rxmer_collection(selectedFile)
    }
    FileDialog {
        id: collectionSaveDialog
        title: "Export full-resolution RxMER Min-Avg-Max"
        fileMode: FileDialog.SaveFile
        nameFilters: ["JSON files (*.json)"]
        defaultSuffix: "json"
        onAccepted: backend.export_rxmer_collection(selectedFile)
    }
    FileDialog {
        id: preEqOpenDialog
        title: "Attach Pre-EQ Last Update"
        fileMode: FileDialog.OpenFile
        nameFilters: ["PNM captures (*)"]
        currentFolder: openDialog.currentFolder
        onAccepted: { window.comparisonSelected=true; backend.attach_pre_eq_update(selectedFile) }
    }
    FileDialog {
        id: preEqSaveDialog
        title: "Export full-resolution Pre-EQ comparison (new file)"
        fileMode: FileDialog.SaveFile
        nameFilters: ["JSON files (*.json)"]
        defaultSuffix: "json"
        onAccepted: backend.export_pre_eq(selectedFile)
    }
    FileDialog {
        id: analysisSaveDialog
        title: "Export full-resolution RxMER analysis (new file)"
        fileMode: FileDialog.SaveFile
        nameFilters: ["JSON files (*.json)"]
        defaultSuffix: "json"
        onAccepted: backend.export_rxmer_analysis(selectedFile)
    }
    FileDialog {
        id: openDialog
        title: "Open PNM capture"
        fileMode: FileDialog.OpenFile
        nameFilters: ["PNM captures (*)"]
        currentFolder: {
            const examples = backend.examples_folder()
            if (examples.toString().length > 0) {
                return examples
            }
            return StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]
        }
        onAccepted: backend.open_file(selectedFile)
    }
    FileDialog {
        id: saveDialog
        title: "Export full-resolution JSON (new file)"
        fileMode: FileDialog.SaveFile
        nameFilters: ["JSON files (*.json)"]
        defaultSuffix: "json"
        onAccepted: backend.export_json(selectedFile)
    }
    Shortcut { sequences: [StandardKey.Open]; enabled: !window.blocked; onActivated: openDialog.open() }
    Shortcut { sequences: [StandardKey.Close]; enabled: !window.blocked; onActivated: backend.close_capture() }


    header: ToolBar {
        height: 38
        background: Rectangle { color: appTheme.sidebar }
        RowLayout {
            anchors.fill: parent
            anchors.leftMargin: 8
            anchors.rightMargin: 8
            spacing: 8
            ToolButton { text: "Open"; Accessible.name: "Open PNM file"; enabled: !window.blocked; onClicked: openDialog.open() }
            ToolButton {
                text: "Settings"
                enabled: !window.blocked
                onClicked: settingsMenu.open()
                Menu { id: settingsMenu; y: parent.height; MenuItem { text: "System…"; onTriggered: systemDialog.open() } }
            }
            ToolButton { text: "About"; enabled: !window.blocked; onClicked: aboutDialog.open() }
            Item { Layout.fillWidth: true }
            BusyIndicator { running: backend.busy; visible: running; Layout.preferredWidth: 24; Layout.preferredHeight: 24; Accessible.name: "Processing capture" }
            UpdateChip {
                updates: updateBackend
                enabled: !window.blocked
                onClicked: aboutDialog.open()
            }
            PanelToggle {
                theme: appTheme
                checked: window.navigationVisible
                Accessible.name: checked ? "Hide navigation" : "Show navigation"
                onClicked: window.navigationVisible = !window.navigationVisible
            }
        }
    }
    Dialog {
        id: systemDialog
        title: "System settings"
        anchors.centerIn: parent
        width: 360
        modal: true
        standardButtons: Dialog.Close
        ColumnLayout {
            anchors.fill: parent
            spacing: 16
            Label { text: "Appearance"; color: appTheme.text }
            ComboBox {
                objectName: "themeSelector"
                Accessible.name: "Appearance"
                model: ["System", "Dark", "Light"]
                currentIndex: appTheme.preference
                Layout.fillWidth: true
                onActivated: appTheme.preference = currentIndex
            }
            Label { text: "System follows your desktop appearance.\nPreferences apply to this session."; color: appTheme.muted }
        }
    }
    AboutDialog {
        id: aboutDialog
        anchors.centerIn: parent
        theme: appTheme
        updates: updateBackend
        appVersion: backend.app_version
        captureBusy: backend.busy
        onRestartRequested: Qt.quit()
        onOpenReleaseRequested: address => Qt.openUrlExternally(address)
    }
    RowLayout {
        id: captureSurface
        anchors.fill: parent
        spacing: 0
        Rectangle {
            visible: window.navigationVisible
            Layout.preferredWidth: window.rxmerCollectionSelected ? 340 : backend.pre_eq_current ? 244 : 180
            Layout.fillHeight: true
            color: appTheme.sidebar
            ScrollView {
                id: navigationScroll
                anchors.fill: parent
                anchors.margins: 12
                contentWidth: availableWidth
                clip: true
                ColumnLayout {
                width: navigationScroll.availableWidth
                spacing: 16
                Image {
                    id: brandLogo
                    Layout.topMargin: 8
                    Layout.fillWidth: true
                    Layout.preferredHeight: 100
                    Layout.preferredWidth: 148
                    source: appTheme.dark ? "qrc:/branding/logo-dark.png" : "qrc:/branding/logo-light.png"
                    fillMode: Image.PreserveAspectFit
                    mipmap: true
                    Accessible.role: Accessible.Graphic
                    Accessible.name: "PyPNMApps PNM Viewer Toolkit"
                    // Blend only the artwork's outer padding into the live theme.
                    // Keep the original README image and logo lettering intact.
                    readonly property color clearSidebar: Qt.rgba(appTheme.sidebar.r, appTheme.sidebar.g, appTheme.sidebar.b, 0)
                    Rectangle {
                        anchors { left: parent.left; right: parent.right; top: parent.top }
                        height: parent.height * 0.20
                        gradient: Gradient {
                            GradientStop { position: 0; color: appTheme.sidebar }
                            GradientStop { position: 1; color: brandLogo.clearSidebar }
                        }
                    }
                    Rectangle {
                        anchors { left: parent.left; right: parent.right; bottom: parent.bottom }
                        height: parent.height * 0.20
                        gradient: Gradient {
                            GradientStop { position: 0; color: brandLogo.clearSidebar }
                            GradientStop { position: 1; color: appTheme.sidebar }
                        }
                    }
                    Rectangle {
                        anchors { left: parent.left; top: parent.top; bottom: parent.bottom }
                        width: parent.width * 0.05
                        gradient: Gradient {
                            orientation: Gradient.Horizontal
                            GradientStop { position: 0; color: appTheme.sidebar }
                            GradientStop { position: 1; color: brandLogo.clearSidebar }
                        }
                    }
                    Rectangle {
                        anchors { right: parent.right; top: parent.top; bottom: parent.bottom }
                        width: parent.width * 0.05
                        gradient: Gradient {
                            orientation: Gradient.Horizontal
                            GradientStop { position: 0; color: brandLogo.clearSidebar }
                            GradientStop { position: 1; color: appTheme.sidebar }
                        }
                    }
                }

                Label { text: "WORKSPACE"; color: appTheme.muted; font.pixelSize: 10 }
                Label { text: backend.loaded ? (window.metadata.title || "Measurement") : "Home"; color: appTheme.accent; Layout.fillWidth: true; wrapMode: Text.Wrap }
                ThemeButton {
                    theme: appTheme
                    text: "Original capture"
                    visible: window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    onClicked: window.rxmerCollectionSelected=false
                }
                ToolButton {
                    id: rxmerToolGroup
                    objectName: "toggleRxmerTools"
                    text: (checked ? "▾  " : "▸  ")+"RxMER"
                    Accessible.name: "RxMER tools"
                    checkable: true
                    checked: true
                    Layout.fillWidth: true
                    contentItem: Label {
                        text: rxmerToolGroup.text
                        color: appTheme.text
                        font.bold: true
                        verticalAlignment: Text.AlignVCenter
                    }
                }
                ColumnLayout {
                    visible: rxmerToolGroup.checked
                    Layout.fillWidth: true
                    Layout.leftMargin: 14
                    spacing: 8
                    ThemeButton {
                        objectName: "showRxmerMinAvgMax"
                        theme: appTheme
                        text: "Min-Avg-Max"
                        enabled: !backend.rxmer_collection_busy
                        Layout.fillWidth: true
                        borderHighlighted: window.rxmerCollectionSelected && !window.rxmerPlaybackSelected
                        onClicked: { window.rxmerPlaybackSelected=false; window.rxmerCollectionSelected=true; window.comparisonSelected=false }
                    }
                    ThemeButton {
                        objectName: "showRxmerPlayback"
                        theme: appTheme
                        text: "Playback"
                        enabled: !backend.rxmer_collection_busy
                        Layout.fillWidth: true
                        borderHighlighted: window.rxmerCollectionSelected && window.rxmerPlaybackSelected
                        onClicked: { window.rxmerPlaybackSelected=true; window.rxmerCollectionSelected=true; window.comparisonSelected=false }
                    }
                    RxMerCollectionTree {
                        visible: window.rxmerCollectionSelected && (catalog.groups || []).length>0
                        Layout.fillWidth: true
                        Layout.preferredHeight: 360
                        theme: appTheme
                        catalog: JSON.parse(backend.rxmer_collection_catalog)
                        selectedMac: rxmerCollection.selectedMac
                        working: backend.rxmer_collection_busy
                        playbackEnabled: window.rxmerPlaybackSelected
                        selectedGroup: rxmerCollection.selectedGroup
                        selectedFrame: rxmerCollection.viewport.frame_index===undefined ? -1 : rxmerCollection.viewport.frame_index
                        onFrameRequested: (macAddress,groupId,frameIndex) => {
                            rxmerCollection.pausePlayback()
                            if (rxmerCollection.selectedGroup!==groupId) {
                                rxmerCollection.selectGroup(macAddress,groupId)
                            }
                            rxmerCollection.showFrame(frameIndex)
                        }
                        onGroupRequested: (macAddress,groupId) => rxmerCollection.selectGroup(macAddress,groupId)
                    }
                }
                PreEqComparisonSidebar {
                    id: preEqSidebar
                    visible: backend.pre_eq_current && !window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    theme: appTheme
                    comparisonSelected: window.comparisonSelected
                    attachment: window.preEqAttachment
                    resultMode: window.preEqResult.mode || ""
                    working: backend.pre_eq_busy
                    onChooseFile: preEqOpenDialog.open()
                    onCalculate: mode => { window.comparisonSelected=true; backend.compare_pre_eq(mode) }
                    onRemoveAttachment: backend.remove_pre_eq_update()
                    onShowComparison: window.comparisonSelected=true
                    onShowOriginal: window.comparisonSelected=false
                    onExportResult: preEqSaveDialog.open()
                }
                Item { Layout.fillHeight: true }
                }
            }
        }
        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 4
            spacing: 10
            Label {
                visible: backend.error_message.length > 0
                text: backend.error_message
                color: appTheme.dark ? "#ffb4ab" : "#a92b26"
                wrapMode: Text.Wrap
                Layout.fillWidth: true
                Accessible.role: Accessible.AlertMessage
            }
                Rectangle {
                    visible: !backend.loaded && !window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    color: appTheme.panel
                    border.color: appTheme.line
                    radius: 14
                    ColumnLayout {
                        anchors.centerIn: parent
                        spacing: 20
                        Label { text: "YOUR CAPTURE, ON YOUR DESKTOP"; color: appTheme.accent; font.pixelSize: 10; font.letterSpacing: 1.6; Layout.alignment: Qt.AlignHCenter }
                        Label { text: "Explore a PNM capture"; color: appTheme.text; font.pixelSize: 26; font.bold: true; Layout.alignment: Qt.AlignHCenter }
                        ThemeButton {
                            theme: appTheme
                            objectName: "openCaptureButton"
                            text: "Open capture"
                            enabled: !window.blocked
                            highlighted: true
                            Layout.alignment: Qt.AlignHCenter
                            onClicked: openDialog.open()
                        }
                    }
                }
                RxMerCollectionView {
                    id: rxmerCollection
                    visible: window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    theme: appTheme
                    catalog: JSON.parse(backend.rxmer_collection_catalog)
                    viewport: JSON.parse(backend.rxmer_collection_view)
                    working: backend.rxmer_collection_busy
                    playbackMode: window.rxmerPlaybackSelected
                    playbackBlocked: window.blocked || !window.active
                    onFrameRequested: (groupId,frameIndex,start,count) => backend.show_rxmer_frame(groupId,frameIndex,start,count)
                    onChooseDirectory: collectionFolderDialog.open()
                    onChooseArchive: collectionArchiveDialog.open()
                    onCancelImport: backend.cancel_rxmer_collection()
                    onGroupRequested: groupId => backend.select_rxmer_group(groupId)
                    onRangeRequested: (start,count) => backend.show_rxmer_collection_range(start,count)
                    onBitLoadingRequested: (start,count) => { window.averageAnalysisSelected=true; bitLoadingDialog.openRange(start,count) }
                    onExportRequested: collectionSaveDialog.open()
                }
                PreEqComparisonView {
                    visible: backend.loaded && window.comparisonSelected && backend.pre_eq_current && !window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    theme: appTheme
                    result: window.preEqResult
                    onRangeRequested: (start,count)=>backend.show_pre_eq_range(start,count)
                    onResetRequested: backend.show_pre_eq_range(0,window.preEqResult.pair ? window.preEqResult.pair.sample_count : 0)
                }
                Rectangle {
                    visible: backend.loaded && !window.comparisonSelected && !window.rxmerCollectionSelected
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.minimumHeight: 300
                    color: appTheme.panel
                    border.color: appTheme.line
                    radius: 12
                    ColumnLayout {
                        anchors.fill: parent
                        anchors.margins: 10
                        spacing: 8
                        RowLayout {
                            Layout.fillWidth: true
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 6
                                Label { text: (window.metadata.title || "Measurement"); elide: Text.ElideMiddle; Layout.fillWidth: true; color: appTheme.text; font.pixelSize: 17; font.bold: true }
                                Flow {
                                    Layout.fillWidth: true
                                    spacing: 8
                                    Repeater {
                                        model: window.metadata.chips || []
                                        Rectangle {
                                            required property var modelData
                                            implicitWidth: chipContent.implicitWidth + 24
                                            implicitHeight: 30
                                            radius: height / 2
                                            color: appTheme.dark ? "#0affffff" : "#06000000"
                                            border.color: appTheme.line
                                            Row {
                                                id: chipContent
                                                anchors.centerIn: parent
                                                spacing: 8
                                                Label { text: modelData.title; color: appTheme.accent; font.pixelSize: 12; font.bold: true }
                                                Label { text: modelData.value === undefined ? "" : modelData.value.toString(); color: appTheme.text; font.pixelSize: 12 }
                                            }
                                        }
                                    }
                                    Repeater {
                                        model: spectrumTools.powerChips
                                        SpectrumPowerChip {
                                            required property var modelData
                                            theme: appTheme
                                            title: modelData.title
                                            value: modelData.value
                                        }
                                    }
                                    ProfileBandwidth {
                                        id: profileBandwidth
                                        visible: window.metadata.modulation_profile !== undefined
                                        theme: appTheme
                                        metadata: window.metadata.modulation_profile || ({profiles: []})
                                        enabled: !backend.busy
                                    }
                                }
                            }
                            ToolButton { text: "⇩"; font.pixelSize: 22; Layout.preferredWidth: 32; Layout.preferredHeight: 32; Accessible.name: "Export JSON"; ToolTip.visible: hovered; ToolTip.text: "Export JSON"; enabled: !window.blocked; onClicked: saveDialog.open() }
                            ToolButton { text: "×"; font.pixelSize: 22; Layout.preferredWidth: 32; Layout.preferredHeight: 32; Accessible.name: "Close capture"; ToolTip.visible: hovered; ToolTip.text: "Close capture"; enabled: !window.blocked; onClicked: backend.close_capture() }
                        }
                        RowLayout {
                            visible: (window.metadata.views || []).length > 0
                            spacing: 6
                            Repeater {
                                model: window.metadata.views || []
                                ToolButton {
                                    required property string modelData
                                    required property int index
                                    text: modelData
                                    Accessible.name: modelData
                                    readonly property bool selected: window.metadata.view_index === index
                                    Accessible.description: selected ? "Selected view" : "Show " + modelData
                                    enabled: !window.blocked
                                    onClicked: backend.select_view(index)
                                    background: Rectangle {
                                        radius: 6
                                        color: parent.selected ? appTheme.button : "transparent"
                                        border.color: parent.selected ? appTheme.accent : appTheme.line
                                    }
                                }
                            }
                        }
                        ModulationControls {
                            visible: window.metadata.modulation_profile !== undefined
                            theme: appTheme
                            metadata: window.metadata.modulation_profile || ({profiles: []})
                            Layout.fillWidth: true
                            onProfileRequested: index => backend.toggle_modulation_profile(index)
                            onMetricRequested: index => backend.select_modulation_metric(index)
                            onAllRequested: backend.select_all_modulation_profiles()
                            onCarrierRequested: index => backend.toggle_modulation_carrier(index)
                            onAllCarriersRequested: backend.select_all_modulation_carriers()
                        }
                        SpectrumTools {
                            id: spectrumTools
                            visible: window.metadata.spectrum_analysis !== undefined
                            enabled: !window.blocked
                            theme: appTheme
                            workspace: JSON.parse(backend.spectrum_workspace_json)
                            validateRange: (start, stop, unit) => backend.spectrum_range_error(start, stop, unit)
                            Layout.fillWidth: true
                            onRangeRequested: (start, stop, unit) => {
                                window.saveSpectrumPresentation()
                                backend.open_spectrum_band(start, stop, unit)
                                window.restoreSpectrumPresentation()
                            }
                            onTabRequested: index => {
                                window.saveSpectrumPresentation()
                                backend.select_spectrum_tab(index)
                                window.restoreSpectrumPresentation()
                            }
                            onCloseRequested: index => {
                                window.saveSpectrumPresentation()
                                backend.close_spectrum_tab(index)
                                const states = window.spectrumPresentations
                                for (let tab = index; tab < (spectrumTools.workspace.tabs || []).length; tab++) {
                                    states[tab] = states[tab + 1]
                                }
                                delete states[(spectrumTools.workspace.tabs || []).length]
                                window.restoreSpectrumPresentation()
                            }
                        }
                        SpectrumControls {
                            id: spectrumControls
                            visible: window.metadata.spectrum_analysis !== undefined
                            enabled: !window.blocked
                            theme: appTheme
                            sampleCount: window.metadata.sample_count || 0
                            averageWindow: window.viewport.spectrum_average ? window.viewport.spectrum_average.window_points : 0
                            Layout.fillWidth: true
                            onAverageRequested: windowPoints => backend.apply_spectrum_average(windowPoints)
                        }
                        ConstellationChart {
                            visible: window.viewport.constellation !== undefined
                            theme: appTheme
                            constellation: window.viewport.constellation || ({})
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            onBoundsRequested: (firstI, lastI, firstQ, lastQ) => backend.zoom_constellation(firstI, lastI, firstQ, lastQ)
                            onResetRequested: backend.select_view(0)
                        }
                        IqScatterChart {
                            visible: window.metadata.view_index === 2
                            theme: appTheme
                            density: window.viewport.iq_density || ({})
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                        }
                        HistogramChart {
                            visible: window.metadata.histogram === true
                            theme: appTheme
                            viewport: window.viewport
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            onRangeRequested: (start, count) => backend.show_range(start, count)
                        }
                        FecSummaryChart {
                            visible: window.metadata.fec_summary !== undefined
                            theme: appTheme
                            metadata: window.metadata.fec_summary || ({profiles: [], profile_index: 0})
                            viewport: window.viewport
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            onProfileRequested: index => backend.select_fec_profile(index)
                            onTimeRangeRequested: (first, last) => backend.show_fec_time_range(first, last)
                            onRangeRequested: (start, count) => backend.show_range(start, count)
                        }
                        MeasurementChart {
                            spectrumEnabled: window.metadata.spectrum_analysis !== undefined
                            onFrequencyRequested: spectrumTools.openFrequency()
                            onPowerRequested: (first, last) => backend.calculate_spectrum_power(first, last)
                            showSpectrumAverage: spectrumControls.showAverage
                            visible: window.viewport.constellation === undefined && window.metadata.view_index !== 2 && window.metadata.histogram !== true && window.metadata.fec_summary === undefined
                            rxmerEnabled: window.metadata.title === "Downstream RxMER"
                            onBitLoadingRequested: { window.averageAnalysisSelected=false; bitLoadingDialog.openRange(window.viewport.start || 0, window.viewport.count || 0) }
                            stepPlot: window.metadata.modulation_profile !== undefined
                            unit: window.metadata.unit || "dB"
                            axisLabel: window.metadata.axis_label || "Amplitude"
                            minimum: window.metadata.minimum === undefined ? 0 : window.metadata.minimum
                            maximum: window.metadata.maximum === undefined ? 64 : window.metadata.maximum
                            theme: appTheme
                            viewport: window.viewport
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            onRangeRequested: (start, count) => backend.show_range(start, count)
                        }
                    }
                }

        }
    }
}
