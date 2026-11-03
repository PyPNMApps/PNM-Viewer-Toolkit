# Annex C - Format and Content for Event, Syslog, and SNMP Notification

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 127-149
Classification: Normative (explicit)

<!-- PDF_PAGE: 127 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


Annex C           Format and Content for Event, SYSLOG, and SNMP
                  Notification (Normative)
The CM MUST format the CM MAC Address field <CM-MAC> of the Event Message text, including such
instances of docsDevEvText, using lowercase letters.
The CM MUST format the CMTS MAC Address field <CMTS-MAC> of the Event Message text, including such
instances of docsDevEvText, using lower case letters.
The CM MAY append additional vendor-specific text to the end of the event text reported in the docsDevEvText
object and the syslog text field.
Table 49 in this annex summarizes the format and content for event, syslog, and SNMP notifications required for a
DOCSIS 3.1-compliant CM.
Each row specifies a possible event that may appear in the CM. These events are to be reported by a CM through
local event logging, and may be accompanied by syslog or SNMP notification.
The "Process" and "Sub-Process" columns indicate in which stage the event happens. The "CM Priority" column
indicates the priority the event is assigned in the CM. The priority is the same as is reported in the docsDevEvLevel
object in the cable device MIB [RFC 4639] and in the LEVEL field of the syslog.
The "Event Message" column specifies the event text, which is reported in the docsDevEvText object of the cable
device MIB and the text field of the syslog. The "Message Notes And Details" column provides additional
information about the event text in the "Event Message" column. Some of the text fields include variable
information. The variables are explained in the "Message Notes And Details" column. For events where the "Event
Message" or "Message Notes and Details" column includes either <P1> or <P2>, there is a single space between the
value as defined by the <P1> or <P2> and the preceding text.
Example SNMP Notification and Syslog message "Event Message" text string for Event ID 69020900:
SNMP CVC Validation Failure SNMP Manager: 10.50.1.11;CM-MAC=00:22:ce:03:f4:da;CMTS-
MAC=00:15:20:00:25:ab;CM-QOS=1.1;CM-VER=3.0;
This specification defines the following keywords as part of the "Event Message" column:
"<TAGS>" (without the quotes) corresponds to:
For the CM (without the quotes):                      ";<CM-MAC>;<CMTS-MAC>;<CM-QOS>;<CM-VER>;"

Where:
<CM-MAC>:    CM MAC Address;
Format*: "CM-MAC=xx:xx:xx:xx:xx:xx"
<CMTS-MAC>: CMTS MAC Address;
Format*: "CMTS-MAC=xx:xx:xx:xx:xx:xx"
<CM-QOS>:    CM DOCSIS QOS Version;
Format*: "CM-QOS=1.0" or "CM-QOS=1.1"
<CM-VER>:    CM DOCSIS Version;
Format*: "CM-VER=1.1" or "CM-VER=2.0" or "CM-VER=3.0" or "CM-VER=3.1"
(*) without the quotes

Example SNMP Notification and Syslog message "Event Message" text string for Event ID 69010100:
SW Download INIT - Via NMS SW file: junk.bin - SW server: 10.50.1.11;CM-
MAC=00:22:ce:03:f4:da;CMTS-MAC=00:15:20:00:25:ab;CM-QOS=1.1;CM-VER=3.0;

The "Error Code Set" column specifies the error code. The "Event ID" column indicates a unique identification
number for the event, which is assigned to the docsDevEvId object in the cable device MIB and the <eventId> field
of the syslog. The "Notification Name" column specifies the SNMP notification, which notifies this event to an
SNMP notification receiver.
The syslog format, as well as the rules to uniquely generate an event ID from the error code, are described in Section
8.1.2.3 of this specification.



02/19/25                                           CableLabs®                                                     127
<!-- PDF_PAGE: 128 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                      Data-Over-Cable Service Interface Specifications


                                                          Table 49 - Event Format and Content

      Process      Sub-Process        CM                        Event                   Message Notes      Error         Event      Notification
                                    Priority                   Message                    and Detail      Code Set        ID          Name
  Authentication and Encryption
                                                  <Reserved>                                                         0
  BPKM          AUTH-FSM          Warning         Auth Reject - No                                       B301.2      66030102    CM:
                                                  Information<TAGS>                                                              docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Reject - Unauthorized                             B301.3      66030103    CM:
                                                  CM<TAGS>                                                                       docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Reject - Unauthorized                             B301.4      66030104    CM:
                                                  SAID<TAGS>                                                                     docsIf3CmEventNotif
  BPKM          AUTH-FSM          Error           Auth Reject - Permanent                                B301.8      66030108    CM:
                                                  Authorization Failure<TAGS>                                                    docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Reject - Time of Day not                          B301.9      66030109    CM:
                                                  acquired<TAGS>                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Informational   Auth Reject - EAE disabled<TAGS>                       B301.10     66030110
  BPKM          AUTH-FSM          Alert           CM Certificate Error<TAGS>                             B301.11     66030111    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Invalid - No                                      B302.2      66030202    CM:
                                                  Information<TAGS>                                                              docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Invalid - Unauthorized                            B302.3      66030203    CM:
                                                  CM<TAGS>                                                                       docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Invalid - Unsolicited<TAGS>                       B302.5      66030205    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Invalid - Invalid Key Sequence                    B302.6      66030206    CM:
                                                  Number<TAGS>                                                                   docsIf3CmEventNotif
  BPKM          AUTH-FSM          Warning         Auth Invalid - Message (Key                            B302.7      66030207    CM:
                                                  Request) Authentication                                                        docsIf3CmEventNotif
                                                  Failure<TAGS>
  BPKM          AUTH-FSM          Warning         Unsupported Crypto Suite<TAGS>                         B303.0      66030300    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Informational   Authorized<TAGS>                                       B401.0      66040100    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Informational   Auth Pend<TAGS>                                        B402.0      66040200    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Informational   Auth Comp<TAGS>                                        B403.0      66040300    CM:
                                                                                                                                 docsIf3CmEventNotif
  BPKM          AUTH-FSM          Informational   Stop<TAGS>                                             B404.0      66040400    CM:
                                                                                                                                 docsIf3CmEventNotif




