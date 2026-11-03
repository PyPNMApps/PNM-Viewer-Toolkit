# 3 - Terms and Definitions

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 23-28
Classification: Main body (definitions)

<!-- PDF_PAGE: 23 -->
Cable Modem Operations Support System Interface Specification                          CM-SP-CM-OSSIv4.0-I13-260429


3 TERMS AND DEFINITIONS
This specification uses the following terms and definitions.
  active queue management       AQM schemes attempt to maintain low queue occupancy (within downstream and upstream
                                service flows) while supporting the ability to absorb a momentary traffic burst.
  allocation                    A group of contiguous minislots in a MAP that constitute a single transmit opportunity.
  backup primary                A primary-capable downstream channel that is assigned to this CM as a non-primary
  downstream channel            downstream channel but is designated to become the new primary downstream channel if the
                                currently assigned primary downstream channel is no longer usable by this CM.
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
  CM-to-CM co-channel           Interference caused by two or more different transmitters operating in the same channel, without
  interference (CCI)            proper channel access control or scheduling, leading to decreases in throughput for devices
                                operating in the channel.
  Complete Transmit             For a CM, the combination of upstream channels in its transmit channel set and in its extended
  Channel Set                   transmit channel set. Pre-DOCSIS 4.0 CMs do not have an extended transmit channel set. In
                                other words, their complete transmit channel set consists only of upstream channels in their
                                transmit channel set.
                                A CM's upstream service flows may be associated with some or all of the channels in the
                                complete transmit channel set.
  customer                      See end user.
  customer premises             Equipment at the end user's premises; may be provided by the end user or the service provider.
  equipment (CPE)
  data model                    A data model (as opposed to an information model) is defined at a lower level of abstraction, is
                                intended for implementations, and includes protocol-specific constructs. Since conceptual
                                models can be implemented in different ways, multiple data models can be derived from a single
                                information model. Data models are technology specific. The CM has defined data models for
                                SNMP as SNMP MIB modules.
  DOCSIS 4.0 cable modem        A DOCSIS 4.0 CM supports DOCSIS 4.0 requirements and reports FDD capability, FDX
                                capability, or both of these capabilities.
  downstream (DS)               In cable television, the direction of transmission from the headend to the subscriber.
  downstream channel            Physical layer characteristics and MAC layer parameters and functions associated with a
                                DOCSIS forward channel.
  echo cancellation             A process by which an FDX CM receiver's performance is improved by canceling out adjacent
                                leakage interference (ALI) and adjacent channel interference (ACI) resulting from concurrent
                                upstream transmissions.
  end user                      A human being, organization, or telecommunications system that accesses a network in order to
                                communicate via the services provided by the network (also known as customer).
  extended transmit channel     The set of extended upstream channels that a DOCSIS 4.0 CM is configured to use for
  set                           upstream transmission. The extended transmit channel set (TCS_EXT) is alternatively referred
                                to as the FDX transmit channel set (TCS_FDX) for FDX CMs in an FDX band plan and as the
                                FDD transmit channel set (TCS_FDD) for FDD CMs in an FDD band plan.
                                See also transmit channel set and complete transmit channel set.




