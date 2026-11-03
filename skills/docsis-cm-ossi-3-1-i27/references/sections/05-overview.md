# 5 - Overview

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 26-30
Classification: Main body (mixed overview and requirements)

<!-- PDF_PAGE: 26 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


5 OVERVIEW
This section provides a brief description of the key management features introduced in DOCSIS 3.1. These features
are categorized according to the five conceptual categories of management developed as part of ITU
Recommendation [ITU-T M.3400]. This set of management categories is referred to as the FCAPS model,
represented by the individual management categories of Fault, Configuration, Accounting, Performance, and
Security.
In addition to the description of features, the rationale behind the introduction of information models is presented.
Section 5.1 discusses the requirements introduced in this DOCSIS 3.1 specification, and Section 5.2 is a technical
introduction to the detailed models in support of the user requirements.

5.1       DOCSIS 3.1 OSSI Key Features
Table 3 summarizes the new requirements that support new DOCSIS 3.1 features and the enhancements to existing
management features. The table shows the management features along with the traditional network management
functional areas (Fault, Configuration, Accounting, Performance, and Security) for the network elements (NE) cable
modem (CM) and the corresponding OSI layer where those features operate.
                             Table 3 - Management Feature Requirements for DOCSIS 3.1

              Features              Management           OSI Layer                         Description
                                   Functional Area
      OFDM downstream signals     Configuration        PHY              Provisioning physical downstream and upstream
      and OFDMA upstream                                                interfaces that support OFDM/OFDMA receivers
      signals                                                           according to their capabilities
      Plant topology              Configuration        PHY, MAC         Provisioning flexible arrangements of US/DS
                                                       (data link)      channels for channel bonding configuration to
                                                                        reflect HFC plant topology
      Enhanced diagnostics        Fault                PHY, MAC,        Expanded metrics for Proactive Network
                                                       network          Maintenance (PNM)
      Enhanced performance        Performance          PHY, MAC,        Collection of large statistical data sets for
      data collection                                  network          DOCSIS 3.1 feature sets
      Protective Network          Performance          PHY              To gather information on narrow band ingress
      Maintenance and enhanced                                          and distortion affecting the quality of the RF
      signal quality monitoring                                         signals
      Light sleep mode            Configuration        MAC              Energy efficiency mode for the cable modem to
                                                                        minimize power consumption
      Backup primary channels     Configuration        MAC              Retrieval of configuration status of backup
                                                                        downstream interfaces
      Active queue management     Configuration        MAC              Configuration of buffer management associated
      (AQM)                                                             with service flows
      Low latency                 Configuration and    MAC              Configuration and reporting of low-latency
                                  performance                           service statistics to manage queuing
                                                                        performance


5.1.1      Fault Management Features
The DOCSIS 3.1 fault management requirements include
      •    extended lists of detailed events related to the new set of DOCSIS 3.1 features and
      •    expanded metrics for Proactive Network Maintenance (PNM).




26                                                    CableLabs®                                                        02/19/25
<!-- PDF_PAGE: 27 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


5.1.2      Configuration Management Features
The configuration of the DOCSIS protocols for CM/CMTS interactions for configuring features in support of PHY
MAC/QoS and Security (BPI) uses the CM configuration file and CMTS policies via MAC messages exchange. The
reporting of configuration state information is done via SNMP MIB objects. This model provides a CM standard
configuration with minimal operator intervention.
The DOCSIS 3.1 configuration requirements include
      •    updates to CM configuration parameters to support OFDM downstream interfaces, OFDMA upstream
           interfaces, light sleep mode, and active queue management (AQM);
      •    retrieval of configuration status information for OFDM downstream interfaces, OFDMA upstream
           interfaces, light sleep mode, backup primary channels, and active queue management (AQM); and
      •    updates to CM QoS attributes to configure service flow histogram calculations and reporting.

