# 8.5 - Security Management

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 67-78
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 67 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


In the case where the state of an interface within the CM changes resulting in an "interface counter discontinuity"
[RFC 2863], the CM value of the ifXTable.ifXEntry.ifCounterDiscontinuityTime for the affected interface MUST
be set to the current value of sysUpTime and ALL counters for the affected interface set to ZERO. When setting the
ifAdminStatus of the affected interface to down(2), the CM MUST NOT consider this as an interface reset.
Case 2: SNMP Agent Reset.
An SNMP Agent Reset is defined as the reinitialization of the SNMP Agent software caused by a device reboot or
device reset initiated through SNMP.
In the case of an SNMP Agent Reset within the CM, the CM MUST:
      •    set the value of sysUpTime to zero (0)
      •    set all interface ifCounterDiscontinuityTime values to zero (0)
      •    set all interface counters to zero (0)
      •    set all other counters maintained by the CM SNMP Agent to zero (0).
Case 3: Counter Rollover.
When a Counter32 object within the CM reaches its maximum value of 4,294,967,295, the next value MUST be
ZERO. When a Counter64 object within the CM reaches its maximum value of 18,446,744,073,709,551,615, the
next value MUST be ZERO.
NOTE: Unless a CM vendor provides a means outside of SNMP to preset a Counter64 or Counter32 object to an
      arbitrary value, it will not be possible to test any rollover scenarios for Counter64 objects (and many
      Counter32 objects as well). This is because it is not possible for these counters to rollover during the service
      life of the device (see discussion in section 3.1.6 of [RFC 2863]).

8.5       Security Management
The CM is required to provide SNMP responses in accordance with the SNMP framework defined in [RFC 3411]
through [RFC 3416] and the guidelines defined in this section.

8.5.1      CM SNMP Modes of Operation
The CM MUST support SNMPv1, SNMPv2c, and SNMPv3 as well as SNMP-coexistence [RFC 3584] subject to
the requirements in the following sections.
The CM access control configuration supports SNMPv1v2c in NmAccess mode and SNMPv1v2c Coexistence mode
as described in [RFC 4639] and Section 8.5.2.7, respectively.

8.5.2      CM SNMP Access Control Configuration
The CM SNMP access control is configured via the CM config file and later updated for an authorized entity. The
confidentiality and authenticity of the information in the config file is defined in [MULPIv3.1] and [SECv3.1]. The
CM access control configuration supports SNMPv3 configuration through the Diffie-Hellman SNMP Kickstart
process defined in Section 8.5.2.6.

8.5.2.1     SNMP Operation Before CM Registration
IP connectivity between the CM and the SNMP management station MUST be implemented as described in Section
9.1.
The CM MUST provide read-only access to the following MIB objects prior to CM registration:
      •    docsIfDownChannelFrequency
      •    docsIfDownChannelPower
      •    docsIf3CmStatusValue




02/19/25                                             CableLabs®                                                    67
<!-- PDF_PAGE: 68 -->
CM-SP-CM-OSSIv3.1-I27-250219                                 Data-Over-Cable Service Interface Specifications


     •    docsDevEventTable
The CM MAY provide read-only access to the following MIB objects prior to CM registration:
     •    sysDescr
     •    sysUptime
     •    ifXTable
     •    docsIfUpChannelFrequency
     •    docsIfSignalQualityTable
     •    docsIfCmCmtsAddress
     •    docsIfCmStatusUsTxPower
     •    docsDevSwCurrentVers
The CM MUST NOT provide access to the following information prior to CM registration:
     •    QoS service flow information
     •    Configuration file contents
     •    Secure Software Download information
     •    Key authentication and encryption material
     •    SNMP management and control
     •    DOCSIS functional modules statistics and configuration
     •    Network provisioning hosts and servers IPs addresses
Additionally, prior to registration, the CM MUST adhere to the following requirements:
     •    The CM MAY provide access to additional information not listed in the statements above.
     •    The CM MUST NOT provide SNMP access from the RF interface prior to registration.
     •    The CM MUST accept any SNMPv1/v2c packets regardless of SNMP community string.
     •    The CM MUST drop all SNMPv3 packets.
