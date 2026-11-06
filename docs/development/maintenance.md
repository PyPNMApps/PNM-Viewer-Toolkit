# Rust build maintenance

[Developer guide](README.md) · [Development setup](../install/development.md)

`tools/maintenance/rust_clean.py` owns cleanup. Build entry points call this
script's disk check; they do not implement their own directory deletion.

Before project Cargo builds/tests/checks, Qt checks, the pre-push gate and native
release builds, the helper checks usage on the checkout's filesystem. At **95%
or higher**, it calls Rust's `cargo clean` for both workspaces and rechecks space.
If usage is still at least 95%, compilation stops with an error. This is a
pre-build check, not a background monitor that deletes files mid-compilation.
Direct invocations of a system `cargo` outside project helpers do not run it.

The only cleanup targets are `target/` and `apps/qt/target/`. The script validates
Cargo's `CACHEDIR.TAG`, rejects symlink redirects and tracked contents, then lets
Cargo acquire build locks. It preserves `local/cargo`, `local/rustup`, `local/qt`,
release installers, captures and other repositories. Custom/external
`CARGO_TARGET_DIR` locations are not cleaned automatically. Cleanup can make the
next build cold. No system packages or administrator access are required.

Preview a manual cleanup:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/maintenance/rust_clean.py --dry-run
```

Clean both project Rust build directories now:

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/maintenance/rust_clean.py
```

Run only the threshold check (the mode used by build helpers):

```bash
cd /home/dev01/Projects/PNM-Viewer-Toolkit &&
python3 tools/maintenance/rust_clean.py --if-disk-full
```

These local commands do not clear GitHub Actions caches or workflow artifacts.
