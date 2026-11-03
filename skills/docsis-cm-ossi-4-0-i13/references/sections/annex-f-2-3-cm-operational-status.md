# Annex F.2.3 - CM Operational Status Objects

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 257-274
Classification: Normative (explicit)

<!-- PDF_PAGE: 257 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


F.2.3     CM Operational Status Objects
This section defines configuration state and operational status management objects for CMs.




                 Figure 14 - CM Configuration State and Operational Status Information Model


           CmStatus
The CmStatus object provides CM connectivity status information of the CM previously available in the deprecated
SNMP table docsIfCmStatusTable.
References: [RFC 4546] docsIfCmStatusTable
                                       Table 125 - CmStatus Object Attributes

                                                                                                         Default
     Attribute Name              Type             Access            Type Constraints           Units      Value
IfIndex                    InterfaceIndex       Key          Interface Index of the MAC
                                                             interface
Value                      CmRegState           R/O
Code                       HexBinary            R/O          SIZE( 0 | 5 | 6 )




04/29/26                                              CableLabs                                             257
<!-- PDF_PAGE: 258 -->
CM-SP-CM-OSSIv4.0-I13-260429                                           Data-Over-Cable Service Interface Specifications


                                                                                                                         Default
      Attribute Name               Type                 Access              Type Constraints                 Units       Value
Resets                       Counter32              R/O                                                  resets
LostSyncs                    Counter32              R/O                                                  messages
InvalidMaps                  Counter32              R/O                                                  maps
InvalidUcds                  Counter32              R/O                                                  messages
InvalidRangingRsps           Counter32              R/O                                                  messages
InvalidRegRsps               Counter32              R/O                                                  messages
T1Timeouts                   Counter32              R/O                                                  timeouts
T2Timeouts                   Counter32              R/O                                                  timeouts
EnergyMgt1x1OperStatus       Boolean                R/O
EmDlsOperStatus              Boolean                R/O


                                       Table 126 - CmStatus Object Associations

                                                        Near-End                 Far-End
   Associated Object Name               Type            Multiplicity            Multiplicity                 Label
 CmFdxStatus                       Association      1                    0..1                  fdxL or fdx capability
                                   (with
                                   constraint) to
                                   CmFdxStatus



F.2.3.1.1     IfIndex
This key attribute denotes the MAC Domain interface index of the CM.

F.2.3.1.2     Value
This attribute denotes the current CM connectivity state. For the case of IP acquisition related states, this attribute
reflects states for the current CM provisioning mode, not the other DHCP process associated with dual stack
operation.
References: [MULPIv4.0] Establishing IP Connectivity section

F.2.3.1.3     Code
This attribute denotes the status code for CM as defined in the OSSI Specification. The status code consists of a
single character indicating error groups, followed by a two- or three-digit number indicating the status condition,
followed by a decimal. An example of a returned value could be 'T101.0'. The zero-length hex string indicates no
status code yet registered.
References: Annex C

F.2.3.1.4     Resets
This attribute denotes the number of times the CM reset or initialized this interface. Discontinuities in the value of
this counter can occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.5     LostSyncs
This attribute denotes the number of times the CM lost synchronization with the downstream channel.
Discontinuities in the value of this counter can occur at re-initialization of the managed system, and at other times as
indicated by the value of ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]


258                                                       CableLabs                                                    04/29/26
<!-- PDF_PAGE: 259 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


F.2.3.1.6   InvalidMaps
This attribute denotes the number of times the CM received invalid MAP messages. Discontinuities in the value of
this counter can occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.7   InvalidUcds
This attribute denotes the number of times the CM received invalid UCD messages. Discontinuities in the value of
this counter can occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.8   InvalidRangingRsps
This attribute denotes the number of times the CM received invalid ranging response messages. Discontinuities in
the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by the
value of ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.9   InvalidRegRsps
This attribute denotes the number of times the CM received invalid registration response messages. Discontinuities
in the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by
the value of ifCounterDiscontinuityTime for the CM MAC Domain interface.

F.2.3.1.10 T1Timeouts
This attribute denotes the number of times counter T1 expired in the CM. Discontinuities in the value of this counter
can occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.11 T2Timeouts
This attribute denotes the number of times counter T2 expired in the CM. Discontinuities in the value of this counter
can occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

F.2.3.1.12 EnergyMgt1x1OperStatus
This attribute indicates whether the CM is currently operating in Energy Management 1x1 Mode. If this attribute
returns true, the CM is operating in Energy Management 1x1 Mode.
References: [MULPIv4.0] Energy Management Mode Indicator section

F.2.3.1.13 EmDlsOperStatus
This attribute indicates whether the CM is currently operating in Energy Management DLS Mode. If this attribute
returns true, the CM is operating in Energy Management DLS Mode.
References: [MULPIv4.0] Energy Management Mode Indicator section




04/29/26                                            CableLabs                                                     259
<!-- PDF_PAGE: 260 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


            CmStatusUs
The CmStatusUs object defines PHY and MAC information about the CM's SC-QAM upstream channels. This
object provides per-CM upstream channel information.
                                        Table 127 - CmStatusUs Object Attributes

                                                                                                               Default
 Attribute Name                  Type           Access              Type Constraints                  Units     Value
IfIndex                InterfaceIndex          Key         Interface Index of upstream interface
TxPower                TenthdBmV               R/O                                                 dBmV
T3Timeouts             Counter32               R/O                                                 timeouts
T4Timeouts             Counter32               R/O                                                 timeouts
RangingAborteds        Counter32               R/O                                                 attempts
ModulationType         DocsisUpstreamType      R/O
EqData                 DocsEqualizerData       R/O
T3Exceededs            Counter32               R/O                                                 timeouts
IsMuted                Boolean                 R/O
RangingStatus          RangingState            R/O