128                                                                    CableLabs®                                                           02/19/25
<!-- PDF_PAGE: 129 -->
Cable Modem Operations Support System Interface Specification                                                             CM-SP-CM-OSSIv3.1-I27-250219



   Process         Sub-Process        CM                       Event                      Message Notes          Error         Event        Notification
                                    Priority                  Message                       and Detail          Code Set        ID            Name
  BPKM          TEK-FSM           Warning         Key Reject - No Information<TAGS>                             B501.2        66050102   CM:
                                                                                                                                         docsIf3CmEventNotif
  BPKM          TEK-FSM           Warning         Key Reject - Unauthorized                                     B501.3        66050103   CM:
                                                  SAID<TAGS>                                                                             docsIf3CmEventNotif
  BPKM          TEK-FSM           Warning         TEK Invalid - No                                              B502.3        66050203   CM:
                                                  Information<TAGS>                                                                      docsIf3CmEventNotif
  BPKM          TEK-FSM           Warning         TEK Invalid - Invalid Key Sequence                            B502.6        66050206   CM:
                                                  Number<TAGS>                                                                           docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Informational   SA Map State Machine                                          B601.0        66060100   CM:
                                                  Started<TAGS>                                                                          docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Warning         Unsupported Crypto Suite<TAGS>                                B602.0        66060200   CM:
                                                                                                                                         docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Error           Map Request Retry                                             B603.0        66060300   CM:
                                                  Timeout<TAGS>                                                                          docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Informational   Unmap<TAGS>                                                   B604.0        66060400   CM:
                                                                                                                                         docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Informational   Map Reject - Downstream Traffic                               B605.10       66060510   CM:
                                                  Flow Not Mapped to BPI+ SAID                                                           docsIf3CmEventNotif
                                                  (EC=8)<TAGS>
  Dynamic SA    SA MAP-FSM        Warning         Map Reject - Not Authorized for                               B605.9        66060509   CM:
                                                  Requested Downstream Traffic Flow                                                      docsIf3CmEventNotif
                                                  (EC=7)<TAGS>
  Dynamic SA    SA MAP-FSM        Warning         Mapped to Existing SAID<TAGS>                                 B606.0        66060600   CM:
                                                                                                                                         docsIf3CmEventNotif
  Dynamic SA    SA MAP-FSM        Warning         Mapped to New SAID<TAGS>                                      B607.0        66060700   CM:
                                                                                                                                         docsIf3CmEventNotif
  DBC and DCC
  DBC           DBC Request       Warning         CMTS Bad DBC - confirmation code     P1=<Confirmation Code>   C501.0        67050100
                                                  <P1>: <P2><TAGS>                     P2=<Confirmation> See
                                                                                       Annex C.4 Confirmation
                                                                                       Code
  DBC           DBC Request       Warning         DBC-REQ denied - confirmation        P1=<Confirmation Code>   C502.0        67050200
                                                  code <P1>: <P2><TAGS>                P2=<Confirmation> See
                                                                                       Annex C.4 Confirmation
                                                                                       Code
  DBC           DBC Request       Warning         DBC-REQ Mismatch Between                                      C503.0        67050300
                                                  Calculated Value for P1.6hi
                                                  Compared to CCAP Provided
                                                  Value<TAGS>




02/19/25                                                              CableLabs®                                                                           129
<!-- PDF_PAGE: 130 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                                  Data-Over-Cable Service Interface Specifications



      Process      Sub-Process            CM                       Event                       Message Notes             Error      Event        Notification
                                        Priority                  Message                        and Detail             Code Set     ID            Name
  DBC           DBC Acknowledgement   Error           DBC-ACK not received<TAGS>                                        C701.0     67070100
  DBC           DBC Acknowledgement   Notice          Bad CMTS DBC-ACK:                    P1="unspecified reason" |    C702.0     67070200
                                                      <P1><TAGS>                           "unknown transaction ID" |
                                                                                           "authentication failure" |
                                                                                           "msg syntax error"
  DCC           DCC Request           Error           DCC rejected already there<TAGS>                                  C201.0     67020100   CM:
                                                                                                                                              docsIf3CmEventNotif
  DCC           DCC Request           Informational   DCC depart old<TAGS>                                              C202.0     67020200   CM:
                                                                                                                                              docsIf3CmEventNotif
  DCC           DCC Request           Informational   DCC arrive new<TAGS>                                              C203.0     67020300   CM:
                                                                                                                                              docsIf3CmEventNotif
  DCC           DCC Request           Critical        DCC aborted unable to acquire new                                 C204.0     67020400
                                                      downstream channel<TAGS>
  DCC           DCC Request           Critical        DCC aborted no UCD for new                                        C205.0     67020500
                                                      upstream channel<TAGS>
  DCC           DCC Request           Critical        DCC aborted unable to                                             C206.0     67020600
                                                      communicate on new upstream
                                                      channel<TAGS>
  DCC           DCC Request           Error           DCC rejected unspecified                                          C207.0     67020700   CM:
                                                      reason<TAGS>                                                                            docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected permanent - DCC not                                  C208.0     67020800   CM:
                                                      supported<TAGS>                                                                         docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected service flow not                                     C209.0     67020900   CM:
                                                      found<TAGS>                                                                             docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected required parameter                                   C210.0     67021000   CM:
                                                      not present<TAGS>                                                                       docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected authentication                                       C211.0     67021100   CM:
                                                      failure<TAGS>                                                                           docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected multiple                                             C212.0     67021200   CM:
                                                      errors<TAGS>                                                                            docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected, duplicate SF                                        C215.0     67021500   CM:
                                                      reference-ID or index in                                                                docsIf3CmEventNotif
                                                      message<TAGS>
  DCC           DCC Request           Error           DCC rejected parameter invalid for                                C216.0     67021600   CM:
                                                      context<TAGS>                                                                           docsIf3CmEventNotif
  DCC           DCC Request           Error           DCC rejected message syntax                                       C217.0     67021700   CM:
                                                      error<TAGS>                                                                             docsIf3CmEventNotif




130                                                                        CableLabs®                                                                    02/19/25
<!-- PDF_PAGE: 131 -->
Cable Modem Operations Support System Interface Specification                                                              CM-SP-CM-OSSIv3.1-I27-250219



    Process      Sub-Process            CM                     Event                       Message Notes           Error        Event        Notification
                                      Priority                Message                        and Detail           Code Set       ID            Name
  DCC         DCC Request           Error        DCC rejected message too                                         C218.0       67021800   CM:
                                                 big<TAGS>                                                                                docsIf3CmEventNotif
  DCC         DCC Request           Error        DCC rejected 2.0 mode                                            C219.0       67021900   CM:
                                                 disabled<TAGS>                                                                           docsIf3CmEventNotif
  DCC         DCC Acknowledgement   Error        DCC-ACK not received<TAGS>                                       C401.0       67040100   CM:
                                                                                                                                          docsIf3CmEventNotif
  DCC         DCC Acknowledgement   Error        DCC-ACK rejected unspecified                                     C402.0       67040200   CM:
                                                 reason<TAGS>                                                                             docsIf3CmEventNotif
  DCC         DCC Acknowledgement   Error        DCC-ACK rejected unknown                                         C403.0       67040300   CM:
                                                 transaction ID<TAGS>                                                                     docsIf3CmEventNotif
  DCC         DCC Acknowledgement   Error        DCC-ACK rejected authentication                                  C404.0       67040400   CM:
                                                 failure<TAGS>                                                                            docsIf3CmEventNotif
  DCC         DCC Acknowledgement   Error        DCC-ACK rejected message syntax                                  C405.0       67040500   CM:
                                                 error<TAGS>                                                                              docsIf3CmEventNotif
  DHCP, TOD and TFTP
  DHCP                              Error        DHCP RENEW sent - No response         P1=IPv4 or IPv6            D101.0       68010100
                                                 for <P1><TAGS>
  DHCP                              Error        DHCP REBIND sent - No response        P1=IPv4 or IPv6            D102.0       68010200
                                                 for <P1><TAGS>
  DHCP                              Error        DHCP RENEW WARNING - Field            P1=v4                      D103.0       68010300
                                                 invalid in response <P1>
                                                 option<TAGS>
  DHCP                              Critical     DHCP RENEW FAILED - Critical                                     D103.1       68010301
                                                 field invalid in response
  DHCP                              Error        DHCP REBIND WARNING - Field                                      D104.0       68010400
                                                 invalid in response <TAGS>
  DHCP                              Critical     DHCP REBIND FAILED - Critical                                    D104.1       68010401
                                                 field invalid in response
  DHCP                              Notice       DHCP Reconfigure                                                 D105.0       68010500
                                                 received<TAGS>
  DHCP                              Notice       DHCP Renew - lease parameters         P1 = list of params that   D106.0       68010600
                                                 <P1> modified<TAGS>                   changed at renew
  DHCP                              Error        Primary lease failed, IPv4 fallback                              D107.0       68010700
                                                 initiated<TAGS>
  Init        DHCP                  Critical     DHCP FAILED - Discover sent, no                                  D01.0        68000100
                                                 offer received<TAGS>
  Init        DHCP                  Critical     DHCP FAILED - Request sent, No                                   D02.0        68000200
                                                 response<TAGS>




