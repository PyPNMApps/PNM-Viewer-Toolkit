# Annex F.3.2 - CM Downstream Interface Information Model

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 230-238
Classification: Normative (explicit)

<!-- PDF_PAGE: 230 -->
CM-SP-CM-OSSIv3.1-I27-250219                                  Data-Over-Cable Service Interface Specifications


F.2.3.11.11 LgVqEwmaAlpha
This attribute represents the log base 2 of the reciprocal of the exponentially weighted moving average weight for
the Low Latency Virtual Queue (VQ).
This attribute is configurable only through the CM configuration file via TLV-11. This attribute is read-only during
the CM operational state.
References: [MULPIv3.1] LG_VQ_EWMA_ALPHA parameter defined in pseudocode in Annex O.1 AQM Utility
Functions

F.2.3.11.12 VqInterval
This attribute represents the interval between alignments of the Virtual Queue (VQ) with the actual queue.
This attribute is configurable only through the CM configuration file via TLV-11. This attribute is read-only during
the CM operational state.
References: [MULPIv3.1] VQ_INTERVAL parameter defined in pseudocode in Annex O.1 AQM Utility Functions

          CmService
This object is only applicable in the DOCSIS 1.0 CoS mode and is therefore deprecated for DOCSIS 3.1.

          QosProfile
This object is only applicable in the DOCSIS 1.0 CoS mode and is therefore deprecated for DOCSIS 3.1.

          (Pre-3.0 DOCSIS) CmStatus
This object has been reconstituted as CmStatus and CmStatusUs in DOCSIS 3.0 and is only applicable in the Pre-3.0
DOCSIS versions and is therefore deprecated.

F.3 CM Downstream and Upstream Interfaces Information Models
F.3.1    DS US Common Data Type Definitions
                               Table 102 - CM Downstream Parameter Data Types

                            Data Type Name            Base Type         Permitted Values
                        SubcarrierSpacingType        UnsignedByte   (25 | 50)



          SubcarrierSpacingType
This data type defines the subcarrier spacing for the FFT mode in use. For downstream OFDM channels, if the FFT
mode is 4K mode, then spacing is 50 kHz; if it is 8K mode, then the spacing is 25 kHz. These values of subcarrier
spacing (for downstream OFDM channels) are defined in the Downstream OFDM Parameters table in [PHYv3.1].
For upstream OFDMA channels, if the FFT mode is 2K mode, then the spacing is 50kHz; if the mode is 4K mode,
then the spacing is 25kHz. These values of subcarrier spacing (for upstream OFDMA channels) are defined in the
Upstream OFDMA Parameters table in [PHYv3.1].

F.3.2    CM Downstream Interface Information Model

          Overview
This section defines the configuration and status reporting requirements for the CM Downstream Interfaces. This
information is contained in the [PHYv3.1] specification.




230                                                CableLabs®                                                02/19/25
<!-- PDF_PAGE: 231 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219




                                Figure 14 - CM Downstream Information Model


           Data Type Definitions
Table 103 and the subsections which follow enumerate the CM downstream interface data types.
                              Table 103 - CM Downstream Parameter Data Types

                    Data Type Name          Base Type         Permitted Values          Reference
                DsOfdmChanIndicatorType    UnsignedByte
                DsOfdmCyclicPrefixType     UnsignedShort   (192 | 256 | 512 | 768 |     [PHYv3.1]
                                                           1024)
                DsOfdmRollOffPeriodType    UnsignedShort   (0 | 64 | 128 | 192 | 256)   [PHYv3.1]
                DsOfdmModulationType       Enum            other(1)                     [PHYv3.1]
                                                           zeroValued(2)
                                                           qpsk(3)
                                                           qam16(4)
                                                           qam64(5)
                                                           qam128(6)
                                                           qam256(7)
                                                           qam512(8)
                                                           qam1024(9)
                                                           qam2048(10)
                                                           qam4096(11)
                                                           qam8192 (12)
                                                           qam16384 (13)




02/19/25                                          CableLabs®                                             231
<!-- PDF_PAGE: 232 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


F.3.2.2.1       DsOfdmChanIndicatorType
This data type is defined to specify the channel indicator type for the downstream channel. The permitted values are
'other', 'primary', 'backupPrimary' and 'nonPrimary'.

F.3.2.2.2       DsOfdmCyclicPrefixType
This data type is defined to specify the five possible values for the length of cyclic prefix. The cyclic prefix (in μs)
are converted into samples using the sample rate of 204.8 Msamples/s and is an integer multiple of: 1/64 * 20 μs.
The possible values come from the Downstream OFDM Parameters table in [PHYv3.1].

