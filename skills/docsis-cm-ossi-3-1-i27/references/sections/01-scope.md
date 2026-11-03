# 1 - Scope

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 11-15
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 11 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


1 SCOPE
1.1       Introduction and Purpose
This specification is part of the DOCSIS family of specifications developed by Cable Television Laboratories
(CableLabs). In particular, this specification is part of a series of specifications that defines the fifth generation of
high-speed data-over-cable systems. This specification was developed for the benefit of the cable industry and
includes contributions by operators and vendors from North America, Europe, and other regions.
This specification defines the Operations Support System Interface (OSSI) requirements for the cable modem (CM).

1.2       Background
1.2.1      Broadband Access Network
A coaxial-based broadband access network is assumed. This may take the form of either an all-coax or hybrid-
fiber/coax (HFC) network. The generic term "cable network" is used here to cover all cases.
A cable network uses a tree-and-branch architecture with analog transmission. The key functional characteristics
assumed in this document are the following:
      •    two-way transmission and
      •    a maximum optical/electrical spacing between the CMTS and the most distant CM of 100 miles (160 km)
           in each direction, though typical maximum separation may be 10–15 miles (16–24 km).

1.2.2      Network and System Architecture

1.2.2.1     The DOCSIS Network
The elements that participate in the provisioning of DOCSIS services are shown in Figure 1.


                                                                                        IPv4
                                                                                        CPE

                           NMS
                                                                         CM
                                                                                        IPv6
                                                                                        CPE

                                                  CMTS           HFC


                                                                                        IPv4
                                                                                        CPE
                                                                         CM
                       Provisioning
                        Systems                                                         IPv6
                                                                                        CPE



                           Back Office Network            HFC Network            Home Network

                                            Figure 1 - The DOCSIS Network


The CM connects to the operator's HFC network and to a home network, bridging packets between them. Many CPE
devices can connect to the CM's LAN interfaces. CPE devices can be embedded with the CM in a single device, or
they can be separate standalone devices (as shown in Figure 1).



02/19/25                                              CableLabs®                                                            11
<!-- PDF_PAGE: 12 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


CPE devices may use IPv4, IPv6, or both forms of IP addressing. Examples of typical CPE devices are home
routers, set-top devices, and personal computers.
The CMTS connects the operator's back office and core network with the HFC network. Its main function is to
forward packets between these two domains, and optionally to forward packets between upstream and downstream
channels on the HFC network. The CMTS performs this forwarding with any combination of link-layer (bridging)
and network-layer (routing) semantics.
Various applications are used to provide back office configuration and other support to the devices on the DOCSIS
network. These applications use IPv4 and/or IPv6 as appropriate to the particular operator's deployment. The
following applications include the following.
Provisioning Systems
     •   The DHCP servers provide the CM with initial configuration information, including the device IP
         address(es), when the CM boots.
     •   The configuration file server is used to download configuration files to CMs when they boot. Configuration
         files are in binary format and permit the configuration of the CM's parameters.

         The software download server is used to download software upgrades to the CM.
     •   The time protocol server provides time protocol clients, typically CMs, with the current time of day.
Network Management System (NMS)
     •   The SNMP manager allows the operator to configure and monitor SNMP agents, which reside within the
         cable modems.
     •   The Syslog server collects messages pertaining to the operation of devices.

1.2.3    Service Goals
As cable operators have widely deployed high-speed data services on cable television systems, the demand for
bandwidth has increased. Additionally, networks have scaled to such a degree that IPv4 address constraints are
becoming a burden on network operations. To this end, CableLabs' member companies have added new features to
the DOCSIS® specification for the purpose of increasing channel capacity, enhancing network security, expanding
addressability of network elements, and deploying new service offerings.
The DOCSIS system allows transparent bi-directional transfer of Internet Protocol (IP) traffic, between the cable
system headend and customer locations, over an all-coaxial or hybrid-fiber/coax (HFC) cable network. This is
shown in simplified form in Figure 2.




                     Figure 2 - Transparent IP Traffic Through the Data-Over-Cable System


