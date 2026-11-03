# Annex A.1 - MIB-Object Details

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 116-152
Classification: Normative (explicit)

<!-- PDF_PAGE: 116 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


Annex A           Detailed MIB Requirements (Normative)
This Annex defines the SNMP MIB modules and MIB variables required for DOCSIS 4.0 CM devices.
                                     Table 68 - MIB Implementation Support

  Requirement Type      Table Notation                                       Description
 Deprecated             D                 Deprecated objects are optional. If a vendor chooses to implement the object, the
                                          object is required to be implemented correctly according to the MIB definition. If a
                                          vendor chooses not to implement the object, an agent is not to instantiate such
                                          object and is required to respond with the appropriate error/exception condition
                                          (e.g., 'noSuchObject' for SNMPv2c).
 Mandatory              M                 The object is required to be implemented correctly according to the MIB definition.
 Not Applicable         NA                Not applicable to the device.
 Not Supported          N-Sup             An agent is not to instantiate such object and is required to respond with the
                                          appropriate error/exception condition (e.g., 'noSuchObject' for SNMPv2c).
 Optional               O                 A vendor can choose to implement or not implement the object. If a vendor
                                          chooses to implement the object, the object is required to be implemented
                                          correctly according to the MIB definition. If a vendor chooses not to implement the
                                          object, an agent is not to instantiate such object and is required to respond with
                                          the appropriate error/exception condition (e.g., 'noSuchObject' for SNMPv2c).
 Obsolete               Ob                In SNMP convention, obsolete objects are not to be implemented. This
                                          specification allows vendors to implement or not implement obsolete objects. If a
                                          vendor chooses to implement an obsoleted object, the object is required be
                                          implemented correctly according to the MIB definition. If a vendor chooses not to
                                          implement the obsoleted object, the SNMP agent is not to instantiate such object
                                          and is required to respond with the appropriate error/exception condition (e.g.,
                                          'noSuchObject' for SNMPv2c).


                                     Table 69 - SNMP Access Requirements

 SNMP Access Type       Table Notation                                       Description
 N-Acc                  Not Accessible    The object is not accessible and is usually an index in a table
 Read-Create            RC                The access of the object is implemented as Read-Create
 Read-Write             RW                The access of the object is implemented as Read-Write
 Read-Only              RO                The access of the object is implemented as Read-Only
 Read-Create or         RC/RO             The access of the object is implemented as either Read-Create or Read-Only as
 Read-Only                                described in the MIB definition
 Read-Write/            RW/RO             The access of the object is implemented as either Read-Write or Read-Only as
 Read-Only                                described in the MIB definition
 Accessible for SNMP    Acc-FN            These objects are used for SNMP notifications by the CM SNMP Agent
 notifications



A.1 MIB-Object Details
The CM instantiates SNMP MIB objects based on its configuration and operational parameters acquired during
registration. Below are denominations for several Table 70 columns that indicate modes of operation where a CM
has specific management requirements for certain MIB object instantiation and syntax.
The CM always operates in "1.1 QoS Mode".
The CM SNMP access control configuration is either NmAccess Mode or SNMP Coexistence Mode.
The CM upstream channel types can be categorized as "OFDMA upstream" and "TDMA/ATDMA upstream".




116                                               CableLabs                                                        04/29/26
<!-- PDF_PAGE: 117 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


                                            Table 70 - MIB Object Details
DOCS-IF-MIB [RFC 4546]
NOTE: Refer to Section 7.1.3.10 for detailed requirements for handling SC-QAM and OFDM/OFDMA channels.
Object                                                                                                   CM         Access
docsIfDownstreamChannelTable                                                                       M            N-Acc
docsIfDownstreamChannelEntry                                                                       M            N-Acc
docsIfDownChannelId                                                                                M            RO
docsIfDownChannelFrequency                                                                         M            RO
docsIfDownChannelWidth                                                                             M            RO
docsIfDownChannelModulation                                                                        M            RO
docsIfDownChannelInterleave                                                                        M            RO
docsIfDownChannelPower                                                                             M            RO
docsIfDownChannelAnnex                                                                             M            RO
docsIfDownChannelStorageType                                                                       M            RO


                                          CM
                                         TDMA/                                                              CM
                                         ATDMA                                                            SCDMA
                Object                  Upstream   Access                     Object                     Upstream    Access
docsIfUpstreamChannelTable              M          N-Acc     docsIfUpstreamChannelTable                  O          N-Acc
docsIfUpstreamChannelEntry              M          N-Acc     docsIfUpstreamChannelEntry                  O          N-Acc
docsIfUpChannelId                       M          RO        docsIfUpChannelId                           O          RO
docsIfUpChannelFrequency                M          RO        docsIfUpChannelFrequency                    O          RO
docsIfUpChannelWidth                    M          RO        docsIfUpChannelWidth                        O          RO
docsIfUpChannelModulationProfile        M          RO        docsIfUpChannelModulationProfile            O          RO
docsIfUpChannelSlotSize                 M          RO        docsIfUpChannelSlotSize                     O          RO
docsIfUpChannelTxTimingOffset           M          RO        docsIfUpChannelTxTimingOffset               O          RO
docsIfUpChannelRangingBackoffStart      M          RO        docsIfUpChannelRangingBackoffStart          O          RO
docsIfUpChannelRangingBackoffEnd        M          RO        docsIfUpChannelRangingBackoffEnd            O          RO
docsIfUpChannelTxBackoffStart           M          RO        docsIfUpChannelTxBackoffStart               O          RO
docsIfUpChannelTxBackoffEnd             M          RO        docsIfUpChannelTxBackoffEnd                 O          RO
docsIfUpChannelScdmaActiveCodes         O          RO        docsIfUpChannelScdmaActiveCodes             O          RO
docsIfUpChannelScdmaCodesPerSlot        O          RO        docsIfUpChannelScdmaCodesPerSlot            O          RO
docsIfUpChannelScdmaFrameSize           O          RO        docsIfUpChannelScdmaFrameSize               O          RO
docsIfUpChannelScdmaHoppingSeed         O          RO        docsIfUpChannelScdmaHoppingSeed             O          RO
docsIfUpChannelType                     M          RO        docsIfUpChannelType                         O          RO
docsIfUpChannelCloneFrom                O          RO        docsIfUpChannelCloneFrom                    O          RO
docsIfUpChannelUpdate                   O          RO        docsIfUpChannelUpdate                       O          RO
docsIfUpChannelStatus                   O          RO        docsIfUpChannelStatus                       O          RO
docsIfUpChannelPreEqEnable              M          RO        docsIfUpChannelPreEqEnable                  O          RO


Object                                                                                                   CM         Access
docsIfQosProfileTable                                                                               D           N-Acc
docsIfQosProfileEntry                                                                              D            N-Acc
docsIfQosProfIndex                                                                                  D           N-Acc
docsIfQosProfPriority                                                                               D           RO
docsIfQosProfMaxUpBandwidth                                                                         D           RO
docsIfQosProfGuarUpBandwidth                                                                        D           RO




04/29/26                                             CableLabs                                                        117
<!-- PDF_PAGE: 118 -->
CM-SP-CM-OSSIv4.0-I13-260429                        Data-Over-Cable Service Interface Specifications


Object                                                                                CM     Access
docsIfQosProfMaxDownBandwidth                                                    D          RO
docsIfQosProfMaxTxBurst                                                          D          RO
docsIfQosProfBaselinePrivacy                                                     D          RO
docsIfQosProfStatus                                                              D          RO
docsIfQosProfMaxTransmitBurst                                                    D          RO
docsIfQosProfStorageType                                                         D          RO
docsIfSignalQualityTable                                                         M          N-Acc
docsIfSignalQualityEntry                                                         M          N-Acc
docsIfSigQIncludesContention                                                     M          RO
docsIfSigQUnerroreds                                                             M          RO
docsIfSigQCorrecteds                                                             M          RO
docsIfSigQUncorrectables                                                         M          RO
docsIfSigQSignalNoise                                                            D          RO
docsIfSigQMicroreflections                                                       M          RO
docsIfSigQEqualizationData                                                       M          RO
docsIfSigQExtUnerroreds                                                          M          RO
docsIfSigQExtCorrecteds                                                          M          RO
docsIfSigQExtUncorrectables                                                      M          RO
docsIfDocsisBaseCapability                                                       D          RO
docsIfCmMacTable                                                                 M          N-Acc
docsIfCmMacEntry                                                                 M          N-Acc
docsIfCmCmtsAddress                                                              M          RO
docsIfCmCapabilities                                                             M          RO
docsIfCmRangingRespTimeout                                                       Ob         RW
docsIfCmRangingTimeout                                                           M          RW
docsIfCmStatusTable                                                              D          N-Acc
docsIfCmStatusEntry                                                              D          N-Acc
docsIfCmStatusValue                                                              D          RO
docsIfCmStatusCode                                                               D          RO
docsIfCmStatusTxPower                                                            D          RO
docsIfCmStatusResets                                                             D          RO
docsIfCmStatusLostSyncs                                                          D          RO
docsIfCmStatusInvalidMaps                                                        D          RO
docsIfCmStatusInvalidUcds                                                        D          RO
docsIfCmStatusInvalidRangingResponses                                            D          RO
docsIfCmStatusInvalidRegistrationResponses                                       D          RO
docsIfCmStatusT1Timeouts                                                         D          RO
docsIfCmStatusT2Timeouts                                                         D          RO
docsIfCmStatusT3Timeouts                                                         D          RO
docsIfCmStatusT4Timeouts                                                         D          RO
docsIfCmStatusRangingAborteds                                                    D          RO
docsIfCmStatusDocsisOperMode                                                     D          RO
docsIfCmStatusModulationType                                                     D          RO
docsIfCmStatusEqualizationData                                                   D          RO
docsIfCmStatusUCCs                                                               D          RO
docsIfCmStatusUCCFails                                                           D          RO




118                                          CableLabs                                    04/29/26
<!-- PDF_PAGE: 119 -->
Cable Modem Operations Support System Interface Specification                            CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                                                       CM   Access
IF-MIB [RFC 2863]
ifNumber                                                                                                 M        RO
ifTableLastChange                                                                                        M        RO
ifTable                                                                                                  M        N-Acc
NOTE: The ifTable Counter32 objects are not reflected here; refer to Table 73 and Table 74 of A.5 for
details on these objects.
ifEntry                                                                                                  M        N-Acc
ifIndex                                                                                                  M        RO
ifDescr                                                                                                  M        RO
ifType                                                                                                   M        RO
ifMtu                                                                                                    M        RO
ifSpeed                                                                                                  M        RO
ifPhysAddress                                                                                            M        RO
ifAdminStatus                                                                                            M        RW
ifOperStatus                                                                                             M        RO
ifLastChange                                                                                             M        RO
ifOutQLen                                                                                                D        RO
ifSpecific                                                                                               D        RO
ifXTable                                                                                                 M        N-Acc
NOTE: The ifXTable Counter32 and Counter64 objects are not reflected here; refer to Table 73 and Table
74 of A.5 for details on these objects.
ifXEntry                                                                                                 M        N-Acc
ifName                                                                                                   M        RO
ifLinkUpDownTrapEnable                                                                                   M        RW
ifHighSpeed                                                                                              M        RO
ifPromiscuousMode                                                                                        M        RW/RO
ifConnectorPresent                                                                                       M        RO
ifAlias                                                                                                  M        RW/RO
ifCounterDiscontinuityTime                                                                               M        RO
ifStackTable                                                                                             M        N-Acc
ifStackEntry                                                                                             M        N-Acc
ifStackHigherLayer                                                                                       M        N-Acc
ifStackLowerLayer                                                                                        M        N-Acc
ifStackStatus                                                                                            M        RC/RO
ifStackLastChange                                                                                        M        RO
ifRcvAddressTable                                                                                        O        N-Acc
ifRcvAddressEntry                                                                                        O        N-Acc
ifRcvAddressAddress                                                                                      O        N-Acc
ifRcvAddressStatus                                                                                       O        RC
IfRcvAddressType                                                                                         O        RC
Notification
linkUp                                                                                                   M        Acc-FN
linkDown                                                                                                 M        Acc-FN
NOTE: See Section 7.1.3.11.4 for details.
ifTestTable                                                                                              D        N-Acc
ifTestEntry                                                                                              D        N-Acc
ifTestId                                                                                                 D        RW




04/29/26                                                 CableLabs                                                    119
<!-- PDF_PAGE: 120 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


Object                                                                                         CM      Access
ifTestStatus                                                                               D          RW
ifTestType                                                                                 D          RW
ifTestResult                                                                               D          RO
ifTestCode                                                                                 D          RO
ifTestOwner                                                                                D          RW
BRIDGE-MIB [RFC 4188]
dot1dBase
dot1dBaseBridgeAddress                                                                     M          RO
dot1dBaseNumPorts                                                                          M          RO
dot1dBaseType                                                                              M          RO
dot1dBasePortTable                                                                         M          N-Acc
dot1dBasePortEntry                                                                         M          N-Acc
dot1dBasePort                                                                              M          RO
dot1dBasePortIfIndex                                                                       M          RO
dot1dBasePortCircuit                                                                       M          RO
dot1dBasePortDelayExceededDiscards                                                         M          RO
dot1dBasePortMtuExceededDiscards                                                           M          RO
dot1dStp
dot1dStpProtocolSpecification                                                              M          RO
dot1dStpPriority                                                                           M          RW
dot1dStpTimeSinceTopologyChange                                                            M          RO
dot1dStpTopChanges                                                                         M          RO
dot1dStpDesignatedRoot                                                                     M          RO
dot1dStpRootCost                                                                           M          RO
dot1dStpRootPort                                                                           M          RO
dot1dStpMaxAge                                                                             M          RO
dot1dStpHelloTime                                                                          M          RO
dot1dStpHoldTime                                                                           M          RO
dot1dStpForwardDelay                                                                       M          RO
dot1dStpBridgeMaxAge                                                                       M          RW
dot1dStpBridgeHelloTime                                                                    M          RW
dot1dStpBridgeForwardDelay                                                                 M          RW
dot1dStpPortTable                                                                          O          N-Acc
NOTE: This table is required ONLY if STP is implemented.
dot1dStpPortEntry                                                                          O          N-Acc
dot1dStpPort                                                                               O          RO
dot1dStpPortPriority                                                                       O          RW
dot1dStpPortState                                                                          O          RO
dot1dStpPortEnable                                                                         O          RW
dot1dStpPortPathCost                                                                       O          RW
dot1dStpPortDesignatedRoot                                                                 O          RO
dot1dStpPortDesignatedCost                                                                 O          RO
dot1dStpPortDesignatedBridge                                                               O          RO
dot1dStpPortDesignatedPort                                                                 O          RO
dot1dStpPortForwardTransitions                                                             O          RO
dot1dStpPortPathCost32                                                                     O          RO




