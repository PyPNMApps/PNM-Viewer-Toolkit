# Annex G.2.3 - DOCSIS QoS Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 257-281
Classification: Normative (explicit)

<!-- PDF_PAGE: 257 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


G.2.3    DOCSIS QoS Objects
This section defines the reporting of the DOCSIS CM QoS configuration. The model is updated in this specification
to include objects to configure the Active Queue Management (AQM) feature introduced in [MULPIv3.1].




                             Figure 17 - QoS Configuration Status Information Model


           PktClass
This object describes the packet classification configured on the CM or CMTS. The model is that a packet either
received as input from an interface or transmitted for output on an interface may be compared against an ordered list
of rules pertaining to the packet contents. Each rule is an instance of this object. A matching rule provides a Service


02/19/25                                            CableLabs®                                                     257
<!-- PDF_PAGE: 258 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


Flow ID to which the packet is classified. All rules need to match for a packet to match a classifier. The attributes in
this row correspond to a set of Classifier Encoding parameters in a DOCSIS MAC management message. The
BitMap attribute indicates which particular parameters were present in the classifier as signaled in the DOCSIS
message. If the referenced parameter was not present in the signaled Classifier, the corresponding attribute in this
instance reports a value as specified by that attribute description.
References: [MULPIv3.1] Service Flows and Classifiers section
                                          Table 124 - PktClass Object Attributes

      Attribute Name              Type              Access               Type Constraints           Units      Default
 IfIndex                 InterfaceIndex           Key          Interface Index of MAC Domain
                                                               interface
 ServiceFlowId           UnsignedInt              Key          1..4294967295
 Id                      UnsignedInt              Key          1..65535
 Direction               IfDirection              R/O
 Priority                UnsignedByte             R/O
 IpTosLow                HexBinary                R/O          SIZE(1)
 IpTosHigh               HexBinary                R/O          SIZE(1)
 IpTosMask               HexBinary                R/O          SIZE(1)
 IpProtocol              UnsignedShort            R/O          0..258
 IpSourceAddr            InetAddress              R/O
 IpSourceMask            InetAddress              R/O
 IpDestAddr              InetAddress              R/O
 IpDestMask              InetAddress              R/O
 SourcePortStart         InetPortNumber           R/O
 SourcePortEnd           InetPortNumber           R/O
 DestPortStart           InetPortNumber           R/O
 DestPortEnd             InetPortNumber           R/O
 IcmpTypeLow             UnsignedByte             R/O
 IcmpTypeHigh            UnsignedByte             R/O
 DestMacAddr             MacAddress               R/O
 DestMacMask             MacAddress               R/O
 SourceMacAddr           MacAddress               R/O
 EnetProtocolType        Enum                     R/O
 EnetProtocol            UnsignedInt              R/O          0..65535
 UserPriLow              UnsignedByte             R/O          0..7
 UserPriHigh             UnsignedByte             R/O          0..7
 VlanId                  UnsignedInt              R/O          0 | 1..4094
 State                   Enum                     R/O          active(1)
                                                               inactive(2)
 Pkts                    Counter64                R/O                                             packets




258                                                     CableLabs®                                            02/19/25
<!-- PDF_PAGE: 259 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


   Attribute Name                 Type              Access              Type Constraints              Units      Default
 BitMap                  EnumBits                 R/O           rulePriority(0),
                                                                activationState(1),
                                                                ipTos(2),
                                                                ipProtocol(3),
                                                                ipSourceAddr(4),
                                                                ipSourceMask(5),
                                                                ipDestAddr(6),
                                                                ipDestMask(7),
                                                                sourcePortStart(8),
                                                                sourcePortEnd(9),
                                                                destPortStart(10),
                                                                destPortEnd(11),
                                                                destMac(12),
                                                                sourceMac(13),
                                                                ethertype(14),
                                                                userPri(15),
                                                                vlanId(16),
                                                                flowLabel(17),
                                                                cmInterfaceMask(18),
                                                                icmpTypeLow(19),
                                                                icmpTypeHigh(20)
 IpAddrType              InetAddressType          R/O
 FlowLabel               UnsignedInt              R/O           0..1048575
 CmInterfaceMask         CpeInterfaceMaskType     R/O



G.2.3.1.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.3.1.2 ServiceFlowId
This key represents an identifier assigned to a Service Flow by CMTS within a MAC Domain. The value 0 is used
only for the purpose of reporting instances pertaining UDCs and not used for association of QoS classifiers to
service flows.

G.2.3.1.3 Id
This key indicates the assigned identifier to the packet classifier instance by the CMTS, which is unique per Service
Flow. For UDCs this corresponds to the Service Flow Reference of the classifier.
References: [MULPIv3.1] Classifier Identifier section in the Common Radio Frequency Interface Encodings annex

G.2.3.1.4 Direction
This attribute indicates the direction to which the classifier is applied.

G.2.3.1.5 Priority
This attribute specifies the order of evaluation of the classifiers. The higher the value, the higher the priority. The
value of 0 is used as default in provisioned Service Flows Classifiers. The default value of 64 is used for dynamic
Service Flow Classifiers. If the referenced parameter is not present in a classifier, this attribute reports the default
value as defined above.
References: [MULPIv3.1] Rule Priority section in the Common Radio Frequency Interface Encodings annex




02/19/25                                                CableLabs®                                                     259
<!-- PDF_PAGE: 260 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.3.1.6 IpTosLow
This attribute indicates the low value of a range of TOS byte values. If the referenced parameter is not present in a
classifier, this attribute reports the value of 0. The IP TOS octet as originally defined in [RFC 791] has been
superseded by the 6-bit Differentiated Services Field (DSField, [RFC 3260]) and the 2-bit Explicit Congestion
Notification Field (ECN field, [RFC 3168]). This object is defined as an 8-bit octet as defined by the DOCSIS
Specification for packet classification.
References: [MULPIv3.1] IPv4 Type of Service Range and Mask and IPv6 Traffic Class Range and Mask sections
in the Common Radio Frequency Interface Encodings annex

G.2.3.1.7 IpTosHigh
This attribute indicates the 8-bit high value of a range of TOS byte values. If the referenced parameter is not present
in a classifier, this attribute reports the value of 0. The IP TOS octet as originally defined in [RFC 791] has been
superseded by the 6-bit Differentiated Services Field (DSField, [RFC 3260]) and the 2-bit Explicit Congestion
Notification Field (ECN field, [RFC 3168]). This object is defined as an 8-bit octet as defined by the DOCSIS
Specification for packet classification.
References: [MULPIv3.1] IPv4 Type of Service Range and Mask and IPv6 Traffic Class Range and Mask sections
in the Common Radio Frequency Interface Encodings annex

G.2.3.1.8 IpTosMask
This attribute indicates the mask value is bitwise ANDed with TOS byte in an IP packet, and this value is used for
range checking of TosLow and TosHigh. If the referenced parameter is not present in a classifier, this attribute
reports the value of 0. The IP TOS octet as originally defined in [RFC 791] has been superseded by the 6-bit
Differentiated Services Field (DSField, [RFC 3260]) and the 2-bit Explicit Congestion Notification Field (ECN
field, [RFC 3168]). This object is defined as an 8-bit octet per the DOCSIS Specification for packet classification.
References: [MULPIv3.1] IPv4 Type of Service Range and Mask and IPv6 Traffic Class Range and Mask sections
in the Common Radio Frequency Interface Encodings annex

G.2.3.1.9 IpProtocol
This attribute indicates the value of the IP Protocol field required for IP packets to match this rule. The value 256
matches traffic with any IP Protocol value. The value 257 by convention matches both TCP and UDP. If the
referenced parameter is not present in a classifier, this attribute reports the value of 258.
References: [MULPIv3.1] IP Protocol and IPv6 Next Header Type sections in the Common Radio Frequency
Interface Encodings annex

G.2.3.1.10 IpSourceAddr
This attribute specifies the value of the IP Source Address required for packets to match this rule. An IP packet
matches the rule when the packet IP Source Address bitwise ANDed with the IpSourceMask value equals the
IpSourceAddr value. The address type of this object is specified by IpAddrType. If the referenced parameter is not
present in a classifier, this object reports the value of '00000000'H.
References: [MULPIv3.1] IPv4 Source Address and IPv6 Source Address sections in the Common Radio Frequency
Interface Encodings annex

