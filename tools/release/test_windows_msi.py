#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""MSI identity and security contracts independent of native Windows tooling."""
from pathlib import Path
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'scripts/release'))
import windows_msi


class WindowsMsiTests(unittest.TestCase):
    """Inspect actual installer authoring with synthetic, non-executable files."""

    def setUp(self):
        """Create a nested image with spaces/ampersands to exercise XML escaping."""
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.payload = self.root / 'payload & spaces'
        self.payload.mkdir()
        for name in ('pnm-viewer.exe', 'pnm-decode.exe', 'platforms/qwindows.dll'):
            path = self.payload / name
            path.parent.mkdir(exist_ok=True)
            path.write_bytes(b'synthetic fixture')
        self.names = {'w': windows_msi.NAMESPACE}

    def test_machine_scope_upgrade_identity_and_no_executable_actions(self):
        """Keep stable components across versions without app-launch/security overrides."""
        products = []
        for version in ('0.1.0', '0.1.1'):
            source = windows_msi.author(self.payload, version, self.root / (version + '.wxs'))
            products.append(ET.parse(source).find('w:Product', self.names))
        first, second = products
        self.assertEqual(first.get('UpgradeCode'), second.get('UpgradeCode'))
        self.assertEqual(first.find('w:Package', self.names).get('InstallScope'), 'perMachine')
        self.assertEqual(first.find('w:MajorUpgrade', self.names).get('Schedule'), 'afterInstallInitialize')
        self.assertEqual([item.attrib for item in first.findall('.//w:Component', self.names)],
                         [item.attrib for item in second.findall('.//w:Component', self.names)])
        self.assertIsNone(first.find('.//w:CustomAction', self.names))
        self.assertIsNone(first.find('.//w:Environment', self.names))
        shortcut = first.find('.//w:Shortcut[@Id="StartMenuViewer"]', self.names)
        self.assertEqual(shortcut.get('Advertise'), 'yes')
        for installed in first.findall('.//w:File', self.names):
            self.assertTrue(Path(installed.get('Source')).is_file())
        self.assertEqual(len(first.findall('.//w:File', self.names)), 3)

    def test_branded_installer_offers_an_unchecked_desktop_choice(self):
        """Use real branding and a conditioned shortcut without launching the app."""
        source = windows_msi.author(self.payload, '0.2.2', self.root / 'branded.wxs')
        product = ET.parse(source).find('w:Product', self.names)
        icon = product.find('w:Icon[@Id="ApplicationIcon"]', self.names)
        self.assertTrue(Path(icon.get('SourceFile')).is_file())
        self.assertEqual(product.find('w:Property[@Id="ARPPRODUCTICON"]', self.names).get('Value'), 'ApplicationIcon')
        for identifier in ('WixUIDialogBmp', 'WixUIBannerBmp'):
            artwork = product.find(f'w:WixVariable[@Id="{identifier}"]', self.names)
            self.assertTrue(Path(artwork.get('Value')).is_file())
        choice = product.find('w:Property[@Id="CREATE_DESKTOP_SHORTCUT"]', self.names)
        self.assertIsNone(choice.get('Value'))
        checkbox = product.find('.//w:Control[@Id="DesktopChoice"]', self.names)
        self.assertEqual(checkbox.get('Property'), 'CREATE_DESKTOP_SHORTCUT')
        self.assertEqual(checkbox.get('CheckBoxValue'), '1')
        component = product.find('.//w:Component[@Id="DesktopShortcutComponent"]', self.names)
        self.assertEqual(component.find('w:Condition', self.names).text, 'CREATE_DESKTOP_SHORTCUT = "1"')
        self.assertEqual(component.find('w:RegistryValue', self.names).get('KeyPath'), 'yes')
        self.assertEqual(component.find('w:Shortcut', self.names).get('Icon'), 'ApplicationIcon')
        self.assertIsNone(product.find('.//w:CustomAction', self.names))

    def test_version_bounds_case_collisions_and_existing_source_rejected(self):
        """MSI must reject invalid identity before compiling or modifying installers."""
        for version in ('256.1.0', '1.256.0', '1.0.65536', '1.2.3.4'):
            with self.assertRaises(ValueError):
                windows_msi.author(self.payload, version, self.root / 'invalid.wxs')
        windows_msi.author(self.payload, '255.255.65535', self.root / 'maximum.wxs')
        with self.assertRaises(FileExistsError):
            windows_msi.author(self.payload, '1.0.0', self.root / 'maximum.wxs')
        # Distinct case spellings can coexist only on a case-sensitive host.
        duplicate = self.payload / 'PNM-VIEWER.exe'
        if not duplicate.exists():
            duplicate.write_bytes(b'collision')
            with self.assertRaisesRegex(ValueError, 'collision'):
                windows_msi.author(self.payload, '1.0.0', self.root / 'collision.wxs')


if __name__ == '__main__':
    unittest.main()
