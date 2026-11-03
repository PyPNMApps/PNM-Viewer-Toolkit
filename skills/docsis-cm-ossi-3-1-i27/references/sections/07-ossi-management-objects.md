# 7 - OSSI Management Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 32-45
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 32 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


7 OSSI MANAGEMENT OBJECTS
7.1       SNMP Management Information Bases (MIBS)
This section defines the minimum set of managed objects required to support the management of a CM.
The CM MAY augment the required MIBs with objects from other standard or vendor-specific MIBs where
appropriate.
The DOCSIS 3.1 Cable Modem OSSI specification has priority over the IETF MIBs and all objects. Though
deprecated or optional in the IETF MIB, the object can be required by this specification as mandatory.
The CM MUST implement the MIB requirements in accordance with this specification regardless of the value of an
IETF MIB object's status (e.g., deprecated or optional).
If not required by this specification, deprecated objects are optional. If a CM implements a deprecated MIB object,
the CM MUST implement the MIB object according to the MIB definition.
If a CM does not implement a deprecated MIB object, the following conditions MUST be met:
      •    The CM MUST NOT instantiate the deprecated MIB object.
      •    The CM MUST respond with the appropriate error/exception condition, such as noSuchObject for
           SNMPv2c, when an attempt to access the deprecated MIB object is made.
If not required by this specification, additional objects are optional. If a CM implements any additional MIB objects,
the CM MUST implement the MIB object according to the MIB definition.
If a CM does not implement one or more additional MIB objects, the following conditions MUST be met:
      •    The CM MUST NOT instantiate the additional MIB object or objects.
      •    The CM MUST respond with the appropriate error/exception condition, such as noSuchObject for
           SNMPv2c when an attempt to access the non-existent additional MIB object is made, when the additional
           MIB object or objects are accessed.
If not required by this specification, obsolete objects are optional. If a CM implements an obsolete MIB object, the
CM MUST implement the MIB object according to the MIB definition.
If a CM does not implement an obsolete MIB object, the following conditions MUST be met:
      •    The CM MUST NOT instantiate the obsolete MIB object.
      •    The CM MUST respond with the appropriate error/exception condition, such as noSuchObject for
           SNMPv2c, when an attempt to access the obsolete MIB object is made.
Objects which are not supported by this specification are not implemented by an agent.
      •    The CM MUST NOT instantiate MIB objects listed as not supported in Annex A.
      •    The CM MUST respond with the appropriate error/exception condition, such as noSuchObject for
           SNMPv2c, when an attempt to access a not supported MIB object is made.
Sections 7.1.1 and 7.1.2 include an overview of the MIB modules required for management of the facilities specified
in the [MULPIv3.1] and [SECv3.1] specifications.




32                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 33 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


7.1.1   CableLabs MIB Modules
The following CableLabs MIB Modules are normatively required for CMs by this specification.
                                        Table 7 - CableLabs MIB Modules

                     Reference                                      MIB Module
              [DOCS-BPI2EXT-MIB]          DOCSIS Security MIB Module BPI+ Extension for DOCSIS 3.1 CM:
                                          DOCS-BPI2EXT-MIB
              [DOCS-IFEXT2-MIB]           DOCSIS Interface Extension 2 MIB Module:
                                          DOCS-IFEXT2-MIB
              [DOCS-IF3-MIB]              DOCSIS Interface 3 MIB Module:
                                          DOCS-IF3-MIB
              [DOCS-IF31-MIB]             DOCSIS Interface 3.1 MIB Module:
                                          DOCS-IF31-MIB
              [DOCS-PNM-MIB]              DOCSIS Proactive Network Maintenance MIB Module:
                                          DOCS-PNM-MIB
              [DOCS-QOS3-MIB]             DOCSIS Quality of Service 3 MIB Module:
                                          DOCS-QOS3-MIB
              [DOCS-SEC-MIB]              DOCSIS Security MIB Module:
                                          DOCS-SEC-MIB



