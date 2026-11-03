# 8.2 - Configuration Management

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 64-71
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 64 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


                                            Table 28 - vacmAccessTable

          Column Name                       First Row                    Second Row                  Third Row
        (* = Part of Index)               Column Value                   Column Value               Column Value
 * vacmGroupName                    "@configV1"                   "@configV2"                "@configUSM"
 * vacmAccessContextPrefix          <zero-length string>          <zero-length string>       <zero-length string>
 * vacmAccessSecurityModel          SNMPV1 (1)                    SNMPV2c (2)                USM (3)
 * vacmAccessSecurityLevel          noAuthNoPriv (1)              noAuthNoPriv (1)           noAuthNoPriv (1)
 vacmAccessContextMatch             exact (1)                     exact (1)                  exact (1)
 vacmAccessReadViewName             <Zero-length OCTET            <Zero-length OCTET         <Zero-length OCTET
                                    STRING>                       STRING>                    STRING>
 vacmAccessWriteViewName            <Zero-length OCTET            <Zero-length OCTET         <Zero-length OCTET
                                    STRING>                       STRING>                    STRING>
 vacmAccessNotifyViewName           "@config"                     "@config"                  "@config"
 vacmAccessStorageType              volatile (2)                  volatile (2)               volatile (2)
 vacmAccessStatus                   active (1)                    active (1)                 active (1)



8.1.4.1.12    vacmViewTreeFamilyTable
The vacmViewTreeFamilyTable is defined in the Definitions section of [RFC 3415].
The CM MUST create one row in vacmViewTreeFamilyTable with fixed values if one or more SNMPv3
Notification Receiver TLVs are present in the config file.
This row is used for the SNMPv3 Notification Receiver TLV entries with <TrapType> set to 1, 2, or 3 or with a
zero-length <SecurityName>. The SNMPv3 Notification Receiver TLV entries with <TrapType> set to 4 or 5 and a
non-zero length <SecurityName> will use the rows created in the vacmViewTreeFamilyTable by the DH Kickstart
process.
                                      Table 29 - vacmViewTreeFamilyTable

             Column Name (* = Part of Index)                                         Column Value
 * vacmViewTreeFamilyViewName                               "@config"
 * vacmViewTreeFamilySubtree                                1.3
 vacmViewTreeFamilyMask                                     <default from MIB>
 vacmViewTreeFamilyType                                     included (1)
 vacmViewTreeFamilyStorageType                              volatile (2)
 vacmViewTreeFamilyStatus                                   active (1)



8.1.5    Non-SNMP Fault Management Protocols
The OSS can use a variety of tools and techniques to examine faults at multiple layers. For the IP layer, useful non-
SNMP based tools include ping (ICMP Echo and Echo Reply), and trace route (UDP and various ICMP Destination
Unreachable flavors). The CM MUST respond to ICMP Echo Request (ping) messages received through its CMCI
[CMCIv3.0] interface(s) to enable local connectivity testing from a subscriber's PC to the modem. The CM MUST
support IP end-station generation of ICMP error messages and processing of all ICMP messages.
Syslog requirements are defined in Section 8.1.2.

8.2     Configuration Management
The CM is required to support the SNMP protocol interface as specified in Section 6. Section 7 defines the SNMP
MIB objects that are required to be supported by a CM.




64                                                  CableLabs                                                  04/29/26
<!-- PDF_PAGE: 65 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


In addition to the SNMP interface to modify the attribute values stored in the CM, vendors may specify additional
methods such as Command Line Interface (CLI) or an HTTP interface, as examples. Irrespective of the method
used, it is necessary to assure the data integrity as a result of changes performed using different interfaces. For
example, when the attribute value is modified using one management interface, this changed value is reported when
that attribute is accessed from any of the other interfaces. When a change in the value of the attribute does not
succeed, requesting the same change from another interface should also results in failure (assuming the same level
of access control for all those interfaces for the specific operation). If an event is generated as a result of making the
change in one management interface, this is reported independent of which method was used to initiate the change.