G.2.3.1.11 IpSourceMask
This attribute specifies which bits of a packet's IP Source Address are compared to match this rule. An IP packet
matches the rule when the packet source address bitwise ANDed with the IpSourceMask value equals the
IpSourceAddr value. The address type of this attribute is specified by IpAddrType. If the referenced parameter is not
present in a classifier, this attribute reports the value of 'FFFFFFFF'H.
References: [MULPIv3.1] IPv4 Source Mask and IPv6 Source Prefix Length (bits) sections in the Common Radio
Frequency Interface Encodings annex



260                                                  CableLabs®                                               02/19/25
<!-- PDF_PAGE: 261 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


G.2.3.1.12 IpDestAddr
This attribute specifies the value of the IP Destination Address required for packets to match this rule. An IP packet
matches the rule when the packet IP Destination Address bitwise ANDed with the IpDestMask value equals the
IpDestAddr value. The address type of this attribute is specified by IpAddrType. If the referenced parameter is not
present in a classifier, this attribute reports the value of '00000000'H.
References: [MULPIv3.1] IPv4 Destination Address and IPv6 Destination Address sections in the Common Radio
Frequency Interface Encodings annex

G.2.3.1.13 IpDestMask
This attribute specifies which bits of a packet's IP Destination Address are compared to match this rule. An IP
packet matches the rule when the packet destination address bitwise ANDed with the IpDestMask value equals the
IpDestAddr value. The address type of this attribute is specified by IpAddrType. If the referenced parameter is not
present in a classifier, this attribute reports the value of 'FFFFFFFF'H.
References: [MULPIv3.1] IPv4 Destination Mask and IPv6 Destination Prefix Length (bits) sections in the Common
Radio Frequency Interface Encodings annex

G.2.3.1.14 SourcePortStart
This attribute specifies the low-end inclusive range of TCP/UDP source port numbers to which a packet is
compared. This attribute is irrelevant for non-TCP/UDP IP packets. If the referenced parameter is not present in a
classifier, this attribute reports the value of 0.
References: [MULPIv3.1] TCP/UDP Source Port Start section in the Common Radio Frequency Interface
Encodings annex

G.2.3.1.15 SourcePortEnd
This attribute specifies the high-end inclusive range of TCP/UDP source port numbers to which a packet is
compared. This attribute is irrelevant for non-TCP/UDP IP packets. If the referenced parameter is not present in a
classifier, this attribute reports the value of 65535.
References: [MULPIv3.1] TCP/UDP Source Port End section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.16 DestPortStart
This attribute specifies the low-end inclusive range of TCP/UDP destination port numbers to which a packet is
compared. If the referenced parameter is not present in a classifier, this attribute reports the value of 0.
References: [MULPIv3.1] TCP/UDP Destination Port Start section in the Common Radio Frequency Interface
Encodings annex

G.2.3.1.17 DestPortEnd
This attribute specifies the high-end inclusive range of TCP/UDP destination port numbers to which a packet is
compared. If the referenced parameter is not present in a classifier, this attribute reports the value of 65535.
References: [MULPIv3.1] TCP/UDP Destination Port End section in the Common Radio Frequency Interface
Encodings annex

G.2.3.1.18 IcmpTypeLow
This attribute specifies the low-end inclusive range of the ICMP type numbers to which a packet is compared. If the
referenced parameter is not present in a classifier, this attribute reports the value of 0.
References: [MULPIv3.1] TypeLow Encodings section in the Common Radio Frequency Interface annex




02/19/25                                            CableLabs®                                                     261
<!-- PDF_PAGE: 262 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.3.1.19 IcmpTypeHigh
This attribute specifies the high-end inclusive range of the ICMP type numbers to which a packet is compared. If the
referenced parameter is not present in a classifier, this attribute reports the value of 255.
References: [MULPIv3.1] TypeHigh Encodings section in the Common Radio Frequency Interface annex

G.2.3.1.20 DestMacAddr
An Ethernet packet matches an entry when its destination MAC address bitwise ANDed with DestMacMask equals
the value of DestMacAddr. If the referenced parameter is not present in a classifier, this attribute reports the value of
'000000000000'H.
References: [MULPIv3.1] Destination MAC Address section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.21 DestMacMask
An Ethernet packet matches an entry when its destination MAC address bitwise ANDed with DestMacMask equals
the value of DestMacAddr. If the referenced parameter is not present in a classifier, this attribute reports the value of
'000000000000'H.
References: [MULPIv3.1] Destination MAC Address section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.22 SourceMacAddr
An Ethernet packet matches this entry when its source MAC address equals the value of this attribute. If the
referenced parameter is not present in a classifier, this attribute reports the value of 'FFFFFFFFFFFF'.
References: [MULPIv3.1] Source MAC Address section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.23 EnetProtocolType
This attribute indicates the format of the layer 3 protocol ID in the Ethernet packet. A value of 'none' means that the
rule does not use the layer 3 protocol type as a matching criteria. A value of 'ethertype' means that the rule applies
only to frames that contain an EtherType value. Ethertype values are contained in packets using the Dec-Intel-Xerox
(DIX) encapsulation or the RFC1042 Sub-Network Access Protocol (SNAP) encapsulation formats. A value of
'dsap' means that the rule applies only to frames using the IEEE802.3 encapsulation format with a Destination
Service Access Point (DSAP) other than 0xAA (which is reserved for SNAP). A value of 'mac' means that the rule
applies only to MAC management messages for MAC management messages. A value of 'all' means that the rule
matches all Ethernet packets. If the Ethernet frame contains an 802.1P/Q Tag header (i.e., EtherType 0x8100), this
attribute applies to the embedded EtherType field within the 802.1P/Q header. If the referenced parameter is not
present in a classifier, this attribute reports the value of 0.
References: [MULPIv3.1] Ethertype/DSAP/MacType section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.24 EnetProtocol
If EnetProtocolType is 'none', this attribute is ignored when considering whether a packet matches the current rule.
If EnetProtocolType is 'ethertype', this attribute gives the 16-bit value of the EtherType that the packet must match
in order to match the rule. If EnetProtocolType is 'dsap', the lower 8 bits of this attribute's value must match the
DSAP byte of the packet in order to match the rule. If EnetProtocolType is 'mac', the lower 8 bits of this attribute's
value represent a lower bound (inclusive) of MAC management message type codes matched, and the upper 8 bits
represent the upper bound (inclusive) of matched MAC message type codes. Certain message type codes are
excluded from matching, as specified in the reference. If the Ethernet frame contains an 802.1P/Q Tag header (i.e.,
EtherType 0x8100), this attribute applies to the embedded EtherType field within the 802.1P/Q header. If the
referenced parameter is not present in the classifier, the value of this attribute is reported as 0.




262                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 263 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Ethertype/DSAP/MacType section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.25 UserPriLow
This attribute applies only to Ethernet frames using the 802.1P/Q tag header (indicated with EtherType 0x8100).
Such frames include a 16-bit Tag that contains a 3-bit Priority field and a 12-bit VLAN number. Tagged Ethernet
packets must have a 3-bit Priority field within the range of PriLow to PriHigh in order to match this rule. If the
referenced parameter is not present in the classifier, the value of this attribute is reported as 0.
References: [MULPIv3.1] IEEE 802.1P User_Priority section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.26 UserPriHigh
This attribute applies only to Ethernet frames using the 802.1P/Qtag header (indicated with EtherType 0x8100).
Such frames include a 16-bit Tag that contains a 3-bit Priority field and a 12-bit VLAN number. Tagged Ethernet
packets must have a 3-bit Priority field within the range of PriLow to PriHigh in order to match this rule. If the
referenced parameter is not present in the classifier, the value of this attribute is reported as 7.
References: [MULPIv3.1] IEEE 802.1P User_Priority section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.27 VlanId
This attribute applies only to Ethernet frames using the 802.1P/Q tag header. Tagged packets must have a VLAN
Identifier that matches the value in order to match the rule. If the referenced parameter is not present in the
classifier, the value of this attribute is reported as 0.
References: [MULPIv3.1] IEEE 802.1Q VLAN_ID section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.28 State
This attribute indicates whether or not the classifier is enabled to classify packets to a Service Flow. If the
referenced parameter is not present in the classifier, the value of this attribute is reported as 'true'.
References: [MULPIv3.1] Classifier Activation State section in the Common Radio Frequency Interface Encodings
annex

G.2.3.1.29 Pkts
This attribute counts the number of packets that have been classified using this entry. This includes all packets
delivered to a Service Flow maximum rate policing function, whether or not that function drops the packets. This
counter's last discontinuity is the ifCounterDiscontinuityTime for the same ifIndex that indexes this attribute.