1.2.4    Statement of Compatibility
This specification defines the DOCSIS 3.1 interface. Prior generations of DOCSIS were commonly referred to as the
DOCSIS 1.0, 1.1, 2.0, and 3.0 interfaces. DOCSIS 3.1 is backward-compatible with equipment built to the previous
specifications with the exception of DOCSIS 1.0 CMs. DOCSIS 3.1-compliant CMs interoperate seamlessly with




12                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 13 -->
Cable Modem Operations Support System Interface Specification                                                             CM-SP-CM-OSSIv3.1-I27-250219


DOCSIS 3.1 and DOCSIS 3.0 CMTSs. DOCSIS 3.1-compliant CMTSs seamlessly support DOCSIS 3.1, DOCSIS
3.0, DOCSIS 2.0, and DOCSIS 1.1.

1.2.5      Reference Architecture
The reference architecture for data-over-cable services and interfaces is shown in Figure 3.

                                       Distribution Hub or Headend
                                         DOCSIS Timing
                                         Interface (DTI)       DOCSIS                               Operations               Operations Support System
                                                             Timing Server                        Support System                  Interface (OSSI)




                                                                        EQAM
                                      M-CMTS           Downstream
                                       Core            External Phy                    Downstream
                                                     Interface (DEPI)                  RF Interface
                 Network Side                                                           ( DRFI)
                Interface (NSI)                Upstream                                                                                      Cable Modem to
                                               Receiver
                                                               M-CMTS                                                                         CPE Interface
                                                                                 Downstream
                                                                                                                                                 (CMCI)
                                                                                 RF Network
                                                                                                     Tx
                                                                                                  Opt.
                                                                                                   Tx                    Fiber
                                                                                                  Tx                    Fiber
                                                                                                                         Node                               Customer
                                                                                                          Fiber       Fiber
                                                                                                                       Node    Coax       Cable             Premises
                                                                                                       Distribution   Node Distribution   Modem             Equipment
        Wide Area                                                                                    Rx                                    (CM)
                                                                                                  Opt.
                                                                                                   Rx
         Network                                                                                  Rx
                                                                                   Upstream
                                                   I-CMTS                         RF Network
                                                   / CCAP



                                                                                                                                                  MAC & Upper Layer
                                                                                                                                               Protocols Interface (MULPI)
                                                                                                                                               & Security Interface (SEC)
                      NOTE: Gray-shaded areas represent related                          Physical Layer Interface
                      functionality, but are out of scope of this document.                       (PHY)




                                              Figure 3 - Data-Over-Cable Reference Architecture


1.2.6      DOCSIS 3.1 Documents
A list of the specifications in the DOCSIS 3.1 series is provided in Table 1. For further information, please refer to
http://www.cablelabs.com.
                                                   Table 1 - DOCSIS 3.1 Series of Specifications

             Designation                                                                                       Title
 CM-SP-PHYv3.1                                       Physical Layer Specification
 CM-SP-MULPIv3.1                                     Media Access Control and Upper Layer Protocols Interface Specification
 CM-SP-CM-OSSIv3.1                                   Cable Modem Operations Support System Interface Specification
 CM-SP-CCAP-OSSIv3.1                                 Converged Cable Access Platform Operations Support System Interface Specification
 CM-SP-SECv3.1                                       Security Specification
 CM-SP-CMCIv3.0                                      Cable Modem CPE Interface Specification

This specification defines the interface for the Operations Support Systems Interface (OSSI), specifically for the
Cable Modem.
Related DOCSIS specifications are listed in Table 2.
                                                    Table 2 - DOCSIS 3.1 Related Specifications

             Designation                                                                                       Title
 CM-SP-eDOCSIS                                       eDOCSIS™ Specification
 CM-SP-DRFI                                          Downstream Radio Frequency Interface Specification




