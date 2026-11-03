# Annex A.1 - MIB-Object Details

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 83-118
Classification: Normative (explicit)

<!-- PDF_PAGE: 83 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


Annex A           Detailed MIB Requirements (Normative)
This annex defines the SNMP MIB modules and MIB variables required for DOCSIS 3.1 CM devices.
                                     Table 39 - MIB Implementation Support

 Requirement Type      Table Notation                                       Description
 Deprecated            D                 Deprecated objects are optional. If a vendor chooses to implement the object, the
                                         object must be implemented correctly according to the MIB definition. If a vendor
                                         chooses not to implement the object, an agent must not instantiate such object and
                                         must respond with the appropriate error/exception condition (e.g., 'noSuchObject'
                                         for SNMPv2c).
 Mandatory             M                 The object must be implemented correctly according to the MIB definition.
 Not Applicable        NA                Not applicable to the device.
 Not Supported         N-Sup             An agent must not instantiate such object and must respond with the appropriate
                                         error/exception condition (e.g., 'noSuchObject' for SNMPv2c).
 Optional              O                 A vendor can choose to implement or not implement the object. If a vendor
                                         chooses to implement the object, the object must be implemented correctly
                                         according to the MIB definition. If a vendor chooses not to implement the object, an
                                         agent must not instantiate such object and must respond with the appropriate
                                         error/exception condition (e.g., 'noSuchObject' for SNMPv2c).
 Obsolete              Ob                In SNMP convention, obsolete objects are not to be implemented. This specification
                                         allows vendors to implement or not implement obsolete objects. If a vendor chooses
                                         to implement an obsoleted object, the object must be implemented correctly
                                         according to the MIB definition. If a vendor chooses not to implement the obsoleted
                                         object, the SNMP agent must not instantiate such object and must respond with the
                                         appropriate error/exception condition (e.g., 'noSuchObject' for SNMPv2c).


                                     Table 40 - SNMP Access Requirements

    SNMP Access         Table Notation                                      Description
       Type
 N-Acc                  Not Accessible   The object is not accessible and is usually an index in a table
 Read Create            RC               The access of the object is implemented as Read-Create
 Read Write             RW               The access of the object is implemented as Read-Write
 Read Only              RO               The access of the object is implemented as Read-Only
 Read Create or         RC/RO            The access of the object is implemented as either Read-Create or Read-Only as
 Read Only                               described in the MIB definition
 Read Write /           RW/RO            The access of the object is implemented as either Read-Write or Read-Only as
 Read Only                               described in the MIB definition
 Accessible for SNMP    Acc-FN           These objects are used for SNMP Notifications by the CM SNMP Agent
 Notifications



A.1 MIB-Object Details
The CM instantiates SNMP MIB objects based on its configuration and operational parameters acquired during
registration. Below are denominations for several Table 41 columns that indicate modes of operation where a CM
has specific management requirements for certain MIB object instantiation and syntax.
The CM always operates in "1.1 QoS Mode".
The CM SNMP access control configuration is either NmAccess Mode or SNMP Coexistence Mode.
The CM upstream channel types can be categorized as "OFDMA upstream", "TDMA/ATDMA upstream", and
"SCDMA upstream".




02/19/25                                           CableLabs®                                                              83
<!-- PDF_PAGE: 84 -->
CM-SP-CM-OSSIv3.1-I27-250219                                             Data-Over-Cable Service Interface Specifications


                                                Table 41 - MIB Object Details

 DOCS-IF-MIB [RFC 4546]
 Note: Refer to Section 7.1.3.7 for detailed requirements for handling
 SC-QAM and OFDM/OFDMA channels.
 Object                                                                                            CM           Access
 docsIfDownstreamChannelTable                                                                      M            N-Acc
 docsIfDownstreamChannelEntry                                                                      M            N-Acc
 docsIfDownChannelId                                                                               M            RO
 docsIfDownChannelFrequency                                                                        M            RO
 docsIfDownChannelWidth                                                                            M            RO
 docsIfDownChannelModulation                                                                       M            RO
 docsIfDownChannelInterleave                                                                       M            RO
 docsIfDownChannelPower                                                                            M            RO
 docsIfDownChannelAnnex                                                                            M            RO
 docsIfDownChannelStorageType                                                                      M            RO
 Object                                                                    CM TDMA/     Access     CM SCDMA     Access
                                                                           ATDMA                   upstream
                                                                           upstream
 docsIfUpstreamChannelTable                                                M            N-Acc      O            N-Acc
 docsIfUpstreamChannelEntry                                                M            N-Acc      O            N-Acc
 docsIfUpChannelId                                                         M            RO         O            RO
 docsIfUpChannelFrequency                                                  M            RO         O            RO
 docsIfUpChannelWidth                                                      M            RO         O            RO
 docsIfUpChannelModulationProfile                                          M            RO         O            RO
 docsIfUpChannelSlotSize                                                   M            RO         O            RO
 docsIfUpChannelTxTimingOffset                                             M            RO         O            RO
 docsIfUpChannelRangingBackoffStart                                        M            RO         O            RO
 docsIfUpChannelRangingBackoffEnd                                          M            RO         O            RO
 docsIfUpChannelTxBackoffStart                                             M            RO         O            RO
 docsIfUpChannelTxBackoffEnd                                               M            RO         O            RO
 docsIfUpChannelScdmaActiveCodes                                           O            RO         O            RO
 docsIfUpChannelScdmaCodesPerSlot                                          O            RO         O            RO
 docsIfUpChannelScdmaFrameSize                                             O            RO         O            RO
 docsIfUpChannelScdmaHoppingSeed                                           O            RO         O            RO
 docsIfUpChannelType                                                       M            RO         O            RO
 docsIfUpChannelCloneFrom                                                  O            RO         O            RO
 docsIfUpChannelUpdate                                                     O            RO         O            RO
 docsIfUpChannelStatus                                                     O            RO         O            RO
 docsIfUpChannelPreEqEnable                                                M            RO         O            RO
 Object                                                                                            CM           Access
 docsIfQosProfileTable                                                                             D            N-Acc
 docsIfQosProfileEntry                                                                             D            N-Acc
 docsIfQosProfIndex                                                                                D            N-Acc
 docsIfQosProfPriority                                                                             D            RO
 docsIfQosProfMaxUpBandwidth                                                                       D            RO
 docsIfQosProfGuarUpBandwidth                                                                      D            RO
 docsIfQosProfMaxDownBandwidth                                                                     D            RO
 docsIfQosProfMaxTxBurst                                                                           D            RO




84                                                       CableLabs®                                             02/19/25
<!-- PDF_PAGE: 85 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsIfQosProfBaselinePrivacy                                            D          RO
 docsIfQosProfStatus                                                     D          RO
 docsIfQosProfMaxTransmitBurst                                           D          RO
 docsIfQosProfStorageType                                                D          RO
 docsIfSignalQualityTable                                                M          N-Acc
 docsIfSignalQualityEntry                                                M          N-Acc
 docsIfSigQIncludesContention                                            M          RO
 docsIfSigQUnerroreds                                                    M          RO
 docsIfSigQCorrecteds                                                    M          RO
 docsIfSigQUncorrectables                                                M          RO
 docsIfSigQSignalNoise                                                   D          RO
 docsIfSigQMicroreflections                                              M          RO
 docsIfSigQEqualizationData                                              M          RO
 docsIfSigQExtUnerroreds                                                 M          RO
 docsIfSigQExtCorrecteds                                                 M          RO
 docsIfSigQExtUncorrectables                                             M          RO
 docsIfDocsisBaseCapability                                              D          RO
 docsIfCmMacTable                                                        M          N-Acc
 docsIfCmMacEntry                                                        M          N-Acc
 docsIfCmCmtsAddress                                                     M          RO
 docsIfCmCapabilities                                                    M          RO
 docsIfCmRangingRespTimeout                                              Ob         RW
 docsIfCmRangingTimeout                                                  M          RW
 docsIfCmStatusTable                                                     D          N-Acc
 docsIfCmStatusEntry                                                     D          N-Acc
 docsIfCmStatusValue                                                     D          RO
 docsIfCmStatusCode                                                      D          RO
 docsIfCmStatusTxPower                                                   D          RO
 docsIfCmStatusResets                                                    D          RO
 docsIfCmStatusLostSyncs                                                 D          RO
 docsIfCmStatusInvalidMaps                                               D          RO
 docsIfCmStatusInvalidUcds                                               D          RO
 docsIfCmStatusInvalidRangingResponses                                   D          RO
 docsIfCmStatusInvalidRegistrationResponses                              D          RO
 docsIfCmStatusT1Timeouts                                                D          RO
 docsIfCmStatusT2Timeouts                                                D          RO
 docsIfCmStatusT3Timeouts                                                D          RO
 docsIfCmStatusT4Timeouts                                                D          RO
 docsIfCmStatusRangingAborteds                                           D          RO
 docsIfCmStatusDocsisOperMode                                            D          RO
 docsIfCmStatusModulationType                                            D          RO
 docsIfCmStatusEqualizationData                                          D          RO
 docsIfCmStatusUCCs                                                      D          RO
 docsIfCmStatusUCCFails                                                  D          RO




02/19/25                                      CableLabs®                                    85
<!-- PDF_PAGE: 86 -->
CM-SP-CM-OSSIv3.1-I27-250219                                            Data-Over-Cable Service Interface Specifications


 IF-MIB [RFC 2863]
 Object                                                                                           CM           Access
 ifNumber                                                                                         M            RO
 ifTableLastChange                                                                                M            RO
 ifTable                                                                                          M            N-Acc
 Note: The ifTable Counter32 objects are not reflected here; refer to
 Table 44 and Table 45 of A.2 for details on these objects.
 ifEntry                                                                                          M            N-Acc
 ifIndex                                                                                          M            RO
 ifDescr                                                                                          M            RO
 ifType                                                                                           M            RO
 ifMtu                                                                                            M            RO
 ifSpeed                                                                                          M            RO
 ifPhysAddress                                                                                    M            RO
 ifAdminStatus                                                                                    M            RW
 ifOperStatus                                                                                     M            RO
 ifLastChange                                                                                     M            RO
 ifOutQLen                                                                                        D            RO
 ifSpecific                                                                                       D            RO
 ifXTable                                                                                         M            N-Acc
 Note: The ifXTable Counter32 and Counter64 objects are not
 reflected here; refer to Table 44 and Table 45 of A.2 for details on
 these objects.
 ifXEntry                                                                                         M            N-Acc
 ifName                                                                                           M            RO
 ifLinkUpDownTrapEnable                                                                           M            RW
 ifHighSpeed                                                                                      M            RO
 ifPromiscuousMode                                                                                M            RW/RO
 ifConnectorPresent                                                                               M            RO
 ifAlias                                                                                          M            RW/RO
 ifCounterDiscontinuityTime                                                                       M            RO
 ifStackTable                                                                                     M            N-Acc
 ifStackEntry                                                                                     M            N-Acc
 ifStackHigherLayer                                                                               M            N-Acc
 ifStackLowerLayer                                                                                M            N-Acc
 ifStackStatus                                                                                    M            RC/RO
 ifStackLastChange                                                                                M            RO
 ifRcvAddressTable                                                                                O            N-Acc
 ifRcvAddressEntry                                                                                O            N-Acc
 ifRcvAddressAddress                                                                              O            N-Acc
 ifRcvAddressStatus                                                                               O            RC
 IfRcvAddressType                                                                                 O            RC
 Notification
 linkUp                                                                                           M            Acc-FN
 linkDown                                                                                         M            Acc-FN
 Note: See Section 7.1.3.8.4 for details.
 ifTestTable                                                                                      D            N-Acc
 ifTestEntry                                                                                      D            N-Acc




86                                                         CableLabs®                                          02/19/25
<!-- PDF_PAGE: 87 -->
Cable Modem Operations Support System Interface Specification       CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                      CM         Access
 ifTestId                                                                    D          RW
 ifTestStatus                                                                D          RW
 ifTestType                                                                  D          RW
 ifTestResult                                                                D          RO
 ifTestCode                                                                  D          RO
 ifTestOwner                                                                 D          RW
 BRIDGE-MIB [RFC 4188]
 Object                                                                      CM         Access
 dot1dBase
 dot1dBaseBridgeAddress                                                      M          RO
 dot1dBaseNumPorts                                                           M          RO
 dot1dBaseType                                                               M          RO
 dot1dBasePortTable                                                          M          N-Acc
 dot1dBasePortEntry                                                          M          N-Acc
 dot1dBasePort                                                               M          RO
 dot1dBasePortIfIndex                                                        M          RO
 dot1dBasePortCircuit                                                        M          RO
 dot1dBasePortDelayExceededDiscards                                          M          RO
 dot1dBasePortMtuExceededDiscards                                            M          RO
 dot1dStp
 dot1dStpProtocolSpecification                                               M          RO
 dot1dStpPriority                                                            M          RW
 dot1dStpTimeSinceTopologyChange                                             M          RO
 dot1dStpTopChanges                                                          M          RO
 dot1dStpDesignatedRoot                                                      M          RO
 dot1dStpRootCost                                                            M          RO
 dot1dStpRootPort                                                            M          RO
 dot1dStpMaxAge                                                              M          RO
 dot1dStpHelloTime                                                           M          RO
 dot1dStpHoldTime                                                            M          RO
 dot1dStpForwardDelay                                                        M          RO
 dot1dStpBridgeMaxAge                                                        M          RW
 dot1dStpBridgeHelloTime                                                     M          RW
 dot1dStpBridgeForwardDelay                                                  M          RW
 dot1dStpPortTable                                                           O          N-Acc
 Note: This table is required ONLY if STP is implemented.
 dot1dStpPortEntry                                                           O          N-Acc
 dot1dStpPort                                                                O          RO
 dot1dStpPortPriority                                                        O          RW
 dot1dStpPortState                                                           O          RO
 dot1dStpPortEnable                                                          O          RW
 dot1dStpPortPathCost                                                        O          RW
 dot1dStpPortDesignatedRoot                                                  O          RO
 dot1dStpPortDesignatedCost                                                  O          RO
 dot1dStpPortDesignatedBridge                                                O          RO
 dot1dStpPortDesignatedPort                                                  O          RO




