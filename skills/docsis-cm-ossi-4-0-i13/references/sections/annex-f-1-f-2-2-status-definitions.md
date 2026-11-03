# Annex F.1-F.2.2 - CM Status Reporting Overview and Type Definitions

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 252-256
Classification: Normative (explicit)

<!-- PDF_PAGE: 252 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


Annex F           CM Status Reporting Requirements (Normative)
F.1 Overview
This Annex defines the operational status reporting requirements for a CM.

F.2 CM Operational Status Object Definitions
This section defines the CM configuration and status reporting objects.

F.2.1    Overview
This section defines the configuration and status reporting requirements for the CM. This information is contained in
the [MULPIv4.0] and [PHYv4.0] specifications.

F.2.2    Type Definitions
This section defines data types used in this information model.
                                         Table 123 - Data Type Definitions

                       Data Type Name         Base Type              Permitted Values
                     CmRegState              Enum           other(1),
                                                            notReady(2),
                                                            notSynchronized(3),
                                                            phySynchronized(4),
                                                            dsTopologyResolutionInProgress(21),
                                                            usParametersAcquired(5),
                                                            rangingInProgress(22),
                                                            rangingComplete(6),
                                                            eaeInProgress(14),
                                                            dhcpv4InProgress(15),
                                                            dhcpv6InProgress(16),
                                                            dhcpV4Complete(7),
                                                            dhcpV6Complete(17),
                                                            todEstablished(8),
                                                            securityEstablished(9),
                                                            configFileDownloadComplete(10),
                                                            registrationInProgress(18),
                                                            registrationComplete(11),
                                                            accessDenied(13),
                                                            operational(12),
                                                            bpiInit(19),
                                                            forwardingDisabled(20),
                                                            rfMuteAll(23)
                     DocsisVersion           Enum           other(0),
                                                            docsis10(1),
                                                            docsis11(2),
                                                            docsis20(3),
                                                            docsis30(4),
                                                            docsis31(5),
                                                            docsis40(6)




252                                                 CableLabs                                             04/29/26
<!-- PDF_PAGE: 253 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


                        Data Type Name        Base Type               Permitted Values
                     RangingState            Enum            other(1),
                                                             aborted(2),
                                                             retriesExceeded(3),
                                                             success(4),
                                                             continue(5),
                                                             timeoutT4(6)
                     RbaSubbandDirSetType    HexBinary       SIZE (1..3)
                     Tlv8                    HexBinary



           CmRegState
This data type defines the CM connectivity state as reported by the CM.
References: [MULPIv4.0] Cable Modem - CMTS Interaction section
The enumerated values associated with the CmRegState are:
    •   other
        'other' indicates any state not described below.
    •   notReady
        'notReady' indicates that the CM has not started the registration process yet.
    •   notSynchronized
        'notSynchronized' indicates that the CM has not initiated or completed the synchronization of the
        downstream physical layer.
    •   phySynchronized
        'phySynchronized' indicates that the CM has completed the synchronization of the downstream physical
        layer.
    •   dsTopologyResolutionInProgress
        'dsTopologyResolutionInProgress' indicates that the CM is attempting to determine its MD-DS-SG.
    •   usParametersAcquired
        'usParametersAcquired' indicates that the CM has completed the upstream parameters acquisition or have
        completed the downstream and upstream service groups resolution, whether the CM is registering in a pre-
        3.0 or a 3.0 CMTS.
    •   rangingInProgress
        'rangingInProgress' indicates that the CM has initiated the initial ranging process.
    •   rangingComplete
        'rangingComplete' indicates that the CM has completed initial ranging and received a Ranging Status of
        success from the CMTS in the RNG-RSP message.
    •   eaeInProgress
        'eaeInProgress' indicates that the CM has sent an Auth Info message for EAE.
    •   dhcpv4InProgress
        'dhcpv4InProgress' indicates that the CM has sent a DHCPv4 DISCOVER to gain IP connectivity.




04/29/26                                            CableLabs                                                   253
<!-- PDF_PAGE: 254 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


      •   dhcpv6InProgress
          'dhcpv6InProgress' indicates that the CM has sent an DHCPv6 Solicit message.
      •   dhcpv4Complete
          'dhcpv4Complete' indicates that the CM has received a DHCPv4 ACK message from the CMTS.
      •   dhcpv6Complete
          'dhcpv6Complete' indicates that the CM has received a DHCPv6 Reply message from the CMTS.
      •   todEstablished
          'todEstablished' indicates that the CM has successfully acquired time of day; if the ToD is acquired after the
          CM is operational, this value SHOULD NOT be reported.
      •   securityEstablished
          'securityEstablished' indicates that the CM has successfully completed the BPI initialization process.
      •   configFileDownloadComplete
          'configFileDownloadComplete' indicates that the CM has completed the config file download process.
      •   registrationInProgress
          'registrationInProgress' indicates that the CM has sent a Registration Request (REG-REQ or REG-REQ-
          MP)
      •   registrationComplete
          'registrationComplete' indicates that the CM has successfully completed the Registration process with the
          CMTS.
      •   accessDenied
          'accessDenied' indicates that the CM has received a registration aborted notification from the CMTS.
      •   operational
          'operational' indicates that the CM has completed all necessary initialization steps and is operational.
      •   bpiInit
          'bpiInit' indicates that the CM has started the BPI initialization process as indicated in the CM config file. If
          the CM already performed EAE, this state is skipped by the CM.
      •   forwardingDisabled
          'forwardingDisabled' indicates that the registration process was completed, but the network access option in
          the received configuration file prohibits forwarding.
      •   rfMuteAll
          'rfMuteAll' indicates that the CM is instructed to mute all channels in the CM-CTRL-REQ message from
          CMTS.
