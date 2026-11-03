# Validation checklist

All application checks are pending; this repository has no runnable application.

| Requirement | Acceptance evidence needed |
| --- | --- |
| R1, R6 | Independent Git root at /home/dev01/Projects/pnm-viewer, preserving the former simple-pnm repository. |
| R2 | Launch desktop package without browser/backend; inspect UI on each target OS. |
| R3 | Compare shared fixtures with a recorded PyPNM revision and independently checked values. |
| R4 | Review actual Java sources, license, dependencies, and compile baseline. |
| R5 | Validate skill structure, links, provenance, and issue-specific citations. |

## Proposed first-increment checks

- Valid RxMER capture: header, sample count, frequency axis, units, and values.
- Truncated header/payload, unknown signature/version, inconsistent lengths,
  invalid counts, and allocation limits: deterministic diagnostic, no crash.
- Numerical fixtures: signedness, byte order, scaling, sentinel handling, and
  frequencies checked against the controlling specification for that format.
- UI: open/cancel, drag/drop, rapid file switching, zoom/reset, keyboard access,
  window scaling, and visible error recovery.
- Representative large capture: responsive UI and measured memory use; set
  quantitative targets once target machines and capture sizes are known.
- Separate DOCSIS 3.1 and 4.0 format/version fixtures and supported-format matrix.

## Setup checks

Repository initialization, skill validation, relative-link resolution, and local
Markdown link checks are setup checks only; they do not validate PNM behavior.