02/19/25                                                               CableLabs®                                                                           131
<!-- PDF_PAGE: 132 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                             Data-Over-Cable Service Interface Specifications



      Process      Sub-Process       CM                     Event                        Message Notes              Error      Event     Notification
                                   Priority                Message                         and Detail              Code Set     ID         Name
  Init          DHCP             Warning      DHCP WARNING - Non-critical field                                    D03.0      68000300
                                              invalid in response <TAGS>
  Init          DHCP             Critical     DHCP FAILED - Critical field invalid                                 D03.1      68000301
                                              in response <TAGS>
  Init          DHCP             Critical     DHCP failed - RS sent, no RA                                         D12.0      68001200
                                              received<TAGS>
  Init          DHCP             Critical     DHCP Failed - Invalid RA<TAGS>                                       D12.1      68001201
  Init          DHCP             Critical     DHCP failed - DHCP Solicit sent, No                                  D12.2      68001202
                                              DHCP Advertise received<TAGS>
  Init          DHCP             Critical     DHCP failed - DHCP Request sent,                                     D12.3      68001203
                                              No DHCP REPLY received<TAGS>
  Init          DHCP             Error        Primary address acquired,                                            D12.4      68001204
                                              secondary failed<TAGS>
  Init          DHCP             Error        Primary address failed, secondary                                    D12.5      68001205
                                              active<TAGS>
  Init          IPv6 Address     Critical     Link-Local address failed                                            D13.1      68001301
                Acquisition                   DAD<TAGS>
  Init          IPv6 Address     Critical     DHCP lease address failed                                            D13.2      68001302
                Acquisition                   DAD<TAGS>
  Init          TOD              Warning      ToD request sent - No Response                                       D04.1      68000401
                                              received<TAGS>
  Init          TOD              Warning      ToD Response received - Invalid                                      D04.2      68000402
                                              data format<TAGS>
  Init          TFTP             Critical     TFTP failed - Request sent - No                                      D05.0      68000500
                                              Response<TAGS>
  Init          TFTP             Critical     TFTP failed - configuration file NOT   For SYSLOG only:              D06.0      68000600
                                              FOUND<TAGS>                            append: File name = <P1>
                                                                                     P1 = requested file name
  Init          TFTP             Critical     TFTP Failed - OUT OF ORDER                                           D07.0      68000700
                                              packets<TAGS>
  Init          TFTP             Critical     TFTP file complete - but failed        For SYSLOG only:              D08.0      68000800
                                              Message Integrity check                append: File name = <P1>
                                              MIC<TAGS>                              P1 = file name of TFTP file
  Init          TFTP             Critical     TFTP file complete - but missing                                     D09.0      68000900
                                              mandatory TLV<TAGS>
  Init          TFTP             Critical     TFTP Failed - file too big<TAGS>                                     D10.0      68001000




132                                                                 CableLabs®                                                                  02/19/25
<!-- PDF_PAGE: 133 -->
Cable Modem Operations Support System Interface Specification                                                               CM-SP-CM-OSSIv3.1-I27-250219



    Process       Sub-Process         CM                     Event                       Message Notes              Error        Event        Notification
                                    Priority                Message                        and Detail              Code Set       ID            Name
  Init         TFTP               Critical     TFTP Request Retries exceeded,        For SYSLOG only:              D11.1        68001101
                                               CM unable to register                 append: File name = <P1>
                                                                                     P1 = file name of TFTP file
  Init         TFTP               Critical     Config File Rejected due to Invalid   P1 = First TLV encoding       D11.2        68001102   CM:
                                               or Unexpected TLV <P1>                (e.g., 53, 54, 60, etc.)                              docsIf3CmEventNotif
                                               encoding<TAGS>                        which caused config file
                                                                                     rejection
  TOD                             Error        ToD request sent - No Response                                      D04.3        68000403   CM:
                                               received<TAGS>                                                                              docsIf3CmEventNotif
  TOD                             Error        ToD Response received - Invalid                                     D04.4        68000404   CM:
                                               data format<TAGS>                                                                           docsIf3CmEventNotif
  Secure Software Download
  SW Upgrade   SW UPGRADE INIT    Notice       SW Download INIT - Via NMS            Other than Local Log,         E101.0       69010100   CM:
                                                                                     append:                                               docsIf3CmEventNotif
                                                                                     SW file: <P1> - SW server:
                                                                                     < P2><TAGS>

                                                                                     P1 = SW file name
                                                                                     P2 = SW Download server
                                                                                     IP address
  SW Upgrade   SW UPGRADE INIT    Notice       SW Download INIT - Via Config file    Other than Local Log,         E102.0       69010200   CM:
                                               <P1>                                  append:                                               docsIf3CmEventNotif
                                                                                     SW file: <P2> - SW server:
                                                                                     < P3><TAGS>

                                                                                     P1 = CM config file name
                                                                                     P2 = SW file name
                                                                                     P3 = SW Download server
                                                                                     IP address
  SW Upgrade   SW UPGRADE         Error        SW Upgrade Failed during download     Other than Local Log,         E103.0       69010300   CM:
               GENERAL FAILURE                 - Max retry exceed (3)                append:                                               docsIf3CmEventNotif
                                                                                     SW file: <P1> - SW server:
                                                                                     < P2><TAGS>

                                                                                     P1 = SW file name
                                                                                     P2 = SW Download server
                                                                                     IP address




02/19/25                                                             CableLabs®                                                                              133
<!-- PDF_PAGE: 134 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                           Data-Over-Cable Service Interface Specifications



      Process      Sub-Process        CM                    Event                       Message Notes             Error      Event        Notification
                                    Priority               Message                        and Detail             Code Set     ID            Name
  SW Upgrade    SW UPGRADE        Error        SW Upgrade Failed Before             Other than Local Log,        E104.0     69010400   CM:
                GENERAL FAILURE                Download - Server not Present        append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        SW upgrade Failed before download    Other than Local Log,        E105.0     69010500   CM:
                GENERAL FAILURE                - File not Present                   append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        SW upgrade Failed before download    Other than Local Log,        E106.0     69010600   CM:
                GENERAL FAILURE                -TFTP Max Retry Exceeded             append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        SW upgrade Failed after download -   Other than Local Log,        E107.0     69010700   CM:
                GENERAL FAILURE                Incompatible SW file                 append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        SW upgrade Failed after download -   Other than Local Log,        E108.0     69010800   CM:
                GENERAL FAILURE                SW File corruption                   append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address