02/19/25                                                                       CableLabs®                                                                                    13
<!-- PDF_PAGE: 14 -->
CM-SP-CM-OSSIv3.1-I27-250219                                         Data-Over-Cable Service Interface Specifications


           Designation                                                            Title
 CM-SP-DTI                           DOCSIS Timing Interface Specification
 CM-SP-DEPI                          Downstream External PHY Interface Specification
 CM-SP-DSG                           DOCSIS Set-Top Gateway Interface Specification
 CM-SP-ERMI                          Edge Resource Manager Interface Specification
 CM-SP-M-OSSI                        M-CMTS Operations Support System Interface Specification
 CM-SP-L2VPN                         Layer 2 Virtual Private Networks Specification
 CM-SP-TEI                           TDM Emulation Interface Specification


1.3    Requirements
Throughout this document, the words that are used to define the significance of particular requirements are
capitalized.
  "MUST"                This word means that the item is an absolute requirement of this specification.
  "MUST NOT"            This phrase means that the item is an absolute prohibition of this specification.
  "SHOULD"              This word means that there may exist valid reasons in particular circumstances to ignore this item, but
                        the full implications should be understood and the case carefully weighed before choosing a different
                        course.
  "SHOULD NOT"          This phrase means that there may exist valid reasons in particular circumstances when the listed
                        behavior is acceptable or even useful, but the full implications should be understood and the case
                        carefully weighed before implementing any behavior described with this label.
  "MAY"                 This word means that this item is truly optional. One vendor may choose to include the item because,
                        for example, a particular marketplace requires it or because it enhances the product; another vendor
                        may omit the same item.

This document defines many features and parameters, and a valid range for each parameter is usually specified.
Equipment (CM) requirements are always explicitly stated. Equipment needs to comply with all mandatory (MUST
and MUST NOT) requirements to be considered compliant with this specification. Support of non-mandatory
features and parameter values is optional.

1.4    Conventions
In this specification the following convention applies any time a bit field is displayed in a figure. The bit field should
be interpreted by reading the figure from left to right, then from top to bottom, with the MSB being the first bit so
read and the LSB being the last bit so read.
SNMP MIB syntax is represented by this code sample font.
NOTE: Notices and/or Warnings are identified by this style font and label.

1.5    Organization of Document
Section 1 provides an overview of the DOCSIS 3.1 series of specifications, including the DOCSIS reference
architecture and statement of compatibility.
Section 2 includes a list of normative and informative references used within this specification.
Section 3 defines the terms used throughout this specification.
Section 4 defines the acronyms used throughout this specification.
Section 5 provides a technical overview and lists the DOCSIS 3.1 key features for the functional areas of this
specification.
Section 6 defines requirements for the OSSI management protocols.
Section 7 defines the requirements for the OSSI management objects, including SNMP MIBs.




14                                                      CableLabs®                                                     02/19/25
<!-- PDF_PAGE: 15 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


Section 8 defines the FCAPS OSSI requirements for the PHY, MAC, and network layers.
Section 9 defines the OSSI requirements for the Cable Modem to CPE Interface (CMCI).
Section 10 defines the OSSI requirements for the cable modem device, including LED operations.

1.5.1    Annexes (Normative)
Annex A includes a detailed list of MIB object requirements for the CM.
Annex B defines protocol filtering requirements.
Annex C includes a detailed list of DOCSIS events and the associated formats.
Annex D defines the information model for the DOCSIS 3.1 Proactive Network Maintenance feature.
Annex E defines the DOCSIS 3.1 data type definitions.
Annex F defines the information model for the CM status and interface requirements.
Annex G defines the information model for the CM MULPI requirements.

1.5.2    Appendices (Informative)
Appendix I identifies spectrum analysis use cases.
Appendix II includes acknowledgements and contains a list of contributors.
Appendix III is the revision history of the specification.




02/19/25                                             CableLabs®                                          15
