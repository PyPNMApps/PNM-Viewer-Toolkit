# 9 - OSSI for CMCI

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 112-113
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 112 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


9 OSSI FOR CMCI
This section defines the operational mechanisms needed to support the transmission of data over cable services
between a Cable Modem (CM) and Customer Premise Equipment (CPE). Specifically, this section outlines the
following:
      •    SNMP access via CMCI
      •    Console Access
      •    CM diagnostic capabilities
      •    Protocol Filtering
      •    Required MIBs
Refer to Section 6 of [CMCIv3.0] for additional CMCI requirements.

9.1       SNMP Access via CMCI
DOCSIS 4.0 CMs have provisions for dual-stack management or management of the CM using SNMP over IPv4
and IPv6. During provisioning, the management of the CM is determined by the MSO. However, SNMP access
from the CMCI port(s) for diagnostic purposes prior to the CM being registered needs to operate in a dual-stack
management mode and allow access for both IPv4 and IPv6 hosts. CM SNMP access from the CMCI before
completing the CMTS registration process MUST comply with the access requirements specified in Section 8.5.2.1.
The CM DHCP-acquired IP MUST ignore SNMP requests from CMCI before registration.
The CM DHCP-acquired IP MUST accept SNMP requests from CMCI after completing the CMTS registration
process where such SNMP access complies with the requirements stated in Section 8.5.2.2.
The CM MUST support SNMP access, as specified in Section 8.5.2, through the following IP addresses regardless
of the CM registration state:
      •    The CM MUST support 192.168.100.1, as the well-known diagnostic IP address accessible only from the
           CMCI interfaces. The CM MUST support the well-known diagnostic IP address, 192.168.100.1, on all
           physical interfaces associated with the CMCI. The CM MUST drop SNMP requests coming from the RF
           interface targeting the well-known IP address.
      •    The CM MAY also implement alternative IPv4 interfaces like link-local method described in [RFC 3927].
           If implemented, the CM MUST restrict the IP address range described in "Address Selection, Defense and
           Delivery" of [RFC 3927] to 169.254.1.0 to 169.254.254.255 inclusive.
      •    The CM MAY support an IPv6 EUI-64 link-local scope address in the format
           FE80::<vendorId>:FFFE:<remainingMacAddress> of the CMCI port. The CM MUST drop SNMP
           requests coming from the RF interface targeting this IPv6 address. Refer to [RFC 4291] for additional
           details.

9.2       Console Access
The CM MUST NOT allow access to the CM functions by a console port. In this specification, a console port is
defined as a communication path, either hardware or software, that allows a user to issue commands to modify the
configuration or operational status of the CM. The CM MUST only allow access using DOCSIS defined RF
interfaces and operator-controlled SNMP access by the CMCI.

9.3       CM Diagnostic Capabilities
The CM MAY have a diagnostic interface for debugging and troubleshooting purposes. If supported, the CM's
diagnostic interface MUST be limited by default to the requirements described in Section 8.5.2 before and after
registration. The CM's diagnostic interface SHOULD be disabled by default after registration has been completed.



112                                                 CableLabs                                              04/29/26
<!-- PDF_PAGE: 113 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv4.0-I13-260429


The CM MAY provide additional controls that will enable the MSO to alter or customize the diagnostic interface,
such as by the configuration process or management through the setting of a proprietary MIB.

9.4    Protocol Filtering
The CM MUST be capable of filtering traffic to and from the host CPE as defined in Annex B.




04/29/26                                          CableLabs                                                 113
