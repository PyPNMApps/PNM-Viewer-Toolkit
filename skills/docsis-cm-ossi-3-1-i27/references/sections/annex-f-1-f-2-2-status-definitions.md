# Annex F.1-F.2.2 - CM Status Reporting Overview and Data Type Definitions

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 213-216
Classification: Normative (explicit)

<!-- PDF_PAGE: 213 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


Annex F           CM Status Reporting Requirements (Normative)
F.1 Overview
This annex defines the operational status reporting requirements for a CM.

F.2 CM Operational Status Object Definitions
This section defines the CM configuration and status reporting objects.

F.2.1    Overview
This section defines the configuration and status reporting requirements for the CM. This information is contained in
the [MULPIv3.1] and [PHYv3.1] specifications.

F.2.2    Data Type Definitions
This section defines data types used in this information model.
                                         Table 90 - Data Type Definitions
                       Data Type Name        Base Type               Permitted Values
                     CmRegState             Enum            other(1)
                                                            notReady(2)
                                                            notSynchronized(3)
                                                            phySynchronized(4)
                                                            dsTopologyResolutionInProgress(21)
                                                            usParametersAcquired(5)
                                                            rangingInProgress(22)
                                                            rangingComplete(6)
                                                            eaeInProgress(14)
                                                            dhcpv4InProgress(15)
                                                            dhcpv6InProgress(16)
                                                            dhcpV4Complete(7)
                                                            dhcpV6Complete(17)
                                                            todEstablished(8)
                                                            securityEstablished(9)
                                                            configFileDownloadComplete(10)
                                                            registrationInProgress(18)
                                                            registrationComplete(11)
                                                            accessDenied(13)
                                                            operational(12)
                                                            bpiInit(19)
                                                            forwardingDisabled(20)
                                                            rfMuteAll(23)
                     DocsisVersion          Enum            other(0)
                                                            docsis10(1)
                                                            docsis11(2)
                                                            docsis20(3)
                                                            docsis30(4)
                                                            docsis31(5)
                     RangingState           Enum            other (1)
                                                            aborted(2)
                                                            retriesExceeded(3)
                                                            success(4)
                                                            continue(5)
                                                            timeoutT4(6)
                     Tlv8                   HexBinary




02/19/25                                           CableLabs®                                                   213
<!-- PDF_PAGE: 214 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


           CmRegState
This data type defines the CM connectivity state as reported by the CM.
References: [MULPIv3.1] Cable Modem - CMTS Interaction section
The enumerated values associated with the CmRegState are:
      •   other
              'other' indicates any state not described below.
      •   notReady
              'notReady' indicates that the CM has not started the registration process yet.
      •   notSynchronized
              'notSynchronized' indicates that the CM has not initiated or completed the synchronization of the
              downstream physical layer.
      •   phySynchronized
              'phySynchronized' indicates that the CM has completed the synchronization of the downstream
              physical layer.
      •   dsTopologyResolutionInProgress
              'dsTopologyResolutionInProgress' indicates that the CM is attempting to determine its MD-DS-SG.
      •   usParametersAcquired
              'usParametersAcquired' indicates that the CM has completed the upstream parameters acquisition or
              have completed the downstream and upstream service groups resolution, whether the CM is registering
              in a pre-3.0 or a 3.0 CMTS.
      •   rangingInProgress
              'rangingInProgress' indicates that the CM has initiated the initial ranging process.
      •   rangingComplete
              'rangingComplete' indicates that the CM has completed initial ranging and received a Ranging Status
              of success from the CMTS in the RNG-RSP message.
      •   eaeInProgress
              'eaeInProgress' indicates that the CM has sent an Auth Info message for EAE.
      •   dhcpv4InProgress
              'dhcpv4InProgress' indicates that the CM has sent a DHCPv4 DISCOVER to gain IP connectivity.
      •   dhcpv6InProgress
              'dhcpv6InProgress' indicates that the CM has sent an DHCPv6 Solicit message.
      •   dhcpv4Complete
              'dhcpv4Complete' indicates that the CM has received a DHCPv4 ACK message from the CMTS.
      •   dhcpv6Complete
              'dhcpv6Complete' indicates that the CM has received a DHCPv6 Reply message from the CMTS.
      •   todEstablished
              'todEstablished' indicates that the CM has successfully acquired time of day; if the ToD is acquired
              after the CM is operational, this value SHOULD NOT be reported.




