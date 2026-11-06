# Developer guide

[Documentation index](../README.md) · [Desktop user guide](../ui/README.md)

PNM Viewer Toolkit is under development. Start with setup, then use the guide
for the component you are changing. End-user controls belong in `docs/ui/`;
CLI behavior belongs in `docs/cli/`.

| Work | Guide |
| --- | --- |
| Set up local dependencies | [Development environment](../install/development.md) |
| Build, launch and check native Qt | [Qt setup](qt.md) |
| Understand charts and Rust/QML ownership | [Qt measurement internals](qt-measurements.md) |
| Understand spectrum integration and range tabs | [dBmV integration math](spectrum-power.md) |
| Work on core APIs, parsers and math | [Rust development](rust.md) |
| Change compiled QAM resources | [QAM conversion and geometry](qam-resources.md) |
| Review future IPC and retrieval plans | [API contract](api-contract.md) |
| Save and push changes | [Git workflow](git.md) |
| Understand required OS checks | [CI and platform matrix](ci.md) |
| Reclaim Rust build space | [Maintenance and 95% disk guard](maintenance.md) |
| Bump versions or prepare the first tag | [Release preparation](releasing.md) |
| Refresh README and gallery pictures | [Screenshots](screenshots.md) |

The root `VERSION` is the single version source. Generated consumers must agree;
run `python3 tools/release/versions.py` before release preparation. Keep detailed
working plans and private specification evidence in ignored root `development/`.