02/19/25                                               CableLabs®                               87
<!-- PDF_PAGE: 88 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


 Object                                                                                   CM            Access
 dot1dStpPortForwardTransitions                                                           O             RO
 dot1dStpPortPathCost32                                                                   O             RO
 dot1dTp
 Note: This group is required ONLY if transparent bridging is
 implemented.
 dot1dTpLearnedEntryDiscards                                                              M             RO
 dot1dTpAgingTime                                                                         M             RW
 dot1dTpFdbTable                                                                          M             N-Acc
 dot1dTpFdbEntry                                                                          M             N-Acc
 dot1dTpFdbAddress                                                                        M             RO
 dot1dTpFdbPort                                                                           M             RO
 dot1dTpFdbStatus                                                                         M             RO
 dot1dTpPortTable                                                                         M             N-Acc
 dot1dTpPortEntry                                                                         M             N-Acc
 dot1dTpPort                                                                              M             RO
 dot1dTpPortMaxInfo                                                                       M             RO
 dot1dTpPortInFrames                                                                      M             RO
 dot1dTpPortOutFrames                                                                     M             RO
 dot1dTpPortInDiscards                                                                    M             RO
 dot1dStaticTable                                                                         O             N-Acc
 Note: Implementation of dot1dStaticTable is OPTIONAL.
 dot1dStaticEntry                                                                         O             N-Acc
 dot1dStaticAddress                                                                       O             RW
 dot1dStaticReceivePort                                                                   O             RW
 dot1dStaticAllowedToGoTo                                                                 O             RW
 dot1dStaticStatus                                                                        O             RW
 Notification
 newRoot                                                                                  O             Acc-FN
 topologyChange                                                                           O             Acc-FN
 DOCS-CABLE-DEVICE-MIB [RFC 4639]
 Object                                                                                   CM            Access
 docsDevBase
 docsDevRole                                                                              M             RO
 docsDevDateTime                                                                          M             RO/RW
 docsDevResetNow                                                                          M             RW
 docsDevSerialNumber                                                                      M             RO
 docsDevSTPControl                                                                        M             RW/RO
 docsDevIgmpModeControl                                                                   N-Sup
 docsDevMaxCpe                                                                            M             RO
 Object                                                           CM in        Access     CM in SNMP    Access
                                                                  NmAccess                Coexistence
                                                                  Mode                    Mode
 docsDevNmAccessTable                                             M            N-Acc      N-Sup
 docsDevNmAccessEntry                                             M            N-Acc      N-Sup
 docsDevNmAccessIndex                                             M            N-Acc      N-Sup
 docsDevNmAccessIp                                                M            RC         N-Sup
 docsDevNmAccessIpMask                                            M            RC         N-Sup




88                                                       CableLabs®                                     02/19/25
<!-- PDF_PAGE: 89 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 docsDevNmAccessCommunity                              M          RC       N-Sup
 docsDevNmAccessControl                                M          RC       N-Sup
 docsDevNmAccessInterfaces                             M          RC       N-Sup
 docsDevNmAccessStatus                                 M          RC       N-Sup
 docsDevNmAccessTrapVersion                            M          RC       N-Sup
 Object                                                                    CM            Access
 docsDevSoftware
 docsDevSwServer                                                           D             RW
 docsDevSwFilename                                                         M             RW
 docsDevSwAdminStatus                                                      M             RW
 docsDevSwOperStatus                                                       M             RO
 docsDevSwCurrentVers                                                      M             RO
 docsDevSwServerAddressType                                                M             RW
 docsDevSwServerAddress                                                    M             RW
 docsDevSwServerTransportProtocol                                          M             RW
 docsDevServer
 docsDevServerBootState                                                    D             RO
 docsDevServerDhcp                                                         D             RO
 docsDevServerTime                                                         D             RO
 docsDevServerTftp                                                         D             RO
 docsDevServerConfigFile                                                   M             RO
 docsDevServerDhcpAddressType                                              M             RO
 docsDevServerDhcpAddress                                                  M             RO
 docsDevServerTimeAddressType                                              M             RO
 docsDevServerTimeAddress                                                  M             RO
 docsDevServerConfigTftpAddressType                                        M             RO
 docsDevServerConfigTftpAddress                                            M             RO
 docsDevEvent
 docsDevEvControl                                                          M             RW
 docsDevEvSyslog                                                           D             RW
 docsDevEvThrottleAdminStatus                                              M             RW
 docsDevEvThrottleInhibited                                                D             RO
 docsDevEvThrottleThreshold                                                M             RW
 docsDevEvThrottleInterval                                                 M             RW
 docsDevEvControlTable                                                     M             N-Acc
 docsDevEvControlEntry                                                     M             N-Acc
 docsDevEvPriority                                                         M             N-Acc
 docsDevEvReporting                                                        M             RW
 docsDevEventTable                                                         M             N-Acc
 docsDevEventEntry                                                         M             N-Acc
 docsDevEvIndex                                                            M             N-Acc
 docsDevEvFirstTime                                                        M             RO
 docsDevEvLastTime                                                         M             RO
 docsDevEvCounts                                                           M             RO




02/19/25                                   CableLabs®                                            89
<!-- PDF_PAGE: 90 -->
CM-SP-CM-OSSIv3.1-I27-250219                 Data-Over-Cable Service Interface Specifications


 Object                                                                CM           Access
 docsDevEvLevel                                                        M            RO
 docsDevEvId                                                           M            RO
 docsDevEvText                                                         M            RO
 docsDevEvSyslogAddressType                                            M            RW
 docsDevEvSyslogAddress                                                M            RW
 docsDevEvThrottleThresholdExceeded                                    M            RO
 docsDevFilter
 docsDevFilterLLCUnmatchedAction                                       D            RW
 docsDevFilterLLCTable                                                 D            N-Acc
 docsDevFilterLLCEntry                                                 D            N-Acc
 docsDevFilterLLCIndex                                                 D            N-Acc
 docsDevFilterLLCStatus                                                D            RC
 docsDevFilterLLCIfIndex                                               D            RC
 docsDevFilterLLCProtocolType                                          D            RC
 docsDevFilterLLCProtocol                                              D            RC
 docsDevFilterLLCMatches                                               D            RO
 docsDevFilterIpDefault                                                D            RW
 docsDevFilterIpTable                                                  D            N-Acc
 docsDevFilterIpEntry                                                  D            N-Acc
 docsDevFilterIpIndex                                                  D            N-Acc
 docsDevFilterIpStatus                                                 D            RC
 docsDevFilterIpControl                                                D            RC
 docsDevFilterIpIfIndex                                                D            RC
 docsDevFilterIpDirection                                              D            RC
 docsDevFilterIpBroadcast                                              D            RC
 docsDevFilterIpSaddr                                                  D            RC
 docsDevFilterIpSmask                                                  D            RC
 docsDevFilterIpDaddr                                                  D            RC
 docsDevFilterIpDmask                                                  D            RC
 docsDevFilterIpProtocol                                               D            RC
 docsDevFilterIpSourcePortLow                                          D            RC
 docsDevFilterIpSourcePortHigh                                         D            RC
 docsDevFilterIpDestPortLow                                            D            RC
 docsDevFilterIpDestPortHigh                                           D            RC
 docsDevFilterIpMatches                                                D            RO
 docsDevFilterIpTos                                                    D            RC
 docsDevFilterIpTosMask                                                D            RC
 docsDevFilterIpContinue                                               D            RC
 docsDevFilterIpPolicyId                                               D            RC
 docsDevFilterPolicyTable                                              D            N-Acc
 docsDevFilterPolicyEntry                                              D            N-Acc
 docsDevFilterPolicyIndex                                              D            N-Acc
 docsDevFilterPolicyId                                                 D            RC
 docsDevFilterPolicyStatus                                             D            RC
 docsDevFilterPolicyPtr                                                D            RC
 docsDevFilterTosTable                                                 D            N-Acc




90                                    CableLabs®                                    02/19/25
<!-- PDF_PAGE: 91 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsDevFilterTosEntry                                                   D          N-Acc
 docsDevFilterTosIndex                                                   D          N-Acc
 docsDevFilterTosStatus                                                  D          RC
 docsDevFilterTosAndMask                                                 D          RC
 docsDevFilterTosOrMask                                                  D          RC
 docsDevCpeEnroll                                                        O          RW
 docsDevCpeIpMax                                                         O          RW
 docsDevCpeTable                                                         Ob         N-Acc
 docsDevCpeEntry                                                         Ob         N-Acc
 docsDevCpeIp                                                            Ob         N-Acc
 docsDevCpeSource                                                        Ob         RO
 docsDevCpeStatus                                                        Ob         RC
 docsDevCpeInetTable                                                     O          N-Acc
 docsDevCpeInetEntry                                                     O          N-Acc
 docsDevCpeInetType                                                      O          N-Acc
 docsDevCpeInetAddr                                                      O          RC
 docsDevCpeInetSource                                                    O          RO
 docsDevCpeInetRowStatus                                                 O          RC
 IP-MIB [RFC 4293]
 Object                                                                  CM         Access
 ipv4GeneralGroup
 ipForwarding                                                            M          RW
 ipDefaultTTL                                                            M          RW
 ipReasmTimeout                                                          M          RW
 ipv6GeneralGroup2
 ipv6IpForwarding                                                        M          RW
 ipv6IpDefaultHopLimit                                                   M          RW
 ipv4InterfaceTableLastChange                                            M          RO
 ipv4InterfaceTable                                                      M          N-Acc
 ipv4InterfaceEntry                                                      M          N-Acc
 ipv4InterfaceIfIndex                                                    M          N-Acc
 ipv4InterfaceReasmMaxSize                                               M          RO
 ipv4InterfaceEnableStatus                                               M          RW
 ipv4InterfaceRetransmitTime                                             M          RO
 ipv6InterfaceTableLastChange                                            M          RO
 ipv6InterfaceTable                                                      M          N-Acc
 ipv6InterfaceEntry                                                      M          N-Acc
 ipv6InterfaceIfIndex                                                    M          N-Acc
 ipv6InterfaceReasmMaxSize                                               M          RO
 ipv6InterfaceIdentifier                                                 M          RO
 ipv6InterfaceEnableStatus                                               M          RW
 ipv6InterfaceReachableTime                                              M          RO
 ipv6InterfaceRetransmitTime                                             M          RO
 ipv6InterfaceForwarding                                                 M          RW
 ipSystemStatsTable                                                      O          N-Acc
 ipSystemStatsEntry                                                      O          N-Acc




02/19/25                                   CableLabs®                                       91
<!-- PDF_PAGE: 92 -->
CM-SP-CM-OSSIv3.1-I27-250219              Data-Over-Cable Service Interface Specifications


 Object                                                             CM           Access
 ipSystemStatsIPVersion                                             O            N-Acc
 ipSystemStatsInReceives                                            O            RO
 ipSystemStatsHCInReceives                                          O            RO
 ipSystemStatsInOctets                                              O            RO
 ipSystemStatsHCInOctets                                            O            RO
 ipSystemStatsInHdrErrors                                           O            RO
 ipSystemStatsInNoRoutes                                            O            RO
 ipSystemStatsInAddrErrors                                          O            RO
 ipSystemStatsInUnknownProtos                                       O            RO
 ipSystemStatsInTruncatedPkts                                       O            RO
 ipSystemStatsInForwDatagrams                                       O            RO
 ipSystemStatsHCInForwDatagrams                                     O            RO
 ipSystemStatsReasmReqds                                            O            RO
 ipSystemStatsReasmOKs                                              O            RO
 ipSystemStatsReasmFails                                            O            RO
 ipSystemStatsInDiscards                                            O            RO
 ipSystemStatsInDelivers                                            O            RO
 ipSystemStatsHCInDelivers                                          O            RO
 ipSystemStatsOutRequests                                           O            RO
 ipSystemStatsHCOutRequests                                         O            RO
 ipSystemStatsOutNoRoutes                                           O            RO
 ipSystemStatsOutForwDatagrams                                      O            RO
 ipSystemStatsHCOutForwDatagrams                                    O            RO
 ipSystemStatsOutDiscards                                           O            RO
 ipSystemStatsOutFragReqds                                          O            RO
 ipSystemStatsOutFragOKs                                            O            RO
 ipSystemStatsOutFragFails                                          O            RO
 ipSystemStatsOutFragCreates                                        O            RO
 ipSystemStatsOutTransmits                                          O            RO
 ipSystemStatsHCOutTransmits                                        O            RO
 ipSystemStatsOutOctets                                             O            RO
 ipSystemStatsHCOutOctets                                           O            RO
 ipSystemStatsInMcastPkts                                           O            RO
 ipSystemStatsHCInMcastPkts                                         O            RO
 ipSystemStatsInMcastOctets                                         O            RO
 ipSystemStatsHCInMcastOctets                                       O            RO
 ipSystemStatsOutMcastPkts                                          O            RO
 ipSystemStatsHCOutMcastPkts                                        O            RO
 ipSystemStatsOutMcastOctets                                        O            RO
 ipSystemStatsHCOutMcastOctets                                      O            RO
 ipSystemStatsInBcastPkts                                           O            RO
 ipSystemStatsHCInBcastPkts                                         O            RO
 ipSystemStatsOutBcastPkts                                          O            RO
 ipSystemStatsHCOutBcastPkts                                        O            RO
 ipSystemStatsDiscontinuityTime                                     O            RO
 ipSystemStatsRefreshRate                                           O            RO




