# DOCSIS 3.1 CM-OSSI I27 Topic Map

Use this map first, then open the smallest relevant section file from `section-index.md`.
The PDF remains authoritative.

| Topic | Primary references | Notes |
|---|---|---|
| SNMP management protocol and IPv6 | `sections/06-ossi-management-protocols.md` | SNMP protocol requirements and IPv6 handling. |
| CableLabs/IETF MIB modules and managed-object requirements | `sections/07-ossi-management-objects.md`, `sections/annex-a-1-mib-object-details.md`, `sections/annex-a-2-if-table-details.md` | MIB support, object access, interface tables, counters and object-level requirements. |
| Fault management, events, traps and syslog | `sections/08-1-fault-management.md`, `sections/annex-c-event-syslog-snmp-notification.md` | Event notification, throttling/priority, event formats, Syslog and SNMP notification behavior. |
| SNMPv3 notification receiver configuration | `sections/08-1-fault-management.md` | Config-file TLV mapping for notification receivers. |
| Configuration management | `sections/08-2-configuration-management.md` | Version/system configuration, secure software download, config files, TLV-11 and MIB OID/value handling. |
| Accounting and performance management | `sections/08-3-accounting-management.md`, `sections/08-4-performance-management.md` | Usage/accounting concepts and MIB counter treatment/interpretation. |
| SNMP security, access control, VACM/USM | `sections/08-5-security-management.md` | CM SNMP modes and access-control configuration. |
| CMCI diagnostics and protocol filtering | `sections/09-ossi-for-cmci.md`, `sections/annex-b-ip-protocol-filtering.md` | CMCI SNMP/console/diagnostics plus LLC/IP filtering and classifiers. |
| CM LED behavior and operational indication | `sections/10-ossi-for-led-indicators.md` | Power-on/self-test, downstream scan, ranging/CM-SG resolution, operational and activity indications. |
| PNM information model | `sections/annex-d-1-overview.md`, `sections/annex-d-2-1-type-definitions.md` | Extended network monitoring, PNM object model and shared data types. |
| Spectrum analysis | `sections/annex-d-2-4-spectrum-analysis.md`, `sections/appendix-i-spectrum-analysis-use-cases.md` | Spectrum analysis controls/results; Appendix I use cases are informative. |
| Downstream symbol capture | `sections/annex-d-2-5-symbol-capture.md` | CmSymbolCapture objects and file format. |
| Channel estimate coefficients | `sections/annex-d-2-6-channel-estimate.md`, `sections/annex-d-4-slope-ripple-algorithms.md` | Channel-estimate object/file format plus slope/ripple and group-delay metrics. |
| Constellation display | `sections/annex-d-2-7-constellation-display.md` | CmDsConstDispMeas objects and file format. |
| Downstream OFDM RxMER | `sections/annex-d-2-8-rxmer.md` | CmDsOfdmRxMer objects, summary/statistics and RxMER file format. |
| OFDM MER margin and required QAM MER | `sections/annex-d-2-9-mer-margin.md`, `sections/annex-d-2-11-required-qam-mer.md` | MER margin and required-QAM-MER management objects. |
| Downstream OFDM FEC summary | `sections/annex-d-2-10-fec-summary.md` | CmDsOfdmFecSummary and file format. |
| Downstream histogram | `sections/annex-d-2-12-downstream-histogram.md` | CmDsHist objects, file format and histogram bin centers. |
| Upstream pre-equalization | `sections/annex-d-2-13-upstream-pre-equalization.md` | CmUsPreEq objects and upstream pre-equalization file formats. |
| Downstream OFDM modulation profile | `sections/annex-d-2-14-modulation-profile.md` | CmDsOfdmModulationProfile object and profile file data. |
| Latency reporting | `sections/annex-d-3-latency-reporting.md` | CmLatencyRpt, latency files, summary data and example. |
| Bulk data transfer | `sections/annex-d-5-bulk-data-transfer.md` | CM bulk-data transfer requirements and objects. |
| DOCSIS data-type definitions | `sections/annex-e-data-type-definitions.md` | General/extended data types and shortened common terms. |
| CM operational status | `sections/annex-f-1-f-2-2-status-definitions.md`, `sections/annex-f-2-3-cm-operational-status.md` | CM registration/status mappings and operational status objects. |
| Downstream interface status/information | `sections/annex-f-3-2-downstream-interface-model.md` | SC-QAM/OFDM channel objects, OFDM profile statistics and channel power. |
| Upstream interface status/information | `sections/annex-f-3-3-upstream-interface-model.md` | SC-QAM/OFDMA channel objects, profile statistics and minislot configuration state. |
| RCC / receive-channel status | `sections/annex-g-2-2-rcc-status.md` | Receive module and receive-channel status information model. |
| QoS configuration objects | `sections/annex-g-2-3-docsis-qos-objects.md` | Classifier, parameter set, aggregate/service-flow and SID-cluster objects. |
| QoS and latency/congestion statistics | `sections/annex-g-2-4-qos-statistics.md` | Service-flow, aggregate, dynamic-service, CM service, latency histogram and congestion statistics. |
| DSID objects and statistics | `sections/annex-g-2-5-dsid-objects.md` | DSID configuration, statistics and client objects. |
| CM provisioning / MAC-domain configuration | `sections/annex-g-2-6-cm-provisioning.md` | MAC-domain configuration, energy-management configuration and CM MAC objects. |
| Tables, object attributes and file formats | Relevant section plus authoritative PDF | Verify layout-sensitive row/column associations, enumerations, ranges, units and object access semantics against the PDF. |
| Cross-spec PHY/MULPI/SEC requirements | Relevant section plus corresponding separate skill | CM-OSSI references PHY I20, MULPI I25 and SEC I11; do not silently import requirements without the corresponding source. |
