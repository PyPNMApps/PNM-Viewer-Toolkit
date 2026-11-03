# Annex A.2 - RFC 2863 ifTable/ifXTable MIB-Object Details

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 119-122
Classification: Normative (explicit)

<!-- PDF_PAGE: 119 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                                     CM             Access
 docsPnmCmUsPreEqAmpMean                                                                    M              RO
 docsPnmCmUsPreEqGrpDelaySlope                                                              M              RO
 docsPnmCmUsPreEqGrpDelayMean                                                               M              RO
 docsPnmCmDsOfdmModProfTable                                                                M              N-Acc
 docsPnmCmDsOfdmModProfEntry                                                                M              N-Acc
 docsPnmCmDsOfdmModProfFileEnable                                                           M              RW
 docsPnmCmDsOfdmModProfMeasStatus                                                           M              RO
 docsPnmCmDsOfdmModProfFileName                                                             M              RW
 docsCmLatencyRptCfgSnapshotDuration                                                        M              RW
 docsCmLatencyRptCfgNumSnapshots                                                            M              RW
 docsCmLatencyRptCfgNumFiles                                                                M              RW
 docsCmLatencyRptCfgMeasStatus                                                              M              RO
 docsCmLatencyRptCfgFileName                                                                M              RW
 docsIf3CmSpectrumAnalysis
 docsIf3CmSpectrumAnalysisCtrlCmdFileEnable                                                 M              RW
 docsIf3CmSpectrumAnalysisCtrlCmdMeasStatus                                                 M              RO
 docsIf3CmSpectrumAnalysisCtrlCmdFileName                                                   M              RW
 [DOCS-BPI2EXT-MIB]
 docsBpi2Ext31CmDeviceCertTable                                                             M              N-Acc
 docsBpi2Ext31CmDeviceCertEntry                                                             M              N-Acc
 docsBpi2Ext31CmDeviceCmCert                                                                M              RW
 docsBpi2Ext31CmDeviceManufCert                                                             M              RO
 docsBpi2Ext31CodeDownloadControl
 docsBpi2Ext31CodeUpdateCvcChain                                                            M              RW
 docsBpi2Ext31CodeMfgOrgName                                                                M              RO
 docsBpi2Ext31CodeMfgCodeAccessStart                                                        M              RO
 docsBpi2Ext31CodeMfgCvcAccessStart                                                         M              RO
 docsBpi2Ext31CodeCoSignerOrgName                                                           M              RO
 docsBpi2Ext31CodeCoSignerCodeAccessStart                                                   M              RO
 docsBpi2Ext31CodeCoSignerCvcAccessStart                                                    M              RO



A.2 RFC 2863 ifTable/ifXTable MIB-Object Details
Refer to [RFC 2863] for MIB object descriptions. Table 39 includes DOCSIS 3.1 specific object information.
The following tables detail the specific ifTable and ifXTable MIB objects and values that are expected for the
interfaces on the CM.
Section 7.1.3.8.5 has defined the requirements for the [RFC 2863] ifTable and ifXTable MIB objects. This section
applies these general requirements to the CM interfaces. Table 42 defines the specific requirements for the CM
ethernet, USB and other interfaces. Table 43 defines the specific requirements for the CM upstream, downstream
and MAC interfaces. Table 42 and Table 43 exclude the Counter32 and Counter64 MIB objects as these counter
objects are defined in Table 44 and Table 45.
In order to simplify and compile all the requirements for the Counter32 and Counter64 MIB objects in a single
location, the specific SNMP Access requirements and MIB implementation details that are normally detailed in
Annex A.1 are reflected in Table 44 and Table 45. The nomenclature for the MIB implementation details can be
found in Table 39 and the SNMP Access Requirements are detailed in Table 40. Please refer to these tables for the
values found for each of the interfaces in Table 44 and Table 45.



02/19/25                                           CableLabs®                                                    119
<!-- PDF_PAGE: 120 -->
CM-SP-CM-OSSIv3.1-I27-250219                                        Data-Over-Cable Service Interface Specifications