92                                 CableLabs®                                    02/19/25
<!-- PDF_PAGE: 93 -->
Cable Modem Operations Support System Interface Specification         CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                        CM         Access
 ipIfStatsTableLastChange                                                      O          RO
 ipIfStatsTable                                                                O          N-Acc
 Note: This table is required ONLY if routing is implemented.
 ipIfStatsEntry                                                                O          N-Acc
 ipIfStatsIPVersion                                                            O          N-Acc
 ipIfStatsIfIndex                                                              O          N-Acc
 ipIfStatsInReceives                                                           O          RO
 ipIfStatsHCInReceives                                                         O          RO
 ipIfStatsInOctets                                                             O          RO
 ipIfStatsHCInOctets                                                           O          RO
 ipIfStatsInHdrErrors                                                          O          RO
 ipIfStatsInNoRoutes                                                           O          RO
 ipIfStatsInAddrErrors                                                         O          RO
 ipIfStatsInUnknownProtos                                                      O          RO
 ipIfStatsInTruncatedPkts                                                      O          RO
 ipIfStatsInForwDatagrams                                                      O          RO
 ipIfStatsHCInForwDatagrams                                                    O          RO
 ipIfStatsReasmReqds                                                           O          RO
 ipIfStatsReasmOKs                                                             O          RO
 ipIfStatsReasmFails                                                           O          RO
 ipIfStatsInDiscards                                                           O          RO
 ipIfStatsInDelivers                                                           O          RO
 ipIfStatsHCInDelivers                                                         O          RO
 ipIfStatsOutRequests                                                          O          RO
 ipIfStatsHCOutRequests                                                        O          RO
 ipIfStatsOutForwDatagrams                                                     O          RO
 ipIfStatsHCOutForwDatagrams                                                   O          RO
 ipIfStatsOutDiscards                                                          O          RO
 ipIfStatsOutFragReqds                                                         O          RO
 ipIfStatsOutFragOKs                                                           O          RO
 ipIfStatsOutFragFails                                                         O          RO
 ipIfStatsOutFragCreates                                                       O          RO
 ipIfStatsOutTransmits                                                         O          RO
 ipIfStatsHCOutTransmits                                                       O          RO
 ipIfStatsOutOctets                                                            O          RO
 ipIfStatsHCOutOctets                                                          O          RO
 ipIfStatsInMcastPkts                                                          O          RO
 ipIfStatsHCInMcastPkts                                                        O          RO
 ipIfStatsInMcastOctets                                                        O          RO
 ipIfStatsHCInMcastOctets                                                      O          RO
 ipIfStatsOutMcastPkts                                                         O          RO
 ipIfStatsHCOutMcastPkts                                                       O          RO
 ipIfStatsOutMcastOctets                                                       O          RO
 ipIfStatsHCOutMcastOctets                                                     O          RO
 ipIfStatsInBcastPkts                                                          O          RO
 ipIfStatsHCInBcastPkts                                                        O          RO




02/19/25                                                 CableLabs®                               93
<!-- PDF_PAGE: 94 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


 Object                                                                                   CM           Access
 ipIfStatsOutBcastPkts                                                                    O            RO
 ipIfStatsHCOutBcastPkts                                                                  O            RO
 ipIfStatsDiscontinuityTime                                                               O            RO
 ipIfStatsRefreshRate                                                                     O            RO
 ipAddressPrefixTable                                                                     O            N-Acc
 Note: This table is required ONLY if routing is implemented.
 ipAddressPrefixEntry                                                                     O            N-Acc
 ipAddressPrefixIfIndex                                                                   O            N-Acc
 ipAddressPrefixType                                                                      O            N-Acc
 ipAddressPrefixPrefix                                                                    O            N-Acc
 ipAddressPrefixLength                                                                    O            N-Acc
 ipAddressPrefixOrigin                                                                    O            RO
 ipAddressPrefixOnLinkFlag                                                                O            RO
 ipAddressPrefixAutonomousFlag                                                            O            RO
 ipAddressPrefixAdvPreferredLifetime                                                      O            RO
 ipAddressPrefixAdvValidLifetime                                                          O            RO
 ipAddressSpinLock                                                                        O            RW
 ipAddressTable                                                                           O            N-Acc
 ipAddressEntry                                                                           O            N-Acc
 ipAddressAddrType                                                                        O            N-Acc
 ipAddressAddr                                                                            O            N-Acc
 ipAddressIfIndex                                                                         O            RC
 ipAddressType                                                                            O            RC
 ipAddressPrefix                                                                          O            RO
 ipAddressOrigin                                                                          O            RO
 ipAddressStatus                                                                          O            RC
 ipAddressCreated                                                                         O            RC
 ipAddressLastChanged                                                                     O            RC
 ipAddressRowStatus                                                                       O            RC
 ipAddressStorageType                                                                     O            RC
 ipNetToPhysicalTable                                                                     O            N-Acc
 Note: This table is required ONLY if routing is implemented.
 ipNetToPhysicalEntry                                                                     O            N-Acc
 ipNetToPhysicalIfIndex                                                                   O            N-Acc
 ipNetToPhysicalNetAddressType                                                            O            N-Acc
 ipNetToPhysicalNetAddress                                                                O            N-Acc
 ipNetToPhysicalPhysAddress                                                               O            RC
 ipNetToPhysicalLastUpdated                                                               O            RO
 ipNetToPhysicalType                                                                      O            RC
 ipNetToPhysicalState                                                                     O            RO
 ipNetToPhysicalRowStatus                                                                 O            RC
 ipDefaultRouterTable                                                                     O            N-Acc
 Note: This table is required ONLY if routing is implemented.
 ipDefaultRouterEntry                                                                     O            N-Acc
 ipDefaultRouterAddressType                                                               O            N-Acc
 ipDefaultRouterAddress                                                                   O            N-Acc




94                                                       CableLabs®                                    02/19/25
<!-- PDF_PAGE: 95 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 ipDefaultRouterIfIndex                                                  O          N-Acc
 ipDefaultRouterLifetime                                                 O          RC
 ipDefaultRouterPreference                                               O          RO
 icmpStatsTable                                                          M          N-Acc
 icmpStatsEntry                                                          M          N-Acc
 icmpStatsIPVersion                                                      M          N-Acc
 icmpStatsInMsgs                                                         M          RO
 icmpStatsInErrors                                                       M          RO
 icmpStatsOutMsgs                                                        M          RO
 icmpStatsOutErrors                                                      M          RO
 icmpMsgStatsTable                                                       M          N-Acc
 icmpMsgStatsEntry                                                       M          N-Acc
 icmpMsgStatsIPVersion                                                   M          N-Acc
 icmpMsgStatsType                                                        M          N-Acc
 icmpMsgStatsInPkts                                                      M          RO
 icmpMsgStatsOutPkts                                                     M          RO
 UDP-MIB [RFC 4113]
 Object                                                                  CM         Access
 UDPGroup
 udpInDatagrams                                                          O          RO
 udpNoPorts                                                              O          RO
 udpInErrors                                                             O          RO
 udpOutDatagrams                                                         O          RO
 udpEndpointTable                                                        O          N-Acc
 udpEndpointEntry                                                        O          N-Acc
 udpEndpointLocalAddressType                                             O          N-Acc
 udpEndpointLocalAddress                                                 O          N-Acc
 udpEndpointLocalPort                                                    O          N-Acc
 udpEndpointRemoteAddressType                                            O          N-Acc
 udpEndpointRemoteAddress                                                O          N-Acc
 udpEndpointRemotePort                                                   O          N-Acc
 udpEndpointInstance                                                     O          N-Acc
 udpEndpointProcess                                                      O          RO
 TCP-MIB [RFC 4022]
 Object                                                                  CM         Access
 tcpBaseGroup
 tcpRtoAlgorithm                                                         O          RO
 tcpRtoMin                                                               O          RO
 tcpRtoMax                                                               O          RO
 tcpMaxConn                                                              O          RO
 tcpActiveOpens                                                          O          RO
 tcpPassiveOpens                                                         O          RO
 tcpAttemptFails                                                         O          RO
 tcpEstabResets                                                          O          RO
 tcpCurrEstab                                                            O          RO
 tcpInSegs                                                               O          RO




02/19/25                                   CableLabs®                                       95
<!-- PDF_PAGE: 96 -->
CM-SP-CM-OSSIv3.1-I27-250219            Data-Over-Cable Service Interface Specifications


 Object                                                           CM           Access
 tcpOutSegs                                                       O            RO
 tcpRetransSegs                                                   O            RO
 tcpInErrs                                                        O            RO
 tcpOutRsts                                                       O            RO
 tcpHCGroup
 tcpHCInSegs                                                      O            RO
 tcpHCOutSegs                                                     O            RO
 tcpConnectionTable                                               O            N-Acc
 tcpConnectionEntry                                               O            N-Acc
 tcpConnectionLocalAddressType                                    O            N-Acc
 tcpConnectionLocalAddress                                        O            N-Acc
 tcpConnectionLocalPort                                           O            N-Acc
 tcpConnectionRemAddressType                                      O            N-Acc
 tcpConnectionRemAddress                                          O            N-Acc
 tcpConnectionRemPort                                             O            N-Acc
 tcpConnectionState                                               O            RW
 tcpConnectionProcess                                             O            RO
 tcpListenerTable                                                 O            N-Acc
 tcpListenerEntry                                                 O            N-Acc
 tcpListenerLocalAddressType                                      O            N-Acc
 tcpListenerLocalAddress                                          O            N-Acc
 tcpListenerLocalPort                                             O            N-Acc
 tcpListenerProcess                                               O            RO
 SNMPv2-MIB [RFC 3418]
 Object                                                           CM           Access
 SystemGroup
 sysDescr                                                         M            RO
 sysObjectID                                                      M            RO
 sysUpTime                                                        M            RO
 sysContact                                                       M            RW
 sysName                                                          M            RW
 sysLocation                                                      M            RW
 sysServices                                                      M            RO
 sysORLastChange                                                  M            RO
 sysORTable                                                       M            N-Acc
 sysOREntry                                                       M            N-Acc
 sysORIndex                                                       M            N-Acc
 sysORID                                                          M            RO
 sysORDescr                                                       M            RO
 sysORUpTime                                                      M            RO
 SNMPGroup
 snmpInPkts                                                       M            RO
 snmpInBadVersions                                                M            RO
 snmpOutPkts                                                      Ob           RO
 snmpInBadCommunityNames                                          M            RO
 snmpInBadCommunityUses                                           M            RO




96                               CableLabs®                                    02/19/25
<!-- PDF_PAGE: 97 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 snmpInASNParseErrs                                                      M          RO
 snmpInTooBigs                                                           Ob         RO
 snmpInNoSuchNames                                                       Ob         RO
 snmpInBadValues                                                         Ob         RO
 snmpInReadOnlys                                                         Ob         RO
 snmpInGenErrs                                                           Ob         RO
 snmpInTotalReqVars                                                      Ob         RO
 snmpInTotalSetVars                                                      Ob         RO
 snmpInGetRequests                                                       Ob         RO
 snmpInGetNexts                                                          Ob         RO
 snmpInSetRequests                                                       Ob         RO
 snmpInGetResponses                                                      Ob         RO
 snmpInTraps                                                             Ob         RO
 snmpOutTooBigs                                                          Ob         RO
 snmpOutNoSuchNames                                                      Ob         RO
 snmpOutBadValues                                                        Ob         RO
 snmpOutGenErrs                                                          Ob         RO
 snmpOutGetRequests                                                      Ob         RO
 snmpOutGetNexts                                                         Ob         RO
 snmpOutSetRequests                                                      Ob         RO
 snmpOutGetResponses                                                     Ob         RO
 snmpOutTraps                                                            Ob         RO
 snmpEnableAuthenTraps                                                   M          RW
 snmpSilentDrops                                                         M          RO
 snmpProxyDrops                                                          M          RO
 snmpTrapsGroup
 coldStart                                                               O          Acc-FN
 warmStart                                                               O          Acc-FN
 authenticationFailure                                                   M          Acc-FN
 snmpSetGroup
 snmpSetSerialNo                                                         M          RW
 Etherlike-MIB [RFC 3635]
 Object                                                                  CM         Access
 dot3StatsTable                                                          O          N-Acc
 dot3StatsEntry                                                          O          N-Acc
 dot3StatsIndex                                                          O          RO
 dot3StatsAlignmentErrors                                                O          RO
 dot3StatsFCSErrors                                                      O          RO
 dot3StatsInternalMacTransmitErrors                                      O          RO
 dot3StatsFrameTooLongs                                                  O          RO
 dot3StatsInternalMacReceiveErrors                                       O          RO
 dot3StatsSymbolErrors                                                   O          RO
 dot3StatsSingleCollisionFrames                                          O          RO
 dot3StatsMultipleCollisionFrames                                        O          RO
 dot3StatsDeferredTransmissions                                          O          RO
 dot3StatsLateCollisions                                                 O          RO