The following table provides a mapping of Pre-3.0 DOCSIS and DOCSIS 3.0, 3.1, and 4.0 registration states as
reported by CM.
               Table 124 - Pre-3.0 DOCSIS and DOCSIS 3.0/3.1/4.0 CM Registration Status Mapping

                            CM Pre-3.0 DOCSIS
                        (from docsIfCmStatusValue)                CM DOCSIS 3.0, 3.1 and 4.0
                      other(1)                              other(1)
                      notReady(2)                           notReady(2)
                      notSynchronized(3)                    notSynchronized(3)




254                                                   CableLabs                                                 04/29/26
<!-- PDF_PAGE: 255 -->
Cable Modem Operations Support System Interface Specification                         CM-SP-CM-OSSIv4.0-I13-260429


                         CM Pre-3.0 DOCSIS
                     (from docsIfCmStatusValue)                   CM DOCSIS 3.0, 3.1 and 4.0
                   phySynchronized(4)                      phySynchronized(4)
                                                           dsTopologyResolutionInProgress(21)
                   usParametersAcquired(5)                 usParametersAcquired(5)
                                                           rangingInProgress(22)
                   rangingComplete(6)                      rangingComplete(6)
                                                           eaeInProgress(14)
                                                           dhcpv4InProgress(15)
                                                           dhcpv6InProgress(16)
                   ipComplete(7)                           dhcpv4Complete(7)
                                                           dhcpv6Complete(17)
                   todEstablished(8)                       todEstablished(8)
                   securityEstablished(9)                  securityEstablished(9)
                   paramTransferComplete(10)               configFileDownloadComplete(10)
                                                           registrationInProgress(18)
                   registrationComplete(11)                registrationComplete(11)
                   accessDenied(13)                        accessDenied(13)
                   operational(12)                         operational(12)
                                                           bpiInit (19)
                                                           forwardingDisabled(20)
                                                           rfMuteAll(23)
                   NOTE: DOCSIS 3.0 introduced new CM registration states which are given higher
                   enumeration values even though they are intermediate CM registration states.



           DocsisVersion
This data type defines the DOCSIS capability of a device.
The enumerated values associated with the DocsisVersion are:
    •   other
        'other' indicates any state not described below.
    •   docsis10
        'docsis10' indicates DOCSIS 1.0.
    •   docsis11
        'docsis11' indicates DOCSIS 1.1.
    •   docsis20
        'docsis20' indicates DOCSIS 2.0.
    •   docsis30
        'docsis30' indicates DOCSIS 3.0.
    •   docsis31
        'docsis31' indicates DOCSIS 3.1.
    •   docsis40
        'docsis40' indicates DOCSIS 4.0.



04/29/26                                            CableLabs                                                255
<!-- PDF_PAGE: 256 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


           RbaSubbandDirSetType

This data type represents the encoding for RBA Sub-Band Direction Set, which is the set of all active FDX sub-
bands and the associated direction for those sub-bands. The length of the HexBinary value indicates the number of
sub-bands. The direction of each sub-band in the RBA Sub-Band Direction Set encoding is the same as in the RBA
message: 0 is downstream, 1 is upstream. A value of 2 is undefined and not applicable to FDX CMs. The left-most
byte represents sub-band id 0, which is at the lowest frequency.

The following explains the two example encodings.

      •   001: RBA Sub-Band Direction Set consisting of three sub-bands of sub-band 0 as downstream, sub-band 1
          as downstream, and sub-band 2 as upstream

      •   10: RBA Sub-Band Direction Set consisting of two sub-bands of sub-band 0 as upstream and sub-band 1 as
          downstream

References: [MULPIv4.0] Resource Block Assignment (RBA) section

           RangingState

This data type defines the ranging status of the Upstream Channel.

The enumerated values associated with the RangingState are:
      •   other
          'other' indicates any state not described below.
      •   aborted
          'aborted' indicates that the CMTS has sent a ranging abort.
      •   retriesExceeded
          'retriesExceeded' indicates CM ranging retry limit has been exceeded.
      •   success
          'success' indicates that the CMTS has sent a ranging success in the ranging response.
      •   continue
          'continue' indicates that the CMTS has sent a ranging continue in the ranging response.
      •   timeoutT4
          'timeoutT4' indicates that the T4 timer expired on the CM.

References: [MULPIv4.0] Ranging Response (RNG-RSP) section

           Tlv8

This data type represents a single TLV encoding. This first octet represents the Type of the TLV. The second octet
represents an unsigned 8-bit Length of the subsequent Value part of the TLV. The remaining octets represent the
value. The Value could be an atomic value or a sequence of one or more sub-TLVs.

References: [MULPIv4.0] Common TLV Encodings Annex, Modem Capabilities Encoding section




256                                                  CableLabs                                           04/29/26
