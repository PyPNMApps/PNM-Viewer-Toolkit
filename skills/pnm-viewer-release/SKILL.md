---
name: pnm-viewer-release
description: Prepare PNM Viewer three-component version bumps and release commands. Use for this repository's major, minor, patch, tag, and release workflow; not for other repositories.
---

# PNM Viewer release workflow

Read `docs/release/versioning.md`, `VERSION`, and the repository's helpers before preparing
commands. Resolve those paths from the repository root (two levels above this
canonical skill directory). VERSION is the canonical source. The bump helper synchronizes local Cargo
manifests/lockfiles, including the standalone Qt crate. Run
`python3 tools/release/versions.py` to reject drift; never override a version from
a tag or edit consumer manifests independently.

Use `tools/bump-version.sh --bump major|minor|patch`, with `--dry-run` to preview.
Require exactly MAJOR.MINOR.PATCH. Major resets minor and patch; minor resets
patch. Use tags `vMAJOR.MINOR.PATCH`. Do not copy PyPNM's fourth component.

Inspect tracked, staged, and untracked changes before drafting a save command;
the save helper stages all non-ignored files. Follow AGENTS.md message format.
Supply copy/paste commands beginning with `cd /home/dev01/Projects/PNM-Viewer-Toolkit &&`
unless explicitly asked to execute. A request to add tooling is not a request to
release. Combined --tag-patch, --tag-minor and --tag-major modes explicitly
automate versioning, checks, commit, annotated tag and atomic push when the user
runs them. Plain push stays branch-only; the standalone bump helper stays local.

Before a release tag, require clean master (the tag workflow checks origin/master), verify VERSION matches
the committed file, review validation evidence, and check local and remote tag
availability. Never replace existing tags. Prefer the combined release modes
with --dry-run for a preview. They validate the bumped candidate before committing
and atomically push exact objects; never use `git push --tags` or Git force-push.
For existing tags or failed uploads use git-push.sh --tag vMAJOR.MINOR.PATCH:
it checks VERSION/HEAD and revalidates without bumping again. If both origin refs
match it is a no-op. Preserve local candidate/commit/tag state after failures;
inspect it before retrying. See docs/development/releasing.md for recovery.
A bump at an already published current-version HEAD is a no-op by default.
The user may explicitly request `--force` with a bump mode for a rebuild/update
test without new development changes. This creates a fresh version/tag while
retaining clean-tree, validation, collision and normal atomic-push checks.
Do not supply it by default or use it to bypass unpublished release recovery.

The project is in the dev phase. A version or tag alone is not evidence of a
working application, tested platform, packaged artifact, or published release.

## Native CI and packages

Read `docs/development/ci.md` and `docs/release/packaging.md` for tag automation.
A pushed stable tag must match VERSION and belong to master. The tag workflow
calls the same ordered CI as master before packaging; all Ubuntu 22.04/24.04/26.04,
Windows x64 and macOS arm64/x64 checks are required. Do not skip a platform to
make a release pass. Missing, skipped and canceled jobs block the aggregate.

Build locally with `python3 scripts/release/build.py --output dist/release`
(`python` on Windows) only after native prerequisites are present. Preserve
existing outputs; the builder refuses reuse. Verify artifact versions, hashes
and the 50,000,000-byte budget with `scripts/release/verify.py`.

Installed lifecycle checks belong on disposable GitHub runners; do not set
GITHUB_ACTIONS locally to bypass their guard. Report local relocated-binary smoke
checks separately from native install/remove checks. Ubuntu installs the same
22.04-built DEBs on all three versions, and removing the UI must preserve the CLI.

User authorization on 2026-11-04 supersedes the earlier draft-only rule.
Complete stable releases require every target and installed lifecycle gate.
On 2026-11-05 the user authorized publishing successful same-run platforms as
a labeled partial prerelease when another target fails. Keep all failed jobs
and the overall workflow failed. Global identity/CodeQL/code checks must pass;
every included platform must pass native and installed checks. Linux needs all
three Ubuntu native/lifecycle checks. Continue Ubuntu installation tests when
its baseline is available despite an unrelated failure. Exclude failed targets,
list missing platforms, reject an empty set and use prerelease/latest=false.
Never offer partial releases through the stable in-app update path. Existing
public assets remain immutable; missing targets need a new version/tag.
Verify uploaded sizes, GitHub DEB SHA-256 digests and re-downloaded bytes before
automatically publishing; only complete passing releases are marked latest.
Never mutate a public release.
The user restored automatic full tag delivery on 2026-11-04; the old Ubuntu-only
workflow is manual troubleshooting only. Require MSI/DMG and all Ubuntu lifecycle
gates; do not treat legacy Ubuntu-only success as multi-OS validation. Windows
and macOS in-app installers/MSP patches are deferred; manual installer releases
come first. External signing/notarization remain separate.
A workflow file or local build is not proof of hosted platform success.

