# Annex F.2.3 - CM Operational Status Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 217-229
Classification: Normative (explicit)

<!-- PDF_PAGE: 217 -->
Cable Modem Operations Support System Interface Specification                 CM-SP-CM-OSSIv3.1-I27-250219


    •   Success
             'success' indicates that the CMTS has sent a ranging success in the ranging response.
    •   Continue
             'continue' indicates that the CMTS has sent a ranging continue in the ranging response.
    •   timeoutT4
             'timeoutT4' indicates that the T4 timer expired on the CM.

           Tlv8
This data type represents a single TLV encoding. This first octet represents the Type of the TLV. The second octet
represents an unsigned 8-bit Length of the subsequent Value part of the TLV. The remaining octets represent the
value. The Value could be an atomic value or a sequence of one or more sub-TLVs.
References: [MULPIv3.1] Common Radio Frequency Interface Encodings annex

F.2.3   CM Operational Status Objects
These objects report the CM configuration and operational status.




                              Figure 13 - CM Operational Status Information Model



02/19/25                                           CableLabs®                                                   217
<!-- PDF_PAGE: 218 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


            CmStatus
This object provides CM connectivity status information of the CM previously available in the SNMP table
docsIfCmStatusTable.
References: [RFC 4546]
                                         Table 92 - CmStatus Object Attributes

      Attribute Name               Type             Access              Type Constraints               Units      Default
IfIndex                      InterfaceIndex      Key             Interface Index of the MAC
                                                                 interface
Value                        CmRegState          R/O
Code                         HexBinary           R/O             SIZE( 0 | 5 | 6 )
Resets                       Counter32           R/O                                                resets
LostSyncs                    Counter32           R/O                                                messages
InvalidMaps                  Counter32           R/O                                                maps
InvalidUcds                  Counter32           R/O                                                messages
InvalidRangingRsps           Counter32           R/O                                                messages
InvalidRegRsps               Counter32           R/O                                                messages
T1Timeouts                   Counter32           R/O                                                timeouts
T2Timeouts                   Counter32           R/O                                                timeouts
EnergyMgt1x1OperStatus       Boolean             R/O
EmDlsOperStatus              Boolean             R/O



F.2.3.1.1     IfIndex
This attribute denotes the MAC Domain interface index of the CM.

F.2.3.1.2     Value
This attribute denotes the current CM connectivity state. For the case of IP acquisition related states, this attribute
reflects states for the current CM provisioning mode, not the other DHCP process associated with dual stack
operation.
References: [MULPIv3.1] Establishing IP Connectivity section

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




218                                                    CableLabs®                                               02/19/25
<!-- PDF_PAGE: 219 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


F.2.3.1.5   LostSyncs
This attribute denotes the number of times the CM lost synchronization with the downstream channel.
Discontinuities in the value of this counter can occur at re-initialization of the managed system, and at other times as
indicated by the value of ifCounterDiscontinuityTime for the CM MAC Domain interface.
References: [RFC 2863]

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
References: [MULPIv3.1] Energy Management Mode Indicator section




02/19/25                                            CableLabs®                                                      219
<!-- PDF_PAGE: 220 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


F.2.3.1.13 EmDlsOperStatus
This attribute indicates whether the CM is currently operating in Energy Management DLS Mode. If this attribute
returns true, the CM is operating in Energy Management DLS Mode.
References: [MULPIv3.1] Energy Management Mode Indicator section

            CmStatusUs
This object defines PHY and MAC information about the CM's SC-QAM upstream channels. This object provides
per-CM Upstream channel information previously available in the SNMP table docsIfCmStatusTable.
                                        Table 93 - CmStatusUs Object Attributes

 Attribute Name                  Type          Access               Type Constraints                   Units    Default
IfIndex                InterfaceIndex         Key          Interface Index of upstream interface
TxPower                TenthdBmV              R/O                                                  dBmV
T3Timeouts             Counter32              R/O                                                  timeouts
T4Timeouts             Counter32              R/O                                                  timeouts
RangingAborteds        Counter32              R/O                                                  attempts
ModulationType         DocsisUpstreamType     R/O
EqData                 DocsEqualizerData      R/O
T3Exceededs            Counter32              R/O                                                  timeouts
IsMuted                Boolean                R/O
RangingStatus          RangingState           R/O



