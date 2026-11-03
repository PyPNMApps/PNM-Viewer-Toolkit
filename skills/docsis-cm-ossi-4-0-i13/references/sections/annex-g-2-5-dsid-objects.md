# Annex G.2.5 - DSID Objects

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 341-344
Classification: Normative (explicit)

<!-- PDF_PAGE: 341 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


                                   Table 183 - SfCongestionStats Object Attributes

   Attribute Name           Type          Access               Type Constraints                     Units   Default
 IfIndex               InterfaceIndex    Key        Interface index of the MAC Domain interface
 ServiceFlowId         UnsignedInt       Key        1.. 4294967295
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




04/29/26                                           CableLabs                                                    341
<!-- PDF_PAGE: 342 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications




                                        Figure 20 - DSID Information Model


             CmDsid
This object describes the DSID information stored in the CM.
The CM reports the current status of existing DSIDs. When a DSID is created during the registration process or a
DBC transaction, a corresponding object instance is created. If a DSID is deleted or changed via a DBC message the
corresponding object instance is deleted or updated respectively.
                                      Table 184 - CmDsid Object Attributes

          Attribute Name               Type          Access       Type Constraints               Units           Default
IfIndex                        InterfaceIndex       Key          Interface Index of MAC
                                                                 Domain interface
Dsid                           Dsid                 Key
Usage                          EnumBits             R/O          resequencing(0)
                                                                 multicastCapable(1)
NumReseqChs                    UnsignedShort        R/O          0 | 1..65535
ReseqChList                    ChannelList          R/O          SIZE (0|2..255)
ReseqWaitTime                  UnsignedByte         R/O          0 | 1..180               hundredMicroseconds
ReseqWarnThrshld               UnsignedByte         R/O          0..179                   hundredMicroseconds
StatusHldoffTimerSeqOutOfRng   UnsignedShort        R/O                                   20 milliseconds



342                                               CableLabs                                                    04/29/26
<!-- PDF_PAGE: 343 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


        Attribute Name                   Type             Access      Type Constraints              Units          Default
OutOfRangeDiscards               Counter32               R/O
NextExpectedSeqNum               UnsignedShort           R/O
CmInterfaceMask                  CpeInterfaceMaskType R/O
FwdCmInterfaceMask               CpeInterfaceMaskType R/O



G.2.5.1.1 IfIndex
This key represents the interface index of the MAC Domain associated with the DSID.

G.2.5.1.2 Dsid
This key represents the DSID.

G.2.5.1.3 Usage
This attribute indicates the properties of the DSID. The bits are defined as follows.
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
resequencing bit set to 1. This attribute returns a value of 0 when the Usage attribute has the resequencing bit set to 0.

G.2.5.1.8 StatusHldoffTimerSeqOutOfRng
This attribute represents the hold-off timer for reporting Out-of-Range Events via the CM-STATUS MAC
Management message. This attribute is only valid when the Usage attribute has the resequencing bit set to 1. This
attribute returns a value of 0 when the Usage attribute has the resequencing bit set to 0.

G.2.5.1.9 OutOfRangeDiscards
This attribute represents the current count of out-of-range packets discarded by the CM for a given resequencing
context since an in-range packet was received. When this count exceeds 1000 and more than two minutes have elapsed
since an in-range packet was received, the CM will reacquire sequence numbers for this resequencing context.




04/29/26                                              CableLabs                                                        343
<!-- PDF_PAGE: 344 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


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
                                     Table 185 - CmDsidStats Object Attributes

          Attribute Name                   Type          Access         Type Constraints           Units        Default
IfIndex                             InterfaceIndex     Key
Dsid                                Dsid               Key
SeqNumMissing                       Counter32          R/O
SkewThreshExceeds                   Counter32          R/O                                      packets
OutOfRangePackets                   Counter32          R/O                                      packets
NumPackets                          Counter64          R/O                                      packets



G.2.5.2.1 IfIndex
This key represents the interface index of the MAC Domain associated with the DSID.

G.2.5.2.2 Dsid
This key represents the DSID.

G.2.5.2.3 SeqNumMissing
This attribute counts the number of times the Next Expected Packet Sequence Number is declared lost. In this case
one or more data packets are lost. This is generally caused by downstream packet loss.
References: [MULPIv4.0] Downstream Sequencing section

G.2.5.2.4 SkewThreshExceeds
This attribute counts in-range sequenced packets which were successfully received by the CM after a wait time
longer than the Resequencing Warning Threshold.
References: [MULPIv4.0] Downstream Sequencing section

G.2.5.2.5 OutOfRangePackets
This attribute counts the number of packets Counter received in a DSID reassembly context where the sequence
number which is out of range.
References: [MULPIv4.0] Receive Channels section




344                                                  CableLabs                                                04/29/26