134                                                                CableLabs®                                                                     02/19/25
<!-- PDF_PAGE: 135 -->
Cable Modem Operations Support System Interface Specification                                                             CM-SP-CM-OSSIv3.1-I27-250219



   Process        Sub-Process         CM                    Event                       Message Notes             Error        Event        Notification
                                    Priority               Message                        and Detail             Code Set       ID            Name
  SW Upgrade   SW UPGRADE         Error        Disruption during SW download -      Other than Local Log,        E109.0       69010900   CM:
               GENERAL FAILURE                 Power Failure                        append:                                              docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade   SW UPGRADE         Error        Disruption during SW download - RF   Other than Local Log,        E110.0       69011000   CM:
               GENERAL FAILURE                 removed                              append:                                              docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade   SW UPGRADE         Notice       SW download Successful - Via NMS     Other than Local Log,        E111.0       69011100   CM:
               SUCCESS                                                              append:                                              docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade   SW UPGRADE         Notice       SW download Successful - Via         Other than Local Log,        E112.0       69011200   CM:
               SUCCESS                         Config file                          append:                                              docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade   SW UPGRADE         Error        Improper Code File Controls          Other than Local Log,        E201.0       69020100   CM:
               GENERAL FAILURE                                                      append:                                              docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address




02/19/25                                                           CableLabs®                                                                              135
<!-- PDF_PAGE: 136 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                            Data-Over-Cable Service Interface Specifications



      Process      Sub-Process        CM                    Event                       Message Notes             Error      Event        Notification
                                    Priority               Message                        and Detail             Code Set     ID            Name
  SW Upgrade    SW UPGRADE        Error        Code File Manufacturer CVC           Other than Local Log,        E202.0     69020200   CM:
                GENERAL FAILURE                Validation Failure                   append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        Code File Manufacturer CVS           Other than Local Log,        E203.0     69020300   CM:
                GENERAL FAILURE                Validation Failure                   append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        Code File Co-Signer CVC Validation   Other than Local Log,        E204.0     69020400   CM:
                GENERAL FAILURE                Failure                              append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    SW UPGRADE        Error        Code File Co-Signer CVS Validation   Other than Local Log,        E205.0     69020500   CM:
                GENERAL FAILURE                Failure                              append:                                            docsIf3CmEventNotif
                                                                                    SW file: <P1> - SW server:
                                                                                    < P2><TAGS>

                                                                                    P1 = SW file name
                                                                                    P2 = SW Download server
                                                                                    IP address
  SW Upgrade    VERIFICATION OF   Error        Improper Configuration File CVC      Other than Local Log,        E206.0     69020600   CM:
                CVC                            Format                               append:                                            docsIf3CmEventNotif
                                                                                    Config file: <P1> - Config
                                                                                    file server: < P2><TAGS>

                                                                                    P1 = Config file name
                                                                                    P2 = Config file server IP
                                                                                    address




136                                                                CableLabs®                                                                     02/19/25
<!-- PDF_PAGE: 137 -->
Cable Modem Operations Support System Interface Specification                                                                CM-SP-CM-OSSIv3.1-I27-250219



    Process        Sub-Process            CM                    Event                      Message Notes             Error        Event        Notification
                                        Priority               Message                       and Detail             Code Set       ID            Name
  SW Upgrade   VERIFICATION OF        Error        Configuration File CVC Validation   Other than Local Log,        E207.0       69020700   CM:
               CVC                                 Failure                             append:                                              docsIf3CmEventNotif
                                                                                       Config file: <P1> - Config
                                                                                       file server: < P2><TAGS>

                                                                                       P1 = Config file name
                                                                                       P2 = Config file server IP
                                                                                       address
  SW Upgrade   VERIFICATION OF        Error        Improper SNMP CVC Format            Other than local Log,        E208.0       69020800   CM:
               CVC                                                                     append: SNMP Manager:                                docsIf3CmEventNotif
                                                                                       <P1><TAGS>

                                                                                       P1= IP Address of SNMP
                                                                                       Manager
  SW Upgrade   VERIFICATION OF        Error        SNMP CVC Validation Failure         Other than local Log,        E209.0       69020900   CM:
               CVC                                                                     append: SNMP Manager:                                docsIf3CmEventNotif
                                                                                       <P1><TAGS>

                                                                                       P1= IP Address of SNMP
                                                                                       Manager
  Registration and TLV-11
  Init         REGISTRATION           Critical     REG-RSP - invalid format or not                                  I01.0        73000100
               RESPONSE                            recognized;<TAGS>
  Init         REGISTRATION           Critical     REG RSP not received<TAGS>                                       I02.0        73000200
               RESPONSE
  Init         REGISTRATION           Critical     REG RSP bad SID <P1><TAGS>          P1 = SID which was reject    I03.0        73000300
               RESPONSE
  Init         1.1 and 2.0 SPECIFIC   Critical     REG RSP contains service flow       P1 = Service Flow ID         I251.0       73025100
               REGISTRATION                        parameters that CM cannot support
               RESPONSE                            <P1><TAGS>
  Init         1.1 and 2.0 SPECIFIC   Critical     REG RSP contains classifier         P1 = Service Flow ID         I251.1       73025101
               REGISTRATION                        parameters that CM cannot support
               RESPONSE                            <P1><TAGS>
  Init         1.1 and 2.0 SPECIFIC   Critical     Registration RSP rejected                                        I251.3       73025103
               REGISTRATION                        unspecified reason<TAGS>
               RESPONSE
  Init         1.1 and 2.0 SPECIFIC   Critical     Registration RSP rejected message   P1 = message                 I251.4       73025104
               REGISTRATION                        syntax error <P1><TAGS>
               RESPONSE




02/19/25                                                                CableLabs®                                                                            137
<!-- PDF_PAGE: 138 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                                    Data-Over-Cable Service Interface Specifications



      Process      Sub-Process             CM                        Event                       Message Notes           Error      Event        Notification
                                         Priority                   Message                        and Detail           Code Set     ID            Name
  Init          1.1 and 2.0 SPECIFIC   Critical        Registration RSP rejected message     P1 = # of characters      I251.5      73025105
                REGISTRATION                           too big <P1><TAGS>
                RESPONSE
  Init          2.0 SPECIFIC           Warning         REG-RSP received after REG-ACK.                                 I261.0      73026100
                REGISTRATION                           Returning to 1.x transmit
                RESPONSE                               mode<TAGS>
  Init          TLV-11 PARSING         Notice          TLV-11 - unrecognized OID<TAGS>                                 I401.0      73040100   CM:
                                                                                                                                              docsIf3CmEventNotif
  Init          TLV-11 PARSING         Critical        TLV-11 - Illegal Set operation                                  I402.0      73040200   CM:
                                                       failed<TAGS>                                                                           docsIf3CmEventNotif
  Init          TLV-11 PARSING         Critical        TLV-11 - Failed to set duplicate                                I403.0      73040300   CM:
                                                       elements<TAGS>                                                                         docsIf3CmEventNotif
  Init          Waiting for REG-RSP    Error           T6 Timeout and retries                                          I271.0      73027100
                or REG-RSP-MP                          exceeded<TAGS>
  Init          CM Complete            Error           Cannot create US Primary Service                                I501.0      73050100
                Registration                           Flow<TAGS>
  Init          CM Complete            Notice          Received REG-RSP while in REG-                                  I502.0      73050200
                Registration                           HOLD1 state<TAGS>
  Init          CM Complete            Notice          Received REG-RSP while in REG-                                  I503.0      73050300
                Registration                           HOLD2 state<TAGS>
  Init          CM Complete            Warning         REG-RSP-MP Mismatch Between                                     I504.0      73050400
                Registration                           Calculated Value for P1.6hi
                                                       Compared to CCAP Provided
                                                       Value<TAGS>
  General
                                       Informational   A transmit opportunity was missed                               N01.0       78000100
                                                       because the MAP arrived too late.
  Ranging
  Init          RANGING                Critical        No Maintenance Broadcasts for                                   R01.0       82000100
                                                       Ranging opportunities received - T2
                                                       time-out<TAGS>
  Init          RANGING                Critical        No Ranging Response received - T3                               R02.0       82000200
                                                       time-out<TAGS>
  Init          RANGING                Critical        Ranging Request Retries                                         R03.0       82000300
                                                       exhausted<TAGS>
  Init          RANGING                Critical        Received Response to Broadcast                                  R04.0       82000400
                                                       Maintenance Request, But no
                                                       Unicast Maintenance opportunities
                                                       received - T4 time out<TAGS>