8.2.1    Version Control
The CM MUST support software revision and operational parameter configuration interrogation.
The CM includes the hardware version, boot ROM image version, vendor name, current software version, and
model number in the sysDescr object (from [RFC 3418]).
The CM MUST support docsDevSwCurrentVers MIB object (from [RFC 4639]) and report the current software
version of the CM.
The CM MUST report for the sysDescr object the Type and Value fields identified in Table 30 - sysDescr Format.
                                              Table 30 - sysDescr Format

                                             Type                   Value
                                        HW_REV             <Hardware Version>
                                        VENDOR             <Vendor Name>
                                        BOOTR              <Boot ROM Version>
                                        SW_REV             <Software Version>
                                        MODEL              <Model Number>


The CM MUST report each Type and Value for the sysDescr object identified in Table 30 - sysDescr Format, with
each Type field and corresponding Value field separated with a colon followed by a single blank space and each
Type-Value pair is separated by a semicolon followed by a single blank space. The correct format is illustrated
below.

HW_REV: <value>; VENDOR: <value>; BOOTR: <value>; SW_REV: <value>; MODEL: <value>

For instance, a sysDescr of a CM of vendor X, hardware version 5.2, boot ROM image version 1.4, software version
2.2, and model number Z is formatted as follows:
         any text<<HW_REV: 5.2; VENDOR: X; BOOTR: 1.4; SW_REV: 2.2; MODEL: Z>>any text
The CM MUST report all of the information necessary in determining what software the CM is capable of being
upgraded to. If any fields in Table 30 - sysDescr Format are not applicable, the CM MUST report "NONE" as the
value.
For instance, a sysDescr of a CM of vendor X, hardware version 5.2, no boot ROM image information, software
version 2.2, and model number Z is formatted as follows:
         any text<<HW_REV: 5.2; VENDOR: X; BOOTR: NONE; SW_REV: 2.2; MODEL: Z>>any text
The intent of specifying the format of sysDescr is to define how to report information in a consistent manner so that
sysDescr field information can be programmatically parsed. This format specification does not intend to restrict the
vendor's hardware version numbering policy.

8.2.2    System Configuration
The CM MUST support system configuration by configuration file, configuration-file-based SNMP encoded object,
and SNMP Set operation. The CM MUST support any valid configuration file created in accordance with
configuration file size limitations defined in the CM Configuration Interface Specification Annex in [MULPIv4.0].



04/29/26                                              CableLabs                                                        65
<!-- PDF_PAGE: 66 -->
CM-SP-CM-OSSIv4.0-I13-260429                               Data-Over-Cable Service Interface Specifications


8.2.3    Secure Software Download
The CM Secure Software Download (SSD) process is documented in detail in the Secure Software Download
section of [SECv4.0].
The CM MUST use the Secure Software Download mechanism to perform software upgrade regardless of the
DOCSIS specification version the CMTS it is connected to complies with.
There are two available Secure Software Download schemes: the manufacturer control scheme and the operator
control scheme.




                                   Figure 6 - Manufacturer Control Scheme


In reference to Figure 6 above:
     Note 1: Use docsDevSoftware group ([RFC 2669], [RFC 4639]) in case that the software downloading is
     triggered by the MIB.




                                     Figure 7 - Operator Control Scheme


In reference to Figure 7 above:
     Note 1: Use docsDevSoftware group ([RFC 2669], [RFC 4639]) in case that the software downloading is
     triggered by the MIB.




66                                               CableLabs                                          04/29/26
<!-- PDF_PAGE: 67 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


