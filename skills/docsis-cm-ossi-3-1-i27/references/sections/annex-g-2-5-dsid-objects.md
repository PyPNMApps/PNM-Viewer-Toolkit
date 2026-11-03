# Annex G.2.5 - DSID Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 294-298
Classification: Normative (explicit)

<!-- PDF_PAGE: 294 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


            SfCongestionStats
The SfCongestionStats object defines congestion statistics for CM upstream service flows. The CM MUST create an
active instance of SfCongestionStats for every upstream Low Latency service flow. The CM MAY create an active
instance of SfCongestionStats for other upstream service flow types. The CM MUST delete an instance of
SfCongestionStats if the indexed service flow is deleted.
                                   Table 136 - SfCongestionStats Object Attributes

   Attribute Name           Type          Access                Type Constraints                   Units    Default
 IfIndex               InterfaceIndex    Key        Interface index of the MAC Domain interface
 ServiceFlowId         UnsignedInt       Key        1..4294967295
 SanctionedPkts        Counter64         R/O                                                      packets
 ArrivedEct0Pkts       Counter64         R/O                                                      packets
 ArrivedEct1Pkts       Counter64         R/O                                                      packets
 CeMarkedEct1Pkts      Counter64         R/O                                                      packets
 ArrivedCePkts         Counter64         R/O                                                      packets



G.2.4.7.1 IfIndex
This key represents the interface index of the MAC Domain where the upstream Service Flow has been created.

G.2.4.7.2 ServiceFlowId
This key represents the upstream Service Flow to which this instance applies.

G.2.4.7.3 SanctionedPkts
This attribute counts the number of packets redirected from the Low Latency Service Flow to the Classic Service
Flow. For other Service Flow types in the CM, this counter reports 0.

G.2.4.7.4 ArrivedEct0Pkts
This attribute reports the count of packets that arrived marked as ECT0. This attribute includes only those packets
classified into the Service Flow, not those sanctioned into the Service Flow.

G.2.4.7.5 ArrivedEct1Pkts
This attribute reports the count of packets that arrived marked as ECT1. This attribute includes only those packets
classified into the Service Flow, not those sanctioned into the Service Flow.

G.2.4.7.6 CeMarkedEct1Pkts
This attribute reports the count of packets that arrived marked as ECT1 and were re-marked as Congestion
Experienced (CE) by the CM. This attribute includes only those packets classified into the Service Flow, not those
sanctioned into the Service Flow.

G.2.4.7.7 ArrivedCePkts
This attribute reports the count of packets that arrived marked as Congestion Experienced (CE). This attribute
includes only those packets classified into the Service Flow, not those sanctioned into the Service Flow.

G.2.5      DSID Objects
This section defines Downstream Service Identifier (DSID) related objects.




294                                                 CableLabs®                                              02/19/25
<!-- PDF_PAGE: 295 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219




                                       Figure 19 - DSID Information Model


             CmDsid
This object describes the DSID information stored in the CM.
The CM reports the current status of existing DSIDs. When a DSID is created during the registration process or a
DBC transaction, a corresponding object instance is created. If a DSID is deleted or changed via a DBC message the
corresponding object instance is deleted or updated, respectively.
                                      Table 137 - CmDsid Object Attributes

          Attribute Name               Type           Access    Type Constraints            Units           Default
IfIndex                        InterfaceIndex        Key        Interface Index of
                                                                MAC Domain
                                                                interface
Dsid                           Dsid                  Key
Usage                          EnumBits              R/O        resequencing(0)
                                                                multicastCapable(1)
NumReseqChs                    UnsignedShort         R/O        0 | 1..65535
ReseqChList                    ChannelList           R/O        SIZE (0|2..255)
ReseqWaitTime                  UnsignedByte          R/O        0 | 1..180            hundredMicroseconds
ReseqWarnThrshld               UnsignedByte          R/O        0..179                hundredMicroseconds




02/19/25                                          CableLabs®                                                  295
<!-- PDF_PAGE: 296 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


          Attribute Name                  Type             Access     Type Constraints              Units           Default
StatusHldoffTimerSeqOutOfRng      UnsignedShort            R/O                              20 milliseconds
OutOfRangeDiscards                Counter32                R/O
NextExpectedSeqNum                UnsignedShort            R/O
CmInterfaceMask                   CpeInterfaceMaskType     R/O
FwdCmInterfaceMask                CpeInterfaceMaskType     R/O



G.2.5.1.1 IfIndex
This key represents the interface index of the MAC Domain associated with the DSID.

G.2.5.1.2 Dsid
This key represents the DSID.

G.2.5.1.3 Usage
This attribute indicates the properties of the DSID. The bits are defined as follows:
      •    'resequencing'
           This bit is set to 1 for a Resequencing DSID.
      •    'multicastCapable'
           This bit is set to 1 for a DSID that is capable of transporting multicast traffic (e.g., the DSID has multicast
           forwarding attributes).

G.2.5.1.4 NumReseqChs
This attribute represents the number of channels in the downstream resequencing channel list for this DSID. When a
DSID is used only for a non-bonded multicast replication, this object returns a value of 0.

