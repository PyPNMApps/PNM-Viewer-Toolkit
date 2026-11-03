# Annex F.3.3 - CM Upstream Interface Information Model

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 239-248
Classification: Normative (explicit)

<!-- PDF_PAGE: 239 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


frequency band for each of the 6 MHz bands of the channel, using an index of 1 to represent the lowest frequency
band of the Modulated Spectrum. Thus, an index of 33 represents the highest possible 6 MHz frequency band of the
Modulated Spectrum of a DOCSIS 3.1 OFDM channel. If there are interior Exclusion Bands resulting in 6 MHz
bands which contain no Active Subcarriers, then the indices corresponding to those bands will be skipped and the
power for those bands will not be reported. The CM MUST also provide the power of the PLC channel and utilize
an index value of 0 to represent the PLC channel in this table. If the placement of the OFDM channel is such that the
center frequency of the lowest or highest active subcarrier is placed on a 1 MHz grid and exactly at the edge of two
6 MHz bands, then the band which contains only the spectral edge of that subcarrier will not be included in the list
of 6 MHz bands even though by definition, that band would contain a minute portion of the Modulated Spectrum.
This logic also applies to any 6 MHz bands which are skipped due to interior Exclusion Bands.

F.3.2.3.2.35 CenterFrequency
This attribute corresponds to the center frequency of the 6 MHz band where the CM measured the average channel
power. The 6 MHz measurement band is defined as any 6 MHz band with a center frequency of 111 + 6 (n-1) MHz
for n = 1,2...185 (i.e., 111,117,...1215 MHz).
The CM MUST provide the center frequency for the 6 MHz channel, other than the one encompassing the PLC
channel, per the following formula:
    centerfreq = 111 + 6(n-1)
such that (centerfreq - 111) / 6, must be a whole number.
If the CM is reporting the frequency of the PLC, using a BandIndex of 0, the CM MUST provide the center
frequency of the lowest subcarrier of the 6 MHz encompassed spectrum containing the PLC at its center.

F.3.2.3.2.36 ReceivedPower
This attribute provides an estimate of the average power measured at the F connector input of the CM in the receive
downstream channel set for any 6 MHz bandwidth with the Center Frequency of 111 + 6(n-1) MHz for n =1,2,...185
(i.e., 111,117,...1215 MHz).
If the Center Frequency is 0, then this attribute provides an estimate of the average power measured at the F
connector input of the CM for a 6 MHz encompassed spectrum containing the DOCSIS 3.1 PLC at its center.

F.3.3    CM Upstream Interface Information Model

           Overview
This section defines the configuration and status reporting requirements for the CM Upstream Interface. This
information is contained in the [PHYv3.1] specification.




02/19/25                                           CableLabs®                                                   239
<!-- PDF_PAGE: 240 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications




                                   Figure 15 - CM Upstream Information Model


            Data Type Definitions
The following table and subsections define the CM upstream interface data types.
                                              Table 111 - Data Types

                Data Type Name                Base Type                  Permitted Values                  Reference
        UsCyclicPrefixType                  Unsigned Short    (96 | 128 | 160 | 192 | 224 | 256 | 288 |    [PHYv3.1]
                                                              320 | 384 | 512 | 640)
        UsOfdmaRollOffPeriodType            UnsignedByte      (0 | 32 | 64 | 96 | 128 | 160 | 192 | 224)   [PHYv3.1]
        UsOfdmaModulationType               Enum              other(1)                                     [PHYv3.1]
                                                              zeroValued(2)
                                                              bpsk(3)
                                                              qpsk(4)
                                                              qam8(5)
                                                              qam16(6)
                                                              qam32(7)
                                                              qam64(8)
                                                              qam128(9)
                                                              qam256(10)
                                                              qam512(11)
                                                              qam1024(12)
                                                              qam2048(13)
                                                              qam4096(14)
        UsOfdmaSubcarrierperMinislotType    UnsignedByte      (8 | 16)                                     [PHYv3.1]