F.2.3.2.1    IfIndex
This attribute denotes the interface index of the upstream interface to which this instance applies.

F.2.3.2.2    TxPower
This attribute denotes the operational CM transmit power for this SC-QAM upstream channel. In order for this
attribute to provide consistent information under all circumstances, a 3.1 CM will report the average total power for
the SC-QAM channel the same as was done for DOCSIS 3.0, regardless of whether it is operating with a 3.1 or a 3.0
CMTS. The value that is reported was referred to as Pr in [PHYv3.0].

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



220                                                  CableLabs®                                                02/19/25
<!-- PDF_PAGE: 221 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


References: [RFC 2863]

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
References: [MULPIv3.1] Media Access Control Specification section

F.2.3.2.10 RangingStatus
This attribute denotes ranging status of this upstream channel.
References: [MULPIv3.1] Media Access Control Specification section

            CmStatusOfdmaUs
This object defines PHY and MAC information about the CM's OFDMA upstream channels. This object provides
per-CM Upstream channel information previously available in the SNMP table docsIfCmStatusTable.
                                      Table 94 - CmStatusOfdmaUs Object Attributes

 Attribute Name                  Type           Access              Type Constraints                   Units   Default
IfIndex                InterfaceIndex          Key         Interface Index of upstream interface
T3Timeouts             Counter32               R/O                                                 timeouts
T4Timeouts             Counter32               R/O                                                 timeouts
RangingAborteds        Counter32               R/O                                                 attempts
T3Exceededs            Counter32               R/O                                                 timeouts
IsMuted                Boolean                 R/O
RangingStatus          RangingState            R/O



F.2.3.3.1    IfIndex
This attribute denotes the interface index of the upstream interface to which this instance applies.




02/19/25                                             CableLabs®                                                   221
<!-- PDF_PAGE: 222 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


F.2.3.3.2    T3Timeouts
This attribute denotes the number of times counter T3 expired in the CM for this upstream channel. Discontinuities
in the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by
the value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.3.3    T4Timeouts
This attribute denotes the number of times counter T4 expired in the CM for this upstream channel. Discontinuities
in the value of this counter can occur at re-initialization of the managed system, and at other times as indicated by
the value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.3.4    RangingAborteds
This attribute denotes the number of times the ranging process was aborted by the CMTS. Discontinuities in the
value of this counter can occur at re-initialization of the managed system, and at other times as indicated by the
value of ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.3.5    T3Exceededs
This attribute denotes the number of times for excessive T3 timeouts. Discontinuities in the value of this counter can
occur at re-initialization of the managed system, and at other times as indicated by the value of
ifCounterDiscontinuityTime for the associated upstream channel.
References: [RFC 2863]

F.2.3.3.6    IsMuted
This attribute denotes whether the upstream channel is muted.
References: [MULPIv3.1] Media Access Control Specification section

F.2.3.3.7    RangingStatus
This attribute denotes ranging status of this upstream channel.
References: [MULPIv3.1] Media Access Control Specification section

            CmCapabilities
This object defines attributes of the CM capabilities.
                                      Table 95 - CmCapabilities Object Attributes

      Attribute Name                  Type          Access            Type Constraints             Units      Default
Req                            Tlv8                R/O
Rsp                            Tlv8                R/O
DocsisBaseCapability           DocsisVersion       R/O



F.2.3.4.1    Req
This attribute contains the TLV encoding for TLV-5 sent in a REG-REQ. The first byte of this encoding is expected
to be '05'H.
References: [MULPIv3.1] Modem Capabilities Encoding section in the Common Radio Frequency Interface
Encodings annex



222                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 223 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv3.1-I27-250219


F.2.3.4.2     Rsp
This attribute contains the TLV encoding for TLV-5 (see the Modem Capabilities Encoding section in Common
Radio Frequency Interface Encodings Annex of [MULPIv3.1]) received in a REG-RSP. The first byte of this
encoding is expected to be '05'H.
References: [MULPIv3.1] Modem Capabilities Encoding section in the Common Radio Frequency Interface
Encodings annex

F.2.3.4.3     DocsisBaseCapability
This attribute indicates the DOCSIS capability of the CM. CMs report their supported DOCSIS version. This
attribute replaces docsIfDocsisBaseCapability defined in [RFC 4546].

             CmDpvStats
