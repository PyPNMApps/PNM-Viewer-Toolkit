# Annex G.2.4 - QoS Statistics Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 282-293
Classification: Normative (explicit)

<!-- PDF_PAGE: 282 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.3.5.2 ServiceFlowId
This key represents the Service Flow ID for the service flow.

G.2.3.5.3 Id
This key represents the identifier of the SID Cluster.
References: [MULPIv3.1] SID Cluster ID section in the Common Radio Frequency Interface Encodings annex

G.2.3.5.4 Ucid
This key represents the upstream Channel ID mapped to the corresponding SID.

G.2.3.5.5 Sid
This attribute represents the SID assigned to the upstream channel in this SID Cluster.

G.2.4    QoS Statistics Objects




                                   Figure 18 - QoS Statistics Information Model




282                                                 CableLabs®                                         02/19/25
<!-- PDF_PAGE: 283 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


           ServiceFlowStats
This object describes statistics associated with the Service Flows in a managed device.
                                   Table 130 - ServiceFlowStats Object Attributes

   Attribute Name            Type          Access                 Type Constraints                  Units     Default
 IfIndex                 InterfaceIndex   Key           Interface Index of MAC Domain interface
 ServiceFlowId           UnsignedInt      Key           1..4294967295
 Pkts                    Counter64        R/O                                                     packets
 Octets                  Counter64        R/O                                                     bytes
 TimeCreated             TimeStamp        R/O
 TimeActive              Counter32        R/O                                                     seconds
 PolicedDropPkts         Counter32        R/O                                                     packets
 PolicedDelayPkts        Counter32        R/O                                                     packets
 AqmDroppedPkts          Counter64        R/O                                                     packets



G.2.4.1.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.4.1.2 ServiceFlowId
This key represents an identifier assigned to a Service Flow by CMTS within a MAC Domain.

G.2.4.1.3 Pkts
For outgoing Service Flows, this attribute counts the number of Packet Data PDUs forwarded to this Service Flow.
For incoming upstream CMTS service flows, this attribute counts the number of Packet Data PDUs actually
received on the Service Flow identified by the SID for which the packet was scheduled. CMs not classifying
downstream packets may report this attribute's value as 0 for downstream Service Flows. This attribute does not
count MAC-specific management messages. Particularly for UGS flows, packets sent on the primary Service Flow
in violation of the UGS grant size should be counted only by the instance of this attribute that is associated with the
primary service flow. Unclassified upstream user data packets (i.e., non- MAC-management) forwarded to the
primary upstream Service Flow should be counted by the instance of this attribute that is associated with the primary
service flow. This attribute does include packets counted by ServiceFlowPolicedDelayPkts, but does not include
packets counted by ServiceFlowPolicedDropPkts. This counter's last discontinuity is the
ifCounterDiscontinuityTime for of the associated MAC Domain interface index.

G.2.4.1.4 Octets
This attribute indicates the count of the number of octets from the byte after the MAC header HCS to the end of the
CRC for all packets counted in the ServiceFlowPkts attribute for this row. Note that this counts the octets after
payload header suppression and before payload header expansion have been applied. This counter's last
discontinuity is the ifCounterDiscontinuityTime for of the associated MAC Domain interface index.

G.2.4.1.5 TimeCreated
This attribute indicates the value of sysUpTime when the service flow was created.

G.2.4.1.6 TimeActive
This attribute indicates the number of seconds that the service flow has been active. This counter's last discontinuity
is the ifCounterDiscontinuityTime for of the associated MAC Domain interface index.




02/19/25                                            CableLabs®                                                     283
<!-- PDF_PAGE: 284 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