The CM MUST NOT complete registration prior to successful processing of all MIB elements in the configuration
file.
The CM MUST complete registration prior to beginning calculation of the public values in the
usmDHKickstartTable.
If the CM configuration file contains SNMPv3 parameters, the CM MUST drop all SNMPv3 packets prior to
calculating the public values in the usmDHKickstartTable.

8.5.2.2    SNMP Operation After CM Registration
After registration, the CM can be in one of the following SNMP operation modes:
     •    SNMPv1/v2c NmAccess mode
     •    SNMP Coexistence mode
NOTE: OpenAccess mode available in pre-3.0 DOCSIS OSSI specifications is not supported in DOCSIS 3.1.
The CM MUST NOT provide SNMP access if the configuration file does not contain SNMP access control TLVs
such as docsDevNmAccessTable or SNMP coexistence TLV-11 or TLV-34, TLV-53 or TLV-54.
The SNMP mode of the CM is determined by the contents of the CM config file as follows:



68                                                 CableLabs®                                        02/19/25
<!-- PDF_PAGE: 69 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv3.1-I27-250219


    •     The CM is in SNMPv1/v2c NmAccess mode if the CM configuration file contains
          docsDevNmAccessTable settings for SNMP access control, does not contain SNMP coexistence TLV-11,
          TLV-34, TLV-38, TLV-53 or TLV-54 [MULPIv3.1].
    •     The CM is in SNMP coexistence mode if the CM configuration file contains snmpCommunityTable
          settings and/or TLV-34.1/34.2 and/or TLV-38. In this case, any entries made to the
          docsDevNmAccessTable are ignored.

8.5.2.3    SNMP NmAccessMode
SNMPv1/v2c NmAccess Mode (using docsDevNmAccessTable)
    •     The CM MUST implement docsDevNmAccessTable which controls access and trap destinations as
          described in [RFC 4639] for backward compatibility with pre-3.0 DOCSIS.
    •     The CM MUST process SNMPv1/v2c packets only in NmAccess mode and drop all SNMPv3 packets.
    •     The CM MUST NOT allow access to SNMPv3 MIBs as defined in [RFC 3411] through [RFC 3415] and
          [RFC 3584] while in NmAccess mode.

8.5.2.4    SNMP Coexistence Mode
The CM MUST process SNMPv1/v2c/v3 messages for SNMP Access Control and SNMP notifications as described
by [RFC 3411] through [RFC 3415] and [RFC 3584] as follows:
    •     The SNMP-COMMUNITY-MIB controls the translation of SNMPv1/v2c packet community string into
          security name which select entries in the SNMP-USER-BASED-SM-MIB. Access control is provided by
          the SNMP-VIEW-BASED-ACM-MIB.
    •     SNMP-USER-BASED-SM-MIB and SNMP-VIEW-BASED-ACM-MIB controls SNMPv3 packets.
    •     Notification destinations are specified in the SNMP-TARGET-MIB, SNMP-NOTIFICATION-MIB and
          SNMP-VIEW-BASED-ACM-MIB.
    •     The CM MUST NOT provide access to docsDevNmAccessTable.
    •     When the CM is configured for SNMPv3, the CM MUST NOT allow SNMP access from the RF port
          during calculation of usmDHKickstartTable public value.
    •     When the CM is configured for SNMPv3, the CM MAY continue to allow access from the CPE port with
          the limited access as configured by the SNMP-COMMUNITY-MIB, SNMP-TARGET-MIB, SNMP-
          VIEW-BASED-ACM-MIB, and SNMP-USER-BASED-SM-MIB during calculation of
          usmDHKickstartTable public value.