120                                                    CableLabs                                    04/29/26
<!-- PDF_PAGE: 121 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                                             CM        Access
dot1dTp
NOTE: This group is required ONLY if transparent bridging is implemented.
dot1dTpLearnedEntryDiscards                                                                    M             RO
dot1dTpAgingTime                                                                               M             RW
dot1dTpFdbTable                                                                                M             N-Acc
dot1dTpFdbEntry                                                                                M             N-Acc
dot1dTpFdbAddress                                                                              M             RO
dot1dTpFdbPort                                                                                 M             RO
dot1dTpFdbStatus                                                                               M             RO
dot1dTpPortTable                                                                               M             N-Acc
dot1dTpPortEntry                                                                               M             N-Acc
dot1dTpPort                                                                                    M             RO
dot1dTpPortMaxInfo                                                                             M             RO
dot1dTpPortInFrames                                                                            M             RO
dot1dTpPortOutFrames                                                                           M             RO
dot1dTpPortInDiscards                                                                          M             RO
dot1dStaticTable                                                                               O             N-Acc
NOTE: Implementation of dot1dStaticTable is OPTIONAL.
dot1dStaticEntry                                                                               O             N-Acc
dot1dStaticAddress                                                                             O             RW
dot1dStaticReceivePort                                                                         O             RW
dot1dStaticAllowedToGoTo                                                                       O             RW
dot1dStaticStatus                                                                              O             RW
Notification
newRoot                                                                                        O             Acc-FN
topologyChange                                                                                 O             Acc-FN
DOCS-CABLE-DEVICE-MIB [RFC 4639]
docsDevBase
docsDevRole                                                                                    M             RO
docsDevDateTime                                                                                M             RO/RW
docsDevResetNow                                                                                M             RW
docsDevSerialNumber                                                                            M             RO
docsDevSTPControl                                                                              M             RW/RO
docsDevIgmpModeControl                                                                         N-Sup
docsDevMaxCpe                                                                                  M             RO


                                         CM in                                                 CM in SNMP
                                       NmAccess                                                Coexistence
               Object                    Mode         Access                  Object              Mode        Access
docsDevNmAccessTable                    M            N-Acc        docsDevNmAccessTable         N-Sup
docsDevNmAccessEntry                    M            N-Acc        docsDevNmAccessEntry         N-Sup
docsDevNmAccessIndex                    M            N-Acc        docsDevNmAccessIndex         N-Sup
docsDevNmAccessIp                       M            RC           docsDevNmAccessIp            N-Sup
docsDevNmAccessIpMask                   M            RC           docsDevNmAccessIpMask        N-Sup
docsDevNmAccessCommunity                M            RC           docsDevNmAccessCommunity     N-Sup
docsDevNmAccessControl                  M            RC           docsDevNmAccessControl       N-Sup
docsDevNmAccessInterfaces               M            RC           docsDevNmAccessInterfaces    N-Sup




04/29/26                                                CableLabs                                                121
<!-- PDF_PAGE: 122 -->
CM-SP-CM-OSSIv4.0-I13-260429                     Data-Over-Cable Service Interface Specifications


docsDevNmAccessStatus                M   RC     docsDevNmAccessStatus         N-Sup
docsDevNmAccessTrapVersion           M   RC     docsDevNmAccessTrapVersion    N-Sup


Object                                                                            CM     Access
docsDevSoftware
docsDevSwServer                                                               D          RW
docsDevSwFilename                                                             M          RW
docsDevSwAdminStatus                                                          M          RW
docsDevSwOperStatus                                                           M          RO
docsDevSwCurrentVers                                                          M          RO
docsDevSwServerAddressType                                                    M          RW
docsDevSwServerAddress                                                        M          RW
docsDevSwServerTransportProtocol                                              M          RW
docsDevServer
docsDevServerBootState                                                        D          RO
docsDevServerDhcp                                                             D          RO
docsDevServerTime                                                             D          RO
docsDevServerTftp                                                             D          RO
docsDevServerConfigFile                                                       M          RO
docsDevServerDhcpAddressType                                                  M          RO
docsDevServerDhcpAddress                                                      M          RO
docsDevServerTimeAddressType                                                  M          RO
docsDevServerTimeAddress                                                      M          RO
docsDevServerConfigTftpAddressType                                            M          RO
docsDevServerConfigTftpAddress                                                M          RO
docsDevEvent
docsDevEvControl                                                              M          RW
docsDevEvSyslog                                                               D          RW
docsDevEvThrottleAdminStatus                                                  M          RW
docsDevEvThrottleInhibited                                                    D          RO
docsDevEvThrottleThreshold                                                    M          RW
docsDevEvThrottleInterval                                                     M          RW
docsDevEvControlTable                                                         M          N-Acc
docsDevEvControlEntry                                                         M          N-Acc
docsDevEvPriority                                                             M          N-Acc
docsDevEvReporting                                                            M          RW
docsDevEventTable                                                             M          N-Acc
docsDevEventEntry                                                             M          N-Acc
docsDevEvIndex                                                                M          N-Acc
docsDevEvFirstTime                                                            M          RO
docsDevEvLastTime                                                             M          RO
docsDevEvCounts                                                               M          RO
docsDevEvLevel                                                                M          RO
docsDevEvId                                                                   M          RO
docsDevEvText                                                                 M          RO
docsDevEvSyslogAddressType                                                    M          RW
docsDevEvSyslogAddress                                                        M          RW




122                                       CableLabs                                    04/29/26
<!-- PDF_PAGE: 123 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
docsDevEvThrottleThresholdExceeded                                          M        RO
docsDevFilter
docsDevFilterLLCUnmatchedAction                                             D        RW
docsDevFilterLLCTable                                                       D        N-Acc
docsDevFilterLLCEntry                                                       D        N-Acc
docsDevFilterLLCIndex                                                       D        N-Acc
docsDevFilterLLCStatus                                                      D        RC
docsDevFilterLLCIfIndex                                                     D        RC
docsDevFilterLLCProtocolType                                                D        RC
docsDevFilterLLCProtocol                                                    D        RC
docsDevFilterLLCMatches                                                     D        RO
docsDevFilterIpDefault                                                      D        RW
docsDevFilterIpTable                                                        D        N-Acc
docsDevFilterIpEntry                                                        D        N-Acc
docsDevFilterIpIndex                                                        D        N-Acc
docsDevFilterIpStatus                                                       D        RC
docsDevFilterIpControl                                                      D        RC
docsDevFilterIpIfIndex                                                      D        RC
docsDevFilterIpDirection                                                    D        RC
docsDevFilterIpBroadcast                                                    D        RC
docsDevFilterIpSaddr                                                        D        RC
docsDevFilterIpSmask                                                        D        RC
docsDevFilterIpDaddr                                                        D        RC
docsDevFilterIpDmask                                                        D        RC
docsDevFilterIpProtocol                                                     D        RC
docsDevFilterIpSourcePortLow                                                D        RC
docsDevFilterIpSourcePortHigh                                               D        RC
docsDevFilterIpDestPortLow                                                  D        RC
docsDevFilterIpDestPortHigh                                                 D        RC
docsDevFilterIpMatches                                                      D        RO
docsDevFilterIpTos                                                          D        RC
docsDevFilterIpTosMask                                                      D        RC
docsDevFilterIpContinue                                                     D        RC
docsDevFilterIpPolicyId                                                     D        RC
docsDevFilterPolicyTable                                                    D        N-Acc
docsDevFilterPolicyEntry                                                    D        N-Acc
docsDevFilterPolicyIndex                                                    D        N-Acc
docsDevFilterPolicyId                                                       D        RC
docsDevFilterPolicyStatus                                                   D        RC
docsDevFilterPolicyPtr                                                      D        RC
docsDevFilterTosTable                                                       D        N-Acc
docsDevFilterTosEntry                                                       D        N-Acc
docsDevFilterTosIndex                                                       D        N-Acc
docsDevFilterTosStatus                                                      D        RC
docsDevFilterTosAndMask                                                     D        RC
docsDevFilterTosOrMask                                                      D        RC




04/29/26                                   CableLabs                                     123
<!-- PDF_PAGE: 124 -->
CM-SP-CM-OSSIv4.0-I13-260429          Data-Over-Cable Service Interface Specifications


Object                                                                  CM    Access
docsDevCpeEnroll                                                   O          RW
docsDevCpeIpMax                                                    O          RW
docsDevCpeTable                                                    Ob         N-Acc
docsDevCpeEntry                                                    Ob         N-Acc
docsDevCpeIp                                                       Ob         N-Acc
docsDevCpeSource                                                   Ob         RO
docsDevCpeStatus                                                   Ob         RC
docsDevCpeInetTable                                                O          N-Acc
docsDevCpeInetEntry                                                O          N-Acc
docsDevCpeInetType                                                 O          N-Acc
docsDevCpeInetAddr                                                 O          RC
docsDevCpeInetSource                                               O          RO
docsDevCpeInetRowStatus                                            O          RC
IP-MIB [RFC 4293]
ipv4GeneralGroup
ipForwarding                                                       M          RW
ipDefaultTTL                                                       M          RW
ipReasmTimeout                                                     M          RW
ipv6GeneralGroup2
ipv6IpForwarding                                                   M          RW
ipv6IpDefaultHopLimit                                              M          RW
ipv4InterfaceTableLastChange                                       M          RO
ipv4InterfaceTable                                                 M          N-Acc
ipv4InterfaceEntry                                                 M          N-Acc
ipv4InterfaceIfIndex                                               M          N-Acc
ipv4InterfaceReasmMaxSize                                          M          RO
ipv4InterfaceEnableStatus                                          M          RW
ipv4InterfaceRetransmitTime                                        M          RO
ipv6InterfaceTableLastChange                                       M          RO
ipv6InterfaceTable                                                 M          N-Acc
ipv6InterfaceEntry                                                 M          N-Acc
ipv6InterfaceIfIndex                                               M          N-Acc
ipv6InterfaceReasmMaxSize                                          M          RO
ipv6InterfaceIdentifier                                            M          RO
ipv6InterfaceEnableStatus                                          M          RW
ipv6InterfaceReachableTime                                         M          RO
ipv6InterfaceRetransmitTime                                        M          RO
ipv6InterfaceForwarding                                            M          RW
ipSystemStatsTable                                                 O          N-Acc
ipSystemStatsEntry                                                 O          N-Acc
ipSystemStatsIPVersion                                             O          N-Acc
ipSystemStatsInReceives                                            O          RO
ipSystemStatsHCInReceives                                          O          RO
ipSystemStatsInOctets                                              O          RO
ipSystemStatsHCInOctets                                            O          RO
ipSystemStatsInHdrErrors                                           O          RO




124                            CableLabs                                    04/29/26
<!-- PDF_PAGE: 125 -->
Cable Modem Operations Support System Interface Specification        CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                               CM    Access
ipSystemStatsInNoRoutes                                                          O        RO
ipSystemStatsInAddrErrors                                                        O        RO
ipSystemStatsInUnknownProtos                                                     O        RO
ipSystemStatsInTruncatedPkts                                                     O        RO
ipSystemStatsInForwDatagrams                                                     O        RO
ipSystemStatsHCInForwDatagrams                                                   O        RO
ipSystemStatsReasmReqds                                                          O        RO
ipSystemStatsReasmOKs                                                            O        RO
ipSystemStatsReasmFails                                                          O        RO
ipSystemStatsInDiscards                                                          O        RO
ipSystemStatsInDelivers                                                          O        RO
ipSystemStatsHCInDelivers                                                        O        RO
ipSystemStatsOutRequests                                                         O        RO
ipSystemStatsHCOutRequests                                                       O        RO
ipSystemStatsOutNoRoutes                                                         O        RO
ipSystemStatsOutForwDatagrams                                                    O        RO
ipSystemStatsHCOutForwDatagrams                                                  O        RO
ipSystemStatsOutDiscards                                                         O        RO
ipSystemStatsOutFragReqds                                                        O        RO
ipSystemStatsOutFragOKs                                                          O        RO
ipSystemStatsOutFragFails                                                        O        RO
ipSystemStatsOutFragCreates                                                      O        RO
ipSystemStatsOutTransmits                                                        O        RO
ipSystemStatsHCOutTransmits                                                      O        RO
ipSystemStatsOutOctets                                                           O        RO
ipSystemStatsHCOutOctets                                                         O        RO
ipSystemStatsInMcastPkts                                                         O        RO
ipSystemStatsHCInMcastPkts                                                       O        RO
ipSystemStatsInMcastOctets                                                       O        RO
ipSystemStatsHCInMcastOctets                                                     O        RO
ipSystemStatsOutMcastPkts                                                        O        RO
ipSystemStatsHCOutMcastPkts                                                      O        RO
ipSystemStatsOutMcastOctets                                                      O        RO
ipSystemStatsHCOutMcastOctets                                                    O        RO
ipSystemStatsInBcastPkts                                                         O        RO
ipSystemStatsHCInBcastPkts                                                       O        RO
ipSystemStatsOutBcastPkts                                                        O        RO
ipSystemStatsHCOutBcastPkts                                                      O        RO
ipSystemStatsDiscontinuityTime                                                   O        RO
ipSystemStatsRefreshRate                                                         O        RO
ipIfStatsTableLastChange                                                         O        RO
ipIfStatsTable                                                                   O        N-Acc
NOTE: This table is required ONLY if routing is implemented.
ipIfStatsEntry                                                                   O        N-Acc
ipIfStatsIPVersion                                                               O        N-Acc
ipIfStatsIfIndex                                                                 O        N-Acc




04/29/26                                                CableLabs                             125
<!-- PDF_PAGE: 126 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