G.2.5.1.5 ReseqChList
This attribute represents the Downstream Channel Set over which the DSID is being resequenced.

G.2.5.1.6 ReseqWaitTime
This attribute represents the DSID Resequencing Wait Time that is used for this DSID. This attribute is only valid
when the Usage attribute has the resequencing bit set to 1. This attribute returns a value of 0 when the Usage
attribute has the resequencing bit set to 0.

G.2.5.1.7 ReseqWarnThrshld
This attribute represents the DSID Resequencing Warning Threshold that is used for this DSID. The value of 0
indicates that the threshold warnings are disabled. This attribute is only valid when the Usage attribute has the
resequencing bit set to 1. This attribute returns a value of 0 when the Usage attribute has the resequencing bit set
to 0.

G.2.5.1.8 StatusHldoffTimerSeqOutOfRng
This attribute represents the hold-off timer for reporting Out-of-Range Events via the CM-STATUS MAC
Management message. This attribute is only valid when the Usage attribute has the resequencing bit set to 1. This
attribute returns a value of 0 when the Usage attribute has the resequencing bit set to 0.

G.2.5.1.9 OutOfRangeDiscards
This attribute represents the current count of out-of-range packets discarded by the CM for a given resequencing
context since an in-range packet was received. When this count exceeds 1000 and more than two minutes have



296                                                    CableLabs®                                                02/19/25
<!-- PDF_PAGE: 297 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


elapsed since an in-range packet was received, the CM will reacquire sequence numbers for this resequencing
context.

G.2.5.1.10 NextExpectedSeqNum
This attribute represents the Next Expected Packet Sequence Number for a given resequencing context. This
attribute is only valid when the Usage attribute has the resequencing bit set to 1. This attribute returns a value of 0
when the Usage attribute has the resequencing bit set to 0.

G.2.5.1.11 CmInterfaceMask
This attribute represents the bitmap of the interfaces communicated to the CM in a Multicast DSID encoding.

G.2.5.1.12 FwdCmInterfaceMask
This attribute represents the bitmap of the interfaces to which the CM forwards multicast traffic: a logical OR of
interfaces identified in CmInterfaceMask and interfaces associated with the client MAC addresses identified in the
instances for this DSID.

           CmDsidStats
This object defines a set of statistics the CM collects per DSID.
                                       Table 138 - CmDsidStats Object Attributes

    Attribute Name                Type          Access          Type Constraints              Units           Default
 IfIndex                   InterfaceIndex     Key
 Dsid                      Dsid               Key
 SeqNumMissing             Counter32          R/O
 SkewThreshExceeds         Counter32          R/O                                        packets
 OutOfRangePackets         Counter32          R/O                                        packets
 NumPackets                Counter64          R/O                                        packets



G.2.5.2.1 IfIndex
This key represents the interface index of the MAC Domain associated with the DSID.

G.2.5.2.2 Dsid
This key represents the DSID.

G.2.5.2.3 SeqNumMissing
This attribute counts the number of times the Next Expected Packet Sequence Number is declared lost. In this case
one or more data packets are lost. This is generally caused by downstream packet loss.
References: [MULPIv3.1] Downstream Sequencing section

G.2.5.2.4 SkewThreshExceeds
This attribute counts in-range sequenced packets which were successfully received by the CM after a wait time
longer than the Resequencing Warning Threshold.
References: [MULPIv3.1] Downstream Sequencing section

G.2.5.2.5 OutOfRangePackets
This attribute counts the number of packets Counter received in a DSID reassembly context where the sequence
number which is out of range.



02/19/25                                             CableLabs®                                                      297
<!-- PDF_PAGE: 298 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


References: [MULPIv3.1] Receive Channels section

G.2.5.2.6 NumPackets
This attribute counts the total number of data packets of a DSID context forwarded for further processing.

           CmDsidClient
This object contains the client MAC addresses that the CMTS requests that the CM uses to replicate Multicast
DSIDs during registration or during a DBC transaction.
When a DSID is created that includes client MAC addresses, or when client MAC addresses are added to a DSID,
new rows are created to indicate the added client MAC addresses. When a Client MAC address is deleted from a
DSID, the corresponding row is deleted. When a DSID is deleted, all corresponding rows are deleted, too.
References: [MULPIv3.1] DSID Encodings section in the Common Radio Frequency Interface Encodings annex
                                   Table 139 - CmDsidClient Object Attributes

         Attribute Name                   Type          Access          Type                Units            Default
                                                                      Constraints
 Dsid                              Dsid               Key
 MacId                             UnsignedShort      Key           1..65535
 MacAddr                           MacAddress         R/O



G.2.5.3.1 Dsid
This key defines the DSID with which the client MAC addresses are associated.

G.2.5.3.2 MacId
This key defines a uniquely identified client MAC address associated with the DSID.

G.2.5.3.3 MacAddr
This attribute defines a client MAC address to which multicast traffic labeled with this DSID should be forwarded.




298                                                CableLabs®                                                02/19/25