F.2.3.2.1    IfIndex
This key attribute denotes the interface index of the upstream interface to which this instance applies.

F.2.3.2.2    TxPower
This attribute denotes the operational CM transmit power for this SC-QAM upstream channel. In order for this
attribute to provide consistent information under all circumstances, a 4.0 CM will report the average total power for
the SC-QAM channel the same as was done for DOCSIS 3.0, regardless of whether it is operating with a 4.0, 3.1, or
a 3.0 CMTS. The value that is reported was referred to as Pr in [PHYv3.0].

F.2.3.2.3    T3Timeouts
This attribute denotes the number of times counter T3 expired in the CM for this upstream channel. Discontinuities
in the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by
the value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.2.4    T4Timeouts
This attribute denotes the number of times counter T4 expired in the CM for this upstream channel. Discontinuities
in the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by
the value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.2.5    RangingAborteds
This attribute denotes the number of times the ranging process was aborted by the CMTS. Discontinuities in the
value of this counter can occur at re-initialization of the managed system, and at other times as indicated by the
value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]




260                                                  CableLabs                                               04/29/26
<!-- PDF_PAGE: 261 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


F.2.3.2.6    ModulationType
This attribute denotes the modulation type status currently used by the CM for this upstream channel. Since this
object specifically identifies PHY Layer mode, the shared upstream channel type 'tdmaAndAtdma' is not permitted.
References: [RFC 2863]

F.2.3.2.7    EqData
This attribute denotes the pre-equalization data for the specified upstream channel on this CM after convolution with
data indicated in the RNG-RSP. This data is valid when docsIfUpChannelPreEqEnable is set to 'true'.
References: [RFC 4546]

F.2.3.2.8    T3Exceededs
This attribute denotes the number of times for excessive T3 timeouts. Discontinuities in the value of this counter can
occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.2.9    IsMuted
This attribute denotes whether the upstream channel is muted.
References: [MULPIv4.0] Media Access Control Specification section

F.2.3.2.10 RangingStatus
This attribute denotes ranging status of this upstream channel.
References: [MULPIv4.0] Media Access Control Specification section

            CmStatusOfdmaUs
The CmStatusOfdmaUs object defines PHY and MAC information of an OFDMA upstream channel, whether it is a
non-extended upstream channel or an extended upstream channel. This object provides OFDMA upstream channel
status information.
An FDX CM instantiates this object for each FDX extended upstream channel in addition to the non-extended
upstream channels.
An FDD CM instantiates this object for each FDD extended upstream channel in addition to the non-extended
upstream channels.
                                  Table 128 - CmStatusOfdmaUs Object Attributes

                                                                                                              Default
 Attribute Name                 Type          Access               Type Constraints                  Units     Value
IfIndex               InterfaceIndex        Key           Interface Index of upstream interface
T3Timeouts            Counter32             R/O                                                   timeouts
T4Timeouts            Counter32             R/O                                                   timeouts
RangingAborteds       Counter32             R/O                                                   attempts
T3Exceededs           Counter32             R/O                                                   timeouts
IsMuted               Boolean               R/O
RangingStatus         RangingState          R/O




04/29/26                                            CableLabs                                                    261
<!-- PDF_PAGE: 262 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


                               Table 129 - CmStatusOfdmaUs Object Associations

           Associated Object                           Near-End            Far-End
                Name                    Type           Multiplicity       Multiplicity             Label
      If                         Association           1              *                  If.Type= OFDMA Upstream
                                                                                         Interface



F.2.3.3.1     IfIndex
This key attribute is the unique index of the OFDMA upstream channel.

F.2.3.3.2     T3Timeouts
This attribute denotes the number of times counter T3 expired for this OFDMA upstream channel. Discontinuities in
the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by the
value of ifCounterDiscontinuityTime for the associated OFDMA upstream channel.
References: [RFC 2863]

F.2.3.3.3     T4Timeouts
This attribute denotes the number of times counter T4 expired for this OFDMA upstream channel. Discontinuities in
the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by the
value of ifCounterDiscontinuityTime for the associated OFDMA upstream channel.
References: [RFC 2863]

F.2.3.3.4     RangingAborteds
This attribute denotes the number of times the ranging process was aborted by the CMTS for this OFDMA upstream
channel. Discontinuities in the value of this counter can occur at re-initialization of the managed system, and at other
times as indicated by the value of ifCounterDiscontinuityTime for the associated OFDMA upstream channel.
References: [RFC 2863]

F.2.3.3.5     T3Exceededs
This attribute denotes the number of times for excessive T3 timeouts for this OFDMA upstream channel.
Discontinuities in the value of this counter can occur at re-initialization of the managed system, and at other times as
indicated by the value of ifCounterDiscontinuityTime for the associated OFDMA upstream channel.
References: [RFC 2863]

F.2.3.3.6     IsMuted
This attribute denotes whether the OFDMA upstream channel is muted. A value of 'true' indicates the OFDMA
upstream channel is muted. A value of 'false' indicates the OFDMA upstream channel is not muted.
References: [MULPIv4.0] CM Status TLV Encodings section

F.2.3.3.7     RangingStatus
This attribute denotes ranging status of this OFDMA upstream channel.
References: [MULPIv4.0] Ranging Response (RNG-RSP) section

             CmCapabilities
The CmCapabilities object defines capability attributes for the CM.