Object                                                                                          CM     Access
ipIfStatsInReceives                                                                         O          RO
ipIfStatsHCInReceives                                                                       O          RO
ipIfStatsInOctets                                                                           O          RO
ipIfStatsHCInOctets                                                                         O          RO
ipIfStatsInHdrErrors                                                                        O          RO
ipIfStatsInNoRoutes                                                                         O          RO
ipIfStatsInAddrErrors                                                                       O          RO
ipIfStatsInUnknownProtos                                                                    O          RO
ipIfStatsInTruncatedPkts                                                                    O          RO
ipIfStatsInForwDatagrams                                                                    O          RO
ipIfStatsHCInForwDatagrams                                                                  O          RO
ipIfStatsReasmReqds                                                                         O          RO
ipIfStatsReasmOKs                                                                           O          RO
ipIfStatsReasmFails                                                                         O          RO
ipIfStatsInDiscards                                                                         O          RO
ipIfStatsInDelivers                                                                         O          RO
ipIfStatsHCInDelivers                                                                       O          RO
ipIfStatsOutRequests                                                                        O          RO
ipIfStatsHCOutRequests                                                                      O          RO
ipIfStatsOutForwDatagrams                                                                   O          RO
ipIfStatsHCOutForwDatagrams                                                                 O          RO
ipIfStatsOutDiscards                                                                        O          RO
ipIfStatsOutFragReqds                                                                       O          RO
ipIfStatsOutFragOKs                                                                         O          RO
ipIfStatsOutFragFails                                                                       O          RO
ipIfStatsOutFragCreates                                                                     O          RO
ipIfStatsOutTransmits                                                                       O          RO
ipIfStatsHCOutTransmits                                                                     O          RO
ipIfStatsOutOctets                                                                          O          RO
ipIfStatsHCOutOctets                                                                        O          RO
ipIfStatsInMcastPkts                                                                        O          RO
ipIfStatsHCInMcastPkts                                                                      O          RO
ipIfStatsInMcastOctets                                                                      O          RO
ipIfStatsHCInMcastOctets                                                                    O          RO
ipIfStatsOutMcastPkts                                                                       O          RO
ipIfStatsHCOutMcastPkts                                                                     O          RO
ipIfStatsOutMcastOctets                                                                     O          RO
ipIfStatsHCOutMcastOctets                                                                   O          RO
ipIfStatsInBcastPkts                                                                        O          RO
ipIfStatsHCInBcastPkts                                                                      O          RO
ipIfStatsOutBcastPkts                                                                       O          RO
ipIfStatsHCOutBcastPkts                                                                     O          RO
ipIfStatsDiscontinuityTime                                                                  O          RO
ipIfStatsRefreshRate                                                                        O          RO
ipAddressPrefixTable                                                                        O          N-Acc
NOTE: This table is required ONLY if routing is implemented.




126                                                     CableLabs                                    04/29/26
<!-- PDF_PAGE: 127 -->
Cable Modem Operations Support System Interface Specification        CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                               CM    Access
ipAddressPrefixEntry                                                             O        N-Acc
ipAddressPrefixIfIndex                                                           O        N-Acc
ipAddressPrefixType                                                              O        N-Acc
ipAddressPrefixPrefix                                                            O        N-Acc
ipAddressPrefixLength                                                            O        N-Acc
ipAddressPrefixOrigin                                                            O        RO
ipAddressPrefixOnLinkFlag                                                        O        RO
ipAddressPrefixAutonomousFlag                                                    O        RO
ipAddressPrefixAdvPreferredLifetime                                              O        RO
ipAddressPrefixAdvValidLifetime                                                  O        RO
ipAddressSpinLock                                                                O        RW
ipAddressTable                                                                   O        N-Acc
ipAddressEntry                                                                   O        N-Acc
ipAddressAddrType                                                                O        N-Acc
ipAddressAddr                                                                    O        N-Acc
ipAddressIfIndex                                                                 O        RC
ipAddressType                                                                    O        RC
ipAddressPrefix                                                                  O        RO
ipAddressOrigin                                                                  O        RO
ipAddressStatus                                                                  O        RC
ipAddressCreated                                                                 O        RC
ipAddressLastChanged                                                             O        RC
ipAddressRowStatus                                                               O        RC
ipAddressStorageType                                                             O        RC
ipNetToPhysicalTable                                                             O        N-Acc
NOTE: This table is required ONLY if routing is implemented.
ipNetToPhysicalEntry                                                             O        N-Acc
ipNetToPhysicalIfIndex                                                           O        N-Acc
ipNetToPhysicalNetAddressType                                                    O        N-Acc
ipNetToPhysicalNetAddress                                                        O        N-Acc
ipNetToPhysicalPhysAddress                                                       O        RC
ipNetToPhysicalLastUpdated                                                       O        RO
ipNetToPhysicalType                                                              O        RC
ipNetToPhysicalState                                                             O        RO
ipNetToPhysicalRowStatus                                                         O        RC
ipDefaultRouterTable                                                             O        N-Acc
NOTE: This table is required ONLY if routing is implemented.
ipDefaultRouterEntry                                                             O        N-Acc
ipDefaultRouterAddressType                                                       O        N-Acc
ipDefaultRouterAddress                                                           O        N-Acc
ipDefaultRouterIfIndex                                                           O        N-Acc
ipDefaultRouterLifetime                                                          O        RC
ipDefaultRouterPreference                                                        O        RO
icmpStatsTable                                                                   M        N-Acc
icmpStatsEntry                                                                   M        N-Acc
icmpStatsIPVersion                                                               M        N-Acc




04/29/26                                                CableLabs                             127
<!-- PDF_PAGE: 128 -->
CM-SP-CM-OSSIv4.0-I13-260429          Data-Over-Cable Service Interface Specifications


Object                                                                 CM     Access
icmpStatsInMsgs                                                    M          RO
icmpStatsInErrors                                                  M          RO
icmpStatsOutMsgs                                                   M          RO
icmpStatsOutErrors                                                 M          RO
icmpMsgStatsTable                                                  M          N-Acc
icmpMsgStatsEntry                                                  M          N-Acc
icmpMsgStatsIPVersion                                              M          N-Acc
icmpMsgStatsType                                                   M          N-Acc
icmpMsgStatsInPkts                                                 M          RO
icmpMsgStatsOutPkts                                                M          RO
UDP-MIB [RFC 4113]
UDPGroup
udpInDatagrams                                                     O          RO
udpNoPorts                                                         O          RO
udpInErrors                                                        O          RO
udpOutDatagrams                                                    O          RO
udpEndpointTable                                                   O          N-Acc
udpEndpointEntry                                                   O          N-Acc
udpEndpointLocalAddressType                                        O          N-Acc
udpEndpointLocalAddress                                            O          N-Acc
udpEndpointLocalPort                                               O          N-Acc
udpEndpointRemoteAddressType                                       O          N-Acc
udpEndpointRemoteAddress                                           O          N-Acc
udpEndpointRemotePort                                              O          N-Acc
udpEndpointInstance                                                O          N-Acc
udpEndpointProcess                                                 O          RO
TCP-MIB [RFC 4022]
tcpBaseGroup
tcpRtoAlgorithm                                                    O          RO
tcpRtoMin                                                          O          RO
tcpRtoMax                                                          O          RO
tcpMaxConn                                                         O          RO
tcpActiveOpens                                                     O          RO
tcpPassiveOpens                                                    O          RO
tcpAttemptFails                                                    O          RO
tcpEstabResets                                                     O          RO
tcpCurrEstab                                                       O          RO
tcpInSegs                                                          O          RO
tcpOutSegs                                                         O          RO
tcpRetransSegs                                                     O          RO
tcpInErrs                                                          O          RO
tcpOutRsts                                                         O          RO
tcpHCGroup
tcpHCInSegs                                                        O          RO
tcpHCOutSegs                                                       O          RO
tcpConnectionTable                                                 O          N-Acc




128                            CableLabs                                    04/29/26
<!-- PDF_PAGE: 129 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                           CM   Access
tcpConnectionEntry                                                          O         N-Acc
tcpConnectionLocalAddressType                                               O         N-Acc
tcpConnectionLocalAddress                                                   O         N-Acc
tcpConnectionLocalPort                                                      O         N-Acc
tcpConnectionRemAddressType                                                 O         N-Acc
tcpConnectionRemAddress                                                     O         N-Acc
tcpConnectionRemPort                                                        O         N-Acc
tcpConnectionState                                                          O         RW
tcpConnectionProcess                                                        O         RO
tcpListenerTable                                                            O         N-Acc
tcpListenerEntry                                                            O         N-Acc
tcpListenerLocalAddressType                                                 O         N-Acc
tcpListenerLocalAddress                                                     O         N-Acc
tcpListenerLocalPort                                                        O         N-Acc
tcpListenerProcess                                                          O         RO
SNMPv2-MIB [RFC 3418]
SystemGroup
sysDescr                                                                    M         RO
sysObjectID                                                                 M         RO
sysUpTime                                                                   M         RO
sysContact                                                                  M         RW
sysName                                                                     M         RW
sysLocation                                                                 M         RW
sysServices                                                                 M         RO
sysORLastChange                                                             M         RO
sysORTable                                                                  M         N-Acc
sysOREntry                                                                  M         N-Acc
sysORIndex                                                                  M         N-Acc
sysORID                                                                     M         RO
sysORDescr                                                                  M         RO
sysORUpTime                                                                 M         RO
SNMPGroup
snmpInPkts                                                                  M         RO
snmpInBadVersions                                                           M         RO
snmpOutPkts                                                                 Ob        RO
snmpInBadCommunityNames                                                     M         RO
snmpInBadCommunityUses                                                      M         RO
snmpInASNParseErrs                                                          M         RO
snmpInTooBigs                                                               Ob        RO
snmpInNoSuchNames                                                           Ob        RO
snmpInBadValues                                                             Ob        RO
snmpInReadOnlys                                                             Ob        RO
snmpInGenErrs                                                               Ob        RO
snmpInTotalReqVars                                                          Ob        RO
snmpInTotalSetVars                                                          Ob        RO
snmpInGetRequests                                                           Ob        RO




04/29/26                                   CableLabs                                      129
<!-- PDF_PAGE: 130 -->
CM-SP-CM-OSSIv4.0-I13-260429                Data-Over-Cable Service Interface Specifications


Object                                                                        CM    Access
snmpInGetNexts                                                           Ob         RO
snmpInSetRequests                                                        Ob         RO
snmpInGetResponses                                                       Ob         RO
snmpInTraps                                                              Ob         RO
snmpOutTooBigs                                                           Ob         RO
snmpOutNoSuchNames                                                       Ob         RO
snmpOutBadValues                                                         Ob         RO
snmpOutGenErrs                                                           Ob         RO
snmpOutGetRequests                                                       Ob         RO
snmpOutGetNexts                                                          Ob         RO
snmpOutSetRequests                                                       Ob         RO
snmpOutGetResponses                                                      Ob         RO
snmpOutTraps                                                             Ob         RO
snmpEnableAuthenTraps                                                    M          RW
snmpSilentDrops                                                          M          RO
snmpProxyDrops                                                           M          RO
snmpTrapsGroup
coldStart                                                                O          Acc-FN
warmStart                                                                O          Acc-FN
authenticationFailure                                                    M          Acc-FN
snmpSetGroup
snmpSetSerialNo                                                          M          RW
Etherlike-MIB [RFC 3635]
dot3StatsTable                                                           O          N-Acc
dot3StatsEntry                                                           O          N-Acc
dot3StatsIndex                                                           O          RO
dot3StatsAlignmentErrors                                                 O          RO
dot3StatsFCSErrors                                                       O          RO
dot3StatsInternalMacTransmitErrors                                       O          RO
dot3StatsFrameTooLongs                                                   O          RO
dot3StatsInternalMacReceiveErrors                                        O          RO
dot3StatsSymbolErrors                                                    O          RO
dot3StatsSingleCollisionFrames                                           O          RO
dot3StatsMultipleCollisionFrames                                         O          RO
dot3StatsDeferredTransmissions                                           O          RO
dot3StatsLateCollisions                                                  O          RO
dot3StatsExcessiveCollisions                                             O          RO
dot3StatsCarrierSenseErrors                                              O          RO
dot3StatsDuplexStatus                                                    O          RO
dot3StatsSQETestErrors                                                   O          RO
dot3CollTable                                                            O          N-Acc
dot3CollEntry                                                            O          N-Acc
dot3CollCount                                                            O          NA
dot3CollFrequencies                                                      O          RO
dot3ControlTable                                                         O          N-Acc
dot3ControlEntry                                                         O          N-Acc




130                                  CableLabs                                    04/29/26
<!-- PDF_PAGE: 131 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
dot3ControlFunctionsSupported                                               O        RO
dot3ControlInUnknownOpcodes                                                 O        RO
dot3PauseTable                                                              O        N-Acc
dot3PauseEntry                                                              O        N-Acc
dot3PauseAdminMode                                                          O        RW
dot3PauseOperMode                                                           O        RO
dot3InPauseFrames                                                           O        RO
dot3OutPauseFrames                                                          O        RO
DOCS-IETF-BPI2-MIB [RFC 4131]
docsBpi2CmBaseTable                                                         M        N-Acc
docsBpi2CmBaseEntry                                                         M        N-Acc
docsBpi2CmPrivacyEnable                                                     M        RO
docsBpi2CmPublicKey                                                         M        RO
docsBpi2CmAuthState                                                         M        RO
docsBpi2CmAuthKeySequenceNumber                                             M        RO
docsBpi2CmAuthExpiresOld                                                    M        RO
docsBpi2CmAuthExpiresNew                                                    M        RO
docsBpi2CmAuthReset                                                         M        RW
docsBpi2CmAuthGraceTime                                                     M        RO
docsBpi2CmTEKGraceTime                                                      M        RO
docsBpi2CmAuthWaitTimeout                                                   M        RO
docsBpi2CmReauthWaitTimeout                                                 M        RO
docsBpi2CmOpWaitTimeout                                                     M        RO
docsBpi2CmRekeyWaitTimeout                                                  M        RO
docsBpi2CmAuthRejectWaitTimeout                                             M        RO
docsBpi2CmSAMapWaitTimeout                                                  M        RO
docsBpi2CmSAMapMaxRetries                                                   M        RO
docsBpi2CmAuthentInfos                                                      M        RO
docsBpi2CmAuthRequests                                                      M        RO
docsBpi2CmAuthReplies                                                       M        RO
docsBpi2CmAuthRejects                                                       M        RO
docsBpi2CmAuthInvalids                                                      M        RO
docsBpi2CmAuthRejectErrorCode                                               M        RO
docsBpi2CmAuthRejectErrorString                                             M        RO
docsBpi2CmAuthInvalidErrorCode                                              M        RO
docsBpi2CmAuthInvalidErrorString                                            M        RO
docsBpi2CmTEKTable                                                          M        N-Acc
docsBpi2CmTEKEntry                                                          M        N-Acc
docsBpi2CmTEKSAId                                                           M        N-Acc
docsBpi2CmTEKSAType                                                         M        RO
docsBpi2CmTEKDataEncryptAlg                                                 M        RO
docsBpi2CmTEKDataAuthentAlg                                                 M        RO
docsBpi2CmTEKState                                                          M        RO
docsBpi2CmTEKKeySequenceNumber                                              M        RO
docsBpi2CmTEKExpiresOld                                                     M        RO
docsBpi2CmTEKExpiresNew                                                     M        RO




