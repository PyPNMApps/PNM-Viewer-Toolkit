# pnm-decode

PNM Viewer Toolkit's standalone Rust decoder reads one local capture, detects
its binary identity/version and exports JSON through the existing core parsers.

```bash
./install.sh --cli --prefix "$PWD/local/cli-install"
local/cli-install/bin/pnm-decode capture.bin
local/cli-install/bin/pnm-decode capture.bin --output result.json
local/cli-install/bin/pnm-decode capture.bin --stdout
local/cli-install/bin/pnm-decode capture.bin --header-only
man -l local/cli-install/share/man/man1/pnm-decode.1
```

The source installer needs the repository's local Rust toolchain. It builds only
`pnm-decode` and installs the executable, section-1 man page, Apache license and
NOTICE. It does not install UI dependencies or edit PATH. Choose an explicit
prefix; existing installation files are refused. Upgrade/uninstall management
is not implemented yet. An interrupted installation may leave partial files.

Default output replaces the input extension with `.json`. Existing files or
symlinks are refused, including the input itself. Output parents must exist.
JSON is fully generated before publication; publication requires filesystem
hard-link support and fails safely when unavailable. Normal errors clean up owned
temporary files; forced termination can leave `.pnm-decode-*.tmp` siblings.
`--stdout` avoids filesystem publication and conflicts with `--output`.

Options: `-o`/`--output PATH`, `--stdout`, `--header-only`, `-h`/`--help`,
`-V`/`--version`, and `--` to end options. Quote paths containing spaces.
Diagnostics go to stderr. The CLI does not initialize tracing, avoiding duplicate
nested parser error logs. Errors retain core field/offset information.

Input and serialized JSON are limited to 16 MiB each; the final newline is extra.
Decode policy allows 65,536 items. Format support is documented in
[format support](../reference/formats.md): symbol payloads remain unsupported and
latency has only synthetic fixture coverage.

| Exit code | Meaning |
| --- | --- |
| 0 | Success/help/version |
| 1 | File or stream I/O failure/existing destination |
| 2 | Invalid command usage |
| 3 | Unsupported identity/version/layout/decoder |
| 4 | Invalid capture or resource/output limit |

[The man page](../../crates/cli/man/pnm-decode.1) and this document must be updated
with command changes. The initial implementation uses the Rust standard library
for arguments and a checked-in manual; generation from argument metadata can be
introduced later. Windows uses `pnm-decode.exe --help` and this Markdown manual.

Verified locally on Ubuntu 22.04 x86_64. Windows/macOS native execution, packaging,
and full-toolkit installation remain unverified/unimplemented.