262                                                 CableLabs                                                04/29/26
<!-- PDF_PAGE: 263 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


                                 Table 130 - CmCapabilities Object Attributes

       Attribute Name                  Type        Access        Type Constraints        Multiplicity     Units   Default
                                                                                                                   Value
 Req                            Tlv8               R/O                                   1
 Rsp                            Tlv8               R/O                                   1
 DocsisBaseCapability           DocsisVersion      R/O                                   1
 DiplexerUsUpperBandEdgeCap     EnumBits           R/O          usFreqRng42Mhz(0),       1
 ability                                                        usFreqRng65Mhz(1),
                                                                usFreqRng85Mhz(2),
                                                                usFreqRng117Mhz(3),
                                                                usFreqRng204Mhz(4)
 DiplexerDsLowerBandEdgeCap     EnumBits           R/O          dsFreqRng108Mhz(0),      1
 ability                                                        dsFreqRng258Mhz(1)
 DiplexerDsUpperBandEdgeCap     EnumBits           R/O          dsFreqRng1218Mhz(0),     1
 ability                                                        dsFreqRng1794Mhz(1),
                                                                dsFreqRng1002Mhz(2)
 AdvBandPlanCapability          EnumBits           R/O          supportsFdxL(0),         1
                                                                supportsFdx(1),
                                                                supportsFdd(2)


                               Table 131 - CmCapabilities Object Associations

  Associated Object Name               Type              Near-end          Far-end                       Label
                                                        Multiplicity      Multiplicity
 CmFddCapabilities             Directed             1                  0..1                  fdd capability
                               composition (with
                               constraint) to
                               CmFddCapabilities



F.2.3.4.1   Req
This attribute contains the TLV encoding for TLV-5 sent in a REG-REQ-MP. The first byte of this encoding is
expected to be '05'H.

References: [MULPIv4.0] Common TLV Encodings annex, Modem Capabilities Encoding section

F.2.3.4.2   Rsp
This attribute contains the TLV encoding for TLV-5 received in a REG-RSP-MP. The first byte of this encoding is
expected to be '05'H.

References: [MULPIv4.0] Common TLV Encodings annex, Modem Capabilities Encoding section

F.2.3.4.3   DocsisBaseCapability
This attribute indicates the DOCSIS capability of the CM. CMs report their supported DOCSIS version. This
attribute replaces docsIfDocsisBaseCapability defined in [RFC 4546].

References: [MULPIv4.0] Common TLV Encodings annex, DOCSIS Version section




04/29/26                                            CableLabs                                                       263
<!-- PDF_PAGE: 264 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


F.2.3.4.4    DiplexerUsUpperBandEdgeCapability
This attribute specifies all of the diplexer upstream upper band edge configurations supported by the CM device.
This corresponds to the Diplexer Upstream Upper Band Edge Options modem capability TLV 5.62 sent by the CM
in the Registration Request. The EnumBits value represents the value sent in the Registration Request as a bit mask.
The supported values include:
      usFreqRng42Mhz(0): Upstream Frequency Range up to 42 MHz
      usFreqRng65Mhz(1): Upstream Frequency Range up to 65 MHz
      usFreqRng85Mhz(2): Upstream Frequency Range up to 85 MHz
      usFreqRng117Mhz(3): Upstream Frequency Range up to 117 MHz
      usFreqRng204Mhz(4): Upstream Frequency Range up to 204 MHz
Example:
CM supports switchable Upstream Diplexer Upstream Upper Band Edge configurations as follows:
usFreqRng42Mhz(0): Upstream Frequency Range up to 42 MHz
usFreqRng85Mhz(2): Upstream Frequency Range up to 85 MHz

The EnumBits value set for the DiplexerUsUpperBandEdgeCapability in this case would equal 5.
References: [MULPIv4.0] Common TLV Encodings annex, Diplexer Upstream Upper Band Edge Options; [DOCS-
IF31-MIB] docsIf31CmSystemCfgStateDiplexerCapability

F.2.3.4.5    DiplexerDsLowerBandEdgeCapability
This attribute specifies the diplexer downstream lower band edge configurations supported by the CM device. This
corresponds to the Diplexer Downstream Lower Band Edge Options modem capability TLV 5.60 sent by the CM in
the Registration Request. The EnumBits value represents the value sent in the Registration Request as a bit mask.
The supported values include:
      dsFreqRng108Mhz(0): Downstream Frequency Range starting from 108 MHz
      dsFreqRng258Mhz(1): Downstream Frequency Range starting from 258 MHz
Example:
CM supports switchable Downstream Diplexer Downstream Lower Band Edge configurations as follows:
dsFreqRng108Mhz(0): Downstream Frequency Range starting from 108 MHz
dsFreqRng258Mhz(1): Downstream Frequency Range starting from 258 MHz

The EnumBits value set for the DiplexerDsLowerBandEdgeCapability in this case would be 3.
References: [MULPIv4.0] Common TLV Encodings annex, Diplexer Downstream Lower Band Edge Options;
[DOCS-IF31-MIB] docsIf31CmSystemCfgStateDiplexerDsLowerCapability

F.2.3.4.6    DiplexerDsUpperBandEdgeCapability
This attribute specifies the diplexer downstream upper band edge configurations supported by the CM device. This
corresponds to the Diplexer Downstream Upper Band Edge Options modem capability TLV 5.61 sent by the CM in
the Registration Request. The EnumBits value represents the value sent in the Registration Request as a bit mask.
The supported values include:
      dsFreqRng1218Mhz(0): Downstream Frequency Range up to 1218 MHz
      dsFreqRng1794Mhz(1): Downstream Frequency Range up to 1794 MHz
      dsFreqRng1002Mhz(2): Downstream Frequency Range up to 1002 MHz
Example:
CM supports only a single configuration for the DiplexerDsUpperBandEdgeCapability as follows:
dsFreqRng1218Mhz(0): Downstream Frequency Range up to 1218 MHz

The EnumBits values set for the DiplexerDsUpperBandEdgeCapability in this case would be 1.




264                                                CableLabs                                              04/29/26
<!-- PDF_PAGE: 265 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv4.0-I13-260429