138                                                                          CableLabs®                                                                  02/19/25
<!-- PDF_PAGE: 139 -->
Cable Modem Operations Support System Interface Specification                                                               CM-SP-CM-OSSIv3.1-I27-250219



    Process          Sub-Process       CM                     Event                        Message Notes            Error        Event        Notification
                                     Priority                Message                         and Detail            Code Set       ID            Name
  Init          RANGING            Critical     Started Unicast Maintenance                                        R05.0        82000500
                                                Ranging - No Response received -
                                                T3 time-out<TAGS>
  Init          RANGING            Critical     Unicast Maintenance Ranging                                        R06.0        82000600
                                                attempted - No response - Retries
                                                exhausted<TAGS>
  Init          RANGING            Critical     Unicast Ranging Received Abort                                     R07.0        82000700
                                                Response - Re-initializing
                                                MAC<TAGS>
  Init          RANGING            Critical     16 consecutive T3 timeouts while        P1 = Upstream Channel ID   R08.0        82000800
                                                trying to range on upstream channel
                                                <P1><TAGS>
  Init          RANGING            Warning      B-INIT-RNG Failure - Retries                                       R09.0        82000900
                                                exceeded<TAGS>
  Station       RANGING            Warning      RNG-RSP CCAP Commanded                                             R10.0        82001000
  Maintenance                                   Power Exceeds Value for
                                                Pmax<TAGS>
  Station       RANGING            Warning      RNG-RSP CCAP Commanded                                             R11.0        82001100
  Maintenance                                   Power Exceeds Value
                                                Corresponding to the Top of the
                                                DRW<TAGS>
  Station       RANGING            Warning      RNG-RSP CCAP Commanded                                             R12.0        82001200
  Maintenance                                   Power in Excess of 6 dB Below the
                                                Value Corresponding to the Top of
                                                the DRW<TAGS>
  Dynamic Services
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Unspecified                                 S01.0        83000100   CM:
  SERVICES      REQUEST                         reason<TAGS>                                                                               docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Unrecognized                                S01.1        83000101   CM:
  SERVICES      REQUEST                         configuration setting<TAGS>                                                                docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Classifier not                              S01.10       83000110   CM:
  SERVICES      REQUEST                         found<TAGS>                                                                                docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Classifier                                  S01.11       83000111   CM:
  SERVICES      REQUEST                         exists<TAGS>                                                                               docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Duplicated                                  S01.14       83000114   CM:
  SERVICES      REQUEST                         reference-ID or index in                                                                   docsIf3CmEventNotif
                                                message<TAGS>
  DYNAMIC       DYNAMIC SERVICE    Error        Service Add rejected - Multiple                                    S01.15       83000115   CM:
  SERVICES      REQUEST                         upstream flows<TAGS>                                                                       docsIf3CmEventNotif




02/19/25                                                              CableLabs®                                                                             139
<!-- PDF_PAGE: 140 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                     Data-Over-Cable Service Interface Specifications



      Process      Sub-Process        CM                     Event                     Message Notes      Error      Event        Notification
                                    Priority                Message                      and Detail      Code Set     ID            Name
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Multiple                          S01.16      83000116   CM:
  SERVICES      REQUEST                        downstream flows<TAGS>                                                          docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Classifier for                    S01.17      83000117   CM:
  SERVICES      REQUEST                        another flow<TAGS>                                                              docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Parameter                         S01.19      83000119   CM:
  SERVICES      REQUEST                        invalid for context<TAGS>                                                       docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Temporary no                      S01.2       83000102   CM:
  SERVICES      REQUEST                        resource<TAGS>                                                                  docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Authorization                     S01.20      83000120   CM:
  SERVICES      REQUEST                        failure<TAGS>                                                                   docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Major service                     S01.21      83000121   CM:
  SERVICES      REQUEST                        flow error<TAGS>                                                                docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Major                             S01.22      83000122   CM:
  SERVICES      REQUEST                        classifier error<TAGS>                                                          docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Multiple major                    S01.24      83000124   CM:
  SERVICES      REQUEST                        errors<TAGS>                                                                    docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Message                           S01.25      83000125   CM:
  SERVICES      REQUEST                        syntax error<TAGS>                                                              docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Message too                       S01.26      83000126   CM:
  SERVICES      REQUEST                        big<TAGS>                                                                       docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Temporary                         S01.27      83000127   CM:
  SERVICES      REQUEST                        DCC<TAGS>                                                                       docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Permanent                         S01.3       83000103   CM:
  SERVICES      REQUEST                        administrative<TAGS>                                                            docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Required                          S01.4       83000104   CM:
  SERVICES      REQUEST                        parameter not present<TAGS>                                                     docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Header                            S01.5       83000105   CM:
  SERVICES      REQUEST                        suppression setting not                                                         docsIf3CmEventNotif
                                               supported<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Service flow                      S01.6       83000106   CM:
  SERVICES      REQUEST                        exists<TAGS>                                                                    docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - HMAC Auth                         S01.7       83000107   CM:
  SERVICES      REQUEST                        failure<TAGS>                                                                   docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Add                               S01.8       83000108   CM:
  SERVICES      REQUEST                        aborted<TAGS>                                                                   docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add rejected - Multiple                          S01.9       83000109   CM:
  SERVICES      REQUEST                        errors<TAGS>                                                                    docsIf3CmEventNotif




