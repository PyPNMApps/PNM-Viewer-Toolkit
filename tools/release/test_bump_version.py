import importlib.util
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).with_name("bump-version.py")
SPEC = importlib.util.spec_from_file_location("bump_version", SCRIPT)
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class BumpVersionTests(unittest.TestCase):
    def test_reset_rules(self):
        for part, expected in [("major", "2.0.0"), ("minor", "1.3.0"), ("patch", "1.2.10")]:
            with self.subTest(part=part):
                self.assertEqual(MODULE.bumped("1.2.9", part), expected)

    def test_invalid_versions(self):
        for value in ["1.2", "1.2.3.4", "01.2.3", "1.-2.3", "1.2.3-dev", "a.b.c", ""]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                MODULE.bumped(value, "patch")

    def test_cli_from_other_directory(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            script = root / "tools/release/bump-version.py"
            script.parent.mkdir(parents=True)
            shutil.copy2(SCRIPT, script)
            version = root / "VERSION"
            version.write_text("0.1.0\n")
            command = ["python3", str(script), "--bump", "minor"]
            subprocess.run(command + ["--dry-run"], cwd="/", check=True, capture_output=True)
            self.assertEqual(version.read_text(), "0.1.0\n")
            subprocess.run(command, cwd="/", check=True, capture_output=True)
            self.assertEqual(version.read_text(), "0.2.0\n")
            version.write_text("1.2.3.4\n")
            result = subprocess.run(command, cwd="/", capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertEqual(version.read_text(), "1.2.3.4\n")


if __name__ == "__main__":
    unittest.main()