In addition to the requirements for Ethernet and USB detailed in Table 42 - [RFC 2863] ifTable/ifXTable MIB-
Object Details for Ethernet and USB Interfaces below, note that the various packet and octet counters from the
ifTable and ifXTable MAY exclude LAN-LAN traffic which is not bridged upstream or downstream. From the
ifTable, these counters include the following: ifInOctets, ifInUcastPkts, ifOutOctets, and ifOutUcastPkts. From the
ifXTable, included counters are ifInMulticastPkts, ifInBroadcastPkts, ifOutMulticastPkts, ifOutBroadcastPkts,
ifHCInOctets, ifHCInUcastPkts, ifHCInMulticastPkts, ifHCInBroadcastPkts, ifHCOutOctets, ifHCOutUcastPkts,
ifHCOutMulticastPkts, and ifHCOutBroadcastPkts.
              Table 42 - [RFC 2863] ifTable/ifXTable MIB-Object Details for Ethernet and USB Interfaces

                      MIB Objects                CM-Ethernet              CM USB CDC           CM-CPE OtherType
                                                                            Ethernet
           IfTable
           ifIndex                        1 or [4+(n)]              1 or [4+(n)]              1 or [4+(n)]
           ifDescr                                                  See Section 7.1.3.8.7.1
           ifType                         6                         160                       (IANA num)
           ifMtu                          1500                      1500                      Media dependent
           ifSpeed                        10,000,000,               12,000,000, 480,000,000   speed
                                          100,000,000, ...
           ifPhysAddress                  MAC Address of this       MAC Address of this       Media dependent
                                          interface                 interface
           ifAdminStatus                  up(1),                    up(1),                    up(1),
                                          down(2),                  down(2),                  down(2),
                                          testing(3)                testing(3)                testing(3)
           ifOperStatus                   up(1),                    See Section 7.1.3.8.2.2   up(1),
                                          down(2),                                            down(2),
                                          testing(3),                                         testing(3),
                                          dormant(5),                                         dormant(5),
                                          notPresent(6)                                       notPresent(6)
           ifLastChange
           ifXTable
           ifName
           ifLinkUpDownTrapEnable
           NOTE: See Section 7.1.3.8.4
           for details
           ifHighSpeed                    10, 100, ...              12, 480                   speed
           ifPromiscuousMode              true, false               true, false               true, false
           ifConnectorPresent
           ifAlias
           ifCounterDiscontinuityTime


NOTE: Refer to Table 45 for Counter32 and Counter64 MIB object details.
                     Table 43 - [RFC 2863] ifTable/ifXTable MIB-Object Details for MAC and RF Interfaces

                     MIB Objects                         CM-MAC                   CM-Downstream             CM-Upstream
 IfTable
 ifIndex                                      2                               3                     4
 ifDescr
 ifType                                       127                             128 (SC-QAM)          129 (SC-QAM)
                                                                              277 (OFDM)            278 (OFDMA)




