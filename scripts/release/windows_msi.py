#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Author a standard x64 MSI using the pinned WiX 3.14.1 build tools."""
import hashlib
import os
from pathlib import Path
import shutil
import uuid
import xml.etree.ElementTree as ET

from common import ROOT, run
from versions import validate

NAMESPACE = 'http://schemas.microsoft.com/wix/2006/wi'
# Stable identity for this product family; never reuse OSCAR-UI's upgrade code.
UPGRADE_CODE = '54ACD1D9-4027-47BA-9A41-B9A04109C7CE'
COMPONENT_NAMESPACE = uuid.UUID('35d58893-a0ea-4aa7-8a56-6bd194294568')
APPLICATION_ICON_ID = 'ApplicationIcon.ico'
ET.register_namespace('', NAMESPACE)


def _element(name, parent=None, **attributes):
    """Create a namespaced authoring element with XML-escaped attributes."""
    tag = '{' + NAMESPACE + '}' + name
    if parent is None:
        return ET.Element(tag, attributes)
    return ET.SubElement(parent, tag, attributes)


def _identifier(prefix, relative):
    """Give each case-insensitive Windows resource path a stable WiX identifier."""
    return prefix + hashlib.sha256(relative.casefold().encode()).hexdigest()[:32]


def _desktop_options(product):
    """Offer a pre-install shortcut choice without executing custom actions."""
    _element('Property', product, Id='CREATE_DESKTOP_SHORTCUT', Secure='yes')
    ui = _element('UI', product)
    dialog = _element('Dialog', ui, Id='ShortcutOptionsDlg', Width='370', Height='270',
                      Title='PNM Viewer Toolkit Setup')
    _element('Control', dialog, Id='Banner', Type='Bitmap', X='0', Y='0',
             Width='370', Height='44', TabSkip='yes', Text='WixUI_Bmp_Banner')
    _element('Control', dialog, Id='Title', Type='Text', X='20', Y='15', Width='280',
             Height='18', Transparent='yes', Text='{\\WixUI_Font_Title}Desktop shortcut')
    _element('Control', dialog, Id='DesktopChoice', Type='CheckBox', X='20', Y='80',
             Width='330', Height='22', Property='CREATE_DESKTOP_SHORTCUT',
             CheckBoxValue='1', Text='Create a desktop shortcut')
    _element('Control', dialog, Id='Description', Type='Text', X='20', Y='110',
             Width='330', Height='40', Text='The application is always available from the Start menu.')
    _element('Control', dialog, Id='BottomLine', Type='Line', X='0', Y='234', Width='370', Height='0')
    for identifier, text, position, target in (
            ('Back', '< Back', '180', 'InstallDirDlg'),
            ('Next', 'Next >', '236', 'VerifyReadyDlg'),
            ('Cancel', 'Cancel', '304', 'CancelDlg')):
        button = _element('Control', dialog, Id=identifier, Type='PushButton',
                          X=position, Y='243', Width='56', Height='17', Text=text,
                          **({'Cancel': 'yes'} if identifier == 'Cancel' else {}),
                          **({'Default': 'yes'} if identifier == 'Next' else {}))
        event = _element('Publish', button, Event='SpawnDialog' if identifier == 'Cancel' else 'NewDialog', Value=target)
        event.text = '1'
    # Stock path validation runs first; the last eligible NewDialog event wins.
    forward = _element('Publish', ui, Dialog='InstallDirDlg', Control='Next',
                       Event='NewDialog', Value='ShortcutOptionsDlg', Order='5')
    forward.text = 'NOT Installed AND (WIXUI_DONTVALIDATEPATH OR WIXUI_INSTALLDIR_VALID = "1")'
    backward = _element('Publish', ui, Dialog='VerifyReadyDlg', Control='Back',
                        Event='NewDialog', Value='ShortcutOptionsDlg', Order='5')
    backward.text = 'NOT Installed'


