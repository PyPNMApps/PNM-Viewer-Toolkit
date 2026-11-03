# Annex F.3.2 - CM Downstream Interface Information Model

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 275-284
Classification: Normative (explicit)

<!-- PDF_PAGE: 275 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


TLV 5.82 sent by the FDD CM in the Registration Request. The list of UnsignedShort values represents the list of
supported frequencies specified in MHz and sent in the Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Diplexer Downstream Lower Band Edge
Options List

F.2.3.15.3 DiplexerDsUpperBandEdgeCapability
This attribute specifies the advanced diplexer downstream upper band edge configurations supported by the FDD
CM. This corresponds to the Advanced Diplexer Downstream Upper Band Edge Options List modem capability
TLV 5.83 sent by the FDD CM in the Registration Request. The list of UnsignedShort values represents the list of
supported frequencies specified in MHz and sent in the Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Diplexer Downstream Upper Band Edge
Options List

           CmService
The CmService object is only applicable in the DOCSIS 1.0 CoS mode and is therefore deprecated for DOCSIS 4.0.

           QosProfile
The QosProfile object is only applicable in the DOCSIS 1.0 CoS mode and is therefore deprecated for DOCSIS 4.0.

           (Pre-3.0 DOCSIS) CmStatus
The Pre-3.0 DOCSIS CmStatus object has been reconstituted as CmStatus and CmStatusUs in DOCSIS 3.0 and is
only applicable in the Pre-3.0 DOCSIS versions and is therefore deprecated.

F.3 CM Downstream and Upstream Interfaces Information Models
F.3.1   DS US Common Data Type Definitions
                               Table 146 - CM Downstream Parameter Data Types

                          Data Type Name             Base Type           Permitted Values
                      SubcarrierSpacingType       UnsignedByte        (25 | 50)



           SubcarrierSpacingType
This data type defines the subcarrier spacing for the FFT mode in use. For downstream OFDM channels, if the FFT
mode is 4K mode, then spacing is 50 kHz; if it is 8K mode, then the spacing is 25 kHz. These values of subcarrier
spacing (for downstream OFDM channels) are defined in the Downstream OFDM Parameters table in [PHYv4.0].
For upstream OFDMA channels, if the FFT mode is 2K mode, then the spacing is 50kHz; if the mode is 4K mode,
then the spacing is 25kHz. These values of subcarrier spacing (for upstream OFDMA channels) are defined in the
Upstream OFDMA Parameters table in [PHYv4.0].

F.3.2   CM Downstream Interface Information Model

           Overview
This section defines the configuration and status reporting requirements for CM downstream interfaces. Refer to the
[PHYv4.0] specification for specific details.




04/29/26                                          CableLabs                                                   275
<!-- PDF_PAGE: 276 -->
CM-SP-CM-OSSIv4.0-I13-260429                                Data-Over-Cable Service Interface Specifications




                           Figure 15 - CM Downstream Interface Information Model


          Data Type Definitions
Table 147 and the subsections which follow enumerate the CM downstream interface data types.
                              Table 147 - CM Downstream Parameter Data Types

                    Data Type Name          Base Type         Permitted Values          Reference
                DsOfdmChanIndicatorType    Enum            other(1),
                                                           primary(2),
                                                           backupPrimary(3),
                                                           nonPrimary(4)
                DsOfdmCyclicPrefixType     UnsignedShort   (192 | 256 | 512 | 768 |     [PHYv4.0]
                                                           1024)
                DsOfdmRollOffPeriodType    UnsignedShort   (0 | 64 | 128 | 192 | 256)   [PHYv4.0]
                DsOfdmModulationType       Enum            other(1),                    [PHYv4.0]
                                                           zeroValued(2),
                                                           qpsk(3),
                                                           qam16(4),
                                                           qam64(5),
                                                           qam128(6),
                                                           qam256(7),
                                                           qam512(8),
                                                           qam1024(9),
                                                           qam2048(10),
                                                           qam4096(11),
                                                           qam8192(12),
                                                           qam16384(13)




276                                               CableLabs                                        04/29/26
<!-- PDF_PAGE: 277 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