F.3.2.2.3       DsOfdmRollOffPeriodType
This data type is defined to specify the five possible values for the windowing roll-off period. The Roll-Off Period is
reported in 'number of samples'. The possible values come from the Downstream OFDM Parameters table in
[PHYv3.1].

F.3.2.2.4       DsOfdmModulationType
This data type is defined to specify the modulation types supported by the CM demodulator. The values are defined
in the Modulation Formats section of [PHYv3.1].

             CM Downstream Interface Status Object Definitions

F.3.2.3.1       ScQamDownstreamChannel
This object reports the channel configuration of an SC-QAM downstream channel.
References: [RFC 4546] docsIfDownstreamChannelTable
                             Table 104 - ScQamDownstreamChannel Object Attributes

          Attribute           Type           Access         Type Constraints             Units      Default Value
           Name
      IfIndex            InterfaceIndex   Key
      ChannelId          ChId             R/O             0..255
      Freq               UnsignedInt      R/O             111000000..1002000000     Hz
      Width              UnsignedInt      R/O             6000000 | 8000000         Hz
      Modulation         Enum             R/O             unknown(1)
                                                          other(2)
                                                          qam64(3)
                                                          qam256(4)
      Interleave         Enum             R/O             unknown(1)
                                                          other(2)
                                                          taps8Increment16(3)
                                                          taps16Increment8(4)
                                                          taps32Increment4(5)
                                                          taps64Increment2(6)
                                                          taps128Increment1(7)
                                                          taps12increment17(8)
      Power              TenthdBmV        R/O                                       dBmV
      Annex              Enum             R/O             unknown(1)
                                                          other(2)
                                                          annexA(3)
                                                          annexB(4)
                                                          annexC(5)




232                                                   CableLabs®                                                02/19/25
<!-- PDF_PAGE: 233 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv3.1-I27-250219


                           Table 105 - ScQamDownstreamChannel Object Associations

          Associated Object         Type            Near-end           Far-end                     Label
               Name                                Multiplicity       Multiplicity
     If                         Association    1                  *                     if.Type=SC-QAM Downstream
                                                                                        Interface



F.3.2.3.1.1 IfIndex
This attribute is the interface index of the downstream interface and is a key to provide an index into the table.

F.3.2.3.1.2 ChannelId
This attribute is the Downstream Channel Identifier. This is an 8-bit identifier that distinguishes a Downstream
Channel within a MAC Domain.
The Cable Modem Termination System identification of the downstream channel within this particular MAC
interface. If the interface is down, the object returns the most current value. If the downstream channel ID is
unknown, this object returns a value of 0.
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

F.3.2.3.1.8 Annex
This attribute returns the annex used by this channel. This value indicates the conformance of the implementation to
important regional cable standards.
    •      annexA: Annex A from ITU-T J.83 is used. (equivalent to EN 300 429)
    •      annexB: Annex B from ITU-T J.83 is used.




02/19/25                                               CableLabs®                                                     233
<!-- PDF_PAGE: 234 -->
CM-SP-CM-OSSIv3.1-I27-250219                                          Data-Over-Cable Service Interface Specifications


       •   annexC: Annex C from ITU-T J.83 is used.
References: [RFC 4546] docsIfDownChannelAnnex

F.3.2.3.2       DsOfdmChannel
This object reports the configuration and statistics for a downstream OFDM channel.
                                       Table 106 - DsOfdmChannel Object Attributes

       Attribute Name                      Type                Access        Type Constraints          Units    Default Value
 IfIndex                       InterfaceIndex                 Key          1..255
 ChannelId                     ChId                           R/O
 ChannelIndicator              DsOfdmChanIndicatorType        R/O
 SubcarrierZeroFreq            UnsignedInt                    R/O          108000000..1770000000     Hz
 FirstActiveSubcarrierNum      UnsignedInt                    R/O          148..7895
 LastActiveSubcarrierNum       UnsignedInt                    R/O          148..7895
 NumActiveSubcarriers          UnsignedInt                    R/O          1...7600
 SubcarrierSpacing             SubcarrierSpacingType          R/O                                    kHz
 CyclicPrefix                  DsOfdmCyclicPrefixType         R/O                                    samples
 RollOffPeriod                 DsOfdmRollOffPeriodType        R/O                                    samples
 PlcFreq                       UnsignedInt                    R/O          108000000..               Hz
                                                                           1788000000
 NumPilots                     UnsignedInt                    R/O
 TimeInterleaverDepth          UnsignedByte                   R/O          1...32                    symbols
 PlcTotalCodewords             Counter64                      R/O
 PlcUnreliableCodewords        Counter64                      R/O
 NcpTotalFields                Counter64                      R/O
 NcpFieldCrcFailures           Counter64                      R/O


                                      Table 107 - DsOfdmChannel Object Associations

        Associated                    Type                   Near-end             Far-end                  Label
       Object Name                                          Multiplicity         Multiplicity
  If                        Association                 1                    *                  if.Type=OFDM Downstream
                                                                                                Interface
  DsOfdmProfileStats        Directed Composition                             1...5