G.2.4.1.7 PolicedDropPkts
For upstream service flows, this attribute counts the number of Packet Data PDUs classified to this service flow
dropped due to: (1) exceeding the selected Buffer Size for the service flow (see the Buffer Control section in the
Common Radio Frequency Interface Encodings Annex of [MULPIv3.1]; or (2) UGS packets dropped due to
exceeding the Unsolicited Grant Size with a Request/Transmission policy that requires such packets to be dropped.
Classified packets dropped due to other reasons must be counted in either AqmDroppedPkts or ifOutDiscards for the
interface of this service flow (depending on the reason for the discard). This attribute reports 0 for downstream
service flows. This counter's last discontinuity is the ifCounterDiscontinuityTime of the associated MAC Domain
interface index.

G.2.4.1.8 PolicedDelayPkts
This attribute counts only outgoing packets delayed in order to maintain the Maximum Sustained Traffic Rate. This
attribute will always report a value of 0 for UGS flows because the Maximum Sustained Traffic Rate does not apply.
This attribute is 0 for incoming service flows. This counter's last discontinuity is the ifCounterDiscontinuityTime of
the associated MAC Domain interface index.

G.2.4.1.9 AqmDroppedPkts
For upstream service flows on which AQM is enabled, this attribute reports the count of the number of Packet Data
PDUs classified to this service flow dropped due to Active Queue Management drop decisions. Classified packets
dropped due to other reasons are counted in either PolicedDropPkts or ifOutDiscards for the interface of this service
flow (depending on the reason for the discard). The cable modem reports zero for this attribute for downstream
service flows.
References: [MULPIv3.1] Active Queue Management Algorithm section

           AggregateServiceFlowStats
This object describes statistics associated with the Aggregate Service Flows in a managed device.
                            Table 131 - AggregateServiceFlowStats Object Attributes

      Attribute Name          Type          Access               Type Constraints                   Units   Default
 IfIndex                  InterfaceIndex   Key          Interface Index of MAC Domain interface
 AggregateServiceFlowId   UnsignedInt      Key          1..4294967295
 Pkts                     Counter64        R/O                                                    packets
 Octets                   Counter64        R/O                                                    bytes
 TimeCreated              TimeStamp        R/O
 TimeActive               Counter32        R/O                                                    seconds



G.2.4.2.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.4.2.2 AggregateServiceFlowId
This key represents an identifier assigned to a Aggregate Service Flow by CMTS within a MAC Domain.

G.2.4.2.3 Pkts
This attribute provides the sum of the Packet Data PDUs forwarded on the Low Latency and Classic Service Flows
aggregated within this ASF. This counter's last discontinuity is the ifCounterDiscontinuityTime for of the associated
MAC Domain interface index.




284                                                CableLabs®                                               02/19/25
<!-- PDF_PAGE: 285 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


G.2.4.2.4 Octets
This attribute represents the sum of the number of octets for the Low Latency and Classic Service Flows aggregated
within this ASF. For each packet, the count begins from the byte after the MAC header HCS to the end of the CRC.

G.2.4.2.5 TimeCreated
This attribute indicates the value of sysUpTime when the aggregate service flow was created.

G.2.4.2.6 TimeActive
This attribute indicates the number of seconds that the aggregate service flow has been active. This counter's last
discontinuity is the ifCounterDiscontinuityTime for of the associated MAC Domain interface index.

               DynamicServiceStats
This object describes statistics associated with the Dynamic Service Flows, Dynamic Channel Changes and
Dynamic Bonding Changes in a managed device within a MAC Domain. For each MAC Domain there are two
instances for the for the upstream and downstream direction. On the CMTS, the downstream direction instance
indicates messages transmitted or transactions originated by the CMTS. The upstream direction instance indicates
messages received or transaction originated by the CM. On the CM, the downstream direction instance indicates
messages received or transactions originated by the CMTS. The upstream direction instance indicates messages
transmitted by the CM or transactions originated by the CM.
                                 Table 132 - DynamicServiceStats Object Attributes

   Attribute Name            Type        Access                Type Constraints                  Units       Default
 IfIndex                InterfaceIndex   Key         Interface Index of MAC Domain interface
 IfDirection            IfDirection      Key
 DSAReqs                Counter32        R/O                                                   messages
 DSARsps                Counter32        R/O                                                   messages
 DSAAcks                Counter32        R/O                                                   messages
 DSCReqs                Counter32        R/O                                                   messages
 DSCRsps                Counter32        R/O                                                   messages
 DSCAcks                Counter32        R/O                                                   messages
 DSDReqs                Counter32        R/O                                                   messages
 DSDRsps                Counter32        R/O                                                   messages
 DynamicAdds            Counter32        R/O                                                   messages
 DynamicAddFails        Counter32        R/O                                                   messages
 DynamicChanges         Counter32        R/O                                                   messages
 DynamicChangeFails     Counter32        R/O                                                   messages
 DynamicDeletes         Counter32        R/O                                                   messages
 DynamicDeleteFails     Counter32        R/O                                                   messages
 DCCReqs                Counter32        R/O                                                   messages
 DCCRsps                Counter32        R/O                                                   messages
 DCCAcks                Counter32        R/O                                                   messages
 DCCs                   Counter32        R/O                                                   messages
 DCCFails               Counter32        R/O                                                   messages
 DCCRspDeparts          Counter32        R/O                                                   messages
 DCCRspArrives          Counter32        R/O                                                   messages
 DbcReqs                Counter32        R/O                                                   messages
 DbcRsps                Counter32        R/O                                                   messages
 DbcAcks                Counter32        R/O                                                   messages




02/19/25                                            CableLabs®                                                        285
<!-- PDF_PAGE: 286 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


   Attribute Name            Type          Access                 Type Constraints                   Units       Default
 DbcSuccesses            Counter32        R/O                                                     transactions
 DbcFails                Counter32        R/O                                                     transactions
 DbcPartial              Counter32        R/O                                                     transactions


G.2.4.3.1 IfIndex
This key represents the interface index of the MAC Domain.

G.2.4.3.2 IfDirection
This attribute indicates the interface direction for the instance the statistics are collected.

G.2.4.3.3 DSAReqs
This attribute indicates the number of Dynamic Service Addition Requests, including retries. Discontinuities in the
value of this counter can occur at reinitialization of the managed system, and at other times as indicated by the value
of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Addition section; [RFC 2863]

G.2.4.3.4 DSARsps
The number of Dynamic Service Addition Responses, including retries. Discontinuities in the value of this counter
can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Addition section; [RFC 2863]

G.2.4.3.5 DSAAcks
The number of Dynamic Service Addition Acknowledgements, including retries. Discontinuities in the value of this
counter can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Addition section; [RFC 2863]

G.2.4.3.6 DSCReqs
The number of Dynamic Service Change Requests, including retries. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]