7.1.2   IETF RFC MIB Modules
                                         Table 8 - IETF RFC MIB Modules

                 Reference                                       MIB Module
              [RFC 2786]          Diffie-Helman USM Key MIB Module:
                                  SNMP-USM-DH-OBJECTS-MIB
              [RFC 2790]          Host Resources MIB Module:
                                  HOST-RESOURCES-MIB
              [RFC 2863]          Interfaces Group MIB Module:
                                  IF-MIB
              [RFC 2933]          Internet Group Management Protocol MIB Module:
                                  IGMP-STD-MIB
              [RFC 3410]          SNMPv3 MIB Modules:
              [RFC 3411]          SNMP-FRAMEWORK-MIB, SNMP-MPD-MIB, SNMP-NOTIFICATION-MIB,
              [RFC 3412]          SNMP-TARGET-MIB, SNMP-USER-BASED-SM-MIB, SNMP-VIEW-
              [RFC 3413]          BASED-ACM-MIB, SNMP-COMMUNITY-MIB
              [RFC 3414]
              [RFC 3415]
              [RFC 3584]
              [RFC 3418]          SNMPv2 MIB Module:
                                  SNMPv2-MIB
              [RFC 3433]          Entity Sensor MIB Module:
                                  ENTITY-SENSOR-MIB
              [RFC 3635]          Ethernet Interface MIB Module:
                                  EtherLike-MIB
              [RFC 4022]          Transmission Control Protocol MIB Module:
                                  TCP-MIB
              [RFC 4113]          User Datagram Protocol MIB Module:
                                  UDP-MIB
              [RFC 4131]          DOCSIS Baseline Privacy Plus MIB Module:
                                  DOCS-IETF-BPI2-MIB




02/19/25                                            CableLabs®                                              33
<!-- PDF_PAGE: 34 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


                  Reference                                       MIB Module
                [RFC 4188]        Bridge MIB Module:
                                  BRIDGE-MIB
                [RFC 4293]        Internet Protocol MIB Module:
                                  IP-MIB
                [RFC 4546]        DOCSIS RF MIB Module:
                                  DOCS-IF-MIB
                [RFC 4639]        DOCSIS Device MIB Module:
                                  DOCS-CABLE-DEVICE-MIB
                [RFC 6933]        Entity MIB Module:
                                  ENTITY-MIB



7.1.3     Managed Objects Requirements
The following sections detail additional implementation requirements for the MIB modules listed.
The CM MUST implement the compliance and syntax of the MIB objects as specified in Annex A.
The CM MUST implement the MIB access requirements defined in Annex A as follows:
     •    MIB objects with Not-Accessible (N-Acc) access type are implemented with not-accessible access and are
          typically indexes in MIB tables.
     •    MIB objects with Read-Create (RC) access type are implemented with read-create access.
     •    MIB objects with Read-Write (RW) access type are implemented with read-write access.
     •    MIB objects with Read-Only (RO) access type are implemented with read-only access.
     •    MIB objects with Read-Create (RC) access type are implemented with read-create access.
     •    MIB objects with Read-Create (RC) or Read-Only (RO) access types are implemented with either read-
          create access or read-only access as described in the object.
     •    MIB objects with Read-Write (RW) or Read-Write (RO) access types are implemented with either read-
          write access or read-only access as described in the object.
     •    MIB objects with Accessible for SNMP Notification (Acc-FN) access type are implemented as SNMP
          Notifications or Traps.
The CM MUST support a minimum of 10 available SNMP table rows, unless otherwise specified by RFC or
DOCSIS specification. The CM minimum number of available SNMP table rows SHOULD mean rows (per table)
that are available to support device configuration. The CM used (default) SNMP table row entries MUST NOT
apply to the minimum number of available SNMP table rows.

7.1.3.1    Requirements for CableLabs DOCSIS Interface Extension 2 MIB
The CM MUST implement DOCS-IFEXT2-MIB, as specified in [DOCS-IFEXT2-MIB].

7.1.3.2    Requirements for CableLabs DOCSIS Interface 3 MIB
The CM MUST implement the DOCS-IF3-MIB, as specified in [DOCS-IF3-MIB].

7.1.3.3    Requirements for CableLabs DOCSIS Interface 3.1 MIB
The CM MUST implement the DOCS-IF31-MIB, as specified in [DOCS-IF31-MIB].

7.1.3.4    Requirements for CableLabs DOCSIS Proactive Network Maintenance MIB
The CM MUST implement the DOCS-PNM-MIB, as specified in [DOCS-PNM-MIB].




34                                                     CableLabs®                                        02/19/25
<!-- PDF_PAGE: 35 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


7.1.3.5    Requirements for CableLabs DOCSIS Quality of Service 3 MIB
The CM MUST implement the DOCS-QOS3-MIB, as specified in [DOCS-QOS3-MIB].

7.1.3.6    Requirements for DOCSIS Device MIB (RFC 4639)
The CM MUST implement [RFC 4639].
Note:     [RFC 4639] includes Compliance requirements for DIFFSERV-MIB [RFC 3289] to support IPv6 filtering as a
          replacement for the deprecated docsDevFilterIpTable. For backwards compatibility, this specification has
          requirements for docsDevFilterIpTable. IPv6 filtering requirements are specified in Annex A. This
          specification does not define requirements for [RFC 3289].
Additional requirements affecting [RFC 4639] are also found in Section 9.4.