F.3.2.3.2.1 IfIndex
This attribute is the interface index of the downstream interface and is a key to provide an index into the table.

F.3.2.3.2.2 ChannelId
The CMTS identification of the downstream channel within this particular MAC interface. If the downstream
channel Id is unknown, then this attribute returns a value of 0.

F.3.2.3.2.3 ChannelIndicator
This attribute is used to identify the OFDM downstream channel as primary, backup primary or non-primary. A
value of 1 indicates that OFDM channel is assigned to be the CM's primary downstream channel. A value greater
than 1 indicates that the OFDM channel is assigned to be the CM's backup primary downstream channel. A value of
0 indicates the OFDM channel is not assigned to be a CM's primary or backup primary downstream channel.




234                                                         CableLabs®                                              02/19/25
<!-- PDF_PAGE: 235 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


F.3.2.3.2.4 SubcarrierZeroFreq
This attribute specifies the center frequency of subcarrier 0 of the OFDM channel. This is the frequency of
subcarrier X(0) in the definition of the Discrete Fourier Transform.

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
FFT mode and 16 subcarriers for 8K FFT mode. Therefore the maximum value of NumActiveSubcarriers is 3736
(or 3800 - 56 - 8) for 4K FFT mode and 7528 (or 7600 - 56 - 16) for 8K FFT mode.

F.3.2.3.2.8 SubcarrierSpacing
This attribute defines the subcarrier spacing associated with a particular FFT mode configured on the OFDM
downstream channel. If it is 4K mode, then the subcarrier spacing is 50kHz. If it is 8K mode, then the subcarrier
spacing is 25kHz.

F.3.2.3.2.9 CyclicPrefix
Cyclic prefix enables the receiver to overcome the effects of inter-symbol-interference and intercarrier-interference
caused by micro-reflections in the channel. There are five possible values for the length of the CP and the choice
depends on the delay spread of the channel - a longer delay spread requires a longer cyclic prefix.
References: [PHYv3.1] Cyclic Prefix Values section

F.3.2.3.2.10 RollOffPeriod
Roll off period maximizes channel capacity by sharpening the edges of the spectrum of the OFDM signal. For
windowing purposes another segment at the start of the IDFT output is appended to the end of the IDFT output -the
roll-off postfix (RP). There are five possible values for the (RP), and the choice depends on the bandwidth of the
channel and the number of exclusion bands within the channel. A larger RP provides sharper edges in the spectrum
of the OFDM signal; however, there is a time vs. frequency trade-off. Larger RP values reduce the efficiency of
transmission in the time domain, but because the spectral edges are sharper, more useful subcarriers appear in the
frequency domain. There is an optimum value for the RP that maximizes capacity for a given bandwidth and/or
exclusion band scenario.
References: [PHYv3.1] Roll-off Period Values section

F.3.2.3.2.11 PlcFreq
This attribute defines the location of the PHY Link Channel (PLC). It is the center frequency of the lowest
subcarrier of the 6 MHz encompassed spectrum containing the PLC at its center.

F.3.2.3.2.12 NumPilots
This attribute indicates the number of continuous pilots configured for the downstream channel.




02/19/25                                            CableLabs®                                                      235
<!-- PDF_PAGE: 236 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


F.3.2.3.2.13 TimeInterleaverDepth
The attribute defines the depth of Time interleaving used for this downstream channel as received in the OCD
message.

F.3.2.3.2.14 PlcTotalCodewords
This attribute represents the total number of PLC codewords received by the CM.

F.3.2.3.2.15 PlcUnreliableCodewords
This attribute represents the total number of PLC codewords which failed post-decoding LDPC syndrome check.

F.3.2.3.2.16 NcpTotalFields
This attribute represents the total number of NCP fields received by the CM.

F.3.2.3.2.17 NcpFieldCrcFailures
This attribute represents the total number of NCP fields received by the CM which failed the CRC check.

F.3.2.3.2.18 DsOfdmProfileStats
This CM object provides usage statistics for a modulation profile assigned to an OFDM downstream channel. A row
entry is created when a profile is assigned. The row entry is deleted when a profile id becomes unassigned. The
counts in this table are only of data on data profiles that is intended for this CM.
                                    Table 108 - DsOfdmProfileStats Object Attributes

                  Attribute Name               Type       Access        Type           Units     Default Value
                                                                      Constraints
      ProfileId                           UnsignedByte    R/O       0..15|255
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