120                                                          CableLabs®                                            02/19/25
<!-- PDF_PAGE: 121 -->
Cable Modem Operations Support System Interface Specification                             CM-SP-CM-OSSIv3.1-I27-250219


                 MIB Objects                                CM-MAC            CM-Downstream                  CM-Upstream
 ifMtu                                         1522 (if Extended Packet      1764 (if Extended       1764 (if Extended Packet
 (For RF Upstream/Downstream; the              Length Support modem          Packet Length Support   Length Support modem
 value includes the length of the MAC          capability is disabled)       modem capability is     capability is disabled
 header.)                                                                    disabled)
 Note: The ifMtu MIB reports a value           Value of Extended Packet                              Value of Extended Packet
 based on the value returned in the            Length Support modem          Value of Extended       Length Support modem
 Extended Packet Length Support modem          capability (TLV 5.48) (if     Packet Length Support   capability (TLV 5.48) + 30
 capability [MULPIv3.1]. In the case of the    Extended Packet Length        modem capability        (if Extended Packet Length
 CM-Downstream and CM-Upstream, the            Support modem capability is   (TLV 5.48) + 30 (if     Support modem capability
 reported value of ifMTU additionally          enabled)                      Extended Packet         is enabled)
 accounts for the length of the DOCSIS                                       Length Support
 MAC header.                                                                 modem capability is
                                                                             enabled)
 ifSpeed                                       0                             Refer to Section        Refer to Section 7.1.3.8.6
                                                                             7.1.3.8.6
 ifPhysAddress:                                MAC Address of this           Empty-String            Empty-String
                                               interface
 ifAdminStatus:                                up(1),                        up(1),                  up(1),
 Refer to Section 7.1.3.8.3                    down(2),                      down(2),                down(2),
                                               testing(3)                    testing(3)              testing(3)
 ifOperStatus:                                 up(1),                        up(1),                  up(1), down(2), testing(3),
 Refer to Section 7.1.3.8.2                    down(2),                      down(2),                dormant(5), notPresent(6)
                                               testing(3),                   testing(3),
                                               dormant(5),                   dormant(5),
                                               notPresent(6)                 notPresent(6)
 ifLastChange:
 ifXTable
 ifName
 ifLinkUpDownTrapEnable
 See Section 7.1.3.8.5
 ifHighSpeed                                   0                             Refer to Section        Refer to Section 7.1.3.8.6
                                                                             7.1.3.8.6
 ifPromiscuousMode                             true                          true                    false
 ifConnectorPresent
 ifAlias
 ifCounterDiscontinuityTime


NOTE: Refer to Table 45 for Counter32 and Counter64 MIB object details.
  Table 44 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object Details for Ethernet and USB
                                                 Interfaces

      MIB Counter Objects                     ACCESS               CM-Ethernet              CM-USB                 CM-CPE
                                                                                                                  OtherType
 ifTable
 ifInOctets                           RO                       M                     M                       M
 ifInUcastPkts                        RO                       M                     M                       M
 ifInDiscards                         RO                       M                     M                       M
 ifInErrors                           RO                       O                     O                       M
 ifInUnknownProtos                    RO                       M                     M                       M
 ifOutOctets                          RO                       M                     M                       M
 ifOutUcastPkts                       RO                       M                     M                       M
 ifOutDiscards                        RO                       M                     M                       M
 ifOutErrors                          RO                       M                     M                       M




02/19/25                                                     CableLabs®                                                       121
<!-- PDF_PAGE: 122 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


      MIB Counter Objects               ACCESS                CM-Ethernet            CM-USB                CM-CPE
                                                                                                          OtherType
 ifXTable
 ifInMulticastPkts                 RO                     M                     M                     M
 ifInBroadcastPkts                 RO                     M                     M                     M
 ifOutMulticastPkts                RO                     M                     M                     M
 ifOutBroadcastPkts                RO                     M                     M                     M
 IfHCInOctets                      RO                     O                     O                     O
 ifHCInUcastPkts                   RO                     O                     O                     O
 ifHCInMulticastPkts               RO                     O                     O                     O
 ifHCInBroadcastPkts               RO                     O                     O                     O
 ifHCOutOctets                     RO                     O                     O                     O
 ifHCOutUcastPkts                  RO                     O                     O                     O
 ifHCOutMulticastPkts              RO                     O                     O                     O
 ifHCOutBroadcastPkts              RO                     O                     O                     O


In Table 45, the packet and octet counters are implemented based on the requirements in Section 7 of this
specification. In this table, the value NA means that the particular counter is not applicable to this interface. Objects
labeled as NA or O in Table 45 can be optionally implemented and if implemented, the object will return 0 when
read.
      Table 45 - [RFC 2863] ifTable/ifXTable Counter32 and Counter64 MIB-Object Details for MAC and RF
                                                   Interfaces

                 MIB Counter Objects                      Access           CM-MAC           CM-                 CM-
                                                                                         Downstream           Upstream
 ifTable
 ifInOctets                                          RO                M                M                 NA
 ifInUcastPkts                                       RO                M                O                 NA
 ifInDiscards                                        RO                M                O                 NA
 ifInErrors                                          RO                M                O                 NA
 ifInUnknownProtos                                   RO                M                O                 NA
 ifOutOctets                                         RO                M                NA                M
 ifOutUcastPkts                                      RO                M                NA                O
 ifOutDiscards                                       RO                M                NA                O
 ifOutErrors                                         RO                M                NA                O
 ifXTable
 ifInMulticastPkts                                   RO                M                O                 NA
 ifInBroadcastPkts                                   RO                M                O                 NA
 ifOutMulticastPkts                                  RO                M                NA                O
 ifOutBroadcastPkts                                  RO                M                NA                O
 IfHCInOctets                                        RO                M                M                 NA
 ifHCInUcastPkts                                     RO                O                O                 NA
 ifHCInMulticastPkts                                 RO                O                O                 NA
 ifHCInBroadcastPkts                                 RO                O                O                 NA
 ifHCOutOctets                                       RO                M                NA                M
 ifHCOutUcastPkts                                    RO                O                NA                O
 ifHCOutMulticastPkts                                RO                O                NA                O
 ifHCOutBroadcastPkts                                RW                O                NA                O




122                                                  CableLabs®                                                 02/19/25