7.1.3.7    Requirements for DOCSIS RF MIB (RFC 4546)
The CM MUST implement [RFC 4546]. However, much of [RFC 4546] is not applicable to OFDM/OFDMA
channels. Thus, this section defines separate requirements for handling both SC-QAM and OFDM/OFDMA
channels.
The CM MUST instantiate a row entry for all SC-QAM and OFDM channels in the docsIfDownChannelTable. The
CM MUST return appropriate values for all columns of the docsIfDownChannelTable for SC-QAM channels as
described in the MIB itself and further specified in this section.
OFDM channels are defined and configured differently than SC-QAM channels. Thus, the
docsIfDownChannelTable cannot properly represent OFDM channels. However, it is useful for the NMS to have
some representation of OFDM channels in the docsIfDownChannelTable as an indication that the channel exists and
that more information can be found in other tables. Thus, rules are defined for OFDM channels to provide standard
data via the docsIfDownChannelTable.
The CM MUST report the following values (Table 9 - docsIfDownChannelTable Requirements for OFDM
Channels) for OFDM channel row entries in the docsIfDownChannelTable:
                     Table 9 - docsIfDownChannelTable Requirements for OFDM Channels

                                        MIB Object                            Value
                           docsIfDownChannelFrequency              0
                           docsIfDownChannelWidth                  0
                           docsIfDownChannelModulation             other(2)
                           docsIfDownChannelInterleave             other(2)
                           docsIfDownChannelPower                  0
                           docsIfDownChannelAnnex                  other(2)
                           docsIfDownChannelStorageType            other(2)


For SC-QAM channels, the CM MUST implement the docsIfDownChannelPower MIB object with read-only
access. For SC-QAM channels, the CM MUST report a power value for docsIfDownChannelPower within 3 dB of
the actual received channel power when operated at nominal line-voltage, at normal room temperature (refer to
[PHYv3.1]).
On SC-QAM channels, for any 1 dB change in input power, the CM MUST report a power change in the same
direction that is not less than 0.6 dB and not more than 1.4 dB, as specified in [PHYv3.0].
Similarly, upstream OFDMA channels cannot be represented properly in the docsIfUpChannelTable. Thus, for
OFDMA channels, the CM MUST report the following (Table 10 - docsIfUpChannelTable Requirements for
OFDMA Channels) for OFDMA channel row entries in the docsIfUpChannelTable:




02/19/25                                            CableLabs®                                                  35
<!-- PDF_PAGE: 36 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


                    Table 10 - docsIfUpChannelTable Requirements for OFDMA Channels

                                      MIB Object                           Value
                         docsIfUpChannelFrequency                  0
                         docsIfUpChannelWidth                      0
                         docsIfUpChannelModulationProfile          0
                         docsIfUpChannelSlotSize                   0
                         docsIfUpChannelTxTimingOffset             0
                         docsIfUpChannelType                       unknown(0)


Other values in the docsIfUpChannelTable for OFDMA channels are reported in an implementation-dependent
manner. Operators are advised to not derive meaning from any other column in this table for rows whose columns
match the values defined in Table 10.
The CM MUST report the docsIfSignalQualityTable for SC-QAM channels. The CM MUST NOT include row
entries for OFDM channels in the docsIfSignalQualityTable.
The object docsIfDocsisBaseCapability is deprecated in DOCSIS 3.1.
The CM MUST extend the DOCS-IF-MIB textual convention DocsEqualizerData SYNTAX as follows: the OCTET
STRING range restriction has been removed.
DocsEqualizerData ::= TEXTUAL-CONVENTION
         STATUS       current
         DESCRIPTION
             "This data type represents the equalizer data
              as measured at the receiver interface.
              The format of the equalizer follows the structure of the
              Transmit Equalization Adjust RNG-RSP TLV of DOCSIS RFI
              v2.0 :
              1 byte Main tap location 1..(n + m)
              1 byte Number of forward taps per symbol
              1 byte Number of forward taps: n
              1 byte Number of reverse taps: m

                  Following are the equalizer coefficients:
                  First, forward taps coefficients:
                  2 bytes F1 (real), 2 bytes F1 (imag)
                  ...
                  2 bytes Fn (real), 2 bytes Fn (imag)

                  Then, reverse taps coefficients:
                  2 bytes D1 (real), 2 bytes D1 (imag)
                  ...

                  2 bytes Dm (real),        2 bytes         Dm (imag)

                  The equalizer coefficients are considered signed 16-bit
                  integers in the range from -32768 (0x8000) to 32767
                  (0x7FFF).

                DOCSIS specifications require up to a maximum of
                64 equalizer taps (n + m); therefore, this object size
                  can get up 260 bytes (4 + 4x64).
                The minimum object size (other than zero) for a t-spaced
                tap with a minimum of 8 symbols will be 36 (4 + 4x8)."
           REFERENCE
              "Data-Over-Cable Service Interface Specifications: Radio
               Frequency Interface Specification SP-RFIv2.0-I10-051209,
               Figure 8-23."
           SYNTAX        OCTET STRING



