/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
import QtQuick
import QtQuick.Controls as Controls

Controls.ComboBox {
    id: control
    required property Theme theme
    property ThemePalette themePalette: ThemePalette { targetControl: control; theme: control.theme }
}