Prior to Secure Software Download initialization, CVC information needs to be initialized at the CM for software
upgrade. Depending on the scheme (described above) that the operator chooses to implement, the CM requires
appropriate CVC information in the configuration file. It is recommended that CVC information always be present
in the configuration file so that a device will always have the CVC information initialized and read if the operator
decides to use a SNMP-initiated upgrade as a method to trigger a Secure Software Download operation. If the
operator decides to use a configuration-file-initiated upgrade as a method to trigger Secure Software Download,
CVC information needs to be present in the configuration file at the time the CM is rebooted to get the configuration
file that will trigger the upgrade only.
There are two methods to trigger Secure Software Download: SNMP-initiated and configuration-file-initiated. The
CM MUST support both SNMP-initiated and configuration-file-initiated methods to trigger Secure Software
Download.
The following describes the SNMP-initiated mechanism. Prior to an SNMP-initiated upgrade, a CM MUST have
valid X.509-compliant code verification certificate information. From a network management station:
    1.   Set docsDevSwServerAddressType to 'ipv4' or 'ipv6'.
    2.   Set docsDevSwServerAddress to the IPv4 or IPv6 address of the Software Download server for software
         upgrades.
    3.   Set docsDevSwFilename to the file path name of the software upgrade image.
    4.   Set docsDevSwAdminStatus to 'upgradeFromMgt'.
If docsDevSwAdminStatus is set to 'ignoreProvisioningUpgrade', the CM MUST ignore any software download
configuration file setting and not attempt a configuration file initiated upgrade.
The CM MUST preserve the value of docsDevSwAdminStatus across reset/reboots until over-written from an
SNMP manager or by a TLV-11 [MULPIv4.0] setting in the CM configuration file. That is, the value of
docsDevSwAdminStatus is required to persist across CM reboots.
The CM MUST report 'allowProvisioningUpgrade' as the default value of docsDevSwAdminStatus until it is over-
written by 'ignoreProvisioningUpgrade', following a successful SNMP-initiated software upgrade or otherwise
altered by the management station.
The CM MUST preserve the value of docsDevSwOperStatus across reset/reboots. That is, the value of the CM's
docsDevSwOperStatus object is required to persist across resets to report the outcome of the last software upgrade
attempt.
After the CM has completed a configuration-file-initiated secure software upgrade, the CM MUST reboot and
become operational with the correct software image as specified in [MULPIv4.0]. After the CM is registered
following a reboot after a configuration file initiated secure software upgrade, the CM MUST adhere to the
following requirements.
    •    The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
    •    The CM MAY report the filename of the software currently operating on the CM as the value for
         docsDevSwFilename.
    •    The CM MAY report the IP address of the Software Download server containing the software that is
         currently operating on the CM as the value for docsDevSwServerAddress.
    •    The CM MUST report 'completeFromProvisioning' as the value for docsDevSwOperStatus.
    •    The CM MUST report the current version of the software that is operating on the CM as the value for
         docsDevSwCurrentVers.
After the CM has completed an SNMP-initiated secure software upgrade, the CM MUST reboot and become
operational with the correct software image as specified in [MULPIv4.0]. After the CM is registered following a
reboot after an SNMP-initiated secure software upgrade, the CM MUST adhere to the following requirements.
    •    The CM MUST report 'ignoreProvisioningUpgrade' as the value for docsDevSwAdminStatus.




04/29/26                                           CableLabs                                                     67
<!-- PDF_PAGE: 68 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


     •   The CM MAY report the IP address of the Software Download server containing the software that is
         currently operating on the CM as the value for docsDevSwServerAddress.
     •   The CM MUST report 'completeFromMgt' as the value for docsDevOperStatus.
     •   The CM MUST report the current version of the software that is operating on the CM as the value for
         docsDevSwCurrentVers.
If the value of docsDevSwAdminStatus is 'ignoreProvisioningUpgrade', the CM MUST ignore any software upgrade
value that is optionally included in the CM configuration file and become operational with the current software
image after the CM is registered. After the CM is registered following a reboot with a software upgrade value in the
CM configuration file, the CM MUST adhere to the following requirements.
     •   The CM MUST report 'ignoreProvisioningUpgrade' as the value for docsDevSwAdminStatus.
     •   The CM MAY report the filename of the software currently operating on the CM as the value for
         docsDevSwFilename.
     •   The CM MAY report the IP address of the Software Download server containing the software that is
         currently operating on the CM as the value for docsDevSwServerAddress.
     •   The CM MUST report 'completeFromMgt' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of the software that is operating on the CM as the value for
         docsDevSwCurrentVers.