02/19/25                                   CableLabs®                                        97
<!-- PDF_PAGE: 98 -->
CM-SP-CM-OSSIv3.1-I27-250219              Data-Over-Cable Service Interface Specifications


 Object                                                             CM           Access
 dot3StatsExcessiveCollisions                                       O            RO
 dot3StatsCarrierSenseErrors                                        O            RO
 dot3StatsDuplexStatus                                              O            RO
 dot3StatsSQETestErrors                                             O            RO
 dot3CollTable                                                      O            N-Acc
 dot3CollEntry                                                      O            N-Acc
 dot3CollCount                                                      O            NA
 dot3CollFrequencies                                                O            RO
 dot3ControlTable                                                   O            N-Acc
 dot3ControlEntry                                                   O            N-Acc
 dot3ControlFunctionsSupported                                      O            RO
 dot3ControlInUnknownOpcodes                                        O            RO
 dot3PauseTable                                                     O            N-Acc
 dot3PauseEntry                                                     O            N-Acc
 dot3PauseAdminMode                                                 O            RW
 dot3PauseOperMode                                                  O            RO
 dot3InPauseFrames                                                  O            RO
 dot3OutPauseFrames                                                 O            RO
 DOCS-IETF-BPI2-MIB [RFC 4131]
 Object                                                             CM           Access
 docsBpi2CmBaseTable                                                M            N-Acc
 docsBpi2CmBaseEntry                                                M            N-Acc
 docsBpi2CmPrivacyEnable                                            M            RO
 docsBpi2CmPublicKey                                                M            RO
 docsBpi2CmAuthState                                                M            RO
 docsBpi2CmAuthKeySequenceNumber                                    M            RO
 docsBpi2CmAuthExpiresOld                                           M            RO
 docsBpi2CmAuthExpiresNew                                           M            RO
 docsBpi2CmAuthReset                                                M            RW
 docsBpi2CmAuthGraceTime                                            M            RO
 docsBpi2CmTEKGraceTime                                             M            RO
 docsBpi2CmAuthWaitTimeout                                          M            RO
 docsBpi2CmReauthWaitTimeout                                        M            RO
 docsBpi2CmOpWaitTimeout                                            M            RO
 docsBpi2CmRekeyWaitTimeout                                         M            RO
 docsBpi2CmAuthRejectWaitTimeout                                    M            RO
 docsBpi2CmSAMapWaitTimeout                                         M            RO
 docsBpi2CmSAMapMaxRetries                                          M            RO
 docsBpi2CmAuthentInfos                                             M            RO
 docsBpi2CmAuthRequests                                             M            RO
 docsBpi2CmAuthReplies                                              M            RO
 docsBpi2CmAuthRejects                                              M            RO
 docsBpi2CmAuthInvalids                                             M            RO
 docsBpi2CmAuthRejectErrorCode                                      M            RO
 docsBpi2CmAuthRejectErrorString                                    M            RO
 docsBpi2CmAuthInvalidErrorCode                                     M            RO




98                                 CableLabs®                                    02/19/25
<!-- PDF_PAGE: 99 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsBpi2CmAuthInvalidErrorString                                        M          RO
 docsBpi2CmTEKTable                                                      M          N-Acc
 docsBpi2CmTEKEntry                                                      M          N-Acc
 docsBpi2CmTEKSAId                                                       M          N-Acc
 docsBpi2CmTEKSAType                                                     M          RO
 docsBpi2CmTEKDataEncryptAlg                                             M          RO
 docsBpi2CmTEKDataAuthentAlg                                             M          RO
 docsBpi2CmTEKState                                                      M          RO
 docsBpi2CmTEKKeySequenceNumber                                          M          RO
 docsBpi2CmTEKExpiresOld                                                 M          RO
 docsBpi2CmTEKExpiresNew                                                 M          RO
 docsBpi2CmTEKKeyRequests                                                M          RO
 docsBpi2CmTEKKeyReplies                                                 M          RO
 docsBpi2CmTEKKeyRejects                                                 M          RO
 docsBpi2CmTEKInvalids                                                   M          RO
 docsBpi2CmTEKAuthPends                                                  M          RO
 docsBpi2CmTEKKeyRejectErrorCode                                         M          RO
 docsBpi2CmTEKKeyRejectErrorString                                       M          RO
 docsBpi2CmTEKInvalidErrorCode                                           M          RO
 docsBpi2CmTEKInvalidErrorString                                         M          RO
 docsBpi2CmIpMulticastMapTable                                           D          N-Acc
 docsBpi2CmIpMulticastMapEntry                                           D          N-Acc
 docsBpi2CmIpMulticastIndex                                              D          N-Acc
 docsBpi2CmIpMulticastAddressType                                        D          RO
 docsBpi2CmIpMulticastAddress                                            D          RO
 docsBpi2CmIpMulticastSAId                                               D          RO
 docsBpi2CmIpMulticastSAMapState                                         D          RO
 docsBpi2CmIpMulticastSAMapRequests                                      D          RO
 docsBpi2CmIpMulticastSAMapReplies                                       D          RO
 docsBpi2CmIpMulticastSAMapRejects                                       D          RO
 docsBpi2CmIpMulticastSAMapRejectErrorCode                               D          RO
 docsBpi2CmIpMulticastSAMapRejectErrorString                             D          RO
 docsBpi2CmDeviceCertTable                                               M          N-Acc
 docsBpi2CmDeviceCertEntry                                               M          N-Acc
 docsBpi2CmDeviceCmCert                                                  M          RW/RO
 docsBpi2CmDeviceManufCert                                               M          RO
 docsBpi2CmCryptoSuiteTable                                              M          N-Acc
 docsBpi2CmCryptoSuiteEntry                                              M          N-Acc
 docsBpi2CmCryptoSuiteIndex                                              M          N-Acc
 docsBpi2CmCryptoSuiteDataEncryptAlg                                     M          RO
 docsBpi2CmCryptoSuiteDataAuthentAlg                                     M          RO
 docsBpi2CodeDownloadGroup
 docsBpi2CodeDownloadStatusCode                                          M          RO
 docsBpi2CodeDownloadStatusString                                        M          RO
 docsBpi2CodeMfgOrgName                                                  M          RO
 docsBpi2CodeMfgCodeAccessStart                                          M          RO




02/19/25                                       CableLabs®                                   99
<!-- PDF_PAGE: 100 -->
CM-SP-CM-OSSIv3.1-I27-250219                         Data-Over-Cable Service Interface Specifications


 Object                                                                        CM           Access
 docsBpi2CodeMfgCvcAccessStart                                                 M            RO
 docsBpi2CodeCoSignerOrgName                                                   M            RO
 docsBpi2CodeCoSignerCodeAccessStart                                           M            RO
 docsBpi2CodeCoSignerCvcAccessStart                                            M            RO
 docsBpi2CodeCvcUpdate                                                         M            RW
 DOCS-IFEXT2-MIB
 Object                                                                        CM           Access
 docsIfExt2CmMscStatusTable                                                    M            N-Acc
 docsIfExt2CmMscStatusEntry                                                    M            N-Acc
 docsIfExt2CmMscStatusState                                                    M            RO
 docsIfExt2CmMscStatusPowerShortfall                                           M            RO
 docsIfExt2CmMscStatusCodeRatio                                                M            RO
 docsIfExt2CmMscStatusMaximumScheduledCodes                                    M            RO
 docsIfExt2CmMscStatusPowerHeadroom                                            M            RO
 docsIfExt2CmMscStatusEffectivePower                                           M            RO
 docsIfExt2CmMscStatusIUC2Control                                              M            RW
 docsIfExt2CmClearLearnedMacAddresses                                          M            RW
 HOST-RESOURCES-MIB [RFC 2790]
 Object                                                                        CM           Access
 hrDeviceTable                                                                 O            N-Acc
 hrDeviceEntry                                                                 O            N-Acc
 hrDeviceIndex                                                                 O            RO
 hrDeviceType                                                                  O            RO
 hrDeviceDescr                                                                 O            RO
 hrDeviceID                                                                    O            RO
 hrDeviceStatus                                                                O            RO
 hrDeviceErrors                                                                O            RO
 hrSystem
 hrMemorySize                                                                  O            RO
 hrStorageTable                                                                O            N-Acc
 hrStorageEntry                                                                O            N-Acc
 hrStorageIndex                                                                O            RO
 hrStorageType                                                                 O            RO
 hrStorageDescr                                                                O            RO
 hrStorageAllocationUnits                                                      O            RO
 hrStorageSize                                                                 O            RO
 hrStorageUsed                                                                 O            RO
 hrStorageAllocationFailures                                                   O            RO
 hrSWRunTable                                                                  O            N-Acc
 hrSWRunEntry                                                                  O            N-Acc
 hrSWRunIndex                                                                  O            RO
 hrSWRunName                                                                   O            RO
 hrSWRunID                                                                     O            RO
 hrSWRunPath                                                                   O            RO
 hrSWRunParameters                                                             O            RO
 hrSWRunType                                                                   O            RO




100                                           CableLabs®                                    02/19/25
<!-- PDF_PAGE: 101 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 hrSWRunStatus                                                           O          RO
 hrSWRunPerfTable                                                        O          N-Acc
 hrSWRunPerfEntry                                                        O          N-Acc
 hrSWRunIndex                                                            O          N-Acc
 hrSWRunPerfCPU                                                          O          RO
 hrSWRunPerfMem                                                          O          RO
 hrProcessorTable                                                        O          N-Acc
 hrProcessorEntry                                                        O          N-Acc
 hrProcessorFrwID                                                        O          RO
 hrProcessorLoad                                                         O          RO
 ENTITY-MIB [RFC 6933]
 Object                                                                  CM         Access
 entPhysicalTable                                                        O          N-Acc
 entPhysicalEntry                                                        O          N-Acc
 entPhysicalIndex                                                        O          RO
 entPhysicalDescr                                                        O          RO
 entPhysicalVendorType                                                   O          RO
 entPhysicalContainedIn                                                  O          RO
 entPhysicalClass                                                        O          RO
 entPhysicalParentRelPos                                                 O          RO
 entPhysicalName                                                         O          RO
 entPhysicalHardwareRev                                                  O          RO
 entPhysicalFirmwareRev                                                  O          RO
 entPhysicalSoftwareRev                                                  O          RO
 entPhysicalSerialNum                                                    O          RO
 entPhysicalMfgName                                                      O          RO
 entPhysicalModelName                                                    O          RO
 entPhysicalAlias                                                        O          RO
 entPhysicalAssetID                                                      O          RO
 entPhysicalIsFRU                                                        O          RO
 entPhysicalMfgDate                                                      O          RO
 entPhysicalUris                                                         O          RO
 entPhysicalUUID                                                         O          RO
 ENTITY-SENSOR-MIB [RFC 3433]
 Object                                                                  CM         Access
 entPhySensorTable                                                       O          N-Acc
 entPhySensorEntry                                                       O          N-Acc
 entPhySensorType                                                        O          RO
 entPhySensorScale                                                       O          RO
 entPhySensorPrecision                                                   O          RO
 entPhySensorValue                                                       O          RO
 entPhySensorOperStatus                                                  O          RO
 entPhySensorUnitsDisplay                                                O          RO
 entPhySensorValueTimeStamp                                              O          RO
 entPhySensorValueUpdateRate                                             O          RO




02/19/25                                   CableLabs®                                    101
<!-- PDF_PAGE: 102 -->
CM-SP-CM-OSSIv3.1-I27-250219                 Data-Over-Cable Service Interface Specifications


 SNMP-USM-DH-OBJECTS-MIB [RFC 2786]
 Object                                        CM in        Access     CM in SNMP    Access
                                               NmAccess                Coexistence
                                               Mode                    Mode
 usmDHParameters                               N-Sup                   M             RW
 usmDHUserKeyTable                             N-Sup                   M             N-Acc
 usmDHUserKeyEntry                             N-Sup                   M             N-Acc
 usmDHUserAuthKeyChange                        N-Sup                   M             RC
 usmDHUserOwnAuthKeyChange                     N-Sup                   M             RC
 usmDHUserPrivKeyChange                        N-Sup                   M             RC
 usmDHUserOwnPrivKeyChange                     N-Sup                   M             RC
 usmDHKickstartTable                           N-Sup                   M             N-Acc
 usmDHKickstartEntry                           N-Sup                   M             N-Acc
 usmDHKickstartIndex                           N-Sup                   M             N-Acc
 usmDHKickstartMyPublic                        N-Sup                   M             RO
 usmDHKickstartMgrPublic                       N-Sup                   M             RO
 usmDHKickstartSecurityName                    N-Sup                   M             RO
 SNMP-VIEW-BASED-ACM-MIB [RFC 3415]
 Object                                        CM in        Access     CM in SNMP    Access
                                               NmAccess                Coexistence
                                               Mode                    Mode
 vacmContextTable                              N-Sup                   M             N-Acc
 vacmContextEntry                              N-Sup                   M             N-Acc
 vacmContextName                               N-Sup                   M             RO
 vacmSecurityToGroupTable                      N-Sup                   M             N-Acc
 vacmSecurityToGroupEntry                      N-Sup                   M             N-Acc
 vacmSecurityModel                             N-Sup                   M             N-Acc
 vacmSecurityName                              N-Sup                   M             N-Acc
 vacmGroupName                                 N-Sup                   M             RC
 vacmSecurityToGroupStorageType                N-Sup                   M             RC
 vacmSecurityToGroupStatus                     N-Sup                   M             RC
 vacmAccessTable                               N-Sup                   M             N-Acc
 vacmAccessEntry                               N-Sup                   M             N-Acc
 vacmAccessContextPrefix                       N-Sup                   M             N-Acc
 vacmAccessSecurityModel                       N-Sup                   M             N-Acc
 vacmAccessSecurityLevel                       N-Sup                   M             N-Acc
 vacmAccessContextMatch                        N-Sup                   M             RC
 vacmAccessReadViewName                        N-Sup                   M             RC
 vacmAccessWriteViewName                       N-Sup                   M             RC
 vacmAccessNotifyViewName                      N-Sup                   M             RC
 vacmAccessStorageType                         N-Sup                   M             RC
 vacmAccessStatus                              N-Sup                   M             RC
 vacmViewSpinLock                              N-Sup                   M             RW
 vacmViewTreeFamilyTable                       N-Sup                   M             N-Acc
 vacmViewTreeFamilyEntry                       N-Sup                   M             N-Acc
 vacmViewTreeFamilyViewName                    N-Sup                   M             N-Acc
 vacmViewTreeFamilySubtree                     N-Sup                   M             N-Acc
 vacmViewTreeFamilyMask                        N-Sup                   M             RC