G.2.4.3.7 DSCRsps
The number of Dynamic Service Change Responses, including retries. Discontinuities in the value of this counter
can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]

G.2.4.3.8 DSCAcks
The number of Dynamic Service Change Acknowledgements, including retries. Discontinuities in the value of this
counter can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]




286                                                    CableLabs®                                                02/19/25
<!-- PDF_PAGE: 287 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


G.2.4.3.9 DSDReqs
The number of Dynamic Service Delete Requests, including retries. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Deletion section; [RFC 2863]

G.2.4.3.10 DSDRsps
The number of Dynamic Service Delete Responses, including retries. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]

G.2.4.3.11 DynamicAdds
The number of successful Dynamic Service Addition transactions. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Addition section; [RFC 2863]

G.2.4.3.12 DynamicAddFails
The number of failed Dynamic Service Addition transactions. Discontinuities in the value of this counter can occur
at reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Addition section; [RFC 2863]

G.2.4.3.13 DynamicChanges
The number of successful Dynamic Service Change transactions. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]

G.2.4.3.14 DynamicChangeFails
The number of failed Dynamic Service Change transactions. Discontinuities in the value of this counter can occur at
reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Change section; [RFC 2863]

G.2.4.3.15 DynamicDeletes
The number of successful Dynamic Service Delete transactions. Discontinuities in the value of this counter can
occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Delete section; [RFC 2863]

G.2.4.3.16 DynamicDeleteFails
The number of failed Dynamic Service Delete transactions. Discontinuities in the value of this counter can occur at
reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Service Delete section; [RFC 2863]



02/19/25                                           CableLabs®                                                    287
<!-- PDF_PAGE: 288 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.4.3.17 DCCReqs
The number of Dynamic Channel Change Request messages traversing an interface. This count is nonzero only on
downstream direction rows. This count should include the number of retries. Discontinuities in the value of this
counter can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.18 DCCRsps
The number of Dynamic Channel Change Response messages traversing an interface. This count is nonzero only on
upstream direction rows. This count should include the number of retries. Discontinuities in the value of this counter
can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.19 DCCAcks
The number of Dynamic Channel Change Acknowledgement messages traversing an interface. This count is
nonzero only on downstream direction rows. This count should include the number of retries. Discontinuities in the
value of this counter can occur at reinitialization of the managed system, and at other times as indicated by the value
of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.20 DCCs
The number of successful Dynamic Channel Change transactions. This count is nonzero only on downstream
direction rows. Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at
other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.21 DCCFails
The number of failed Dynamic Channel Change transactions. This count is nonzero only on downstream direction
rows. Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other
times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.22 DccRspDeparts
This attribute contains the number of Dynamic Channel Change Response (depart) messages. It only applies to
upstream direction. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]

