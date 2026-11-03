# 8.4 - Performance Management

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 73-73
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 73 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


Service Package, therefore, needs to define the two primary Service Flows using SCNs (e.g., PrimaryUp and
PrimaryDn) that are known to the CMTS if these Service Flows are to be visible to external systems by this billing
interface. Note that it is often the practice in a usage sensitive billing environment to segregate the operator's own
maintenance traffic, to and from the CM, into the primary service flows so that this traffic is not reflected in the
traffic counters associated the subscriber's SLA service flows.
The [MULPIv4.0] specification also provides for dynamically created Service Flows. An example could be a set of
dynamic Service Flows created by an embedded PacketCable Multimedia Terminal Adapter (MTA) to manage VoIP
signaling and media flows. All dynamic Service Flows need to be created using an SCN known to the CMTS if they
are to be visible to the billing system. These dynamic SCNs do not need to appear in the CM Config File but the
MTA may refer to them directly during its own initialization and operation.
During initialization, a CM communicates with a DHCP Server that provides the CM with its assigned IP address
and, in addition, receives a pointer to the Config File server that stores the assigned CM Config File for that CM.
The CM reads the CM Config File and forwards the set of Service Flow definitions (using SCNs) up to the CMTS.
The CMTS then performs a macro-expansion on the SCNs (using its provisioned SCN templates) into QoS
Parameter Sets sent in the Registration Response for the CM. Internally, each active Service Flow is identified by a
32-bit SFID assigned by the CMTS to a specific CM (relative to the RFI interface). For billing purposes, however,
the SFID is not sufficient as the only identifier of a Service Flow because the billing system cannot distinguish the
class of service being delivered by one SFID from another. Therefore, the SCN is necessary, in addition to the SFID,
to identify the Service Flow's class of service characteristics to the billing system.
The billing system can then rate the charges differently for each of the Service Flow traffic counts based on its
Service Class (e.g., Gold octet counts are likely to be charged more than Bronze octet counts). Thus, the billing
system obtains, from the CMTS, the traffic counts for each named Service Flow (identified by SFID and SCN) that a
subscriber's CM uses during the billing data collection interval. This is true even if multiple active Service Flows
(i.e., SFIDs) are created using the same SCN for a given CM over time. This will result in multiple billing records
for the CM for Service Flows that have the same SCN (but different SFIDs). Note that the SFID is the primary key
to the Service Flow. When an active Service Flow exists across multiple sequential billing files, the SFID allows the
sequence of recorded counter values to be correlated to the same Service Flow instance.

8.4     Performance Management
At the CATV MAC and PHY layers, performance management focuses on the monitoring of the effectiveness of
cable plant segmentation and rates of upstream traffic and collisions. Instrumentation is provided in the form of the
standard interface statistics [RFC 2863] and service queue statistics (from [RFC 4546] and Annex G).
At the LLC layer, the performance management focus is on bridge traffic management. The CM implements the
Bridge MIB [RFC 4188] as specified in Section 7.1.3.13 and Annex A.
The DOCS-IF-MIB [RFC 4546] includes variables to track PHY state such as codeword collisions and corruption,
signal-to-noise ratios, transmit and receive power levels, propagation delays, micro-reflections, in channel response,
and sync loss. The DOCS-IF-MIB [RFC 4546] also includes counters to track MAC state, such as collisions and
excessive retries for requests, immediate data transmits, and initial ranging requests. Annex D provides Proactive
Network Maintenance monitoring and diagnostic capabilities for detecting cable plant issues.
A final performance concern is the ability to diagnose unidirectional loss. The CM implements the MIB-II
[RFC 1213] Interfaces Group [RFC 2863] as specified in Section 7.1.3.11 and Annex A.

8.4.1    Treatment and Interpretation of MIB Counters
Octet and packet counters implemented as Counter32 and Counter64 MIB objects are monotonically increasing
positive integers with no specific initial value and a maximum value based on the counter size that will roll-over to
zero when it is exceeded. In particular, counters are defined such that the only meaningful value is the difference
between counter values as seen over a sequence of counter polls. However, there are two situations that can cause
this consistent monotonically increasing behavior to change: (1) resetting the counter due to a system or interface
reinitialization or (2) a rollover of the counter when it reaches its maximum value of 232-1 or 264-1. In these
situations, it needs to be clear what the expected behavior of the counters should be.




04/29/26                                            CableLabs                                                       73
