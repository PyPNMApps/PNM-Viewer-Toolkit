# Annex G.2.2 - RCC Status Objects

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 300-303
Classification: Normative (explicit)

<!-- PDF_PAGE: 300 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


           RcpId
This data type defines a 'Receive Channel Profile Identifier' (RCP-ID). An RCP-ID consists of 5-octet length string
where the first 3-bytes (from left to right corresponds to the Organizational Unique ID (OUI) followed by a two-byte
vendor-maintained identifier to represent multiple versions or models of RCP-IDs.
References: [MULPIv4.0] RCP-ID section in the Common Radio Frequency Interface Encodings Annex

           ScdmaSelectionString
This data type represents the S-CDMA selection string for active codes used with Selectable Active Codes Mode 2.
A 128-bit string indicating which codes are active. The first element in the string corresponds to code 0 (the all-ones
code), and the last element in the string corresponds to code 127. A '1' element in the string indicates an active code,
and a '0' indicates an unused code. A zero-length string is returned for an unknown or non-applicable value.
References: [MULPIv4.0] Minislot Numbering Parameters in Timing and Synchronization section

           SchedulingType
The scheduling service provided by a CMTS for an upstream Service Flow. This parameter needs to be reported as
'undefined' for downstream QoS Parameter Sets.
Valid enumerations for the data type are as follows.
      •   undefined(1)
      •   bestEffort(2)
      •   nonRealTimePollingService(3)
      •   realTimePollingService(4)
      •   unsolicitedGrantServiceWithAD(5)
      •   unsolicitedGrantService(6)
      •   proactiveGrantService(7)
References: [MULPIv4.0] Service Flow Scheduling Type section

G.2.2     RCC Status Objects
This section defines the CM Receive Channel Configuration (RCC) Status objects.




300                                                 CableLabs                                                04/29/26
<!-- PDF_PAGE: 301 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429




                                       Figure 17 - RCC Status Information Model


           RxModuleStatus
The Receive Module Status object provides a read-only view of the statically configured and dynamically created
Receive Modules within an RCC. When this object is defined on the CM, the value of RccStatusId is always 1.
                                  Table 168 - RxModuleStatus Object Attributes

   Attribute Name               Type           Access              Type Constraints                Units   Default
 IfIndex                InterfaceIndex        Key         InterfaceIndex of MAC Domain interface
 RcpId                  RcpId                 Key
 RccStatusId            UnsignedByte          Key         1..255
 RmId                   UnsignedByte          Key         1..255
 RmRmConnectivityId     UnsignedByte          R/O
 FirstCenterFrequency   UnsignedInt           R/O                                                  Hz


G.2.2.1.1 IfIndex
This key represents the interface index of the MAC Domain to which this instance applies.

G.2.2.1.2 RcpId
This key represents the RCP-ID to which this instance applies.




04/29/26                                            CableLabs                                                 301
<!-- PDF_PAGE: 302 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


G.2.2.1.3 RccStatusId
This key represents an RCC combination for a particular RcpId either from an RCC configuration object or a CMTS
determined RCC and is unique per combination of MAC Domain interface index and RcpId. Note that when this
attribute is instantiated at the CM, its value will always be 1.

G.2.2.1.4 RmId
This key represents an identifier of a Receive Module instance within the Receive Channel Profile.
References: [MULPIv4.0] Receive Module Index section in the Common Radio Frequency Interface Encodings
Annex

G.2.2.1.5 RmRmConnectivityId
This attribute represents the Receive Module to which this Receive Module connects. Requirements for module
connectivity are detailed in the RmRmConnectivityId of the RccCfg object.

G.2.2.1.6 FirstCenterFrequency
This attribute represents the low frequency channel of the Receive Module, or 0 if not applicable to the Receive
Module.

             Pre-DOCSIS 3.1/4.0 RxChStatus
The Receive Channel Status object reports the status of the statically-configured and dynamically-created Receive
Channels within an RCC. When this object is defined on the CM, the value of RccStatusId is always 1.
This object provides the ability for the CM to report its receive channel configuration and is applicable for cases
where a DOCSIS 3.1 or DOCSIS 4.0 CM registers with a DOCSIS 3.0 CCAP.
                                       Table 169 - RxChStatus Object Attributes

   Attribute Name               Type           Access               Type Constraints                Units     Default
 IfIndex                InterfaceIndex       Key          InterfaceIndex of MAC Domain interface
 RcpId                  RcpId                Key
 RccStatusId            UnsignedByte         Key          1..255
 RcId                   UnsignedByte         Key          1..255
 ChIfIndex              InterfaceIndex       R/O          InterfaceIndex of Downstream Channel
                                                          assigned to the Receive Channel
 PrimaryDsIndicator     Boolean              R/O
 RcRmConnectivityId     UnsignedByte         R/O



G.2.2.2.1 IfIndex
This key represents the interface index of the MAC Domain to which this instance applies.

G.2.2.2.2 RcpId
This key represents the RCP-ID to which this instance applies.

G.2.2.2.3 RccStatusId
This key represents an RCC combination for a particular RcpId either from an RCC configuration object or a CMTS
determined RCC. It is unique per combination of MAC Domain interface index and RcpId. Note that when this
attribute is instantiated at the CM, its value will always be 1.




302                                                 CableLabs                                               04/29/26
<!-- PDF_PAGE: 303 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


G.2.2.2.4 RcId
This key represents an identifier for the parameters of the Receive Channel instance within the Receive Channel
Profile.

G.2.2.2.5 ChIfIndex
This attribute contains the interface index of the Downstream Channel that this Receive Channel Instance defines.

G.2.2.2.6 PrimaryDsIndicator
If set to 'true', this attribute indicates the Receive Channel is to be the primary-capable downstream channel for the
CM receiving this RCC. Otherwise, the downstream channel is to be a non-primary-capable channel.

G.2.2.2.7 RcRmConnectivityId
This attribute identifies the Receive Module to which this Receive Channel connects. A value of zero indicates that
the Receive Channel Connectivity TLV is omitted from the RCC.

           RxChStatus
The Receive Channel Status object reports the status of the statically-configured and dynamically-created Receive
Channels within an RCC.
This object provides the ability for the CM to report its receive channel configuration and is applicable for cases
where a DOCSIS 4.0 CM registers with a DOCSIS 4.0 CCAP.
                                         Table 170 - RxChStatus Object Attributes

   Attribute Name               Type                Access            Type Constraints               Units     Default
 IfIndex                InterfaceIndex             Key         SC-QAM or OFDM Index
 PrimaryDsIndicator     PrimaryDsIndicatorType     R/O
 OfdmProfiles           EnumBits                   R/O



G.2.2.3.1 IfIndex
This key represents the SC-QAM or OFDM interface index to which this instance applies.

G.2.2.3.2 PrimaryDsIndicator
This key attribute encodes the type of downstream channel.

G.2.2.3.3 OfdmProfiles
This attribute identifies the downstream channel profiles provisioned on the CM. An example of EnumBits follows.
    Example 1: A cable modem configured with OFDM Profiles 3 and 8 would return a query response as follows.
    EnumBits: 0001000010000000 or 0x1080.
    Example 2: A cable modem configured with an SC_QAM Channel would return a query response as follows.
    EnumBits: 000000000000000 or 0x0.




04/29/26                                                 CableLabs                                                   303