8.5.2.5    SNMPv3 Initialization and Key Changes
Note that the SNMPv3 Initialization and Key Change process defined below is based on [RFC 2786], which always
configures the SNMP agent with SNMPv3 HMAC-MD5-96 as the authentication protocol and CBC-DES as the
privacy protocol, both specified in [RFC 3414]. Therefore, this specification does not provide a mechanism to
initialize SNMPv3 using CFB128-AES-128 for privacy key, as defined in [RFC 3826] or any other configuration
defined in [RFC 3414] and are left out of scope of this specification.
The DOCSIS 3.1 CM is designated as having a "very-secure" security posture in the context of [RFC 3414] and
[RFC 3415], which means that default usmUserTable and VACM tables entries defined in Appendix A of
[RFC 3414] and Appendix A of [RFC 3415] MUST NOT be present. The major implication for the CM is that only
the config file can be used to provide the initial SNMPv3 security configuration.
[RFC 2786] provides a mechanism to kick start an SNMPv3 agent User-based Security Model [RFC 3414] and
extensions to the same model for key change. [RFC 2786] does not define the mechanism to configure the initial key
material for the kick start process. This specification defines the configuration requirements to initialize the
SNMPv3 KickStart initialization defined in [RFC 2786] to configure SNMPv3 for the CM.




02/19/25                                          CableLabs®                                                   69
<!-- PDF_PAGE: 70 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


The CM MUST support the config file TLV-34 as defined in [MULPIv3.1] to configure the initial key material
(KickStart Security Name and KickStart Public Number) used for the SNMPv3 agent initialization.
The TLV-34.1 KickStart Security Name corresponds to the SNMPv3 userName [RFC 3414] to be initialized in the
CM.
The TLV-34.2 KickStart Public Number is a Diffie-Helman public number generated as described in the description
of usmDHKickstartMgrPublic MIB object of [RFC 2786].
The CM MUST support a minimum of 5 entries of TLV-34 in the config file.
The CM MUST provide, by default, pre-defined entries in the USM table and VACM tables to correctly create the
userName 'dhKickstart' with security level 'noAuthNoPriv' that has read-only access to system group and
usmDHKickstartTable of [RFC 2786].
The CM MUST provide access to TLV-34 [MULPIv3.1] and dhKickstart defined userNames in usmUserTable as
follows:
     •   Access as specified in the config file or the default access if corresponding to usernames defined above
     •   StorageType is 'permanent'
     •   Prohibit entry deletion
     •   Entries do not persist across MAC initialization

8.5.2.5.1     SNMPv3 Initialization
For each of up to five different TLV-34 (KickStart Security Name, KickStart Public Number) [MULPIv3.1] pairs
from the configuration file, the CM MUST populate in the usmDHKickstartTable the MIB objects
usmDHKickstartSecurityName and usmDHKickstartMgrPublic (each pair as an entry).
When a usmDHKickstartMgrPublic instance is set with a valid value during the initialization, the CM MUST create
a corresponding row in the usmUserTable as defined in the clause description of usmDHKickstartMgrPublic MIB
object of [RFC 2786].
After the CM has registered with the CMTS:
     •   The CM MUST populate the usmDHKickstartMyPublic MIB object of the usmDHKickstartTable as
         defined in [RFC 2786] for each entry that a non-zero length usmDHKickstartSecurityName and
         usmDHKickstartMgrPublic.
     •   [RFC 2786] Textual Convention DHKeyChange defines the mechanism to determine the Diffie-Helman
         shared secret for the CM and the SNMP manager. With the Diffie-Helman shared secret, the CM and other
         entities can derive the SNMPv3 privacy and authentication keys for the corresponding USM userName.
     •   The CM MUST derive the USM userName security and authentication keys as described in the description
         clause of the usmDHKickstartMgrPublic MIB object of [RFC 2786].
At this point the CM has completed its SNMPv3 initialization process.
After SNMPv3 initialization process has been finished, the CM MUST allow appropriate access level to a valid
securityName with the correct authentication key and/or privacy key.
The CM MUST properly populate keys to appropriate tables as specified by the SNMPv3-related RFCs and
[RFC 2786].
The following describes the process that the manager uses to derive the CM's unique authentication key and privacy
key:
     •   The SNMP manager accesses the contents of the usmDHKickstartTable using the security name of
         'dhKickstart' with no authentication.
     •   The SNMP manager gets the value of the CM's usmDHKickstartMyPublic number associated with the
         securityName for which the manager wants to derive authentication and privacy keys.



70                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 71 -->
Cable Modem Operations Support System Interface Specification                 CM-SP-CM-OSSIv3.1-I27-250219


    •     Using the private random number, the manager can calculate the DH shared secret. From that shared secret,
          the manager can derive operational authentication and confidentiality keys for the securityName that the
          manager is going to use to communicate with the CM.