F.3.2.2.1      DsOfdmChanIndicatorType
This data type is defined to specify the channel indicator type for the downstream channel. The permitted values are
'other', 'primary', 'backupPrimary' and 'nonPrimary'.

F.3.2.2.2      DsOfdmCyclicPrefixType
This data type is defined to specify the five possible values for the length of cyclic prefix. The cyclic prefix (in μs)
are converted into samples using the sample rate of 204.8 Msamples/s and is an integer multiple of: 1/64 * 20 μs.
The possible values come from the Downstream OFDM Parameters table in[PHYv3.1].
References: [PHYv3.1] Downstream OFDMA Parameters table

F.3.2.2.3      DsOfdmRollOffPeriodType
This data type is defined to specify the five possible values for the windowing roll-off period. The Roll-Off Period is
reported in 'number of samples'. The possible values come from the Downstream OFDM Parameters table
in[PHYv3.1].
References: [PHYv3.1] Downstream OFDMA Parameters table

F.3.2.2.4      DsOfdmModulationType
This data type is defined to specify the modulation types supported by the CM demodulator.
References: [PHYv3.1] Cable Modem Receiver Input Requirements section

             CM Downstream Interface Object Definitions

F.3.2.3.1      ScQamDownstreamChannel
The ScQamDownstreamChannel object reports the channel configuration of an SC-QAM downstream channel.
References: [RFC 4546], docsIfDownstreamChannelTable
                               Table 148 - ScQamDownstreamChannel Object Attributes

        Attribute
         Name                  Type           Access          Type Constraints             Units      Default Value
     IfIndex            InterfaceIndex      Key
     ChannelId          ChId                R/O            0..255
     Freq               UnsignedInt         R/O            111000000..1002000000      Hz
     Width              UnsignedInt         R/O            6000000 | 8000000          Hz
     Modulation         DsModulationType    R/O            unknown(1),
                                                           other(2),
                                                           qam64(3),
                                                           qam256(4)
     Interleave         Enum                R/O            unknown(1),
                                                           other(2),
                                                           taps8Increment16(3),
                                                           taps16Increment8(4),
                                                           taps32Increment4(5),
                                                           taps64Increment2(6),
                                                           taps128Increment1(7),
                                                           taps12increment17(8)
     Power              TenthdBmV           R/O                                       dBmV




04/29/26                                             CableLabs                                                       277
<!-- PDF_PAGE: 278 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


          Attribute
           Name                Type              Access       Type Constraints           Units        Default Value
        Annex           Enum                R/O            unknown(1),
                                                           other(2),
                                                           annexA(3),
                                                           annexB(4),
                                                           annexC(5)


                           Table 149 - ScQamDownstreamChannel Object Associations

                                                      Near-End             Far-End
      Associated Object Name           Type           Multiplicity        Multiplicity               Label
   If                              Association      0..*              1                  If.Type = SC-QAM Downstream
                                                                                         Interface



F.3.2.3.1.1 IfIndex
This key attribute is the interface index of the SC-QAM downstream interface.

F.3.2.3.1.2 ChannelId
This attribute is the Downstream Channel Identifier. This is an 8-bit identifier that distinguishes a Downstream
Channel within a MAC Domain.
The cable modem termination system identification of the downstream channel within this particular MAC interface.
If the interface is down, the object returns the most current value. If the downstream channel ID is unknown, this
object returns a value of 0.
References: [RFC 4546] docsIfDownChannelId

F.3.2.3.1.3 Freq
This attribute is the center of the downstream frequency associated with this channel. This object will return the
current tuner frequency.
References: [RFC 4546] docsIfDownChannelFrequency

F.3.2.3.1.4 Width
This attribute is bandwidth of this downstream channel.
References: [RFC 4546] docsIfDownChannelWidth

F.3.2.3.1.5 Modulation
This attribute is the modulation of the channel.
References: [RFC 4546] docsIfDownChannelModulation

F.3.2.3.1.6 Interleave
This attribute is the Forward Error Correction (FEC) interleaving used for this downstream channel.
References: [RFC 4546] docsIfDownChannelInterleave