04/29/26                                   CableLabs                                     131
<!-- PDF_PAGE: 132 -->
CM-SP-CM-OSSIv4.0-I13-260429                         Data-Over-Cable Service Interface Specifications


Object                                                                                CM     Access
docsBpi2CmTEKKeyRequests                                                          M          RO
docsBpi2CmTEKKeyReplies                                                           M          RO
docsBpi2CmTEKKeyRejects                                                           M          RO
docsBpi2CmTEKInvalids                                                             M          RO
docsBpi2CmTEKAuthPends                                                            M          RO
docsBpi2CmTEKKeyRejectErrorCode                                                   M          RO
docsBpi2CmTEKKeyRejectErrorString                                                 M          RO
docsBpi2CmTEKInvalidErrorCode                                                     M          RO
docsBpi2CmTEKInvalidErrorString                                                   M          RO
docsBpi2CmIpMulticastMapTable                                                     D          N-Acc
docsBpi2CmIpMulticastMapEntry                                                     D          N-Acc
docsBpi2CmIpMulticastIndex                                                        D          N-Acc
docsBpi2CmIpMulticastAddressType                                                  D          RO
docsBpi2CmIpMulticastAddress                                                      D          RO
docsBpi2CmIpMulticastSAId                                                         D          RO
docsBpi2CmIpMulticastSAMapState                                                   D          RO
docsBpi2CmIpMulticastSAMapRequests                                                D          RO
docsBpi2CmIpMulticastSAMapReplies                                                 D          RO
docsBpi2CmIpMulticastSAMapRejects                                                 D          RO
docsBpi2CmIpMulticastSAMapRejectErrorCode                                         D          RO
docsBpi2CmIpMulticastSAMapRejectErrorString                                       D          RO
docsBpi2CmDeviceCertTable                                                         M          N-Acc
docsBpi2CmDeviceCertEntry                                                         M          N-Acc
docsBpi2CmDeviceCmCert                                                            M          RW/RO
docsBpi2CmDeviceManufCert                                                         M          RO
docsBpi2CmCryptoSuiteTable                                                        M          N-Acc
docsBpi2CmCryptoSuiteEntry                                                        M          N-Acc
docsBpi2CmCryptoSuiteIndex                                                        M          N-Acc
docsBpi2CmCryptoSuiteDataEncryptAlg                                               M          RO
docsBpi2CmCryptoSuiteDataAuthentAlg                                               M          RO
docsBpi2CodeDownloadGroup
docsBpi2CodeDownloadStatusCode                                                    M          RO
docsBpi2CodeDownloadStatusString                                                  M          RO
docsBpi2CodeMfgOrgName                                                            M          RO
docsBpi2CodeMfgCodeAccessStart                                                    M          RO
docsBpi2CodeMfgCvcAccessStart                                                     M          RO
docsBpi2CodeCoSignerOrgName                                                       M          RO
docsBpi2CodeCoSignerCodeAccessStart                                               M          RO
docsBpi2CodeCoSignerCvcAccessStart                                                M          RO
docsBpi2CodeCvcUpdate                                                             M          RW
DOCS-IFEXT2-MIB
docsIfExt2CmMscStatusTable                                                        M          N-Acc
docsIfExt2CmMscStatusEntry                                                        M          N-Acc
docsIfExt2CmMscStatusState                                                        M          RO
docsIfExt2CmMscStatusPowerShortfall                                               M          RO
docsIfExt2CmMscStatusCodeRatio                                                    M          RO




132                                           CableLabs                                    04/29/26
<!-- PDF_PAGE: 133 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
docsIfExt2CmMscStatusMaximumScheduledCodes                                  M        RO
docsIfExt2CmMscStatusPowerHeadroom                                          M        RO
docsIfExt2CmMscStatusEffectivePower                                         M        RO
docsIfExt2CmMscStatusIUC2Control                                            M        RW
docsIfExt2CmClearLearnedMacAddresses                                        M        RW
HOST-RESOURCES-MIB [RFC 2790]
hrDeviceTable                                                               O        N-Acc
hrDeviceEntry                                                               O        N-Acc
hrDeviceIndex                                                               O        RO
hrDeviceType                                                                O        RO
hrDeviceDescr                                                               O        RO
hrDeviceID                                                                  O        RO
hrDeviceStatus                                                              O        RO
hrDeviceErrors                                                              O        RO
hrSystem
hrMemorySize                                                                O        RO
hrStorageTable                                                              O        N-Acc
hrStorageEntry                                                              O        N-Acc
hrStorageIndex                                                              O        RO
hrStorageType                                                               O        RO
hrStorageDescr                                                              O        RO
hrStorageAllocationUnits                                                    O        RO
hrStorageSize                                                               O        RO
hrStorageUsed                                                               O        RO
hrStorageAllocationFailures                                                 O        RO
hrSWRunTable                                                                O        N-Acc
hrSWRunEntry                                                                O        N-Acc
hrSWRunIndex                                                                O        RO
hrSWRunName                                                                 O        RO
hrSWRunID                                                                   O        RO
hrSWRunPath                                                                 O        RO
hrSWRunParameters                                                           O        RO
hrSWRunType                                                                 O        RO
hrSWRunStatus                                                               O        RO
hrSWRunPerfTable                                                            O        N-Acc
hrSWRunPerfEntry                                                            O        N-Acc
hrSWRunPerfCPU                                                              O        RO
hrSWRunPerfMem                                                              O        RO
hrProcessorTable                                                            O        N-Acc
hrProcessorEntry                                                            O        N-Acc
hrProcessorFrwID                                                            O        RO
hrProcessorLoad                                                             O        RO
ENTITY-MIB [RFC 6933]
entPhysicalTable                                                            O        N-Acc
entPhysicalEntry                                                            O        N-Acc
entPhysicalIndex                                                            O        RO




04/29/26                                     CableLabs                                   133
<!-- PDF_PAGE: 134 -->
CM-SP-CM-OSSIv4.0-I13-260429                         Data-Over-Cable Service Interface Specifications


Object                                                                                CM        Access
entPhysicalDescr                                                                  O             RO
entPhysicalVendorType                                                             O             RO
entPhysicalContainedIn                                                            O             RO
entPhysicalClass                                                                  O             RO
entPhysicalParentRelPos                                                           O             RO
entPhysicalName                                                                   O             RO
entPhysicalHardwareRev                                                            O             RO
entPhysicalFirmwareRev                                                            O             RO
entPhysicalSoftwareRev                                                            O             RO
entPhysicalSerialNum                                                              O             RO
entPhysicalMfgName                                                                O             RO
entPhysicalModelName                                                              O             RO
entPhysicalAlias                                                                  O             RO
entPhysicalAssetID                                                                O             RO
entPhysicalIsFRU                                                                  O             RO
entPhysicalMfgDate                                                                O             RO
entPhysicalUris                                                                   O             RO
entPhysicalUUID                                                                   O             RO
ENTITY-SENSOR-MIB [RFC 3433]
entPhySensorTable                                                                 O             N-Acc
entPhySensorEntry                                                                 O             N-Acc
entPhySensorType                                                                  O             RO
entPhySensorScale                                                                 O             RO
entPhySensorPrecision                                                             O             RO
entPhySensorValue                                                                 O             RO
entPhySensorOperStatus                                                            O             RO
entPhySensorUnitsDisplay                                                          O             RO
entPhySensorValueTimeStamp                                                        O             RO
entPhySensorValueUpdateRate                                                       O             RO


                                 CM in                                            CM in SNMP
                               NmAccess                                           Coexistence
                  Object         Mode     Access                Object               Mode        Access
SNMP-USM-DH-OBJECTS-MIB [RFC 2786]
usmDHParameters                N-Sup               usmDHParameters                M              RW
usmDHUserKeyTable              N-Sup               usmDHUserKeyTable              M              N-Acc
usmDHUserKeyEntry              N-Sup               usmDHUserKeyEntry              M              N-Acc
usmDHUserAuthKeyChange         N-Sup               usmDHUserAuthKeyChange         M              RC
usmDHUserOwnAuthKeyChange      N-Sup               usmDHUserOwnAuthKeyChange      M              RC
usmDHUserPrivKeyChange         N-Sup               usmDHUserPrivKeyChange         M              RC
usmDHUserOwnPrivKeyChange      N-Sup               usmDHUserOwnPrivKeyChange      M              RC
usmDHKickstartTable            N-Sup               usmDHKickstartTable            M              N-Acc
usmDHKickstartEntry            N-Sup               usmDHKickstartEntry            M              N-Acc
usmDHKickstartIndex            N-Sup               usmDHKickstartIndex            M              N-Acc
usmDHKickstartMyPublic         N-Sup               usmDHKickstartMyPublic         M              RO
usmDHKickstartMgrPublic        N-Sup               usmDHKickstartMgrPublic        M              RO
usmDHKickstartSecurityName     N-Sup               usmDHKickstartSecurityName     M              RO



134                                        CableLabs                                       04/29/26
<!-- PDF_PAGE: 135 -->
Cable Modem Operations Support System Interface Specification            CM-SP-CM-OSSIv4.0-I13-260429


                                   CM in                                              CM in SNMP
                                 NmAccess                                             Coexistence
              Object               Mode     Access                Object                 Mode       Access
SNMP-VIEW-BASED-ACM-MIB [RFC 3415]
vacmContextTable                 N-Sup               vacmContextTable                 M             N-Acc
vacmContextEntry                 N-Sup               vacmContextEntry                 M             N-Acc
vacmContextName                  N-Sup               vacmContextName                  M             RO
vacmSecurityToGroupTable         N-Sup               vacmSecurityToGroupTable         M             N-Acc
vacmSecurityToGroupEntry         N-Sup               vacmSecurityToGroupEntry         M             N-Acc
vacmSecurityModel                N-Sup               vacmSecurityModel                M             N-Acc
vacmSecurityName                 N-Sup               vacmSecurityName                 M             N-Acc
vacmGroupName                    N-Sup               vacmGroupName                    M             RC
vacmSecurityToGroupStorageType   N-Sup               vacmSecurityToGroupStorageType   M             RC
vacmSecurityToGroupStatus        N-Sup               vacmSecurityToGroupStatus        M             RC
vacmAccessTable                  N-Sup               vacmAccessTable                  M             N-Acc
vacmAccessEntry                  N-Sup               vacmAccessEntry                  M             N-Acc
vacmAccessContextPrefix          N-Sup               vacmAccessContextPrefix          M             N-Acc
vacmAccessSecurityModel          N-Sup               vacmAccessSecurityModel          M             N-Acc
vacmAccessSecurityLevel          N-Sup               vacmAccessSecurityLevel          M             N-Acc
vacmAccessContextMatch           N-Sup               vacmAccessContextMatch           M             RC
vacmAccessReadViewName           N-Sup               vacmAccessReadViewName           M             RC
vacmAccessWriteViewName          N-Sup               vacmAccessWriteViewName          M             RC
vacmAccessNotifyViewName         N-Sup               vacmAccessNotifyViewName         M             RC
vacmAccessStorageType            N-Sup               vacmAccessStorageType            M             RC
vacmAccessStatus                 N-Sup               vacmAccessStatus                 M             RC
vacmViewSpinLock                 N-Sup               vacmViewSpinLock                 M             RW
vacmViewTreeFamilyTable          N-Sup               vacmViewTreeFamilyTable          M             N-Acc
vacmViewTreeFamilyEntry          N-Sup               vacmViewTreeFamilyEntry          M             N-Acc
vacmViewTreeFamilyViewName       N-Sup               vacmViewTreeFamilyViewName       M             N-Acc
vacmViewTreeFamilySubtree        N-Sup               vacmViewTreeFamilySubtree        M             N-Acc
vacmViewTreeFamilyMask           N-Sup               vacmViewTreeFamilyMask           M             RC
vacmViewTreeFamilyType           N-Sup               vacmViewTreeFamilyType           M             RC
vacmViewTreeFamilyStorageType    N-Sup               vacmViewTreeFamilyStorageType    M             RC
vacmViewTreeFamilyStatus         N-Sup               vacmViewTreeFamilyStatus         M             RC
SNMP-COMMUNITY-MIB [RFC 3584]
snmpCommunityTable               N-Sup               snmpCommunityTable               M             N-Acc
snmpCommunityEntry               N-Sup               snmpCommunityEntry               M             N-Acc
snmpCommunityIndex               N-Sup               snmpCommunityIndex               M             N-Acc
snmpCommunityName                N-Sup               snmpCommunityName                M             RC
snmpCommunitySecurityName        N-Sup               snmpCommunitySecurityName        M             RC
snmpCommunityContextEngineID     N-Sup               snmpCommunityContextEngineID     M             RC
snmpCommunityContextName         N-Sup               snmpCommunityContextName         M             RC
snmpCommunityTransportTag        N-Sup               snmpCommunityTransportTag        M             RC
snmpCommunityStorageType         N-Sup               snmpCommunityStorageType         M             RC
snmpCommunityStatus              N-Sup               snmpCommunityStatus              M             RC
snmpTargetAddrExtTable           N-Sup               snmpTargetAddrExtTable           M             N-Acc
snmpTargetAddrExtEntry           N-Sup               snmpTargetAddrExtEntry           M             N-Acc




04/29/26                                     CableLabs                                                  135
<!-- PDF_PAGE: 136 -->
CM-SP-CM-OSSIv4.0-I13-260429                          Data-Over-Cable Service Interface Specifications


                                  CM in                                             CM in SNMP
                                NmAccess                                            Coexistence
                Object            Mode     Access                   Object             Mode       Access