def author(payload, version, destination):
    """Write fresh MSI authoring with stable file components and rollback-safe upgrades.

    Args:
        payload: Fresh deployment tree containing the GUI, CLI and app-local DLLs.
        version: Canonical major.minor.patch within MSI's 255.255.65535 bounds.
        destination: New XML source path; existing authoring is never replaced.
    Returns:
        The written authoring path.
    Raises:
        ValueError: Invalid version, missing entry points, links or path collisions.
        OSError: Source tree or destination I/O failure.
    """
    version = validate(version)
    if any(part > limit for part, limit in zip(map(int, version.split('.')), (255, 255, 65535))):
        raise ValueError('Version exceeds Windows Installer limits')
    payload = payload.resolve()
    if not all((payload / name).is_file() for name in ('pnm-viewer.exe', 'pnm-decode.exe')):
        raise ValueError('MSI payload requires both viewer and decoder')
    wix = _element('Wix')
    product = _element('Product', wix, Id='*', Name='PNM Viewer Toolkit', Language='1033',
                      Version=version, Manufacturer='PyPNMApps contributors', UpgradeCode=UPGRADE_CODE)
    _element('Package', product, InstallerVersion='500', Compressed='yes', InstallScope='perMachine', Platform='x64')
    _element('MajorUpgrade', product, DowngradeErrorMessage='A newer PNM Viewer Toolkit version is already installed.',
            Schedule='afterInstallInitialize')
    _element('MediaTemplate', product, EmbedCab='yes', CompressionLevel='high')
    _element('Property', product, Id='INSTALLFOLDER', Secure='yes')
    _element('Property', product, Id='WIXUI_INSTALLDIR', Value='INSTALLFOLDER')
    _element('UIRef', product, Id='WixUI_InstallDir')
    _element('WixVariable', product, Id='WixUILicenseRtf', Value=str(destination.with_suffix('.rtf')))
    for identifier, filename in (('WixUIDialogBmp', 'dialog.bmp'), ('WixUIBannerBmp', 'banner.bmp')):
        _element('WixVariable', product, Id=identifier, Value=str(ROOT / 'packaging/windows' / filename))
    _element('Icon', product, Id=APPLICATION_ICON_ID, SourceFile=str(ROOT / 'packaging/windows/pnm-viewer.ico'))
    _element('Property', product, Id='ARPPRODUCTICON', Value=APPLICATION_ICON_ID)
    _desktop_options(product)
    condition = _element('Condition', product, Message='64-bit Windows is required.')
    condition.text = 'Installed OR VersionNT64'
    root = _element('Directory', product, Id='TARGETDIR', Name='SourceDir')
    programs = _element('Directory', root, Id='ProgramFiles64Folder')
    install = _element('Directory', programs, Id='INSTALLFOLDER', Name='PNM Viewer Toolkit')
    menu = _element('Directory', root, Id='ProgramMenuFolder')
    _element('Directory', menu, Id='ApplicationMenu', Name='PNM Viewer Toolkit')
    _element('Directory', root, Id='DesktopFolder')
    feature = _element('Feature', product, Id='Complete', Title='PNM Viewer Toolkit', Level='1')
    shortcut_component = _element('Component', install, Id='DesktopShortcutComponent', Win64='yes',
                                  Guid=str(uuid.uuid5(COMPONENT_NAMESPACE, 'desktop-shortcut')),
                                  Transitive='yes')
    _element('Condition', shortcut_component).text = 'CREATE_DESKTOP_SHORTCUT = "1"'
    _element('Shortcut', shortcut_component, Id='DesktopViewer', Directory='DesktopFolder',
             Name='PNM Viewer Toolkit', Target='[INSTALLFOLDER]pnm-viewer.exe',
             WorkingDirectory='INSTALLFOLDER', Icon=APPLICATION_ICON_ID, IconIndex='0')
    _element('RegistryValue', shortcut_component, Root='HKCU', Key='Software\\PyPNMApps\\PNMViewerToolkit',
             Name='DesktopShortcut', Type='integer', Value='1', KeyPath='yes')
    _element('ComponentRef', feature, Id='DesktopShortcutComponent')
    directories = {'.': install}
    seen = set()
    for path in sorted(payload.rglob('*')):
        relative = path.relative_to(payload)
        spelling = relative.as_posix()
        if path.is_symlink() or spelling.casefold() in seen:
            raise ValueError('MSI payload contains a link or case-insensitive path collision')
        seen.add(spelling.casefold())
        if path.is_dir():
            directories[spelling] = _element('Directory', directories[relative.parent.as_posix()],
                                            Id=_identifier('D', spelling), Name=path.name)
            continue
        if not path.is_file():
            raise ValueError('MSI payload contains a special file')
        component_id = _identifier('C', spelling)
        component = _element('Component', directories[relative.parent.as_posix()], Id=component_id,
                            Guid=str(uuid.uuid5(COMPONENT_NAMESPACE, spelling.casefold())), Win64='yes')
        installed = _element('File', component, Id=_identifier('F', spelling), Source=str(path), KeyPath='yes')
        if spelling == 'pnm-viewer.exe':
            # Use the key executable's embedded icon. An explicit ICO override
            # on this advertised shortcut violates MSI ICE50 extension matching.
            _element('Shortcut', installed, Id='StartMenuViewer', Directory='ApplicationMenu',
                    Name='PNM Viewer Toolkit', Advertise='yes', WorkingDirectory='INSTALLFOLDER')
            _element('RemoveFolder', component, Id='RemoveApplicationMenu', Directory='ApplicationMenu', On='uninstall')
        _element('ComponentRef', feature, Id=component_id)
    legal = 'Desktop: GPL-3.0-only. Core libraries: Apache-2.0. See bundled notices.\n\n'
    legal += (ROOT / 'LICENSES/GPL-3.0-only.txt').read_text(encoding='utf-8')
    escaped = []
    for character in legal:
        if character in '\\{}':
            escaped.append('\\' + character)
        elif character == '\n':
            escaped.append('\\par\n')
        elif ord(character) < 128:
            escaped.append(character)
        else:
            codepoint = ord(character)
            if codepoint >= 32768:
                codepoint -= 65536
            escaped.append(f'\\u{codepoint}?')
    with destination.with_suffix('.rtf').open('x', encoding='ascii') as license_file:
        license_file.write('{\\rtf1\\ansi\\deff0 {\\fonttbl {\\f0 Segoe UI;}}\\f0\\fs18 ' + ''.join(escaped) + '}')
    ET.indent(wix)
    with destination.open('xb') as stream:
        ET.ElementTree(wix).write(stream, encoding='utf-8', xml_declaration=True)
    return destination