References: [MULPIv4.0] Common TLV Encodings annex, Diplexer Downstream Upper Band Edge Options;
[DOCS-IF31-MIB] docsIf31CmSystemCfgStateDiplexerDsUpperCapability

F.2.3.4.7     AdvBandPlanCapability
This attribute indicates the ability of the CM to participate in Advanced Band Plan Operation as an FDX, FDD, or
FDX-L.
If the 'supportsFdxL' bit (bit 0) is set to '1', the CM is able to operate as an FDX-L CM.
If the 'supportsFdx' bit (bit 1) is set to '1', the CM is able to operate as an FDX CM.
If the 'supportsFdd' bit (bit 2) is set to '1', the CM is able to operate as an FDD CM.
The 'supportsFdxL' and 'supportsFdx' options are mutually exclusive.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Band Plan Capability section

             CmDpvStats
The CmDpvStats object represents the DOCSIS Path Verify Statistics collected in the cable modem device. The
CMTS controls the logging of DPV statistics in the cable modem. Therefore, the context and nature of the
measurements are governed by the CMTS and not self-descriptive when read from the CM.
                                         Table 132 - CmDpvStats Object Attributes

                                                                                                                Default
    Attribute Name          Type             Access               Type Constraints                  Units        Value
   IfIndex              InterfaceIndex     Key          Interface index of downstream interface
   GrpId                UnsignedByte       Key          1..2
   LastMeasLatency      UnsignedInt        R/O                                                    nanoseconds
   LastMeasTime         DateTime           R/O
   MinLatency           UnsignedInt        R/O                                                    nanoseconds
   MaxLatency           UnsignedInt        R/O                                                    nanoseconds
   AvgLatency           UnsignedInt        R/O                                                    nanoseconds
   NumMeas              UnsignedInt        R/O                                                    nanoseconds
   LastClearTime        DateTime           R/O



F.2.3.5.1     IfIndex
This key attribute represents the interface Index of the Downstream Interface where the measurements are taken.

F.2.3.5.2     GrpId
This key attribute represents the DPV Group ID. The CM reports two instance of DPV statistics per downstream
normally referred as Statistical Group 1 and Statistical Group 2.

F.2.3.5.3     LastMeasLatency
This attribute represents the last latency measurement for this statistical group.

F.2.3.5.4     LastMeasTime
This attribute represents the last measurement time of the last latency measurement for this statistical group. This
attribute reports the epoch time value when no measurements are being reported or after the statistics were cleared.




04/29/26                                              CableLabs                                                     265
<!-- PDF_PAGE: 266 -->
CM-SP-CM-OSSIv4.0-I13-260429                                       Data-Over-Cable Service Interface Specifications


F.2.3.5.5       MinLatency
This attribute represents the minimum latency measurement for this statistical group since the last time statistics
were cleared.

F.2.3.5.6       MaxLatency
This attribute represents the maximum latency measurement for this statistical group since the last time statistics
were cleared.

F.2.3.5.7       AvgLatency
This attribute represents the average latency measurement for this statistical group since the last time statistics were
cleared. The averaging mechanism is controlled by the CMTS.
References: [MULPIv4.0] DPV Math section

F.2.3.5.8       NumMeas
This attribute represents the number of latency measurements made for this statistical group since the last time
statistics were cleared.

F.2.3.5.9       LastClearTime
This attribute represents the last time statistics were cleared for this statistical group, otherwise this attribute reports
the epoch time value.

              CmEventCtrl
The CmEventCtrl object represents the control mechanism to enable the dispatching of events based on the event Id.
The following rules define the event control behavior.
      •      The CmEventCtrl object has no instances or contains an instance with Event ID 0.
             All events matching the Local Log settings of docsDevEvReporting are sent to local log ONLY.
      •      Additionally, if The CmEventCtrl object contains configured instances with non-zero Event IDs.
             Events matching the Event Ids configured in the object are sent according to the settings of the
             docsDevEvReporting object; i.e., traps, syslog, etc.
The CM MUST NOT persist instances of CmEventCtrl across reinitializations.
                                           Table 133 - CmEventCtrl Object Attributes

          Attribute Name        Type            Access         Type Constraints             Units       Default Value
          EventId            UnsignedInt       Key



F.2.3.6.1       EventId
This key attribute represents the Event ID of the event being enabled for delivery to a dispatch mechanism (e.g.,
syslog).

              CmEventNotif
The CmEventNotif object represents the abstract definition of an event object for the CM. The realization of the
event object depends of the management protocol that carries the event. For example, the object event realization as
a SNMP notification is defined as the docsIf3CmEventNotif defined in [DOCS-IF3-MIB].

              CmEm1x1Stats
The CmEm1x1Stats object defines Energy Management 1x1 mode statistics on the CM to provide insight into
configuration of appropriate EM 1x1 Mode Activity Detection thresholds and/or to get feedback on how/if the



266                                                      CableLabs                                                04/29/26
<!-- PDF_PAGE: 267 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


current thresholds are working well or are causing user experience issues. These statistics are only applicable/valid
when the Energy Management 1x1 mode is enabled in the CM.
                                    Table 134 - CmEm1x1Stats Object Attributes

                                                                                                              Default
                   Attribute Name                 Type         Access       Type Constraints       Units       Value
   NumberTimesCrossedBelowUsEntryThrshlds     UnsignedInt     R/O
   NumberTimesCrossedBelowDsEntryThrshlds     UnsignedInt     R/O
   TotalDuration                              UnsignedInt     R/O                                seconds
   TotalDurationBelowUsThrshlds               UnsignedInt     R/O                                seconds
   TotalDurationBelowDsThrshlds               UnsignedInt     R/O                                seconds
   TotalDurationBelowUsDsThrshlds             UnsignedInt     R/O                                seconds