Native dependency artifacts span successful ancestor tags for Ubuntu 22.04 and
both Mac architectures. Preserve full checkout history, compiler/image/Qt and
architecture fingerprints, bounded extraction and exclusion of workspace/test
outputs. The first revised Mac tag seeds its cache; measure restoration on a
later compatible tag. Never skip tests or change public assets to warm a cache.

For tag packaging, keep Ubuntu 22.04 in its own call to the shared native
workflow so its three installed lifecycle jobs can overlap the other five
native runners. Manual non-packaging checks retain the six-runner call. Required
checks must include both calls; publication must inspect successful individual
Ubuntu native and lifecycle results, rejecting missing/duplicate evidence.
Linux and both Mac hosts wrap C/C++ with ccache; preserve its content/flag/source
validation, bounded local cache and final per-run statistics. Reuse only the
existing trusted native artifact path. Measure a compatible hosted tag before
claiming a speedup; never trade away tests or compiler validation for cache hits.
Put the runner and Qt identifier inside rust-cache's shared-key itself: supplying
both shared-key and key discards key, allowing the Ubuntu jobs to collide.

## Native icons, shortcuts and examples

Use packaging/ assets derived from the existing desktop PNG; regenerate into a
new review directory with tools/branding/generate.py. Embed the ICO in the GUI
executable and MSI shell integration; macOS needs CFBundleIconFile and its ICNS.
WiX artwork uses the project navy/teal palette. Offer an unchecked desktop
shortcut option before installation, with explicit silent opt-in/out and no
application-launch custom action. Keep Start-menu integration independent.
Advertised Start-menu shortcuts use the embedded key-executable icon. Give
separate desktop/ARP ICO entries a .ico identifier; an ICO override on an
advertised .exe key file violates ICE50. Keep native installer validation on.
macOS uses conventional Applications/Dock icons and an optional Finder alias.

Desktop installers include nine single captures, generated RxMER collection ZIPs
and the pre-EQ comparison pair. scripts/release/samples.py owns bundling, upstream
notices/provenance and the expanded-source manifest. Apply the sw-qa-hygiene skill:
check expanded captures before generating any ZIP; never use prebuilt demo archives
as installer sources. Failed hygiene stops packaging. CLI-only archives stay
capture-free. Install demos in `files/demo` beside Windows/Linux executables or under
`Contents/Resources/files/demo` on macOS. Resolve the initial capture/file/folder/
archive selection location from the installed executable, use that shared demo
root for every picker, fall back to Home for unpackaged builds,
and never select an installation directory as the default export destination.
Require installed smoke checks to use and verify bundled examples, not silently
substitute checkout fixtures. Keep installed icon/shortcut lifecycle checks and
the 50 MB artifact gate. Add new build/asset-conversion packages to the explicit
development installer and documentation; do not install them while validating.

## Catch platform failures before tags

Master/PR desktop preflight runs packaging tests on Windows and both Macs, and
the complete QML suite under Cocoa on both Macs, without Rust compilation. Push
the implementation branch separately and inspect Required checks before offering
a release tag. Combined tag-push still performs local checks and an atomic push;
it does not itself wait for branch CI. Full native/tag runs skip duplicate
preflight but retain every native and installed lifecycle gate.

For QML frame assertions use the shared RenderTestCase waitForFrame helper, which
requests a frame even for unchanged scenes. For input assertions await polished,
visible hit geometry and check the actual signal arguments. Never replace failed
assertions with retries, sleeps or skips. A Linux offscreen pass does not prove
native Cocoa behavior; retain reports and require native confirmation.

About installer downloads must select the exact published asset matching the
Rust OS/architecture and release version, with no guessed links when assets are
missing. Keep Windows MSI and Intel/Apple Silicon DMG cases independently tested.
Ubuntu manual downloads expose the paired DEB release page; preserve verified
in-app installation and restart. Keep one compact update status and the installed
version header, with Windows Uninstall under Software.