8.5.2.5.2      SNMPv3 Initialization Failure
In case of failure to complete SNMPv3 initialization (i.e., NMS cannot access CM via SNMPv3 PDU), the CM is in
the SNMP Coexistence mode and will allow SNMPv1/v2c access if and only if the SNMP-COMMUNITY-MIB
entries (and related entries) are configured.

8.5.2.5.3      DH Key Changes
The CMs MUST support the key-change mechanism specified in the textual convention DHKeyChange of
[RFC 2786].

8.5.2.6     View-Based Access Control Model (VACM) Profile
This section addresses the default VACM profile for DOCSIS CMs operating in SNMP Coexistence mode.
The CM MUST support pre-installed entries in VACM tables of [RFC 3415] as follows:
    •     The system manager, with full read/write/config access:
          vacmSecurityModel: 3 (USM)
          vacmSecurityName: docsisManager
          vacmGroupName: docsisManager
          vacmSecurityToGroupStorageType: permanent
          vacmSecurityToGroupStatus: active
    •     An operator/CSR with read/reset access to full modem:
          vacmSecurityModel: 3 (USM)
          vacmSecurityName: docsisOperator
          vacmGroupName: docsisOperator
          vacmSecurityToGroupStorageType: permanent
          vacmSecurityToGroupStatus: active
    •     RF Monitoring with read access to RF plant statistics:
          vacmSecurityModel: 3 (USM)
          vacmSecurityName: docsisMonitor
          vacmGroupName: docsisMonitor
          vacmSecurityToGroupStorageType: permanent
          vacmSecurityToGroupStatus: active
    •     User debugging with read access to 'useful' variables:
          vacmSecurityModel: 3 (USM)
          vacmSecurityName: docsisUser
          vacmGroupName: docsisUser
          vacmSecurityToGroupStorageType: permanent
          vacmSecurityToGroupStatus: active




02/19/25                                            CableLabs®                                                  71
<!-- PDF_PAGE: 72 -->
CM-SP-CM-OSSIv3.1-I27-250219                            Data-Over-Cable Service Interface Specifications


     •   Group name to view translations
         vacmGroupName: docsisManager
         vacmAccessContextPrefix: ''
         vacmAccessSecurityModel: 3 (USM)
         vacmAccessSecurityLevel: AuthPriv
         vacmAccessContextMatch: exact
         vacmAccessReadViewName: docsisManagerView
         vacmAccessWriteViewName: docsisManagerView
         vacmAccessNotifyViewName: docsisManagerView
         vacmAccessStorageType: permanent
         vacmAccessStatus: active
         vacmGroupName: docsisOperator
         vacmAccessContextPrefix: ''
         vacmAccessSecurityModel: 3 (USM)
         vacmAccessSecurityLevel: AuthNoPriv and AuthPriv
         vacmAccessContextMatch: exact
         vacmAccessReadViewName: docsisManagerView
         vacmAccessWriteViewName: docsisOperatorWriteView
         vacmAccessNotifyViewName: docsisManagerView
         vacmAccessStorageType: permanent
         vacmAccessStatus: active
         vacmGroupName: docsisMonitor
         vacmAccessContextPrefix: ''
         vacmAccessSecurityModel: 3 (USM)
         vacmAccessSecurityLevel: AuthNoPriv and AuthPriv
         vacmAccessContextMatch: exact
         vacmAccessReadViewName: docsisMonitorView
         vacmAccessWriteViewName: ''
         vacmAccessNotifyViewName: docsisMonitorView
         vacmAccessStorageType: permanent
         vacmAccessStatus: active
         vacmGroupName: docsisUser
         vacmAccessContextPrefix: ''
         vacmAccessSecurityModel: 3 (USM)
         vacmAccessSecurityLevel: AuthNoPriv and AuthPriv
         vacmAccessContextMatch: exact
         vacmAccessReadViewName: docsisUserView



72                                            CableLabs®                                       02/19/25
<!-- PDF_PAGE: 73 -->
Cable Modem Operations Support System Interface Specification              CM-SP-CM-OSSIv3.1-I27-250219


          vacmAccessWriteViewName: ''
          vacmAccessNotifyViewName: ''
          vacmAccessStorageType: permanent
          vacmAccessStatus: active
