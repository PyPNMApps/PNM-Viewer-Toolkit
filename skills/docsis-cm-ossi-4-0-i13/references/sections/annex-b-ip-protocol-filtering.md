# Annex B - IP Protocol and LLC Filtering and Classification

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 158-161
Classification: Normative (explicit)

<!-- PDF_PAGE: 158 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


Annex B           IP Protocol and LLC Filtering and Classification
                  (Normative)
DOCSIS 3.0 CMs supported two packet filtering/classification methods consisting of the legacy IP filtering
mechanism specified in [RFC 4639] and Upstream Drop Classifiers (UDCs). A DOCSIS 4.0 CM is only required to
support UDCs and is not required to support legacy [RFC 4639] IP and LLC filtering mechanisms.
UDCs are modeled on the existing QoS Classifiers that were introduced in DOCSIS 1.1. UDCs apply only to the
CM, the RF interface and only in the upstream direction of flow. The use of UDCs facilitates delegation of upstream
protocol filtering at the CM through parameters in the configuration file that can be controlled by the CMTS. Any
packet classified by the Upstream Drop Classifier rule is discarded, conceptually similarly to directing an IP route to
'null 0' or output to /dev/null in a UNIX system.
As with legacy IP filters, UDC rules may be configured through the CM configuration file statically, assigned
dynamically from the CMTS through a Group ID reference in the CM configuration file, dynamically added,
changed or deleted after registration through a DSC (Dynamic Service Change) MAC management message from
the CMTS, or both the static and dynamic configuration methods may be used together. The CMTS alone provides
the downstream protocol filtering/classification and can further reinforce the upstream classification policy through
Subscriber Management traffic filtering functionality if desired.
Among the specific requirements for classification at the CM, the CM is required to perform protocol classification
from the host CPE(s) to the RF interface when UDCs are enabled. All IP and LLC packets will be forwarded from
the CMCI interface to the RFI upstream interface based on rules outlined in the Upstream Drop Classifiers section
of [MULPIv4.0], unless they are specifically required to be discarded according to applied protocol classification
rules.

B.1 Filtering Mechanisms
The legacy DOCSIS filters are subdivided into two (2) filtering layers (LLC and IP) at the CM. The two legacy
classification/filtering tables at the CM are the docsDevFilterIpTable and the docsDevFilterLlcTable. Upstream
Drop Classifiers cover both the LLC and IP packet criterion, matching the functionality of the legacy filtering
mechanisms.

B.1.1    LLC Filters
A DOCSIS 4.0 CM is not required to support [RFC 4639] LLC Filters.

B.1.2    Special filters
Special filters include IP spoofing filters, inter-eSAFE and eSAFE to CPE communications and SNMP access filters
such as SNMPv1/v2c NmAccess mode (see Section 8.5.2.2) and SNMP CPE Access Control (see Section 8.5.2.9).

           IP Spoofing Filters
A DOCSIS 4.0 CM is not required to support [RFC 4639] IP Spoofing Filters.

           Additional Requirement on dot1dTpFdbTable ([RFC 4188])
CM CPE MAC addresses learned via the CM configuration file MUST set the dot1dTpFdbStatus to "mgmt". It is
assumed that the number of "mgmt"-configured CM CPE MAC addresses is less than, or equal to, the TLV type-18
value (Maximum Number of CPE).

           SNMP Access Filter
When the CM is operating in SNMPv1/v2c NmAccess mode, the CM MUST apply the SNMP access filters to
SNMP packets entering from any interface and destined for the CM. The CM MUST apply SNMP access filters
after IP spoofing filters for the packets entering the CM from the CMCI interface. Since SNMP access filter function




158                                                 CableLabs                                               04/29/26
<!-- PDF_PAGE: 159 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


is controlled by docsDevNmAccessTable, SNMP access filter is available and applies only when the CM is in
SNMP v1/v2c NmAccess mode.
When the CM is operating in SNMP Coexistence mode, SNMP access MUST be controlled and specified by the
MIB objects in [RFC 3411] through [RFC 3415], and [RFC 3584].
CMs may have multiple interfaces. If SNMP access filters are applied to CM IfIndex 1, the CM MUST apply the
same filters to the "Additional CPE interfaces" (see Section 7.1.3.11.1).