36                                                 CableLabs®                                          02/19/25
<!-- PDF_PAGE: 37 -->
Cable Modem Operations Support System Interface Specification                 CM-SP-CM-OSSIv3.1-I27-250219


The OCTET STRING range restriction has been removed.

7.1.3.8     Requirements for Interfaces Group MIB (RFC 2863)
The CM MUST implement the interface MIB [RFC 2863].
The ifType object associated with a DOCSIS interface can have the following enumerated values:
    •     CATV MAC interface: docsCableMacLayer (127)
    •     CATV downstream channel: docsCableDownstream (128)
    •     CATV M-CMTS downstream channel: docsCableMCmtsDownstream (229) (see [M-OSSI])
    •     CATV Downstream OFDM interface: docsOfdmDownstream (277)
    •     CATV upstream interface: docsCableUpStream (129)
    •     CATV logical upstream channel: docsCableUpstreamChannel (205)
    •     Upstream OFDMA interface: docsOfdmaUpstream (278)

7.1.3.8.1     Interface Organization and Numbering
Assigned interface numbers for DOCSIS-MAC and Ethernet (Ethernet-like interface) are used in the
NMAccessTable to configure access policy at these interfaces. These configurations are generally encoded in the
configuration file using TLV encoding.
The following statements define the CM interface-numbering scheme requirements:
    The CM MUST implement an instance of ifEntry for each configured DOCSIS-MAC interface, downstream
    interface, upstream interface, and for all of its LAN interfaces. If a DOCSIS-MAC interface consists of more
    than one upstream and downstream channel, the CM MUST populate the ifTable with a separate instance of
    ifEntry for each channel.
    The CM MAY fix LAN interfaces during the manufacturing process or determine these dynamically during the
    operation of the CM based on whether or not an interface has a CPE device attached to it.
    If the CM has multiple CPE interfaces, but only one CPE interface that can be enabled at any given time, the
    CM MUST populate the ifTable to contain only the entry corresponding to the enabled or the default CPE
    interface.
    The CM MUST populate the ifTable as specified in Table 42 - [RFC 2863] ifTable/ifXTable MIB-Object
    Details for Ethernet and USB Interfaces through Table 43 - [RFC 2863] ifTable/ifXTable MIB-Object Details
    for MAC and RF Interfaces of Section A.2. The CM MUST maintain entries in the ifTable for the downstream
    and upstream interfaces for which the CMTS has configured DS Receive Channels and US Transmit Channels,
    respectively, for this particular CM, and not for the total number of the CM receivers and transmitters the CM
    supports. CMTS configured Receive Channels and Transmit Channels for a CM are defined in [MULPIv3.1].
    While the CM is registered, the CM SHOULD use a different ifIndex to allocate a new CMTS configured
    Receive Channel or Transmit Channel, and avoid the reuse of previously assigned IfIndexes that are not
    currently part of the CMTS configured Receive Channel Set (RCS) or Transmit Channel Set (TCS).
    When a DS or US interface is configured as part of a RCS or TCS with a new channel id, the CM MUST update
    the ifCounterDiscontinuityTime and ifLastChange MIB variables.
    The CM MUST populate ifStackTable with an entry for the DOCSIS-MAC interface and include the
    downstream and upstream interfaces are reported in the ifTable.
    The CM MUST implement the MIB variable ifStackLastChange to report the value of sysUpTime where the
    ifStackTable change as a consequence of an addition or removal of a channel ID from a CM-SG as defined in
    [MULPIv3.1].




02/19/25                                          CableLabs®                                                       37
<!-- PDF_PAGE: 38 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


The following example illustrates a MAC interface with four downstream and four upstream interfaces for a CM.




                                         Figure 5 - ifIndex Example for CM


The CM MUST number its interfaces as described in Table 11 - CM Interface Numbering.
                                         Table 11 - CM Interface Numbering

                             Interface                              Type
                         1                  Primary CPE interface
                         2                  DOCSIS-MAC interface
                         3                  Primary downstream RF interface
                         4                  One of the upstream RF interfaces
                         5–15               Additional CPE interfaces
                         16–31              eDOCSIS eSAFE interfaces
                         32–47              Additional CPE interfaces




38                                                 CableLabs®                                          02/19/25
<!-- PDF_PAGE: 39 -->
Cable Modem Operations Support System Interface Specification                           CM-SP-CM-OSSIv3.1-I27-250219


                                 Interface                                Type
                           48–79                 Additional downstream RF interfaces
                           80–111                Additional upstream RF interfaces
                           112–143               Additional downstream RF interfaces


