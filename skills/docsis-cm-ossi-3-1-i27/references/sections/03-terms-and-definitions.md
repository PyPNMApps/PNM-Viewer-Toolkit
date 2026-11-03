# 3 - Terms and Definitions

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 19-21
Classification: Main body (definitions)

<!-- PDF_PAGE: 19 -->
Cable Modem Operations Support System Interface Specification                            CM-SP-CM-OSSIv3.1-I27-250219


3 TERMS AND DEFINITIONS
This specification uses the following terms and definitions.
  active queue management       AQM schemes attempt to maintain low queue occupancy (within downstream and upstream
                                service flows) while supporting the ability to absorb a momentary traffic burst.
  allocation                    A group of contiguous minislots in a MAP that constitute a single transmit opportunity.
  burst                         A single continuous RF signal from the upstream transmitter, from transmitter on to transmitter
                                off.
  cable modem (CM)              A modulator-demodulator at subscriber locations intended for use in conveying data
                                communications on a cable television system.
  cable modem termination       A device located at the cable television system headend or distribution hub that provides
  system (CMTS)                 complementary functionality to the cable modems to enable data connectivity to a wide-area
                                network.
  cable modem to CPE            The interface, defined in [CMCIv3.0], between the CM and CPE devices.
  interface (CMCI)
  carrier-to-noise plus         The ratio of the expected commanded received signal power at the CMTS input to the noise plus
  interference ratio (CNIR)     interference in the channel.
  channel                       The frequency spectrum occupied by a signal, usually specified by center frequency and
                                bandwidth parameters.
  classifier                    A set of criteria used for packet matching according to TCP, UDP, IP, LLC, and/or 802.1P/Q
                                packet fields. A classifier maps each packet to a service flow. A downstream classifier is used by
                                the CMTS to assign packets to downstream service flows. An upstream classifier is used by the
                                CM to assign packets to upstream service flows.
  customer                      See end user.
  customer premises             Equipment at the end user's premises; may be provided by the end user or the service provider.
  equipment (CPE)
  data model                    A data model (as opposed to an information model) is defined at a lower level of abstraction, is
                                intended for implementations, and includes protocol-specific constructs. Since conceptual
                                models can be implemented in different ways, multiple data models can be derived from a single
                                information model. Data models are technology specific. The CM has defined data models for
                                SNMP as SNMP MIB modules.
  downstream (DS)               In cable television, the direction of transmission from the headend to the subscriber.
  end user                      A human being, organization, or telecommunications system that accesses a network in order to
                                communicate via the services provided by the network (also known as customer).
  FCAPS                         A set of principles for managing networks and systems wherein each letter represents one
                                principle. F is for fault, C is for configuration, A is for accounting, P is for performance, and S is
                                for security.
  fiber node                    A point of interface between a fiber trunk and the coaxial distribution.
  hybrid fiber-coax (HFC)       A broadband bidirectional shared-media transmission system using fiber trunks between the
  system                        headend and the fiber nodes, and coaxial distribution from the fiber nodes to the customer
                                locations.
  inform                        A confirmed SNMP message for asynchronous notification of events from an SNMP entity.
  information model             An information model (as opposed to a data model) is an abstraction and only provides a high
                                level view of things of interest (e.g., information) to the business. It aids in understanding the
                                scope and breadth of the business, rather than the depth. An information model is a way of
                                representing and structuring information that has advantages over other common artifacts such
                                as a glossary, descriptive document, database, or source code. A common information model
                                will streamline the processes associated with information exchange, both within a business (e.g.,
                                Enterprise) and between a business and its external stakeholders.
  International Organization    An international standards body commonly known as the International Standards Organization.
  for Standardization (ISO)
  local log                     A volatile or non-volatile log stored within a network element.
  logical upstream channel      A MAC entity identified by a unique channel ID and for which bandwidth is allocated by an
                                associated MAP message. A physical upstream channel may support multiple logical upstream
                                channels. The associated UCD and MAP messages completely describe the logical channel.
  Low Latency DOCSIS            A set of tools for providing control, status, and statistics of packet queuing delays incurred by the
                                processing of packets through the forwarding plane of the system referred to as Low Latency
                                Support as defined in [MULPIv3.1].




