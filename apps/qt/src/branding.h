/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#pragma once
#include <QGuiApplication>
#include <QIcon>

/**
 * Update the desktop window icon from the embedded theme-specific asset.
 * Called on the Qt UI thread after application construction; no file I/O paths
 * outside the resource bundle are accepted.
 */
inline void setViewerIcon(bool dark) {
    QGuiApplication::setWindowIcon(QIcon(dark
        ? QStringLiteral(":/branding/icon-dark.png")
        : QStringLiteral(":/branding/icon-light.png")));
}
