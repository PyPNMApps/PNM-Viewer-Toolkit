# 6 - OSSI Management Protocols

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 38-38
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 38 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


6 OSSI MANAGEMENT PROTOCOLS
6.1     SNMP Protocol
The SNMP protocol has been selected as the communication protocol for management of data-over-cable services.
The CM MUST implement the SNMPv3 protocol.
Although SNMPv3 offers certain security advantages over previous SNMP versions, many existing management
systems do not fully support SNMPv3, necessitating support of the theoretically less secure but more ubiquitous
SNMPv1 and SNMPv2c protocols.
The CM MUST implement the SNMPv1 and SNMPv2c protocols.
The IETF SNMP-related RFCs listed in Table 4 are supported by the CM.
                                         Table 4 - IETF SNMP-Related RFCs
 [RFC 1157]     A Simple Network Management Protocol
 [RFC 1901]     Introduction to Community-based SNMPv2 (Informational)
 [RFC 3410]     Introduction and Applicability Statements for Internet Standard Management Framework
 [RFC 3411]     An Architecture for Describing Simple Network Management Protocol (SNMP) Management Frameworks
 [RFC 3412]     Message Processing and Dispatching for the Simple Network Management Protocol (SNMP)
 [RFC 3413]     Simple Network Management Protocol (SNMP) Applications
 [RFC 3414]     User-based Security Model (USM) for version 3 of the Simple Network Management Protocol (SNMPv3)
 [RFC 3415]     View-based Access Control Model (VACM) for the simple Network Management Protocol (SNMP)
 [RFC 3416]     Version 2 of the Protocol Operations for the Simple Network Management Protocol (SNMP)
 [RFC 3417]     Transport Mappings for the Simple Network Management Protocol (SNMP)
 [RFC 3418]     Management Information Base for the Simple Network Management Protocol (SNMP)
 [RFC 3419]     Textual Conventions for Transport Addresses
 [RFC 3584]     Coexistence between Version 1, Version 2, and Version 3 of the Internet-standard Network Management
                Framework
 [RFC 3826]     The Advanced Encryption Standard (AES) Cipher Algorithm in the SNMP User-based Security Model


For support of SMIv2, Table 5 lists the IETF SNMP-related RFCs which are supported by the CM.
                                     Table 5 - SMIv2 IETF SNMP-Related RFCs
 [RFC 2578]     Structure of Management Information Version 2 (SMIv2)
 [RFC 2579]     Textual Conventions for SMIv2
 [RFC 2580]     Conformance Statements for SMIv2


For support of Diffie-Helman Key exchange for the User Based Security Model, Table 6 lists the IETF SNMP-
related RFC which is supported by the CM.
                                 Table 6 - Diffie-Helman IETF SNMP-Related RFC
 [RFC 2786]    Diffie-Helman USM Key Management Information Base and Textual Convention



6.1.1    Requirements for IPv6
Several transport domains were initially defined for SNMP (see [RFC 3417]). To support IPv6, [RFC 3419] adds a
new set of transport domains not only for SNMP but for any application protocol.
The CM MUST support the recommendations of [RFC 3419] to support SNMP over IPv6.




38                                                   CableLabs                                                  04/29/26