At any time, the CM MUST use ifIndex 3 for its primary downstream RF interface. The CM MUST use additional
interface numbering sequentially starting from 48. If the interface numbers between 48-79 are exhausted, the CM
MUST use 112-143 sequentially.
At any time, the CM MUST use ifIndex 4 for its first upstream RF interface. The CM MUST use additional
interface numbering sequentially starting from 80.
For example, if the RCS is configured with channels on ifIndex 3 and 48 and the Dynamic Bonding Change DBC
message demands ifIndex 3 be removed, the ifIndex 48 becomes ifIndex 3.
If the CM has more than one CPE interface, the vendor is required to define which of the CPE interfaces is the
primary CPE interface. The CM is permitted to have its primary CPE interface fixed during the manufacturing
process, or determine it dynamically during operation based on which interface has a CPE device attached to it.
Regardless of the number of CPE interfaces the CM has, or how the primary CPE interface is determined, the CM
will set the primary interface to interface number 1.
The CM MAY have additional CPE interfaces fixed during the manufacturing process or determined dynamically
during operation based on which interface has a CPE device attached to it. Additional CPE interface ifIndexes are
described in Table 11.

7.1.3.8.2     ifOperStatus Relationships

7.1.3.8.2.1   CmStatusValue and ifOperStatus Relationship
The CM MUST ensure that its CATV-MAC, downstream and upstream interfaces conform to the following
relationships, as shown in Table 12 - CmStatusValue and ifOperStatus Relationship, of ifOperStatus and
CmStatusValue (see Annex F) when ifAdminStatus value of those interfaces is 'up'.
                              Table 12 - CmStatusValue and ifOperStatus Relationship

 IfOperStatus                                                     CmStatusValue
 'down'           'other', 'notReady'
 'dormant'        'notSynchronized', 'phySynchronized', 'usParametersAcquired', 'rangingComplete', 'dhcpV4Complete',
                  'dhcpV6Complete', 'todEstablished', 'configFileDownloadComplete', 'startRegistration', 'bpiInit', 'accessDenied'
 'up'             'registrationComplete', 'securityEstablished', 'operational'



7.1.3.8.2.2   USB State and ifOperStatus Relationships
If the CM supports USBs as CPE interfaces, the CM SHOULD report the value of the MIB object ifOperStatus as
noted in Table 13 - USB State and ifOperStatus Relationship.
                                  Table 13 - USB State and ifOperStatus Relationship

 IfOperStatus                                   USB states and other conditions (see [USB])
 'down'           'Attached', 'Powered', 'Default', and STALL operation
 'dormant'        'Suspended', 'Address'
 'up'             'Configured'




02/19/25                                                 CableLabs®                                                             39
<!-- PDF_PAGE: 40 -->
CM-SP-CM-OSSIv3.1-I27-250219                                        Data-Over-Cable Service Interface Specifications


7.1.3.8.3       ifAdminStatus and Traffic
The CM MUST NOT accept or forward any traffic over an interface whose ifAdminStatus is 'down', (traffic
includes data and MAC management traffic where applicable).
When the CM initializes, all interfaces start with ifAdminStatus in the up(1) state. As a result of explicit
management action, ifAdminStatus is then changed to either the down(2) or testing(3) states (or remains in the up(1)
state). As a result of either explicit management or configuration information saved via other non-SNMP method
(i.e., CLI commands) retained by the managed system, ifAdminStatus is then changed to either the down(2) or
testing(3) states (or remains in the up(1) state).

7.1.3.8.4       SNMP Notification Control Requirements
If a multi-layer interface model is present in the device, each sub-layer for which there is an entry in the ifTable can
generate linkUp/Down traps. Since interface state changes would tend to propagate through the interface stack (from
top to bottom, or bottom to top), it is likely that several traps would be generated for each linkUp/Down occurrence.
The ifLinkUpDownTrapEnable object allows managers to control SNMP notification generation, and configure only
the interface sub-layers of interest.
The CM MUST implement the MIB object ifLinkUpDownTrapEnable specified in [RFC 2863].
For linkUp/Down events on CM DOCSIS interfaces, the CM SHOULD generate an SNMP notification for the CM
MAC interface and not for any sub-layers of the interface. Therefore, the CM MUST have its default setting of
ifLinkUpDownTrapEnable for the CM MAC interface set to 'enabled'. The CM MUST have its default setting of
ifLinkUpDownTrapEnable for the RF-Up interface(s) set to 'disabled'. The CM MUST have its default setting of
ifLinkUpDownTrapEnable for the RF-Down interface(s) set to 'disabled'. The CM SHOULD have its default setting
of ifLinkUpDownTrapEnable for interfaces 1 and 5 through 47 listed in Table 11 - CM Interface Numbering set to
'disabled'.
If the ifLinkUpDownTrapEnable for the CM MAC interface set to 'enabled', the CM MUST generate a linkUp
SNMP notification [RFC 2863].