5.1.3      Performance Management Features
DOCSIS 3.1 technology requires an efficient mechanism for collecting large data sets as described above. The
identified data sets are
      •    enhanced signal quality monitoring for granular plant status;
      •    statistics for dropped AQM packets;
      •    statistics for OFDM and OFMDA interface, subcarrier, profile, and minislot counters;
      •    measurement statistics for Proactive Network Maintenance (PNM); and
      •    latency statistics for histogram enabled service flows.

5.1.4      Security Management Features
Security management includes both security of management information (e.g., SNMP access control) and
management of network security related to authentication, authorization, and privacy of data plane communications.
DOCSIS 3.1 specifications define a new certificate Public Key Infrastructure (PKI), with a set of management
objects that strengthens the security of CM authentication and secure software download features.

5.1.5      Accounting Management Features
The DOCSIS 3.1 accounting management feature set is unchanged from DOCSIS 3.0.

5.2       Technical Overview
The technical overview presented in this section details functional areas of the FCAPS management model
addressed by DOCSIS specifications for managing the CM.

5.2.1      Architectural Overview
This section defines the functional areas of network management in terms of FCAPS (Fault, Configuration,
Accounting, Performance, and Security) as applied to the management of a CM within a DOCSIS network.
The requirements in the previous section were grouped both according to the management functional area and the
relevant DOCSIS layer (using the OSI reference model) where they apply. This section provides an overview of the
functions supported by each area. Even though specific functions are described for each area, there are
interdependencies amongst all these functions to achieve the overall objective of efficient and proactive
management of a CM in the DOCSIS network.
Fault management seeks to identify, isolate, correct, and record system faults. Configuration management modifies
system configuration variables and collects configuration information. Accounting management collects usage
statistics for subscribers, sets usage quotas, and bills users according to their use of the system. Performance
management focuses on the collection of performance metrics, analysis of these metrics, and the setting of


02/19/25                                              CableLabs®                                                27
<!-- PDF_PAGE: 28 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


thresholds and rate limits. Security management encompasses identification and authorization of users and
equipment, provides audit logs and alerting functions, and provides vulnerability assessment.
Figure 4 illustrates the CM management architecture from the MSO back office interface perspective. The CM and
CMTS reside within the network layer where services are provided to end subscribers and various metrics are
collected about network and service performance, among other things. Various management servers reside in the
network management layer within the MSO back office to provision, monitor, and administer the network elements
within the network layer (CM in this case). These management servers include, but are not limited to, the following.
     •   The SNMP Manager performs SNMP configuration and queries against a CM's SNMP agent.
     •   The Configuration File Server has the responsibility of transferring configuration files, via TFTP or
         optionally HTTP to the CM upon reinitialization.
     •   The Firmware File Server has the responsibility of transferring firmware images, according to the secure
         software download mechanism, to a CM.
     •   The Notification Receiver receives autonomous SNMP notifications and Syslog messages from a CM.
     •   The DHCP Server has the responsibility of assigning a CM its IPv4 and/or IPv6 address, as well as other
         DHCP parameters, in order for the CM to obtain its configuration file and register on the network.
     •   The Time Server provides a CM with current Time of Day (ToD).
     •   The IPDR Collector Servers do not communicate directly with the CM. Rather, the CMTS collects various
         CM-related statistics and communicates this information to the IPDR collector servers.
     •   The TR-069 Server does not communicate directly with the CM. Rather, if the CM is an eDOCSIS device
         and includes an eSAFE that supports the TR-069 protocol, the eDOCSIS device will communicate with the
         TR-069 server.




                                     Figure 4 - CM Management Architecture


Finally, the business and service management layer is where higher level MSO business processes are implemented
via BSS/OSS systems. These BSS/OSS systems utilize the data and information from the network management
layer, which interrogated data from the network layer.




28                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 29 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


5.2.1.1    Fault Management
The goals of fault management are to provide failure detection, diagnosis, and perform or indicate necessary fault
correction. Fault identification relies on the ability to monitor and detect problems, such as error-detection events.
Fault resolution relies on the ability to diagnose and correct problems, such as executing a sequence of diagnostic
test scripts, and correcting equipment or configuration faults. DOCSIS supports event reporting using local log,
Syslog, and SNMP notifications.

