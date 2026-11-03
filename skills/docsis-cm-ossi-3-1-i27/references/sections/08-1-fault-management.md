# 8.1 - Fault Management

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 46-56
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 46 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


8 OSSI FOR PHY, MAC AND NETWORK LAYERS
8.1        Fault Management
This section defines requirements for remote monitoring/detection, diagnosis, reporting, and correction of problems.
Refer also to Section 7, for requirements for managed objects supporting CM fault management.

8.1.1       SNMP Usage
In the DOCSIS environment, SNMP is used to achieve the goals of fault management: remote detection, diagnosis,
reporting, and correction of CM network faults. Therefore, the CM MUST support SNMP management traffic
across the CATV MAC interfaces as long as the CM has ranged and registered.
The CM SNMP access might be restricted by configuration parameters to support the operator's policy goals. Cable
operators' CM installation personnel can use SNMP queries from a station on the CMCI side to perform on-site CM
and diagnostics and fault classification (note that this may require temporary provisioning of the CM from a local
DHCP server). Further, CMCI side subscriber applications, using SNMP queries, can diagnose simple post-
installation problems, avoiding visits from service personnel and minimizing help desk telephone queries.
The CM sends SNMP notifications to one or more NMSs (subject to operator imposed policy). CM requirements for
SNMP notifications are detailed in Section 8.1.2. The CM sends events to a syslog server. CM requirements for
syslog events are detailed in Section 8.1.2.

8.1.2       Event Notification
A CM is required to generate asynchronous events that indicate malfunction situations and notify about important
events. The methods for reporting events are defined below:
      1.    Stored in Local Log (docsDevEventTable [RFC 4639]).
      2.    Reported to SNMP entities as an SNMP notification.
      3.    Sent as a message to a syslog server.
This specification defines the support of DOCSIS specific events (see Annex C) and IETF events. The former are
normally in the form of SNMP notifications. The delivery of IETF Notifications to local log and syslog server is
optional.
Event Notifications are enabled and disabled by configuration. IETF SNMP notifications normally define specific controls
to enable and disable notifications. For example, see Section 7.1.3.8.4 for requirements on ifLinkUpDownTrapEnable.
DOCSIS specific events can be reported to local log and as syslog message and/or SNMNP notification as defined in
docsDevEvControlTable [RFC 4639], Section 8.1.2.2, and Annex F, Section F.2.3.6. A CM supports event notification
functions including local event logging, syslog (limiting/throttling) and SNMP notification (limiting/throttling), as
specified in [RFC 4639] and this specification. A CM operating in SNMP v1/v2c NmAccess mode is required to
support SNMP trap control as specified in [RFC 4639] and this specification. A CM operating in SNMP
Coexistence mode is required to support SNMP notification functions, as specified in [RFC 3416] and [RFC 3413]
and this specification.

8.1.2.1      Format of Events
Annex C lists all DOCSIS events.
The following sections explain in detail how to report these events by any of the three mechanisms (local event
logging, SNMP notification and syslog).

8.1.2.1.1        Local Event Logging
A CM MUST maintain Local Log events, defined in Annex C, in both local-volatile storage and local non-volatile
storage. A CM MAY retain in local non-volatile storage events designated for local volatile storage. A CM MAY
retain in local volatile storage events designated for local non-volatile storage.




46                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 47 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


A CM MUST implement its Local Log as a cyclic buffer with a minimum of ten entries. The CM Local Log non-
volatile storage events MUST persist across reboots. The CM MUST provide access to the Local Log events
through the docsDevEventTable [RFC 4639].
Section 8.1.2.1.3 describes rules to generate unique EventIds from the error code.
The [RFC 4639] docsDevEvIndex object provides relative ordering of events in the log. The creation of local-
volatile and local non-volatile logs necessitates a method for synchronizing docsDevEvIndex values between the
two Local Logs after reboot. The CM MUST adhere to the rules listed below for creating local volatile and local
non-volatile logs following a re-boot.
The CM MUST clear both the local volatile and local non-volatile event logs when an event log reset is initiated
through an SNMP SET of the docsDevEvControl object [RFC 4639].