G.2.3.1.30 BitMap
This attribute indicates which parameter encodings were actually present in the DOCSIS packet classifier encoding
signaled in the DOCSIS message that created or modified the classifier. Note that Dynamic Service Change
messages have replace semantics, so that all non-default parameters must be present whether the classifier is being
created or changed. A bit of this attribute is set to 1 if the parameter indicated by the comment was present in the
classifier encoding, and to 0 otherwise. Note that BITS are encoded most significant bit first, so that if, for example,
bits 6 and 7 are set, this attribute is encoded as the octet string '030000'H.

G.2.3.1.31 IpAddrType
This attribute indicates the type of the Internet address for IpSourceAddr, IpSourceMask, IpDestAddr, and
IpDestMask. If the referenced parameter is not present in a classifier, this object reports the value of 'ipv4'.




02/19/25                                             CableLabs®                                                     263
<!-- PDF_PAGE: 264 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


G.2.3.1.32 FlowLabel
This attribute represents the Flow Label field in the IPv6 header to be matched by the classifier. The value zero
indicates that the Flow Label is not specified as part of the classifier and is not matched against the packets.
References: [MULPIv3.1] IPv6 Flow Label section in the Common Radio Frequency Interface Encodings annex

G.2.3.1.33 CmInterfaceMask
This attribute represents a bit-mask of the CM in-bound interfaces to which this classifier applies. This attribute only
applies to QoS upstream Classifiers and upstream Drop Classifiers. For QoS downstream classifiers this object
reports the zero-length string.
References: [MULPIv3.1] CM Interface Mask (CMIM) Encoding section in the Common Radio Frequency
Interface Encodings annex

              ParamSet
This object describes the set of QoS parameters defined in a managed device. Each row corresponds to a DOCSIS
QoS Parameter Set as signaled via DOCSIS MAC management messages. Each attribute of an instance of this
object corresponds to one or part of one Service Flow Encoding. The BitMap attribute indicates which particular
parameters were signaled in the original registration or dynamic service request message that created the QoS
Parameter Set. In many cases, even if a QoS Parameter Set parameter was not signaled, the DOCSIS specification
calls for a default value to be used. That default value is reported as the value of the corresponding attribute in this
object instance. Many attributes are not applicable, depending on the Service Flow direction, upstream scheduling
type or Service Flow bonding configuration. The attribute value reported in this case is specified by those attributes
descriptions.
For CM devices supporting the ParamSet object, the CM MUST report only the active service flow parameter set
values. The CM MAY report the admitted and provisioned service flow parameter sets.
References: [MULPIv3.1] Service Flow Encodings section in the Common Radio Frequency Interface Encodings
annex
                                       Table 125 - ParamSet Object Attributes

            Attribute Name            Type         Access              Type Constraints           Units       Default (See
                                                                                                               Attribute
                                                                                                              Description)
 IfIndex                        InterfaceIndex     Key        Interface Index of MAC Domain
                                                              interface
 ServiceClassName               AdminString        R/O        SIZE (0..15)
 Priority                       UnsignedByte       R/O        0..7
 MaxTrafficRate                 BitRate            R/O
 MaxTrafficBurst                UnsignedInt        R/O                                        bytes
 MinReservedRate                BitRate            R/O
 MinReservedPkt                 UnsignedShort      R/O                                        bytes
 ActiveTimeout                  UnsignedShort      R/O                                        seconds
 AdmittedTimeout                UnsignedShort      R/O                                        seconds
 MaxConcatBurst                 UnsignedShort      R/O                                        bytes
 SchedulingType                 SchedulingType     R/O
 NomPollInterval                UnsignedInt        R/O                                        microseconds
 TolPollJitter                  UnsignedInt        R/O                                        microseconds
 UnsolicitGrantSize             UnsignedShort      R/O                                        bytes
 NomGrantInterval               UnsignedInt        R/O                                        microseconds
 TolGrantJitter                 UnsignedInt        R/O                                        microseconds
 GrantsPerInterval              UnsignedByte       R/O        0..127                          dataGrants




264                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 265 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv3.1-I27-250219


          Attribute Name               Type     Access         Type Constraints               Units       Default (See
                                                                                                           Attribute
                                                                                                          Description)
 GuaranteedGrantInterval        UnsignedShort   R/O                                        microseconds
 GuaranteedGrantRate            BitRate         R/O
 GuaranteedGrantRequestInterv   UnsignedShort   R/O                                        microseconds
 al
 TosAndMask                     HexBinary       R/O      SIZE (1)
 TosOrMask                      HexBinary       R/O      SIZE (1)
 MaxLatency                     UnsignedInt     R/O                                        microseconds
 Type                           Enum            Key      active (1)
                                                         admitted (2)
                                                         provisioned (3)
 RequestPolicyOct               HexBinary       R/O      SIZE (4)
 BitMap                         EnumBits        R/O      trafficPriority(0)
                                                         maxTrafficRate(1)
                                                         maxTrafficBurst(2)
                                                         minReservedRate(3)
                                                         minReservedPkt(4)
                                                         activeTimeout(5)
                                                         admittedTimeout(6)
                                                         maxConcatBurst(7)
                                                         schedulingType(8)
                                                         requestPolicy(9)
                                                         nomPollInterval(10)
                                                         tolPollJitter(11)
                                                         unsolicitGrantSize(12)
                                                         nomGrantInterval(13)
                                                         tolGrantJitter(14)
                                                         grantsPerInterval(15)
                                                         tosOverwrite(16)
                                                         maxLatency(17)
                                                         requiredAttrMask(18)
                                                         forbiddenAttrMask(19)
                                                         attrAggrMask(20)
                                                         applicationId(21)
                                                         multipCntnReqWindow(22)
                                                         multipBytesReq(23)
                                                         maxReqPerSidCluster(24)
                                                         maxOutstandingBytesPerSidClus
                                                         ter(25)
                                                         maxTotalBytesReqPerSidCluster
                                                         (26)
                                                         maximumTimeInSidCluster(27)
                                                         peakTrafficRate(28)
                                                         dsResequencing(29)
                                                         minimumBuffer(30)
                                                         targetBuffer(31)
                                                         maximumBuffer(32)
                                                         aqmEnabled(33)
                                                         aqmLatencyTarget(34)
                                                         dataRateUnit(35)
                                                         aqmAlgInUse(36)
                                                         guaranteedGrantInterval(37)
                                                         guaranteedGrantRate(38)
                                                         guaranteedGrantRequestInterval(
                                                         39)
                                                         immedAqmMaxThrshld(40)
                                                         immedAqmRngExpRampFunc(4
                                                         1)
                                                         pgsActivityDetectionDisable(42)
 ServiceFlowId                  UnsignedInt     Key      1..4294967295
 RequiredAttrMask               AttributeMask   R/O




02/19/25                                         CableLabs®                                                    265
<!-- PDF_PAGE: 266 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


         Attribute Name                Type        Access             Type Constraints              Units    Default (See
                                                                                                              Attribute
                                                                                                             Description)
 ForbiddenAttrMask              AttributeMask      R/O
 AttrAggrRuleMask               AttrAggrRuleMask R/O         SIZE (0 | 4)
 AppId                          UnsignedInt        R/O
 MultiplierContentionReqWindow UnsignedByte        R/O       4..12                            eighths
 MultiplierBytesReq             UnsignedByte       R/O       1 | 2 | 4 | 8 | 16               requests
 MaxReqPerSidCluster            UnsignedByte       R/O                                        bytes
 MaxOutstandingBytesPerSidClu UnsignedInt          R/O                                        bytes
 ster
 MaxTotBytesReqPerSidCluster    UnsignedInt        R/O                                        bytes
 MaxTimeInSidCluster            UnsignedShort      R/O                                        milliseconds
 PeakTrafficRate                BitRate            R/O
 DsResequencing                 Enum               R/O       resequencingDsidIfBonded(0)
                                                             noResequencingDsid(1)
                                                             notApplicable(2)
 MinimumBuffer                  UnsignedInt        R/O       0..4294967295                    bytes
 TargetBuffer                   UnsignedInt        R/O       0..4294967295                    bytes
 MaximumBuffer                  UnsignedInt        R/O       0..4294967295                    bytes
 AqmDisabled                    Boolean            R/O
 ClassicAqmLatencyTarget        UnsignedShort      R/O       0..256                           msec
 AqmAlgInUse                    Enum               R/O       unknown(1)
                                                             other(2)
                                                             docsisPIE(3)
                                                             immediateAqm(4)
 ImmedAqmMaxThreshold           UnsignedShort      R/O
 ImmedAqmRangeExponentRa        UnsignedByte       R/O
 mpFunction
 DataRateUnitSetting            DataRateUnitTyp    R/O                                        N/A            'bps'
                                e



