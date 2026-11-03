# Annex G.1-G.2.1 - MULPI Requirements Overview and Type Definitions

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 296-299
Classification: Normative (explicit)

<!-- PDF_PAGE: 296 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


Annex G           MAC and Upper Layer Protocols Interface (MULPI)
                  Requirements (Normative)
G.1 Overview
This Annex defines management object extensions for Media Access Control (MAC) information, including
DOCSIS interface configuration, RF Topology, Channel Bonding, QoS, and related extensions.

G.1.1    Cable Modem Service Groups (CM-SGs)
The HFC RF combining and splitting topology between a CMTS and cable modems results in distinct sets of CMs
called Cable Modem Service Groups (CM-SGs) that are served by distinct combinations (i.e., non-overlapping
subsets) of Downstream Channels and Upstream Channels. Because a MAC Domain defines a separate number
space for many DOCSIS protocol elements (e.g., DSIDs, SAIDs, etc.), an operator should define separate MAC
Domains that serve disjoint subsets of CM-SGs rather than a single MAC Domain for all CM-SGs.

G.1.2   Downstream Bonding Group (DBG)
A Downstream Bonding Group (DBG) is a set of Downstream Channels (DCs) on which the CMTS distributes
packets. The CMTS enforces that all Downstream Channels of a DBG are contained within the same MAC Domain
Downstream Service Group (MD-DS-SG). A CMTS permits configuration of a Downstream Channel as a member
of multiple DBGs. A CMTS can restrict the assignment of Downstream Channels to DBGs based on vendor product
implementation. For example, a CMTS product implementation may restrict the set of Downstream Channels that
could be bonded to a given Bonded Channel Set to a subset of the downstream channels in the MAC Domain.

G.1.3   Upstream Bonding Group (UBG)
An Upstream Bonding Group (UBG) is a set of Upstream Channels (UCs) on which upstream data forwarding
service may be provided to a single CM. All Upstream Channels in an Upstream Bonding Group need to be
contained within the same MAC Domain Upstream Service Group (MD-US-SG). A CMTS permits configuration of
an Upstream Channel as a member of multiple UBGs. A CMTS can restricts the assignment of Upstream Channels
to UBGs based on vendor product implementation. For example, a CMTS product implementation could restrict the
set of Upstream Channels that could be bonded to a subset of the downstream channels in the MAC Domain.

G.2 Object Definitions
This section defines the MULPI objects including the associated attributes.

G.2.1    Type Definitions
This section defines data types used in the object definitions for the MULPI information model.
                                        Table 167 - Data Type Definitions

                     Data Type Name             Base Type                      Permitted Values
              AttrAggrRuleMask             HexBinary                SIZE (4)
              AttributeMask                EnumBits                 bonded(0),
                                                                    lowLatency(1),
                                                                    highAvailability(2)
              BitRate                      UnsignedInt              0..4294967295
              ChannelList                  HexBinary                SIZE (0..255)
              ChId                         UnsignedByte             0..255
              ChSetId                      UnsignedInt              0..4294967295
              CpeInterfaceMaskType         See [CCAP-OSSIv4.0]




296                                                CableLabs                                        04/29/26
<!-- PDF_PAGE: 297 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


                     Data Type Name                 Base Type                  Permitted Values
              DataRateUnitType             Enum                     bps(0),
                                                                    kbps(1),
                                                                    mbps(2),
                                                                    gbps(3)
              Dsid                         UnsignedInt              0..1048575
              IfDirection                  Enum                     downstream(1),
                                                                    upstream(2)
              NodeName                     String                   SIZE(0..64)
              OfdmProfiles                 EnumBits                 profile0(0),
                                                                    profile1(1),
                                                                    profile2(2),
                                                                    profile3(3),
                                                                    profile4(4),
                                                                    profile5(5),
                                                                    profile6(6),
                                                                    profile7(7),
                                                                    profile8(8),
                                                                    profile9(9),
                                                                    profile10(10),
                                                                    profile11(11),
                                                                    profile12(12),
                                                                    profile13(13),
                                                                    profile14(14),
                                                                    profile15(15)
              PrimaryDsIndicatorType       Enum                     other(1),
                                                                    primaryDsChannel(2),
                                                                    backupPrimaryDs(3),
                                                                    notSpecified(4)
              RcpId                        HexBinary                SIZE (5)
              ScdmaSelectionString         HexBinary                SIZE (0 | 16)
              SchedulingType               Enum                     undefined(1),
                                                                    bestEffort(2),
                                                                    nonRealTimePollingService(3),
                                                                    realTimePollingService(4),
                                                                    unsolicitedGrantServiceWithAD(5),
                                                                    unsolicitedGrantService(6),
                                                                    proactiveGrantService(7)



           AttrAggrRuleMask
This data type represents a sequence of 32-bit positions that defines logical (e.g., AND, OR) operations to match
against the channel list Provisioned Mask and Service Flow Required Mask bit positions when the CMTS is
determining the service flow for assignment to a bonding group not configured by the management system.
References: [MULPIv4.0] Service Flow Assignment section




04/29/26                                              CableLabs                                                    297
<!-- PDF_PAGE: 298 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


            AttributeMask