8.1.2.1.2     SNMP Notifications
A CM MUST implement the generic SNMP notifications according to Annex A.
When any event causes a generic SNMP notification occurrence in the CM, the CM MUST send notifications if
throttling/limiting mechanisms defined in [RFC 4639] and other limitations [RFC 3413] do not restrict notification
sending.
A CM MUST implement SNMP notifications defined in [DOCS-IF3-MIB].
A CM operating in SNMP v1/v2c NmAccess mode MUST support SNMPv1 and SNMPv2c Traps as defined in
[RFC 3416].
A CM operating in SNMP Coexistence mode MUST support SNMP notification type 'trap' and 'inform' as defined in
[RFC 3416] and [RFC 3413].
The CM MUST send notifications for any event, if docsDevEvControl object [RFC 4639], throttling/limiting
mechanism [RFC 4639] and [RFC 3413] limitations applied later do not restrict notification sending.
The CM MUST NOT report via SNMP notifications vendor-specific events that are not described in instructions
submitted with certification testing application documentation.

8.1.2.1.3     Syslog Message Format
When the CM sends a syslog message for a DOCSIS-defined event, the CM MUST send it in the following format:
<level>CABLEMODEM[vendor]: <eventId> text vendor-specific-text
Where:
    •    level is an ASCII representation of the event priority, enclosed in angle brackets, which is constructed as an
         OR of the default Facility (128) and event priority (0–7). The resulting level ranges between 128 and 135.
    •    TIMESTAMP and HOSTNAME follow the format of [RFC 3164]. The single space after TIMESTAMP is
         part of the TIMESTAMP field. The single space after HOSTNAME is part of the HOSTNAME field.
    •    vendor is the vendor name for the vendor-specific syslog messages or DOCSIS for the standard DOCSIS
         messages.
    •    eventId is an ASCII representation of the INTEGER number in decimal format, enclosed in angle brackets,
         which uniquely identifies the type of event. The CM MUST equate the eventId with the value stored in the
         docsDevEvId object in docsDevEventTable. For the standard DOCSIS events this number is converted
         from the error code using the following rules:
    •    The number is an eight-digit decimal number.
    •    The first two digits (left-most) are the ASCII code for the letter in the Error code.
    •    The next four digits are filled by 2 or 3 digits between the letter and the dot in the Error code with zero
         filling in the gap in the left side.




02/19/25                                             CableLabs®                                                        47
<!-- PDF_PAGE: 48 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


     •    The last two digits are filled by the number after the dot in the Error code with zero filling in the gap in the
          left side.
For example, event D04.2 is converted into 68000402, and Event I114.1 is converted into 73011401. This
convention only uses a small portion of available number space reserved for DOCSIS (0 to 231-1). The first letter of
an error code is always in upper-case. See Annex C for event definitions.
     •    text contains the textual description for the standard DOCSIS event message, as defined in Annex C.
     •    vendor-specific-text contains vendor-specific information. This field is optional.
For example, the syslog event for the event D04.2, "ToD Response received - Invalid data format", is as follows:
<132>CABLEMODEM[DOCSIS]: <68000402> ToD Response received - Invalid data format

The number 68000402 in the example is the number assigned by DOCSIS to this particular event.
The CM MAY report non-DOCSIS events in the standard syslog message format [RFC 3164] rather than the
DOCSIS syslog message format defined above. When the CM sends a syslog message for an event not defined in
this specification, the CM MAY send it according to the format and semantics of the elements defined above.

8.1.2.2    BIT Values for docsDevEvReporting (RFC 4639)
Permissible BIT values for [RFC 4639] docsDevEvReporting objects include:
     1: local(0)
     2: traps(1)
     3: syslog(2)
     4: localVolatile(8)
     5: stdInterface(9)