7.1.3.8.5       ifTable and IfXTable Counters
Application of the [RFC 2863] ifTable and ifXTable MIB counter objects are done on a per-interface basis for
DOCSIS 3.0 and DOCSIS 3.1 and are detailed in Table 42 and Table 43 of Annex A.2. These tables define specific
SNMP Access and MIB requirements for each of the interface counters defined in [RFC 2863]. The CM MUST only
count octets on the downstream and upstream interfaces. The CM MAY implement the packet counters from
[RFC 2863], but when implemented on these interfaces, the counter object will return a value of zero. The CM
ethernet and MAC interfaces count both packet and octet counters. Per the requirements in [RFC 2863] Counter Size
section, a given interface may support only 32-bit or 64-bit (High Capacity), or both sets of counters based on
interface speed.
The CM MUST implement the ifTable and ifXTable [RFC 2863] Counter32 and Counter64 MIB objects as defined
for each interface in Table 44 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object Details for
Ethernet and USB Interfaces and Table 45 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object
Details for MAC and RF Interfaces of Section A.2.
The following table describes the rules for counting packets and octets on RF and MAC domain interfaces.
                                            Table 14 - IF-MIB Counter Rules

     MIB Counter                          MAC Domain Interfaces                               Upstream/Downstream RF
       Objects                                                                                       Interfaces
 ifInOctets              The total number of data octets (data in transit, data targeted   This includes MAC packets as well as
 ifHCInOctets            to the managed device) received on this interface from the RF     data packets, and includes the length
                         interface and before application of protocol filters.             of the MAC header; this does not
                                                                                           include any PHY overhead.
 ifInUcastPkts           The total number of Unicast data packets (data in transit, data   Reports zero if implemented.
 ifHCInUcastPkts         targeted to the managed device) received on this interface
                         from the RF interface before application of protocol filters.




40                                                      CableLabs®                                                     02/19/25
<!-- PDF_PAGE: 41 -->
Cable Modem Operations Support System Interface Specification                             CM-SP-CM-OSSIv3.1-I27-250219


     MIB Counter                         MAC Domain Interfaces                                Upstream/Downstream RF
       Objects                                                                                       Interfaces
 ifOutOctets            The total number of data octets (data in transit, data             This includes MAC packets as well as
 ifHCOutOctets          generated by the managed device) transmitted on this               data packets, and includes the length
                        interface to the RF interface after application of protocol        of the MAC header; this does not
                        filters.                                                           include any PHY overhead.
 ifOutUcastPkts         The total number of Unicast data packets (data in transit, data    Reports zero if implemented.
 ifHCOutUcastPkts       generated by the managed device) transmitted on this
                        interface to the RF interface after application of protocol
                        filters.
 ifInMulticastPkts      The total number of Multicast data packets (data in transit,       Reports zero if implemented.
 ifHCInMulticastPkts    data targeted to the managed device) received on this
                        interface from the RF interface before application of protocol
                        filters.
 ifInBroadcastPkts      The total number of Broadcast data packets (data in transit,       Reports zero if implemented.
 ifHCInBroadcastPkts    data targeted to the managed device) received on this
                        interface from the RF interface before application of protocol
                        filters.
 ifOutMulticastPkts     The total number of Multicast data packets (data in transit,       Reports zero if implemented.
 ifHCOutMulticastPkts   data generated by the managed device) transmitted on this
                        interface to the RF interface after application of protocol
                        filters.
 ifOutBroadcastPkts     The total number of Broadcast data packets (data in transit,       Reports zero if implemented.
 ifHCOutBroadcastPkts   data generated by the managed device) transmitted on this
                        interface to the RF interface after application of protocol
                        filters.



7.1.3.8.6     ifSpeed and ifHighSpeed
For SC-QAM downstream channels, the ifSpeed is the symbol rate multiplied by the number of bits per symbol. For
SC-QAM upstream channels, the ifSpeed is the raw bandwidth in bits per second of this interface, regarding the
highest speed modulation profile that is defined. This is the symbol rate multiplied with the number of bits per
symbol for this modulation profile.
For OFDM downstream channels, the CM MUST calculate the ifSpeed per the following algorithm:
numCountedSubcarriers = 0;
totalBitLoading = 0;

for (i = 0; i < numActiveSubcarriers; ++i)
{
if subcarrier is not PLC and not continuous pilot then
{
totalBitLoading += modulationOrder(i); //in bits per symbol
numCountedSubcarriers += 1;
}
}

averageBitLoading = (totalBitLoading / numCountedSubcarriers);
ifSpeed = numCountedSubcarriers * averageBitLoading * subcarrierSpacingHz *
          [numFftSamples/(numFftSamples + cyclicPrefixSamples)] * (127/128)