140                                                                  CableLabs®                                                           02/19/25
<!-- PDF_PAGE: 141 -->
Cable Modem Operations Support System Interface Specification                                                  CM-SP-CM-OSSIv3.1-I27-250219



   Process       Sub-Process          CM                     Event                    Message Notes    Error        Event        Notification
                                    Priority                Message                     and Detail    Code Set       ID            Name
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.0        83000200   CM:
  SERVICES    REQUEST                          Unspecified reason<TAGS>                                                       docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.1        83000201   CM:
  SERVICES    REQUEST                          Unrecognized configuration                                                     docsIf3CmEventNotif
                                               setting<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Classifier                   S02.10       83000210   CM:
  SERVICES    REQUEST                          not found<TAGS>                                                                docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Classifier                   S02.11       83000211   CM:
  SERVICES    REQUEST                          exists<TAGS>                                                                   docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.14       83000214   CM:
  SERVICES    REQUEST                          Duplicated reference-ID or index in                                            docsIf3CmEventNotif
                                               message<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Multiple                     S02.15       83000215   CM:
  SERVICES    REQUEST                          upstream flows<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Multiple                     S02.16       83000216   CM:
  SERVICES    REQUEST                          downstream flows<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Classifier                   S02.17       83000217   CM:
  SERVICES    REQUEST                          for another flow<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Invalid                      S02.19       83000219   CM:
  SERVICES    REQUEST                          parameter for context<TAGS>                                                    docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.2        83000202   CM:
  SERVICES    REQUEST                          Temporary no resource<TAGS>                                                    docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.20       83000220   CM:
  SERVICES    REQUEST                          Authorization failure<TAGS>                                                    docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Major                        S02.21       83000221   CM:
  SERVICES    REQUEST                          service flow error<TAGS>                                                       docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -Major                         S02.22       83000222   CM:
  SERVICES    REQUEST                          classifier error<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Multiple                     S02.24       83000224   CM:
  SERVICES    REQUEST                          major errors<TAGS>                                                             docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Message                      S02.25       83000225   CM:
  SERVICES    REQUEST                          syntax error<TAGS>                                                             docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected - Message                      S02.26       83000226   CM:
  SERVICES    REQUEST                          too big<TAGS>                                                                  docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.27       83000227   CM:
  SERVICES    REQUEST                          Temporary DCC<TAGS>                                                            docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change rejected -                              S02.3        83000203   CM:
  SERVICES    REQUEST                          Permanent administrative<TAGS>                                                 docsIf3CmEventNotif




02/19/25                                                            CableLabs®                                                                  141
<!-- PDF_PAGE: 142 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                  Data-Over-Cable Service Interface Specifications



      Process      Sub-Process        CM                     Event                  Message Notes      Error      Event        Notification
                                    Priority                Message                   and Detail      Code Set     ID            Name
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected -                             S02.4       83000204   CM:
  SERVICES      REQUEST                        Requester not owner of service                                               docsIf3CmEventNotif
                                               flow<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected - Service                     S02.5       83000205   CM:
  SERVICES      REQUEST                        flow not found<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected - Required                    S02.6       83000206   CM:
  SERVICES      REQUEST                        parameter not present<TAGS>                                                  docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected - Header                      S02.7       83000207   CM:
  SERVICES      REQUEST                        suppression setting not                                                      docsIf3CmEventNotif
                                               supported<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected - HMAC                        S02.8       83000208   CM:
  SERVICES      REQUEST                        Auth failure<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change rejected - Multiple                    S02.9       83000209   CM:
  SERVICES      REQUEST                        errors<TAGS>                                                                 docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete rejected -                             S03.0       83000300   CM:
  SERVICES      REQUEST                        Unspecified reason<TAGS>                                                     docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete rejected -Requester                    S03.1       83000301   CM:
  SERVICES      REQUEST                        not owner of service flow<TAGS>                                              docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete rejected - Service                     S03.2       83000302   CM:
  SERVICES      REQUEST                        flow not found<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete rejected - HMAC                        S03.3       83000303   CM:
  SERVICES      REQUEST                        Auth failure<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete rejected - Message                     S03.4       83000304   CM:
  SERVICES      REQUEST                        syntax error<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S101.0      83010100   CM:
  SERVICES      RESPONSE                       Invalid transaction ID<TAGS>                                                 docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add aborted - No                              S101.1      83010101   CM:
  SERVICES      RESPONSE                       RSP<TAGS>                                                                    docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S101.11     83010111   CM:
  SERVICES      RESPONSE                       Duplicate reference_ID or index in                                           docsIf3CmEventNotif
                                               message<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S101.12     83010112   CM:
  SERVICES      RESPONSE                       Classifier for another flow<TAGS>                                            docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S101.13     83010113   CM:
  SERVICES      RESPONSE                       Parameter invalid for                                                        docsIf3CmEventNotif
                                               context<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S101.14     83010114   CM:
  SERVICES      RESPONSE                       Major service flow error<TAGS>                                               docsIf3CmEventNotif




142                                                                 CableLabs®                                                         02/19/25
<!-- PDF_PAGE: 143 -->
Cable Modem Operations Support System Interface Specification                                                 CM-SP-CM-OSSIv3.1-I27-250219



   Process       Sub-Process          CM                    Event                   Message Notes    Error         Event        Notification
                                    Priority               Message                    and Detail    Code Set        ID            Name
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.15       83010115   CM:
  SERVICES    RESPONSE                         Major classifier error<TAGS>                                                  docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.17       83010117   CM:
  SERVICES    RESPONSE                         Multiple major errors<TAGS>                                                   docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.18       83010118   CM:
  SERVICES    RESPONSE                         Message too big<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.2        83010102   CM:
  SERVICES    RESPONSE                         HMAC Auth failure<TAGS>                                                       docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.3        83010103   CM:
  SERVICES    RESPONSE                         Message syntax error<TAGS>                                                    docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.4        83010104   CM:
  SERVICES    RESPONSE                         Unspecified reason<TAGS>                                                      docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.5        83010105   CM:
  SERVICES    RESPONSE                         Unrecognized configuration                                                    docsIf3CmEventNotif
                                               setting<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.6        83010106   CM:
  SERVICES    RESPONSE                         Required parameter not                                                        docsIf3CmEventNotif
                                               present<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.7        83010107   CM:
  SERVICES    RESPONSE                         Service Flow exists<TAGS>                                                     docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.8        83010108   CM:
  SERVICES    RESPONSE                         Multiple errors<TAGS>                                                         docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Add Response rejected -                      S101.9        83010109   CM:
  SERVICES    RESPONSE                         Classifier exists<TAGS>                                                       docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change Response rejected -                   S102.0        83010200   CM:
  SERVICES    RESPONSE                         Invalid transaction ID<TAGS>                                                  docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change aborted- No                           S102.1        83010201   CM:
  SERVICES    RESPONSE                         RSP<TAGS>                                                                     docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change Response rejected -                   S102.10       83010210   CM:
  SERVICES    RESPONSE                         Duplicated reference-ID or index                                              docsIf3CmEventNotif
                                               in<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change Response rejected -                   S102.11       83010211   CM:
  SERVICES    RESPONSE                         Invalid parameter for                                                         docsIf3CmEventNotif
                                               context<TAGS>
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change Response rejected -                   S102.12       83010212   CM:
  SERVICES    RESPONSE                         Major classifier error<TAGS>                                                  docsIf3CmEventNotif
  DYNAMIC     DYNAMIC SERVICE     Error        Service Change Response rejected -                   S102.14       83010214   CM:
  SERVICES    RESPONSE                         Multiple Major errors<TAGS>                                                   docsIf3CmEventNotif