Bit-0 means non-volatile Local Log storage and bit-8 is used for volatile Local Log storage (see Section 8.1.2.1).
Bit-1 means SNMP Notifications which correspond to both SNMP Trap and SNMP Inform.
For backward compatibility with Pre-3.0 DOCSIS devices, the CM MUST support bit-3 in docsDevEvReporting
BITS encoding for volatile Local Log storage.
DOCSIS 3.0 devices need to support bit override mechanisms during SNMP SET operations with either one-byte or
two-byte BITS encoding for docsDevEvReporting for backward compatibility with Pre-3.0 DOCSIS behavior.
The CM MUST use the bit-3 value to set both bit-3 and bit-8 for SNMP SET operations on docsDevEvReporting
using a one-byte BITS encoded value. Therefore, the CM reports bit-3 and bit-8 with identical values for SNMP
GET operations.
The CM MUST use the bit-8 value to set both bit-3 and bit-8 for SNMP SET operations, irrespective of the bit-3
value, on docsDevEvReporting using a two or more byte BITS encoded value.
The CM MAY support bit-9 in docsDevEvReporting BITS encoding in accordance with [RFC 4639] definition.
A CM that reports an event by SNMP Notification or syslog MUST also report the event by a Local Log (volatile or
non-volatile).
Combinations of docsDevEvReporting with traps(1) and/or syslog(2) bits with no Local Log bits (bit-0, bit-3 or bit-
8) set are known as unacceptable combinations.
The CM MUST reject and report a 'Wrong Value' error for SNMPv2c/v3 PDUs or a 'Bad Value' error for SNMPv1
PDUs for any attempt to set docsDevEvReporting with unacceptable combinations.
The CM MUST accept any SNMP SET operation to docsDevEvReporting different than the unacceptable
combinations.
The CM MUST ignore any undefined bits in docsDevEvReporting on SNMP SET operations and report a zero value
for those bits.



48                                                    CableLabs®                                                02/19/25
<!-- PDF_PAGE: 49 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


Refer to Section 8.1.2.1.1 for details on Local Log requirements for the CM.
The CM MUST maintain the non-volatile storage when both non-volatile Local Log and volatile Local Log bits are
set for a specific docsDevEvReporting event priority. The CM MAY maintain the volatile storage when both non-
volatile Local Log and volatile Local Log bits are set for a specific docsDevEvReporting event priority. When both
non-volatile Local Log and volatile Local Log bits are set for a specific docsDevEvReporting event priority, the CM
MUST NOT report duplicate events in the docsDevEventTable.

8.1.2.3    Standard DOCSIS Events for CMs
Aside from the procedures defined in this document, event recording conforms to the requirements of [RFC 4639].
Event descriptions are defined in English. A CM MUST implement event descriptors such that no event descriptor is
longer than 255 characters, which is the maximum defined for SnmpAdminString [RFC 3411].
Events are identical if their EventIds are identical. For identical events occurring consecutively, the CM MAY
choose to store only a single event. If a CM stores as a single event multiple identical events that occur
consecutively, the CM MUST reflect in the event description the most recent event.
The EventId digit is a 32-bit unsigned integer. EventIds ranging [RFC 4639] from 0 to (231 - 1) are reserved by
DOCSIS. The CM MUST report in the docsDevEvTable [RFC 4639] the EventId as a 32-bit unsigned integer and
convert the EventId from the error codes defined in Annex C to be consistent with this number format.
The DOCS-CABLE-DEVICE-MIB [RFC 4639] defines 8 priority levels and a corresponding reporting mechanism
for each level.
Emergency event (priority 1)
    Reserved for vendor-specific 'fatal' hardware or software errors that prevent normal system operation and cause
    the reporting system to reboot.
    Every vendor may define their own set of emergency events. Examples of such events might be 'no memory
    buffers available', 'memory test failure', etc.
Alert event (priority 2)
    A serious failure, which causes the reporting system to reboot, but it is not caused by hardware or software
    malfunctioning.
Critical event (priority 3)
    A serious failure that requires attention and prevents the device from transmitting data, but could be recovered
    without rebooting the system. Examples of such events might be configuration file problems detected by the
    modem or the inability to get an IP address from the DHCP server.
Error event (priority 4)
    A failure occurred that could interrupt the normal data flow, but will not cause the modem to re-register. Error
    events could be reported in real time by using the trap or syslog mechanism.
Warning event (priority 5)
    A failure occurred that could interrupt the normal data flow, but will not cause the modem to re-register.
    'Warning' level is assigned to events that both CM and CMTS have information about. To prevent sending the
    same event, both from the CM and the CMTS, the trap and syslog reporting mechanism is disabled by default
    for the CM for this level.