04/29/26                                              CableLabs                                                              23
<!-- PDF_PAGE: 24 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


 extended upstream           An OFDMA upstream channel present above 108 MHz in an FDX band plan or in an FDD UHS
 channel                     band plan. In an FDX band plan, extended upstream channels exist only in the FDX allocated
                             spectrum of the 108 MHz to 684 MHz FDX band. In an FDD band plan, extended upstream
                             channels exist only between 108 MHz and the UHS upstream upper band edge. An extended
                             upstream channel's bandwidth is always 96 MHz, as specified in [PHYv4.0].
                             Extended upstream channel has meaning only when the plant is operating with a UHS or FDX
                             band plan. Otherwise, channels used for upstream transmission are referred to as upstream
                             channels.
 FCAPS                       A set of principles for managing networks and systems wherein each letter represents one
                             principle. F is for fault, C is for configuration, A is for accounting, P is for performance, and S is
                             for security.
 FDD extended upstream       An extended upstream channel in an FDD band plan.
 channel
 FDD mode                    See frequency division duplex mode.
 FDX extended upstream       An extended upstream channel in an FDX band plan.
 channel
 FDX mode                    A DOCSIS 4.0 mode of operation in which a full duplex CM is configured for full duplex operation
                             on a full duplex plant.
 fiber node                  A point of interface between a fiber trunk and the coaxial distribution.
 frequency division duplex   A band plan where a given band of spectrum is used for either upstream or downstream
 (FDD)                       transmission.
 frequency division duplex   A diplexer-based band plan with an upstream/downstream split as per the ultra-high split band
 band plan                   plans.
 frequency division duplex   A DOCSIS 4.0 CM that is designed to operate in an FDD band plan. An FDD CM can access
 cable modem                 upstream channels below the upstream/downstream split and downstream channels above the
                             upstream/downstream split. An FDD CM can access all such upstream channels and
                             downstream channels in a high-split band plan. An FDD CM can access all such upstream and
                             downstream channels in a UHS band plan, with the exception that the FDD CM is not required to
                             be able to access upstream channels between 85 MHz and 108 MHz.
 frequency division duplex   A DOCSIS 4.0 CMTS that is designed to operate in an FDD band plan. An FDD CMTS can
 cable modem termination     access all upstream channels below the upstream/downstream split, and all downstream
 system                      channels above the upstream/downstream split. An FDD CMTS supports mid-split, high-split,
                             and UHS band plans.
 frequency division duplex   An OFDM downstream channel in an FDD band plan.
 downstream channel
 frequency division duplex   A DOCSIS 4.0 mode of operation in which a frequency division duplex cable modem is
 mode                        configured to operate on a frequency division duplex plant.
 full duplex allocated       The portion of the full duplex band that the access network allocates for FDX operation, whether
 spectrum                    that spectrum is currently in use or not by the FDX node receiver or any full duplex cable
                             modems. Five values are defined for FDX-allocated spectrum: 96 MHz, 192 MHz, 288 MHz, 384
                             MHz, and 576 MHz.
 full duplex band            Always 108 to 684 MHz. Contiguous range of RF spectrum defined in [PHYv4.0] and configured
                             for full duplex operation. Any given access network may operate only a strict subset of the full
                             duplex band in full duplex operation (see also full duplex allocated spectrum).
 full duplex band plan       A band plan where only upstream channels are present up to an 85 MHz upstream upper band
                             edge, the full duplex band exists from 108 to 684 MHz (which can have upstream and
                             downstream channels), and only downstream channels exist above 684 MHz.
 full duplex cable modem     A cable modem compliant to the full duplex specific requirements of the DOCSIS 4.0
                             specifications. A full duplex cable modem can access the full duplex channel when it is used in
                             the upstream direction or when it is used in the downstream direction.
 full duplex cable modem     A DOCSIS 4.0 CMTS that is designed to operate in an FDX band plan. An FDX CMTS is
 termination system          compliant with all FDX-specific requirements of the DOCSIS 4.0 specifications. An FDX CMTS
                             can access upstream channels below the FDX band, full duplex channels within the full duplex
                             allocated spectrum of the FDX band, and downstream channels above the FDX allocated
                             spectrum.
 full duplex capable cable   Full duplex capable (FDX-capable) cable modem refers to both the full duplex cable modem and
 modem                       the full duplex limited cable modem.
 full duplex limited cable   A DOCSIS 3.1 cable modem compliant to all of the requirements in the DOCSIS 3.1
 modem                       specification, as well as full duplex requirements in the DOCSIS 4.0 specification that support
                             transmit-only on a specified full duplex sub-band and/or receive-only on different specified full
                             duplex sub-bands.