B.1.2.3.1     docsDevNmAccessIP and docsDevNmAccessIpMask
A CM that implements docsDevNmAccessTable MUST apply the following rules in order to determine whether to
permit SNMP access from a given source IP address (SrcIpAddr):
    1.     If (docsDevNmAccessIp == "255.255.255.255"), the CM MUST permit the access from any SrcIpAddr.
    2.     If ((docsDevNmAccessIp AND docsDevNmAccessIpMask) == (SrcIpAddr AND
           docsDevNmAccessIpMask)), the CM MUST permit the access from SrcIpAddr.
    3.     If (docsDevNmAccessIp == "0.0.0.0" AND docsDevNmAccessIpMask != '255.255.255.255'), the CM
           MUST permit access from any SrcIpAddr.
    4.     If neither #1, #2, or #3 is applied, the CM MUST NOT permit the access from SrcIpAddr.
The CM's default value of the docsDevNmAccessIpMask MUST be set to "0.0.0.0".
The following table contains sample MIB values and the access granted by those values.
                                   Table 75 - Sample docsDevNmAccessIp Values

            docsDevNmAccessIp                      docsDevNmAccessIpMask                            Access
 255.255.255.255                             Any IP Address Mask                          Any NMS
 Any IP Address                              0.0.0.0                                      Any NMS
 Any IP Address except 255.255.255.255       255.255.255.255                              Single NMS
 0.0.0.0                                     255.255.255.255                              No NMS (disables all access)
 0.0.0.0                                     Any IP Address Mask except 255.255.255.255   Any NMS


If the CMTS implements docsDevNmAccessTable, the same rules as stated above for the CM are followed.

B.1.3      IP Protocol Filtering
A DOCSIS 4.0 CM is not required to support [RFC 4639] IP Filters.

B.1.4      Protocol Classification Through Upstream Drop Classifiers
The Upstream Drop Classifier (UDC) is a structural convention re-using the definition of upstream classifiers from
[MULPIv4.0]. A unique top-level TLV (Upstream Drop Packet Classification Encoding, TLV 60) defines UDCs
and distinguishes this type of classifier from the QoS classifier type (Upstream Packet Classification Encoding, TLV
22). UDCs are used to discard a packet matched to the classifier rule criteria. See the Upstream Drop Packet
Classification Encoding section in the Common Radio Frequency Interface Encodings Annex of [MULPIv4.0] for
more details.
The CM is required to support a minimum of 64 UDC rules.
The Upstream Drop Classifier configuration structure is strictly designed to discard packets before they reach the
output queue of the RFI interface and does not require attributes such as QoS. Upstream Drop Classifiers have a
many-to-one relationship between UDC rules and the packet discard function. UDCs operate only within the local
context of the CM. Any packet matched by a classifier rule is immediately discarded.
The CM will ignore UDC parameters which are incompatible with the packet discard function when they are
configured in the CM configuration file.



04/29/26                                           CableLabs                                                      159
<!-- PDF_PAGE: 160 -->
CM-SP-CM-OSSIv4.0-I13-260429                                        Data-Over-Cable Service Interface Specifications


           IP Classification Rule Order Priority