F.2.3.8.1    NumberTimesCrossedBelowUsEntryThrshlds
This attribute indicates the number of times since registration the CM crossed below the upstream entry bitrate
threshold for a number of consecutive seconds equal to or exceeding the upstream entry time threshold.

F.2.3.8.2    NumberTimesCrossedBelowDsEntryThrshlds
This attribute indicates the number of times since registration the CM crossed below the downstream entry bitrate
threshold for a number of consecutive seconds equal to or exceeding the downstream entry time threshold.

F.2.3.8.3    TotalDuration
This attribute indicates the total time duration, in seconds since registration, the CM has been in Energy
Management 1x1 mode, as controlled by the DBC-REQ Energy Management 1x1 Mode Indicator TLV. This
attribute differs from TotalDurationBelowUsDsThrshlds because it is dependent on effects of the Energy
Management Cycle Period, and processing of EM-REQ/EM-RSP messages and DBC messages that specifically
indicate entry into or exit from Energy Management 1x1 mode.

F.2.3.8.4    TotalDurationBelowUsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM satisfied upstream conditions
for entry into or remaining in Energy Management 1x1 mode.

F.2.3.8.5    TotalDurationBelowDsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM satisfied downstream conditions
for entry into or remaining in Energy Management 1x1 mode.

F.2.3.8.6    TotalDurationBelowUsDsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM, with respect to both upstream
and downstream entry and exit thresholds, satisfied conditions for entry into and remaining in Energy Management
1x1 mode. This attribute differs from TotalDuration because it is not dependent on effects of the Energy
Management Cycle Period or processing of EM-REQ/EM-RSP messages and DBC messages that specifically
indicate entry into or exit from Energy Management 1x1 mode.

            CmEmDlsStats
The CmEmDlsStats object defines the DLS Energy Management mode statistics on the CM to provide insight into
configuration of appropriate DLS EM Mode Activity Detection thresholds and/or to get feedback on how/if the
current thresholds are working well or are causing user experience issues. These statistics are only applicable/valid
when the DLS Energy Management mode is enabled in the CM.




04/29/26                                            CableLabs                                                      267
<!-- PDF_PAGE: 268 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


                                    Table 135 - CmEmDlsStats Object Attributes

                   Attribute Name                  Type         Access     Type Constraints        Units      Default
   NumberTimesCrossedBelowUsEntryThrshlds       UnsignedInt    R/O
   NumberTimesCrossedBelowDsEntryThrshlds       UnsignedInt    R/O
   TotalDuration                                UnsignedInt    R/O                               seconds
   TotalDurationBelowUsThrshlds                 UnsignedInt    R/O                               seconds
   TotalDurationBelowDsThrshlds                 UnsignedInt    R/O                               seconds
   TotalDurationBelowUsDsThrshlds               UnsignedInt    R/O                               seconds
   NumSleepLatencyTriggers                      Counter32      R/O
   NumSleepByteCtTriggers                       Counter32      R/O



F.2.3.9.1    NumberTimesCrossedBelowUsEntryThrshlds
This attribute indicates the number of times since registration the CM crossed below the upstream entry bitrate
threshold for a number of consecutive seconds equal to or exceeding the upstream entry time threshold.

F.2.3.9.2    NumberTimesCrossedBelowDsEntryThrshlds
This attribute indicates the number of times since registration the CM crossed below the downstream entry bitrate
threshold for a number of consecutive seconds equal to or exceeding the downstream entry time threshold.

F.2.3.9.3    TotalDuration
This attribute indicates the total time duration, in seconds since registration, the CM has been in DLS Energy
Management mode, as controlled by the DBC-REQ Energy Management Mode Indicator TLV. This attribute differs
from TotalDurationBelowUsDsThrshlds because it is dependent on effects of the Energy Management Cycle Period,
and processing of EM-REQ/EM-RSP messages and DBC messages that specifically indicate entry into or exit from
DLS Energy Management mode.

F.2.3.9.4    TotalDurationBelowUsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM satisfied upstream conditions
for entry into or remaining in DLS Energy Management mode.

F.2.3.9.5    TotalDurationBelowDsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM satisfied downstream conditions
for entry into or remaining in DLS Energy Management mode.

F.2.3.9.6    TotalDurationBelowUsDsThrshlds
This attribute indicates the total time duration, in seconds since registration, the CM, with respect to both upstream
and downstream entry and exit thresholds, satisfied conditions for entry into and remaining in DLS Energy
Management mode. This attribute differs from TotalDuration because it is not dependent on effects of the Energy
Management Cycle Period or processing of EM-REQ/EM-RSP messages and DBC messages that specifically
indicate entry into or exit from DLS Energy Management mode.

F.2.3.9.7    NumSleepLatencyTriggers
This attribute indicates the number of times since registration the CM transitioned to the DLS wake state due to the
DLS Maximum Sleep Latency being exceeded.

F.2.3.9.8    NumSleepByteCtTriggers
This attribute indicates the number of times since registration the CM transitioned to the DLS wake state due to the
DLS Maximum Byte Count being exceeded.



268                                                 CableLabs                                                04/29/26
<!-- PDF_PAGE: 269 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


             CmEmDlsStatus
The CmEmDlsStatus object defines the DLS Energy Management mode status on the CM to provide insight into the
current configuration of DLS Mode. This information is only applicable/valid when the DLS Energy Management
mode is enabled on the CM.
                                  Table 136 - CmEmDlsStatus Object Attributes

     Attribute Name                Type              Access    Type Constraints              Units              Default
  AssignedEmIds           HexBinary                  R/O       SIZE (2 | 4 | 6)
  DlsReceiveTimer         PlcFrameInterval(1 byte)   R/O       0..2                       PlcFrame        0
  DlsMaxSleepLatency      UnsignedByte               R/O       1..255                     msec            100
  DlsMaxSleepBytes        UnsignedShort              R/O       1..65535                   bytes           1Kbytes