F.3.3.2.1    UsOfdmaCyclicPrefixType
This data type is defined to specify the eleven possible values for the length of cyclic prefix. The cyclic prefix (in
μs) are converted into samples using the sample rate of 102.4 Msamples/s and is an integer multiple of: 1/64 * 20 μs.
The possible values come from the Upstream OFDMA Parameters table in [PHYv3.1].




240                                                CableLabs®                                                          02/19/25
<!-- PDF_PAGE: 241 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


F.3.3.2.2    UsOfdmaSubcarrierperMinislotType
This data type defines the number of subcarriers per minislot. For 2K mode, its value is 8 and for 4K mode, it is 16.
The possible values are defined in the Minislot Parameters table of [PHYv3.1].

F.3.3.2.3    UsOfdmaRollOffPeriodType
This data type is defined to specify the eight possible values for the windowing roll-off period. The Roll-Off Period
is given in number of samples using the sample rate of 102.4 Msamples/s. The possible values come from the
Upstream OFDMA Parameters table in [PHYv3.1].

F.3.3.2.4    UsOfdmaModulationType
This data type is defined to specify the modulation types supported by the CM modulator. The values are defined in
the Modulation Formats section of [PHYv3.1].

            Object Definitions

F.3.3.3.1    UsScQamChan
The UsScQamChan object provides the configuration state of an upstream SC-QAM channel.
                                   Table 112 - UsScQamChan Object Attributes

   Attribute Name              Type            Access            Type Constraints                Units     Default
                                                                                                            Value
 IfIndex                InterfaceIndex         Key
 ChannelId              ChId                   R/O       0..255
 Freq                   UnsignedInt            R/O       5800000..84200000                  Hz
 Width                  UnsignedInt            R/O       1600000 | 3200000 | 6400000        Hz
 ModulationProfile      UnsignedInt            R/O
 SlotSize               UnsignedInt            R/O                                          ticks
 TxTimingOffset         UnsignedInt            R/O
 RangingBackoffStart    UnsignedByte           R/O       0..16
 RangingBackoffEnd      UnsignedByte           R/O       0..16
 TxBackoffStart         UnsignedByte           R/O       0..16
 TxBackoffEnd           UnsignedByte           R/O       0..16
 ScdmaActiveCodes       UnsignedInt            R/O       0 | 64..66 |68..70 |72 |74..78
                                                         |80..82 |84..88 |90..96 |98..100
                                                         |102 |104..106 |108 |110..112
                                                         |114..126 |128
 ScdmaCodesPerSlot      UnsignedByte           R/O       0 | 2..32
 ScdmaFrameSize         UnsignedByte           R/O       0 | 2..32
 ScdmaHoppingSeed       UnsignedShort          R/O       0..32767
 Type                   Enum                   R/O       unknown(0)
                                                         tdma(1)
                                                         atdma(2)
                                                         scdma(3)
                                                         tdmaAndAtdma(4)
 CloneFrom              InterfaceIndexOrZero   R/O
 Update                 Boolean                R/O
 PreEqEnable            Boolean                R/O
 TxPsd                  UnsignedShort          R/O                                          quarter dBmV
 EstimatedRgDelay       UnsignedInt            R/O                                          microseconds




02/19/25                                             CableLabs®                                                   241
<!-- PDF_PAGE: 242 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


                                  Table 113 - UsScQamChan Object Associations

  Associated Object           Type          Near-end            Far-end                         Label
       Name                                Multiplicity        Multiplicity
  If                      Association     1                0                  if.Type=SC-QAM Upstream Interface



F.3.3.3.1.1 IfIndex
This attribute is the interface index of the upstream interface and is a key to provide an index into the table.

F.3.3.3.1.2 ChannelId
This attribute is the Upstream Channel Identifier.
References: [RFC 4546] docsIfUpChannelId

F.3.3.3.1.3 Freq
This attribute is the center of the frequency band associated with this upstream interface. This object returns 0 if the
frequency is undefined or unknown.
References: [RFC 4546] docsIfUpChannelFrequency

F.3.3.3.1.4 Width
This attribute is the bandwidth of this upstream interface. This object returns 0 if the interface width is undefined or
unknown.
References: [RFC 4546] docsIfUpChannelWidth