F.3.2.3.1.7 Power
This attribute is the received power level. If the interface is down, this object either returns the most recent value or
the value of 0.
References: [RFC 4546] docsIfDownChannelPower


278                                                   CableLabs                                                04/29/26
<!-- PDF_PAGE: 279 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


F.3.2.3.1.8 Annex
This attribute returns the Annex used by this channel. This value indicates the conformance of the implementation to
important regional cable standards.
    •      annexA: Annex A from ITU-T J.83 is used. (equivalent to EN 300 429)
    •      annexB: Annex B from ITU-T J.83 is used.
    •      annexC: Annex C from ITU-T J.83 is used.
References: [RFC 4546] docsIfDownChannelAnnex

F.3.2.3.2       DsOfdmChannel
The DsOfdmChannel object reports the configuration and statistics of an OFDM downstream channel.
The OFDM downstream channel boundary frequency limits depend on the mode of operation to which DOCSIS
equipment is configured to operate. Refer to [PHYv3.1] Downstream CM Spectrum section, for the OFDM
downstream channel frequency limits when equipment is configured to be compliant with the DOCSIS 3.1 and (non-
FDX and non-FDD extended spectrum) DOCSIS 4.0 frequency plans. Refer to [PHYv4.0] Downstream FDX CM
Spectrum section for the OFDM downstream channel frequency limits when equipment is configured to be
compliant with FDX mode. Refer to [PHYv4.0] Upstream and Downstream Frequency Plan for FDD Operation
section for the OFDM downstream channel frequency limits when the equipment is configured to be compliant with
FDD mode.
An FDX CM instantiates this object for each FDX Downstream Channel, in addition to the non-FDX downstream
channels.
An FDD CM instantiates this object for each FDD Downstream Channel, in addition to the non-FDD downstream
channels.
References: [PHYv3.1] Downstream CM Spectrum section; [PHYv4.0] Downstream FDX CM Spectrum section
and Upstream and Downstream Frequency Plan for FDD Operation section
                                    Table 150 - DsOfdmChannel Object Attributes

        Attribute Name                   Type             Access    Type Constraints       Units     Default Value
 IfIndex                     InterfaceIndex              Key       Interface Index of
                                                                   downstream interface
 ChannelId                   ChId                        R/O
 ChannelIndicator            DsOfdmChanIndicatorType     R/O
 SubcarrierZeroFreq          UnsignedInt                 R/O                              Hz
 FirstActiveSubcarrierNum    UnsignedInt                 R/O       148..7895
 LastActiveSubcarrierNum     UnsignedInt                 R/O
 NumActiveSubcarriers        UnsignedInt                 R/O       1..7600
 SubcarrierSpacing           SubcarrierSpacingType       R/O                              kHz
 CyclicPrefix                DsOfdmCyclicPrefixType      R/O                              samples
 RollOffPeriod               DsOfdmRollOffPeriodType     R/O                              samples
 PlcFreq                     UnsignedInt                 R/O                              Hz
 NumPilots                   UnsignedInt                 R/O
 TimeInterleaverDepth        UnsignedByte                R/O       1..32                  symbols
 PlcTotalCodewords           Counter64                   R/O
 PlcUnreliableCodewords      Counter64                   R/O
 NcpTotalFields              Counter64                   R/O
 NcpFieldCrcFailures         Counter64                   R/O
 FdxEnabled                  Boolean                     R/O




04/29/26                                              CableLabs                                                279
<!-- PDF_PAGE: 280 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


                                 Table 151 - DsOfdmChannel Object Associations

                                                           Near-End         Far-End
   Associated Object Name                 Type             Multiplicity    Multiplicity             Label
  If                             Association               0..*            1              If.Type = OFDM Downstream
                                                                                          Interface
  DsOfdmProfileStats             Directed Composition to   1               1..5
                                 DsOfdmProfileStats



F.3.2.3.2.1 IfIndex
This key attribute is the unique index of the OFDM downstream channel.

F.3.2.3.2.2 ChannelId
This attribute is the OFDM downstream channel identifier. This is an 8-bit identifier that uniquely identifies an
OFDM downstream channel within a MAC domain.
If the interface is down, the attribute returns the most current value. If the OFDM downstream channel ID is
unknown, this attribute returns a value of 0.