F.2.3.10.1 AssignedEmIds
This attribute reports the set of CMTS-assigned EM-IDs for this cable modem. This attribute is encoded as an array
16-bit binary values with up to 3 elements. The broadcast EM-ID is not included in the list. This is generally
displayed as a comma-delimited list of EM-IDS such as: DF13,ABAB,0002.
References: [MULPIv4.0] DOCSIS Light Sleep Feature section

F.2.3.10.2 DlsReceiveTimer
This attribute specifies how long the CM is required to continue listening on the downstream for traffic, after
reception of the EMM with Sleep Time with a non-zero value. The CMTS communicates the EM Receive Timer to
the CM during registration or in DBC message.

F.2.3.10.3 DlsMaxSleepLatency
This attribute specifies the amount of time the CM would allow an upstream channel to queue the packets without
transitioning to DLS wake state.

F.2.3.10.4 DlsMaxSleepBytes
This attribute specifies the maximum number of bytes a CM would allow an upstream service flow to enqueue
without transitioning to DLS wake state.

             CmSystemCfgState
The CmSystemCfgState object defines configuration state at the global or system wide level for the CM.
                                Table 137 - CmSystemCfgState Object Attributes

        Attribute Name                 Type           Access          Type Constraints            Units             Default
AqmUsEnable                      Boolean             R/O
DefaultUsTargetBuffer            UnsignedShort       R/O                                      msec
EnergyMgtFeatureEnabled          EnumBits            R/O       em1x1Feature(0)
                                                               dls(1)
EnergyMgtCyclePeriod             UnsignedShort       R/O                                      seconds           900
DiplexerUsUpperBandEdgeCfg       UnsignedInt         R/O       42 | 65 | 85 | 117 | 204       MHz
DiplexerDsLowerBandEdgeCfg       UnsignedInt         R/O       108 | 258                      MHz
DiplexerDsUpperBandEdgeCfg       UnsignedInt         R/O       1218 | 1794 | 1002             MHz
LgVqEwmaAlpha                    UnsignedByte        R/W R/O                                                    7
VqInterval                       UnsignedShort       R/W R/O   300 | 400 | 500 | 600 | 700 | microseconds       500
                                                               800 | 900 | 1000




04/29/26                                             CableLabs                                                           269
<!-- PDF_PAGE: 270 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


                               Table 138 - CmSystemCfgState Object Associations

                                                          Near-End           Far-End
  Associated Object Name                 Type             Multiplicity      Multiplicity                Label
 CmFddSystemCfgState            Directed                 1               0..1                fdd capability
                                composition(with
                                constraint) to
                                CmFddSystemCfgState



F.2.3.11.1 AqmUsEnable
If this attribute is set to 'false', the CM disables Active Queue Management (AQM) on all upstream service flows.
References: [DOCS-QOS3-MIB] docsQosCmSystemCfgStateAqmUsEnable

F.2.3.11.2 DefaultUsTargetBuffer
This attribute specifies the default upstream service flow target buffer size, in milliseconds, when not specified
otherwise in service flow TLV encodings.
References: [MULPIv4.0] Default Upstream Target Buffer Configuration Annex; [DOCS-QOS3-MIB]
docsQosCmSystemCfgStateDefaultUsTargetBuffer

F.2.3.11.3 EnergyMgtFeatureEnabled
This attribute indicates which energy savings features have been enabled in the cable modem. The CM enables use
of Energy Management Features only if both the Energy Management Feature Control TLV and Energy
Management Modem Capability Response from the CMTS indicate that the feature is enabled. If bit 0 is set, the
Energy Management 1x1 Mode feature is enabled. If bit 1 is set, the DOCSIS Light Sleep Mode feature is enabled.
References: [MULPIv4.0] Energy Management Feature Control section; [DOCS-IF3-MIB]
docsIf3CmEnergyMgtCfgFeatureEnabled

F.2.3.11.4 EnergyMgtCyclePeriod
This attribute specifies a minimum time period (in seconds) that needs to elapse between EM-REQ transactions in
certain situations.
      •   This attribute sets the minimum cycle time that a CM will use for sending requests to enter an Energy
          Management Mode. The CM will not request to enter an Energy Management Mode while this amount of
          time has yet to elapse since the last time the CM requested an Energy Management Mode and received a
          response indicating (0) OK or (1) Reject Temporary (with no Hold-off Timer value provided).
      •   In the case that the CM fails to receive an EM-RSP message after the maximum number of retries, this
          attribute sets the minimum amount of time to elapse before the CM can attempt another EM-REQ
          transaction.
References: [MULPIv4.0] Energy Management Cycle Period section; [DOCS-IF3-MIB]
docsIf3CmEnergyMgtCfgCyclePeriod

F.2.3.11.5 DiplexerUsUpperBandEdgeCfg
This attribute specifies the current configured frequency of the upstream upper band edge of the diplexer in the CM
device. This value is expressed in MHz and corresponds to one of the Diplexer Upstream Upper Band Edge Option
capabilities reported by the CM in its Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Diplexer Upstream Upper Band Edge Configuration;
[DOCS-IF31-MIB] docsIf31CmSystemCfgStateDiplexerCfgBandEdge




270                                                 CableLabs                                                  04/29/26
<!-- PDF_PAGE: 271 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


F.2.3.11.6 DiplexerDsLowerBandEdgeCfg
This attribute specifies the current configured frequency of the downstream lower band edge of the diplexer in the
CM device. This value is expressed in MHz and corresponds to one of the Diplexer Downstream Lower Band Edge
Options capabilities reported by the CM in its Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Downstream Lower Band Edge Configuration; [DOCS-
IF31-MIB] docsIf31CmSystemCfgStateDiplexerCfgDsLowerBandEdge