G.2.3.2.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.3.2.2 ServiceClassName
This attribute represents the Service Class Name from which the parameter set values were derived. If the referenced
parameter is not present in the corresponding DOCSIS QoS Parameter Set, this attribute returns the zero-length
string.
References: [MULPIv3.1] Service Class Name section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.3 Priority
This attribute represents the relative priority of a Service Flow. Higher numbers indicate higher priority. This
priority should only be used to differentiate Service Flow from identical parameter sets. This attribute returns 0 if the
referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set or if the parameter is not
applicable.
References: [MULPIv3.1] Traffic Priority section in the Common Radio Frequency Interface Encodings annex




266                                                  CableLabs®                                               02/19/25
<!-- PDF_PAGE: 267 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


G.2.3.2.4 MaxTrafficRate
This attribute represents the 4-byte value of the maximum sustained traffic rate allowed for this Service Flow. It
represents all MAC frame data PDUs from the bytes following the MAC header HCS to the end of the CRC. The
number of bytes forwarded is limited during any time interval. The value 0 means no maximum traffic rate is
enforced. The value of the DataRateUnitSetting attribute defines the units of MaxTrafficRate. This attribute applies
to both upstream and downstream Service Flows. This attribute returns 0 if the referenced parameter is not present in
the corresponding DOCSIS QoS Parameter Set, or if the parameter is not applicable.
References: [MULPIv3.1] Maximum Sustained Traffic Rate section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.5 MaxTrafficBurst
This attribute specifies the token bucket size in bytes for this parameter set. The value is calculated from the byte
following the MAC header HCS to the end of the CRC. This object is applied in conjunction with MaxTrafficRate
to calculate maximum sustained traffic rate. If the referenced parameter is not present in the corresponding DOCSIS
QoS Parameter Set, this attribute returns 3044 for scheduling types 'bestEffort', 'nonRealTimePollingService' and
'realTimePollingService'. If this parameter is not applicable, it is reported as 0.
References: [MULPIv3.1] Maximum Traffic Burst section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.6 MinReservedRate
This attribute represents the 4-byte value of the guaranteed minimum rate for this Service Flow. The value is
calculated from the byte following the MAC header HCS to the end of the CRC. The value of 0 indicates that no
bandwidth is reserved. If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set,
this attribute returns 0. If the parameter is not applicable, it is reported as 0. The value of the DataRateUnitSetting
attribute defines the units of MinReservedRate.
References: [MULPIv3.1] Minimum Reserved Traffic Rate section of the Common Radio Frequency Interface
Encodings annex

G.2.3.2.7 MinReservedPkt
This attribute specifies an assumed minimum packet size in bytes for which the MinReservedRate will be provided.
The value is calculated from the byte following the MAC header HCS to the end of the CRC. If the referenced
parameter is omitted from a DOCSIS QoS parameter set, the used and reported value is CMTS implementation and
the CM reports a value of 0. If the referenced parameter is not applicable to the direction or scheduling type of the
Service Flow, both CMTS and CM report the value 0.
References: [MULPIv3.1] Assumed Minimum Reserved Rate Packet Size in the Common Radio Frequency
Interface Encodings annex

G.2.3.2.8 ActiveTimeout
This attribute specifies the maximum duration in seconds that resources remain unused on an active service flow
before the CMTS signals that both the active and admitted parameter sets are null. The value 0 signifies an infinite
amount of time. If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set, this
attribute returns 0.
References: [MULPIv3.1] Timeout for Active QoS Parameters section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.9 AdmittedTimeout
This attribute specifies the maximum duration in seconds that resources remain in admitted state before resources
must be released. The value of 0 signifies an infinite amount of time. If the referenced parameter is not present in the
corresponding DOCSIS QoS Parameter Set, this attribute returns 200.




02/19/25                                             CableLabs®                                                     267
<!-- PDF_PAGE: 268 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


References: [MULPIv3.1] Timeout for Admitted QoS Parameters section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.10 MaxConcatBurst
This attribute specifies the maximum concatenated burst in bytes that an upstream Service Flow is allowed. The
value is calculated from the FC byte of the Concatenation MAC Header to the last CRC byte of the last concatenated
MAC frame, inclusive. The value of 0 specifies no maximum burst. If the referenced parameter is not present in the
corresponding DOCSIS QoS Parameter Set, this attribute returns the value of 1522 for scheduling types 'bestEffort',
'nonRealTimePollingService', and 'realTimePollingService'. If the parameter is not applicable, it is reported as 0.
References: [MULPIv3.1] Maximum Concatenated Burst section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.11 SchedulingType
This attribute specifies the upstream scheduling service used for upstream Service Flow. If the referenced parameter
is not present in the corresponding DOCSIS QoS Parameter Set of an upstream Service Flow, this attribute returns
the value of 'bestEffort'. For QoS parameter sets of downstream Service Flows, this attribute's value is reported as
'undefined'.
References: [MULPIv3.1] Service Flow Scheduling Type section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.12 NomPollInterval
This attribute specifies the nominal interval in microseconds between successive unicast request opportunities on an
upstream Service Flow. This attribute applies only to upstream Service Flows with SchedulingType of value
'nonRealTimePollingService', 'realTimePollingService', and 'unsolictedGrantServiceWithAD'. The parameter is
mandatory for 'realTimePollingService'. If the parameter is omitted with 'nonRealTimePollingService', the CMTS
uses an implementation-dependent value. If the parameter is omitted with 'unsolictedGrantServiceWithAD(5)' the
CMTS uses the value of the Nominal Grant Interval parameter. In all cases, the CMTS reports the value it is using
when the parameter is applicable. The CM reports the signaled parameter value if it was signaled. Otherwise, it
returns 0. If the referenced parameter is not applicable to the direction or scheduling type of the corresponding
DOCSIS QoS Parameter Set, both CMTS and CM report this attribute's value as 0.
References: [MULPIv3.1] Polling Interval section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.13 TolPollJitter
This attribute specifies the maximum amount of time in microseconds that the unicast request interval may be
delayed from the nominal periodic schedule on an upstream Service Flow. This parameter is applicable only to
upstream Service Flows with a SchedulingType of 'realTimePollingService' or 'unsolictedGrantServiceWithAD'. If
the referenced parameter is applicable but not present in the corresponding DOCSIS QoS Parameter Set, the CMTS
uses an implementation-dependent value and reports the value it is using. The CM reports a value of 0 in this case. If
the parameter is not applicable to the direction or upstream scheduling type of the Service Flow, both CMTS and
CM report this attribute's value as 0.
References: [MULPIv3.1] Tolerated Poll Jitter section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.14 UnsolicitGrantSize
This attribute specifies the unsolicited grant size in bytes. The grant size includes the entire MAC frame data PDU
from the Frame Control byte to the end of the MAC frame. The referenced parameter is applicable only for upstream
flows with a SchedulingType of 'unsolicitedGrantServiceWithAD' or 'unsolicitedGrantService', and it is mandatory
when applicable. Both CMTS and CM report the signaled value of the parameter in this case. If the referenced
parameter is not applicable to the direction or scheduling type of the corresponding DOCSIS QoS Parameter Set,
both CMTS and CM report this attribute's value as 0.




268                                                CableLabs®                                               02/19/25
<!-- PDF_PAGE: 269 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Unsolicited Grant Size section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.15 NomGrantInterval
This attribute specifies the nominal interval in microseconds between successive data grant opportunities on an
upstream Service Flow. The referenced parameter is applicable only for upstream flows with a SchedulingType of
'unsolicitedGrantServiceWithAD' or 'unsolicitedGrantService(6)', and it is mandatory when applicable. Both CMTS
and CM report the signaled value of the parameter in this case. If the referenced parameter is not applicable to the
direction or scheduling type of the corresponding DOCSIS QoS Parameter Set, both CMTS and CM report this
attribute's value as 0.
References: [MULPIv3.1] Nominal Grant Interval section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.16 TolGrantJitter
This attribute specifies the maximum amount of time in microseconds that the transmission opportunities may be
delayed from the nominal periodic schedule. The referenced parameter is applicable only for upstream flows with a
SchedulingType of 'unsolicitedGrantServiceWithAD' or 'unsolicitedGrantService(6)', and it is mandatory when
applicable. Both CMTS and CM report the signaled value of the parameter in this case. If the referenced parameter
is not applicable to the direction or scheduling type of the corresponding DOCSIS QoS Parameter Set, both CMTS
and CM report this attribute's value as 0.
References: [MULPIv3.1] Tolerated Grant Jitter section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.17 GrantsPerInterval
This attribute specifies the number of data grants per Nominal Grant Interval (NomGrantInterval). The referenced
parameter is applicable only for upstream flows with a SchedulingType of 'unsolicitedGrantServiceWithAD' or
'unsolicitedGrantService', and it is mandatory when applicable. Both CMTS and CM report the signaled value of the
parameter in this case. If the referenced parameter is not applicable to the direction or scheduling type of the
corresponding DOCSIS QoS Parameter Set, both CMTS and CM report this attribute's value as 0.
References: [MULPIv3.1] Grants per Interval section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.18 GuaranteedGrantInterval
This attribute specifies the maximum interval between successive data transmission opportunities for a PGS Service
Flow. This attribute is a 16-bit representation of the grant interval in units of microseconds.
References: [MULPIv3.1] Guaranteed Grant Interval section in the Common TLV Encodings annex