24                                                  CableLabs                                                           04/29/26
<!-- PDF_PAGE: 25 -->
Cable Modem Operations Support System Interface Specification                         CM-SP-CM-OSSIv4.0-I13-260429


 full duplex channel          A downstream OFDM channel or upstream OFDMA channel within the full duplex band
                              configured for full duplex operation.
 full duplex DOCSIS           A mode of operations within the DOCSIS 4.0 specification that is targeted at significantly
                              increasing upstream capacity by using the spectrum currently used for downstream transmission
                              for simultaneous upstream and downstream communications via full duplex communications.
 full duplex downstream       An OFDM channel in the occupied full duplex band. A full duplex downstream channel's
 channel                      bandwidth can be 96 MHz or 192 MHz, as specified in [PHYv4.0].
 full duplex mode             A DOCSIS 4.0 mode of operation in which a full duplex cable modem is configured for full duplex
                              operation on a full duplex plant.
 full duplex sub-band         A portion of the electromagnetic spectrum within the occupied full duplex band that contains only
                              full duplex channels. An FDX duplex sub-band always contains a single full duplex downstream
                              channel. An FDX duplex sub-band always contains either one or two full duplex upstream
                              channels.
 full duplex upstream         An OFDMA channel in the occupied full duplex band. A full duplex upstream channel's
 channel                      bandwidth is 96 MHz, as specified in [PHYv4.0].
 high split                   A band plan where there is an upstream/downstream split at a 204 MHz upstream upper band
                              edge and a 258 MHz lower downstream band edge.
 hybrid fiber-coax (HFC)      A broadband bidirectional shared-media transmission system using fiber trunks between the
 system                       headend and the fiber nodes, and coaxial distribution from the fiber nodes to the customer
                              locations.
 inform                       A confirmed SNMP message for asynchronous notification of events from an SNMP entity.
 information model            An information model (as opposed to a data model) is an abstraction and only provides a high
                              level view of things of interest (e.g., information) to the business. It aids in understanding the
                              scope and breadth of the business, rather than the depth. An information model is a way of
                              representing and structuring information that has advantages over other common artifacts such
                              as a glossary, descriptive document, database, or source code. A common information model
                              will streamline the processes associated with information exchange, both within a business (e.g.,
                              Enterprise) and between a business and its external stakeholders.
 interference group           A group of cable modems with active channels in the full duplex band that are susceptible to
                              interfering with one another. The CMTS uses sounding to determine interference groups that
                              are, in turn, mapped into transmission groups for resource block assignment. An interference
                              group is part of a transmission group that non-overlapping downstream and upstream channels
                              are allocated to avoid the upstream-to-downstream interference among cable modems in the
                              same interference group.
 International Organization   An international standards body commonly known as the International Standards Organization.
 for Standardization (ISO)
 local log                    A volatile or non-volatile log stored within a network element.
 logical upstream channel     A MAC entity identified by a unique channel ID and for which bandwidth is allocated by an
                              associated MAP message. A physical upstream channel may support multiple logical upstream
                              channels. The associated UCD and MAP messages completely describe the logical channel.
 Low Latency DOCSIS           A set of tools for providing control, status, and statistics of packet queuing delays incurred by the
                              processing of packets through the forwarding plane of the system referred to as Low Latency
                              Support as defined in [MULPIv4.0].
 low split                    A band plan where there is an upstream/downstream split at a 42 MHz upstream upper band
                              edge and a 108 MHz lower downstream band edge. This is also referred to as the extended-
                              subsplit in the [PHYv4.0].
 MAC domain                   A subcomponent of the CMTS that provides data forwarding services to a set of downstream
                              and upstream channels.
 MAC domain downstream        The subset of a downstream service group (DS-SG) that is confined to the downstream
 service group                channels of a single MAC domain. An MD-DS-SG differs from a DS-SG only when multiple MAC
                              domains are configured per DS-SG.
 MAC domain upstream          The subset of an upstream service group (US-SG) that is confined to the upstream channels of a
 service group                single MAC domain. An MD-US-SG differs from a US-SG only when multiple MAC domains are
                              defined per US-SG.
 media access control         The "built-in" hardware address of a device connected to a shared medium.
 (MAC) address
 micro-reflections            Echoes in the forward or reverse transmission path because of impedance mismatches between
                              the physical plant components. Micro-reflections are distinguished from discrete echoes by
                              having a time difference (between the main signal and the echo) on the order of 1 microsecond.
                              Micro-reflections cause departures from ideal amplitude and phase characteristics for the
                              transmission channel.




