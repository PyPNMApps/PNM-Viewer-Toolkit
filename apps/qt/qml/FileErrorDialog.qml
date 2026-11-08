/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls
Dialog {
    property ThemePalette themePalette: ThemePalette { targetControl: dialog; theme: dialog.theme }
    id: dialog
    required property var theme
    property string errorMessage: ""
    title: "File error"
    anchors.centerIn: parent
    width: Math.min(520, parent.width - 32)
    modal: true
    closePolicy: Popup.CloseOnEscape

    // Present each nonempty backend failure, including a repeated message.
    // Empty notifications clear backend state without opening a popup.
    function showError(message) {
        if (message.length > 0) {
            errorMessage = message
            open()
        }
    }
    background: Rectangle {
        radius: 10
        color: dialog.theme.panel
        border.color: dialog.theme.line
    }
    header: Label {
        text: dialog.title
        color: dialog.theme.text
        font.bold: true
        padding: 16
    }
    contentItem: Label {
        objectName: "fileErrorMessage"
        text: dialog.errorMessage
        color: dialog.theme.text
        wrapMode: Text.Wrap
    }
    footer: DialogButtonBox {
        ThemeButton {
            objectName: "dismissFileError"
            theme: dialog.theme
            text: "OK"
            DialogButtonBox.buttonRole: DialogButtonBox.AcceptRole
        }
        onAccepted: dialog.accept()
    }
}