02/19/25                                                           CableLabs®                                                                  143
<!-- PDF_PAGE: 144 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                  Data-Over-Cable Service Interface Specifications



      Process      Sub-Process        CM                    Event                   Message Notes      Error      Event        Notification
                                    Priority               Message                    and Detail      Code Set     ID            Name
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.15     83010215   CM:
  SERVICES      RESPONSE                       Message too big<TAGS>                                                        docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.2      83010202   CM:
  SERVICES      RESPONSE                       HMAC Auth failure<TAGS>                                                      docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.3      83010203   CM:
  SERVICES      RESPONSE                       Message syntax error<TAGS>                                                   docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.4      83010204   CM:
  SERVICES      RESPONSE                       Unspecified reason<TAGS>                                                     docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.5      83010205   CM:
  SERVICES      RESPONSE                       Unrecognized configuration                                                   docsIf3CmEventNotif
                                               setting<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.6      83010206   CM:
  SERVICES      RESPONSE                       Required parameter not                                                       docsIf3CmEventNotif
                                               present<TAGS>
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.7      83010207   CM:
  SERVICES      RESPONSE                       Multiple errors<TAGS>                                                        docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Response rejected -                    S102.8      83010208   CM:
  SERVICES      RESPONSE                       Classifier exists<TAGS>                                                      docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Delete Response rejected -                    S103.0      83010300   CM:
  SERVICES      RESPONSE                       Invalid transaction ID<TAGS>                                                 docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Response rejected -                       S201.0      83020100   CM:
  SERVICES      ACKNOWLEDGEMENT                Invalid Transaction ID<TAGS>                                                 docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add Aborted - No                              S201.1      83020101   CM:
  SERVICES      ACKNOWLEDGEMENT                ACK<TAGS>                                                                    docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add ACK rejected - HMAC                       S201.2      83020102   CM:
  SERVICES      ACKNOWLEDGEMENT                auth failure<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Add ACK rejected- Message                     S201.3      83020103   CM:
  SERVICES      ACKNOWLEDGEMENT                syntax error<TAGS>                                                           docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change ACK rejected -                         S202.0      83020200   CM:
  SERVICES      ACKNOWLEDGEMENT                Invalid transaction ID<TAGS>                                                 docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change Aborted - No                           S202.1      83020201   CM:
  SERVICES      ACKNOWLEDGEMENT                ACK<TAGS>                                                                    docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change ACK rejected -                         S202.2      83020202   CM:
  SERVICES      ACKNOWLEDGEMENT                HMAC Auth failure<TAGS>                                                      docsIf3CmEventNotif
  DYNAMIC       DYNAMIC SERVICE   Error        Service Change ACK rejected -                         S202.3      83020203   CM:
  SERVICES      ACKNOWLEDGEMENT                Message syntax error<TAGS>                                                   docsIf3CmEventNotif




144                                                                CableLabs®                                                          02/19/25
<!-- PDF_PAGE: 145 -->
Cable Modem Operations Support System Interface Specification                                                 CM-SP-CM-OSSIv3.1-I27-250219



    Process       Sub-Process         CM                     Event                   Message Notes    Error        Event     Notification
                                    Priority                Message                    and Detail    Code Set       ID         Name
  Downstream Acquisition
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T01.0        84000100
              ACQUISITION                      failure - Failed to acquire
                                               QAM/QPSK symbol timing;<TAGS>
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T02.0        84000200
              ACQUISITION                      failure - Failed to acquire FEC
                                               framing<TAGS>
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T02.1        84000201
              ACQUISITION                      failure, Acquired FEC framing -
                                               Failed to acquire MPEG2
                                               Sync<TAGS>
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T03.0        84000300
              ACQUISITION                      failure - Failed to acquire MAC
                                               framing<TAGS>
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T04.0        84000400
              ACQUISITION                      failure - Failed to receive MAC
                                               SYNC frame within time-out
                                               period<TAGS>
  Init        DOWNSTREAM          Critical     SYNC Timing Synchronization                           T05.0        84000500
              ACQUISITION                      failure - Loss of Sync<TAGS>
  Init        DOWNSTREAM          Error        RCS Primary DS Failure<TAGS>                          T06.0        84000600
              ACQUISITION
  Init        DOWNSTREAM          Warning      RCS Partial Service<TAGS>                             T07.0        84000700
              ACQUISITION
  Init        RCP and RCC         Error        RCP-ID in RCC not                                     T101.0       84010100
                                               supported<TAGS>
  Init        RCP and RCC         Error        More than one RCP-ID included in                      T102.0       84010200
                                               RCC<TAGS>
  Init        RCP and RCC         Error        Receive Module Index missing in                       T103.0       84010300
                                               RCC<TAGS>
  Init        RCP and RCC         Error        RCC contains a Receive Module                         T104.0       84010400
                                               Index which is not
                                               supported<TAGS>
  Init        RCP and RCC         Error        Receive channel center frequency                      T105.0       84010500
                                               not within allowed range of center
                                               frequencies for Receive
                                               Module<TAGS>
  Init        RCP and RCC         Error        Receive Module first channel center                   T106.0       84010600
                                               frequency not within allowed range
                                               of center frequencies<TAGS>




02/19/25                                                             CableLabs®                                                             145
<!-- PDF_PAGE: 146 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                        Data-Over-Cable Service Interface Specifications



      Process      Sub-Process            CM                     Event                    Message Notes      Error      Event       Notification
                                        Priority                Message                     and Detail      Code Set     ID           Name
  Init          RCP and RCC           Error        Receive Module first channel center                     T107.0      84010700
                                                   frequency not present in
                                                   RCC<TAGS>
  Init          RCP and RCC           Error        No primary downstream channel                           T108.0      84010800
                                                   assignment in RCC<TAGS>
  Init          RCP and RCC           Error        More than one primary downstream                        T109.0      84010900
                                                   channel assignment present in
                                                   RCC<TAGS>
  Init          RCP and RCC           Error        Receive Module connectivity                             T110.0      84011000
                                                   encoding in RCC Requires
                                                   configuration not supported<TAGS>
  Init          RCP and RCC           Error        Receive channel index in RCC not                        T111.0      84011100
                                                   supported by CM<TAGS>
  Init          RCP and RCC           Error        Center frequency in RCC not a                           T112.0      84011200
                                                   multiple of 62500 Hz<TAGS>
  Init          MDD                   Error        Missing Mandatory MDD TLV on                            T201.0      84020100
                                                   primary DS Channel<TAGS>
  Init          MDD                   Warning      Lost MDD Timeout<TAGS>                                  T202.0      84020200
  Init          MDD                   Warning      MDD message timeout<TAGS>                               T203.0      84020300
  Init          OBTAIN UPSTREAM       Critical     No UCDs Received -                                      U01.0       85000100
                PARAMETERS                         Timeout;<TAGS>
  Init          OBTAIN UPSTREAM       Critical     UCD invalid or channel                                  U02.0       85000200
                PARAMETERS                         unusable<TAGS>
  Init          OBTAIN UPSTREAM       Critical     UCD & SYNC valid - NO MAPS for                          U04.0       85000400
                PARAMETERS                         this channel<TAGS>
  Init          OBTAIN UPSTREAM       Critical     US channel wide parameters not set                      U06.0       85000600
                PARAMETERS                         before Burst Descriptors<TAGS>
  Init          Acquire CM Transmit   Error        TCS Fail on all Upstream                                U101.0      85010100
                Channels                           Channels<TAGS>
  Init          Acquire CM Transmit   Warning      TCS Partial Service<TAGS>                               U102.0      85010200
                Channels
  Init          Acquire CM Transmit   Warning      Initializing Channel Timeout Expires                    U103.0      85010300
                Channels                           - Time the CM can perform initial
                                                   ranging on all upstream channels in
                                                   the TCS has expired<TAGS>




