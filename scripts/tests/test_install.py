#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise installer control flow in an isolated checkout with fake host tools."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

PROJECT_DIR = Path(__file__).resolve().parents[2]


class InstallerTests(unittest.TestCase):
    """Prevent system changes while checking installer modes and repeat setup."""

    def setUp(self):
        """Copy installer inputs and supply a supported fake Ubuntu host."""
        self.workspace = tempfile.TemporaryDirectory()
        self.addCleanup(self.workspace.cleanup)
        self.project = Path(self.workspace.name)
        for relative in ("install.sh", "scripts/install-development.sh",
                         "rust-toolchain.toml"):
            destination = self.project / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(PROJECT_DIR / relative, destination)
        # Replace only the OS evidence source inside the disposable test copy.
        script = self.project / "scripts/install-development.sh"
        script.write_text(script.read_text().replace(
            "source /etc/os-release",
            "ID=ubuntu; VERSION_ID=22.04; PRETTY_NAME='Test Ubuntu'"))
        self.bin_dir = self.project / "fake-bin"
        self.bin_dir.mkdir()
        self.environment = dict(os.environ, PATH=f"{self.bin_dir}:{os.environ['PATH']}")
        self.command("uname", 'case "$1" in -s) echo Linux;; -m) echo x86_64;; esac')
        self.command("dpkg-query", "printf 'install ok installed'")
        for command in ("sudo", "curl"):
            self.command(command, f'echo {command} >> "{self.project}/forbidden"; exit 99')

    def command(self, name, body):
        """Create an executable fake host command for this isolated scenario."""
        target = self.bin_dir / name
        target.write_text(f"#!/bin/bash\n{body}\n")
        target.chmod(0o755)

    def run_installer(self, *arguments):
        """Run from outside the checkout and return captured output/status."""
        return subprocess.run(
            ["bash", str(self.project / "install.sh"), *arguments],
            cwd="/tmp", env=self.environment, text=True, capture_output=True)

    def test_help_and_invalid_arguments(self):
        """Help succeeds; missing mode, unknown and conflicting options fail."""
        self.assertEqual(self.run_installer("--help").returncode, 0)
        for arguments in ((), ("--unknown",), ("--check",),
                          ("--development", "--check", "--dry-run")):
            self.assertEqual(self.run_installer(*arguments).returncode, 2)

    def test_dry_run_does_not_write_or_download(self):
        """Planning succeeds despite missing tools and does not create local/."""
        result = self.run_installer("--development", "--dry-run")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse((self.project / "local").exists())
        self.assertFalse((self.project / "forbidden").exists())

    def test_check_reports_missing_tools(self):
        """Checking an incomplete checkout fails without installation."""
        result = self.run_installer("--development", "--check")
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertFalse((self.project / "local").exists())
        self.assertFalse((self.project / "forbidden").exists())

    def test_unsupported_architecture(self):
        """Reject unsupported architectures before changing the environment."""
        self.command("uname", 'case "$1" in -s) echo Linux;; -m) echo riscv64;; esac')
        self.assertEqual(self.run_installer("--development").returncode, 2)
        self.assertFalse((self.project / "forbidden").exists())

    def test_unsupported_os(self):
        """Reject other Linux distributions before package installation."""
        script = self.project / "scripts/install-development.sh"
        script.write_text(script.read_text().replace("ID=ubuntu;", "ID=fedora;"))
        self.assertEqual(self.run_installer("--development").returncode, 2)
        self.assertFalse((self.project / "forbidden").exists())

    @unittest.skipIf(os.geteuid() == 0, "Installer intentionally rejects root")
    def test_package_failure_stops_setup(self):
        """Propagate apt privilege failure before downloading local tools."""
        self.command("dpkg-query", "exit 1")
        result = self.run_installer("--development")
        self.assertEqual(result.returncode, 99, result.stderr)
        self.assertEqual((self.project / "forbidden").read_text(), "sudo\n")
        self.assertFalse((self.project / "local").exists())

    @unittest.skipIf(os.geteuid() == 0, "Installer intentionally rejects root")
    def test_repeat_install_reuses_ready_tools(self):
        """Repeated setup avoids apt and downloads when prerequisites exist."""
        target = self.project / "local/cargo/bin/rustup"
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_text("#!/bin/bash\nexit 0\n")
        target.chmod(0o755)
        for attempt in range(2):
            result = self.run_installer("--development")
            self.assertEqual(result.returncode, 0, (attempt, result.stderr))
        self.assertFalse((self.project / "forbidden").exists())
        self.assertEqual(list((self.project / "local").glob("development-install.*")), [])


if __name__ == "__main__":
    unittest.main()