Retries due to a power loss or reset are only required for an SNMP-initiated upgrade. If a power loss or reset occurs
during a configuration-file-initiated upgrade, the CM will follow the upgrade TLV directives in the configuration
file upon reboot. It will not retry the previous upgrade. The config file upgrade TLVs essentially provides a retry
mechanism that is not available for an SNMP-initiated upgrade.
If a CM suffers a loss of power or resets during an SNMP-initiated upgrade, the CM MUST resume the upgrade
without requiring manual intervention. When the CM resumes the upgrade process after a reset that occurred during
an SNMP-initiated software upgrade, the CM MUST adhere to the following requirements.
     •   The CM MUST report 'upgradeFromMgt' as the value for docsDevSwAdminStatus.
     •   The CM MUST report the filename of the software image to be upgraded as the value for
         docsDevSwFilename.
     •   The CM MUST report the IP address of the Software Download server containing the software upgrade
         image to be upgraded as the value for docsDevSwServerAddress.
     •   The CM MUST report 'inProgress' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
In the case where the CM reaches the maximum number of TFTP Download Retries, as specified in the Parameters
and Constraints Annex of [MULPIv4.0], resulting from multiple losses of power or resets during an SNMP-initiated
upgrade, the CM MUST behave as specified in [MULPIv4.0]. In this case, after the CM is registered, the CM
MUST adhere to the following requirements.
     •   The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
     •   The CM MUST report the filename of the software that failed the upgrade process as the value for
         docsDevSwFilename.
     •   The CM MUST report the IP address of the Software Download server containing the software that failed
         the upgrade process as the value for docsDevSwServerAddress.
     •   The CM MUST report 'other' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.


68                                                 CableLabs                                               04/29/26
<!-- PDF_PAGE: 69 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


When the CM reboots following a reset that occurred during a configuration file-initiated software download, the
CM MUST ignore the fact that a previous upgrade was in progress and either not perform an upgrade if no upgrade
TLVs are present in the config file, or if upgrade TLVs are present, take the action described in the requirements in
the section "Downloading Cable Modem Operating Software" of [MULPIv4.0], at the time of the reboot.
In the case where the CM had a configuration-file-initiated upgrade in progress during a reset and if there are no
upgrade TLVs in the config file upon reboot, the CM MUST adhere to the following requirements.
    •    The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
    •    The CM MAY report the filename of the current software image as the value for docsDevSwFilename.
    •    The CM MAY report the IP address of the Software Download server containing the software that is
         currently operating in the CM as the value for docsDevSwServerAddress.
    •    The CM MUST report 'other' as the value for docsDevSwOperStatus.
    •    The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
In the case where the CM had a configuration-file-initiated upgrade in progress during a reset, if there are upgrade
TLVs in the config file upon reboot, the CM MUST adhere to the following requirements.
    •    The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
    •    The CM MUST report the filename contained in TLV-9 of the config file as the value for
         docsDevSwFilename.
    •    The CM MUST report the IP address of the Software Download server containing the software to be loaded
         into the CM as the value for docsDevSwServerAddress, per the requirements stated in the section
         "Downloading Cable Modem Operating Software" of [MULPIv4.0].
    •    The CM MUST report 'inProgress' as the value for docsDevSwOperStatus.
    •    The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