F.2.3.11.7 DiplexerDsUpperBandEdgeCfg
This attribute specifies the current configured frequency of the downstream upper band edge of the diplexer in the
CM device. This value is expressed in MHz and corresponds to one of the Diplexer Downstream Upper Band Edge
Options capabilities reported by the CM in its Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Downstream Upper Band Edge Configuration; [DOCS-
IF31-MIB] docsIf31CmSystemCfgStateDiplexerCfgDsUpperBandEdge

F.2.3.11.8 LgVqEwmaAlpha
This attribute represents the log base 2 of the reciprocal of the exponentially weighted moving average weight for
the Low Latency Virtual Queue (VQ).
This attribute is configurable only through the CM configuration file via TLV-11. This attribute is read-only during
the CM operational state.
References: [MULPIv3.1] LG_VQ_EWMA_ALPHA parameter defined in pseudocode in Annex O.1 AQM Utility
Functions

F.2.3.11.9 VqInterval
This attribute represents the interval between alignments of the Virtual Queue (VQ) with the actual queue.
This attribute is configurable only through the CM configuration file via TLV-11. This attribute is read-only during
the CM operational state.
References: [MULPIv3.1] VQ_INTERVAL parameter defined in pseudocode in Annex O.1 AQM Utility Functions

           CmFddSystemCfgState
The CmFddSystemCfgState object reports the current configuration properties of FDD Extended Upstream Channel
attributes for the FDD CM.
                              Table 139 - CmFddSystemCfgState Object Attributes

        Attribute Name                 Type         Access         Type Constraints       Units           Default
DiplexerDsLowerBandEdgeCfg        UnsignedShort   R/O            0 | 5..65535           MHz
DiplexerDsUpperBandEdgeCfg        UnsignedShort   R/O            0 | 5..65535           MHz
DiplexerUsUpperBandEdgeCfg        UnsignedShort   R/O            0 | 5..65535           MHz


                             Table 140 - CmFddSystemCfgState Object Associations

                                                  Near-End
  Associated Object Name           Type           Multiplicity          Far-End Multiplicity            Label
 CmCapabilities                 Association   1                        1                       fdd capability
                                (with
                                constraint)




04/29/26                                           CableLabs                                                       271
<!-- PDF_PAGE: 272 -->
CM-SP-CM-OSSIv4.0-I13-260429                                       Data-Over-Cable Service Interface Specifications


F.2.3.12.1 DiplexerDsLowerBandEdgeCfg
This attribute specifies the starting (lowest) frequency for which the downstream band is currently configured in the
FDD cable modem. This corresponds to the Advanced Diplexer Downstream Lower Band Edge Configuration sent
by the CM in the Registration Request as TLV type 5.79. The two-byte unsigned integer represents the frequency in
MHz. The value zero indicates the CM is currently not configured with a channel in the extended spectrum.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Diplexer Downstream Lower Band Edge
Configuration section; [PHYv4.0] Upstream and Downstream Frequency Plan for FDD Operation

F.2.3.12.2 DiplexerDsUpperBandEdgeCfg
This attribute specifies the ending (highest) frequency for which the downstream band is currently configured in the
FDD cable modem. This corresponds to the Advanced Diplexer Downstream Upper Band Edge Configuration sent
by the CM in the Registration Request as TLV type 5.80. The two-byte unsigned integer represents the frequency in
MHz. The value zero indicates the CM is currently not configured with a channel in the extended spectrum.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Downstream Upper Band Edge
Configuration section; [PHYv4.0] Upstream and Downstream Frequency Plan for FDD Operation

F.2.3.12.3 DiplexerUsUpperBandEdgeCfg
This attribute specifies the ending (highest) frequency for which the upstream band is currently configured in the
FDD cable modem. This corresponds to the Advanced Diplexer Upstream Upper Band Edge Configuration sent by
the CM in the Registration Request as TLV type 5.81. The two-byte unsigned integer represents the frequency in
MHz. The value zero indicates the CM is currently not configured with a channel in the extended spectrum.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Diplexer Upstream Upper Band Edge
Configuration section; [PHYv4.0] Upstream and Downstream Frequency Plan for FDD Operation

          CmFdxStatus
The CmFdxStatus object reports current FDX-capable CM status information.
The CmFdxStatus object is instantiated by FDX CMs and FDX-L CMs, irrespective of whether they are operating
on Extended Channels. An instance of this object is created when a CM registers and reports it is FDX-capable via
the AdvBandPlanCapability attribute of the CmCapabilities object.
                                   Table 141 - CmFdxStatus Object Attributes

      Attribute Name                 Type                 Access             Type Constraints             Units         Default
IfIndex                    InterfaceIndex            Key              Interface Index of the MAC
                                                                      interface
TransmissionGroupId        UnsignedByte              R/O
CurrentRbaSubbandDirSet    RbaSubbandDirSetType      R/O


                                  Table 142 - CmFdxStatus Object Associations

                                                      Near-End                   Far-End
  Associated Object Name               Type           Multiplicity              Multiplicity               Label
 CmCapabilities                 Association (with     1                  1                     fdxL or fdx capability
                                constraint)
 CmEcTrainingStatus             Directed              1                  0..*                  fdx capability
                                Composition (with
                                constraint) to
                                CmEcTrainingStatus



F.2.3.13.1 IfIndex
This key attribute denotes the MAC Domain interface index of the FDX-capable CM.


272                                                  CableLabs                                                     04/29/26
<!-- PDF_PAGE: 273 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


