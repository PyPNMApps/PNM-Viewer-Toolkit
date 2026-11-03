# 8.3 - Accounting Management

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 72-72
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 72 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


8.3       Accounting Management
[CCAP-OSSIv4.0] defines an accounting management interface for subscriber usage-based applications
denominated Subscriber Account Management Interface Specification (SAMIS). SAMIS is defined to enable
prospective vendors of cable modems and cable modem termination systems to address the operational requirements
of subscriber account management in a uniform and consistent manner.

8.3.1      Subscriber Usage Billing and Class of Services
The [MULPIv4.0] specification uses the concept of class of service as the term to indicate the type of data services a
CM requests and receives from the CMTS; (see [MULPIv4.0]). From a high-level perspective, classes of services
are observed as subscriber types (e.g., residential or business) and the DOCSIS RFI MAC layer parameters fulfill
the subscriber service needs.

8.3.1.1     DOCSIS 1.1 Quality of Service (QoS)
The [MULPIv4.0] specification provides a mechanism for a CM to register with its CMTS and to configure itself
based on external QoS parameters when it is powered up or reset.
To quote (in part) from the Theory of Operation section of [MULPIv4.0]:
The principal mechanism for providing enhanced QoS is to classify packets traversing the RF MAC interface into a
Service Flow. A Service Flow is a unidirectional flow of packets that provide a particular Quality of Service. The
CM and the CMTS provide this QoS by shaping, policing, and prioritizing traffic according to the QoS Parameter
Set defined for the Service Flow.
The requirements for Quality of Service include
      •    a configuration and registration function for pre-configuring CM-based QoS Service Flows and traffic
           parameters;
      •    utilization of QoS traffic parameters for downstream Service Flows;
      •    classification of packets arriving from the upper layer service interface to a specific active Service Flow;
           and
      •    grouping of Service Flow properties into named Service Classes, so upper layer entities and external
           applications (at both the CM and the CMTS) can request Service Flows with desired QoS parameters in a
           globally consistent way.
A Service Class Name (SCN) is defined in the CMTS by provisioning (see Annex G. An SCN provides an
association to a QoS Parameter Set. Service Flows that are created using an SCN are considered to be "named"
Service Flows. The SCN identifies the service characteristics of a Service Flow to external systems such as a billing
system or customer service system. For consistency in billing, operators should ensure that SCNs are unique within
an area serviced by the same BSS that utilizes this interface. A descriptive SCN might be something like PrimaryUp,
GoldUp, VoiceDn, or BronzeDn to indicate the nature and direction of the Service Flow to the external system.
A Service Package implements a Service Level Agreement (SLA) between the MSO and its Subscribers on the RFI
interface. A Service Package might be known by a name such as Gold, Silver, or Bronze. A Service Package is itself
implemented by the set of named Service Flows (using SCNs) that are placed into a CM Configuration File that is
stored on a Config File server.
NOTE: The CM Configuration File contains several kinds of information needed to properly configure the CM and its
      relationship with the CMTS, but for the sake of this discussion, only the Service Flow and Quality of Service
      components are of interest.
The set of Service Flows defined in the CM Config File are used to create active Service Flows when the CM
registers with the CMTS. Note that many Subscribers are assigned to the same Service Package and, therefore, many
CMs use the same CM Config File to establish their active Service Flows.
A Service Package has to define at least two Service Flows known as Primary Service Flows that are used by default
when a packet matches none of the classifiers for the other Service Flows. A CM Config File that implements a



72                                                    CableLabs                                                04/29/26