Notice event (priority 6)
    The event is important, but is not a failure and could be reported in real time by using the trap or syslog
    mechanism. For a CM, an example of a Notice event is any event from 'SW UPGRADE SUCCESS' group.
Informational event (priority 7)
    The event is of marginal importance, and is not failure, but could be helpful for tracing the normal modem
    operation.



02/19/25                                           CableLabs®                                                      49
<!-- PDF_PAGE: 50 -->
CM-SP-CM-OSSIv3.1-I27-250219                                        Data-Over-Cable Service Interface Specifications


Debug event (priority 8)
     Reserved for vendor-specific non-critical events.
During CM initialization or reinitialization, the CM MUST support, as a minimum, the default event reporting
mechanism shown in Table 15 - CM Default Event Reporting Mechanism Versus Priority.
The CM MAY implement default reporting mechanisms above the minimum requirements listed in Table 15 - CM
Default Event Reporting Mechanism Versus Priority.
The reporting mechanism for each priority could be changed from the default reporting mechanism by using
docsDevEvReporting object of DOCS-CABLE-DEVICE-MIB [RFC 4639].
The CM MUST populate the code of an event (as defined in Annex C) with Critical or Alert event priority through
the docsIf3CmStatusCode SNMP object of DOCS-IF3-MIB before it recovers from the event. The CM MUST
persist the docsIf3CmStatusCode across system reinitializations.
                         Table 15 - CM Default Event Reporting Mechanism Versus Priority

     Event Priority                Local Log                 SNMP                    Syslog           Local Log
                                  Non-volatile             Notification                                Volatile
 Emergency                Yes                        No                        No               No
 Alert                    Yes                        No                        No               No
 Critical                 Yes                        No                        No               No
 Error                    No                         Yes                       Yes              Yes
 Warning                  No                         No                        No               Yes
 Notice                   No                         Yes                       Yes              Yes
 Informational            No                         No                        No               No
 Debug                    No                         No                        No               No


The CM MUST format notifications that it generates for standard DOCSIS events as specified in Annex C.

8.1.2.4     Vendor-Specific Events
The CM MUST implement EventIds ranging from 231 to (232 - 1) as vendor-specific EventIds using the following
format:
Bit 31 is set to indicate vendor-specific event
Bits 30-16 contain the lower 15 bits of the vendor's SNMP enterprise number
Bits 15-0 are used by the vendor to number events

8.1.2.4.1        Event Priorities for DOCSIS and Vendor-Specific Events
A CM MUST assign DOCSIS and vendor-specific events as indicated in Table 16 - Event Priority Assignment for
CMs.
                                       Table 16 - Event Priority Assignment for CMs

                           Event Priority                       CM Event Assignment

                       Emergency                  Vendor-specific
                       Alert                      DOCSIS and Vendor-specific (optional*)
                       Critical                   DOCSIS and Vendor-specific (optional*)
                       Error                      DOCSIS and Vendor-specific (optional*)
                       Warning                    DOCSIS and Vendor-specific (optional*)
                       Notice                     DOCSIS and Vendor-specific (optional*)




50                                                      CableLabs®                                         02/19/25
<!-- PDF_PAGE: 51 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


                         Event Priority                       CM Event Assignment

                     Informational             DOCSIS and Vendor-specific (optional*)
                     Debug                     Vendor-specific
                     *Table Note: Vendor-specific optional event definitions are recommended only
                     where the CM allows for sufficient storage of such events.



8.1.3     Throttling, Limiting and Priority for Event, Trap and Syslog

8.1.3.1    Trap and Syslog Throttling, Trap and Syslog Limiting
A CM MUST support SNMP TRAP/INFORM and syslog throttling and limiting as described in DOCS-CABLE-
DEVICE-MIB [RFC 4639], regardless of SNMP mode.