F.2.3.13.2 TransmissionGroupId
This attribute reports the Transmission Group ID associated with the FDX-capable CM Transmission Group
Assignment. A value of zero (0) indicates no active Transmission Group is assigned to the FDX-capable CM.
References: [MULPIv4.0] Common TLV Encodings annex, Transmission Group ID section

F.2.3.13.3 CurrentRbaSubbandDirSet
This attribute reports the RBA Sub-Band Direction Set that is currently assigned to an FDX-capable CM.
References: [MULPIv4.0] Resource Block Assignment (RBA) section

            CmEcTrainingStatus
The CmEcTrainingStatus object reports the EC training status of up to eight possible RBA Sub-Band Direction Sets
for which an FDX CM can concurrently maintain EC training. The CmEcTrainingStatus object is associated with a
single CmFdxStatus object. An FDX CM instantiates an instance of CmEcTrainingStatus with the following.
    •     An FDX CM instantiates a CmEcTrainingStatus instance for each RbaSubbandDirSet maintained by the
          FDX CM. An FDX CM does not instantiate a CmEcTrainingStatus instance for any RbaSubbandDirSet
          that is not maintained by the FDX CM.
    •     An FDX CM maintains an instance of CmEcTrainingStatus for the CM's current RbaSubbandDirSet.
    •     An FDX CM may maintain an instance of CmEcTrainingStatus for non-current RbaSubbandDirSets that
          were previously assigned to the FDX CM.
    •     An FDX CM maintains created instances for assigned unidirectional RbaSubbandDirSets (i.e., with
          EcTrainingStatus 'notRequired').
    •     An FDX CM removes all instances of CmEcTrainingStatus on FdxReset.
    •     An FDX CM removes all instances of CmEcTrainingStatus on CM reset. Instances of CmEcTrainingStatus
          object do not persist across FDX CM reboot.
    •     An FDX CM maintains CmEcTrainingStatus instances for an RbaSubbandDirSet when its CM
          Transmission Group Id is assigned to 0; e.g., during sounding.
An FDX-L CM does not implement the CmEcTrainingStatus object.
References: [MULPIv4.0] CM Echo Cancellation section
                               Table 143 - CmEcTrainingStatus Object Attributes

        Attribute Name              Type                Access         Type Constraints          Units        Default
  RbaSubbandDirSet         RbaSubbandDirSetType       Key
  EcTrainingStatus         Enum                       R/O          converged(1),
                                                                   notConverged(2),
                                                                   notApplicable(3)
  EcTrainingPhase          Enum                       R/O          initial(1),
                                                                   periodic(2),
                                                                   notApplicable(3)


                              Table 144 - CmEcTrainingStatus Object Associations

                                                                   Near-End            Far-End
        Associated Object Name                 Type                Multiplicity       Multiplicity         Label
    CmCapabilities                 Association (with constraint)   1                  1              fdx capability




04/29/26                                             CableLabs                                                         273
<!-- PDF_PAGE: 274 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


F.2.3.14.1 RbaSubbandDirSet
This key attribute reports an RBA Sub-Band Direction Set for an FDX CM.
References: [MULPIv4.0] Resource Block Assignment (RBA) section

F.2.3.14.2 EcTrainingStatus
This attribute reports the Echo Cancellation Training (ECT) status for an FDX CM of the RBA Sub-Band Direction
Set identified by the RbaSubbandDirSet key attribute.
'converged' – Echo Cancellation Training is converged.
'notConverged' –Echo Cancellation Training is not converged.
'notApplicable' – No Echo Cancellation Training required; i.e., for unidirectional RBA Direction Sets (e.g., 000 or
111).
References: [MULPIv4.0] Common TLV Encodings annex, EC Training Status section

F.2.3.14.3 EcTrainingPhase
This attribute reports the current phase (stage) of Echo Cancellation Training (ECT) status for an FDX CM on the
RBA Sub-Band Direction Set specified by the RbaSubbandDirSet attribute.
'initial' – Initial Echo Cancellation Training phase. This phase is distinguished by the absence of a training
periodicity TLV in the ECT-REQ message.
'periodic' – Periodic Echo Cancellation Training phase. This phase is distinguished by an ECT-REQ message
containing a training periodicity TLV.
'notApplicable' – No Echo Cancellation Training required; i.e., for unidirectional RBA Direction Sets (e.g., 000 or
111).
References: [MULPIv4.0] Common TLV Encodings annex, Foreground Training Periodicity section

           CmFddCapabilities
The CmFddCapabilities object defines capability attributes for the FDD CM.
                                 Table 145 - CmFddCapabilities Object Attributes

                                                                        Type
           Attribute Name                   Type          Access      Constraints       Multiplicity    Units      Default
 DiplexerUsUpperBandEdgeCapability      UnsignedShort    R/O                           0..*
 DiplexerDsLowerBandEdgeCapability      UnsignedShort    R/O                           0..*
 DiplexerDsUpperBandEdgeCapability      UnsignedShort    R/O                           0..*



F.2.3.15.1 DiplexerUsUpperBandEdgeCapability
This attribute specifies all of the advanced diplexer upstream upper band edge configurations supported by the FDD
CM. This corresponds to the Advanced Diplexer Upstream Upper Band Edge Options List modem capability TLV
5.84 sent by the FDD CM in the Registration Request. The list of UnsignedShort values represents the list of
supported frequencies specified in MHz and sent in the Registration Request.
References: [MULPIv4.0] Common TLV Encodings annex, Advanced Diplexer Upstream Upper Band Edge
Options List

F.2.3.15.2 DiplexerDsLowerBandEdgeCapability
This attribute specifies the advanced diplexer downstream lower band edge configurations supported by the FDD
CM. This corresponds to the Advanced Diplexer Downstream Lower Band Edge Options List modem capability



274                                                  CableLabs                                                  04/29/26
