# Annex A.5 - RFC 2863 ifTable/ifXTable MIB-Object Details

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 154-157
Classification: Normative (explicit)

<!-- PDF_PAGE: 154 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications




A.4 DOCSIS FDD MIB-Object Details
The following SNMP MIB objects were defined for the DOCSIS 4.0 CM that supports FDD to provide
configuration and status of the CM when initialized into the FDD mode of operation.
[DOCS-FDD-MIB]                                                                      DOCSIS 4.0 CM Compliance
                                                                                FDXL       FDX       FDD
docsFddCmFddSystemCfgState
docsFddCmFddSystemCfgStateDiplexerDsLowerBandEdgeCfg                                                M            RO
docsFddCmFddSystemCfgStateDiplexerDsUpperBandEdgeCfg                                                M            RO
docsFddCmFddSystemCfgStateDiplexerUsUpperBandEdgeCfg                                                M            RO
docsFddDiplexerUsUpperBandEdgeCapabilityTable                                                       M            N-Acc
docsFddDiplexerUsUpperBandEdgeCapabilityEntry                                                       M            N-Acc
docsFddDiplexerUsUpperBandEdgeCapabilityIndex                                                       M            N-Acc
docsFddDiplexerUsUpperBandEdgeCapability                                                            M            RO
docsFddDiplexerDsLowerBandEdgeCapabilityTable                                                       M            N-Acc
docsFddDiplexerDsLowerBandEdgeCapabilityEntry                                                       M            N-Acc
docsFddDiplexerDsLowerBandEdgeCapabilityIndex                                                       M            N-Acc
docsFddDiplexerDsLowerBandEdgeCapability                                                            M            RO
docsFddDiplexerDsUpperBandEdgeCapabilityTable                                                       M            N-Acc
docsFddDiplexerDsUpperBandEdgeCapabilityEntry                                                       M            N-Acc
docsFddDiplexerDsUpperBandEdgeCapabilityIndex                                                       M            N-Acc
docsFddDiplexerDsUpperBandEdgeCapability                                                            M            RO
docsFddCmUsOfdmaChannelTable                                                                        M            N-Acc
docsFddCmUsOfdmaChannelEntry                                                                        M            N-Acc
docsFddCmUsOfdmaChannelIsExtendedUsChannel                                                          M            RO
docsFddCmUsOfdmaChannelExtendedUsChannelTxPower                                                     M            RO



A.5 RFC 2863 ifTable/ifXTable MIB-Object Details
Refer to [RFC 2863] for MIB object descriptions. Table 68 includes DOCSIS 4.0 specific object information.
The following tables detail the specific ifTable and ifXTable MIB objects and values that are expected for the
interfaces on the CM.
Section 7.1.3.11.5 has defined the requirements for the [RFC 2863] ifTable and ifXTable MIB objects. This section
applies these general requirements to the CM interfaces. Table 71 defines the specific requirements for the CM
ethernet, USB and other interfaces. Table 72 defines the specific requirements for the CM upstream, downstream
and MAC interfaces. Table 71 and Table 72 exclude the Counter32 and Counter64 MIB objects as these counter
objects are defined in Table 73 and Table 74.
In order to simplify and compile all the requirements for the Counter32 and Counter64 MIB objects in a single
location, the specific SNMP Access requirements and MIB implementation details that are normally detailed in
Annex A.1 are reflected in Table 73 and Table 74. The nomenclature for the MIB implementation details can be
found in Table 68 and the SNMP Access Requirements are detailed in Table 69. Please refer to these tables for the
values found for each of the interfaces in Table 73 and Table 74.
In addition to the requirements for Ethernet and USB detailed in Table 71 - [RFC 2863] ifTable/ifXTable MIB-
Object Details for Ethernet and USB Interfaces below, note that the various packet and octet counters from the
ifTable and ifXTable MAY exclude LAN-LAN traffic which is not bridged upstream or downstream. From the



154                                                CableLabs                                              04/29/26
<!-- PDF_PAGE: 155 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