G.2.3.2.19 GuaranteedGrantRate
This attribute specifies the minimum granting rate for an upstream PGS service flow. The value of the
DataRateUnitSetting attribute defines the units of GuaranteedGrantRate.
References: [MULPIv3.1] Guaranteed Grant Rate section in the Common TLV Encodings annex

G.2.3.2.20 GuaranteedRequestInterval
The value of this parameter specifies the maximum interval between successive request opportunities (including
unicast request opportunities and piggyback request opportunities) for an upstream PGS service flow. This attribute
is a 16-bit representation of the request interval in units of microseconds. The value zero represents polling disabled.
References: [MULPIv3.1] Guaranteed Request Interval section in the Common TLV Encodings annex




02/19/25                                            CableLabs®                                                      269
<!-- PDF_PAGE: 270 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.3.2.21 TosAndMask
This attribute specifies the AND mask for the IP TOS byte for overwriting an IPv4 packet's TOS value or IPv6
packet's Traffic Class value. The IP packet TOS byte is bitwise ANDed with TosAndMask, then the result is bitwise
ORed with TosORMask and the result is written to the IP packet TOS byte. A value of 'FF'H for TosAndMask and a
value of '00'H for TosOrMask means that the IP Packet TOS byte is not overwritten. This combination is reported if
the referenced parameter is not present in a QoS Parameter Set. The IP TOS octet as originally defined in [RFC 791]
has been superseded by the 6-bit Differentiated Services Field (DSField, [RFC 3260]) and the 2-bit Explicit
Congestion Notification Field (ECN field, [RFC 3168]). The IPv6 Traffic Class octet [RFC 2460] is consistent with
that new definition. Network operators should avoid specifying values of TosAndMask and TosORMask that would
result in the modification of the ECN bits. In particular, operators should not use values of TosAndMask that have
either of the least-significant two bits set to 0. Similarly, operators should not use values of TosORMask that have
either of the least-significant two bits set to 1. Even though this attribute is only enforced by the CMTS, the CM
reports the value as signaled in the referenced parameter.
References: [MULPIv3.1] IP Type Of Service (DSCP) Overwrite section in the Common Radio Frequency Interface
Encodings annex; [RFC 3168]; [RFC 3260]; [RFC 2460]; [RFC 791]

G.2.3.2.22 TosOrMask
This attribute specifies the OR mask for the IPv4 TOS value or IPv6 Traffic Class value. See the description of
TosAndMask for further details. The IP TOS octet, as originally defined in [RFC 791] has been superseded by the 6-
bit Differentiated Services Field (DSField, [RFC 3260]) and the 2-bit Explicit Congestion Notification Field (ECN
field, [RFC 3168]). The IPv6 Traffic Class octet [RFC 2460] is consistent with that new definition. Network
operators should avoid specifying values of TosAndMask and TosORMask that would result in the modification of
the ECN bits.
References: [MULPIv3.1] IP Type Of Service (DSCP) Overwrite section in the Common Radio Frequency Interface
Encodings annex; [RFC 3168]; [RFC 3260]; [RFC 2460]; [RFC 791]

G.2.3.2.23 MaxLatency
This attribute specifies the maximum latency between the reception of a packet by the CMTS on its NSI and the
forwarding of the packet to the RF interface. A value of 0 signifies no maximum latency is enforced. This attribute
only applies to downstream Service Flows. If the referenced parameter is not present in the corresponding
downstream DOCSIS QoS Parameter Set, this attribute returns 0. This parameter is not applicable to upstream
DOCSIS QoS Parameter Sets, so its value is reported as 0 in that case.
References: [MULPIv3.1] Maximum Downstream Latency section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.24 Type
This key represents the QoS Parameter Set Type of the Service Flow. The following values are defined: 'active'
Indicates the Active QoS parameter set, describing the service currently being provided by the DOCSIS MAC
domain to the service flow. 'admitted' indicates the Admitted QoS Parameter Set, describing services reserved by the
DOCSIS MAC domain for use by the service flow. 'provisioned' indicates the QoS Parameter Set defined in the
DOCSIS CM Configuration file for the service flow.
Only the 'active' service flow parameter set is required to be reported. The 'admitted' and 'provisioned' sets are not
required to be reported.
References: [MULPIv3.1] Service Flow Scheduling Type section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.25 RequestPolicyOct
This attribute specifies which transmit interval opportunities the CM omits for upstream transmission requests and
packet transmissions. This object takes its default value for downstream Service Flows. Unless otherwise indicated,
a bit value of 1 means that a CM must not use that opportunity for upstream transmission. The format of this string



270                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 271 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


enumerated the bits from 0 to 31 from left to right, for example bit 0 corresponds to the left most bit of the fourth
octet. (octets numbered from right to left). The bit positions are defined as follows:
    'broadcastReqOpp' - all CMs broadcast request opportunities
    'priorityReqMulticastReq' - priority request multicast request opportunities
    'reqDataForReq' - request/data opportunities for requests
    'reqDataForData' - request/data opportunities for data
    'piggybackReqWithData' - piggyback requests with data
    'concatenateData' - concatenate data
    'fragmentData' - fragment data
    'suppressPayloadHeaders' - suppress payload headers
    'dropPktsExceedUGSize' - A value of 1 means that the service flow must drop packets that do not fit in the
    Unsolicited Grant size. If the referenced parameter is not present in a QoS Parameter Set, the value of this
    object is reported as '00000000'H.
References: [MULPIv3.1] Request/Transmission Policy section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.26 BitMap
This attribute indicates the set of QoS Parameter Set parameters actually signaled in the DOCSIS registration or
dynamic service request message that created or modified the QoS Parameter Set. Possible QoS Parameter Set
parameters are listed below with the corresponding TLV Type. A bit is set to 1 when the associated parameter is
present in the original request as follows:
    'trafficPriority' Traffic Priority (24/25.7)
    'maxTrafficRate' Maximum Sustained Traffic Rate (24/25.8)
    'maxTrafficBurst' Maximum Traffic Burst (24/25.9)
    'minReservedRate' Minimum Reserved Traffic Rate (24/25.10)
    'minReservedPkt' Assumed Minimum Reserved Rate Packet Size (24/25.11)
    'activeTimeout' Timeout for Active QoS Parameters (24/25.12)
    'admittedTimeout' Timeout for Admitted QoS Parameters (24/25.13)
    'maxConcatBurst' Maximum Concatenated Burst (24.14)
    'schedulingType' Service Flow Scheduling Type (24.15)
    'requestPolicy' Request/Transmission Policy (24.16)
    'nomPollInterval' Nominal Polling Interval (24.17)
    'tolPollJitter' Tolerated Poll Jitter (24.18)
    'unsolicitGrantSize' Unsolicited Grant Size (24.19)
    'nomGrantInterval' Nominal Grant Interval (24.20)
    'tolGrantJitter' Tolerated Grant Jitter (24.21)
    'grantsPerInterval' Grants per Interval (24.22)
    'tosOverwrite' IP Type of Service (DSCP) Overwrite (24.23)
    'maxLatency' Maximum Downstream Latency (25.14)
    'requiredAttrMask' Service Flow Required Attribute Mask (24/25.31)