snmpTargetAddrTMask             N-Sup               snmpTargetAddrTMask             M             RC
snmpTargetAddrMMS               N-Sup               snmpTargetAddrMMS               M             RC
snmpTrapAddress                 N-Sup               snmpTrapAddress                 O             ACC-FN
snmpTrapCommunity               N-Sup               snmpTrapCommunity               O             ACC-FN
SNMP-FRAMEWORK-MIB [RFC 3411]
snmpEngineGroup
snmpEngineID                    N-Sup               snmpEngineID                    M             RO
snmpEngineBoots                 N-Sup               snmpEngineBoots                 M             RO
snmpEngineTime                  N-Sup               snmpEngineTime                  M             RO
snmpEngineMaxMessageSize        N-Sup               snmpEngineMaxMessageSize        M             RO
SNMP-MPD-MIB [RFC 3412]
snmpMPDStats
snmpUnknownSecurityModels       N-Sup               snmpUnknownSecurityModels       M             RO
snmpInvalidMsgs                 N-Sup               snmpInvalidMsgs                 M             RO
snmpUnknownPDUHandlers          N-Sup               snmpUnknownPDUHandlers          M             RO
SNMP-TARGET-MIB [RFC 3413]
snmpTargetSpinLock              N-Sup               snmpTargetSpinLock              M             RW
snmpTargetAddrTable             N-Sup               snmpTargetAddrTable             M             N-Acc
snmpTargetAddrEntry             N-Sup               snmpTargetAddrEntry             M             N-Acc
snmpTargetAddrName              N-Sup               snmpTargetAddrName              M             N-Acc
snmpTargetAddrTDomain           N-Sup               snmpTargetAddrTDomain           M             RC
snmpTargetAddrTAddress          N-Sup               snmpTargetAddrTAddress          M             RC
snmpTargetAddrTimeout           N-Sup               snmpTargetAddrTimeout           M             RC
snmpTargetAddrRetryCount        N-Sup               snmpTargetAddrRetryCount        M             RC
snmpTargetAddrTagList           N-Sup               snmpTargetAddrTagList           M             RC
snmpTargetAddrParams            N-Sup               snmpTargetAddrParams            M             RC
snmpTargetAddrStorageType       N-Sup               snmpTargetAddrStorageType       M             RC
snmpTargetAddrRowStatus         N-Sup               snmpTargetAddrRowStatus         M             RC
snmpTargetParamsTable           N-Sup               snmpTargetParamsTable           M             N-Acc
snmpTargetParamsEntry           N-Sup               snmpTargetParamsEntry           M             N-Acc
snmpTargetParamsName            N-Sup               snmpTargetParamsName            M             N-Acc
snmpTargetParamsMPModel         N-Sup               snmpTargetParamsMPModel         M             RC
snmpTargetParamsSecurityModel   N-Sup               snmpTargetParamsSecurityModel   M             RC
snmpTargetParamsSecurityName    N-Sup               snmpTargetParamsSecurityName    M             RC
snmpTargetParamsSecurityLevel   N-Sup               snmpTargetParamsSecurityLevel   M             RC
snmpTargetParamsStorageType     N-Sup               snmpTargetParamsStorageType     M             RC
snmpTargetParamsRowStatus       N-Sup               snmpTargetParamsRowStatus       M             RC
snmpUnavailableContexts         N-Sup               snmpUnavailableContexts         M             RO
snmpUnknownContexts             N-Sup               snmpUnknownContexts             M             RO
snmpNotifyTable                 N-Sup               snmpNotifyTable                 M             N-Acc
snmpNotifyEntry                 N-Sup               snmpNotifyEntry                 M             N-Acc
snmpNotifyName                  N-Sup               snmpNotifyName                  M             N-Acc
snmpNotifyTag                   N-Sup               snmpNotifyTag                   M             RC
snmpNotifyType                  N-Sup               snmpNotifyType                  M             RC




136                                         CableLabs                                        04/29/26
<!-- PDF_PAGE: 137 -->
Cable Modem Operations Support System Interface Specification                 CM-SP-CM-OSSIv4.0-I13-260429


                                     CM in                                                CM in SNMP
                                   NmAccess                                               Coexistence
                Object               Mode     Access                   Object                Mode        Access
snmpNotifyStorageType              N-Sup               snmpNotifyStorageType              M              RC
snmpNotifyRowStatus                N-Sup               snmpNotifyRowStatus                M              RC
snmpNotifyFilterProfileTable       N-Sup               snmpNotifyFilterProfileTable       M              N-Acc
snmpNotifyFilterProfileEntry       N-Sup               snmpNotifyFilterProfileEntry       M              N-Acc
snmpNotifyFilterProfileName        N-Sup               snmpNotifyFilterProfileName        M              RC
snmpNotifyFilterProfileStorType    N-Sup               snmpNotifyFilterProfileStorType    M              RC
snmpNotifyFilterProfileRowStatus   N-Sup               snmpNotifyFilterProfileRowStatus   M              RC
snmpNotifyFilterTable              N-Sup               snmpNotifyFilterTable              M              N-Acc
snmpNotifyFilterEntry              N-Sup               snmpNotifyFilterEntry              M              N-Acc
snmpNotifyFilterSubtree            N-Sup               snmpNotifyFilterSubtree            M              N-Acc
snmpNotifyFilterMask               N-Sup               snmpNotifyFilterMask               M              RC
snmpNotifyFilterType               N-Sup               snmpNotifyFilterType               M              RC
snmpNotifyFilterStorageType        N-Sup               snmpNotifyFilterStorageType        M              RC
snmpNotifyFilterRowStatus          N-Sup               snmpNotifyFilterRowStatus          M              RC
SNMP-USER-BASED-SM-MIB [RFC 3414]
usmStats
usmStatsUnsupportedSecLevels       N-Sup               usmStatsUnsupportedSecLevels       M              RO
usmStatsNotInTimeWindows           N-Sup               usmStatsNotInTimeWindows           M              RO
usmStatsUnknownUserNames           N-Sup               usmStatsUnknownUserNames           M              RO
usmStatsUnknownEngineIDs           N-Sup               usmStatsUnknownEngineIDs           M              RO
usmStatsWrongDigests               N-Sup               usmStatsWrongDigests               M              RO
usmStatsDecryptionErrors           N-Sup               usmStatsDecryptionErrors           M              RO
usmUser
usmUserSpinLock                    N-Sup               usmUserSpinLock                    M              RW
usmUserTable                       N-Sup               usmUserTable                       M              N-Acc
usmUserEntry                       N-Sup               usmUserEntry                       M              N-Acc
usmUserEngineID                    N-Sup               usmUserEngineID                    M              N-Acc
usmUserName                        N-Sup               usmUserName                        M              N-Acc
usmUserSecurityName                N-Sup               usmUserSecurityName                M              RO
usmUserCloneFrom                   N-Sup               usmUserCloneFrom                   M              RC
usmUserAuthProtocol                N-Sup               usmUserAuthProtocol                M              RC
usmUserAuthKeyChange               N-Sup               usmUserAuthKeyChange               M              RC
usmUserOwnAuthKeyChange            N-Sup               usmUserOwnAuthKeyChange            M              RC
usmUserPrivProtocol                N-Sup               usmUserPrivProtocol                M              RC
usmUserPrivKeyChange               N-Sup               usmUserPrivKeyChange               M              RC
usmUserOwnPrivKeyChange            N-Sup               usmUserOwnPrivKeyChange            M              RC
usmUserPublic                      N-Sup               usmUserPublic                      M              RC
usmUserStorageType                 N-Sup               usmUserStorageType                 M              RC
usmUserStatus                      N-Sup               usmUserStatus                      M              RC


Object                                                                                        CM        Access
IGMP-STD-MIB [RFC 2933]
igmpInterfaceTable                                                                        D             N-Acc
igmpInterfaceEntry                                                                        D             N-Acc
igmpInterfaceIfIndex                                                                      D             N-Acc



04/29/26                                       CableLabs                                                     137
<!-- PDF_PAGE: 138 -->
CM-SP-CM-OSSIv4.0-I13-260429               Data-Over-Cable Service Interface Specifications


Object                                                                      CM     Access
igmpInterfaceQueryInterval                                              D          RC
igmpInterfaceStatus                                                     D          RC
igmpInterfaceVersion                                                    D          RO
igmpInterfaceQuerier                                                    D          RO
igmpInterfaceQueryMaxResponseTime                                       D          RC
igmpInterfaceQuerierUpTime                                              D          RO
igmpInterfaceQuerierExpiryTime                                          D          RO
igmpInterfaceVersion1QuerierTimer                                       D          RO
igmpInterfaceWrongVersionQueries                                        D          RO
igmpInterfaceJoins                                                      D          RO
igmpInterfaceProxyIfIndex                                               D          RO
igmpInterfaceGroups                                                     D          RO
igmpInterfaceRobustness                                                 D          RC
igmpInterfaceLastMembQueryIntvl                                         D          RC
igmpCacheTable                                                          D          N-Acc
igmpCacheEntry                                                          D          N-Acc
igmpCacheAddress                                                        D          N-Acc
igmpCacheIfIndex                                                        D          N-Acc
igmpCacheSelf                                                           D          RC
igmpCacheLastReporter                                                   D          RO
igmpCacheUpTime                                                         D          RO
igmpCacheExpiryTime                                                     D          RO
igmpCacheStatus                                                         D          RO
igmpCacheVersion1HostTimer                                              D          RO
[DOCS-QOS3-MIB]
docsQosPktClassTable                                                    M          N-Acc
docsQosPktClassEntry                                                    M          N-Acc
docsQosPktClassId                                                       M          N-Acc
docsQosPktClassDirection                                                M          RO
docsQosPktClassPriority                                                 M          RO
docsQosPktClassIpTosLow                                                 M          RO
docsQosPktClassIpTosHigh                                                M          RO
docsQosPktClassIpTosMask                                                M          RO
docsQosPktClassIpProtocol                                               M          RO
docsQosPktClassIpSourceAddr                                             M          RO
docsQosPktClassIpSourceMask                                             M          RO
docsQosPktClassIpDestAddr                                               M          RO
docsQosPktClassIpDestMask                                               M          RO
docsQosPktClassSourcePortStart                                          M          RO
docsQosPktClassSourcePortEnd                                            M          RO
docsQosPktClassDestPortStart                                            M          RO
docsQosPktClassDestPortEnd                                              M          RO
docsQosPktClassDestMacAddr                                              M          RO
docsQosPktClassDestMacMask                                              M          RO
docsQosPktClassSourceMacAddr                                            M          RO
docsQosPktClassEnetProtocolType                                         M          RO




138                                 CableLabs                                    04/29/26
<!-- PDF_PAGE: 139 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
docsQosPktClassEnetProtocol                                                 M        RO
docsQosPktClassUserPriLow                                                   M        RO
docsQosPktClassUserPriHigh                                                  M        RO
docsQosPktClassVlanId                                                       M        RO
docsQosPktClassState                                                        M        RO
docsQosPktClassPkts                                                         M        RO
docsQosPktClassBitMap                                                       M        RO
docsQosPktClassIpAddrType                                                   M        RO
docsQosPktClassFlowLabel                                                    M        RO
docsQosPktClassIcmpTypeHigh                                                 M        RO
docsQosPktClassIcmpTypeLow                                                  M        RO
docsQosPktClassCmInterfaceMask                                              M        RO
docsQosParamSetTable                                                        M        N-Acc
docsQosParamSetEntry                                                        M        N-Acc
docsQosParamSetServiceClassName                                             M        RO
docsQosParamSetPriority                                                     M        RO
docsQosParamSetMaxTrafficRate                                               M        RO
docsQosParamSetMaxTrafficBurst                                              M        RO
docsQosParamSetMinReservedRate                                              M        RO
docsQosParamSetMinReservedPkt                                               M        RO
docsQosParamSetActiveTimeout                                                M        RO
docsQosParamSetAdmittedTimeout                                              M        RO
docsQosParamSetMaxConcatBurst                                               M        RO
docsQosParamSetSchedulingType                                               M        RO
docsQosParamSetNomPollInterval                                              M        RO
docsQosParamSetTolPollJitter                                                M        RO
docsQosParamSetUnsolicitGrantSize                                           M        RO
docsQosParamSetNomGrantInterval                                             M        RO
docsQosParamSetTolGrantJitter                                               M        RO
docsQosParamSetGrantsPerInterval                                            M        RO
docsQosParamSetTosAndMask                                                   M        RO
docsQosParamSetTosOrMask                                                    M        RO
docsQosParamSetMaxLatency                                                   M        RO
docsQosParamSetType                                                         M        N-Acc
docsQosParamSetRequestPolicyOct                                             M        RO
docsQosParamSetBitMap                                                       M        RO
docsQosParamSetServiceFlowId                                                M        N-Acc
docsQosParamSetRequiredAttrMask                                             M        RO
docsQosParamSetForbiddenAttrMask                                            M        RO
docsQosParamSetAttrAggrRuleMask                                             M        RO
docsQosParamSetAppId                                                        M        RO
docsQosParamSetMultiplierContentionReqWindow                                M        RO
docsQosParamSetMultiplierBytesReq                                           M        RO
docsQosParamSetMaxReqPerSidCluster                                          D        RO
docsQosParamSetMaxOutstandingBytesPerSidCluster                             D        RO
docsQosParamSetMaxTotBytesReqPerSidCluster                                  D        RO




04/29/26                                          CableLabs                              139
<!-- PDF_PAGE: 140 -->
CM-SP-CM-OSSIv4.0-I13-260429                                Data-Over-Cable Service Interface Specifications