The number of symbols in a minislot for a given OFDMA channel is a factor of the number of symbols in a frame
and the number of subcarriers per minislot. The minislot capacity depends on the minislot bit loading and pilot
pattern, which are variable per minislot based on IUC, the minislot location in the frame and the burst profile being
used. Another factor is whether a minislot is classified as a body minislot or as an edge minislot.
For the purpose of calculating ifSpeed, the CM uses an OFDMA Data IUC with the highest capacity assuming that
all minislots are body minislots. The minislot capacity is calculated by multiplying the number of data symbols in a
minislot by modulation order (in bits per symbol) and adding to the number of complementary pilot symbols in a




02/19/25                                                CableLabs®                                                            41
<!-- PDF_PAGE: 42 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                              Data-Over-Cable Service Interface Specifications


minislot multiplied by the complementary data pilot modulation order. The upstream channel capacity is calculated
by adding the capacity of all minislots in a frame and multiplying that number by the frame rate.
For upstream OFDMA channels, the CM MUST calculate the ifSpeed per the following algorithm:
frameCapacity = 0;

for (i = 0; i < numMinislotsPerFrame; ++i)
{
minislotCapacity(i) = numDataSymbols(i) * modulationOrder(i) +
                             numComplementaryPilotSymbols(i) *
                              compPilotSymbolModulationOrder(i);
       frameCapacity += minislotCapacity(i);
}

                                                                                                     𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖
     𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛 =
                                              (𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖𝑖 + 𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐𝑐)(𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛𝑛)(𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓𝑓)

ifSpeed = numFramesPerSecond * frameCapacity;

7.1.3.8.7           ifDescr

7.1.3.8.7.1         IfDescr for USB Interfaces
If the CM supports USB as CPE interfaces, the CM MUST report the value of the MIB object ifDescr for these
interfaces as follows: USB <dbcUSB> CDC Ethernet; <any text>
<dbcUSB> corresponds to the USB version in the format JJ.M.N (JJ - major version number, M - minor version
number, N - sub-minor version number). See Standard USB Descriptor Definitions from [USB] specification.
For example, if the dbcUSB field in the USB descriptor is 0x0213, <dbcUSB> is presented in ifDescrMIB object as
"2.1.3" and a value of 0x2000 in the dbcUSB field of the USB Descriptor is represented as "2.0" in ifDescrMIB
object, in both cases without double quotes.
<Any text> indicates a vendor-specific text.
A complete example of ifDescr for an USB device is as follows (Assume dbcUBC 0x2000):
      USB 2.0 CDC Ethernet; <any text>

7.1.3.9        Requirements for Ethernet Interface MIB (RFC 3635)
The CM MUST implement [RFC 3635] if Ethernet interfaces are present.

7.1.3.10 Requirements for Bridge MIB (RFC 4188)
The CM MUST implement the Bridge MIB [RFC 4188] to support the forwarding requirements defined in
[MULPIv3.1].
The CM MUST implement a managed object (see docsDevSTPControl in [RFC 4639]) that controls the spanning
tree protocol (STP) policy defined in [IEEE802.1Q] and in accordance with [MULPIv3.1] requirements.
If STP is enabled for the CM, then the CM implements the dot1dStp scalar group [RFC 4188] and optionally the
dot1dStpPortTable [RFC 4188] as specified in Annex A.

7.1.3.11 Requirements for Internet Protocol MIB (RFC 4293)
The CM requirements for [RFC 4293] are defined in the following sections.

7.1.3.11.1          The IP Group
The CM MUST implement the ipv4GeneralGroup.
The CM MUST implement the ipv6GeneralGroup2.



42                                                                        CableLabs®                                                                       02/19/25
<!-- PDF_PAGE: 43 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv3.1-I27-250219


The CM MUST implement the ipv4InterfaceTable.
The CM MUST populate the ipv4InterfaceTable with each Ethernet interface with an assigned IPv4 address. The
CM MAY record other interfaces in the ipv4InterfaceTable which have assigned IPv4 addresses.
The CM MUST populate the ipv6InterfaceTable with each Ethernet interface with an assigned IPv6 address. The
CM MAY record other interfaces in the ipv6InterfaceTable which have assigned IPv6 addresses.
The CM MAY implement the ipSystemStatsTable.
The CM MAY implement the ipIfStatsTable.
The CM MAY implement the ipAddressPrefixTable.
The CM MAY implement the ipAddressTable.
The CM MAY implement the ipNetToPhysicalTable.
The CM MAY implement the ipDefaultRouterTable.
The CM MUST NOT implement the ipv6RouterAdvertTable.

7.1.3.11.2    The ICMP Group
The CM MUST implement the icmpStatsTable.
The CM MUST implement the icmpMsgStatsTable.

7.1.3.12 Requirements for User Datagram Protocol MIB (RFC 4113)
The CM MAY implement the UDP-MIB in [RFC 4113].