The CM includes, by default, the following views referred from the VACM entries above:
    •     docsisManagerView
          subtree: 1.3.6.1 (internet or entire MIB)
    •     docsisOperatorWriteView
          subtree: docsDevBase
          subtree: docsDevSoftware
          object: docsDevEvControl
          object: docsDevEvThrottleAdminStatus
    •     docsisMonitorView
          subtree: 1.3.6.1.2.1.1 (system)
          subtree: docsIfBaseObjects
          subtree: docsIfCmObjects
          docsisUserView.3.6.1.2.1.1 (system)
          subtree: docsDevBase
          object: docsDevSwOperStatus
          object: docsDevSwCurrentVers
          object:docsDevServerConfigFile
          subtree: docsDevEventTable
          subtree: docsDevCpeInetTable
          subtree: docsIfUpstreamChannelTable
          subtree: docsIfDownstreamChannelTable
          subtree: docsIfSignalQualityTable
    •     subtree
          subtree: 1
          docsIfCmStatusTable
The CM MUST also support additional VACM users as they are configured via an SNMP-embedded configuration
file.

8.5.2.7    SNMPv1v2c Coexistence Configuration Config File TLV
This section specifies CM processing requirements for the SNMPv1v2c Coexistence Configuration TLV
[MULPIv3.1] when present in the configuration file. The SNMPv1v2c Coexistence Configuration TLV is used to
configure SNMPv3 tables for SNMPv1 and v2c access. The CM MUST process SNMPv1v2c Coexistence
Configuration TLV in conjunction with SNMP TLV-11 containing SNMPv3 tables, TLV-38, as well as SNMPv3
Access View Configuration TLV (see Section 8.5.2.8).




02/19/25                                              CableLabs®                                             73
<!-- PDF_PAGE: 74 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


Based on the SNMPv1v2c Coexistence Configuration TLV, the CM MUST create entries in the following tables in
order to cause the desired SNMP Access:
     •   snmpCommunityTable
     •   snmpTargetAddrTable
     •   vacmSecurityToGroupTable
     •   vacmAccessTable
The mapping from the TLV to these tables is described in the following section.

8.5.2.7.1     Mapping of TLV Fields into SNMPv3 Tables
The following section describes the mapping of SNMPv1v2c Coexistence Configuration TLV into SNMPv3 entries:
                        Table 31 - SNMPv1v2c Coexistence Configuration TLV Mapping

                  Sub-TLVs                          Variable Name                  Associated MIB Object
SNMPv1v2c Community Name                        CommunityName            snmpCommunityName [RFC 3584]
SNMPv1v2c Transport Address Access
     SNMPv1v2c Transport Address                TAddress                 snmpTargetAddrTAddress [RFC 3413]
     SNMPv1v2c Transport Address Mask           TMask                    snmpTargetAddrTMask [RFC 3584]
SNMPv1v2c Access View Type                      AccessViewType
SNMPv1v2c Access View Name                      AccessViewName           vacmAccessReadViewName and
                                                                         vacmAccessWriteViewName [RFC 3415]


The CM is not required to verify the consistency of linkage of tables unless specified. It is intended that the SNMP
agent will handle the corresponding configuration problems as part of the normal SNMP incoming requests (e.g.,
generating internal abstract data elements like noSuchView [RFC 3415]).
Table 33 through Table 38 describe the CM procedures to populate the SNMP Management Framework Message
Processing and Access Control Subsystems [RFC 3412].
In configuring entries in these SNMPv3 tables, note the following:
     •   The ReadViewName and WriteViewName may correspond to default entries as defined in Section
         8.5.2.5.2, individual entries defined by TLV-11 or entries created using SNMPv3 Access View
         Configuration (see Section 8.5.2.8).
     •   Several columnar objects are configured with indexes with the string "@CMconfig". If these tables are
         configured through other mechanisms, Network operators should not use values beginning with
         "@CMconfig" to avoid conflicts with the mapping process specified here.