8.1.4     SNMPv3 Notification Receiver Config File TLV
This section specifies processing requirements for the SNMPv3 Notification Receiver TLV [MULPIv3.1] when
present in the configuration file. The SNMPv3 Notification Receiver TLV is used to configure SNMPv3 tables for
notification transmission. The CM MUST process the SNMPv3 Notification Receiver TLV only if the CM is in
SNMPv3 Coexistence Mode.
Based on the SNMPv3 Notification Receiver TLV, the CM MUST create entries in the following tables in order to
cause the desired trap transmission:
    •     snmpNotifyTable
    •     snmpTargetAddrTable
    •     snmpTargetAddrExtTable
    •     snmpTargetParamsTable
    •     snmpNotifyFilterProfileTable
    •     snmpNotifyFilterTable
    •     snmpCommunityTable
    •     usmUserTable
    •     vacmContextTable
    •     vacmSecurityToGroupTable
    •     vacmAccessTable
    •     vacmViewTreeFamilyTable
The CM MUST NOT set to 'active' an entry created using the SNMPv3 Notification Receiver TLV (see the
Common Radio Frequency Interface Encodings Annex of [MULPIv3.1]) which does not satisfy the corresponding
[RFC 3413] requirements to do so. This type of misconfiguration does not stop the CM from registering; however,
the SNMP notification process may not work as expected.
The mapping from the TLV to these tables is described in the following section.

8.1.4.1    Mapping of TLV Fields into Created SNMPv3 Table Rows
The following sections illustrate how the fields from the config file SNMPv3 Notification Receiver TLV elements
are placed into the SNMPv3 tables. The TLV fields are shown below as:




02/19/25                                             CableLabs®                                               51
<!-- PDF_PAGE: 52 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                Data-Over-Cable Service Interface Specifications


                               Table 17 - SNMPv3 Notification Receiver TLV Mapping

                       Sub-TLVs                                   Variable Name                Associated MIB Object
 SNMPv3 Notification Receiver IPv4 Address                        TAddress            snmpTargetAddrTAddress [RFC 3413]
 SNMPv3 Notification Receiver IPv6 Address                        TAddress            snmpTargetAddrTAddress [RFC 3413]
 SNMPv3 Notification Receiver UDP Port Number                     Port                snmpTargetAddrTAddress [RFC 3413]
 SNMPv3 Notification Receiver Trap Type                           TrapType            see following sections
 SNMPv3 Notification Receiver Timeout                             Timeout             snmpTargetAddrTimeout [RFC 3413]
 SNMPv3 Notification Receiver Retries                             Retries             snmpTargetAddrRetryCount [RFC 3413]
 SNMPv3 Notification Receiver Filtering Parameters                FilterOID           see following sections
 SNMPv3 Notification Receiver Security Name                       SecurityName        see following sections


The variable names from Table 17 are defined as follows:
     <TAddress>A 32-bit IPv4 or IPv6 address of a notification receiver
     <Port>A 16-bit UDP Port number on the notification receiver to receive the notifications
     <TrapType>Defines the notification type as explained above
     <Timeout>16-bit timeout, in milliseconds to wait before sending a retry of an Inform Notification
     <Retries>16-bit number of times to retry an Inform after the first Inform transmission
     <FilterOID>The OID of the snmpTrapOID value that is the root of the MIB subtree that defines all of the
     notifications to be sent to the Notification Receiver.
     <SecurityName>The security name specified on the TLV element, or "@config" if not specified.
Table 18 through Table 29 are shown in the order that the agent will search down through them when a notification
is generated in order to determine to whom to send the notification, and how to fill out the contents of the
notification packet.
In configuring entries in these SNMPv3 tables, note the following:
The Community Name for traps in SNMPv1 and SNMPv2 packets is configured as "public". The Security Name in
traps and informs in SNMPv3 packets where no security name has been specified is configured as "@config", in
which case the security level is "noAuthNoPriv".
Several columnar objects are configured with a value beginning with the string "@config". If these tables are
configured through other mechanisms, network operators should not use values beginning with "@config" to avoid
conflicts with the mapping process specified here.