102                                   CableLabs®                                     02/19/25
<!-- PDF_PAGE: 103 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 vacmViewTreeFamilyType                                N-Sup               M             RC
 vacmViewTreeFamilyStorageType                         N-Sup               M             RC
 vacmViewTreeFamilyStatus                              N-Sup               M             RC
 SNMP-COMMUNITY-MIB [RFC 3584]
 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 snmpCommunityTable                                    N-Sup               M             N-Acc
 snmpCommunityEntry                                    N-Sup               M             N-Acc
 snmpCommunityIndex                                    N-Sup               M             N-Acc
 snmpCommunityName                                     N-Sup               M             RC
 snmpCommunitySecurityName                             N-Sup               M             RC
 snmpCommunityContextEngineID                          N-Sup               M             RC
 snmpCommunityContextName                              N-Sup               M             RC
 snmpCommunityTransportTag                             N-Sup               M             RC
 snmpCommunityStorageType                              N-Sup               M             RC
 snmpCommunityStatus                                   N-Sup               M             RC
 snmpTargetAddrExtTable                                N-Sup               M             N-Acc
 snmpTargetAddrExtEntry                                N-Sup               M             N-Acc
 snmpTargetAddrTMask                                   N-Sup               M             RC
 snmpTargetAddrMMS                                     N-Sup               M             RC
 snmpTrapAddress                                       N-Sup               O             ACC-FN
 snmpTrapCommunity                                     N-Sup               O             ACC-FN
 SNMP-FRAMEWORK-MIB [RFC 3411]
 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 snmpEngineGroup
 snmpEngineID                                          N-Sup               M             RO
 snmpEngineBoots                                       N-Sup               M             RO
 snmpEngineTime                                        N-Sup               M             RO
 snmpEngineMaxMessageSize                              N-Sup               M             RO
 SNMP-MPD-MIB [RFC 3412]
 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 snmpMPDStats
 snmpUnknownSecurityModels                             N-Sup               M             RO
 snmpInvalidMsgs                                       N-Sup               M             RO
 snmpUnknownPDUHandlers                                N-Sup               M             RO
 SNMP-TARGET-MIB [RFC 3413]
 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 snmpTargetSpinLock                                    N-Sup               M             RW
 snmpTargetAddrTable                                   N-Sup               M             N-Acc
 snmpTargetAddrEntry                                   N-Sup               M             N-Acc




02/19/25                                   CableLabs®                                         103
<!-- PDF_PAGE: 104 -->
CM-SP-CM-OSSIv3.1-I27-250219                Data-Over-Cable Service Interface Specifications


 Object                                       CM in        Access     CM in SNMP    Access
                                              NmAccess                Coexistence
                                              Mode                    Mode
 snmpTargetAddrName                           N-Sup                   M             N-Acc
 snmpTargetAddrTDomain                        N-Sup                   M             RC
 snmpTargetAddrTAddress                       N-Sup                   M             RC
 snmpTargetAddrTimeout                        N-Sup                   M             RC
 snmpTargetAddrRetryCount                     N-Sup                   M             RC
 snmpTargetAddrTagList                        N-Sup                   M             RC
 snmpTargetAddrParams                         N-Sup                   M             RC
 snmpTargetAddrStorageType                    N-Sup                   M             RC
 snmpTargetAddrRowStatus                      N-Sup                   M             RC
 snmpTargetParamsTable                        N-Sup                   M             N-Acc
 snmpTargetParamsEntry                        N-Sup                   M             N-Acc
 snmpTargetParamsName                         N-Sup                   M             N-Acc
 snmpTargetParamsMPModel                      N-Sup                   M             RC
 snmpTargetParamsSecurityModel                N-Sup                   M             RC
 snmpTargetParamsSecurityName                 N-Sup                   M             RC
 snmpTargetParamsSecurityLevel                N-Sup                   M             RC
 snmpTargetParamsStorageType                  N-Sup                   M             RC
 snmpTargetParamsRowStatus                    N-Sup                   M             RC
 snmpUnavailableContexts                      N-Sup                   M             RO
 snmpUnknownContexts                          N-Sup                   M             RO
 snmpNotifyTable                              N-Sup                   M             N-Acc
 snmpNotifyEntry                              N-Sup                   M             N-Acc
 snmpNotifyName                               N-Sup                   M             N-Acc
 snmpNotifyTag                                N-Sup                   M             RC
 snmpNotifyType                               N-Sup                   M             RC
 snmpNotifyStorageType                        N-Sup                   M             RC
 snmpNotifyRowStatus                          N-Sup                   M             RC
 snmpNotifyFilterProfileTable                 N-Sup                   M             N-Acc
 snmpNotifyFilterProfileEntry                 N-Sup                   M             N-Acc
 snmpNotifyFilterProfileName                  N-Sup                   M             RC
 snmpNotifyFilterProfileStorType              N-Sup                   M             RC
 snmpNotifyFilterProfileRowStatus             N-Sup                   M             RC
 snmpNotifyFilterTable                        N-Sup                   M             N-Acc
 snmpNotifyFilterEntry                        N-Sup                   M             N-Acc
 snmpNotifyFilterSubtree                      N-Sup                   M             N-Acc
 snmpNotifyFilterMask                         N-Sup                   M             RC
 snmpNotifyFilterType                         N-Sup                   M             RC
 snmpNotifyFilterStorageType                  N-Sup                   M             RC
 snmpNotifyFilterRowStatus                    N-Sup                   M             RC
 SNMP-USER-BASED-SM-MIB [RFC 3414]
 Object                                       CM in        Access     CM in SNMP    Access
                                              NmAccess                Coexistence
                                              Mode                    Mode
 usmStats
 usmStatsUnsupportedSecLevels                 N-Sup                   M             RO




104                                  CableLabs®                                     02/19/25
<!-- PDF_PAGE: 105 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                CM in      Access   CM in SNMP    Access
                                                       NmAccess            Coexistence
                                                       Mode                Mode
 usmStatsNotInTimeWindows                              N-Sup               M             RO
 usmStatsUnknownUserNames                              N-Sup               M             RO
 usmStatsUnknownEngineIDs                              N-Sup               M             RO
 usmStatsWrongDigests                                  N-Sup               M             RO
 usmStatsDecryptionErrors                              N-Sup               M             RO
 usmUser
 usmUserSpinLock                                       N-Sup               M             RW
 usmUserTable                                          N-Sup               M             N-Acc
 usmUserEntry                                          N-Sup               M             N-Acc
 usmUserEngineID                                       N-Sup               M             N-Acc
 usmUserName                                           N-Sup               M             N-Acc
 usmUserSecurityName                                   N-Sup               M             RO
 usmUserCloneFrom                                      N-Sup               M             RC
 usmUserAuthProtocol                                   N-Sup               M             RC
 usmUserAuthKeyChange                                  N-Sup               M             RC
 usmUserOwnAuthKeyChange                               N-Sup               M             RC
 usmUserPrivProtocol                                   N-Sup               M             RC
 usmUserPrivKeyChange                                  N-Sup               M             RC
 usmUserOwnPrivKeyChange                               N-Sup               M             RC
 usmUserPublic                                         N-Sup               M             RC
 usmUserStorageType                                    N-Sup               M             RC
 usmUserStatus                                         N-Sup               M             RC
 IGMP-STD-MIB [RFC 2933]
 Object                                                                    CM            Access
 igmpInterfaceTable                                                        D             N-Acc
 igmpInterfaceEntry                                                        D             N-Acc
 igmpInterfaceIfIndex                                                      D             N-Acc
 igmpInterfaceQueryInterval                                                D             RC
 igmpInterfaceStatus                                                       D             RC
 igmpInterfaceVersion                                                      D             RO
 igmpInterfaceQuerier                                                      D             RO
 igmpInterfaceQueryMaxResponseTime                                         D             RC
 igmpInterfaceQuerierUpTime                                                D             RO
 igmpInterfaceQuerierExpiryTime                                            D             RO
 igmpInterfaceVersion1QuerierTimer                                         D             RO
 igmpInterfaceWrongVersionQueries                                          D             RO
 igmpInterfaceJoins                                                        D             RO
 igmpInterfaceProxyIfIndex                                                 D             RO
 igmpInterfaceGroups                                                       D             RO
 igmpInterfaceRobustness                                                   D             RC
 igmpInterfaceLastMembQueryIntvl                                           D             RC
 igmpCacheTable                                                            D             N-Acc
 igmpCacheEntry                                                            D             N-Acc
 igmpCacheAddress                                                          D             N-Acc




02/19/25                                   CableLabs®                                         105
<!-- PDF_PAGE: 106 -->
CM-SP-CM-OSSIv3.1-I27-250219              Data-Over-Cable Service Interface Specifications


 Object                                                             CM           Access
 igmpCacheIfIndex                                                   D            N-Acc
 igmpCacheSelf                                                      D            RC
 igmpCacheLastReporter                                              D            RO
 igmpCacheUpTime                                                    D            RO
 igmpCacheExpiryTime                                                D            RO
 igmpCacheStatus                                                    D            RO
 igmpCacheVersion1HostTimer                                         D            RO
 [DOCS-QOS3-MIB]
 Object                                                             CM           Access
 docsQosPktClassTable                                               M            N-Acc
 docsQosPktClassEntry                                               M            N-Acc
 docsQosPktClassId                                                  M            N-Acc
 docsQosPktClassDirection                                           M            RO
 docsQosPktClassPriority                                            M            RO
 docsQosPktClassIpTosLow                                            M            RO
 docsQosPktClassIpTosHigh                                           M            RO
 docsQosPktClassIpTosMask                                           M            RO
 docsQosPktClassIpProtocol                                          M            RO
 docsQosPktClassIpSourceAddr                                        M            RO
 docsQosPktClassIpSourceMask                                        M            RO
 docsQosPktClassIpDestAddr                                          M            RO
 docsQosPktClassIpDestMask                                          M            RO
 docsQosPktClassSourcePortStart                                     M            RO
 docsQosPktClassSourcePortEnd                                       M            RO
 docsQosPktClassDestPortStart                                       M            RO
 docsQosPktClassDestPortEnd                                         M            RO
 docsQosPktClassDestMacAddr                                         M            RO
 docsQosPktClassDestMacMask                                         M            RO
 docsQosPktClassSourceMacAddr                                       M            RO
 docsQosPktClassEnetProtocolType                                    M            RO
 docsQosPktClassEnetProtocol                                        M            RO
 docsQosPktClassUserPriLow                                          M            RO
 docsQosPktClassUserPriHigh                                         M            RO
 docsQosPktClassVlanId                                              M            RO
 docsQosPktClassState                                               M            RO
 docsQosPktClassPkts                                                M            RO
 docsQosPktClassBitMap                                              M            RO
 docsQosPktClassIpAddrType                                          M            RO
 docsQosPktClassFlowLabel                                           M            RO
 docsQosPktClassIcmpTypeHigh                                        M            RO
 docsQosPktClassIcmpTypeLow                                         M            RO
 docsQosPktClassCmInterfaceMask                                     M            RO
 docsQosParamSetTable                                               M            N-Acc
 docsQosParamSetEntry                                               M            N-Acc
 docsQosParamSetServiceClassName                                    M            RO
 docsQosParamSetPriority                                            M            RO