This object represents the DOCSIS Path Verify Statistics collected in the cable modem device. The CMTS controls
the logging of DPV statistics in the cable modem. Therefore the context and nature of the measurements are
governed by the CMTS and not self-descriptive when read from the CM.
                                         Table 96 - CmDpvStats Object Attributes

    Attribute Name          Type            Access               Type Constraints                  Units       Default
   IfIndex              InterfaceIndex    Key          Interface index of downstream interface
   GrpId                UnsignedByte      Key          1..2
   LastMeasLatency      UnsignedInt       R/O                                                    nanoseconds
   LastMeasTime         DateTime          R/O
   MinLatency           UnsignedInt       R/O                                                    nanoseconds
   MaxLatency           UnsignedInt       R/O                                                    nanoseconds
   AvgLatency           UnsignedInt       R/O                                                    nanoseconds
   NumMeas              UnsignedInt       R/O                                                    nanoseconds
   LastClearTime        DateTime          R/O



F.2.3.5.1     ifIndex
This key represents the interface Index of the Downstream Interface where the measurements are taken.

F.2.3.5.2     GrpId
This key represents the DPV Group ID. The CM reports two instance of DPV statistics per downstream normally
referred as Statistical Group 1 and Statistical Group 2.

F.2.3.5.3     LastMeasLatency
This attribute represents the last latency measurement for this statistical group.

F.2.3.5.4     LastMeasTime
This attribute represents the last measurement time of the last latency measurement for this statistical group. This
attribute reports the epoch time value when no measurements are being reported or after the statistics were cleared.

F.2.3.5.5     MinLatency
This attribute represents the minimum latency measurement for this statistical group since the last time statistics
were cleared.




02/19/25                                              CableLabs®                                                      223
<!-- PDF_PAGE: 224 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


F.2.3.5.6       MaxLatency
This attribute represents the maximum latency measurement for this statistical group since the last time statistics
were cleared.

F.2.3.5.7       AvgLatency
This attribute represents the average latency measurement for this statistical group since the last time statistics were
cleared. The averaging mechanism is controlled by the CMTS.
References: [MULPIv3.1] DPV Math section

F.2.3.5.8       NumMeas
This attribute represents the number of latency measurements made for this statistical group since the last time
statistics were cleared.

F.2.3.5.9       LastClearTime
This attribute represents the last time statistics were cleared for this statistical group, otherwise this attribute reports
the epoch time value.

              CmEventCtrl
This object represents the control mechanism to enable the dispatching of events based on the event Id. The
following rules define the event control behavior:
      •      The CmEventCtrl object has no instances or contains an instance with Event ID 0.
             All events matching the Local Log settings of docsDevEvReporting are sent to local log ONLY.
      •      Additionally, if The CmEventCtrl object contains configured instances with non-zero Event IDs.
             Events matching the Event Ids configured in the object are sent according to the settings of the
             docsDevEvReporting object; i.e., Traps, Syslog, etc.
The CM MUST NOT persist instances of CmEventCtrl across reinitializations.
                                           Table 97 - CmEventCtrl Object Attributes

          Attribute Name        Type           Access         Type Constraints              Units          Default
          EventId            UnsignedInt      Key



F.2.3.6.1       EventId
This key represents the Event ID of the event being enabled for delivery to a dispatch mechanism (e.g., syslog).
References: Annex C

              CmEventNotif
This object represents the abstract definition of an event object for the CM. The realization of the event object
depend on the management protocol that carries the event. For example, the object event realization as a SNMP
notification is defined as the docsIf3CmEventNotif defined in [DOCS-IF3-MIB].

              CmEm1x1Stats
This object defines Energy Management 1x1 mode statistics on the CM to provide insight into configuration of
appropriate EM 1x1 Mode Activity Detection thresholds and/or to get feedback on how/if the current thresholds are
working well or are causing user experience issues. These statistics are only applicable/valid when the Energy
Management 1x1 mode is enabled in the CM.