F.3.2.3.2.3 ChannelIndicator
This attribute is used to identify the OFDM downstream channel as a primary downstream channel, backup primary
downstream channel, or non-primary downstream channel. A value of 'primary(2)' indicates that the OFDM
downstream channel is assigned to be the CM's primary downstream channel. A value of 'backupPrimary(3)'
indicates that the OFDM downstream channel is assigned to be the CM's backup primary downstream channel. A
value of 'nonPrimary(4)' indicates the OFDM downstream channel is not assigned to be a CM's primary downstream
channel or backup primary downstream channel.

F.3.2.3.2.4 SubcarrierZeroFreq
This attribute specifies the center frequency of subcarrier 0 of the OFDM downstream channel. This is the frequency
of subcarrier X(0) in the definition of the Discrete Fourier Transform.

F.3.2.3.2.5 FirstActiveSubcarrierNum
This attribute corresponds to the number of the first non-excluded subcarrier.

F.3.2.3.2.6 LastActiveSubcarrierNum
This attribute corresponds to the number of the last non-excluded subcarrier.

F.3.2.3.2.7 NumActiveSubcarriers
This attribute represents the number of active data subcarriers within the OFDM downstream channel (i.e., this
exclude subcarriers for continuous pilots and the PLC). For 4K FFT mode, the maximum number of subcarriers
including continuous pilots and the PLC cannot exceed 3800, and for 8K FFT mode, the maximum number of active
subcarriers including continuous pilots and the PLC cannot be greater than 7600. However, there are a minimum of
56 continuous pilots in a 192 MHz channel that has no exclusions, and the size of the PLC is 8 subcarriers for 4K
FFT mode and 16 subcarriers for 8K FFT mode. Therefore, the maximum value of NumActiveSubcarriers is 3736
(or 3800 - 56 - 8) for 4K FFT mode and 7528 (or 7600 - 56 - 16) for 8K FFT mode.

F.3.2.3.2.8 SubcarrierSpacing
This attribute defines the subcarrier spacing associated with a particular FFT mode configured on the OFDM
downstream channel. If it is 4K mode, then the subcarrier spacing is 50kHz. If it is 8K mode, then the subcarrier
spacing is 25kHz.




280                                                 CableLabs                                               04/29/26
<!-- PDF_PAGE: 281 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


F.3.2.3.2.9 CyclicPrefix
Cyclic prefix enables the receiver to overcome the effects of inter-symbol-interference and intercarrier-interference
caused by micro-reflections in the OFDM downstream channel. There are five possible values for the length of the
cyclic prefix and the choice depends on the delay spread of the channel - a longer delay spread requires a longer
cyclic prefix.
References: [PHYv4.0] Cyclic Prefix Values

F.3.2.3.2.10 RollOffPeriod
Roll off period maximizes channel capacity by sharpening the edges of the spectrum of the OFDM signal. For
windowing purposes another segment at the start of the IDFT output is appended to the end of the IDFT output -the
roll-off postfix (RP). There are five possible values for the (RP), and the choice depends on the bandwidth of the
OFDM downstream channel and the number of exclusion bands within the channel. A larger RP provides sharper
edges in the spectrum of the OFDM signal; however, there is a time vs. frequency trade-off. Larger RP values
reduce the efficiency of transmission in the time domain, but because the spectral edges are sharper, more useful
subcarriers appear in the frequency domain. There is an optimum value for the RP that maximizes capacity for a
given bandwidth and/or exclusion band scenario.
References: [PHYv4.0] Roll-off Period Values

F.3.2.3.2.11 PlcFreq
This attribute defines the location of the PHY Link Channel (PLC). It is always 6 MHz, aligned on a 1 MHz grid
such that the lowest subcarrier center frequency is an integer number on the encompassed spectrum containing the
PLC at its center. The PLC will be in the same band in every OFDM symbol for that downstream channel.
For 50 kHz OFDM subcarrier spacing, the PLC is 8 subcarriers. For 25 kHz OFDM subcarrier spacing, the PLC is
16 subcarriers.
References: [PHYv4.0] Physical Layer Link Channel section