106                                CableLabs®                                    02/19/25
<!-- PDF_PAGE: 107 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsQosParamSetMaxTrafficRate                                           M          RO
 docsQosParamSetMaxTrafficBurst                                          M          RO
 docsQosParamSetMinReservedRate                                          M          RO
 docsQosParamSetMinReservedPkt                                           M          RO
 docsQosParamSetActiveTimeout                                            M          RO
 docsQosParamSetAdmittedTimeout                                          M          RO
 docsQosParamSetMaxConcatBurst                                           M          RO
 docsQosParamSetSchedulingType                                           M          RO
 docsQosParamSetNomPollInterval                                          M          RO
 docsQosParamSetTolPollJitter                                            M          RO
 docsQosParamSetUnsolicitGrantSize                                       M          RO
 docsQosParamSetNomGrantInterval                                         M          RO
 docsQosParamSetTolGrantJitter                                           M          RO
 docsQosParamSetGrantsPerInterval                                        M          RO
 docsQosParamSetTosAndMask                                               M          RO
 docsQosParamSetTosOrMask                                                M          RO
 docsQosParamSetMaxLatency                                               M          RO
 docsQosParamSetType                                                     M          N-Acc
 docsQosParamSetRequestPolicyOct                                         M          RO
 docsQosParamSetBitMap                                                   M          RO
 docsQosParamSetServiceFlowId                                            M          N-Acc
 docsQosParamSetRequiredAttrMask                                         M          RO
 docsQosParamSetForbiddenAttrMask                                        M          RO
 docsQosParamSetAttrAggrRuleMask                                         M          RO
 docsQosParamSetAppId                                                    M          RO
 docsQosParamSetMultiplierContentionReqWindow                            M          RO
 docsQosParamSetMultiplierBytesReq                                       M          RO
 docsQosParamSetMaxReqPerSidCluster                                      D          RO
 docsQosParamSetMaxOutstandingBytesPerSidCluster                         D          RO
 docsQosParamSetMaxTotBytesReqPerSidCluster                              D          RO
 docsQosParamSetMaxTimeInSidCluster                                      D          RO
 docsQosParamSetPeakTrafficRate                                          M          RO
 docsQosParamSetDsResequencing                                           M          RO
 docsQosParamSetMinimumBuffer                                            M          RO
 docsQosParamSetTargetBuffer                                             M          RO
 docsQosParamSetMaximumBuffer                                            M          RO
 docsQosParamSetAqmDisabled                                              M          RO
 docsQosParamSetAqmLatencyTarget                                         M          RO
 docsQosParamSetHCMaxTrafficRate                                         M          RO
 docsQosParamSetHCMinReservedRate                                        M          RO
 docsQosParamSetHCPeakTrafficRate                                        M          RO
 docsQosParamSetAqmAlgInUse                                              M          RO
 docsQosParamSetGuaranteedGrantInterval                                  M          RO
 docsQosParamSetGuaranteedGrantRate                                      M          RO
 docsQosParamSetGuaranteedRequestInterval                                M          RO
 docsQosParamSetImmedAqmMaxThreshold                                     M          RO




02/19/25                                           CableLabs®                            107
<!-- PDF_PAGE: 108 -->
CM-SP-CM-OSSIv3.1-I27-250219                            Data-Over-Cable Service Interface Specifications


 Object                                                                           CM           Access
 docsQosParamSetImmedAqmRangeExponentRampFunc                                     M            RO
 docsQosParamSetDataRateUnitSetting                                               M            RO
 docsQosParamSetPgsActivityDetectionDisable                                       M            RO
 docsQosServiceFlowTable                                                          M            N-Acc
 docsQosServiceFlowEntry                                                          M            N-Acc
 docsQosServiceFlowId                                                             M            N-Acc
 docsQosServiceFlowSID                                                            M            RO
 docsQosServiceFlowDirection                                                      M            RO
 docsQosServiceFlowPrimary                                                        M            RO
 docsQosServiceFlowParamSetTypeStatus                                             M            RO
 docsQosServiceFlowChSetId                                                        M            RO
 docsQosServiceFlowAttrAssignSuccess                                              M            RO
 docsQosServiceFlowDsid                                                           M            RO
 docsQosServiceFlowMaxReqPerSidCluster                                            M            RO
 docsQosServiceFlowMaxOutstandingBytesPerSidCluster                               M            RO
 docsQosServiceFlowMaxTotBytesReqPerSidCluster                                    M            RO
 docsQosServiceFlowMaxTimeInSidCluster                                            M            RO
 docsQosServiceFlowBufferSize                                                     M            RO
 docsQosServiceFlowStatsTable                                                     M            N-Acc
 docsQosServiceFlowStatsEntry                                                     M            N-Acc
 docsQosServiceFlowAggregateServiceFlowId                                         M            RO
 docsQosServiceFlowType                                                           M            RO
 docsQosServiceFlowAllowedAqBytes                                                 M            RO
 docsQosServiceFlowPkts                                                           M            RO
 docsQosServiceFlowOctets                                                         M            RO
 docsQosServiceFlowTimeCreated                                                    M            RO
 docsQosServiceFlowTimeActive                                                     M            RO
 docsQosServiceFlowPolicedDropPkts                                                M            RO
 docsQosServiceFlowPolicedDelayPkts                                               M            RO
 docsQosServiceFlowAqmDroppedPkts                                                 M            RO
 docsQosDynamicServiceStatsTable                                                  M            N-Acc
 docsQosDynamicServiceStatsEntry                                                  M            N-Acc
 docsQosIfDirection                                                               M            N-Acc
 docsQosDSAReqs                                                                   M            RO
 docsQosDSARsps                                                                   M            RO
 docsQosDSAAcks                                                                   M            RO
 docsQosDSCReqs                                                                   M            RO
 docsQosDSCRsps                                                                   M            RO
 docsQosDSCAcks                                                                   M            RO
 docsQosDSDReqs                                                                   M            RO
 docsQosDSDRsps                                                                   M            RO
 docsQosDynamicAdds                                                               M            RO
 docsQosDynamicAddFails                                                           M            RO
 docsQosDynamicChanges                                                            M            RO
 docsQosDynamicChangeFails                                                        M            RO
 docsQosDynamicDeletes                                                            M            RO




108                                              CableLabs®                                    02/19/25
<!-- PDF_PAGE: 109 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsQosDynamicDeleteFails                                               M          RO
 docsQosDCCReqs                                                          M          RO
 docsQosDCCRsps                                                          M          RO
 docsQosDCCAcks                                                          M          RO
 docsQosDCCs                                                             M          RO
 docsQosDCCFails                                                         M          RO
 docsQosDCCRspDeparts                                                    M          RO
 docsQosDCCRspArrives                                                    M          RO
 docsQosDbcReqs                                                          M          RO
 docsQosDbcRsps                                                          M          RO
 docsQosDbcAcks                                                          M          RO
 docsQosDbcSuccesses                                                     M          RO
 docsQosDbcFails                                                         M          RO
 docsQosDbcPartial                                                       M          RO
 docsQosServiceFlowSidClusterTable                                       M          N-Acc
 docsQosServiceFlowSidClusterEntry                                       M          N-Acc
 docsQosServiceFlowSidClusterId                                          M          N-Acc
 docsQosServiceFlowSidClusterUcid                                        M          N-Acc
 docsQosServiceFlowSidClusterSid                                         M          RO
 docsQosAggregateServiceFlowTable                                        M          N-Acc
 docsQosAggregateServiceFlowEntry                                        M          N-Acc
 docsQosAggregateServiceFlowId                                           M          N-Acc
 docsQosAggregateServiceFlowDirection                                    M          RO
 docsQosAggregateServiceFlowPriority                                     M          RO
 docsQosAggregateServiceFlowMaxAggregateTrafficRate                      M          RO
 docsQosAggregateServiceFlowMaxTrafficBurst                              M          RO
 docsQosAggregateServiceFlowMinReservedRate                              M          RO
 docsQosAggregateServiceFlowMinReservedPkt                               M          RO
 docsQosAggregateServiceFlowPeakTrafficRate                              M          RO
 docsQosAggregateServiceFlowDataRateUnitSetting                          M          RO
 docsQosAggregateServiceFlowLowLatencyAsf                                M          RO
 docsQosAggregateServiceFlowLowLatencySfId                               M          RO
 docsQosAggregateServiceFlowClassicSfScn                                 M          RO
 docsQosAggregateServiceFlowLatencySfScn                                 M          RO
 docsQosAggregateServiceFlowAqmCouplingFactor                            M          RO
 docsQosAggregateServiceFlowSchedulingWeight                             M          RO
 docsQosAggregateServiceFlowQpEnable                                     M          RO
 docsQosAggregateServiceFlowQpLatencyThreshold                           M          RO
 docsQosAggregateServiceFlowQpQueuingScoreThreshold                      M          RO
 docsQosAggregateServiceFlowQpDrainRateExponent                          M          RO
 docsQosAggregateServiceFlowHCMaxAggregateTrafficRate                    M          RO
 docsQosAggregateServiceFlowHCMinReservedRate                            M          RO
 docsQosAggregateServiceFlowHCPeakTrafficRate                            M          RO
 docsQosAggregateServiceFlowAsfQosProfileName                            M          RO
 docsQosAggregateServiceFlowStatsTable                                   M          N-Acc
 docsQosAggregateServiceFlowStatsEntry                                   M          N-Acc




02/19/25                                          CableLabs®                             109
<!-- PDF_PAGE: 110 -->
CM-SP-CM-OSSIv3.1-I27-250219                          Data-Over-Cable Service Interface Specifications


 Object                                                                         CM           Access
 docsQosAggregateServiceFlowStatsPkts                                           M            RO
 docsQosAggregateServiceFlowStatsOctets                                         M            RO
 docsQosAggregateServiceFlowStatsTimeCreated                                    M            RO
 docsQosAggregateServiceFlowStatsTimeActive                                     M            RO
 docsQosSfLatencyHistCfgTable                                                   M            N-Acc
 docsQosSfLatencyHistCfgEntry                                                   M            N-Acc
 docsQosSfLatencyHistCfgStatus                                                  M            RC
 docsQosSfLatencySfLabel                                                        M            RC
 docsQosSfLatencyBin1UpperEdge                                                  M            RC
 docsQosSfLatencyBin2UpperEdge                                                  M            RC
 docsQosSfLatencyBin3UpperEdge                                                  M            RC
 docsQosSfLatencyBin4UpperEdge                                                  M            RC
 docsQosSfLatencyBin5UpperEdge                                                  M            RC
 docsQosSfLatencyBin6UpperEdge                                                  M            RC
 docsQosSfLatencyBin7UpperEdge                                                  M            RC
 docsQosSfLatencyBin8UpperEdge                                                  M            RC
 docsQosSfLatencyBin9UpperEdge                                                  M            RC
 docsQosSfLatencyBin10UpperEdge                                                 M            RC
 docsQosSfLatencyBin11UpperEdge                                                 M            RC
 docsQosSfLatencyBin12UpperEdge                                                 M            RC
 docsQosSfLatencyBin13UpperEdge                                                 M            RC
 docsQosSfLatencyBin14UpperEdge                                                 M            RC
 docsQosSfLatencyBin15UpperEdge                                                 M            RC
 docsQosSfLatencyStatsTable                                                     M            N-Acc
 docsQosSfLatencyStatsEntry                                                     M            N-Acc
 docsQosSfLatencyMaxLatency                                                     M            RO
 docsQosSfLatencyNumHistUpdates                                                 M            RO
 docsQosSfLatencyBin1Pkts                                                       M            RO
 docsQosSfLatencyBin2Pkts                                                       M            RO
 docsQosSfLatencyBin3Pkts                                                       M            RO
 docsQosSfLatencyBin4Pkts                                                       M            RO
 docsQosSfLatencyBin5Pkts                                                       M            RO
 docsQosSfLatencyBin6Pkts                                                       M            RO
 docsQosSfLatencyBin7Pkts                                                       M            RO
 docsQosSfLatencyBin8Pkts                                                       M            RO
 docsQosSfLatencyBin9Pkts                                                       M            RO
 docsQosSfLatencyBin10Pkts                                                      M            RO
 docsQosSfLatencyBin11Pkts                                                      M            RO
 docsQosSfLatencyBin12Pkts                                                      M            RO
 docsQosSfLatencyBin13Pkts                                                      M            RO
 docsQosSfLatencyBin14Pkts                                                      M            RO
 docsQosSfLatencyBin15Pkts                                                      M            RO
 docsQosSfLatencyBin16Pkts                                                      M            RO
 docsQosSfCongestionStatsTable                                                  M            N-Acc
 docsQosSfCongestionStatsEntry                                                  M            N-Acc
 docsQosSfCongestionSanctionedPkts                                              M            RO




110                                            CableLabs®                                    02/19/25
<!-- PDF_PAGE: 111 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsQosSfCongestionTotalEct0Pkts                                        M          RO
 docsQosSfCongestionTotalEct1Pkts                                        M          RO
 docsQosSfCongestionCeMarkedEct1Pkts                                     M          RO
 docsQosSfCongestionArrivedCePkts                                        M          RO
 docsQosCmServiceUsStatsTable                                            M          N-Acc
 docsQosCmServiceUsStatsEntry                                            M          N-Acc
 docsQosCmServiceUsStatsTxSlotsImmed                                     M          RO
 docsQosCmServiceUsStatsTxSlotsDed                                       M          RO
 docsQosCmServiceUsStatsTxRetries                                        M          RO
 docsQosCmServiceUsStatsTxExceededs                                      M          RO
 docsQosCmServiceUsStatsRqRetries                                        M          RO
 docsQosCmServiceUsStatsRqExceededs                                      M          RO
 docsQosCmServiceUsStatsSgmts                                            M          RO
 docsQosCmDsidTable                                                      M          N-Acc
 docsQosCmDsidEntry                                                      M          N-Acc
 docsQosCmDsidDsid                                                       M          N-Acc
 docsQosCmDsidUsage                                                      M          RO
 docsQosCmDsidNumReseqChs                                                M          RO
 docsQosCmDsidReseqChList                                                M          RO
 docsQosCmDsidReseqWaitTime                                              M          RO
 docsQosCmDsidReseqWarnThrshld                                           M          RO
 docsQosCmDsidStatusHoldOffTimerSeqOutOfRng                              M          RO
 docsQosCmDsidOutOfRangeDiscards                                         M          RO
 docsQosCmDsidNextExpectedSeqNum                                         M          RO
 docsQosCmDsidCmInterfaceMask                                            M          RO
 docsQosCmDsidFwdCmInterfaceMask                                         M          RO
 docsQosCmDsidStatsTable                                                 M          N-Acc
 docsQosCmDsidStatsEntry                                                 M          N-Acc
 docsQosCmDsidStatsDsid                                                  M          N-Acc
 docsQosCmDsidStatsSeqNumMissing                                         M          RO
 docsQosCmDsidStatsSkewThreshExceeds                                     M          RO
 docsQosCmDsidStatsOutOfRangePackets                                     M          RO
 docsQosCmDsidStatsNumPackets                                            M          RO
 docsQosCmDsidClientTable                                                M          N-Acc
 docsQosCmDsidClientEntry                                                M          N-Acc
 docsQosCmDsidClientDsid                                                 M          N-Acc
 docsQosCmDsidClientClientMacId                                          M          N-Acc
 docsQosCmDsidClientClientMacAddr                                        M          RO
 docsQosCmSystemCfgState
 docsQosCmSystemCfgStateAqmUsEnable                                      M          RO
 docsQosCmSystemCfgStateDefaultUsTargetBuffer                            M          RO
 [DOCS-IF3-MIB]
 docsIf3CmStatusTable                                                    M          N-Acc
 docsIf3CmStatusEntry                                                    M          N-Acc
 docsIf3CmStatusValue                                                    M          RO
 docsIf3CmStatusCode                                                     M          RO