224                                                     CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 225 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


                                    Table 98 - CmEm1x1Stats Object Attributes

               Attribute Name                     Type         Access       Type Constraints       Units      Default
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
This object defines the DLS Energy Management mode statistics on the CM to provide insight into configuration of
appropriate DLS EM Mode Activity Detection thresholds and/or to get feedback on how/if the current thresholds are
working well or are causing user experience issues. These statistics are only applicable/valid when the DLS Energy
Management mode is enabled in the CM.




02/19/25                                            CableLabs®                                                     225
<!-- PDF_PAGE: 226 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


                                    Table 99 - CmEmDlsStats Object Attributes

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



226                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 227 -->
Cable Modem Operations Support System Interface Specification                           CM-SP-CM-OSSIv3.1-I27-250219


           CmEmDlsStatus
This object defines the DLS Energy Management mode status on the CM to provide insight into the current
configuration of DLS Mode. This information is only applicable/valid when the DLS Energy Management mode is
enabled on the CM.
                                    Table 100 - CmEmDlsStatus Object Attributes

     Attribute Name                  Type                Access      Type Constraints             Units              Default
  AssignedEmIds              HexBinary                   R/O         SIZE (2 | 4 | 6)
  DlsReceiveTimer            PlcFrameInterval(1          R/O         0..2                    PlcFrame          0
                             byte)
  DlsMaxSleepLatency         UnsignedByte                R/O         1..255                  msec              100
  DlsMaxSleepBytes           UnsignedShort               R/O         1..65535                bytes             1Kbytes



F.2.3.10.1 AssignedEmIds
This attribute reports the set of CMTS-assigned EM-IDs for this cable modem. This attribute is encoded as an array
16-bit binary values with up to 3 elements. The broadcast EM-ID is not included in the list. This is generally
displayed as a comma-delimited list of EM-IDS such as: DF13,ABAB,0002.
References: [MULPIv3.1] DOCSIS Light Sleep Feature section

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
                                  Table 101 - CmSystemCfgState Object Attributes

           Attribute Name                         Type          Access          Type Constraints          Units          Default
 AqmUsEnable                             Boolean               R/O
 DefaultUsTargetBuffer                   UnsignedShort         R/O                                   msec
 EnergyMgtFeatureEnabled                 EnumBits              R/O              em1x1Feature(0)
                                                                                dls(1)
 EnergyMgtCyclePeriod                    UnsignedShort         R/O                                   seconds           900
 DiplexerCapability                      UnsignedInt           R/O
 DiplexerCfgBandEdge                     UnsignedInt           R/O
 DiplexerDsLowerCapability               UnsignedInt           R/O
 DiplexerCfgDsLowerBandEdge              UnsignedInt           R/O
 DiplexerDsUpperCapability               UnsignedInt           R/O
 DiplexerCfgDsUpperBandEdge              UnsignedInt           R/O




02/19/25                                                 CableLabs®                                                            227
<!-- PDF_PAGE: 228 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


              Attribute Name                 Type           Access      Type Constraints         Units        Default
 LgVqEwmaAlpha                          UnsignedByte       R/W R/O                                           7
 VqInterval                             UnsignedShort      R/W R/O      300 | 400 | 500 |     microseconds   500
                                                                        600 | 700 | 800 |
                                                                        900 | 1000



F.2.3.11.1 AqmUsEnable
If this attribute is set to 'false', the CM disables Active Queue Management (AQM) on all upstream service flows.
References: [DOCS-QOS3-MIB] docsQosCmSystemCfgStateAqmUsEnable

F.2.3.11.2 DefaultUsTargetBuffer
This attribute specifies the default upstream service flow target buffer size, in milliseconds, when not specified
otherwise in service flow TLV encodings.
References: [MULPIv3.1] Default Upstream Target Buffer Configuration Annex; [DOCS-QOS3-MIB]
docsQosCmSystemCfgStateDefaultUsTargetBuffer

F.2.3.11.3 EnergyMgtFeatureEnabled
This attribute indicates which energy savings features have been enabled in the Cable Modem. The CM enables use
of Energy Management Features only if both the Energy Management Feature Control TLV and Energy
Management Modem Capability Response from the CMTS indicate that the feature is enabled. If bit 0 is set, the
Energy Management 1x1 Mode feature is enabled. If bit 1 is set, the DOCSIS Light Sleep Mode feature is enabled.
References: [MULPIv3.1] Energy Management Feature Control section; [DOCS-IF3-MIB]
docsIf3CmEnergyMgtCfgFeatureEnabled