7.1.3.13 Requirements for Transmission Control Protocol (TCP) MIB (RFC 4022)

7.1.3.13.1    The TCP Group
The CM MAY implement the TCP group in [RFC 4022].

7.1.3.14 Requirements for SNMPv2 MIB (RFC 3418)

7.1.3.14.1    The System Group
The CM MUST implement the System Group of [RFC 3418].
See Section 8.2.1 for sysDescr requirements details.

7.1.3.14.2    The SNMP Group
This group provides SNMP protocol statistics and protocol errors counters.
The CM MUST implement the SNMP Group from [RFC 3418].

7.1.3.15 Requirements for Internet Group Management Protocol MIB (RFC 2933)
A DOCSIS 3.1 CM is no longer required to implement [RFC 2933].

7.1.3.16 Requirements for DOCSIS Baseline Privacy Plus MIB (RFC 4131)
The CM MUST implement [RFC 4131].
To support the new PKI defined in [SECv3.1], the CM MUST implement the DOCS-BPI2EXT-MIB defined in
[DOCS-BPI2EXT-MIB], which is an extension to [RFC 4131].




02/19/25                                           CableLabs®                                                 43
<!-- PDF_PAGE: 44 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


The following MIB objects from [RFC 4131] are used to support legacy PKI CM certificate functions defined in the
DOCSIS 3.0 security specification:
     docsBpi2CmDeviceCertTable
     docsBpi2CodeMfgOrgName
     docsBpi2CodeMfgCodeAccessStart
     docsBpi2CodeMfgCvcAccessStart
     docsBpi2CodeCoSignerOrgName
     docsBpi2CodeCoSignerCodeAccessStart
     docsBpi2CodeCoSignerCvcAccessStart
     docsBpi2CodeCvcUpdate
The MIB objects listed above are applicable only if the cable modem supports DOCSIS 3.0 PKI.
The following MIB objects are defined in the [DOCS-BPI2EXT-MIB] module that supports PKI CM certificate
functions defined in [SECv3.1]:
     docsBpi2Ext31CmDeviceCmCert
     docsBpi2Ext31CmDeviceManufCert
     docsBpi2Ext31CodeUpdateCvcChain
     docsBpi2Ext31CodeMfgOrgName
     docsBpi2Ext31CodeMfgCodeAccessStart
     docsBpi2Ext31CodeMfgCvcAccessStart
     docsBpi2Ext31CodeCoSignerOrgName
     docsBpi2Ext31CodeCoSignerCodeAccessStart
     docsBpi2Ext31CodeCoSignerCvcAccessStart

7.1.3.17 Requirements for Diffie-Helman USM Key MIB (RFC 2786)
The CM MUST implement [RFC 2786].

7.1.3.18 Requirements for DOCSIS Baseline Privacy MIB (RFC 3083)
A DOCSIS 3.1 CM is not required to implement [RFC 3083].

7.1.3.19 Requirements for SNMPv3 MIB Modules
The CM MUST implement the MIBs defined in [RFC 3411] through [RFC 3415] and [RFC 3584].
The CM MUST support the default value of 'volatile' for any SNMPv3 object with a StorageType syntax. This
overrides the default value specified in [RFC 3411] through [RFC 3415] and [RFC 3584]. The CM MUST only
accept the value of 'volatile' for any SNMPv3 object with a StorageType syntax. An attempted set to a value of
'other', 'nonVolatile', 'permanent', or 'readOnly' will result in an "inconsistentValue" error. Values other than the
valid range (1–5) would result in a "wrongValue" error.
The CM SHOULD support a minimum of 30 available rows in the vacmViewTreeFamilyTable object.

7.1.3.20 Requirements for Entity MIB (RFC 6933)
The CM MAY implement the ENTITY-MIB [RFC 6933].




44                                                   CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 45 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv3.1-I27-250219


If the CM implements the ENTITY-SENSOR-MIB [RFC 3433], the CM is required to implement the
entPhysicalTable with entries corresponding to any sensors managed in the ENTITY-SENSOR-MIB (e.g.,
temperature sensors). For sensor entries in the entPhysicalTable, the CM reports a value of 'sensor' for
entPhysicalClass.

7.1.3.21 Requirements for Entity Sensor MIB (RFC 3433)
The CM MAY implement the Entity Sensor MIB [RFC 3433].
The CM MAY implement the entPhySensorTable for instances which exist in the entPhysicalTable of the ENTITY-
MIB [RFC 6933] with an entPhysicalClass of 'sensor'. It is recommended that for temperature sensors, the CM
report a value for entPhySensorType of 'celsius'.

7.1.3.22 Requirements for Host Resources MIB (RFC 2790)
The CM MAY implement the HOST-RESOURCES-MIB [RFC 2790].




02/19/25                                          CableLabs®                                               45