02/19/25                                        CableLabs®                               111
<!-- PDF_PAGE: 112 -->
CM-SP-CM-OSSIv3.1-I27-250219                                 Data-Over-Cable Service Interface Specifications


 Object                                                                                CM           Access
 docsIf3CmStatusResets                                                                 M            RO
 docsIf3CmStatusLostSyncs                                                              M            RO
 docsIf3CmStatusInvalidMaps                                                            M            RO
 docsIf3CmStatusInvalidUcds                                                            M            RO
 docsIf3CmStatusInvalidRangingRsps                                                     M            RO
 docsIf3CmStatusInvalidRegRsps                                                         M            RO
 docsIf3CmStatusT1Timeouts                                                             M            RO
 docsIf3CmStatusT2Timeouts                                                             M            RO
 docsIf3CmStatusUCCsSuccesses                                                          D            RO
 docsIf3CmStatusUCCFails                                                               D            RO
 docsIf3CmStatusEnergyMgt1x1OperStatus                                                 M            RO
 docsIf3CmStatusUsTable                                                                M            N-Acc
 docsIf3CmStatusUsEntry                                                                M            N-Acc
 docsIf3CmStatusUsTxPower                                                              M            RO
 docsIf3CmStatusUsT3Timeouts                                                           M            RO
 docsIf3CmStatusUsT4Timeouts                                                           M            RO
 docsIf3CmStatusUsRangingAborteds                                                      M            RO
 docsIf3CmStatusUsModulationType                                                       M            RO
 docsIf3CmStatusUsEqData                                                               M            RO
 docsIf3CmStatusUsT3Exceededs                                                          M            RO
 docsIf3CmStatusUsIsMuted                                                              M            RO
 docsIf3CmStatusUsRangingStatus                                                        M            RO
 docsIf3CmCapabilities
 docsIf3CmCapabilitiesReq                                                              M            RO
 docsIf3CmCapabilitiesRsp                                                              M            RO
 docsIf3RxChStatusTable                                                                M            N-Acc
 docsIf3RxChStatusEntry                                                                M            N-Acc
 docsIf3RxChStatusRcId                                                                 M            N-Acc
 docsIf3RxChStatusChIfIndex                                                            M            RO
 docsIf3RxChStatusPrimaryDsIndicator                                                   M            RO
 docsIf3RxChStatusRcRmConnectivityId                                                   M            RO
 docsIf3RxModuleStatusTable                                                            M            N-Acc
 docsIf3RxModuleStatusEntry                                                            M            N-Acc
 docsIf3RxModuleStatusRmId                                                             M            N-Acc
 docsIf3RxModuleStatusRmRmConnectivityId                                               M            RO
 docsIf3RxModuleStatusFirstCenterFrequency                                             M            RO
 docsIf3SignalQualityExtTable                                                          M            N-Acc
 docsIf3SignalQualityExtEntry                                                          M            N-Acc
 docsIf3SignalQualityExtRxMER                                                          M            RO
 docsIf3SignalQualityExtRxMerSamples                                                   M            RO
 docsIf3SignalQualityExtFbeNormalizationCoefficient                                    M            RO
 docsIf3UsChExtTable                                                                   O            N-Acc
 docsIf3UsChExtEntry                                                                   O            N-Acc
 docsIf3UsChExtSacCodeHoppingSelectionMode                                             O            RO
 docsIf3UsChExtScdmaSelectionStringActiveCodes                                         O            RO
 docsIf3CmDpvStatsTable                                                                M            N-Acc




112                                                   CableLabs®                                    02/19/25
<!-- PDF_PAGE: 113 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                    CM         Access
 docsIf3CmDpvStatsEntry                                                    M          N-Acc
 docsIf3CmDpvStatsGrpId                                                    M          N-Acc
 docsIf3CmDpvStatsLastMeasLatency                                          M          RO
 docsIf3CmDpvStatsLastMeasTime                                             M          RO
 docsIf3CmDpvStatsMinLatency                                               M          RO
 docsIf3CmDpvStatsMaxLatency                                               M          RO
 docsIf3CmDpvStatsAvgLatency                                               M          RO
 docsIf3CmDpvStatsNumMeas                                                  M          RO
 docsIf3CmDpvStatsLastClearTime                                            M          RO
 docsIf3CmEventCtrlTable                                                   M          N-Acc
 docsIf3CmEventCtrlEntry                                                   M          N-Acc
 docsIf3CmEventCtrlEventId                                                 M          N-Acc
 docsIf3CmEventCtrlStatus                                                  M          RC
 docsIf3CmMdCfgTable                                                       M          N-Acc
 docsIf3CmMdCfgEntry                                                       M          N-Acc
 docsIf3CmMdCfgIpProvMode                                                  M          RW
 docsIf3CmMdCfgIpProvModeResetOnChange                                     M          RW
 docsIf3CmMdCfgIpProvModeResetOnChangeHoldOffTimer                         M          RW
 docsIf3CmMdCfgIpProvModeStorageType                                       M          RW
 docsIf3CmEnergyMgtCfg
 docsIf3CmEnergyMgtCfgFeatureEnabled                                       M          RO
 docsIf3CmEnergyMgtCfgCyclePeriod                                          M          RO
 docsIf3CmEnergyMgt1x1CfgTable                                             M          N-Acc
 docsIf3CmEnergyMgt1x1CfgEntry                                             M          N-Acc
 docsIf3CmEnergyMgt1x1CfgDirection                                         M          N-Acc
 docsIf3CmEnergyMgt1x1CfgEntryBitrateThrshld                               M          RW
 docsIf3CmEnergyMgt1x1CfgEntryTimeThrshld                                  M          RW
 docsIf3CmEnergyMgt1x1CfgExitBitrateThrshld                                M          RW
 docsIf3CmEnergyMgt1x1CfgExitTimeThrshld                                   M          RW
 docsIf3CmSpectrumAnalysisCtrlCmd
 docsIf3CmSpectrumAnalysisCtrlCmdEnable                                    M          RW
 docsIf3CmSpectrumAnalysisCtrlCmdInactivityTimeout                         M          RW
 docsIf3CmSpectrumAnalysisCtrlCmdFirstSegmentCenterFrequency               M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdLastSegmentCenterFrequency                M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdSegmentFrequencySpan                      M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdNumBinsPerSegment                         M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdEquivalentNoiseBandwidth                  M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdWindowFunction                            M          RO/RW
 docsIf3CmSpectrumAnalysisCtrlCmdNumberOfAverages                          M          RO/RW
 docsIf3CmSpectrumAnalysisMeasTable                                        M          N-Acc
 docsIf3CmSpectrumAnalysisMeasEntry                                        M          N-Acc
 docsIf3CmSpectrumAnalysisMeasFrequency                                    M          N-Acc
 docsIf3CmSpectrumAnalysisMeasAmplitudeData                                M          RO
 docsIf3CmSpectrumAnalysisMeasTotalSegmentPower                            M          RO
 docsIf3CmEm1x1StatsTable                                                  M          N-Acc
 docsIf3CmEm1x1StatsEntry                                                  M          N-Acc




02/19/25                                             CableLabs®                            113
<!-- PDF_PAGE: 114 -->
CM-SP-CM-OSSIv3.1-I27-250219                                 Data-Over-Cable Service Interface Specifications


 Object                                                                                CM           Access
 docsIf3CmEm1x1StatsNumberTimesCrossedBelowUsEntryThrshlds                             M            RO
 docsIf3CmEm1x1StatsNumberTimesCrossedBelowDsEntryThrshlds                             M            RO
 docsIf3CmEm1x1StatsTotalDuration                                                      M            RO
 docsIf3CmEm1x1StatsTotalDurationBelowUsThrshlds                                       M            RO
 docsIf3CmEm1x1StatsTotalDurationBelowDsThrshlds                                       M            RO
 docsIf3CmEm1x1StatsTotalDurationBelowUsDsThrshlds                                     M            RO
 Notifications
 docsIf3CmEventNotif                                                                   M            Notif
 [DOCS-IF31-MIB]
 docsIf31DocsisBaseCapability                                                          M            RO
 docsIf31RxChStatusTable                                                               M            N-Acc
 docsIf31RxChStatusEntry                                                               M            N-Acc
 docsIf31RxChStatusPrimaryDsIndicator                                                  M            RO
 docsIf31RxChStatusOfdmProfiles                                                        M            RO
 docsIf31CmDsOfdmChanTable                                                             M            N-Acc
 docsIf31CmDsOfdmChanEntry                                                             M            N-Acc
 docsIf31CmDsOfdmChanChannelId                                                         M            RO
 docsIf31CmDsOfdmChanChanIndicator                                                     M            RO
 docsIf31CmDsOfdmChanSubcarrierZeroFreq                                                M            RO
 docsIf31CmDsOfdmChanFirstActiveSubcarrierNum                                          M            RO
 docsIf31CmDsOfdmChanLastActiveSubcarrierNum                                           M            RO
 docsIf31CmDsOfdmChanNumActiveSubcarriers                                              M            RO
 docsIf31CmDsOfdmChanSubcarrierSpacing                                                 M            RO
 docsIf31CmDsOfdmChanCyclicPrefix                                                      M            RO
 docsIf31CmDsOfdmChanRollOffPeriod                                                     M            RO
 docsIf31CmDsOfdmChanPlcFreq                                                           M            RO
 docsIf31CmDsOfdmChanNumPilots                                                         M            RO
 docsIf31CmDsOfdmChanTimeInterleaverDepth                                              M            RO
 docsIf31CmDsOfdmChanPlcTotalCodewords                                                 M            RO
 docsIf31CmDsOfdmChanPlcUnreliableCodewords                                            M            RO
 docsIf31CmDsOfdmChanNcpTotalFields                                                    M            RO
 docsIf31CmDsOfdmChanNcpFieldCrcFailures                                               M            RO
 docsIf31CmDsOfdmProfileStatsTable                                                     M            N-Acc
 docsIf31CmDsOfdmProfileStatsEntry                                                     M            N-Acc
 docsIf31CmDsOfdmProfileStatsProfileId                                                 M            N-Acc
 docsIf31CmDsOfdmProfileStatsConfigChangeCt                                            M            RO
 docsIf31CmDsOfdmProfileStatsTotalCodewords                                            M            RO
 docsIf31CmDsOfdmProfileStatsCorrectedCodewords                                        M            RO
 docsIf31CmDsOfdmProfileStatsUncorrectableCodewords                                    M            RO
 docsIf31CmDsOfdmProfileStatsInOctets                                                  M            RO
 docsIf31CmDsOfdmProfileStatsInUnicastOctets                                           M            RO
 docsIf31CmDsOfdmProfileStatsInMulticastOctets                                         M            RO
 docsIf31CmDsOfdmProfileStatsInFrames                                                  M            RO
 docsIf31CmDsOfdmProfileStatsInUnicastFrames                                           M            RO
 docsIf31CmDsOfdmProfileStatsInMulticastFrames                                         M            RO
 docsIf31CmDsOfdmProfileStatsInFrameCrcFailures                                        M            RO