02/19/25                                              CableLabs®                                                    271
<!-- PDF_PAGE: 272 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


      'forbiddenAttrMask' Service Flow Forbidden Attribute Mask (24/25.32)
      'attrAggrMask' Service Flow Attribute Aggregation Rule Mask (24/25.33)
      'applicationId' Application Identifier (24/25.34)
      'multipCntnReqWindow' Multiplier to Contention Request Backoff Window (24.25)
      'multipBytesReq' Multiplier to Number of Bytes Requested (24.26)
      'maxReqPerSidCluster' Maximum Requests per SID Cluster (47/89.3.1)
      'maxOutstandingBytesPerSidCluster' Maximum Outstanding Bytes per SID Cluster (47/89.3.2)
      'maxTotalBytesReqPerSidCluster' Maximum Total Bytes Requested per SID Cluster (47/89.3.3)
      'maximumTimeInSidCluster' Maximum Time in the SID Cluster (47/89.3.4)
      'peakTrafficRate' Peak Traffic Rate (24/25.27)
      'dsResequencing' - Downstream Resequencing (25.17)
      'minimumBuffer' Minimum Buffer (24/25.35.1)
      'targetBuffer' Target Buffer (24/25.35.2)
      'maximumBuffer' Maximum Buffer (24/25.35.3)
      'aqmEnabled' SF AQM Enabled (24/25.40.1)
      'aqmLatencyTarget' Classic AQM Latency Target (24/25.40.2)
      'dataRateUnit' Data Rate Unit Setting (24/25/70/71.41)
      'aqmAlgInUse' AQM Algorithm (24/25.40.3)
      'guaranteedGrantInterval' Guaranteed Grant Interval (24.44)
      'guaranteedGrantRate' Guaranteed Grant Rate (24.45)
      'guaranteedGrantRequestInterval' Guaranteed Grant Request Interval (24.46)
      'immedAqmMaxThrshld' Immediate AQM Maximum Threshold (24/25.40.4)
      'immedAqmRngExpRampFunc' Immediate AQM Range Exponent of Ramp Function (24/25.40.5)
      'pgsActivityDetectionDisable' PGS Activity Detection Disable (24.49)
