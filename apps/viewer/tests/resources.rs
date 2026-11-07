/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
use viewer::resources::examples_directory;

#[test]
fn installed_examples_follow_the_executable_on_each_platform() {
    let temporary = tempfile::tempdir().unwrap();
    for (executable, samples) in [
        ("windows/pnm-viewer.exe", "windows/files/demo"),
        ("linux/pnm-viewer-qt", "linux/files/demo"),
        (
            "mac/Viewer.app/Contents/MacOS/pnm-viewer",
            "mac/Viewer.app/Contents/Resources/files/demo",
        ),
    ] {
        let executable = temporary.path().join(executable);
        assert_eq!(examples_directory(&executable), None);
        let samples = temporary.path().join(samples);
        std::fs::create_dir_all(&samples).unwrap();
        assert_eq!(examples_directory(&executable), Some(samples));
    }
}
