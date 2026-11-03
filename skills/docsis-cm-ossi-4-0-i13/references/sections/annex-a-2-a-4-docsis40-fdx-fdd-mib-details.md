# Annex A.2-A.4 - DOCSIS Interface 4.0, FDX, and FDD MIB-Object Details

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 153-154
Classification: Normative (explicit)

<!-- PDF_PAGE: 153 -->
Cable Modem Operations Support System Interface Specification             CM-SP-CM-OSSIv4.0-I13-260429


Object                                                                                            CM        Access
docsSecBpiPlusV2Cfg
docsSecBpiPlusV2CfgCmAuthKeyEcCurve                                                           M             RO
docsSecTrustOnFirstUseCfg
docsSecTrustOnFirstUseCfgAllowedBpiVers                                                       M             RW
docsSecCmtsTrustAnchorCert
docsSecCmtsTrustAnchorCertDeviceId                                                            M             RW
docsSecCmtsTrustAnchorCertCaCert                                                              M             RW
docsSecCmSccaServerCfg
docsSecCmSccaServerCfgIpAddrType                                                              M             RW
docsSecCmSccaServerCfgIpAddr                                                                  M             RW
docsSecCmSccaServerCfgRestApiUrl                                                              M             RW
docsSecCmSccaServerCfgRevocationStatusAction                                                  M             RW



A.2 DOCSIS Interface 4.0 MIB-Object Details
The following SNMP MIB objects were defined for the DOCSIS 4.0 CM to provide configuration and status of the
CM when initialized into a DOCSIS 4.0 mode but independent of the mode of operation such as FDD or FDX.
[DOCS-IF40-MIB]                                                                DOCSIS 4.0 CM Compliance
                                                                           FDXL      FDX           FDD
docsIf40CmCapabilities
docsIf40CmCapabilitiesAdvBandPlanCapability                               M        M              M          RO



A.3 DOCSIS FDX MIB-Object Details
The following SNMP MIB objects were defined for a DOCSIS 4.0 CM that supports FDX to provide configuration
and status of the CM when initialized into the FDX mode of operation.
[DOCS-FDX-MIB]                                                                 DOCSIS 4.0 CM Compliance
                                                                           FDXL         FDX           FDD
docsFdxCmFdxStatusTable                                                   M         M                        N-Acc
docsFdxCmFdxStatusEntry                                                   M         M                        N-Acc
docsFdxCmFdxStatusTransmissionGroupId                                     M         M                        RO
docsFdxCmFdxStatusCurrentRbaSubbandDirSet                                 M         M                        RO
docsFdxCmEcTrainingStatusTable                                                      M                        N-Acc
docsFdxCmEcTrainingStatusEntry                                                      M                        N-Acc
docsFdxCmEcTrainingStatusRbaSubbandDirSet                                           M                        N-Acc
docsFdxCmEcTrainingStatusEcTrainingStatus                                           M                        RO
docsFdxCmEcTrainingStatusEcTrainingPhase                                            M                        RO
docsFdxCmDsOfdmChannelTable                                                         M                        N-Acc
docsFdxCmDsOfdmChannelEntry                                                         M                        N-Acc
docsFdxCmDsOfdmChannelFdxEnabled                                                    M                        RO
docsFdxCmUsOfdmaChannelTable                                                        M                        N-Acc
docsFdxCmUsOfdmaChannelEntry                                                        M                        N-Acc
docsFdxCmUsOfdmaChannelFdxEnabled                                                   M                        RO
docsFdxCmUsOfdmaChannelExtendedUsChannelTxPower                                     M                        RO




04/29/26                                        CableLabs                                                       153
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