114                                                CableLabs®                                       02/19/25
<!-- PDF_PAGE: 115 -->
Cable Modem Operations Support System Interface Specification      CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                     CM         Access
 docsIf31CmDsOfdmProfileStatsCtrDiscontinuityTime                           M          RO
 docsIf31CmDsOfdmChannelPowerTable                                          M          N-Acc
 docsIf31CmDsOfdmChannelPowerEntry                                          M          N-Acc
 docsIf31CmDsOfdmChannelBandIndex                                           M          N-Acc
 docsIf31CmDsOfdmChannelPowerCenterFrequency                                M          RO
 docsIf31CmDsOfdmChannelPowerRxPower                                        M          RO
 docsIf31CmStatusOfdmaUsTable                                               M          N-Acc
 docsIf31CmStatusOfdmaUsEntry                                               M          N-Acc
 docsIf31CmStatusOfdmaUsT3Timeouts                                          M          RO
 docsIf31CmStatusOfdmaUsT4Timeouts                                          M          RO
 docsIf31CmStatusOfdmaUsRangingAborteds                                     M          RO
 docsIf31CmStatusOfdmaUsT3Exceededs                                         M          RO
 docsIf31CmStatusOfdmaUsIsMuted                                             M          RO
 docsIf31CmStatusOfdmaUsRangingStatus                                       M          RO
 docsIf31CmUsOfdmaChanTable                                                 M          N-Acc
 docsIf31CmUsOfdmaChanEntry                                                 M          N-Acc
 docsIf31CmStatusOfdmaChanConfigChangeCt                                    M          RO
 docsIf31CmUsOfdmaChanSubcarrierZeroFreq                                    M          RO
 docsIf31CmUsOfdmaChanFirstActiveSubcarrierNum                              M          RO
 docsIf31CmUsOfdmaChanLastActiveSubcarrierNum                               M          RO
 docsIf31CmUsOfdmaChanNumActiveSubcarriers                                  M          RO
 docsIf31CmUsOfdmaChanSubcarrierSpacing                                     M          RO
 docsIf31CmUsOfdmaChanCyclicPrefix                                          M          RO
 docsIf31CmUsOfdmaChanRollOffPeriod                                         M          RO
 docsIf31CmUsOfdmaChanNumSymbolsPerFrame                                    M          RO
 docsIf31CmUsOfdmaChanTxPower                                               M          RO
 docsIf31CmUsOfdmaChanPreEqEnabled                                          M          RO
 docsIf31CmUsOfdmaChanChannelId                                             M          RO
 docsIf31CmUsOfdmaChanEstimatedRgDelay                                      M          RO
 docsIf31CmUsScQamChanTable                                                 M          N-Acc
 docsIf31CmUsScQamChanEntry                                                 M          N-Acc
 docsIf31CmUsScQamChanTxPsd                                                 M          RO
 docsIf31CmUsScQamChanEstimatedRgDelay                                      M          RO
 docsIf31CmUsOfdmaProfileStatsTable                                         M          N-Acc
 docsIf31CmUsOfdmaProfileStatsEntry                                         M          N-Acc
 docsIf31CmUsOfdmaProfileStatsIuc                                           M          N-Acc
 docsIf31CmUsOfdmaProfileStatsOutOctets                                     M          RO
 docsIf31CmUsOfdmaProfileStatsCtrDiscontinuityTime                          M          RO
 docsIf31CmUsOfdmaMinislotCfgStateTable                                     M          N-Acc
 docsIf31CmUsOfdmaMinislotCfgStateEntry                                     M          N-Acc
 docsIf31CmUsOfdmaMinislotCfgStateStartMinislotNum                          M          N-Acc
 docsIf31CmUsOfdmaMinislotCfgStateFirstSubcarrierId                         M          RO
 docsIf31CmUsOfdmaMinislotCfgStateNumConsecutiveMinislots                   M          RO
 docsIf31CmUsOfdmaMinislotCfgStateMinislotPilotPattern                      M          RO
 docsIf31CmUsOfdmaMinislotCfgStateDataSymbolModulation                      M          RO
 docsIf31CmEmDlsStatsTable                                                  M          N-Acc




02/19/25                                              CableLabs®                            115
<!-- PDF_PAGE: 116 -->
CM-SP-CM-OSSIv3.1-I27-250219                                Data-Over-Cable Service Interface Specifications


 Object                                                                               CM           Access
 docsIf31CmEmDlsStatsEntry                                                            M            N-Acc
 docsIf31CmEmDlsStatsNumberTimesCrossedBelowUsEntryThrshlds                           M            RO
 docsIf31CmEmDlsStatsNumberTimesCrossedBelowDsEntryThrshlds                           M            RO
 docsIf31CmEmDlsStatsTotalDuration                                                    M            RO
 docsIf31CmEmDlsStatsTotalDurationBelowUsThrshlds                                     M            RO
 docsIf31CmEmDlsStatsTotalDurationBelowDsThrshlds                                     M            RO
 docsIf31CmEmDlsStatsTotalDurationBelowUsDsThrshlds                                   M            RO
 docsIf31CmEmDlsStatsNumSleepLatencyTriggers                                          M            RO
 docsIf31CmEmDlsStatsNumSleepByteCtTriggers                                           M            RO
 docsIf31CmEmDlsStatusTable                                                           M            N-Acc
 docsIf31CmEmDlsStatusEntry                                                           M            N-Acc
 docsIf31CmEmDlsStatusAssignedEmIds                                                   M            N-Acc
 docsIf31CmEmDlsStatusReceiveTimer                                                    M            RO
 docsIf31CmEmDlsStatusMaxSleepLatency                                                 M            RO
 docsIf31CmEmDlsStatusMaxSleepBytes                                                   M            RO
 docsIf31CmSystemCfgState                                                             M            N-Acc
 docsIf31CmSystemCfgStateDiplexerCapability                                           M            RO
 docsIf31CmSystemCfgStateDiplexerCfgBandEdge                                          M            RO
 docsIf31CmSystemCfgStateDiplexerDsLowerCapability                                    M            RO
 docsIf31CmSystemCfgStateDiplexerCfgDsLowerBandEdge                                   M            RO
 docsIf31CmSystemCfgStateDiplexerDsUpperCapability                                    M            RO
 docsIf31CmSystemCfgStateDiplexerCfgDsUpperBandEdge                                   M            RO
 docsIf31CmSystemCfgStateLgVqEwmaAlpha                                                M            RO
 docsIf31CmSystemCfgStateVqInterval                                                   M            RO
 docsIf31CmStatusTable                                                                M            N-Acc
 docsIf31CmStatusEntry                                                                M            N-Acc
 docsIf31CmStatusEmDlsOperStatus                                                      M            RO
 docsIf31CmEmDlsCfgTable                                                              M            N-Acc
 docsIf31CmEmDlsCfgEntry                                                              M            N-Acc
 docsIf31CmEmDlsCfgDirection                                                          M            N-Acc
 docsIf31CmEmDlsCfgEntryBitrateThrshld                                                M            RW
 docsIf31CmEmDlsCfgEntryTimeThrshld                                                   M            RW
 docsIf31CmEmDlsCfgExitBitrateThrshld                                                 M            RW
 docsIf31CmEmDlsCfgExitTimeThrshld                                                    M            RW
 [DOCS-PNM-MIB]
 docsPnmBulkCtl Group
 docsPnmBulkDestIpAddrType                                                            M            RW
 docsPnmBulkDestIpAddr                                                                M            RW
 docsPnmBulkDestPath                                                                  M            RW
 docsPnmBulkUploadControl                                                             M            RW
 docsPnmBulkFileTable                                                                 M            N-Acc
 docsPnmBulkFileEntry                                                                 M            N-Acc
 docsPnmBulkFileIndex                                                                 M            N-Acc
 docsPnmBulkFileName                                                                  M            RO
 docsPnmBulkFileControl                                                               M            RW
 docsPnmBulkFileUploadStatus                                                          M            RO




116                                                  CableLabs®                                    02/19/25
<!-- PDF_PAGE: 117 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv3.1-I27-250219


 Object                                                                  CM         Access
 docsPnmCmControlObjects Group
 docsPnmCmCtlTest                                                        M          RO
 docsPnmCmCtlTestDuration                                                M          RO
 docsPnmCmCtlStatus                                                      M          RO
 docsPnmCmDsOfdmSymCapTable                                              M          N-Acc
 docsPnmCmDsOfdmSymCapEntry                                              M          N-Acc
 docsPnmCmDsOfdmSymTrigEnable                                            M          RW
 docsPnmCmDsOfdmSymTrigEnableTimeout                                     M          RW
 docsPnmCmDsOfdmSymTrigGroupId                                           M          RW
 docsPnmCmDsOfdmSymRxWindowing                                           M          RO
 docsPnmCmDsOfdmSymTransactionId                                         M          RO
 docsPnmCmDsOfdmSymSampleRate                                            M          RO
 docsPnmCmDsOfdmSymFftLength                                             M          RO
 docsPnmCmDsOfdmSymMeasStatus                                            M          RO
 docsPnmCmDsOfdmSymCaptFileName                                          M          RW
 docsPnmCmOfdmChanEstCoefTable                                           M          N-Acc
 docsPnmCmOfdmChanEstCoefEntry                                           M          N-Acc
 docsPnmCmOfdmChEstCoefTrigEnable                                        M          RW
 docsPnmCmOfdmChEstCoefAmpRipplePkToPk                                   M          RO
 docsPnmCmOfdmChEstCoefAmpRippleRms                                      M          RO
 docsPnmCmOfdmChEstCoefAmpSlope                                          M          RO
 docsPnmCmOfdmChEstCoefGrpDelayRipplePkToPk                              M          RO
 docsPnmCmOfdmChEstCoefGrpDelayRippleRms                                 M          RO
 docsPnmCmOfdmChEstCoefMeasStatus                                        M          RO
 docsPnmCmOfdmChEstCoefFileName                                          M          RW
 docsPnmCmOfdmChEstCoefAmpMean                                           M          RO
 docsPnmCmOfdmChEstCoefGrpDelaySlope                                     M          RO
 docsPnmCmOfdmChEstCoefGrpDelayMean                                      M          RO
 docsPnmCmDsConstDispMeasTable                                           M          N-Acc
 docsPnmCmDsConstDispMeasEntry                                           M          N-Acc
 docsPnmCmDsConstDispTrigEnable                                          M          RW
 docsPnmCmDsConstDispModOrderOffset                                      M          RW
 docsPnmCmDsConstDispNumSampleSymb                                       M          RW
 docsPnmCmDsConstDispSelModOrder                                         M          RO
 docsPnmCmDsConstDispMeasStatus                                          M          RO
 docsPnmCmDsConstDispFileName                                            M          RW
 docsPnmCmDsOfdmRxMerTable                                               M          N-Acc
 docsPnmCmDsOfdmRxMerEntry                                               M          N-Acc
 docsPnmCmDsOfdmRxMerFileEnable                                          M          RW
 docsPnmCmDsOfdmRxMerPercentile                                          M          RW
 docsPnmCmDsOfdmRxMerMean                                                M          RO
 docsPnmCmDsOfdmRxMerStdDev                                              M          RO
 docsPnmCmDsOfdmRxMerThrVal                                              M          RO
 docsPnmCmDsOfdmRxMerThrHighestFreq                                      M          RO
 docsPnmCmDsOfdmRxMerMeasStatus                                          M          RO
 docsPnmCmDsOfdmRxMerFileName                                            M          RW




02/19/25                                      CableLabs®                                 117
<!-- PDF_PAGE: 118 -->
CM-SP-CM-OSSIv3.1-I27-250219                         Data-Over-Cable Service Interface Specifications


 Object                                                                        CM           Access
 docsPnmCmDsOfdmMerMarTable                                                    M            N-Acc
 docsPnmCmDsOfdmMerMarEntry                                                    M            N-Acc
 docsPnmCmDsOfdmMerMarProfileId                                                M            RW
 docsPnmCmDsOfdmMerMarThrshldOffset                                            M            RW
 docsPnmCmDsOfdmMerMarMeasEnable                                               M            RW
 docsPnmCmDsOfdmMerMarNumSymPerSubCarToAvg                                     M            RW
 docsPnmCmDsOfdmMerMarReqAvgMer                                                M            RW
 docsPnmCmDsOfdmMerMarNumSubCarBelowThrshld                                    M            RO
 docsPnmCmDsOfdmMerMarMeasuredAvgMer                                           M            RO
 docsPnmCmDsOfdmMerMarAvgMerMargin                                             M            RO
 docsPnmCmDsOfdmMerMarMeasStatus                                               M            RO
 docsPnmCmDsOfdmFecTable                                                       M            N-Acc
 docsPnmCmDsOfdmFecEntry                                                       M            N-Acc
 docsPnmCmDsOfdmFecSumType                                                     M            RW
 docsPnmCmDsOfdmFecFileEnable                                                  M            RW
 docsPnmCmDsOfdmFecMeasStatus                                                  M            RO
 docsPnmCmDsOfdmFecFileName                                                    M            RW
 docsPnmCmDsOfdmReqMERObjects
 docsPnmCmDsOfdmReqMerQam16                                                    M            RW
 docsPnmCmDsOfdmReqMerQam64                                                    M            RW
 docsPnmCmDsOfdmReqMerQam128                                                   M            RW
 docsPnmCmDsOfdmReqMerQam256                                                   M            RW
 docsPnmCmDsOfdmReqMerQam512                                                   M            RW
 docsPnmCmDsOfdmReqMerQam1024                                                  M            RW
 docsPnmCmDsOfdmReqMerQam2048                                                  M            RW
 docsPnmCmDsOfdmReqMerQam4096                                                  M            RW
 docsPnmCmDsOfdmReqMerQam8192                                                  M            RW
 docsPnmCmDsOfdmReqMerQam16384                                                 M            RW
 docsPnmCmDsHistTable                                                          M            N-Acc
 docsPnmCmDsHistEntry                                                          M            N-Acc
 docsPnmCmDsHistEnable                                                         M            RW
 docsPnmCmDsHistTimeOut                                                        M            RW
 docsPnmCmDsHistMeasStatus                                                     M            RO
 docsPnmCmDsHistFileName                                                       M            RW
 docsPnmCmUsPreEqTable                                                         M            N-Acc
 docsPnmCmUsPreEqEntry                                                         M            N-Acc
 docsPnmCmUsPreEqFileEnable                                                    M            RW
 docsPnmCmUsPreEqAmpRipplePkToPk                                               M            RO
 docsPnmCmUsPreEqAmpRippleRms                                                  M            RO
 docsPnmCmUsPreEqAmpSlope                                                      M            RO
 docsPnmCmUsPreEqGrpDelayRipplePkToPk                                          M            RO
 docsPnmCmUsPreEqGrpDelayRippleRms                                             M            RO
 docsPnmCmUsPreEqPreEqCoAdjStatus                                              M            RO
 docsPnmCmUsPreEqMeasStatus                                                    M            RO
 docsPnmCmUsPreEqLastUpdateFileName                                            M            RW
 docsPnmCmUsPreEqFileName                                                      M            RW




118                                           CableLabs®                                    02/19/25