If a CM exhausts the required number of TFTP Request Retries, as specified in the Parameters and Constraints
Annex of [MULPIv4.0], the CM MUST behave as specified in [MULPIv4.0]. If a CM exhausts the maximum
number of configured TFTP Request Retries without successfully downloading the specified file, the CM MUST
fall back to last known working image and proceed to an operational state. After a CM falls back to the last known
working software image after exhausting the maximum number of configured TFTP Request Retries without
successfully downloading the specified file, the CM MUST adhere to the following requirements.
    •    The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
    •    The CM MUST report the filename of the software that failed the upgrade process as the value for
         docDevSwFilename.
    •    The CM MUST report the IP address of the Software Download server containing the software that failed
         the upgrade process as the value for docsDevSwServerAddress.
    •    The CM MUST report 'failed' as the value for docsDevSwOperStatus.
    •    The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
In the case where a CM successfully downloads (or detects during download) an image that is not intended for the
CM device, the CM behaves as specified in the Downloading Cable Modem Operating Software section of
[MULPIv4.0]. If a CM successfully downloads an image that is not intended for it, or detects during the download
of a software image that the image is not for itself, the CM MUST adhere to the following requirements.
    •    The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.




04/29/26                                            CableLabs                                                         69
<!-- PDF_PAGE: 70 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


     •   The CM MUST report the filename of the software that failed the upgrade as the value for
         docsDevSwFilename.
     •   The CM MUST report the IP address of the Software Download server containing the software that failed
         the upgrade process as the value for docsDevSwServerAddress.
     •   The CM MUST report 'other' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
In the case where the CM determines that the download image is damaged or corrupted, the CM MUST reject the
newly downloaded image. The CM MAY re-attempt to download if the maximum number of TFTP Download
Retries has not been reached, as specified in the Parameters and Constants Annex of [MULPIv4.0]. If the CM
chooses not to retry, the CM MUST fall back to the last known working image and proceed to an operational state
and generate appropriate event notification as specified in Annex C. If the CM does not retry to download a
corrupted software image and falls back to the last known working software image, the CM MUST adhere to the
following requirements.
     •   The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
     •   The CM MUST report the filename of the software that failed the upgrade as the value for
         docsDevSwFilename.
     •   The CM MUST report the IP address of the Software Download server containing the software that failed
         the upgrade process as the value for docsDevSwServerAddress.
     •   The CM MUST report 'other' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.
In the case where the CM determines that the image is damaged or corrupted, the CM MAY re-attempt to download
the new image if the maximum number of TFTP Download Retries has not been reached, as specified in Parameters
and Constraints Annex of [MULPIv4.0]. On the final consecutive failed retry of the CM software download attempt,
the CM MUST fall back to the last known working image and proceed to an operational state and generate
appropriate event notification as specified in Annex C. If a CM falls back to the last known working software image
after failing the defined consecutive retry attempts, the CM MUST send two notifications, one to notify that the max
retry limit has been reached, and another to notify that the image is damaged. Immediately after the CM reaches the
operational state after failing the defined consecutive retry attempts to download a software image and falling back
to the last known working software image, the CM MUST adhere to the following requirements.
     •   The CM MUST report 'allowProvisioningUpgrade' as the value for docsDevSwAdminStatus.
     •   The CM MUST report the filename of the software that failed the upgrade as the value for
         docsDevSwFilename.
     •   The CM MUST report the IP address of the Software Download server containing the software that failed
         the upgrade process as the value for docsDevSwServerAddress.
     •   The CM MUST report 'other' as the value for docsDevSwOperStatus.
     •   The CM MUST report the current version of software that is operating on the CM as the value for
         docsDevSwCurrentVers.

8.2.4    CM Configuration Files, TLV-11 and MIB OIDs/Values
The following sections define the use of CM configuration file TLV-11 elements and the CM rules for translating
TLV-11 elements into SNMP PDU (SNMP MIB OID/instance and MIB OID/instance value combinations; also
referred to as SNMP varbinds).
This section also defines the CM behaviors, or state transitions, after either pass or fail of the CM configuration
process.



70                                                  CableLabs                                                04/29/26
<!-- PDF_PAGE: 71 -->
Cable Modem Operations Support System Interface Specification                 CM-SP-CM-OSSIv4.0-I13-260429