F.3.3.3.1.5 ModulationProfile
This attribute is the modulation profile for the upstream channel.
References: [RFC 4546] docsIfUpChannelModulationProfile

F.3.3.3.1.6 SlotSize
This attribute is the slot size for the upstream channel. It is applicable to TDMA and ATDMA channel types only.
The number of 6.25 microsecond ticks in each upstream minislot. This object returns zero if the value is undefined
or unknown or in case of an SCDMA channel.
References: [RFC 4546] docsIfUpChannelSlotSize

F.3.3.3.1.7 TxTimingOffset
This attribute is the measure of the current round trip time obtained from the ranging offset (initial ranging offset +
ranging offset adjustments). Units are one 64th fraction of 6.25 microseconds.
References: [RFC 4546] docsIfUpChannelTxTimingOffset

F.3.3.3.1.8 RangingBackoffStart
This attribute is the initial random backoff window the CM will use when retrying Ranging Requests. Expressed as a
power of 2. A value of 16 at the CMTS indicates that a proprietary adaptive retry mechanism is to be used.
References: [RFC 4546] docsIfUpChannelRangingBackoffStart

F.3.3.3.1.9 RangingBackoffEnd
This attribute is the final random backoff window the CM will use when retrying Ranging Requests. Expressed as a
power of 2. A value of 16 at the CMTS indicates that a proprietary adaptive retry mechanism is to be used.



242                                                  CableLabs®                                                    02/19/25
<!-- PDF_PAGE: 243 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


References: [RFC 4546] docsIfUpChannelRangingBackoffEnd

F.3.3.3.1.10 TxBackoffStart
This attribute is the initial random backoff window the CM will use when retrying transmissions. Expressed as a
power of 2. A value of 16 at the CMTS indicates that a proprietary adaptive retry mechanism is to be used.
References: [RFC 4546] docsIfUpChannelTxBackoffStart

F.3.3.3.1.11 TxBackoffEnd
This attribute is the final random backoff window the CM will use when retrying transmissions. Expressed as a
power of 2. A value of 16 at the CMTS indicates that a proprietary adaptive retry mechanism is to be used.
References: [RFC 4546] docsIfUpChannelTxBackoffEnd

F.3.3.3.1.12 ScdmaActiveCodes
This attribute is the SCDMA Active Codes. It is applicable for SCDMA channel types only. This object returns the
number of active codes. It returns zero for non-SCDMA channel types. Note that legal values for
ScdmaActiveCodes from 64..128 MUST be non-prime.
References: [RFC 4546] docsIfUpChannelScdmaActiveCodes

F.3.3.3.1.13 ScdmaCodesPerSlot
This attribute is the number of SCDMA codes per minislot. It is applicable for SCDMA channel types only.
It returns zero if the value is undefined or unknown or in case of a TDMA or ATDMA channel.
References: [RFC 4546] docsIfUpChannelScdmaCodesPerSlot

F.3.3.3.1.14 ScdmaFrameSize
This attribute is the SCDMA frame size in units of spreading intervals. It is applicable for SCDMA channel types
only. This value returns zero for non-SCDMA Profiles.
References: [RFC 4546] docsIfUpChannelScdmaFrameSize

F.3.3.3.1.15 ScdmaHoppingSeed
This attribute is the 15-bit seed used for code hopping sequence initialization. It is applicable for SCDMA channel
types only. This objects returns zero for non-SCDMA channel types.
References: [RFC 4546] docsIfUpChannelScdmaHoppingSeed

F.3.3.3.1.16 Type
This attribute reflects the Upstream channel type.
References: [RFC 4546] docsIfUpChannelType

F.3.3.3.1.17 CloneFrom
This attribute is meaningless on a Cable Modem.
References: [RFC 4546] docsIfUpChannelCloneFrom

F.3.3.3.1.18 Update
This attribute always returns 'false' on a Cable Modem.
References: [RFC 4546] docsIfUpChannelUpdate




02/19/25                                             CableLabs®                                                   243
<!-- PDF_PAGE: 244 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


