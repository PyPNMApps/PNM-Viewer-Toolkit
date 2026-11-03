# Source investigation and provenance

Investigated 2026-11-02.

## Setup reference

Read PersonalDevelopmentEnviroment/AGENTS.md, README.md, and
`dev-scripts/setup_git_local_config.sh`. Follow its docs/ layout and independent
nested-project conventions. The Git setup script hard-codes user identity and
changes credential/configuration choices; it was not run or copied.

## Java archive

`/home/dev01/Projects/pnm-viewer/OpenPNM-master.tar` contains 613 directory
entries and zero regular files. Directory names suggest Java PNM components but
cannot establish implemented behavior. It was inspected without extracting or
modifying it. A complete source archive or checkout is required for reuse review.

## PyPNM reference

Inspected `/home/dev01/Projects/PyPNM/README.md` and the parser file inventory.
Candidates under `src/pypnm/pnm/parser/` include `pnm_header.py`,
`pnm_file_type.py`, `CmDsOfdmRxMer.py`, `CmDsOfdmChanEstimateCoef.py`,
`CmUsOfdmaPreEq.py`, `CmSpectrumAnalysis.py`, `CmDsConstDispMeas.py`,
`CmDsOfdmFecSummary.py`, `CmDsOfdmModulationProfile.py`, `CmSymbolCapture.py`,
`CmDsHist.py`, and `CmLatencyRpt.py`. Presence is not a correctness/compliance
assessment; algorithms and fixtures still need review.

No OSSI skills were found in this PyPNM checkout or the setup-reference checkout.
PyPNM README links to CableLabs CM-OSSI 3.1 and 4.0 specification pages.

## Copied specification skills

Copied unchanged from the build-server repository's canonical skill directories:

| Destination under skills/ | Source under build-local-mxl-build-server/skills/ | Issue |
| --- | --- | --- |
| docsis-cm-ossi-3-1-i27 | specification/docsis3.1/cm-ossi/docsis-cm-ossi-3-1-i27 | CM-SP-CM-OSSIv3.1-I27-250219 |
| docsis-cm-ossi-4-0-i13 | specification/docsis4.0/cm-ossi/docsis-cm-ossi-4-0-i13 | CM-SP-CM-OSSIv4.0-I13-260429 |

Preserve included metadata and extraction notices. These are pinned local
reference issues, not verified latest releases. Checks of the
[CableLabs 3.1 page](https://www.cablelabs.com/specifications/CM-SP-CM-OSSIv3.1)
and [4.0 page](https://www.cablelabs.com/specifications/CM-SP-CM-OSSIv4.0)
returned the specification search interface without definitive issue details.

Before implementing each format, identify controlling section/table/page,
compare PyPNM and Java behavior, and record discrepancies and fixture evidence.
New specification revisions belong in separate versioned skills; do not edit
existing extracted normative text to make it appear current.

## Repository relocation

Renamed `/home/dev01/Projects/simple-pnm` to
`/home/dev01/Projects/pnm-viewer` at the user's request. Preserved its `.git`
directory and original archive. Copied the initial project setup from
`PersonalDevelopmentEnviroment/pnm-viewer`, excluding that source's `.git`.
The setup-source directory remains in place.