8.1.4.1.1     snmpNotifyTable
The snmpNotifyTable is defined in the "Notification MIB Module" section of [RFC 3413].
The CM MUST create two rows with fixed values if one or more SNMPv3 Notification Receiver TLV elements are
present in the config file.
                                              Table 18 - snmpNotifyTable

      Column Name (* = Part of Index)                     1st Row Column Value                      2nd Row Column Value
 * snmpNotifyName                                    "@config_inform"                          "@config_trap"
 snmpNotifyTag                                       "@config_inform"                          "@config_trap"
 snmpNotifyType                                      inform (2)                                trap (1)
 snmpNotifyStorageType                               volatile (2)                              volatile (2)
 snmpNotifyRowStatus                                 active (1)                                active (1)




52                                                        CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 53 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


8.1.4.1.2    snmpTargetAddrTable
The snmpTargetAddrTable is defined in the "Definitions" section of [RFC 3413].
The CM MUST create one row in snmpTargetAddrTable for each entry defined in Table 19 -
snmpTargetAddrTable.
Thus, two entries are created in this table if both SNMPv3 Notification Receiver IPv4 Address and SNMPv3
Notification Receiver IPv6 Address sub-TLVs are included in the same TLV. All other parameters are the same.
                                          Table 19 - snmpTargetAddrTable

    Column Name (* = Part of Index)                                         Column Value
 * snmpTargetAddrName                       "@config_n_IPv[4 | 6]" where n is 0..m-1 and m is the number of SNMPv3
                                            Notification Receiver config file TLVs
                                            @config_n_IPv4 is for an entry created if SNMPv3 Notification Receiver config
                                            file TLV contains <TrapType> of TDomain SnmpUDPAddress
                                            @config_n_IPv6 is for an entry created if SNMPv3 Notification Receiver config
                                            file TLV contains <TrapType> of TDomain TransportAddressIPv6
 snmpTargetAddrTDomain                      IPv4: snmpUDPDomain [RFC 3417]
                                            IPv6: transportDomainUdpIpv6 [RFC 3419]
 snmpTargetAddrTAddress (IP Address and     IPv4: SnmpUDPAddress [RFC 3417]
 UDP Port of the Notification Receiver)     OCTET STRING (6) Octets 1-4: <TAddress> Octets 5-6: <Port>
                                            IPv6: TransportAddressIPv6 [RFC 3419]
                                            OCTET STRING (18) Octets 1-16: <TAddress> Octets 17-18: <Port>
 snmpTargetAddrTimeout                      <Timeout>
 snmpTargetAddrRetryCount                   <Retries>
 snmpTargetAddrTagList                      "@config_trap" if <TrapType> is 1, 2, or 4
                                            "@config_inform" if <TrapType> is 3 or 5
 snmpTargetAddrParams                       "@config_n"
 snmpTargetAddrStorageType                  volatile (2)
 snmpTargetAddrRowStatus                    active (1)



8.1.4.1.3    snmpTargetAddrExtTable
The snmpTargetAddrExtTable is defined in the "SNMP Community MIB Module" section of [RFC 3584].
The CM MUST create one row in snmpTargetAddrExtTable for each entry defined in Table 19 -
snmpTargetAddrTable.
                                      Table 20 - snmpTargetAddrExtTable

    Column Name (* = Part of Index)                                         Column Value
 * snmpTargetAddrName                       "@config_n_IPv[4 | 6]" where n is 0..m-1 and m is the number of SNMPv3
                                            Notification Receiver config file TLVs (see Table 19 for details).
 snmpTargetAddrTMask                        <Zero-length OCTET STRING>
 snmpTargetAddrMMS                          SM Maximum Message Size



8.1.4.1.4    snmpTargetParamsTable
The snmpTargetParamsTable is defined in the "Definitions" section of [RFC 3413].
The CM MUST create one row in snmpTargetParamsTable for each SNMPv3 Notification Receiver TLV in the
config file.