04/29/26                                             CableLabs                                                                25
<!-- PDF_PAGE: 26 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


 mid-split                   A band plan where there is an upstream/downstream split at an 85 MHz upstream upper band
                             edge and a 108 MHz lower downstream band edge.
 minislot                    An integer multiple of 6.25-microsecond increments.
 network management          The functions related to the management of data link layer and physical layer resources and
                             their stations across the data network supported by the hybrid fiber-coax system.
 network management          The hardware and software components used by a network provider to manage its networks as
 system (NMS)                a whole. The network management system provides an end-to-end network view of the entire
                             network, enabling management of the network elements contained in the network.
 non-extended upstream       An upstream channel present below 108 MHz in an FDD UHS band plan or FDX band plan. An
 channel                     FDD CM is not required to support upstream channels between 85 MHz and 108 MHz in a UHS
                             band plan. An FDX CM is not required to support upstream channels between 85 MHz and 108
                             MHz.
                             Non-extended upstream channel has meaning only when the plant is operating with a UHS or
                             FDX band plan. Otherwise, channels used for upstream transmission are referred to as
                             upstream channels.
 non-primary downstream      A downstream channel received by a cable modem that is not its primary downstream channel.
 channel
 notification                Information emitted by a managed object relating to an event that has occurred within the
                             managed object.
 occupied bandwidth          1) Downstream - The sum of the bandwidth in all standard channel frequency allocations (e.g.,
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
                             with a skip value of zero is equal to the upstream occupied bandwidth [PHYv4.0].
 occupied full duplex band   This term is used interchangeably with FDX allocated spectrum and defines the spectrum in an
                             access network that is allocated to an FDX band, including guard bands, whether it is used for
                             full duplex or not.
 Open Systems                A framework of ISO standards for communication between different systems made by different
 Interconnection (OSI)       vendors in which the communications process is organized into seven different categories that
                             are placed in a layered sequence based on their relationship to the user. Each layer uses the
                             layer immediately below it and provides a service to the layer above. Layers 7 through 4 deal
                             with end-to-end communication between the message source and destination, and layers 3
                             through 1 deal with network functions.
 partial service             A modem is in a partial service mode of operation any time it is operating with a subset of the
                             channels in the RCS and/or TCS because a channel has become unusable, either due to an
                             inability to acquire a channel or because communication on a channel was lost during normal
                             operation.
 physical (PHY) layer        Layer 1 in the OSI architecture; the layer that provides services to transmit bits or groups of bits
                             over a transmission link between open systems and which entails electrical, mechanical, and
                             handshaking procedures.
 PNM server                  One or more software application(s) for initiating PNM test and queries involving network
                             elements, acting as a server from the perspective of other PNM and OSS applications, but
                             acting as a client for network elements and measurement devices providing PNM and OSS
                             results.
 pre-3.0 DOCSIS              Versions of CableLabs Data Over Cable Service Interface Specifications (DOCSIS)
                             specifications prior to the DOCSIS 3.0 suite of specifications.
 pre-3.1 DOCSIS              Versions of CableLabs Data-Over-Cable Service Interface Specifications (DOCSIS)
                             specifications prior to the DOCSIS 3.1 suite of specifications.
 pre-4.0 DOCSIS              Versions of CableLabs Data-Over-Cable Service Interface Specifications (DOCSIS)
                             specifications prior to the DOCSIS 4.0 suite of specifications
 primary downstream          Prior to registration, a primary-capable downstream channel on which the CM has achieved
 channel                     timing lock and successfully received an MDD message containing ambiguity resolution TLVs.
                             After registration, the channel on which the CM acquires timing from the assigned list of primary
                             downstream channels in the simplified RCC encodings.




26                                                  CableLabs                                                        04/29/26
<!-- PDF_PAGE: 27 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


 primary service flow       All CMs have a primary upstream service flow and a primary downstream service flow. They
                            ensure that the CM is always manageable, and they provide a default path for forwarded
                            packets that are not classified to any other service flow.
 primary-capable            A downstream channel that carries timestamp information (SYNC messages for SC-QAM or
 downstream channel         timestamp message block and explicit primary-capability indicator for OFDM), MDD messages
                            containing ambiguity resolution TLVs, and UCD and MAP messages for at least one upstream
                            channel in each of the MD-CM-SGs that the downstream channel reaches.
 proactive network          The process and mechanism of measuring and assessing network conditions of the cable plant
 maintenance                to determine error or fault conditions before they become service impacting.
 QoS parameter set          The set of service flow encodings that describe the quality of service attributes of a service flow
                            or a service class.
 RBA sub-band direction     The set of all active FDX sub-bands and the associated direction for those sub-bands. Because
 set                        of RBA expiration times, there may be RBA messages with sequential change counts that
                            specify the same set of sub-band directions. The term RBA sub-band direction set is used to
                            describe the directions contained in an RBA message to distinguish those directions from the
                            RBA message. The CM and CMTS maintain ECT state on a per RBA sub-band direction set
                            basis. The RBA sub-band direction set is independent of the assigned TG ID.
 receive channel set        The set of downstream channels assigned to an individual CM, explicitly configured by the
                            CMTS using the RCC encodings.
 resource block             The set of sub-bands of the full duplex active spectrum assigned to a transmission group of
                            FDX-capable cable modems. A resource block has fixed configured boundaries and the
                            capability to be dynamically assigned by the CMTS to any of a set of upstream or downstream
                            combinations to satisfy network traffic demand and the service provider's business objectives.
 resource block             Assignment of a resource block to upstream or downstream operation.
 assignment
 service class              A set of queuing and scheduling attributes that is named and configured at the CMTS. A service
                            class is identified by a service class name. A service class has an associated QoS parameter
                            set.
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
 sounding                   A testing process performed by the FDX CMTS to assess the co-channel interference (CCI)
                            level between any CM pair that may share the same spectrum for FDX operation and is
                            performed during interference group (IG) discovery.
 subscriber                 See end user.
 syslog                     A protocol that provides the transport of event notification messages across IP networks.
 transmission group         A logical grouping of cable modems using the full duplex band, formed by the CMTS for the
                            purpose of preventing transmissions from a cable modem from interfering with cable modems
                            receiving in a downstream channel at the same time.




04/29/26                                           CableLabs                                                                 27
<!-- PDF_PAGE: 28 -->
CM-SP-CM-OSSIv4.0-I13-260429                              Data-Over-Cable Service Interface Specifications


 transmit channel set   The set of upstream channels that a pre-DOCSIS 4.0 CM is configured to use for upstream
                        transmission.
                        The set of upstream channels that an FDD CM is configured to use for upstream transmission in
                        a non-UHS band plan.
                        The set of non-extended upstream channels that an FDD CM is configured to use for upstream
                        transmission in a UHS band plan.
                        The set of non-extended upstream channels that an FDX CM is configured to use for upstream
                        transmission in an FDX band plan.
                        See also extended transmit channel set and complete transmit channel set.
 trap                   An unconfirmed SNMP message for asynchronous notification of events from an SNMP entity.
 ultra-high split       An FDD band plan where there is an upstream/downstream split at a 300, 396, 492, or 684 MHz
                        upstream upper band edge. The typical maximum corresponding downstream lower band edges
                        are at 372, 492, 588, or 834 MHz, respectively.
 upstream (US)          The direction from the subscriber location toward the headend.
 upstream channel       Physical layer characteristics and MAC layer parameters and functions associated to a DOCSIS
                        reverse channel.




28                                           CableLabs                                                   04/29/26