8.5.2.7.2     snmpCommunityTable
The snmpCommunityTable is defined in the "SNMP Community MIB Module" section of [RFC 3584].
The CM MUST create one row in snmpCommunityTable for each SNMPv1v2c Coexistence Configuration TLV in
the config file as follows:
     •   The CM MUST set in snmpCommunityIndex the keyword @CMconfig_n where 'n' is a sequential number
         starting at 0 for each TLV processed (e.g., "@CMconfig_0", "@CMconfig_1", etc.)
     •   The CM MUST create space separated tags in snmpCommunityTransportTag for each SNMPv1v2c
         Community Name sub-TLV of the SNMPv1v2c Coexistence Configuration TLV in the config file.




74                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 75 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


                                       Table 32 - snmpCommunityTable

   Column Name (* = Part of Index)                                     Column Value
 * snmpCommunityIndex                   "@CMconfig_n" where n is 0..m-1 and m is the number of SNMPv1v2c Community
                                        Name config file TLVs
 snmpCommunityName                      <CommunityName>
 snmpCommunitySecurityName              "@CMconfig_n"
 snmpCommunityContextEngineID           <the engineID of the cable modem>
 snmpCommunityContextName               <Zero-length OCTET STRING>
 snmpCommunityTransportTag              "@CMconfigTag_n" where n is 0..m-1 and m is the number of SNMPv1v2c
                                        Coexistence Configuration config file TLVs
 snmpCommunityStorageType               volatile (2)
 snmpCommunityStatus                    active (1)



8.5.2.7.3     snmpTargetAddrTable
The snmpTargetAddrTable is defined in the "Definitions" section of [RFC 3413].
The CM MUST create one row in snmpTargetAddrTable for each SNMPv1v2c Transport Address Access sub-TLV
of the SNMPv1v2c Coexistence Configuration TLV in the config file.
                                       Table 33 - snmpTargetAddrTable

   Column Name (* = Part of Index)                                     Column Value
 * snmpTargetAddrName                   "@CMconfigTag_n_i" where n is 0..m-1 and m is the number of SNMPv1v2c
                                        Coexistence Configuration config file TLVs.
                                        Where i is 0..p-1 and p is the number of SNMPv1v2c Transport Address Access
                                        sub-TLV within the SNMPv1v2c Coexistence Configuration config file TLV n
 snmpTargetAddrTDomain                  IPv4: snmpUDPDomain [RFC 3417]
                                        IPv6: transportDomainUdpIpv6 [RFC 3419]
 snmpTargetAddrTAddress                 IPv4: SnmpUDPAddress [RFC 3417]
 (IP Address and UDP Port)              OCTET STRING (6) Octets 1-4: <TAddress> Octets 5-6: <TAddress>
                                        IPv6: TransportAddressIPv6 [RFC 3419]
                                        OCTET STRING (18) Octets 1-16: <TAddress> Octets 17-18: <TAddress>
 snmpTargetAddrTimeout                  Default from MIB
 snmpTargetAddrRetryCount               Default from MIB
 snmpTargetAddrTagList                  "@CMconfigTag_n" where n is 0..m-1 and m is the number of SNMPv1v2c
                                        Coexistence Configuration config file TLVs
 snmpTargetAddrParams                   '00'h (null character)
 snmpTargetAddrStorageType              volatile (2)
 snmpTargetAddrRowStatus                active (1)



8.5.2.7.4     snmpTargetAddrExtTable
The snmpTargetAddrExtTable is defined in the "SNMP Community MIB Module" section of [RFC 3584].
The CM MUST create one row in snmpTargetAddrExtTable for each SNMPv1v2c Transport Address Access sub-
TLV of the SNMPv1v2c Coexistence Configuration TLV in the config file.




02/19/25                                             CableLabs®                                                       75
<!-- PDF_PAGE: 76 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


                                       Table 34 - snmpTargetAddrExtTable

     Column Name (* = Part of Index)                                      Column Value
 * snmpTargetAddrName                    "@CMconfigTag_n_i" where n is 0..m-1 and m is the number of SNMPv1v2c
                                         Coexistence Configuration config file TLVs.
                                         Where i is 0..p-1 and p is the number of SNMPv1v2c Transport Address Access
                                         sub-TLV within the SNMPv1v2c Coexistence Configuration config file TLV n
 snmpTargetAddrTMask                     <Zero-length OCTET STRING> when <TMask> is not provided in the ith
                                         SNMPv1v2c Transport Address Access sub-TLV
                                         IPv4: SnmpUDPAddress [RFC 3417]
                                         OCTET STRING (6) Octets 1-4: <TMask> Octets 5-6: <UDP Port>
                                         IPv6: TransportAddressIPv6 [RFC 3419]
                                         OCTET STRING (18) Octets 1-16: <TMask> Octets 17-18: <UDP Port>
 snmpTargetAddrMMS                       SM Maximum Message Size