def _tools():
    """Resolve both WiX tools from one explicit or discovered installation.

    Returns:
        Native candle.exe/light.exe paths; never downloads tools or changes PATH.
    Raises:
        ValueError: The selected installation does not contain both tools.
    """
    selected = os.environ.get('WIX_BIN')
    compiler = None
    if selected:
        compiler = Path(selected) / 'candle.exe'
    if compiler is None:
        found = shutil.which('candle.exe')
        if found:
            compiler = Path(found)
    if compiler is None or not compiler.is_file() or not (compiler.parent / 'light.exe').is_file():
        raise ValueError('WiX 3.14.1 missing; set WIX_BIN to its bin directory')
    return compiler, compiler.parent / 'light.exe'


def build(payload, stage, output, version):
    """Compile and validate the standard MSI, retaining XML and logs on failure.

    Args:
        payload: App-local deployment tree.
        stage: Fresh generated packaging directory for WiX source/objects.
        output: New MSI destination.
        version: Validated project version.
    Returns:
        None after candle/light succeed, including Windows Installer validation.
    Raises:
        ValueError: Missing tools, invalid authoring or an existing installer.
        subprocess.CalledProcessError: Authoring/compiler/installer validation failure.
    """
    if output.exists():
        raise ValueError('MSI output already exists')
    compiler, linker = _tools()
    source = author(payload, version, stage / 'installer.wxs')
    object_file = stage / 'installer.wixobj'
    run([compiler, '-nologo', '-arch', 'x64', '-out', object_file, source])
    run([linker, '-nologo', '-ext', 'WixUIExtension', '-out', output, object_file])
