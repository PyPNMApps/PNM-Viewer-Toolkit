# DOCSIS 4.0 CM-OSSI I13 Topic Map

Use this map first, then open the smallest relevant section file from `section-index.md`.
The PDF remains authoritative.

| Topic | Primary references | Notes |
|---|---|---|
| SNMP management protocol and IPv6 | `sections/06-ossi-management-protocols.md` | SNMP protocol requirements and IPv6 handling. |
| CableLabs/IETF MIB modules and managed-object requirements | `sections/07-ossi-management-objects.md`, `sections/annex-a-1-mib-object-details.md`, `sections/annex-a-2-a-4-docsis40-fdx-fdd-mib-details.md`, `sections/annex-a-5-if-table-details.md` | MIB support, object access, DOCSIS 4.0/FDX/FDD MIB details, interface tables and counters. |
| DOCSIS 4.0 FDX/FDD management objects | `sections/annex-a-2-a-4-docsis40-fdx-fdd-mib-details.md`, `sections/annex-f-2-3-cm-operational-status.md`, `sections/annex-f-3-3-upstream-interface-model.md` | Includes DOCS-IF40, DOCS-FDX and DOCS-FDD management requirements plus FDX/FDD status/capability/interface objects. |
| Fault management, events, traps and syslog | `sections/08-1-fault-management.md`, `sections/annex-c-event-syslog-snmp-notification.md` | Event notification, throttling/priority, event formats, syslog and SNMP notifications. |
| Configuration management | `sections/08-2-configuration-management.md` | Version/system configuration, secure software download, configuration files, TLV-11, MIB OIDs/values. |
| Accounting and performance management | `sections/08-3-accounting-management.md`, `sections/08-4-performance-management.md` | Usage/accounting and MIB counter treatment. |
| SNMP modes, VACM/USM and access control | `sections/08-5-1-2-snmp-security-access.md` | CM SNMP security modes and access-control configuration. |
| Security management information model | `sections/08-5-3-security-management-model.md` | DOCSIS 4.0 CM security-management UML information model, certificates, BPI+, software download, SSH/CDS-related management objects. |
| CMCI diagnostics and filtering | `sections/09-ossi-for-cmci.md`, `sections/annex-b-ip-protocol-filtering.md` | CMCI SNMP/console/diagnostics and LLC/IP filtering/classification. |
| CM LED behavior | `sections/10-ossi-for-led-indicators.md` | Power-on/self-test, downstream scan, CM-SG/ranging, operational/activity indication. |
| PNM information model | `sections/annex-d-1-overview.md`, `sections/annex-d-2-1-type-definitions.md` | Extended network monitoring, PNM model and shared types. |
| Spectrum analysis | `sections/annex-d-2-4-spectrum-analysis.md`, `sections/appendix-i-spectrum-analysis-use-cases.md` | Spectrum controls/results; Appendix I is informative. |
| Downstream symbol capture | `sections/annex-d-2-5-symbol-capture.md` | CmSymbolCapture objects and file format. |
| Channel estimate coefficients | `sections/annex-d-2-6-channel-estimate.md`, `sections/annex-d-4-slope-ripple-algorithms.md` | Channel-estimate objects/file format plus slope/ripple and group-delay metrics. |
| Constellation display | `sections/annex-d-2-7-constellation-display.md` | CmDsConstDispMeas and file format. |
| Downstream OFDM RxMER | `sections/annex-d-2-8-rxmer.md` | CmDsOfdmRxMer, summary statistics and RxMER file format. |
| OFDM MER margin and required QAM MER | `sections/annex-d-2-9-mer-margin.md`, `sections/annex-d-2-11-required-qam-mer.md` | MER margin and required-QAM-MER objects. |
| Downstream OFDM FEC summary | `sections/annex-d-2-10-fec-summary.md` | FEC summary objects and file format. |
| Downstream histogram | `sections/annex-d-2-12-downstream-histogram.md` | Histogram objects, file format and bin centers. |
| Upstream pre-equalization | `sections/annex-d-2-13-upstream-pre-equalization.md` | CmUsPreEq and pre-equalization file formats. |
| Downstream OFDM modulation profile | `sections/annex-d-2-14-modulation-profile.md` | Modulation-profile management objects and file data. |
| Latency reporting | `sections/annex-d-3-latency-reporting.md` | CmLatencyRpt and latency-report file formats. |
| Slope/ripple and group-delay algorithms | `sections/annex-d-4-slope-ripple-algorithms.md` | Best-fit equations and summary metrics. Verify formulas in PDF if extraction is ambiguous. |
| Bulk data transfer | `sections/annex-d-5-bulk-data-transfer.md` | CM bulk-data requirements and objects. |
| DOCSIS 4.0 data types | `sections/annex-e-data-type-definitions.md` | General/extended data types and shortened common terms. |
| CM operational status | `sections/annex-f-1-f-2-2-status-definitions.md`, `sections/annex-f-2-3-cm-operational-status.md` | Status mapping and operational objects, including DOCSIS 4.0 FDD/FDX/echo-cancellation status. |
| Downstream interfaces | `sections/annex-f-3-2-downstream-interface-model.md` | SC-QAM/OFDM channel objects, profile statistics and channel power. |
| Upstream interfaces | `sections/annex-f-3-3-upstream-interface-model.md` | SC-QAM/OFDMA objects, profile stats, minislot state, FDX and FDD OFDMA channel objects. |
| RCC / receive-channel status | `sections/annex-g-2-2-rcc-status.md` | Receive module/channel status model. |
| QoS configuration objects | `sections/annex-g-2-3-docsis-qos-objects.md` | Classifier, parameter set, aggregate/service-flow and SID-cluster objects. |
| QoS, latency and congestion statistics | `sections/annex-g-2-4-qos-statistics.md` | Service-flow, aggregate, dynamic-service, latency and congestion statistics. |
| DSID objects and statistics | `sections/annex-g-2-5-dsid-objects.md` | DSID configuration/statistics/client objects. |
| CM provisioning / MAC-domain configuration | `sections/annex-g-2-6-cm-provisioning.md` | MAC-domain, energy-management and CM MAC objects. |
| Tables, object attributes and file formats | Relevant section plus authoritative PDF | Verify layout-sensitive row/column associations, enumerations, ranges, units and access semantics against the PDF. |
| Cross-spec MULPI/PHY/SEC/CCAP-OSSI requirements | Relevant section plus corresponding separate skill | I13 normatively references MULPI 4.0 I11, PHY 4.0 I07, SEC 4.0 I08, and other specifications; do not silently import external requirements. |