F.2.3.11.4 EnergyMgtCyclePeriod
This attribute specifies a minimum time period (in seconds) that must elapse between EM-REQ transactions in
certain situations:
      •   This attribute sets the minimum cycle time that a CM will use for sending requests to enter an Energy
          Management Mode. The CM will not request to enter an Energy Management Mode while this amount of
          time has yet to elapse since the last time the CM requested an Energy Management Mode and received a
          response indicating (0) OK or (1) Reject Temporary (with no Hold-off Timer value provided).
      •   In the case that the CM fails to receive an EM-RSP message after the maximum number of retries, this
          attribute sets the minimum amount of time to elapse before the CM can attempt another EM-REQ
          transaction.
References: [MULPIv3.1] Energy Management Cycle Period section; [DOCS-IF3-MIB]
docsIf3CmEnergyMgtCfgCyclePeriod

F.2.3.11.5 DiplexerCapability
This attribute specifies the upstream diplexer upper band edge configurations supported by the CM device. This
corresponds to the Diplexer Upstream Upper Band Edge modem capability sent by the CM in the Registration
Request. The Integer value represents the value sent in the Registration Request as a bit mask.
Example:
CM supports Switchable Upstream Diplexer configurations as follows:
Bit #0: Upstream Frequency Range up to 42 MHz
Bit #2: Upstream Frequency Range up to 85 MHz

The Integer value set for the DiplexerCapability in this case would equal 5.



228                                                 CableLabs®                                                02/19/25
<!-- PDF_PAGE: 229 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


References: [MULPIv3.1] Diplexer Upstream Upper Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerCapability

F.2.3.11.6 DiplexerCfgBandEdge
This attribute specifies the current configured frequency of the upstream upper band edge of the diplexer in the CM
device. This value is expressed in MHz and corresponds to one of the Diplexer Upper Band Edge capabilities
reported by the CM in its Registration Request.
References: [MULPIv3.1] Diplexer Upstream Upper Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerCfgBandEdge

F.2.3.11.7 DiplexerDsLowerCapability
This attribute specifies the downstream diplexer lower band edge configurations supported by the CM device. This
corresponds to the Diplexer Downstream Lower Band Edge modem capability sent by the CM in the Registration
Request. The Integer value represents the value sent in the Registration Request as a bit mask.
Example:
CM supports Switchable Downstream Diplexer Lower Band Edge configurations as follows:
Bit #0: Downstream Frequency Range starting from 108 MHz
Bit #1: Downstream Frequency Range starting from 258 MHz

The Integer value set for the DiplexerDsLowerCapability in this case would be 3.
References: [MULPIv3.1] Diplexer Downstream Lower Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerDsLowerCapability

F.2.3.11.8 DiplexerCfgDsLowerBandEdge
This attribute specifies the current configured frequency of the downstream lower band edge of the diplexer in the
CM device. This value is expressed in MHz and corresponds to one of the Downstream Lower Band Edge
capabilities reported by the CM in its Registration Request.
References: [MULPIv3.1] Downstream Lower Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerCfgDsLowerBandEdge

F.2.3.11.9 DiplexerDsUpperCapability
This attribute specifies the downstream diplexer upper band edge configurations supported by the CM device. This
corresponds to the Diplexer Downstream Upper Band Edge modem capability sent by the CM in the Registration
Request. The Integer value represents the value sent in the Registration Request as a bit mask.
Example:
CM supports only a single configuration for the DiplexerDsUpperCapability
Bit #0: Downstream Frequency Range up to 1218 MHz

The Integer values set for the DiplexerDsUpperCapability in this case would be 1.
References: [MULPIv3.1] Diplexer Downstream Upper Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerDsUpperCapability

F.2.3.11.10 DiplexerCfgDsUpperBandEdge
This attribute specifies the current configured frequency of the downstream upper band edge of the diplexer in the
CM device. This value is expressed in MHz and corresponds to one of the Downstream Upper Band Edge
capabilities reported by the CM in its Registration Request.
References: [MULPIv3.1] Downstream Upper Band Edge section; [DOCS-IF31-MIB]
docsIf31CmSystemCfgStateDiplexerCfgDsUpperBandEdge



02/19/25                                           CableLabs®                                                   229