02/19/25                                             CableLabs®                                                             53
<!-- PDF_PAGE: 54 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


                                       Table 21 - snmpTargetParamsTable

     Column Name (* = Part of Index)                                       Column Value
 * snmpTargetParamsName                   "@config_n" where n is 0..m-1 and m is the number of SNMPv3 Notification
                                          Receiver config file TLVs
 snmpTargetParamsMPModel                  SNMPv1 (0) if <TrapType> is 1
 SYNTAX: SnmpMessageProcessingModel       SNMPv2c (1) if <TrapType> is 2 or 3
                                          SNMPv3 (3) if <TrapType> is 4 or 5
 snmpTargetParamsSecurityModel            SNMPv1 (1) if <TrapType> is 1
 SYNTAX: SnmpSecurityModel                SNMPv2c (2) if <TrapType> is 2 or 3
                                          USM (3) if <TrapType> is 4 or 5
                                          Note: The mapping of SNMP protocol types to value here are different from
                                          snmpTargetParamsMPModel
 snmpTargetParamsSecurityName             If <TrapType> is 1, 2, or 3, or if the <Security Name> field is zero-length:
                                          "@config"
                                          If <TrapType> is 4 or 5, and the <Security Name> field is non-zero length:
                                          <SecurityName>
 snmpTargetParamsSecurityLevel            If <TrapType> is 1, 2, or 3, or if the <Security Name> field is zero-length:
                                          noAuthNoPriv (1)
                                          If <TrapType> is 4 or 5, and the <Security Name> field is non-zero length:
                                          The security level of <SecurityName>
 snmpTargetParamsStorageType              volatile (2)
 snmpTargetParamsRowStatus                active (1)



8.1.4.1.5      snmpNotifyFilterProfileTable
The snmpNotifyFilterProfileTable is defined in the "Notification MIB Module" section of [RFC 3413].
The CM MUST create one row in snmpNotifyFilterProfileTable for each SNMPv3 Notification Receiver TLV that
has a non-zero <FilterOID>.
                                    Table 22 - snmpNotifyFilterProfileTable

     Column Name (* = Part of Index)                                      Column Value
 * snmpTargetParamsName                  "@config_n" where n is 0..m-1 and m is the number of SNMPv3 Notification
                                         Receiver config file TLVs
 snmpNotifyFilterProfileName             "@config_n" where n is 0..m-1 and m is the number of SNMPv3 Notification
                                         Receiver config file TLVs
 snmpNotifyFilterProfileStorType         volatile (2)
 snmpNotifyFilterProfileRowStatus        active (1)



8.1.4.1.6      snmpNotifyFilterTable
The snmpNotifyFilterTable is defined in the "Notification MIB Module" section of [RFC 3413].
The CM MUST create one row in snmpNotifyFilterTable for each SNMPv3 Notification Receiver TLV that has a
non-zero <FilterOID>.
                                        Table 23 - snmpNotifyFilterTable

     Column Name (* = Part of Index)                                      Column Value
 * snmpNotifyFilterProfileName           "@config_n" where n is 0..m-1 and m is the number of SNMPv3 Notification
                                         Receiver config file TLVs
 * snmpNotifyFilterSubtree               <FilterOID>
 snmpNotifyFilterMask                    <Zero-length OCTET STRING>
 snmpNotifyFilterType                    included (1)




54                                                    CableLabs®                                                   02/19/25
<!-- PDF_PAGE: 55 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv3.1-I27-250219


   Column Name (* = Part of Index)                                         Column Value
 snmpNotifyFilterStorageType               volatile (2)
 snmpNotifyFilterRowStatus                 active (1)



8.1.4.1.7     snmpCommunityTable
The snmpCommunityTable is defined in the "SNMP Community MIB Module" section of [RFC 3584].
The CM MUST create one row in snmpCommunityTable with fixed values if one or more SNMPv3 Notification
Receiver TLVs are present in the config file. This causes SNMPv1 and v2c notifications to contain the community
string in snmpCommunityName.
                                         Table 24 - snmpCommunityTable

   Column Name (* = Part of Index)                                         Column Value
 * snmpCommunityIndex                      "@config"
 snmpCommunityName                         "public"
 snmpCommunitySecurityName                 "@config"
 snmpCommunityContextEngineID              <the engineID of the cable modem>
 snmpCommunityContextName                  <Zero-length OCTET STRING>
 snmpCommunityTransportTag                 <Zero-length OCTET STRING>
 snmpCommunityStorageType                  volatile (2)
 snmpCommunityStatus                       active (1)