F.3.2.3.2.12 NumPilots
This attribute indicates the number of continuous pilots configured for the OFDM downstream channel.

F.3.2.3.2.13 TimeInterleaverDepth
The attribute defines the depth of time interleaving used for this OFDM downstream channel as received in the OCD
message. The CM supports a maximum value of 32 symbols for 20 μs symbol duration (50 kHz subcarrier spacing)
and 16 symbols for 40 μs symbol duration (25 kHz subcarrier spacing).
References: [PHYv4.0] Interleaving and De-Interleaving section

F.3.2.3.2.14 PlcTotalCodewords
This attribute represents the total number of PLC codewords received by the CM.

F.3.2.3.2.15 PlcUnreliableCodewords
This attribute represents the total number of PLC codewords which failed post-decoding LDPC syndrome check.

F.3.2.3.2.16 NcpTotalFields
This attribute represents the total number of NCP fields received by the CM.

F.3.2.3.2.17 NcpFieldCrcFailures
This attribute represents the total number of NCP fields received by the CM which failed the CRC check.




04/29/26                                            CableLabs                                                    281
<!-- PDF_PAGE: 282 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


F.3.2.3.2.18 FdxEnabled
This attribute reports if the CM's OFDM downstream channel is enabled for FDX. This is further indicated by the
channel's occupied spectrum being configured from the FDX Allocated Spectrum. A value of 'true' indicates the
OFDM downstream channel is an FDX Downstream Channel. A value of 'false' indicates the OFDM downstream
channel is not an FDX Downstream Channel.

F.3.2.3.3         DsOfdmProfileStats
The DsOfdmProfileStats object provides usage statistics for a modulation profile assigned to an OFDM downstream
channel. An instance is created when a profile is assigned to an instance of a DsOfdmChannel object. An instance is
deleted when a profile id becomes unassigned. The counters in this object are statistics of the data profiles that are
intended for this CM.
                                    Table 152 - DsOfdmProfileStats Object Attributes

                                                                        Type
                  Attribute Name               Type       Access      Constraints        Units     Default Value
      ProfileId                           UnsignedByte    Key       1..13 | 255
      ConfigChangeCt                      UnsignedByte    R/O
      TotalCodewords                      Counter64       R/O
      CorrectedCodewords                  Counter64       R/O
      UncorrectableCodewords              Counter64       R/O
      ProfileInOctets                     Counter64       R/O
      ProfileInUnicastOctets              Counter64       R/O
      ProfileInMulticastOctets            Counter64       R/O
      ProfileInFrames                     Counter64       R/O
      ProfileInUnicastFrames              Counter64       R/O
      ProfileInMulticastFrames            Counter64       R/O
      ProfileInFrameCrcFailures           Counter64       R/O
      ProfileCounterDiscontinuityTime     TimeStamp       R/O



F.3.2.3.3.1 ProfileId
This key attribute is the unique identifier of the downstream profile associated with the OFDM downstream channel.
The Profile ID for Next Codeword Pointer (NCP) Profiles is 255 [MULPIv4.0].

F.3.2.3.3.2 ConfigChangeCt
This attribute contains the value of the Configuration Change Count field in the Downstream Profile Descriptor
(DPD) MAC Management Message corresponding to this profile.

F.3.2.3.3.3 TotalCodewords
This attribute defines the total number of codewords (including full-length and shortened) measured on this profile.

F.3.2.3.3.4 CorrectedCodewords
This attribute defines the number of codewords measured on this profile that failed pre-decoding LDPC syndrome
check and passed BCH decoding.

F.3.2.3.3.5 UncorrectableCodewords
This attribute defines the number of codewords measured on this profile that failed BCH decoding for data profile
and post-decoding LDPC syndrome check for NCP profile.




282                                                   CableLabs                                            04/29/26
<!-- PDF_PAGE: 283 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