ifTable, these counters include the following: ifInOctets, ifInUcastPkts, ifOutOctets, and ifOutUcastPkts. From the
ifXTable, included counters are ifInMulticastPkts, ifInBroadcastPkts, ifOutMulticastPkts, ifOutBroadcastPkts,
ifHCInOctets, ifHCInUcastPkts, ifHCInMulticastPkts, ifHCInBroadcastPkts, ifHCOutOctets, ifHCOutUcastPkts,
ifHCOutMulticastPkts, and ifHCOutBroadcastPkts.
            Table 71 - [RFC 2863] ifTable/ifXTable MIB-Object Details for Ethernet and USB Interfaces

           MIB Objects                  CM-Ethernet                 CM USB CDC Ethernet             CM-CPE OtherType
 IfTable
 ifIndex                        1 or [4+(n)]                    1 or [4+(n)]                    1 or [4+(n)]
 ifDescr                                                        See Section 7.1.3.11.7.1
 ifType                         6                               160                             (IANA num)
 ifMtu                          1500                            1500                            Media dependent
 ifSpeed                        100,000,000                     12,000,000, 480,000,000,…       speed
 NOTE 1
 ifPhysAddress                  MAC Address of this interface   MAC Address of this interface   Media dependent
 ifAdminStatus                  up(1),                          up(1),                          up(1),
                                down(2),                        down(2),                        down(2),
                                testing(3)                      testing(3)                      testing(3)
 ifOperStatus                   up(1),                          See Section 7.1.3.11.2.2        up(1),
                                down(2),                                                        down(2),
                                testing(3),                                                     testing(3),
                                dormant(5),                                                     dormant(5),
                                notPresent(6)                                                   notPresent(6)
 ifLastChange
 ifXTable
 ifName
 ifLinkUpDownTrapEnable
 NOTE: See Section
 7.1.3.11.4 for details
 ifHighSpeed                    100, 1000, ...                  12, 480, …                      speed
 ifPromiscuousMode              true, false                     true, false                     true, false
 ifConnectorPresent
 ifAlias
 ifCounterDiscontinuityTime

NOTE: Refer to Table 74 for Counter32 and Counter64 MIB object details.
NOTE 1: Per RFC 2863, if the bandwidth of the interface is greater than the maximum value reportable by this object,
        then this object reports its maximum value (4,294,967,295), and ifHighSpeed is used to report the interface's
        speed.


                Table 72 - [RFC 2863] ifTable/ifXTable MIB-Object Details for MAC and RF Interfaces

           MIB Objects                        CM-MAC                   CM-Downstream                    CM-Upstream
 IfTable
 ifIndex                        2                               3                               4
 ifDescr
 ifType                         127                             128 (SC-QAM)                    129 (SC-QAM)
                                                                277 (OFDM)                      278 (OFDMA)