Object                                                                                       CM     Access
docsQosParamSetMaxTimeInSidCluster                                                       D          RO
docsQosParamSetPeakTrafficRate                                                           M          RO
docsQosParamSetDsResequencing                                                            M          RO
docsQosParamSetMinimumBuffer                                                             M          RO
docsQosParamSetTargetBuffer                                                              M          RO
docsQosParamSetMaximumBuffer                                                             M          RO
docsQosParamSetAqmDisabled                                                               M          RO
docsQosParamSetAqmLatencyTarget                                                          M          RO
docsQosParamSetHCMaxTrafficRate                                                          M          RO
docsQosParamSetHCMinReservedRate                                                         M          RO
docsQosParamSetHCPeakTrafficRate                                                         M          RO
docsQosParamSetAqmAlgInUse                                                               M          RO
docsQosParamSetGuaranteedGrantInterval                                                   M          RO
docsQosParamSetGuaranteedGrantRate                                                       M          RO
docsQosParamSetGuaranteedRequestInterval                                                 M          RO
docsQosParamSetImmedAqmMaxThreshold                                                      M          RO
docsQosParamSetImmedAqmRangeExponentRampFunc                                             M          RO
docsQosParamSetDataRateUnitSetting                                                       M          RO
docsQosServiceFlowTable                                                                  M          N-Acc
docsQosServiceFlowEntry                                                                  M          N-Acc
docsQosServiceFlowId                                                                     M          N-Acc
docsQosServiceFlowSID                                                                    M          RO
docsQosServiceFlowDirection                                                              M          RO
docsQosServiceFlowPrimary                                                                M          RO
docsQosServiceFlowParamSetTypeStatus                                                     M          RO
docsQosServiceFlowChSetId                                                                M          RO
docsQosServiceFlowAttrAssignSuccess                                                      M          RO
docsQosServiceFlowDsid                                                                   M          RO
docsQosServiceFlowMaxReqPerSidCluster                                                    M          RO
docsQosServiceFlowMaxOutstandingBytesPerSidCluster                                       M          RO
docsQosServiceFlowMaxTotBytesReqPerSidCluster                                            M          RO
docsQosServiceFlowMaxTimeInSidCluster                                                    M          RO
docsQosServiceFlowBufferSize                                                             M          RO
docsQosServiceFlowAggregateServiceFlowId                                                 M          RO
docsQosServiceFlowType                                                                   M          RO
docsQosServiceFlowAllowedAqBytes                                                         M          RO
docsQosServiceFlowStatsTable                                                             M          N-Acc
docsQosServiceFlowStatsEntry                                                             M          N-Acc
docsQosServiceFlowPkts                                                                   M          RO
docsQosServiceFlowOctets                                                                 M          RO
docsQosServiceFlowTimeCreated                                                            M          RO
docsQosServiceFlowTimeActive                                                             M          RO
docsQosServiceFlowPolicedDropPkts                                                        M          RO
docsQosServiceFlowPolicedDelayPkts                                                       M          RO
docsQosServiceFlowAqmDroppedPkts                                                         M          RO
docsQosDynamicServiceStatsTable                                                          M          N-Acc




140                                                  CableLabs                                    04/29/26
<!-- PDF_PAGE: 141 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                            CM    Access
docsQosDynamicServiceStatsEntry                                               M        N-Acc
docsQosIfDirection                                                            M        N-Acc
docsQosDSAReqs                                                                M        RO
docsQosDSARsps                                                                M        RO
docsQosDSAAcks                                                                M        RO
docsQosDSCReqs                                                                M        RO
docsQosDSCRsps                                                                M        RO
docsQosDSCAcks                                                                M        RO
docsQosDSDReqs                                                                M        RO
docsQosDSDRsps                                                                M        RO
docsQosDynamicAdds                                                            M        RO
docsQosDynamicAddFails                                                        M        RO
docsQosDynamicChanges                                                         M        RO
docsQosDynamicChangeFails                                                     M        RO
docsQosDynamicDeletes                                                         M        RO
docsQosDynamicDeleteFails                                                     M        RO
docsQosDCCReqs                                                                M        RO
docsQosDCCRsps                                                                M        RO
docsQosDCCAcks                                                                M        RO
docsQosDCCs                                                                   M        RO
docsQosDCCFails                                                               M        RO
docsQosDCCRspDeparts                                                          M        RO
docsQosDCCRspArrives                                                          M        RO
docsQosDbcReqs                                                                M        RO
docsQosDbcRsps                                                                M        RO
docsQosDbcAcks                                                                M        RO
docsQosDbcSuccesses                                                           M        RO
docsQosDbcFails                                                               M        RO
docsQosDbcPartial                                                             M        RO
docsQosServiceFlowSidClusterTable                                             M        N-Acc
docsQosServiceFlowSidClusterEntry                                             M        N-Acc
docsQosServiceFlowSidClusterId                                                M        N-Acc
docsQosServiceFlowSidClusterUcid                                              M        N-Acc
docsQosServiceFlowSidClusterSid                                               M        RO
docsQosAggregateServiceFlowTable                                              M        N-Acc
docsQosAggregateServiceFlowEntry                                              M        N-Acc
docsQosAggregateServiceFlowId                                                 M        N-Acc
docsQosAggregateServiceFlowDirection                                          M        RO
docsQosAggregateServiceFlowPriority                                           M        RO
docsQosAggregateServiceFlowMaxAggregateTrafficRate                            M        RO
docsQosAggregateServiceFlowMaxTrafficBurst                                    M        RO
docsQosAggregateServiceFlowMinReservedRate                                    M        RO
docsQosAggregateServiceFlowMinReservedPkt                                     M        RO
docsQosAggregateServiceFlowPeakTrafficRate                                    M        RO
docsQosAggregateServiceFlowDataRateUnitSetting                                M        RO
docsQosAggregateServiceFlowLowLatencyAsf                                      M        RO




04/29/26                                             CableLabs                             141
<!-- PDF_PAGE: 142 -->
CM-SP-CM-OSSIv4.0-I13-260429                            Data-Over-Cable Service Interface Specifications


Object                                                                                   CM     Access
docsQosAggregateServiceFlowLowLatencySfId                                            M          RO
docsQosAggregateServiceFlowClassicSfScn                                              M          RO
docsQosAggregateServiceFlowLowLatencySfScn                                           M          RO
docsQosAggregateServiceFlowAqmCouplingFactor                                         M          RO
docsQosAggregateServiceFlowSchedulingWeight                                          M          RO
docsQosAggregateServiceFlowQpEnable                                                  M          RO
docsQosAggregateServiceFlowQpLatencyThreshold                                        M          RO
docsQosAggregateServiceFlowQpQueuingScoreThreshold                                   M          RO
docsQosAggregateServiceFlowQpDrainRateExponent                                       M          RO
docsQosAggregateServiceFlowAsfQosProfileName                                         M          RO
docsQosAggregateServiceFlowStatsTable                                                M          N-Acc
docsQosAggregateServiceFlowStatsEntry                                                M          N-Acc
docsQosAggregateServiceFlowStatsPkts                                                 M          RO
docsQosAggregateServiceFlowStatsOctets                                               M          RO
docsQosAggregateServiceFlowStatsTimeCreated                                          M          RO
docsQosAggregateServiceFlowStatsTimeActive                                           M          RO
docsQosSfLatencyHistCfgTable                                                         M          N-Acc
docsQosSfLatencyHistCfgEntry                                                         M          N-Acc
docsQosSfLatencyHistCfgStatus                                                        M          RC
docsQosSfLatencySfLabel                                                              M          RC
docsQosSfLatencyBin1UpperEdge                                                        M          RC
docsQosSfLatencyBin2UpperEdge                                                        M          RC
docsQosSfLatencyBin3UpperEdge                                                        M          RC
docsQosSfLatencyBin4UpperEdge                                                        M          RC
docsQosSfLatencyBin5UpperEdge                                                        M          RC
docsQosSfLatencyBin6UpperEdge                                                        M          RC
docsQosSfLatencyBin7UpperEdge                                                        M          RC
docsQosSfLatencyBin8UpperEdge                                                        M          RC
docsQosSfLatencyBin9UpperEdge                                                        M          RC
docsQosSfLatencyBin10UpperEdge                                                       M          RC
docsQosSfLatencyBin11UpperEdge                                                       M          RC
docsQosSfLatencyBin12UpperEdge                                                       M          RC
docsQosSfLatencyBin13UpperEdge                                                       M          RC
docsQosSfLatencyBin14UpperEdge                                                       M          RC
docsQosSfLatencyBin15UpperEdge                                                       M          RC
docsQosSfLatencyStatsTable                                                           M          N-Acc
docsQosSfLatencyStatsEntry                                                           M          N-Acc
docsQosSfLatencyMaxLatency                                                           M          RO
docsQosSfLatencyNumHistUpdates                                                       M          RO
docsQosSfLatencyBin1Pkts                                                             M          RO
docsQosSfLatencyBin2Pkts                                                             M          RO
docsQosSfLatencyBin3Pkts                                                             M          RO
docsQosSfLatencyBin4Pkts                                                             M          RO
docsQosSfLatencyBin5Pkts                                                             M          RO
docsQosSfLatencyBin6Pkts                                                             M          RO
docsQosSfLatencyBin7Pkts                                                             M          RO




142                                              CableLabs                                    04/29/26
<!-- PDF_PAGE: 143 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
docsQosSfLatencyBin8Pkts                                                    M        RO
docsQosSfLatencyBin9Pkts                                                    M        RO
docsQosSfLatencyBin10Pkts                                                   M        RO
docsQosSfLatencyBin11Pkts                                                   M        RO
docsQosSfLatencyBin12Pkts                                                   M        RO
docsQosSfLatencyBin13Pkts                                                   M        RO
docsQosSfLatencyBin14Pkts                                                   M        RO
docsQosSfLatencyBin15Pkts                                                   M        RO
docsQosSfLatencyBin16Pkts                                                   M        RO
docsQosCmServiceUsStatsTable                                                M        N-Acc
docsQosCmServiceUsStatsEntry                                                M        N-Acc
docsQosCmServiceUsStatsTxSlotsImmed                                         M        RO
docsQosCmServiceUsStatsTxSlotsDed                                           M        RO
docsQosCmServiceUsStatsTxRetries                                            M        RO
docsQosCmServiceUsStatsTxExceededs                                          M        RO
docsQosCmServiceUsStatsRqRetries                                            M        RO
docsQosCmServiceUsStatsRqExceededs                                          M        RO
docsQosCmServiceUsStatsSgmts                                                M        RO
docsQosCmDsidTable                                                          M        N-Acc
docsQosCmDsidEntry                                                          M        N-Acc
docsQosCmDsidDsid                                                           M        N-Acc
docsQosCmDsidUsage                                                          M        RO
docsQosCmDsidNumReseqChs                                                    M        RO
docsQosCmDsidReseqChList                                                    M        RO
docsQosCmDsidReseqWaitTime                                                  M        RO
docsQosCmDsidReseqWarnThrshld                                               M        RO
docsQosCmDsidStatusHoldOffTimerSeqOutOfRng                                  M        RO
docsQosCmDsidOutOfRangeDiscards                                             M        RO
docsQosCmDsidNextExpectedSeqNum                                             M        RO
docsQosCmDsidCmInterfaceMask                                                M        RO
docsQosCmDsidFwdCmInterfaceMask                                             M        RO
docsQosCmDsidStatsTable                                                     M        N-Acc
docsQosCmDsidStatsEntry                                                     M        N-Acc
docsQosCmDsidStatsDsid                                                      M        N-Acc
docsQosCmDsidStatsSeqNumMissing                                             M        RO
docsQosCmDsidStatsSkewThreshExceeds                                         M        RO
docsQosCmDsidStatsOutOfRangePackets                                         M        RO
docsQosCmDsidStatsNumPackets                                                M        RO
docsQosCmDsidClientTable                                                    M        N-Acc
docsQosCmDsidClientEntry                                                    M        N-Acc
docsQosCmDsidClientDsid                                                     M        N-Acc
docsQosCmDsidClientClientMacId                                              M        N-Acc
docsQosCmDsidClientClientMacAddr                                            M        RO
docsQosCmSystemCfgStateAqmUsEnable                                          M        RO
docsQosCmSystemCfgStateDefaultUsTargetBuffer                                M        RO
docsQosSfCongestionStatsTable                                               M        N-Acc




04/29/26                                       CableLabs                                 143
<!-- PDF_PAGE: 144 -->
CM-SP-CM-OSSIv4.0-I13-260429                     Data-Over-Cable Service Interface Specifications


Object                                                                            CM     Access
docsQosSfCongestionStatsEntry                                                 M          N-Acc
docsQosSfCongestionSanctionedPkts                                             M          RO
docsQosSfCongestionTotalEct0Pkts                                              M          RO
docsQosSfCongestionTotalEct1Pkts                                              M          RO
docsQosSfCongestionCeMarkedEct1Pkts                                           M          RO
docsQosSfCongestionArrivedCePkts                                              M          RO
[DOCS-IF3-MIB]
docsIf3CmStatusTable                                                          M          N-Acc
docsIf3CmStatusEntry                                                          M          N-Acc
docsIf3CmStatusValue                                                          M          RO
docsIf3CmStatusCode                                                           M          RO
docsIf3CmStatusResets                                                         M          RO
docsIf3CmStatusLostSyncs                                                      M          RO
docsIf3CmStatusInvalidMaps                                                    M          RO
docsIf3CmStatusInvalidUcds                                                    M          RO
docsIf3CmStatusInvalidRangingRsps                                             M          RO
docsIf3CmStatusInvalidRegRsps                                                 M          RO
docsIf3CmStatusT1Timeouts                                                     M          RO
docsIf3CmStatusT2Timeouts                                                     M          RO
docsIf3CmStatusUCCsSuccesses                                                  D          RO
docsIf3CmStatusUCCFails                                                       D          RO
docsIf3CmStatusEnergyMgt1x1OperStatus                                         M          RO
docsIf3CmStatusUsTable                                                        M          N-Acc
docsIf3CmStatusUsEntry                                                        M          N-Acc
docsIf3CmStatusUsTxPower                                                      M          RO
docsIf3CmStatusUsT3Timeouts                                                   M          RO
docsIf3CmStatusUsT4Timeouts                                                   M          RO
docsIf3CmStatusUsRangingAborteds                                              M          RO
docsIf3CmStatusUsModulationType                                               M          RO
docsIf3CmStatusUsEqData                                                       M          RO
docsIf3CmStatusUsT3Exceededs                                                  M          RO
docsIf3CmStatusUsIsMuted                                                      M          RO
docsIf3CmStatusUsRangingStatus                                                M          RO
docsIf3CmCapabilities
docsIf3CmCapabilitiesReq                                                      M          RO
docsIf3CmCapabilitiesRsp                                                      M          RO
docsIf3RxChStatusTable                                                        M          N-Acc
docsIf3RxChStatusEntry                                                        M          N-Acc
docsIf3RxChStatusRcId                                                         M          N-Acc
docsIf3RxChStatusChIfIndex                                                    M          RO
docsIf3RxChStatusPrimaryDsIndicator                                           M          RO
docsIf3RxChStatusRcRmConnectivityId                                           M          RO
docsIf3RxModuleStatusTable                                                    M          N-Acc
docsIf3RxModuleStatusEntry                                                    M          N-Acc
docsIf3RxModuleStatusRmId                                                     M          N-Acc
docsIf3RxModuleStatusRmRmConnectivityId                                       M          RO