G.2.4.3.23 DccRspArrives
This attribute contains the number of Dynamic Channel Change Response (arrive) messages and should include
retries. It only applies to the upstream direction. Discontinuities in the value of this counter can occur at
reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Downstream and/or Upstream Channel Changes section; [RFC 2863]




288                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 289 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


G.2.4.3.24 DbcReqs
This attribute contains the number of Dynamic Bonding Change Requests, including retries. It only applies to the
upstream direction. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

G.2.4.3.25 DbcRsps
This attribute contains the number of Dynamic Bonding Change Responses, including retries. It only applies to the
upstream direction. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

G.2.4.3.26 DbcAcks
This attribute contains the number of Dynamic Bonding Change Acknowledgements, including retries. It only
applies to the downstream direction. Discontinuities in the value of this counter can occur at reinitialization of the
managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC
Domain interface index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

G.2.4.3.27 DbcSuccesses
This attribute contains the number of fully successful Dynamic Bonding Change transactions. It only applies to the
downstream direction and does not include DBC transactions that result in Partial Service. Discontinuities in the
value of this counter can occur at reinitialization of the managed system, and at other times as indicated by the value
of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

G.2.4.3.28 DbcFails
This attribute contains the number of failed Dynamic Bonding Change transactions. It only applies to the
downstream direction. Note that Partial Service is not considered a failed transaction. Discontinuities in the value of
this counter can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

G.2.4.3.29 DbcPartial
This attribute contains the number of unsuccessful Dynamic Bonding Change transactions that result in Partial
Service. IT only applies to the downstream direction. Discontinuities in the value of this counter can occur at
reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Dynamic Bonding Change (DBC) section; [RFC 2863]

           CmServiceUsStats
This object defines DOCSIS MAC services primitive statistics of upstream service flows.
References: [MULPIv3.1] Upstream Data Transmission section




02/19/25                                            CableLabs®                                                     289
<!-- PDF_PAGE: 290 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


                                   Table 133 - CmServiceUsStats Object Attributes

  Attribute Name           Type           Access                  Type Constraints                 Units       Default
IfIndex                InterfaceIndex   Key            Interface Index of MAC Domain interface
ServiceFlowId          UnsignedInt      Key            1..4294967295
TxSlotsImmed           Counter32        R/O                                                      minislots
TxSlotsDed             Counter32        R/O                                                      minislots
TxRetries              Counter32        R/O                                                      attempts
TxExceededs            Counter32        R/O                                                      attempts
RqRetries              Counter32        R/O                                                      attempts
RqExceededs            Counter32        R/O                                                      attempts
Sgmts                  Counter32        R/O                                                      segments



G.2.4.4.1 IfIndex
This key represents the interface index of the MAC Domain to which this instance applies.

G.2.4.4.2 ServiceFlowId
This key represents the Service Flow ID for the service flow.
References: [MULPIv3.1] QoS section

G.2.4.4.3 TxSlotsImmed
This attribute contains the number of upstream minislots which have been used to transmit data PDUs in immediate
(contention) mode. This includes only those PDUs that are presumed to have arrived at the headend (i.e., those
which were explicitly acknowledged.) It does not include retransmission attempts or minislots used by Requests.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Upstream Bandwidth Allocation section; [RFC 2863]

G.2.4.4.4 TxSlotsDed
This attribute contains the number of upstream minislots which have been used to transmit data PDUs in dedicated
mode (i.e., as a result of a unicast Data Grant). Discontinuities in the value of this counter can occur at
reinitialization of the managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime
of the associated MAC Domain interface index.
References: [MULPIv3.1] Upstream Data Transmission section; [RFC 2863]

G.2.4.4.5 TxRetries
This attribute contains the number of attempts to transmit data PDUs containing requests for acknowledgment that
did not result in acknowledgment. Discontinuities in the value of this counter can occur at reinitialization of the
managed system, and at other times as indicated by the value of ifCounterDiscontinuityTime for the associated
MAC Domain interface index.
References: [MULPIv3.1] Upstream Bandwidth Allocation section; [RFC 2863]