04/29/26                                               CableLabs                                                     155
<!-- PDF_PAGE: 156 -->
CM-SP-CM-OSSIv4.0-I13-260429                                          Data-Over-Cable Service Interface Specifications


           MIB Objects                         CM-MAC                      CM-Downstream                     CM-Upstream
 ifMtu                            1522 (if Extended Packet          1764 (if Extended Packet         1764 (if Extended Packet
 (For RF                          Length Support modem              Length Support modem             Length Support modem
 Upstream/Downstream; the         capability is disabled)           capability is disabled)          capability is disabled
 value includes the length of
 the MAC header.)                 Value of Extended Packet          Value of Extended Packet         Value of Extended Packet
 NOTE: The ifMtu MIB reports      Length Support modem              Length Support modem             Length Support modem
 a value based on the value       capability (TLV 5.48) (if         capability (TLV 5.48) + 30 (if   capability (TLV 5.48) + 30 (if
 returned in the Extended         Extended Packet Length            Extended Packet Length           Extended Packet Length
 Packet Length Support            Support modem capability is       Support modem capability is      Support modem capability is
 modem capability                 enabled)                          enabled)                         enabled)
 [MULPIv4.0]. In the case of
 the CM-Downstream and CM-
 Upstream, the reported value
 of ifMTU additionally accounts
 for the length of the DOCSIS
 MAC header.
 ifSpeed                          0                                 Refer to Section 7.1.3.11.6      Refer to Section 7.1.3.11.6
 ifPhysAddress                    MAC Address of this interface     Empty-String                     Empty-String
 ifAdminStatus:                   up(1),                            up(1),                           up(1),
 Refer to Section 7.1.3.11.3      down(2),                          down(2), testing(3)              down(2),
                                  testing(3)                                                         testing(3)
 ifOperStatus:                    up(1),                            up(1),                           up(1),
 Refer to Section 7.1.3.11.2      down(2),                          down(2),                         down(2),
                                  testing(3),                       testing(3),                      testing(3),
                                  dormant(5),                       dormant(5),                      dormant(5),
                                  notPresent(6)                     notPresent(6)                    notPresent(6)
 ifLastChange
 ifXTable
 ifName
 ifLinkUpDownTrapEnable:
 See Section 7.1.3.11.5.
 ifHighSpeed                      0                                 Refer to Section 7.1.3.11.6      Refer to Section 7.1.3.11.6
 ifPromiscuousMode                true                              true                             false
 ifConnectorPresent
 ifAlias
 ifCounterDiscontinuityTime

NOTE: Refer to Table 74 for Counter32 and Counter64 MIB object details.


                  Table 73 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object Details
                                            for Ethernet and USB Interfaces

                                                                                                              CM-CPE
             MIB Counter Objects               ACCESS           CM-Ethernet               CM-USB             OtherType
           ifTable
           ifInOctets                     RO                    M                    M                   M
           ifInUcastPkts                  RO                    M                    M                   M
           ifInDiscards                   RO                    M                    M                   M
           ifInErrors                     RO                    O                    O                   M
           ifInUnknownProtos              RO                    M                    M                   M
           ifOutOctets                    RO                    M                    M                   M
           ifOutUcastPkts                 RO                    M                    M                   M
           ifOutDiscards                  RO                    M                    M                   M
           ifOutErrors                    RO                    M                    M                   M



156                                                     CableLabs                                                       04/29/26
<!-- PDF_PAGE: 157 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


                                                                                                      CM-CPE
           MIB Counter Objects              ACCESS         CM-Ethernet             CM-USB            OtherType
        ifXTable
        ifInMulticastPkts              RO                 M                   M                  M
        ifInBroadcastPkts              RO                 M                   M                  M
        ifOutMulticastPkts             RO                 M                   M                  M
        ifOutBroadcastPkts             RO                 M                   M                  M
        IfHCInOctets                   RO                 O                   O                  O
        ifHCInUcastPkts                RO                 O                   O                  O
        ifHCInMulticastPkts            RO                 O                   O                  O
        ifHCInBroadcastPkts            RO                 O                   O                  O
        ifHCOutOctets                  RO                 O                   O                  O
        ifHCOutUcastPkts               RO                 O                   O                  O
        ifHCOutMulticastPkts           RO                 O                   O                  O
        ifHCOutBroadcastPkts           RO                 O                   O                  O


In Table 74, the packet and octet counters are implemented based on the requirements in Section 7 of this
specification. In this table, the value NA means that the particular counter is not applicable to this interface. Objects
labeled as NA or O in Table 74 can be optionally implemented and if implemented, the object will return 0 when
read.
               Table 74 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object Details
                                            for MAC and RF Interfaces

                                                                                  CM-                  CM-
           MIB Counter Objects              Access            CM-MAC           Downstream            Upstream
        ifTable
        ifInOctets                     RO                 M                   M                  NA
        ifInUcastPkts                  RO                 M                   O                  NA
        ifInDiscards                   RO                 M                   O                  NA
        ifInErrors                     RO                 M                   O                  NA
        ifInUnknownProtos              RO                 M                   O                  NA
        ifOutOctets                    RO                 M                   NA                 M
        ifOutUcastPkts                 RO                 M                   NA                 O
        ifOutDiscards                  RO                 M                   NA                 O
        ifOutErrors                    RO                 M                   NA                 O
        ifXTable
        ifInMulticastPkts              RO                 M                   O                  NA
        ifInBroadcastPkts              RO                 M                   O                  NA
        ifOutMulticastPkts             RO                 M                   NA                 O
        ifOutBroadcastPkts             RO                 M                   NA                 O
        IfHCInOctets                   RO                 M                   M                  NA
        ifHCInUcastPkts                RO                 O                   O                  NA
        ifHCInMulticastPkts            RO                 O                   O                  NA
        ifHCInBroadcastPkts            RO                 O                   O                  NA
        ifHCOutOctets                  RO                 M                   NA                 M
        ifHCOutUcastPkts               RO                 O                   NA                 O
        ifHCOutMulticastPkts           RO                 O                   NA                 O
        ifHCOutBroadcastPkts           RW                 O                   NA                 O




04/29/26                                             CableLabs                                                      157