214                                                  CableLabs®                                             02/19/25
<!-- PDF_PAGE: 215 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


    •   securityEstablished
            'securityEstablished' indicates that the CM has successfully completed the BPI initialization process.
    •   configFileDownloadComplete
            'configFileDownloadComplete' indicates that the CM has completed the config file download process.
    •   registrationInProgress
            'registrationInProgress' indicates that the CM has sent a Registration Request (REG-REQ or REG-
            REQ-MP)
    •   registrationComplete
            'registrationComplete' indicates that the CM has successfully completed the Registration process with
            the CMTS.
    •   accessDenied
            'accessDenied' indicates that the CM has received a registration aborted notification from the CMTS.
    •   operational
            'operational' indicates that the CM has completed all necessary initialization steps and is operational.
    •   bpiInit
            'bpiInit' indicates that the CM has started the BPI initialization process as indicated in the CM config
            file. If the CM already performed EAE, this state is skipped by the CM.
    •   forwardingDisabled
            'forwardingDisabled' indicates that the registration process was completed, but the network access
            option in the received configuration file prohibits forwarding.
    •   rfMuteAll
            'rfMuteAll' indicates that the CM is instructed to mute all channels in the CM-CTRL-REQ message
            from CMTS.
The following table provides a mapping of Pre-3.0 DOCSIS and DOCSIS 3.1 registration states as reported by CM.
                  Table 91 - Pre-3.0 DOCSIS and DOCSIS 3.0/3.1 CM Registration Status Mapping

                           CM Pre-3.0 DOCSIS                        CM DOCSIS 3.0 and 3.1
                       (from docsIfCmStatusValue)
                     other(1)                            other(1)
                     notReady(2)                         notReady(2)
                     notSynchronized(3)                  notSynchronized(3)
                     phySynchronized(4)                  phySynchronized(4)
                                                         dsTopologyResolutionInProgress(21)
                     usParametersAcquired(5)             usParametersAcquired(5)
                                                         rangingInProgress(22)
                     rangingComplete(6)                  rangingComplete(6)
                                                         eaeInProgress(14)
                                                         dhcpv4InProgress(15)
                                                         dhcpv6InProgress(16)
                     ipComplete(7)                       dhcpv4Complete(7)
                                                         dhcpv6Complete(17)
                     todEstablished(8)                   todEstablished(8)
                     securityEstablished(9)              securityEstablished(9)




02/19/25                                           CableLabs®                                                     215
<!-- PDF_PAGE: 216 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


                           CM Pre-3.0 DOCSIS                           CM DOCSIS 3.0 and 3.1
                       (from docsIfCmStatusValue)
                     paramTransferComplete(10)               configFileDownloadComplete(10)
                                                             registrationInProgress(18)
                     registrationComplete(11)                registrationComplete(11)
                     accessDenied(13)                        accessDenied(13)
                     operational(12)                         operational(12)
                                                             bpiInit (19)
                                                             forwardingDisabled(20)
                                                             rfMuteAll(23)
                     Note: DOCSIS 3.0 introduced new CM registration states which are given higher
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

           RangingState
This data type defines the ranging status of the Upstream Channel.
References: [MULPIv3.1] Cable Modem - CMTS Interaction section
The enumerated values associated with the RangingState are:
      •   Other
              'other' indicates any state not described below.
      •   Aborted
              'aborted' indicates that the CMTS has sent a ranging abort.
      •   retriesExceeded
              'retriesExceeded' indicates CM ranging retry limit has been exceeded.




216                                                    CableLabs®                                         02/19/25