For TLV-11 definitions, refer to the Common Radio Frequency Interface Encodings Annex of [MULPIv4.0].

8.2.4.1     CM Configuration File TLV-11 Element Translation (to SNMP PDU)
TLV-11 translation defines the process used by the CM to convert CM configuration file information (TLV-11
elements) into SNMP PDU (varbinds). The CM is required to translate CM configuration file TLV-11 elements into
a single SNMP PDU containing (n) MIB OID/instance and value components (SNMP varbinds). Once a single
SNMP PDU is constructed, the CM processes the SNMP PDU and determines the CM configuration pass/fail based
on the rules for CM configuration file processing, described below. However, if a CM is not physically capable of
processing a potentially large single CM configuration file-generated SNMP PDU, the CM is still required to behave
as if all MIB OID/instance and value components (SNMP varbinds) from CM configuration file TLV-11 elements
are processed as a single SNMP PDU.
In accordance with [RFC 3416], the single CM configuration file generated SNMP PDU will be treated "as if
simultaneous" and the CM MUST behave consistently, regardless of the order in which TLV-11 elements appear in
the CM configuration file, or SNMP PDU.
The CM configuration file MUST NOT contain duplicate TLV-11 elements (duplicate means SNMP MIB object has
identical OID). If the configuration file received by the CM contains duplicate TLV-11 elements, the CM MUST
reject the configuration file.

8.2.4.1.1     Rules for CreateAndGo and CreateAndWait
The CM MUST support 'createAndGo' [RFC 2579] for row creation.
The CM MAY support 'createAndWait' [RFC 2579]. If the CM supports 'createAndWait', there is the constraint that
CM configuration file TLV-11 elements MUST NOT be duplicated (all SNMP MIB OID/instance are required to be
unique). If a CM constructs an SNMP PDU from a CM configuration file TLV-11 element that contains an SNMP
'createAndWait' value for a given SNMP MIB OID/instance, the CM MUST NOT also include in that SNMP PDU
an SNMP Active value for the same SNMP MIB OID/instance (and vice versa). A CM MAY accept a configuration
file that contains a TLV-11 'createAndWait' element if the intended result is to create an SNMP table row which will
remain in the SNMP 'notReady' or SNMP 'notInService' state until a non-configuration file SNMP PDU is issued,
from an SNMP manager, to update the SNMP table row status.
Both SNMP 'notReady' and SNMP 'notInService' states are valid table row states after an SNMP 'createAndWait'
instruction.

8.2.4.2     CM Configuration TLV-11 Elements Not Supported by the CM
If any CM configuration file TLV-11 elements translate to SNMP MIB OIDs that are not MIB OID elements
supported by the CM, then the CM MUST ignore those SNMP varbinds, and treat them as if they had not been
present, for the purpose of CM configuration. This means that the CM will ignore SNMP MIB OIDs for other
vendors' private MIBs as well as standard MIB elements that the CM does not support.
CMs that do not support SNMP CreateAndWait for a given SNMP MIB table MUST ignore, and treat as if not
present, the set of columns associated with the SNMP table row.
If any CM configuration file TLV-11 element(s) are ignored, then the CM MUST report them via the CM
configured notification mechanism(s), after the CM is registered. The CM MUST report ignored configuration file
TLV-11 elements following the notification method in accordance with Section 8.1.2.3.

8.2.4.3     CM State After CM Configuration File Processing Success
After successful CM configuration via CM configuration file, the CM MUST proceed to register with the CMTS
and proceed to its operational state.

8.2.4.4     CM State After CM Configuration File Processing Failure
If any CM configuration file generated SNMP PDU varbind performs an illegal set operation (illegal, bad, or
inconsistent value) to any MIB OID/instance supported by the CM, the CM MUST reject the configuration file. The
CM MUST NOT proceed with CM registration if it fails to download and process the configuration file.



04/29/26                                           CableLabs                                                    71