F.3.3.3.1.19 PreEqEnable
This attribute reflects the status of pre-equalization as represented in the RNG-RSP. Pre-equalization is considered
enabled at the CM if a RNG-RSP with pre-equalization data has been received at least once since the last mac
reinitialization.
References: [RFC 4546] docsIfUpChannelPreEqEnable

F.3.3.3.1.20 TxPsd
This attribute represents P1.6r_n, the power spectral density in 1.6 MHz, for the associated SC-QAM upstream
channel.

F.3.3.3.1.21 EstimatedRgDelay
This attribute reports the estimate of the Request-Grant delay for this upstream channel, as calculated in the
calcAllowedAQ function. The value is used by the CM for setting the AllowedAqBytes value for any Low Latency
Service Flow that uses this upstream channel. If a particular upstream channel is not used by any Low Latency
Service Flow, the CM MAY report 0 for the EstimatedRgDelay attribute.
References: [MULPIv3.1] estimatedRgDelay parameter calculated via the 'calcAllowedAQ()' function in Annex O.1
AQM Utility Functions

F.3.3.3.2     UsChExt
This object defines management extensions for upstream channels, in particular SCDMA parameters.
                                      Table 114 - UsChExt Object Attributes

           Attribute Name                  Type            Access          Type Constraints           Units    Default
 IfIndex                           InterfaceIndex         key         InterfaceIndex of MAC Domain
                                                                      interface
 SacCodeHoppingSelectionMode       Enum                   read-only   none(0)
                                                                      sac1NoCodeHopping(1)
                                                                      sac1CodeHoppingMode1(2)
                                                                      sac2CodeHoppingMode2(3)
                                                                      sac2NoCodeHopping(4)
 ScdmaSelectionStringActiveCodes   ScdmaSelectionString   read-only



F.3.3.3.2.1 IfIndex
This key represents the interface index of the logical upstream channel to which this instance applies.

F.3.3.3.2.2 SacCodeHoppingSelectionMode
This attribute indicates the selection mode for active codes and code hopping.
      •    'none'
           Non-SCDMA channel
      •    'sac1NoCodeHopping'
           Selectable active codes mode 1 and code hopping disabled
      •    'sac1CodeHoppingMode1'
           Selectable active codes mode 1 and code hopping mode 1
      •    'sac2CodeHoppingMode2'
           Selectable active codes mode 2 and code hopping mode 2
      •    'sac2NoCodeHopping'
           Selectable active codes mode 2 and code hopping disabled



244                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 245 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


References: Minislot Numbering Parameters in Timing and Synchronization section

F.3.3.3.2.3 ScdmaSelectionStringActiveCodes
This attribute represents the active codes of the upstream channel and it is applicable only when
SacCodeHoppingSelectionMode is 'sac2CodeHoppingMode2'.
References: [MULPIv3.1] Minislot Numbering Parameters in Timing and Synchronization section

F.3.3.3.3       UsOfdmaChannel
The UsOfdmaChannel object reports the configuration properties of an upstream OFDMA channel.
                                   Table 115 - UsOfdmaChannel Object Attributes

     Attribute Name                     Type                Access     Type Constraints            Units           Default
                                                                                                                    Value
 Ifindex                     InterfaceIndex                 Key
 ConfigChangeCt              UnsignedByte                   R/O        0..255
 SubcarrierZeroFreq          UnsignedInt                    R/O        5000000..197600000    Hz
 FirstActiveSubcarrierNum    UnsignedInt                    R/O        74..3947
 LastActiveSubcarrierNum     UnsignedInt                    R/O        74..3947
 RollOffPeriod               UsOfdmaRollOffPeriodType       R/O                              Number of samples
 CyclicPrefix                UsOfdmaCyclicPrefixType        R/O                              Number of samples
 SubcarrierSpacing           SubcarrierSpacingType          R/O                              Hz
 NumSymbolsPerFrame          UnsignedInt                    R/O        6..36
 NumActiveSubcarriers        UnsignedInt                    R/O        1..3800
 TransmitPower               UnsignedInt                    R/O                              Quarter dBmV
 PreEqualizationEnabled      Boolean                        R/O        True/False
 ChannelId                   UnsignedInt                    R/O        0..255
 EstimatedRgDelay            UnsignedInt                    R/O                              microseconds


                                 Table 116 - UsOfdmaChannel Object Associations

           Associated Object               Type          Near-end           Far-end                   Label
                Name                                    Multiplicity       Multiplicity
      If                          Association           1                                   If.Type=Upstream Interface
      UsOfdmaProfileStats         Directed                               1..2
                                  Composition