QoS rule priority generally supersedes drop rules, though this is a configuration decision and not dictated in these
specifications. For example, during a viral outbreak or DoS attack, it may be preferable to apply drop rules with
higher priority relative to QoS rules to more efficiently drop packets that match those associated with a virus, worm,
or DoS attack.
For the purposes of classifying IP protocols, the following objects listed in the second column are encoded within
TLV 60 and shown in comparison with [RFC 2669] to construct L3/L4 rule criteria to enforce the operator's security
policy.
            Table 76 - Mapping of docsDevFilterIpTable (RFC 2669) to UDCs for Layer 3 & 4 Criteria

        IP Filters [RFC 2669]                   UDC TLV 60 Encodings                                  Description
 docsDevFilterIpIndex                     Id                                        Rule index
 docsDevFilterIpControl                   - no equivalent                           discard, accept, policy(*1)
 docsDevFilterIpIfIndex                   CMIM                                      CM interface(s)(*2)
 docsDevFilterIpDirection                 - no equivalent                           inbound, outbound, both(*3)
 docsDevFilterIpBroadcast                 - no equivalent                           Broadcast and multicast or all packets
 - no equivalent                          Rule priority                             Directs order of processing
 docsDevFilterIpStatus                    - no equivalent                           Activation state(*4)
 docsDevFilterIpProtocol                  IpProtocol                                IP transport type, e.g., TCP, UDP
 - no equivalent                          FlowLabel                                 IPv6 flow label
 docsDevFilterIpSaddr                     IpSourceAddr                              IP source address/prefix
 docsDevFilterIpSmask                     IpSourceMask                              IP source mask/prefix length
 docsDevFilterIpDaddr                     IpDestAddr                                IP destination Address/prefix
 docsDevFilterIpDmask                     IpDestMask                                IP destination mask/prefix length
 docsDevFilterIpTos                       IpTosLow                                  Legacy type of service range low
                                          IpTosHigh                                 Legacy type of service range high
 docsDevFilterIpTosMask                   IpTosMask                                 Legacy type of service mask
 docsDevFilterIpSourcePortLow             SourcePortStart                           TCP/UDP source port range start
 docsDevFilterIpSourcePortHigh            SourcePortEnd                             TCP/UDP source port range end
 docsDevFilterIpDestPortLow               DestPortStart                             TCP/UDP source port range start
 docsDevFilterIpDestPortHigh              DestPortEnd                               TCP/UDP source port range end
 docsDevFilterIpContinue                  - no equivalent                           Continue comparing rules on matches(*5)
 docsDevFilterIpPolicyId                  - no equivalent                           Extensions for other criterion
 TABLE NOTES:
 (*1) UDCs only perform discard actions.
 (*2) CMIM allows for multiple interfaces per rule, while [RFC 2669] aggregates only CPE interface.
 (*3) UDCs only perform upstream filtering.
 (*4) UDCs are always active. The SNMP docsDevFilterIpTable table provides RowStatus for controlling the activation state of IP
 filters.
 (*5) UDCs do not continue performing packet comparisons after a match.


The SNMP table docsQosPktClassTable from [DOCS-QOS3-MIB] is used for reporting of both QoS Classifiers and
Drop Classifiers at the CM. The docsQosPktClassPkts object within docsQosPktClassTable is used to count packet
matches to each classifier rule.

           LLC/MAC Classification Through UDCs
L2 criteria such as MAC address source and destination, header type, 802.1p/q VLAN tag or user_priority and cable
modem interface mask (CMIM) may be used to classify packets as deemed necessary by the operator.




160                                                       CableLabs                                                    04/29/26
<!-- PDF_PAGE: 161 -->
Cable Modem Operations Support System Interface Specification               CM-SP-CM-OSSIv4.0-I13-260429


For the purposes of classifying MAC protocols, the following variables listed in the second column are encoded
within TLV 60 and shown in comparison with [RFC 2669]. The variables described here are used to construct L2
rule criteria to enforce the operator's security policy.
                   Table 77 - Upstream Drop Classification Values for LLC/MAC Classification

        LLC Filters [RFC 2669]                UDC TLV 60 Encodings                          Description
 docsDevFilterLLCIndex                 Id                                    Rule index
 docsDevFilterLLCIfIndex               CMIM                                  CM interface
 - no equivalent                       Rule priority                         Directs order of processing
 docsDevFilterLLCStatus                - no equivalent                       Activation state
 - no equivalent                       SourceMacAddr                         Source MAC address
 - no equivalent                       DestMacAddr                           Destination MAC address
 docsDevFilterLLCProtocolType          EnetProtocolType                      Ethernet protocol type
 docsDevFilterLLCProtocol              EnetProtocol                          Ethernet protocol
 - no equivalent                       802.1p User priority low              Ethernet user priority range low
 - no equivalent                       802.1p User priority high             Ethernet user priority range high
 - no equivalent                       VLAN ID                               12-bit Ethernet VLAN ID


The SNMP table docsQosPktClassTable from DOCS-QOS3-MIB is used for reporting of both QoS Classifiers and
Drop Classifiers at the CM. The docsQosPktClassPkts object within docsQosPktClassTable is used to count packet
matches to each classifier rule.




04/29/26                                           CableLabs                                                    161
