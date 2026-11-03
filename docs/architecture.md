# Architecture draft

Status: proposed boundaries; no framework or dependency has been selected.

## Data flow

Local file -> bounded format detection -> version-specific parser -> typed
measurement model -> analysis -> desktop chart/table presentation.

Keep UI widgets out of parsing and analysis. Preserve file type, format version,
metadata, units, raw values, and diagnostic context in models. Missing metadata
must remain distinguishable from a valid zero. Return structured failures with
file offset and expected/actual data where known.

A background task owns parsing and cancellation. Publish completed results to the
UI thread; cancel or discard stale results when a user changes files. Bound input
sizes and sample allocations using verified format limits and explicit application
limits. Keep original captures read-only.

If live acquisition is selected later, put device communication and transfer in
a separate component that feeds the same parser entry point. A Python service is
not required for the proposed desktop flow.

## Technology decision gate

Evaluate JavaFX and Qt Jambi against target OS support, chart performance with
representative captures, accessibility, packaging, dependency maintenance, and
licensing. Choose after platform requirements and Java reuse review. Do not add
both frameworks or choose versions without checking their current support.

## Reuse gate

The legacy archive contains no regular files. No Java API, build compatibility,
parser correctness, or license has been confirmed. Inspect actual sources first.
PyPNM parsers provide comparison candidates, not normative definitions. Establish
shared fixtures and expected values before porting numerical behavior.