G.2.4.4.6 TxExceededs
This attribute contains the number of data PDUs transmission failures due to excessive retries without
acknowledgment. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.




290                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 291 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Upstream Bandwidth Allocation section; [RFC 2863]

G.2.4.4.7 RqRetries
This attribute contains the number of attempts to transmit bandwidth requests which did not result in
acknowledgment. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.
References: [MULPIv3.1] Upstream Bandwidth Allocation section; [RFC 2863]

G.2.4.4.8 RqExceededs
This attribute contains the number of requests for bandwidth which failed due to excessive retries without
acknowledgment. Discontinuities in the value of this counter can occur at reinitialization of the managed system,
and at other times as indicated by the value of ifCounterDiscontinuityTime of the associated MAC Domain interface
index.
References: [MULPIv3.1] Upstream Bandwidth Allocation section; [RFC 2863]

G.2.4.4.9 Sgmts
This attribute contains the number of segments transmitted on this service flow. Discontinuities in the value of this
counter can occur at reinitialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime of the associated MAC Domain interface index.
References: [MULPIv3.1] Upstream and Downstream Common Aspects section; [RFC 2863]

           SfLatencyHistCfg
This object defines the latency histogram structure for each upstream histogram enabled service flow. The CM
MUST create an active instance of this object for every upstream service flow for which histogram calculation is
enabled via the Latency Histogram Encodings TLV. The activation of an instance of this object enables histogram
calculation for the indexed service flow. The deactivation or deletion of an instance of this object disables histogram
calculation for the indexed service flow. The CM MUST delete an instance of this object if the indexed service flow
is deleted (e.g., because of a DSD operation) or the histogram calculation is disabled via the Latency Histogram
Encodings TLV (i.e., because of a DSC operation).
Prior to deactivating or deleting an instance of the SfLatencyHistCfg object, the CM MUST, for the indexed service
flow, conclude any Latency Reporting Snapshot in progress (see Section D.3), and delete the corresponding instance
in the SfLatencyStats object. Once the SfLatencyHistCfg object is deactivated or deleted the CM does not collect
any further latency statistics. The CM MUST NOT persist instances created in the SfLatencyHistCfg object across
reinitializations.
Bins are, by definition, consecutively numbered from one to sixteen. At most fifteen upper edges can be configured.
Each configured upper edge is also the lower edge for the next bin (e.g., Bin 5 lower edge is defined by
LatencyBin4UpperEdge). The lower edge of Bin 1 is defined as zero, and the upper edge of the final active bin (i.e.,
the bin corresponding to LatencyBinEdgeNum + 1) is defined as infinity. The CM validates the configured bin edges
prior to activation.
The Service Flow Latency Estimate values corresponding to Bin N are bounded as follows: Bin N lower edge ≤
Service Flow Latency Estimate < Bin N upper edge.
References: [MULPIv3.1] Latency Histogram Calculation section
                                  Table 134 - SfLatencyHistCfg Object Attributes

   Attribute Name             Type          Access               Type Constraints                       Units   Default
IfIndex                  InterfaceIndex    Key          Interface index of MAC Domain Interface   N/A
ServiceFlowId            UnsignedInt       Key          1..4294967295
SfLabel                  String            R/W          SIZE (0..15)                                            'unknown'




02/19/25                                            CableLabs®                                                      291
<!-- PDF_PAGE: 292 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


   Attribute Name             Type         Access               Type Constraints                 Units       Default
LatencyBinEdgeNum        UnsignedByte     R/W          0..15                                                0
LatencyBin1UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin2UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin3UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin4UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin5UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin6UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin7UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin8UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin9UpperEdge     UnsignedInt      R/W                                                microseconds   0
LatencyBin10UpperEdge    UnsignedInt      R/W                                                microseconds   0
LatencyBin11UpperEdge    UnsignedInt      R/W                                                microseconds   0
LatencyBin12UpperEdge    UnsignedInt      R/W                                                microseconds   0
LatencyBin13UpperEdge    UnsignedInt      R/W                                                microseconds   0
LatencyBin14UpperEdge    UnsignedInt      R/W                                                microseconds   0
LatencyBin15UpperEdge    UnsignedInt      R/W                                                microseconds   0