Note that when Service Class names are expanded, the registration or dynamic response message may contain
parameters expanded by the CMTS based on a stored service class. These expanded parameters are not indicated by
a 1 bit in this attribute. Note that even though some QoS Parameter Set parameters may not be signaled in a message
(so that the parameter's bit in this object is 0), the DOCSIS specification requires that default values be used. These
default values are reported as the corresponding attribute.
References: [MULPIv3.1] Service Flow Encodings section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.27 ServiceFlowId
This key represents the Service Flow ID for the service flow.
References: [MULPIv3.1] Service Identifier section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.28 RequiredAttrMask
This attribute specifies the Required Attribute Mask to compare with the Provisioned Required Attributes when
selecting the bonding groups for the service flow.
If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set, this attribute returns
'00000000'H.


272                                                    CableLabs®                                            02/19/25
<!-- PDF_PAGE: 273 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Service Flow Required Attribute Mask section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.29 ForbiddenAttrMask
This attribute specifies the Forbidden Attribute Mask to compare with the Provisioned Forbidden Attributes when
selecting the bonding groups for the service flow.
References: [MULPIv3.1] Service Flow Forbidden Attribute Mask section in the Common Radio Frequency
Interface Encodings annex
If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set, this attribute returns
'00000000'H.

G.2.3.2.30 AttrAggrRuleMask
This attribute specifies the Attribute Aggregation Mask to compare the Service Flow Required and Forbidden
Attributes with the CMTS dynamically created bonding group when selecting the bonding groups for the service
flow.
References: [MULPIv3.1] Service Flow Attribute Aggregation Mask section in the Common Radio Frequency
Interface Encodings annex
If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set, this attribute returns
'00000000'H.

G.2.3.2.31 AppId
This attribute represents the Application Identifier associated with the service flow for purposes beyond the scope of
this specification.
If the referenced parameter is not present in the corresponding DOCSIS QoS Parameter Set, this attribute returns 0.
References: [MULPIv3.1] Application Identifier section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.32 MultiplierContentionReqWindow
This attribute specifies the multiplier to be applied by a CM when performing contention request backoff for data
requests. This attribute only applies to upstream Service Flows in 3.0 operation. if the referenced parameter is not
present in the upstream DOCSIS QoS Parameter Set, or is not applicable, this attribute returns 8.
References: [MULPIv3.1] Multiplier to Contention Request Backoff Window section in the Common Radio
Frequency Interface Encodings annex

G.2.3.2.33 MultiplierBytesReq
This attribute specifies the assumed bandwidth request multiplier. This attribute only applies to upstream Service
Flows in 3.0 operation. If the referenced parameter is not present in the upstream DOCSIS QoS Parameter Set, or is
not applicable, this attribute returns 4.
References: [MULPIv3.1] Multiplier to Number of Bytes Requested section in the Common Radio Frequency
Interface Encodings annex

G.2.3.2.34 MaxReqPerSidCluster
This attribute specifies the maximum number of requests that a CM can make within a given SID Cluster before it
must switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit. This
attribute only applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0. If the referenced
parameter is not present in the DOCSIS QoS Parameter Set, this attribute returns 0.
NOTE: This attribute has been deprecated and replaced with MaxReqPerSidCluster in the ServiceFlow object.




02/19/25                                             CableLabs®                                                      273
<!-- PDF_PAGE: 274 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


References: [MULPIv3.1] Maximum Requests per SID Cluster section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.35 MaxOutstandingBytesPerSidCluster
This attribute specifies the maximum number of bytes for which a CM can have requests outstanding on a given SID
Cluster. If defined number of bytes are outstanding and further requests are required, the CM must switch to a
different SID Cluster if one is available. A value of 0 indicates there is no limit. This attribute only applies to
upstream Service Flows in 3.0 operation, in other cases it is reported as 0. If the referenced parameter is not present
in the DOCSIS QoS Parameter Set, this attribute returns 0.
NOTE: This attribute has been deprecated and replaced with MaxOutstandingBytesPerSidCluster in the
      ServiceFlow object.
References: [MULPIv3.1] Maximum Outstanding Bytes per SID Cluster section in the Common Radio Frequency
Interface Encodings annex

G.2.3.2.36 MaxTotBytesReqPerSidCluster
This attribute specifies the maximum total number of bytes a CM can have requested using a given SID Cluster
before it must switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit.
This attribute only applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0. If the
referenced parameter is not present in the DOCSIS QoS Parameter Set, this attribute returns 0.
NOTE: This attribute has been deprecated and replaced with MaxTotBytesReqPerSidCluster in the ServiceFlow
      object.
References: [MULPIv3.1] Maximum Total Bytes Requested per SID Cluster section in the Common Radio
Frequency Interface Encodings annex

G.2.3.2.37 MaxTimeInSidCluster
This attribute specifies the maximum time in milliseconds that a CM may use a particular SID Cluster before it must
switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit. This attribute only
applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0. If the referenced parameter is
not present in the DOCSIS QoS Parameter Set, this attribute returns 0.
NOTE: This attribute has been deprecated and replaced with MaxTimeInSidCluster in the ServiceFlow object.
References: [MULPIv3.1] Maximum Time in the SID Cluster section in the Common Radio Frequency Interface
Encodings annex

G.2.3.2.38 PeakTrafficRate
This attribute represents the 4-byte value of the rate parameter 'P' of a token-bucket-based peak rate limiter for
packets of a service flow. A value of 0 signifies no Peak Traffic Rate is enforced. If the referenced parameter is not
present in the corresponding DOCSIS QoS Parameter Set, this attribute returns 0. The value of the
DataRateUnitSetting attribute defines the units of PeakTrafficRate.
References: [MULPIv3.1] Peak Traffic Rate section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.39 DsResequencing
This attribute specifies if a resequencing DSID needs to be allocated to the service flow.
The value 'notApplicable' indicates the value of this attribute is not applicable.
The value 'resequencingDsid' indicates that a resequencing DSID is required if the service flow is assigned to a
downstream bonding group
The value 'noResequencingDsid' indicates no resequencing DSID is associated with the service flow.




274                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 275 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


This attribute only applies to downstream Service Flows in 3.0 operation. If the referenced parameter is not present
in the corresponding downstream DOCSIS QoS Parameter Set, this attribute returns 'notApplicable'. This parameter
is not applicable to upstream DOCSIS QoS Parameter Sets, so the value 'notApplicable' is reported in that case.
References: [MULPIv3.1] Downstream Resequencing section in the Common Radio Frequency Interface Encodings
annex

G.2.3.2.40 MinimumBuffer
This attribute represents the configured minimum buffer size for the service flow.
References: [MULPIv3.1] Buffer Control section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.41 TargetBuffer
This attribute represents the configured target buffer size for the service flow. The value 0 indicates that no target
buffer size was configured, and the device will use a vendor-specific value.
References: [MULPIv3.1] Buffer Control section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.42 MaximumBuffer
This attribute represents the configured maximum buffer size for the service flow. The value 4294967295 indicates
that no maximum buffer size was configured, and thus there is no limit to the buffer size.
References: [MULPIv3.1] Buffer Control section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.43 AqmDisabled
If this attribute is set to 'true', AQM is disabled on the upstream or downstream service flow specified by
ServiceFlowId.
References: [MULPIv3.1] AQM Encodings section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.44 ClassicAqmLatencyTarget
This attribute provides the target latency for this service flow when operating under Classic Active Queue
Management (e.g., DOCSIS-PIE). This parameter will be ignored if the AQM Algorithm used by the Service Flow
is ImmediateAqm. For downstream service flows, the value 256 indicates an unknown latency target. The units are
in milliseconds.
References: [MULPIv3.1] AQM Encodings section in the Common Radio Frequency Interface Encodings annex

G.2.3.2.45 AqmAlgInUse
This attribute specifies the AQM algorithm in use for this service flow. If AQM is disabled on the service flow
specified by ServiceFlowId, this attribute returns unknown(1).
The value unknown(1) is reported for downstream service flows or when AQM is disabled.
The value other(2) indicates a vendor proprietary algorithm for upstream queue management.
The value docsisPIE(3) indicates the upstream queue management Proportional Integral controller Enhanced (PIE)
algorithm.
The value immediateAqm(4) indicates the use of the Immediate AQM algorithm.
References: [MULPIv3.1] Proportional-Integral-Enhanced Active Queue Management Algorithm Annex and
Immediate Active Queue Management annex




02/19/25                                             CableLabs®                                                      275
<!-- PDF_PAGE: 276 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


G.2.3.2.46 ImmedAqmMaxThreshold
This attribute specifies the maximum threshold in microseconds of the ramp function used by the Immediate AQM
algorithm and the Queue Protection algorithm. This attribute reports the actual ImmedAqmMaxThreshold (MAXTH
calculated per the MULPI Annex C, Annex N), rather than the configured value.

G.2.3.2.47 ImmedAqmRangeExponentRampFunction
This attribute specifies the range in nanoseconds of the ramp function used by the Immediate AQM algorithm and
the Queue Protection algorithm. It is expressed as an exponent of 2, e.g., a value of 19 means the range of the ramp
will be 2^19 = 524288 ns (roughly 524 µs).

G.2.3.2.48 DataRateUnitSetting
This attribute indicates the base unit for the Service Flow traffic rate attributes Maximum Sustained Traffic Rate
(MaxTrafficRate), Minimum Reserved Traffic Rate (MinReservedRate), Peak Traffic Rate (PeakTrafficRate), and
Guaranteed Grant Rate (GuaranteedGrantRate). The value of this attribute allows for their interpretation in units of
bps, kbps, Mbps, or Gbps. The default value for DataRateUnitSetting is bits per second (bps).

             AggregateServiceFlow
This object describes the set of DOCSIS-QoS Aggregate Service Flows in a managed device.
References: [MULPIv3.1] Common Upstream and Downstream Quality-of-Service Parameter Encodings section
                               Table 126 - AggregateServiceFlow Object Attributes

        Attribute Name               Type          Access        Type Constraints                Units        Default
 IfIndex                       InterfaceIndex     Key        Interface Index of MAC
                                                             Domain interface
 Id                            UnsignedInt        Key
 Direction                     IfDirection        R/O
 Priority                      UnsignedByte       R/O
 MaxAggregateTrafficRate       BitRate            R/O
 MaxTrafficBurst               UnsignedInt        R/O
 MinReservedRate               BitRate            R/O
 MinReservedPkt                UnsignedShort      R/O                                    bytes
 PeakTrafficRate               BitRate            R/O
 DataRateUnitSetting           DataRateUnitType   R/O
 LowLatencyAsf                 Boolean            R/O
 LowLatencySfid                UnsignedInt        R/O
 ClassicSfScn                  AdminString        R/O
 LowLatencySfScn               AdminString        R/O
 AqmCouplingFactor             UnsignedByte       R/O        0..255                      tenths
 SchedulingWeight              UnsignedByte       R/O
 QpEnable                      EnumBits           R/O        queueProtectionEnabled(0)
 QpLatencyThreshold            UnsignedShort      R/O                                    microseconds
 QpQueuingScoreThreshold       UnsignedShort      R/O                                    microseconds
 QpDrainRateExponent           UnsignedByte       R/O                                    log2(bytes/second)


G.2.3.3.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.




276                                                CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 277 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


G.2.3.3.2 Id
This key represents an identifier assigned to an Aggregate Service Flow by CMTS within a MAC Domain.

G.2.3.3.3 Direction
This attribute represents the direction of the Aggregate Service Flow.

G.2.3.3.4 Priority
This attribute represents the relative priority of the Aggregate Service Flow.

G.2.3.3.5 MaxAggregateTrafficRate
This attribute represents the 4-byte value of the maximum sustained traffic rate allowed for this Aggregate Service
Flow. The value of the DataRateUnitSetting attribute defines the units of MaxAggregateTrafficRate.

G.2.3.3.6 MaxTrafficBurst
This attribute represents the token bucket size in bytes for this parameter set. The value is calculated from the byte
following the MAC header HCS to the end of the CRC. This object is applied in conjunction with MaxTrafficRate
to calculate maximum sustained traffic rate the token bucket size B (in bytes) for this Service Flow or an Aggregate
Service Flow.

G.2.3.3.7 MinReservedRate
This attribute represents the 4-byte value of the guaranteed minimum rate allowed for this Aggregate Service Flow.
The value of the DataRateUnitSetting attribute defines the units of MinReservedRate.

G.2.3.3.8 MinReservedPkt
This attribute represents Assumed Minimum Reserved Rate Packet Size for the Aggregate Service Flow.

G.2.3.3.9 PeakTrafficRate
This attribute represents the 4-byte value of the guaranteed minimum rate allowed for this Aggregate Service Flow.
The value of the DataRateUnitSetting attribute defines the units of PeakTrafficRate. A value of 0 means the peak
traffic rate is not limited.

G.2.3.3.10 DataRateUnitSetting
This attribute indicates the base unit for the AggregateServiceFlow traffic rate attributes Maximum Aggregate
Traffic Rate (MaxAggregateTrafficRate), Minimum Reserved Traffic Rate (MinReservedRate), and Peak Traffic
Rate (PeakTrafficRate). The value of this attribute allows for their interpretation in units of bps, kbps, Mbps, or
Gbps.

G.2.3.3.11 LowLatencyAsf
This attribute indicates whether this ASF is a Low Latency ASF.

G.2.3.3.12 LowLatencySfid
This attribute represents which of the constituent Service Flows within this Aggregate Service Flow is the Low
Latency Service Flow.

G.2.3.3.13 ClassicSfScn
This attribute represents the Service Class Name that was used to instantiate the Classic Service Flow by the CMTS.
This attribute may not be known by the CM, and could report an empty string.




02/19/25                                            CableLabs®                                                     277
<!-- PDF_PAGE: 278 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.3.3.14 LowLatencySfScn
This attribute represents the Service Class Name that was used to instantiate the Low Latency Service Flow by the
CMTS. This attribute may not be known by the CM, and could report an empty string.

G.2.3.3.15 AqmCouplingFactor
This attribute represents the coupling factor for the AQMs between the Classic Service Flow and the Low Latency
Service Flow.

G.2.3.3.16 SchedulingWeight
This attribute represents the value of this parameter defines the amount of scheduling weight the CMTS assigns to
the Low Latency Service Flow of the ASF.

G.2.3.3.17 QpEnable
This attribute indicates the configured Queue Protection status of this Aggregate Service Flow. If the
queueProtectionEnabled bit (bit 0) is set to '0', Queue Protection is disabled. If the queueProtectionEnabled bit is set
to '1', Queue Protection is enabled.
References: [MULPIv3.1] Queue Protection section

G.2.3.3.18 QpLatencyThreshold
This attribute represents the latency threshold for the Queue Protection function in the Latency Queue.
References: [MULPIv3.1] Queue Protection section

G.2.3.3.19 QpQueuingScoreThreshold
This attribute represents Queuing Score Threshold for the Queue Protection function in the Latency Queue.

G.2.3.3.20 QpDrainRateExponent
This attribute represents the drain rate exponent for the Queue Protection function in the Low Latency Service Flow.
The drain rate of the queuing score is expressed as an exponent of 2, in bytes/sec, e.g., a value of 17 means the
Queue Protection function will use a value of: 2^17 bytes/sec.

             ServiceFlow
The ServiceFlow object describes the set of DOCSIS-QoS Service Flows in a managed device.
References: [MULPIv3.1] Service Flows and Classifiers section
                                     Table 127 - ServiceFlow Object Attributes

             Attribute Name                   Type        Access         Type Constraints            Units      Default
 IfIndex                               InterfaceIndex     Key        Interface Index of MAC
                                                                     Domain interface
 Id                                    UnsignedInt        Key
 Sid                                   UnsignedShort      R/O        0..16383
 Direction                             IfDirection        R/O
 Primary                               Boolean            R/O
 ParamSetTypeStatus                    EnumBits           R/O        active(0)
                                                                     admitted(1)
                                                                     provisioned(2)
 ChSetId                               ChSetId            R/O
 AttrAssignSuccess                     Boolean            R/O
 Dsid                                  Dsid               R/O




278                                                     CableLabs®                                            02/19/25
<!-- PDF_PAGE: 279 -->
Cable Modem Operations Support System Interface Specification                           CM-SP-CM-OSSIv3.1-I27-250219


           Attribute Name                   Type           Access           Type Constraints               Units    Default
 MaxReqPerSidCluster                   UnsignedByte       R/O                                        requests
 MaxOutstandingBytesPerSidCluster      UnsignedInt        R/O                                        bytes
 MaxTotBytesReqPerSidCluster           UnsignedInt        R/O                                        bytes
 MaxTimeInSidCluster                   UnsignedShort      R/O                                        milliseconds
 BufferSize                            UnsignedInt        R/O                                        bytes
 AggregateServiceFlowId                UnsignedInt        R/O
 Type                                  Enum               R/O            other(1),
                                                                         standalone(2),
                                                                         classic(3),
                                                                         lowLatency(4),
                                                                         nonLldHqos(5),
                                                                         reserved(6)
 AllowedAqBytes                        UnsignedInt        R/O                                        bytes


                                    Table 128 - ServiceFlow Object Associations

      Associated                                 Type                                 Near-end       Far-end         Label
     Object Name                                                                     Multiplicity   Multiplicity
 AggregateServiceFlow     Directed association to AggregateServiceFlow           0..2               0..1
 ServiceFlowStats         Directed association to ServiceFlowStats               1                  0..1
 CmServiceUsStats         Directed association to CmServiceUsStats               1                  0..1
 SfLatencyStats           Directed association to SfLatencyStats                 1                  0..1
 SfLatencyHistCfg         Directed association to SfLatencyHistCfg               1                  0..1
 SfCongestionStats        Directed association to SfCongestionStats              1                  1..*



G.2.3.4.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow.

G.2.3.4.2 Id
This key represents an identifier assigned to a Service Flow by CMTS within a MAC Domain. The value 0 is used
only for the purpose of reporting instances of the PktClass object pertaining UDCs and not used for association of
QoS classifiers to service flows.
References: [MULPIv3.1] Service Flow Identifier section in the Common Radio Frequency Interface Encodings
annex

G.2.3.4.3 Sid
Service Identifier (SID) assigned to an admitted or active Service Flow. This attribute reports a value of 0 if a
Service ID is not associated with the Service Flow. Only active or admitted upstream Service Flows will have a
Service ID (SID).
References: [MULPIv3.1] Service Identifier section in the Common Radio Frequency Interface Encodings annex

G.2.3.4.4 Direction
This attribute represents the direction of the Service Flow.

G.2.3.4.5 Primary
This attribute reflects whether Service Flow is the primary or a secondary Service Flow.




02/19/25                                               CableLabs®                                                      279
<!-- PDF_PAGE: 280 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


G.2.3.4.6 ParamSetTypeStatus
This attribute represents the status of the service flow based on the admission state. 'active' bit set to '1' indicates that
the service flow is active and that the corresponding QoS ParamSet is stored in the CMTS. 'admitted' bit set to '1'
indicates that the service flow resources were reserved and that the corresponding QoS ParamSet is stored in the
CMTS. 'provisioned' bit set to '1' indicates that the service flow was defined in the CM config file and that the
corresponding QoS ParamSet is stored in the CMTS.
References: [MULPIv3.1] Service Flow section

G.2.3.4.7 ChSetId
This attribute represents the Channel Set Id associated with the service flow.

G.2.3.4.8 AttrAssignSuccess
If set to 'true', this attribute indicates that the current channel set associated with the service flow meets the Required
and Forbidden Attribute Mask encodings. Since this attribute is not applicable for a CM, the CM always returns
'false'.
References: [MULPIv3.1] Service Flow section

G.2.3.4.9 Dsid
This attribute indicates the DSID associated with the downstream service flow. downstream service flows without a
DSID or upstream Service Flows report the value zero.

G.2.3.4.10 MaxReqPerSidCluster
This attribute specifies the maximum number of requests that a CM can make within a given SID Cluster before it
must switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit. This
attribute only applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0.
References: [MULPIv3.1] Maximum Requests per SID Cluster section in the Common Radio Frequency Interface
Encodings annex

G.2.3.4.11 MaxOutstandingBytesPerSidCluster
This attribute specifies the maximum number of bytes for which a CM can have requests outstanding on a given SID
Cluster. If defined number of bytes are outstanding and further requests are required, the CM must switch to a
different SID Cluster if one is available. A value of 0 indicates there is no limit. This attribute only applies to
upstream Service Flows in 3.0 operation, in other cases it is reported as 0.
References: [MULPIv3.1] Maximum Outstanding Bytes per SID Cluster section in the Common Radio Frequency
Interface Encodings annex

G.2.3.4.12 MaxTotBytesReqPerSidCluster
This attribute specifies the maximum total number of bytes a CM can have requested using a given SID Cluster
before it must switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit.
This attribute only applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0.
References: [MULPIv3.1] Maximum Total Bytes Requested per SID Cluster section in the Common Radio
Frequency Interface Encodings annex

G.2.3.4.13 MaxTimeInSidCluster
This attribute specifies the maximum time in milliseconds that a CM may use a particular SID Cluster before it must
switch to a different SID Cluster to make further requests. A value of 0 indicates there is no limit. This attribute only
applies to upstream Service Flows in 3.0 operation, in other cases it is reported as 0.




280                                                    CableLabs®                                                  02/19/25
<!-- PDF_PAGE: 281 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Maximum Time in the SID Cluster section in the Common Radio Frequency Interface
Encodings annex

G.2.3.4.14 BufferSize
This attribute indicates the buffer size for the service flow. For the CM, this attribute only applies to upstream
Service Flows, for the CMTS this attribute only applies to downstream Service Flows, in other cases it is reported
as 0.
References: [MULPIv3.1] Buffer Control section in the Common Radio Frequency Interface Encodings annex

G.2.3.4.15 AggregateServiceFlowId
This attribute indicates the Aggregate Service flow to which this Service Flow belongs. If this Service Flow is not
part of an ASF, this attribute returns a '0'.

G.2.3.4.16 Type
This attribute reports the type of Service Flow.
'other' - Represents an undefined Service Flow type.
'standalone' - Represents a standalone Service Flow that is not associated with an ASF.
'classic' - Represents a classic Service Flow associated with a Low Latency ASF.
'lowLatency' - Represents a Low Latency Service Flow associated with a Low Latency ASF.
'nonLldHqos' - Represents a Service Flow that is a constituent of an Aggregate Service Flow (ASF) but when the
ASF is not a Low Latency ASF.
'reserved' - Reserved value used by IPDR Service Definitions.

G.2.3.4.17 AllowedAqBytes
This attribute reports the estimate of the maximum number of bytes that are expected to be held in the upstream
transmit queue simply because of the media access process (request/grant delay and PGS grant spacing), as
calculated in the calcAllowedAQ function for each upstream Low Latency Service Flow. For other Service Flow
types, the CM MAY report 0 for the AllowedAqBytes attribute.
References: [MULPIv3.1] allowed_AQ_bytes parameter calculated via the 'calcAllowedAQ()' function in
Annex O.1 AQM Utility Functions

           ServiceFlowSidCluster
This object defines the SID clusters associated with an upstream service flow.
References: [MULPIv3.1] Service Flow SID Cluster Assignments section in the Common Radio Frequency
Interface Encodings annex
                               Table 129 - ServiceFlowSidCluster Object Attributes

   Attribute Name             Type         Access                 Type Constraints                Units     Default
 IfIndex               InterfaceIndex     Key          Interface Index of MAC Domain interface
 ServiceFlowId         UnsignedInt        Key          1..4294967295
 Id                    UnsignedByte       Key          0..7
 Ucid                  ChId               Key          1..255
 Sid                   UnsignedShort      R/O          1..16383



G.2.3.5.1 IfIndex
This key represents the interface index of the MAC Domain of the Service Flow SID cluster.


02/19/25                                            CableLabs®                                                   281