F.3.3.3.3.1 IfIndex
This attribute is the interface index of the upstream interface and is a key to provide an index into the table.

F.3.3.3.3.2 ConfigChangeCt
This attribute contains the value of the Configuration Change Count field in the Upstream Channel Descriptor
(UCD) MAC Management Message corresponding to this upstream channel.

F.3.3.3.3.3 SubcarrierZeroFreq
This attribute specifies the center frequency of subcarrier 0 of the OFDMA channel. Note that since subcarrier 0 is
always excluded, it will actually be below the allowed upstream spectrum band.




02/19/25                                               CableLabs®                                                        245
<!-- PDF_PAGE: 246 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


F.3.3.3.3.4 FirstActiveSubcarrierNum
This attribute corresponds to the index of the first non-excluded subcarrier.

F.3.3.3.3.5 LastActiveSubcarrierNum
This attribute corresponds to the index of the last non-excluded subcarrier.

F.3.3.3.3.6 RollOffPeriod
Windowing is applied in order to maximize channel capacity by sharpening the edges of the spectrum of the
OFDMA signal. Windowing is applied in the time domain by tapering (or rolling off) the edges using a raised cosine
function. There are eight possible values of roll-off prefix. The Roll-Off Period is given in the number of samples
using the sample rate of 102.4 Msamples/s. The configuration where Roll-off prefix value is greater than or equal to
cyclic prefix value is considered invalid.

F.3.3.3.3.7 CyclicPrefix
Cyclic prefix is added in order to enable the receiver to overcome the effects of inter-symbol interference (ISI) and
inter-carrier interference caused by microreflections in the channel. The cyclic prefix (in μs) is converted into
samples using the sample rate of 102.4 Msamples/s. There are eleven possible values for the length of the CP and
the choice depends on the delay spread of the channel - a longer delay spread requires a longer cyclic prefix.

F.3.3.3.3.8 SubcarrierSpacing
This attribute defines the subcarrier spacing associated with a particular FFT mode configured on the OFDMA
upstream channel. If it is 2K mode, then the subcarrier spacing is 50kHz. If it is 4K mode, then the subcarrier
spacing is 25kHz.

F.3.3.3.3.9 NumSymbolsPerFrame
This attribute defines the number of symbol periods per frame. For channel bandwidth greater than 72MHz, the
maximum number of symbol periods per frame is 18 for 2K mode and 9 for 4K mode. For channel bandwidth less
than 72 MHz but greater than 48MHz, the maximum number of symbols per frame is 24 for 2K mode and 12 for 4K
mode. For channel bandwidth less than 48MHz, the maximum number of symbol periods is 36 for 2K mode and 18
for 4K mode. The minimum number of symbol periods per frame is 6 for both the FFT modes and is independent of
the channel bandwidth.

F.3.3.3.3.10 NumActiveSubCarriers
This attribute defines the number of active subcarriers within the OFDMA upstream channel.

F.3.3.3.3.11 TransmitPower
This attribute represents the operational transmit power for the associated OFDMA upstream channel. The CM
reports its Target Power, P1.6r_n as described in [PHYv3.1].

F.3.3.3.3.12 PreEqualizationEnabled
This attribute defines whether pre-equalization is enabled on the associated OFDMA upstream channel.

F.3.3.3.3.13 ChannelId
This attribute us the upstream OFDMA channel identifier. This is an 8-bit identifier that uniquely identifies an
OFDMA upstream channel within a MAC domain.
If the interface is down, the object returns the most current value. If the upstream channel ID is unknown, this object
returns a value of 0.




246                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 247 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