8.5.2.7.5     vacmSecurityToGroupTable
The vacmSecurityToGroupTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create two rows in vacmSecurityGroupTable for each SNMPv1v2c Coexistence Configuration TLV
in the config file as follows:
     •   The CM MUST set in vacmSecurityName the keyword @CMconfig_n where 'n' is a sequential number
         starting at 0 for each SNMPv1v2c Coexistence Configuration TLV processed (e.g., "@CMconfig_0",
         "@CMconfig_1", etc.).
     •   The CM MUST set in vacmGroupName the keyword @CMconfigV1_n for the first row and
         @CMconfigV2_n for the second row where 'n' is a sequential number starting at 0 for each SNMPv1v2c
         Coexistence Configuration TLV processed (e.g., "@CMconfigV1_0", "@CMconfigV1_1", etc.).
                                    Table 35 - vacmSecurityToGroupTable

      Column Name (* = Part of Index)            First Row Column Value               Second Row Column Value
 * vacmSecurityModel                        SNMPV1 (1)                             SNMPV2c (2)
 * vacmSecurityName                         "@CMconfig_n"                          "@CMconfig_n"
 vacmGroupName                              "@CMconfigV1_n"                        "@CMconfigV2_n"
 vacmSecurityToGroupStorageType             volatile (2)                           volatile (2)
 vacmSecurityToGroupStatus                  active (1)                             active (1)



8.5.2.7.6     vacmAccessTable
The vacmAccessTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create two rows in vacmAccessTable for each SNMPv1v2c Coexistence Configuration TLV in the
config file as follows:
     •   The CM MUST set in vacmGroupName the keyword @CMconfigV1_n for the first row and
         @CMconfigV2_n for the second row where 'n' is a sequential number starting at 0 for each SNMPv1v2c
         Coexistence Configuration TLV processed (e.g., "@CMconfigV1_0", "@CMconfigV1_1", etc.).
                                          Table 36 - vacmAccessTable

      Column Name (* = Part of Index)                      Column Value                           Column Value
 * vacmGroupName                            "@CMconfigV1_n"                        "@CMconfigV2_n"
 * vacmAccessContextPrefix                  <zero-length string>                   <zero-length string>
 * vacmAccessSecurityModel                  SNMPV1 (1)                             SNMPV2c (2)
 * vacmAccessSecurityLevel                  noAuthNoPriv (1)                       noAuthNoPriv (1)




76                                                 CableLabs®                                                    02/19/25
<!-- PDF_PAGE: 77 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


     Column Name (* = Part of Index)                        Column Value                          Column Value
 vacmAccessContextMatch                      exact (1)                             exact (1)
 vacmAccessReadViewName                      Set <AccessViewName>                  Set <AccessViewName>
 vacmAccessWriteViewName                     When <AccessViewType> == '2' Set      When <AccessViewType> == '2' Set
                                             <AccessViewName>                      <AccessViewName>
                                             Otherwise, set <Zero-length OCTET     Otherwise, set <Zero-length OCTET
                                             STRING>                               STRING>
 vacmAccessNotifyViewName                    <Zero-length OCTET STRING>            <Zero-length OCTET STRING>
 vacmAccessStorageType                       volatile (2)                          volatile (2)
 vacmAccessStatus                            active (1)                            active (1)



8.5.2.8     SNMPv3 Access View Configuration Config File TLV
This section specifies CM processing requirements for SNMPv3 Access View Configuration TLVs when present in
the configuration file. The SNMPv3 Access View Configuration TLV is used to configure the table
vacmViewTreeFamilyTable in a simplified way. The CM MUST process SNMPv3 Access View Configuration
TLV in conjunction with SNMP TLV-11 containing SNMPv3 tables, TLV-38 as well as SNMPv1v2c Coexistence
Configuration TLV (see Section 8.5.2.7).
The mapping from the TLV to these tables is described in the following section.