F.3.2.3.2.19 ProfileId
This attribute is the unique identifier of the downstream profile associated with the OFDM downstream channel. It is
a key defined to provide an index into the table. The Profile ID for Next Codeword Pointer (NCP) Profiles is 255
[MULPIv3.1].

F.3.2.3.2.20 ConfigChangeCt
This attribute contains the value of the Configuration Change Count field in the Downstream Profile Descriptor
(DPD) MAC Management Message corresponding to this profile.




236                                                   CableLabs®                                          02/19/25
<!-- PDF_PAGE: 237 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


F.3.2.3.2.21 TotalCodewords
This attribute defines the total number of codewords (including full-length and shortened) measured on this profile.

F.3.2.3.2.22 CorrectedCodewords
This attribute defines the number of codewords measured on this profile that failed pre-decoding LDPC syndrome
check and passed BCH decoding.

F.3.2.3.2.23 UncorrectableCodewords
This attribute defines the number of codewords measured on this profile that failed BCH decoding for data profile
and post-decoding LDPC syndrome check for NCP profile.

F.3.2.3.2.24 ProfileInOctets
This attribute is the count of MAC-layer octets received by the CM on this profile. This value is the size of all
unicast, multicast or broadcast frames (including all MAC-layer framing) delivered from the PHY layer to the MAC
layer - this includes user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.25 ProfileInUnicastOctets
This attribute is the count of MAC-layer unicast octets received by the CM on this profile. This value is the size of
all unicast frames (including all MAC-layer framing) delivered from the PHY layer to the MAC layer - this includes
user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.26 ProfileInMulticastOctets
This attribute is the count of MAC-layer multicast and broadcast octets received by the CM on this profile. This
value is the size of all frames (including all MAC-layer framing) delivered from the PHY layer to the MAC layer
and addressed to a multicast MAC address - this includes user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.27 ProfileInFrames
This attribute is the count of frames received by the CM on this profile. This value is the count of all unicast,
multicast or broadcast frames delivered from the PHY layer to the MAC layer - this includes user data, DOCSIS
MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.28 ProfileInUnicastFrames
This attribute is the count of unicast frames received by the CM on this profile. This value is the count of all frames
delivered from the PHY layer to the MAC layer and addressed to a unicast MAC address - this includes user data,
DOCSIS MAC Management Messages, etc.



02/19/25                                            CableLabs®                                                      237
<!-- PDF_PAGE: 238 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.29 ProfileInMulticastFrames
This attribute is the count of multicast frames received by the CM on this profile. This value is the count of all
frames delivered from the PHY layer to the MAC layer and addressed to a multicast MAC address - this includes
user data, DOCSIS MAC Management Messages, etc., but excludes frames sent to a broadcast address.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.30 ProfileInFrameCrcFailures
This attribute is the count of frames received by the CM on this profile that failed the MAC frame CRC check. The
CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.31 ProfileCounterDiscontinuityTime
This attribute is the value of sysUpTime on the most recent occasion at which any one or more of this entry's
counters suffered a discontinuity (e.g., counter rollover, other vendor-specific event). If no such discontinuities have
occurred since the last re-initialization of the local management subsystem, then this attribute contains a zero value.
The CM reports zero for this attribute for Next Codeword Pointer profile.

F.3.2.3.2.32 DsOfdmChannelPower
This object provides the attributes to measure the channel power for a 6 MHz wide band at the F connector input of
the CM.
                                 Table 109 - DsOfdmChannelPower Object Attributes

       Attribute Name             Type             Access          Type Constraints       Units      Default Value
      IfIndex              InterfaceIndex      Key
      BandIndex            UnsignedByte        Key             0..33
      CenterFrequency      UnsignedLong        R/O             111000000..1215000000    Hz          0
      ReceivedPower        TenthdBmV           R/O                                      dBmV


                                Table 110 - DsOfdmChannelPower Object Associations

                  Associated        Type         Near-end        Far-end                  Label
                  Object Name                   Multiplicity    Multiplicity
                If               Association   1               1               if.Type=OFDM Downstream
                                                                               Interface



F.3.2.3.2.33 IfIndex
This attribute is the interface index of an OFDM downstream interface in the current CM receive channel set and is
a key to provide an index into the table.

F.3.2.3.2.34 BandIndex
This attribute is a unique index used by the CM to identify each of the 6 MHz bands of a given OFDM downstream
channel (from the lowest 6 MHz band of the Modulated Spectrum to the highest 6 MHz band of the Modulated
Spectrum). The CM MUST assign indices in frequency order from the OFDM channel's lowest to highest 6 MHz



238                                                  CableLabs®                                               02/19/25