F.3.3.3.3.14 EstimatedRgDelay
This attribute reports the estimate of the Request-Grant delay for this upstream channel, as calculated in the
calcAllowedAQ function. The value is used by the CM for setting the AllowedAqBytes value for any Low Latency
Service Flow that uses this upstream channel. If a particular upstream channel is not used by any Low Latency
Service Flow, the CM MAY report 0 for the EstimatedRgDelay attribute.
References: [MULPIv3.1] estimatedRgDelay parameter calculated via the 'calcAllowedAQ()' function in Annex O.1
AQM Utility Functions

F.3.3.3.4     UsOfdmaProfileStats
This object provides usage statistics for an upstream OFDMA profile. The CM MUST create an instance of the
UsOfdmaProfileStats object for each profile associated with the OFDMA upstream channel assigned to the cable
modem.
                                 Table 117 - UsOfdmaProfileStats Object Attributes

          Attribute Name                 Type           Access       Type Constraints        Units      Default Value
  Iuc                               UnsignedByte      Key
  OutOctets                         Counter64         R/O
  CounterDiscontinuityTime          TimeStamp         R/O


                               Table 118 - UsOfdmaProfileStats Object Associations

     Associated Object Name                Type                 Near-end           Far-end Multiplicity       Label
                                                               Multiplicity
    UsOfdmaMinislotCfgState         Directed                                       0..237
                                    Composition



F.3.3.3.4.1 Iuc
This attribute is the unique identifier of the upstream profile associated with the OFDMA upstream channel (in the
upstream direction the ProfileId is synonymous with the IUC). It is a key defined to provide an index into the table.

F.3.3.3.4.2 OutOctets
This attribute is the count of MAC-layer octets transmitted by the CM using this profile. This value is the size of all
unicast, multicast or broadcast frames (including all MAC-layer framing) delivered from the MAC to the Phy - this
includes user data, DOCSIS MAC Management Messages, etc.
Discontinuities in the value of this counter can occur at reinitialization of the managed system, and at other times as
indicated by the value of ProfileCounterDiscontinuityTime.

F.3.3.3.4.3 CounterDiscontinuityTime
This attribute is the value of sysUpTime on the most recent occasion at which any one or more of this entry's
counters suffered a discontinuity. If no such discontinuities have occurred since the last re-initialization of the local
management subsystem, then this attribute contains a zero value.

F.3.3.3.5     UsOfdmaMinislotCfgState
This CM object reports minislot configuration as received in the UCD message for a particular OFDMA profile.
                              Table 119 - UsOfdmaMinislotCfgState Object Attributes

        Attribute Name                 Type               Access      Type Constraints        Units      Default Value
   StartMinislotNum           UnsignedByte              Key           1..237
   FirstSubcarrierId          UnsignedShort             R/O           1..4095




02/19/25                                             CableLabs®                                                       247
<!-- PDF_PAGE: 248 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


       Attribute Name                  Type               Access       Type Constraints      Units    Default Value
   NumConsecutiveMinislots    UnsignedShort              R/O           1..237
   MinislotPilotPattern       UnsignedByte               R/O           1..14
   DataSymbolModulation       UsOfdmaModulationType      R/O



F.3.3.3.5.1 StartMinislotNum
This attribute corresponds to the unique identifier of the minislot received by the CM. It is a key defined to provide
an index into the table.

F.3.3.3.5.2 FirstSubcarrierId
This attribute corresponds to the index of the first/starting subcarrier in this minislot.

F.3.3.3.5.3 NumConsecutiveMinislots
This attribute defines the number of continuous minislots which have the same bit loading, starting with the
StartMinislotNum, defined in the associated upstream profile.

F.3.3.3.5.4 MinislotPilotPattern
This attribute defines the pilot pattern used for edge and body minislots. Pilots are used by the CMTS receiver to
adapt to channel conditions and frequency offset. Pilot patterns differ by the number of pilots in a minislot, and by
their arrangement within the minislot. For both 8 and 16 subcarriers minislot sizes, seven pilot patterns are defined.

F.3.3.3.5.5 DataSymbolModulation
This attribute defines the bit loading within the minislot.




248                                                   CableLabs®                                             02/19/25