5.2.1.2    Configuration Management
Configuration management is concerned with adding, initializing, maintaining, and updating network elements. In a
DOCSIS environment, network elements include CMs and CMTSs.
Configuration management is primarily concerned with network control via modifying operating parameters on
network elements such as the CM and CMTS. Configuration parameters could include both physical resources (for
example, an Ethernet interface) and logical objects (for example, QoS parameters for a given service flow).
While the network is in operation, configuration management is responsible for monitoring the configuration state
and making changes in response to commands by a management system or some other network management
function.
For example, a performance management function may detect that response time is degrading due to a high number
of uncorrected frames and may issue a configuration management change to modify the modulation type from
16-QAM to QPSK. A fault management function may detect and isolate a fault and may issue a configuration
change to mitigate or correct that fault.

5.2.1.3    Accounting Management
Accounting management, in general, includes collection of usage data and permits billing the customer based on the
subscriber's use of network resources. The CMTS is the network element that is responsible for providing the usage
statistics to support billing. Billing is outside the scope of this specification.

5.2.1.4    Performance Management
Performance management functions include collecting statistics of parameters such as number of frames lost at the
MAC layer and number of codeword errors at the PHY layer. These monitoring functions are used to determine the
health of the network and whether the offered Quality of Service (QoS) to the subscriber is met. The quality of
signal at the PHY layer is an indication of plant conditions.
The previous versions of DOCSIS OSSI specifications define SNMP polling as the collection mechanism for CM
and CMTS statistics for performance management. SNMP polling of CMs is scalable and widely deployed with
specialized engines that minimize the upstream bandwidth allocated to management during the polling intervals.

5.2.1.5    Security Management
Security management is concerned with both security of management information to protect the MSO's operations
systems, as well as managing the security information. The latter is used to authenticate and secure the traffic on the
HFC. Security of the management interface is required to prevent end users from accessing and initiating
configuration changes that may provide them with services for which they are not entitled or could result in the
degradation or denial of services for other subscribers.

5.2.2     Management Protocols
As noted earlier in this section, the DOCSIS OSSI specifications use the Simple Network Management Protocol
(SNMP) versions 1, 2c, and 3 to define the management information for a CM DOCSIS network element in support
of the functional areas mentioned in the previous section. SNMP is primarily a polling-based protocol where the
management system retrieves data such as counter values and state information. There are events defined as a
notification that are used to inform the management systems of fault conditions and security violations. The support
for SNMP versions is continued in DOCSIS 3.1.




02/19/25                                             CableLabs®                                                          29
<!-- PDF_PAGE: 30 -->
CM-SP-CM-OSSIv3.1-I27-250219                                 Data-Over-Cable Service Interface Specifications


5.2.3   Information Models
The approach is based on an object-oriented modeling approach well known in the industry for capturing
requirements and analyzing the data in a protocol-independent representation. This approach defines requirements
with use cases to describe the interactions between the operations support systems and the network element. The
management information is represented in terms of objects along with their attributes and the interactions between
these encapsulated objects (or also referred to as entities in some representations). The diagrams developed to
capture these managed objects and their attributes and associations are UML class diagrams. The collection of UML
class diagrams and use case diagrams are referred to as the DOCSIS 3.1 Information Models. With the introduction
of several new, complex features in DOCSIS 3.0 and DOCSIS 3.1 technology and the operator needs for a more
proactive and efficient approach to management information, information modeling methodologies offer the ability
to reuse the same definitions when new protocols are introduced in the future.
The managed objects are then represented in a protocol-specific form referred to as a management data model. The
management data models when using SNMP are described using the Structure of Management Information
Version 2 (SMIv2) [RFC 2578], and the design of these models is determined by the capabilities of the protocol.
Refer to [UML Guidelines] for information on the modeling concepts used throughout this specification.




30                                                CableLabs®                                             02/19/25