G.2.4.5.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.4.5.2 ServiceFlowId
This key represents the Service Flow enabled to calculate queue latency histograms.

G.2.4.5.3 SfLabel
This attribute represents the Service Class Name or a Label (configured by the operator) to be used in Service Flow
Latency Performance Reporting (see Section D.3). The CM SHOULD default this attribute to the SCN if it's known.
If the SCN is not known, the CM MUST default this attribute to the value "unknown". The Operator can set this
attribute to any string.

G.2.4.5.4 LatencyBinEdgeNum
This attribute represents the number of consecutive configured upper bin edges. The number of histogram bins is
then LatencyBinEdgeNum + 1.

G.2.4.5.5 LatencyBin1UpperEdge to LatencyBin15UpperEdge
These attributes represent the upper edges of each bin, bins 1 through 15. The histogram calculation behavior is
undefined if configured upper bin edges are not monotonically increasing beginning with the
LatencyBin1UpperEdge (e.g., if for bin number 'n', LatencyBin(n)UpperEdge ≥ LatencyBin(n+1)UpperEdge).

          SfLatencyStats
This object contains the latency statistics calculated by the CM for each histogram enabled upstream service flow.
The SfLatencyStats object entry describes latency statistics (histogram of latencies, Max Latency, Sanctioned
packets, etc.) at the CM for the upstream service flow. The CM MUST create an instance for each ServiceFlowId
value for which histogram calculation is enabled. The CM MUST delete the instance when histogram calculation is
disabled for the service flow.
The CM MUST NOT delete SfLatencyStats instances while the corresponding SfLatencyHistCfg instance is active.
The CM MUST NOT persist instances created in the SfLatencyStats object across reinitializations.



292                                                CableLabs®                                               02/19/25
<!-- PDF_PAGE: 293 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1]
                                   Table 135 - SfLatencyStats Object Attributes

    Attribute Name                       Type               Access          Type               Units            Default
                                                                          Constraints
IfIndex                    InterfaceIndex                 Key            Interface index
                                                                         of MAC Domain
                                                                         Interface
ServiceFlowId              UnsignedInt                    Key            1..4294967295
MaxLatency                 UnsignedInt                    R/O                              microseconds     -
NumHistUpdates             Counter64                      R/O                              -                -
Bin1PktCount               Counter64                      R/O                              packets
Bin2PktCount               Counter64                      R/O                              packets
Bin3PktCount               Counter64                      R/O                              packets
Bin4PktCount               Counter64                      R/O                              packets
Bin5PktCount               Counter64                      R/O                              packets
Bin6PktCount               Counter64                      R/O                              packets
Bin7PktCount               Counter64                      R/O                              packets          -
Bin8PktCount               Counter64                      R/O                              packets          -
Bin9PktCount               Counter64                      R/O                              packets          -
Bin10PktCount              Counter64                      R/O                              packets          -
Bin11PktCount              Counter64                      R/O                              packets          -
Bin12PktCount              Counter64                      R/O                              packets          -
Bin13PktCount              Counter64                      R/O                              packets          -
Bin14PktCount              Counter64                      R/O                              packets          -
Bin15PktCount              Counter64                      R/O                              packets          -
Bin16PktCount              Counter64                      R/O                              packets          -



G.2.4.6.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.4.6.2 ServiceFlowId
This key represents the Service Flow to which this instance applies.

G.2.4.6.3 MaxLatency
This attribute represents maximum latency observed.

G.2.4.6.4 NumHistUpdates
This attribute represents count of updates to the queue latency histogram. In cases where a latency estimate is not
generated for every packet (e.g., subsampling in the case of Low Latency Service Flow or estimates every update
interval in the case of Classic Service Flow) this attribute provides information regarding the fidelity of the
histogram bin counts. In the case of a Classic Service Flow, this counter will only increment if packets have been
enqueued during the last update interval.

G.2.4.6.5 Bin1PktCount to Bin16PktCount
These attributes represent the count of packets whose latency falls within each histogram Bin. If the number of bin
edges configured is 'n' then only the first 'n+1' bin counts are valid. The CM MUST report zero for Bin'X'PktCount
for values of 'X' greater than n+1.



02/19/25                                            CableLabs®                                                      293