8.5.2.8.1     Mapping of TLV Fields into SNMPv3 Tables
The following section describes the mapping of SNMPv3 Access View Configuration TLVs into
vacmViewTreeFamilyTable:
                          Table 37 - SNMPv3 Access View Configuration TLV Mapping

              Sub-TLVs                      Variable Name                  Associated MIB Object [RFC 3415]
 SNMPv3 Access View Name               AccessViewName                 vacmViewTreeFamilyViewName
 SNMPv3 Access View Subtree            AccessViewSubTree              vacmViewTreeFamilySubtree
 SNMPv3 Access View Mask               AccessViewMask                 vacmViewTreeFamilyMask
 SNMPv3 Access View Type               AccessViewType                 vacmViewTreeFamilyType


Disconnected entries in the CM SNMP access configuration database are not expected to be detected by the CM as
part of the configuration. Eventually, the SNMP agent will not grant access to SNMP requests, for example, to
disconnected Security Names and View trees as a result of a TLV configuration mistake.
Table 38 describes the CM procedures to populate the SNMP Management Framework Access Control Subsystem
[RFC 3412].
In configuring entries for SNMPv3 Access View Configuration TLV, note the following:
    One entry is created for each TLV. Some Access Views may have a number of included/excluded OID
    branches. Only Access View Name will be common for all these OID branches. To support such type of Access
    View with multiple included/excluded OID branches a number of multiple SNMPv3 Access View
    Configuration TLVs need to be defined in configuration file.

8.5.2.8.2     vacmViewTreeFamilyTable
The vacmViewTreeFamilyTable is defined in the "Definitions" section of [RFC 3415].
The CM MUST create one row in vacmViewTreeFamilyTable for each SNMPv3 Access View Configuration TLV
in the config file. The CM MUST reject the config file if two SNMPv3 Access View Configuration TLVs have
identical index components relative to vacmViewTreeFamilyTable. In such instance, the CM would not be able to
create an entry for the second TLV containing the duplicate index.




02/19/25                                            CableLabs®                                                     77
<!-- PDF_PAGE: 78 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


The CM MUST set the object vacmViewTreeFamilySubtree to 1.3.6 when no sub-TLV SNMPv3 Access View
Subtree is defined in the config file.
The CM MUST set the object vacmViewTreeFamilyMask to the default zero-length string when no sub-TLV
SNMPv3 Access View Mask is defined.
The CM MUST set the object vacmViewTreeFamilyType to the default value 1 (included) when no sub-TLV
SNMPv3 Access View Type is defined.
                                       Table 38 - vacmViewTreeFamilyTable

     Column Name (* = Part of Index)                                  Column Value
 * vacmViewTreeFamilyViewName             <AccessViewName>
 * vacmViewTreeFamilySubtree              <AccessViewSubTree>
 vacmViewTreeFamilyMask                   <AccessViewMask>
 vacmViewTreeFamilyType                   <AccessViewType>
 vacmViewTreeFamilyStorageType            volatile (2)
 vacmViewTreeFamilyStatus                 active (1)



8.5.2.9    SNMP CPE Access Control Configuration Config File TLV
The 'SNMP CPE Access Control' config File TLV (see [MULPIv3.1]) provides a mechanism to filter SNMP PDU-
requests originating from a CMCI interface.
The CM MUST enforce the requirements of 'SNMP CPE Access Control' when configured in SNMP Coexistence
mode.
The CM MAY ignore the 'SNMP CPE Access Control' encodings when configured in NmAccess mode.
When applicable, the CM MUST enforce the 'SNMP CPE Access Control' requirements to enable or disable SNMP
Access originating from a CMCI interface directed to any CM provisioned IP addresses (see [MULPIv3.1]) or any
of the CM's CMCI IP addresses defined in Section 9.1, and prior to SNMP protocol specific access control
mechanisms such as SNMPv3 Access View, or NmAccess settings.




78                                                     CableLabs®                                    02/19/25