F.3.2.3.3.6 ProfileInOctets
This attribute is the count of MAC-layer octets received by the CM on this profile. This value is the size of all
unicast, multicast or broadcast frames (including all MAC-layer framing) delivered from the PHY layer to the MAC
layer - this includes user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.7 ProfileInUnicastOctets
This attribute is the count of MAC-layer unicast octets received by the CM on this profile. This value is the size of
all unicast frames (including all MAC-layer framing) delivered from the PHY layer to the MAC layer - this includes
user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.8 ProfileInMulticastOctets
This attribute is the count of MAC-layer multicast and broadcast octets received by the CM on this profile. This
value is the size of all frames (including all MAC-layer framing) delivered from the PHY layer to the MAC layer
and addressed to a multicast MAC address - this includes user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.9 ProfileInFrames
This attribute is the count of frames received by the CM on this profile. This value is the count of all unicast,
multicast or broadcast frames delivered from the PHY layer to the MAC layer - this includes user data, DOCSIS
MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.10 ProfileInUnicastFrames
This attribute is the count of unicast frames received by the CM on this profile. This value is the count of all frames
delivered from the PHY layer to the MAC layer and addressed to a unicast MAC address - this includes user data,
DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.11 ProfileInMulticastFrames
This attribute is the count of multicast frames received by the CM on this profile. This value is the count of all
frames delivered from the PHY layer to the MAC layer and addressed to a multicast MAC address - this includes
user data, DOCSIS MAC Management Messages, etc., but excludes frames sent to a broadcast address.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.




04/29/26                                            CableLabs                                                      283
<!-- PDF_PAGE: 284 -->
CM-SP-CM-OSSIv4.0-I13-260429                                        Data-Over-Cable Service Interface Specifications


The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.12 ProfileInFrameCrcFailures
This attribute is the count of frames received by the CM on this profile that failed the MAC frame CRC check.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.3.13 ProfileCounterDiscontinuityTime
This attribute is the value of sysUpTime on the most recent occasion at which any one or more of this entry's
counters suffered a discontinuity (e.g., counter rollover, other vendor-specific event). If no such discontinuities have
occurred since the last re-initialization of the local management subsystem, then this attribute contains a zero value.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.4       DsOfdmChannelPower
The DsOfdmChannelPower object reports the measured channel power for a 6 MHz wide band at the F connector
input of the CM.
An FDX CM instantiates this object for each FDX Downstream Channel, in addition to the non-FDX downstream
channels.
An FDD CM instantiates this object for each FDD Downstream Channel, in addition to the non-FDD upstream
channels.
                               Table 153 - DsOfdmChannelPower Object Attributes

        Attribute Name            Type               Access          Type Constraints          Units      Default Value
      IfIndex               InterfaceIndex          Key           Interface Index of
                                                                  downstream interface
      BandIndex             UnsignedByte            Key           0..33
      CenterFrequency       UnsignedLong            R/O           111000000.. 1791000000     Hz
      ReceivedPower         TenthdBmV               R/O                                      dBmV


                             Table 154 - DsOfdmChannelPower Object Associations

                                                            Near-End        Far-End
         Associated Object Name              Type           Multiplicity   Multiplicity                Label
        If                           Association           0..*            1               If.Type = OFDM Downstream
                                                                                           Interface



F.3.2.3.4.1 IfIndex
This key attribute is the interface index of an OFDM downstream interface.

F.3.2.3.4.2 BandIndex
This key attribute is a unique index used by the CM to identify each of the 6 MHz bands of a given OFDM
downstream channel (from the lowest 6 MHz band of the Modulated Spectrum to the highest 6 MHz band of the
Modulated Spectrum). The CM MUST assign indices in frequency order from the OFDM channel's lowest to
highest 6 MHz frequency band for each of the 6 MHz bands of the channel, using an index of 1 to represent the
lowest frequency band of the Modulated Spectrum. Thus, an index of 33 represents the highest possible 6 MHz
frequency band of the Modulated Spectrum of an OFDM downstream channel. If there are interior Exclusion Bands
resulting in 6 MHz bands which contain no Active Subcarriers, then the indices corresponding to those bands will be
skipped and the power for those bands will not be reported. The CM MUST also provide the power of the PLC
channel and utilize an index value of 0 to represent the PLC channel in this object. If the placement of the OFDM
downstream channel is such that the center frequency of the lowest or highest active subcarrier is placed on a 1 MHz



284                                                       CableLabs                                             04/29/26