This data type consists of a sequence of 32-bit positions used to select the bonding group or the channel to which a
service flow is assigned. DOCSIS defines three types of Attribute Masks for which this type applies: The
Provisioned Attribute Mask that is configured to a Bonding Group or a single-channel, whereas the Required
Attribute and the Forbidden Attribute Mask are part of the Service Flow QoS Parameter Set to be matched with the
Provisioned Attribute Mask of CMTS-configured Bonding Groups or single-channels. DOCSIS reserves the
assignment of the meaning of the first 8-bit positions (left to right) as follows:
      Bit 0: 'bonding'
      Bit 1: 'lowLatency'
      Bit 2: 'highAvailability'
      Bit positions 3–15 are reserved.
      Bit positions 16–31 are freely assigned by operators to represent their own constraints on the channel(s)
      selection for a particular service flow.
References: [MULPIv4.0] Service Flow Assignment section

            BitRate
This data type represents the rate of traffic. The units are specified by a multiplier attribute, DataRateUnitSetting, as
bits per second (bps or bits/s), kilobits per second (kbps or kbit/s), megabits per second (Mbps or Mbit/s), or gigabits
per second (Gbps or Gbit/s).
Kilobits per second is measured as 1,000 bits per second.
Megabits per second is measured as 1 million bits per second.
Gigabits per second is measured as 1 billion bits per second.

            ChannelList
This data type represents a unique set of channel IDs in either the upstream or the downstream direction. Each octet
represents a UCID or DCID depending on the direction of the channels within the list. The CMTS ensures that this
combination of channels is unique per direction within the MAC Domain.
A query to retrieve the value of an attribute of this type, returns the set of channels in the channel list in ascending
order of Channel Ids.

            ChId
This data type is an 8-bit number that represents a provisioned Downstream Channel ID (DCID) or a provisioned
Upstream Channel ID (UCID). A Channel Id is unique per direction within a MAC Domain. The value zero is
reserved for use when the channel ID is unknown.
References: [MULPIv4.0] Upstream Channel Descriptor (UCD) section

            ChSetId
This data type is a CMTS-derived unique number within a MAC Domain used to reference a Channel Set within the
CMTS. Values in the range of 1 to 255 define a single-channel Channel Set and correspond to either the
Downstream Channel ID (DCID) or an Upstream Channel ID (UCID) of that channel. Values greater than 255
indicate a Channel Set consisting of two or more channels in the same direction within the MAC Domain. The value
zero is reserved for use when the Channel Set is unknown.
References: [MULPIv4.0] Channel Bonding section

            CpeInterfaceMaskType
References: [CCAP-OSSIv4.0] CCAP Data Type Definitions section



298                                                   CableLabs                                                04/29/26
<!-- PDF_PAGE: 299 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


           DataRateUnitType
This data type specifies the base unit for traffic rate parameters. The value of this data type allows for their
interpretation in units of bps, kbps, Mbps, or Gbps. The enumeration starts from 0 to match corresponding DOCSIS
protocol TLV values.
Kilobits per second is measured as 1,000 bits per second.
Megabits per second is measured as 1 million bits per second.
Gigabits per second is measured as 1 billion bits per second.

           Dsid
This data type defines the 20-bit Downstream Service Identifier used by the CM for downstream resequencing,
filtering, and forwarding. The value zero is reserved for use when the DSID is unknown or does not apply.
References: [MULPIv4.0] DSID Definition section

           IfDirection
Indicates a direction on an RF MAC interface. The value downstream(1) is from cable modem termination system to
cable modem. The value upstream(2) is from cable modem to cable modem termination system.
Valid enumerations for the data type are:
    •    downstream(1)
    •    upstream(2)
References: [MULPIv4.0] Terms and Definitions section

           NodeName
This data type is a human readable string that represents the name of a fiber node. Internationalization is supported
by conforming to the SNMP textual convention SnmpAdminString. The US-ASCII control characters (0x00 -
0x1F), the DEL character (0x7F), and the double-quote mark (0x22) are prohibited within the syntax of this data
type.
References: [RFC 3411]

           PrimaryDsIndicatorType
This data type enumerates the different type of Primary downstream channels. Possible values are:
    •    primaryDsChannel—when both the CM and CCAP are using DOCSIS 3.1 or DOCSIS 4.0 mode, this value
         indicates that the channel is the primary channel for the CM receiving this RCC. When DOCSIS 3.0 mode
         is in use, this value indicates that the channel is primary-capable; multiple such channels are allowed in this
         mode.
    •    backupPrimaryDs—when both the CM and CCAP are using DOCSIS 3.1 or DOCSIS 4.0 mode, this value
         indicates that the channel is a backup primary channel for the CM receiving this RCC. The priority-ordered
         list of backup primary channels sent to the CM is the same order as the backupPrimaryDs channels are
         configured in RxChCfg. When DOCSIS 3.0 mode is in use, this value indicates that the channel is primary-
         capable; DOCSIS 3.0 does not support the backup primary channel feature.
    •    notSpecified—indicates that this channel has not been specified as a primary-capable channel.
    •    other—indicates a vendor-specific value.
References: [MULPIv4.0] Receive Channel Primary Downstream Channel Indicator section in the Common Radio
Frequency Interface Encodings Annex




04/29/26                                            CableLabs                                                      299