02/19/25                                                CableLabs®                                                                  19
<!-- PDF_PAGE: 20 -->
CM-SP-CM-OSSIv3.1-I27-250219                                 Data-Over-Cable Service Interface Specifications


 MAC domain              A subcomponent of the CMTS that provides data forwarding services to a set of downstream
                         and upstream channels.
 MAC domain downstream   The subset of a downstream service group (DS-SG) that is confined to the downstream
 service group           channels of a single MAC domain. An MD-DS-SG differs from a DS-SG only when multiple MAC
                         domains are configured per DS-SG.
 MAC domain upstream     The subset of an upstream service group (US-SG) that is confined to the upstream channels of a
 service group           single MAC domain. An MD-US-SG differs from a US-SG only when multiple MAC domains are
                         defined per US-SG.
 media access control    The "built-in" hardware address of a device connected to a shared medium.
 (MAC) address
 micro-reflections       Echoes in the forward or reverse transmission path because of impedance mismatches between
                         the physical plant components. Micro-reflections are distinguished from discrete echoes by
                         having a time difference (between the main signal and the echo) on the order of 1 microsecond.
                         Micro-reflections cause departures from ideal amplitude and phase characteristics for the
                         transmission channel.
 minislot                An integer multiple of 6.25-microsecond increments.
 network management      The functions related to the management of data link layer and physical layer resources and
                         their stations across the data network supported by the hybrid fiber-coax system.
 network management      The hardware and software components used by a network provider to manage its networks as
 system (NMS)            a whole. The network management system provides an end-to-end network view of the entire
                         network, enabling management of the network elements contained in the network.
 notification            Information emitted by a managed object relating to an event that has occurred within the
                         managed object.
 occupied bandwidth      1) Downstream - The sum of the bandwidth in all standard channel frequency allocations (e.g.,
                         6-MHz spaced CEA channels) that are occupied by the OFDM channel. Even if one active
                         subcarrier of an OFDM channel is placed in a given standard channel frequency allocation, that
                         standard channel frequency allocation in its entirety is said to be occupied by the OFDM
                         channel. 2) Upstream - a) For a single OFDMA channel, the sum of the bandwidth in all the
                         subcarriers of that OFDMA channel that are not excluded. The upstream occupied bandwidth is
                         calculated as the number of subcarriers that are not excluded, multiplied by the subcarrier
                         spacing. b) For the transmit channel set, the sum of the occupied bandwidth of all OFDMA
                         channels plus the bandwidth of the legacy channels (counted as 1.25 times the modulation rate
                         for each legacy channel) in a cable modem's transmit channel set. The combined bandwidth of
                         all the minislots in the channel is normally smaller than the upstream occupied bandwidth
                         because of the existence of unused subcarriers. The bandwidth occupied by an OFDMA probe
                         with a skip value of zero is equal to the upstream occupied bandwidth [PHYv3.1].
 Open Systems            A framework of ISO standards for communication between different systems made by different
 Interconnection (OSI)   vendors in which the communications process is organized into seven different categories that
                         are placed in a layered sequence based on their relationship to the user. Each layer uses the
                         layer immediately below it and provides a service to the layer above. Layers 7 through 4 deal
                         with end-to-end communication between the message source and destination, and layers 3
                         through 1 deal with network functions.
 physical (PHY) layer    Layer 1 in the OSI architecture; the layer that provides services to transmit bits or groups of bits
                         over a transmission link between open systems and which entails electrical, mechanical, and
                         handshaking procedures.
 PNM server              One or more software application(s) for initiating PNM test and queries involving network
                         elements, acting as a server from the perspective of other PNM and OSS applications, but
                         acting as a client for network elements and measurement devices providing PNM and OSS
                         results.
 pre-3.0 DOCSIS          Versions of CableLabs Data Over Cable Service Interface Specifications (DOCSIS)
                         specifications prior to the DOCSIS 3.0 suite of specifications.
 primary service flow    All CMs have a primary upstream service flow and a primary downstream service flow. They
                         ensure that the CM is always manageable, and they provide a default path for forwarded
                         packets that are not classified to any other service flow.
 proactive network       The process and mechanism of measuring and assessing network conditions of the cable plant
 maintenance             to determine error or fault conditions before they become service impacting.
 QoS parameter set       The set of service flow encodings that describe the quality of service attributes of a service flow
                         or a service class.
 service class           A set of queuing and scheduling attributes that is named and configured at the CMTS. A service
                         class is identified by a service class name. A service class has an associated QoS parameter
                         set.




20                                              CableLabs®                                                        02/19/25
<!-- PDF_PAGE: 21 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


 service class name         An ASCII string by which a service class may be referenced in modem configuration files and
                            protocol exchanges.
 service flow               A MAC layer transport service that provides unidirectional transport of packets from the upper
                            layer service entity to the RF and shapes, polices, and prioritizes traffic according to QoS traffic
                            parameters defined for the flow.
 service flow identifier    A 32-bit identifier assigned to a service flow by the CMTS.
 (SFID)
 service identifier (SID)   A 14-bit identifier assigned by the CMTS to an active or admitted upstream service flow.
 Simple Network             A network management protocol of the IETF.
 Management Protocol
 (SNMP)
 SNMP agent                 The term "agent" is used throughout this document to refer to 1) an SNMPv1/v2 agent or 2) an
                            SNMPv3 entity [RFC 3411] that contains command responder and notification originator
                            applications.
 SNMP manager               The term "manager" is used throughout this document to refer to 1) an SNMPv1/v2 manager or
                            2) an SNMPv3 entity [RFC 3411] that contains command generator and/or notification receiver
                            applications.
 subscriber                 See end user.
 syslog                     A protocol that provides the transport of event notification messages across IP networks.
 trap                       An unconfirmed SNMP message for asynchronous notification of events from an SNMP entity.
 upstream (US)              The direction from the subscriber location toward the headend.




02/19/25                                           CableLabs®                                                                 21