146                                                                     CableLabs®                                                         02/19/25
<!-- PDF_PAGE: 147 -->
Cable Modem Operations Support System Interface Specification                                                               CM-SP-CM-OSSIv3.1-I27-250219



   Process        Sub-Process          CM                       Event                        Message Notes          Error        Event        Notification
                                     Priority                  Message                         and Detail          Code Set       ID            Name
  CM-CTRL
  CM-CTRL     CM-CTRL              Debug           CM-CTRL - Command: <P1>               P1 =                      L01.0        76000100   CM:
                                                   (if P1= mute                          mute, or                                          docsIf3CmEventNotif
                                                   Add                                   cmReinit, or
                                                   Interval: <P2>                        forwarding
                                                   ChannelID: <P3>)                      P2= mute interval,
                                                   (If P1 = forwarding                   Value 0 indicate
                                                   Add                                   unmute operation
                                                   Action: <P4>)                         P3= Channel ID or 0
                                                   <TAGS>                                P4 = enable, or disable
  CM-CTRL     CM-CTRL              Debug           CM-CTRL- Invalid message                                        L02.0        76000200   CM:
                                                   format<TAGS>                                                                            docsIf3CmEventNotif
  Energy Management
  EM          EM-REQ               Informational   EM-RSP not received<TAGS>                                       L101.0       76010100
  EM          EM-REQ               Warning         EM-REQ retries exhausted<TAGS>                                  L102.0       76010200
  EM          EM-REQ               Informational   EM-RSP received, Reject               <P1> = time to defer      L103.0       76010300
                                                   Temporary, deferring for <P1>         (seconds)
                                                   seconds<TAGS>
  EM          EM-REQ               Warning         EM-RSP received, Reject                                         L104.0       76010400
                                                   Permanent<TAGS>
  EM          DBC                  Informational   CM entered EM 1x1 mode; Reason:       P1=Unknown, Activity      L113.0       76011300   CM:
                                                   <P1><TAGS>                            Detection, eSAFE, CMTS                            docsIf3CmEventNotif
                                                                                         Initiated
  EM          DBC                  Informational   CM exited EM 1x1 mode<TAGS>                                     L114.0       76011400   CM:
                                                                                                                                           docsIf3CmEventNotif
  EM          Activity Detection   Informational   EM 1x1 Activity Detection Threshold   P1=Upstream entry,        L115.0       76011500   CM:
                                                   crossed; Reason:<P1><TAGS>            Downstream entry,                                 docsIf3CmEventNotif
                                                                                         Upstream exit,
                                                                                         Downstream exit
  EM          EM-REQ               Informational   EM-REQ Sent<TAGS>                                               L116.0       76011600




02/19/25                                                               CableLabs®                                                                            147
<!-- PDF_PAGE: 148 -->
CM-SP-CM-OSSIv3.1-I27-250219                                                                             Data-Over-Cable Service Interface Specifications



      Process       Sub-Process         CM                    Event                      Message Notes               Error      Event        Notification
                                      Priority               Message                       and Detail               Code Set     ID            Name
  CM-STATUS
  Status        Status              Notice       CM-STATUS message sent. Event       P1: CM-STATUS event            J101.0     74010100   CM:
                                                 Type Code: <P1>; Chan ID: <P2>;     type code. Refer to                                  docsIf3CmEventNotif
                                                 DSID: <P3>; MAC Addr: <P4>;         [MULPIv3.1] CM-STATUS
                                                 OFDM/OFDMA Profile ID:              Event Type Codes and
                                                 <P5>.<TAGS>                         Status Events for event
                                                                                     type codes.
                                                                                     P2: Upstream/Downstream
                                                                                     channel ID; "N/A" if not
                                                                                     provided for this status
                                                                                     message type.
                                                                                     P3: DSID; "N/A" if not
                                                                                     provided for this status
                                                                                     message type.
                                                                                     P4: MAC Address; "N/A" if
                                                                                     not provided for this status
                                                                                     message type.
                                                                                     P5: OFDM or OFDMA
                                                                                     Profile ID; "N/A" if not
                                                                                     provided for this status
                                                                                     message type.
  Profile Change
  Profile       Profile Change      Notice       DS profile assignment change. DS    P1: Downstream channel         C616.0     67061600   CM:
                                                 Chan ID: <P1>; Previous Profile:    ID                                                   docsIf3CmEventNotif
                                                 <P2>; New Profile: <P3>.<TAGS>      P2: Previous OFDM Profile
                                                                                     ID
                                                                                     P3: New OFDM Profile ID
  Profile       Profile Change      Notice       US profile assignment change. US    P1: Upstream channel ID        C616.1     67061601   CM:
                                                 Chan ID: <P1>; Previous Profile:    P2: Previous OFDMA                                   docsIf3CmEventNotif
                                                 <P2>; New Profile: <P3>.<TAGS>      Profile ID
                                                                                     P3: New OFDMA Profile ID
  Profile       Profile Update      Info         DS profile config update. DS Chan   P1: Downstream channel         C616.2     67061602   CM:
                                                 ID: <P1>.<TAGS>                     ID                                                   docsIf3CmEventNotif
  Profile       Profile Update      Info         US profile config update. US Chan   P1: Upstream channel ID        C616.3     67061603   CM:
                                                 ID: <P1>.<TAGS>                                                                          docsIf3CmEventNotif
  RxMER Reporting
  RxMER         Cancelled Request   Error        DBC or Primary Backup interface                                    C860.0     86010100
                                                 Index modified




148                                                                  CableLabs®                                                                      02/19/25
<!-- PDF_PAGE: 149 -->
Cable Modem Operations Support System Interface Specification                                                                    CM-SP-CM-OSSIv3.1-I27-250219



C.1 Deprecated Events
Table 50 in this annex lists deprecated events, including any associated syslog and SNMP trap notifications for the events, for a DOCSIS 3.1-compliant CM.
Implementation of deprecated events is optional.
                                                                   Table 50 - Deprecated Events

   Process       Sub-Process         CM                       Event                           Message Notes              Error       Event         Notification
                                   Priority                  Message                            and Detail              Code Set      ID             Name
  Authentication and Encryption
  Init (BPI+)   DOCSIS 1.0        Error       Missing BP Configuration Setting TLV      P1 = missing required TLV       B101.0      66010100   CM:
                CONFIG FILE                   Type: <P1><TAGS>                          Type                                                   docsIf3CmEventNotif
  Init (BPI+)   DOCSIS 1.0        Alert       Invalid BP Configuration Setting Value:   P1=The TLV Value for P2.        B102.0      66010200   CM:
                CONFIG FILE                   <P1> for Type: <P2><TAGS>                 P2 = The first Configuration                           docsIf3CmEventNotif
                                                                                        TLV Type that contain invalid
                                                                                        value.
  UCC
  UCC           UCC Request       Error       UCC-REQ received with invalid or out of                                   C01.0       67000100
                                              range US channel ID<TAGS>
  UCC           UCC Request       Error       UCC-REQ received unable to send UCC-                                      C02.0       67000200
                                              RSP<TAGS>
  DHCP, TOD and TFTP
  Init          TFTP              Critical    TFTP file complete- but does not enable   For SYSLOG only: append:        D11.0       68001100
                                              2.0 Mode - conflicts with current US      File name = <P1> P1 = file
                                              channel type<TAGS>                        name of TFTP file




02/19/25                                                                    CableLabs®                                                                            149
