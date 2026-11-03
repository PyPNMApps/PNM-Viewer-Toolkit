# Agent Instructions

PNM Viewer is a Java desktop project in the requirements phase.

- Keep confirmed requirements separate from proposed scope in docs/.
- Maintain requirements, architecture, and validation checklist together.
- Do not introduce browser UI, WebView charts, or a required HTTP backend.
- Use PyPNM as a reference; distinguish implementation observations from DOCSIS requirements.
- Inspect existing Java sources and licensing before porting or replacing code.
- Use the versioned CM-OSSI skills for PNM format and management requirements.
  Cite issue, section, and page; never silently merge DOCSIS 3.1 and 4.0 semantics.
- Treat unknown formats and unsupported versions explicitly, without guessing.
- Keep parsing and analysis independent of UI and future acquisition code.
- Keep dependencies project-local and pinned once selected. Do not install globally.
- Keep credentials, captures containing private data, generated output, and local
  configuration out of Git. Preserve upstream notices when reusing material.
- Keep skills canonical under skills/ with relative discovery links under
  .agents/skills/ and .claude/skills/. Validate changed skills and links.
- Run relevant parser tests and desktop checks once implementation exists;
  never describe unexecuted tests or unverified platforms as passing.

## Git workflow

- Use `tools/git/git-save.sh --commit-msg "..."` for local commits and
  `tools/git/git-push.sh` for pushes. Inspect the helpers before constructing commands.
- Supply copy/paste commands unless the user explicitly asks to execute a commit
  or push. Begin command blocks with `cd /home/dev01/Projects/pnm-viewer &&`.
- Before writing a commit message, inspect `git status --short`, `git diff --stat`,
  `git diff`, staged changes, and every untracked file. The save helper stages all
  non-ignored changes, so the message must describe the complete scope.
- Use `Feature:`, `BugFix:`, or `Documentation:` followed by an imperative summary,
  then `- ` detail lines including validation. Keep each message line at most
  60 characters. Pass multiline messages as one double-quoted argument.
- Push requires a clean main/master branch and origin; never force-push.
- See docs/git-workflow.md for commands and helper behavior.

## Versioning

- Use the `pnm-viewer-release` skill for version bumps and release work.
- Keep VERSION in MAJOR.MINOR.PATCH format; use tools/bump-version.sh.
- See docs/release.md. Version bumps do not authorize commits, tags, or pushes.