144                                       CableLabs                                    04/29/26
<!-- PDF_PAGE: 145 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                            CM    Access
docsIf3RxModuleStatusFirstCenterFrequency                                     M        RO
docsIf3SignalQualityExtTable                                                  M        N-Acc
docsIf3SignalQualityExtEntry                                                  M        N-Acc
docsIf3SignalQualityExtRxMER                                                  M        RO
docsIf3SignalQualityExtRxMerSamples                                           M        RO
docsIf3SignalQualityExtFbeNormalizationCoefficient                            M        RO
docsIf3UsChExtTable                                                           M        N-Acc
docsIf3UsChExtEntry                                                           M        N-Acc
docsIf3UsChExtSacCodeHoppingSelectionMode                                     M        RO
docsIf3UsChExtScdmaSelectionStringActiveCodes                                 O        RO
docsIf3CmDpvStatsTable                                                        M        N-Acc
docsIf3CmDpvStatsEntry                                                        M        N-Acc
docsIf3CmDpvStatsGrpId                                                        M        N-Acc
docsIf3CmDpvStatsLastMeasLatency                                              M        RO
docsIf3CmDpvStatsLastMeasTime                                                 M        RO
docsIf3CmDpvStatsMinLatency                                                   M        RO
docsIf3CmDpvStatsMaxLatency                                                   M        RO
docsIf3CmDpvStatsAvgLatency                                                   M        RO
docsIf3CmDpvStatsNumMeas                                                      M        RO
docsIf3CmDpvStatsLastClearTime                                                M        RO
docsIf3CmEventCtrlTable                                                       M        N-Acc
docsIf3CmEventCtrlEntry                                                       M        N-Acc
docsIf3CmEventCtrlEventId                                                     M        N-Acc
docsIf3CmEventCtrlStatus                                                      M        RC
docsIf3CmMdCfgTable                                                           M        N-Acc
docsIf3CmMdCfgEntry                                                           M        N-Acc
docsIf3CmMdCfgIpProvMode                                                      M        RW
docsIf3CmMdCfgIpProvModeResetOnChange                                         M        RW
docsIf3CmMdCfgIpProvModeResetOnChangeHoldOffTimer                             M        RW
docsIf3CmMdCfgIpProvModeStorageType                                           M        RW
docsIf3CmEnergyMgtCfg
docsIf3CmEnergyMgtCfgFeatureEnabled                                           M        RO
docsIf3CmEnergyMgtCfgCyclePeriod                                              M        RO
docsIf3CmEnergyMgt1x1CfgTable                                                 M        N-Acc
docsIf3CmEnergyMgt1x1CfgEntry                                                 M        N-Acc
docsIf3CmEnergyMgt1x1CfgDirection                                             M        N-Acc
docsIf3CmEnergyMgt1x1CfgEntryBitrateThrshld                                   M        RW
docsIf3CmEnergyMgt1x1CfgEntryTimeThrshld                                      M        RW
docsIf3CmEnergyMgt1x1CfgExitBitrateThrshld                                    M        RW
docsIf3CmEnergyMgt1x1CfgExitTimeThrshld                                       M        RW
docsIf3CmSpectrumAnalysisCtrlCmd
docsIf3CmSpectrumAnalysisCtrlCmdEnable                                        M        RW
docsIf3CmSpectrumAnalysisCtrlCmdInactivityTimeout                             M        RW
docsIf3CmSpectrumAnalysisCtrlCmdFirstSegmentCenterFrequency                   M        RO/RW
docsIf3CmSpectrumAnalysisCtrlCmdLastSegmentCenterFrequency                    M        RO/RW
docsIf3CmSpectrumAnalysisCtrlCmdSegmentFrequencySpan                          M        RO/RW




04/29/26                                             CableLabs                             145
<!-- PDF_PAGE: 146 -->
CM-SP-CM-OSSIv4.0-I13-260429                                Data-Over-Cable Service Interface Specifications


Object                                                                                       CM      Access
docsIf3CmSpectrumAnalysisCtrlCmdNumBinsPerSegment                                        M          RO/RW
docsIf3CmSpectrumAnalysisCtrlCmdEquivalentNoiseBandwidth                                 M          RO/RW
docsIf3CmSpectrumAnalysisCtrlCmdWindowFunction                                           M          RO/RW
docsIf3CmSpectrumAnalysisCtrlCmdNumberOfAverages                                         M          RO/RW
docsIf3CmSpectrumAnalysisMeasTable                                                       M          N-Acc
docsIf3CmSpectrumAnalysisMeasEntry                                                       M          N-Acc
docsIf3CmSpectrumAnalysisMeasFrequency                                                   M          N-Acc
docsIf3CmSpectrumAnalysisMeasAmplitudeData                                               M          RO
docsIf3CmSpectrumAnalysisMeasTotalSegmentPower                                           M          RO
docsIf3CmEm1x1StatsTable                                                                 M          N-Acc
docsIf3CmEm1x1StatsEntry                                                                 M          N-Acc
docsIf3CmEm1x1StatsNumberTimesCrossedBelowUsEntryThrshlds                                M          RO
docsIf3CmEm1x1StatsNumberTimesCrossedBelowDsEntryThrshlds                                M          RO
docsIf3CmEm1x1StatsTotalDuration                                                         M          RO
docsIf3CmEm1x1StatsTotalDurationBelowUsThrshlds                                          M          RO
docsIf3CmEm1x1StatsTotalDurationBelowDsThrshlds                                          M          RO
docsIf3CmEm1x1StatsTotalDurationBelowUsDsThrshlds                                        M          RO
Notifications
docsIf3CmEventNotif                                                                      M          Notif
[DOCS-IF31-MIB]
docsIf31DocsisBaseCapability                                                             M          RO
docsIf31RxChStatusTable                                                                  M          N-Acc
docsIf31RxChStatusEntry                                                                  M          N-Acc
docsIf31RxChStatusPrimaryDsIndicator                                                     M          RO
docsIf31RxChStatusOfdmProfiles                                                           M          RO
docsIf31CmDsOfdmChanTable                                                                M          N-Acc
docsIf31CmDsOfdmChanEntry                                                                M          N-Acc
docsIf31CmDsOfdmChanChannelId                                                            M          RO
docsIf31CmDsOfdmChanChanIndicator                                                        M          RO
docsIf31CmDsOfdmChanSubcarrierZeroFreq                                                   M          RO
docsIf31CmDsOfdmChanFirstActiveSubcarrierNum                                             M          RO
docsIf31CmDsOfdmChanLastActiveSubcarrierNum                                              M          RO
docsIf31CmDsOfdmChanNumActiveSubcarriers                                                 M          RO
docsIf31CmDsOfdmChanSubcarrierSpacing                                                    M          RO
docsIf31CmDsOfdmChanCyclicPrefix                                                         M          RO
docsIf31CmDsOfdmChanRollOffPeriod                                                        M          RO
docsIf31CmDsOfdmChanPlcFreq                                                              M          RO
docsIf31CmDsOfdmChanNumPilots                                                            M          RO
docsIf31CmDsOfdmChanTimeInterleaverDepth                                                 M          RO
docsIf31CmDsOfdmChanPlcTotalCodewords                                                    M          RO
docsIf31CmDsOfdmChanPlcUnreliableCodewords                                               M          RO
docsIf31CmDsOfdmChanNcpTotalFields                                                       M          RO
docsIf31CmDsOfdmChanNcpFieldCrcFailures                                                  M          RO
docsIf31CmDsOfdmProfileStatsTable                                                        M          N-Acc
docsIf31CmDsOfdmProfileStatsEntry                                                        M          N-Acc
docsIf31CmDsOfdmProfileStatsProfileId                                                    M          N-Acc




146                                                 CableLabs                                     04/29/26
<!-- PDF_PAGE: 147 -->
Cable Modem Operations Support System Interface Specification     CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                            CM    Access
docsIf31CmDsOfdmProfileStatsConfigChangeCt                                    M        RO
docsIf31CmDsOfdmProfileStatsTotalCodewords                                    M        RO
docsIf31CmDsOfdmProfileStatsCorrectedCodewords                                M        RO
docsIf31CmDsOfdmProfileStatsUncorrectableCodewords                            M        RO
docsIf31CmDsOfdmProfileStatsInOctets                                          M        RO
docsIf31CmDsOfdmProfileStatsInUnicastOctets                                   M        RO
docsIf31CmDsOfdmProfileStatsInMulticastOctets                                 M        RO
docsIf31CmDsOfdmProfileStatsInFrames                                          M        RO
docsIf31CmDsOfdmProfileStatsInUnicastFrames                                   M        RO
docsIf31CmDsOfdmProfileStatsInMulticastFrames                                 M        RO
docsIf31CmDsOfdmProfileStatsInFrameCrcFailures                                M        RO
docsIf31CmDsOfdmProfileStatsCtrDiscontinuityTime                              M        RO
docsIf31CmDsOfdmChannelPowerTable                                             M        N-Acc
docsIf31CmDsOfdmChannelPowerEntry                                             M        N-Acc
docsIf31CmDsOfdmChannelBandIndex                                              M        N-Acc
docsIf31CmDsOfdmChannelPowerCenterFrequency                                   M        RO
docsIf31CmDsOfdmChannelPowerRxPower                                           M        RO
docsIf31CmStatusOfdmaUsTable                                                  M        N-Acc
docsIf31CmStatusOfdmaUsEntry                                                  M        N-Acc
docsIf31CmStatusOfdmaUsT3Timeouts                                             M        RO
docsIf31CmStatusOfdmaUsT4Timeouts                                             M        RO
docsIf31CmStatusOfdmaUsRangingAborteds                                        M        RO
docsIf31CmStatusOfdmaUsT3Exceededs                                            M        RO
docsIf31CmStatusOfdmaUsIsMuted                                                M        RO
docsIf31CmStatusOfdmaUsRangingStatus                                          M        RO
docsIf31CmUsOfdmaChanTable                                                    M        N-Acc
docsIf31CmUsOfdmaChanEntry                                                    M        N-Acc
docsIf31CmStatusOfdmaChanConfigChangeCt                                       M        RO
docsIf31CmUsOfdmaChanSubcarrierZeroFreq                                       M        RO
docsIf31CmUsOfdmaChanFirstActiveSubcarrierNum                                 M        RO
docsIf31CmUsOfdmaChanLastActiveSubcarrierNum                                  M        RO
docsIf31CmUsOfdmaChanNumActiveSubcarriers                                     M        RO
docsIf31CmUsOfdmaChanSubcarrierSpacing                                        M        RO
docsIf31CmUsOfdmaChanCyclicPrefix                                             M        RO
docsIf31CmUsOfdmaChanRollOffPeriod                                            M        RO
docsIf31CmUsOfdmaChanNumSymbolsPerFrame                                       M        RO
docsIf31CmUsOfdmaChanTxPower                                                  M        RO
docsIf31CmUsOfdmaChanPreEqEnabled                                             M        RO
docsIf31CmUsOfdmaChanChannelId                                                M        RO
docsIf31CmUsOfdmaChanEstimatedRgDelay                                         M        RO
docsIf31CmUsScQamChanTable                                                    M        N-Acc
docsIf31CmUsScQamChanEntry                                                    M        N-Acc
docsIf31CmUsScQamChanTxPsd                                                    M        RO
docsIf31CmUsScQamChanEstimatedRgDelay                                         M        RO
docsIf31CmUsOfdmaProfileStatsTable                                            M        N-Acc
docsIf31CmUsOfdmaProfileStatsEntry                                            M        N-Acc




04/29/26                                             CableLabs                             147
<!-- PDF_PAGE: 148 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


Object                                                                                          CM     Access
docsIf31CmUsOfdmaProfileStatsIuc                                                            M          N-Acc
docsIf31CmUsOfdmaProfileStatsOutOctets                                                      M          RO
docsIf31CmUsOfdmaProfileStatsCtrDiscontinuityTime                                           M          RO
docsIf31CmUsOfdmaMinislotCfgStateTable                                                      M          N-Acc
docsIf31CmUsOfdmaMinislotCfgStateEntry                                                      M          N-Acc
docsIf31CmUsOfdmaMinislotCfgStateStartMinislotNum                                           M          N-Acc
docsIf31CmUsOfdmaMinislotCfgStateFirstSubcarrierId                                          M          RO
docsIf31CmUsOfdmaMinislotCfgStateNumConsecutiveMinislots                                    M          RO
docsIf31CmUsOfdmaMinislotCfgStateMinislotPilotPattern                                       M          RO
docsIf31CmUsOfdmaMinislotCfgStateDataSymbolModulation                                       M          RO
docsIf31CmEmDlsStatsTable                                                                   M          N-Acc
docsIf31CmEmDlsStatsEntry                                                                   M          N-Acc
docsIf31CmEmDlsStatsNumberTimesCrossedBelowUsEntryThrshlds                                  M          RO
docsIf31CmEmDlsStatsNumberTimesCrossedBelowDsEntryThrshlds                                  M          RO
docsIf31CmEmDlsStatsTotalDuration                                                           M          RO
docsIf31CmEmDlsStatsTotalDurationBelowUsThrshlds                                            M          RO
docsIf31CmEmDlsStatsTotalDurationBelowDsThrshlds                                            M          RO
docsIf31CmEmDlsStatsTotalDurationBelowUsDsThrshlds                                          M          RO
docsIf31CmEmDlsStatsNumSleepLatencyTriggers                                                 M          RO
docsIf31CmEmDlsStatsNumSleepByteCtTriggers                                                  M          RO
docsIf31CmEmDlsStatusTable                                                                  M          N-Acc
docsIf31CmEmDlsStatusEntry                                                                  M          N-Acc
docsIf31CmEmDlsStatusAssignedEmIds                                                          M          N-Acc
docsIf31CmEmDlsStatusReceiveTimer                                                           M          RO
docsIf31CmEmDlsStatusMaxSleepLatency                                                        M          RO
docsIf31CmEmDlsStatusMaxSleepBytes                                                          M          RO
docsIf31CmSystemCfgState                                                                    M          N-Acc
docsIf31CmSystemCfgStateDiplexerCapability                                                  M          RO
docsIf31CmSystemCfgStateDiplexerCfgBandEdge                                                 M          RO
docsIf31CmSystemCfgStateDiplexerDsLowerCapability                                           M          RO
docsIf31CmSystemCfgStateDiplexerCfgDsLowerBandEdge                                          M          RO
docsIf31CmSystemCfgStateDiplexerDsUpperCapability                                           M          RO
docsIf31CmSystemCfgStateDiplexerCfgDsUpperBandEdge                                          M          RO
docsIf31CmSystemCfgStateLgVqEwmaAlpha                                                       M          RO
docsIf31CmSystemCfgStateVqInterval                                                          M          RO
docsIf31CmStatusTable                                                                       M          N-Acc
docsIf31CmStatusEntry                                                                       M          N-Acc
docsIf31CmStatusEmDlsOperStatus                                                             M          RO
docsIf31CmEmDlsCfgTable                                                                     M          N-Acc
docsIf31CmEmDlsCfgEntry                                                                     M          N-Acc
docsIf31CmEmDlsCfgDirection                                                                 M          N-Acc
docsIf31CmEmDlsCfgEntryBitrateThrshld                                                       M          RW
docsIf31CmEmDlsCfgEntryTimeThrshld                                                          M          RW
docsIf31CmEmDlsCfgExitBitrateThrshld                                                        M          RW
docsIf31CmEmDlsCfgExitTimeThrshld                                                           M          RW