8.1.4.1.8     usmUserTable
The usmUserTable is defined in the "Definitions" section of [RFC 3414].
The CM MUST create one row in usmUserTable with fixed values if one or more SNMPv3 Notification Receiver
TLVs are present in the config file. Other rows are created, one each time the engine ID of a trap receiver is
discovered. This specifies the user name on the remote notification receivers to which notifications are to be sent.
One row in the usmUserTable is created. When the engine ID of each notification receiver is discovered, the agent
copies this row into a new row and replaces the 0x00 in the usmUserEngineID column with the newly discovered
value.
                                              Table 25 - usmUserTable

   Column Name (* = Part of Index)                                         Column Value
 * usmUserEngineID                         0x00
 * usmUserName                             "@config"
                                           When other rows are created, this is replaced with the <SecurityName> field from
                                           the SNMPv3 Notification Receiver config file TLV.
 usmUserSecurityName                       "@config"
                                           When other rows are created, this is replaced with the <SecurityName> field from
                                           the SNMPv3 Notification Receiver config file TLV.
 usmUserCloneFrom                          <don't care> This row cannot be cloned.
 usmUserAuthProtocol                       None
                                           When other rows are created, this is replaced with None or MD5, depending on the
                                           security level of the V3 User.
 usmUserAuthKeyChange                      <don't care>
                                           Write-only
 usmUserOwnAuthKeyChange                   <don't care>
                                           Write-only




02/19/25                                                CableLabs®                                                        55
<!-- PDF_PAGE: 56 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


     Column Name (* = Part of Index)                                     Column Value
 usmUserPrivProtocol                     None
                                         When other rows are created, this is replaced with None or DES, depending on the
                                         security level of the V3 User.
 usmUserPrivKeyChange                    <don't care>
                                         Write-only
 usmUserOwnPrivKeyChange                 <don't care>
                                         Write-only
 usmUserPublic                           <Zero-length OCTET STRING>
 usmUserStorageType                      volatile (2)
 usmUserStatus                           active (1)



8.1.4.1.9     vacmContextTable
The vacmContextTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create one row in vacmContextTable with the zero length octet string for vacmContextName object.
                                         Table 26 - vacmContextTable

     Column Name (* = Part of Index)                                     Column Value
 * vacmContextName                        <Zero-length OCTET STRING>



8.1.4.1.10    vacmSecurityToGroupTable
The vacmSecurityToGroupTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create three rows in vacmSecurityToGroupTable with fixed values if one or more SNMPv3
Notification Receiver TLVs are present in the config file.
Table 27 depicts the three rows with fixed values which are used for the SNMPv3 Notification Receiver TLV entries
with <TrapType> set to 1, 2, or 3, or with a zero-length <SecurityName>. The SNMPv3 Notification Receiver TLV
entries with <TrapType> set to 4 or 5 and a non-zero length <SecurityName> will use the rows created in the
vacmSecurityToGroupTable by the DH Kickstart process.
                                    Table 27 - vacmSecurityToGroupTable

               Column Name                         First Row              Second Row                  Third Row
             (* = Part of Index)                 Column Value             Column Value               Column Value
 * vacmSecurityModel                        SNMPV1 (1)                SNMPV2c (2)                USM (3)
 * vacmSecurityName                         "@config"                 "@config"                  "@config"
 vacmGroupName                              "@configV1"               "@configV2"                "@configUSM"
 vacmSecurityToGroupStorageType             volatile (2)              volatile (2)               volatile (2)
 vacmSecurityToGroupStatus                  active (1)                active (1)                 active (1)



8.1.4.1.11    vacmAccessTable
The vacmAccessTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create three rows in vacmAccessTable with fixed values if one or more SNMPv3 Notification
Receiver TLVs are present in the config file.
Table 28 depicts the three rows with fixed values which are used for the SNMPv3 Notification Receiver TLV entries
with <TrapType> set to 1, 2, or 3, or with a zero-length <SecurityName>. The SNMPv3 Notification Receiver TLV
entries with <TrapType> set to 4 or 5 and a non-zero length <SecurityName> will use the rows created in the
vacmAccessTable by the DH Kickstart process.



56                                                    CableLabs®                                                02/19/25
