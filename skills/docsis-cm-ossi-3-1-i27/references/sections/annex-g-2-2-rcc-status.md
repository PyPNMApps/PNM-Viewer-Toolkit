# Annex G.2.2 - RCC Status Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 253-256
Classification: Normative (explicit)

<!-- PDF_PAGE: 253 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


A 128-bit string indicating which codes are active. The first element in the string corresponds to code 0 (the all-ones
code), and the last element in the string corresponds to code 127. A '1' element in the string indicates an active code,
and a '0' indicates an unused code. A zero-length string is returned for an unknown or non-applicable value.
References: [MULPIv3.1] Minislot Numbering Parameters in Timing and Synchronization section

           SchedulingType
The scheduling service provided by a CMTS for an upstream Service Flow. This parameter must be reported as
'undefined' for downstream QoS Parameter Sets.
Valid enumerations for the data type are:
    •    undefined(1)
    •    bestEffort(2)
    •    nonRealTimePollingService(3)
    •    realTimePollingService(4)
    •    unsolicitedGrantServiceWithAD(5)
    •    unsolicitedGrantService(6)
    •    proactiveGrantService(7)
References: [MULPIv3.1] Service Flow Scheduling Type section

G.2.2    RCC Status Objects
This section defines the CM Receive Channel Configuration (RCC) Status objects.




02/19/25                                             CableLabs®                                                     253
<!-- PDF_PAGE: 254 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications




                                       Figure 16 - RCC Status Information Model


           RxModuleStatus
The Receive Module Status object provides a read-only view of the statically configured and dynamically created
Receive Modules within an RCC. When this object is defined on the CM, the value of RccStatusId is always 1.
                                  Table 121 - RxModuleStatus Object Attributes

   Attribute Name               Type           Access              Type Constraints                Units   Default
 IfIndex                InterfaceIndex        Key         InterfaceIndex of MAC Domain interface
 RcpId                  RcpId                 Key
 RccStatusId            UnsignedByte          Key         1..255
 RmId                   UnsignedByte          Key         1..255
 RmRmConnectivityId     UnsignedByte          R/O
 FirstCenterFrequency   UnsignedInt           R/O                                                  Hz



G.2.2.1.1 IfIndex
This key represents the interface index of the MAC Domain to which this instance applies.




254                                                 CableLabs®                                             02/19/25
<!-- PDF_PAGE: 255 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


G.2.2.1.2 RcpId
This key represents the RCP-ID to which this instance applies.

G.2.2.1.3 RccStatusId
This key represents an RCC combination for a particular RcpId either from an RCC configuration object or a CMTS
determined RCC and is unique per combination of MAC Domain interface index and RcpId. Note that when this
attribute is instantiated at the CM, its value will always be 1.

G.2.2.1.4 RmId
This key represents an identifier of a Receive Module instance within the Receive Channel Profile.
References: [MULPIv3.1] Receive Module Index section in the Common Radio Frequency Interface Encodings
annex

G.2.2.1.5 RmRmConnectivityId
This attribute represents the Receive Module to which this Receive Module connects. Requirements for module
connectivity are detailed in the RmRmConnectivityId of the RccCfg object.

G.2.2.1.6 FirstCenterFrequency
This attribute represents the low frequency channel of the Receive Module, or 0 if not applicable to the Receive
Module.

             Pre-DOCSIS 3.1 RxChStatus
The Receive Channel Status object reports the status of the statically configured and dynamically created Receive
Channels within an RCC. When this object is defined on the CM, the value of RccStatusId is always 1.
This object provides the ability for the CM to report its receive channel configuration and is applicable for cases
where a DOCSIS 3.1 CM registers with a DOCSIS 3.0 CCAP.
                                       Table 122 - RxChStatus Object Attributes

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




02/19/25                                            CableLabs®                                                        255
<!-- PDF_PAGE: 256 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


G.2.2.2.3 RccStatusId
This key represents an RCC combination for a particular RcpId either from an RCC configuration object or a CMTS
determined RCC. It is unique per combination of MAC Domain interface index and RcpId. Note that when this
attribute is instantiated at the CM, its value will always be 1.

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
The Receive Channel Status object reports the status of the statically configured and dynamically created Receive
Channels within an RCC.
This object provides the ability for the CM to report its receive channel configuration and is applicable for cases
where a DOCSIS 3.1 CM registers with a DOCSIS 3.1 CCAP.
                                         Table 123 - RxChStatus Object Attributes

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
      Example 1: A Cable Modem configured with OFDM Profiles 3 and 8 would return a query response as follows.
      EnumBits: 0001000010000000 or 0x1080.
      Example 2: A Cable Modem configured with an SC_QAM Channel would return a query response as follows.
      EnumBits: 000000000000000 or 0x0.




256                                                      CableLabs®                                           02/19/25