148                                                     CableLabs                                    04/29/26
<!-- PDF_PAGE: 149 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
[DOCS-PNM-MIB]
docsPnmBulkCtl Group
docsPnmBulkDestIpAddrType                                                   M        RW
docsPnmBulkDestIpAddr                                                       M        RW
docsPnmBulkDestPath                                                         M        RW
docsPnmBulkUploadControl                                                    M        RW
docsPnmBulkFileTable                                                        M        N-Acc
docsPnmBulkFileEntry                                                        M        N-Acc
docsPnmBulkFileIndex                                                        M        N-Acc
docsPnmBulkFileName                                                         M        RO
docsPnmBulkFileControl                                                      M        RW
docsPnmBulkFileUploadStatus                                                 M        RO
docsPnmCmControlObjects Group
docsPnmCmCtlTest                                                            M        RO
docsPnmCmCtlTestDuration                                                    M        RO
docsPnmCmCtlStatus                                                          M        RO
docsPnmCmDsOfdmSymCapTable                                                  M        N-Acc
docsPnmCmDsOfdmSymCapEntry                                                  M        N-Acc
docsPnmCmDsOfdmSymTrigEnable                                                M        RW
docsPnmCmDsOfdmSymTrigEnableTimeout                                         M        RW
docsPnmCmDsOfdmSymTrigGroupId                                               M        RW
docsPnmCmDsOfdmSymRxWindowing                                               M        RO
docsPnmCmDsOfdmSymTransactionId                                             M        RO
docsPnmCmDsOfdmSymSampleRate                                                M        RO
docsPnmCmDsOfdmSymFftLength                                                 M        RO
docsPnmCmDsOfdmSymMeasStatus                                                M        RO
docsPnmCmDsOfdmSymCaptFileName                                              M        RW
docsPnmCmOfdmChanEstCoefTable                                               M        N-Acc
docsPnmCmOfdmChanEstCoefEntry                                               M        N-Acc
docsPnmCmOfdmChEstCoefTrigEnable                                            M        RW
docsPnmCmOfdmChEstCoefAmpRipplePkToPk                                       M        RO
docsPnmCmOfdmChEstCoefAmpRippleRms                                          M        RO
docsPnmCmOfdmChEstCoefAmpSlope                                              M        RO
docsPnmCmOfdmChEstCoefGrpDelayRipplePkToPk                                  M        RO
docsPnmCmOfdmChEstCoefGrpDelayRippleRms                                     M        RO
docsPnmCmOfdmChEstCoefMeasStatus                                            M        RO
docsPnmCmOfdmChEstCoefFileName                                              M        RW
docsPnmCmOfdmChEstCoefAmpMean                                               M        RO
docsPnmCmOfdmChEstCoefGrpDelaySlope                                         M        RO
docsPnmCmOfdmChEstCoefGrpDelayMean                                          M        RO
docsPnmCmDsConstDispMeasTable                                               M        N-Acc
docsPnmCmDsConstDispMeasEntry                                               M        N-Acc
docsPnmCmDsConstDispTrigEnable                                              M        RW
docsPnmCmDsConstDispModOrderOffset                                          M        RW
docsPnmCmDsConstDispNumSampleSymb                                           M        RW
docsPnmCmDsConstDispSelModOrder                                             M        RO




04/29/26                                     CableLabs                                   149
<!-- PDF_PAGE: 150 -->
CM-SP-CM-OSSIv4.0-I13-260429                        Data-Over-Cable Service Interface Specifications


Object                                                                               CM     Access
docsPnmCmDsConstDispMeasStatus                                                   M          RO
docsPnmCmDsConstDispFileName                                                     M          RW
docsPnmCmDsOfdmRxMerTable                                                        M          N-Acc
docsPnmCmDsOfdmRxMerEntry                                                        M          N-Acc
docsPnmCmDsOfdmRxMerFileEnable                                                   M          RW
docsPnmCmDsOfdmRxMerPercentile                                                   M          RW
docsPnmCmDsOfdmRxMerMean                                                         M          RO
docsPnmCmDsOfdmRxMerStdDev                                                       M          RO
docsPnmCmDsOfdmRxMerThrVal                                                       M          RO
docsPnmCmDsOfdmRxMerThrHighestFreq                                               M          RO
docsPnmCmDsOfdmRxMerMeasStatus                                                   M          RO
docsPnmCmDsOfdmRxMerFileName                                                     M          RW
docsPnmCmDsOfdmMerMarTable                                                       M          N-Acc
docsPnmCmDsOfdmMerMarEntry                                                       M          N-Acc
docsPnmCmDsOfdmMerMarProfileId                                                   M          RW
docsPnmCmDsOfdmMerMarThrshldOffset                                               M          RW
docsPnmCmDsOfdmMerMarMeasEnable                                                  M          RW
docsPnmCmDsOfdmMerMarNumSymPerSubCarToAvg                                        M          RW
docsPnmCmDsOfdmMerMarReqAvgMer                                                   M          RW
docsPnmCmDsOfdmMerMarNumSubCarBelowThrshld                                       M          RO
docsPnmCmDsOfdmMerMarMeasuredAvgMer                                              M          RO
docsPnmCmDsOfdmMerMarAvgMerMargin                                                M          RO
docsPnmCmDsOfdmMerMarMeasStatus                                                  M          RO
docsPnmCmDsOfdmFecTable                                                          M          N-Acc
docsPnmCmDsOfdmFecEntry                                                          M          N-Acc
docsPnmCmDsOfdmFecSumType                                                        M          RW
docsPnmCmDsOfdmFecFileEnable                                                     M          RW
docsPnmCmDsOfdmFecMeasStatus                                                     M          RO
docsPnmCmDsOfdmFecFileName                                                       M          RW
docsPnmCmDsOfdmReqMERObjects
docsPnmCmDsOfdmReqMerQam16                                                       M          RW
docsPnmCmDsOfdmReqMerQam64                                                       M          RW
docsPnmCmDsOfdmReqMerQam128                                                      M          RW
docsPnmCmDsOfdmReqMerQam256                                                      M          RW
docsPnmCmDsOfdmReqMerQam512                                                      M          RW
docsPnmCmDsOfdmReqMerQam1024                                                     M          RW
docsPnmCmDsOfdmReqMerQam2048                                                     M          RW
docsPnmCmDsOfdmReqMerQam4096                                                     M          RW
docsPnmCmDsOfdmReqMerQam8192                                                     M          RW
docsPnmCmDsOfdmReqMerQam16384                                                    M          RW
docsPnmCmDsHistTable                                                             M          N-Acc
docsPnmCmDsHistEntry                                                             M          N-Acc
docsPnmCmDsHistEnable                                                            M          RW
docsPnmCmDsHistTimeOut                                                           M          RW
docsPnmCmDsHistMeasStatus                                                        M          RO
docsPnmCmDsHistFileName                                                          M          RW




150                                          CableLabs                                    04/29/26
<!-- PDF_PAGE: 151 -->
Cable Modem Operations Support System Interface Specification   CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                          CM    Access
docsPnmCmUsPreEqTable                                                       M        N-Acc
docsPnmCmUsPreEqEntry                                                       M        N-Acc
docsPnmCmUsPreEqFileEnable                                                  M        RW
docsPnmCmUsPreEqAmpRipplePkToPk                                             M        RO
docsPnmCmUsPreEqAmpRippleRms                                                M        RO
docsPnmCmUsPreEqAmpSlope                                                    M        RO
docsPnmCmUsPreEqGrpDelayRipplePkToPk                                        M        RO
docsPnmCmUsPreEqGrpDelayRippleRms                                           M        RO
docsPnmCmUsPreEqPreEqCoAdjStatus                                            M        RO
docsPnmCmUsPreEqMeasStatus                                                  M        RO
docsPnmCmUsPreEqLastUpdateFileName                                          M        RW
docsPnmCmUsPreEqFileName                                                    M        RW
docsPnmCmUsPreEqAmpMean                                                     M        RO
docsPnmCmUsPreEqGrpDelaySlope                                               M        RO
docsPnmCmUsPreEqGrpDelayMean                                                M        RO
docsPnmCmDsOfdmModProfTable                                                 M        N-Acc
docsPnmCmDsOfdmModProfEntry                                                 M        N-Acc
docsPnmCmDsOfdmModProfFileEnable                                            M        RW
docsPnmCmDsOfdmModProfMeasStatus                                            M        RO
docsPnmCmDsOfdmModProfFileName                                              M        RW
docsCmLatencyRptCfgSnapshotDuration                                         M        RW
docsCmLatencyRptCfgNumSnapshots                                             M        RW
docsCmLatencyRptCfgNumFiles                                                 M        RW
docsCmLatencyRptCfgMeasStatus                                               M        RO
docsCmLatencyRptCfgFileName                                                 M        RW
docsIf3CmSpectrumAnalysis
docsIf3CmSpectrumAnalysisCtrlCmdFileEnable                                  M        RW
docsIf3CmSpectrumAnalysisCtrlCmdMeasStatus                                  M        RO
docsIf3CmSpectrumAnalysisCtrlCmdFileName                                    M        RW
[DOCS-BPI2EXT-MIB]
docsBpi2Ext31CmDeviceCertTable                                              M        N-Acc
docsBpi2Ext31CmDeviceCertEntry                                              M        N-Acc
docsBpi2Ext31CmDeviceCmCert                                                 M        RW
docsBpi2Ext31CmDeviceManufCert                                              M        RO
docsBpi2Ext31CodeDownloadControl
docsBpi2Ext31CodeUpdateCvcChain                                             M        RW
docsBpi2Ext31CodeMfgOrgName                                                 M        RO
docsBpi2Ext31CodeMfgCodeAccessStart                                         M        RO
docsBpi2Ext31CodeMfgCvcAccessStart                                          M        RO
docsBpi2Ext31CodeCoSignerOrgName                                            M        RO
docsBpi2Ext31CodeCoSignerCodeAccessStart                                    M        RO
docsBpi2Ext31CodeCoSignerCvcAccessStart                                     M        RO
[DOCS-SEC-MIB]
docsSecCmSshServer
docsSecCmSshServerEnabledInterfaces                                         M        RW
docsSecCmSshServerStatus                                                    M        RO




04/29/26                                     CableLabs                                   151
<!-- PDF_PAGE: 152 -->
CM-SP-CM-OSSIv4.0-I13-260429                                 Data-Over-Cable Service Interface Specifications


Object                                                                                        CM     Access
docsSecCmSshServerPublicKey                                                               M          RO
docsSecCmSshServerNewConnectionTimeout                                                    M          RW
docsSecCmSshServerInactivityTimeout                                                       M          RW
docsSecCmSshServerSshSourceAddrRestrictionType                                            O          RW
docsSecCmSshServerSshSourceAddrRestriction                                                O          RW
docsSecCmSshServerSshSourcePrefixRestriction                                              O          RW
docsSecCmCdsFileServer
docsSecCmCdsFileServerIpAddrType                                                          M          RW
docsSecCmCdsFileServerIpAddr                                                              M          RW
docsSecCmCdsFileServerSshCmCdsDownloadUrl                                                 M          RW
docsSecCmCdsFileServerRevocationStatusAction                                              M          RW
docsSecCmPasswordCredentialTable                                                          M          N-Acc
docsSecCmPasswordCredentialEntry                                                          M          N-Acc
docsSecCmPasswordCredentialIndex                                                          M          N-Acc
docsSecCmPasswordCredentialUserId                                                         M          RC
docsSecCmPasswordCredentialPassword                                                       M          RC
docsSecCmPasswordCredentialMacAddr                                                        O          RC
docsSecCmPasswordCredentialRowStatus                                                      M          RC
docsSecCmPublicKeyCredentialTable                                                         M          N-Acc
docsSecCmPublicKeyCredentialEntry                                                         M          N-Acc
docsSecCmPublicKeyCredentialIndex                                                         M          N-Acc
docsSecCmPublicKeyCredentialSshPublicKey                                                  M          RC
docsSecCmPublicKeyCredentialMacAddr                                                       O          RC
docsSecCmPublicKeyCredentialRowStatus                                                     M          RC
docsSecCmIntermediateCaCert
docsSecCmIntermediateCaCertDocs30IntCaCert                                                M          RO
docsSecCmIntermediateCaCertIntCaCert                                                      M          RO
docsSecBpiPlusBaseTable                                                                   M          N-Acc
docsSecBpiPlusBaseEntry                                                                   M          N-Acc
docsSecBpiPlusBaseCmAuthStatusInfo                                                        M          RO
docsSecBpiPlusBaseCmBpiPlusVer                                                            M          RO
docsSecCmtsDesignationCfgTabl                                                             M          N-Acc
docsSecCmtsDesignationCfgEntry                                                            M          N-Acc
docsSecCmtsDesignationCfgIndex                                                            M          N-Acc
docsSecCmtsDesignationCfgCmtsDesignationType                                              M          RW
docsSecCmtsDesignationCfgCmtsDesignationAttr                                              M          RW
docsSecLearnedCmtsCert
docsSecLearnedCmtsCertDeviceId                                                            M          RW
docsSecLearnedCmtsCertDeviceCert                                                          M          RW
docsSecCmCertRevocationCfg
docsSecCmCertRevocationCfgMethod                                                          M          RW
docsSecCmOnlineCertStatusProtocolCfg
docsSecCmOnlineCertStatusProtocolCfgCache                                                 M          RW
docsSecCmOnlineCertStatusProtocolCfgRefreshInterval                                       M          RW
docsSecCmOnlineCertStatusProtocolCfgUrl                                                   M          RW




152                                                   CableLabs                                    04/29/26
