# Annex C - Format and Content for Event, Syslog, and SNMP Notification

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 162-187
Classification: Normative (explicit)

<!-- PDF_PAGE: 162 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


Annex C            Format and Content for Event, SYSLOG, and SNMP
                   Notification (Normative)
The CM MUST format the CM MAC Address field <CM-MAC> of the Event Message text, including such
instances of docsDevEvText, using lowercase letters.
The CM MUST format the CMTS MAC Address field <CMTS-MAC> of the Event Message text, including such
instances of docsDevEvText, using lower case letters.
The CM MAY append additional vendor-specific text to the end of the event text reported in the docsDevEvText
object and the syslog text field.
Table 78 in this annex summarizes the format and content for event, syslog, and SNMP notifications required for a
DOCSIS 4.0-compliant CM.
Each row specifies a possible event that may appear in the CM. These events are to be reported by a CM through
local event logging, and may be accompanied by syslog or SNMP notification.
The "Process" and "Sub-Process" columns indicate in which stage the event happens. The "CM Priority" column
indicates the priority the event is assigned in the CM. The priority is the same as is reported in the docsDevEvLevel
object in the cable device MIB [RFC 4639] and in the LEVEL field of the syslog.
The "Event Message" column specifies the event text, which is reported in the docsDevEvText object of the cable
device MIB and the text field of the syslog. The "Message Notes and Details" column provides additional
information about the event text in the "Event Message" column. Some of the text fields include variable
information, which are often specified as key-value pairs. The variables are explained in the "Message Notes and
Details" column. For some events the "Message Notes and Details" column may include the keyword <Deprecated>
to indicate this event is being deprecated and its implementation is optional. For events where the "Event Message"
or "Message Notes and Details" column includes either <P1>, <P2>, or <Pn>, there is a colon and single space
between the value as defined by the <P1>, <P2>, or <Pn> and the preceding key text.
The key-value parameters are thus formatted as: [key]: [value]. Key value pairs are delimited by a semi-colon
followed by a single space, as the following example indicates:
[key 1]: [value 1]; [key 2]: [value 2]; [key n]: [value n]
The "Event Message" field structure is defined as follows:
<Initial Event Message Text>; [key 1]: [value 1]; [key 2]: [value 2]; [key n]: [value n]; <TAGS>;
Keys containing values that represent strings can enclose those strings within double quotations to prevent confusion
if those string values contain a delimiter (colon or semicolon). Key strings should capitalize each word (e.g., "Sensor
Unit"). If a key's value is not present, the key is present but the value is omitted (using a single space). For example:
key 1: ;
It is recommended that the <Initial Event Message Text> string does not contain a semicolon since this is used as a
delimiter. The <Initial Event Message Text> string follows a normal sentence capitalization scheme where the first
word is capitalized, as well as any defined terms and acronyms.
Extended TCS fail on all upstream channels; Chan ID: 1; Mode: FDX; CM-MAC=00:22:ce:03:f4:da; CMTS-
MAC=00:15:20:00:25:ab; CM-QOS=1.1; CM-VER=4.0;
This specification defines the following keywords as part of the "Event Message" column:
"<TAGS>" (without the quotes) corresponds to:
For the CM (without the quotes):";<CM-MAC>;<CMTS-MAC>;<CM-QOS>;<CM-VER>;"

Where:
<CM-MAC>:    CM MAC Address;
Format*: "CM-MAC=xx:xx:xx:xx:xx:xx"

<CMTS-MAC>:        CMTS MAC Address;



162                                                   CableLabs                                               04/29/26
<!-- PDF_PAGE: 163 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


Format*: "CMTS-MAC=xx:xx:xx:xx:xx:xx"

<CM-QOS>:    CM DOCSIS QOS Version;
Format*: "CM-QOS=1.0" or "CM-QOS=1.1"

<CM-VER>:    CM DOCSIS Version;
Format*: "CM-VER=1.1" or "CM-VER=2.0" or "CM-VER=3.0" or "CM-VER=3.1" or "CM-VER=4.0"
(*) without the quotes

The "Error Code Set" column specifies the error code. The "Event ID" column indicates a unique identification
number for the event, which is assigned to the docsDevEvId object in the cable device MIB and the <eventId> field
of the syslog. The "Notification Name" column specifies the SNMP notification, which notifies this event to an
SNMP notification receiver.
The syslog format, as well as the rules to uniquely generate an event ID from the error code, are described in Section
8.1.2.1.3 of this specification.




04/29/26                                           CableLabs                                                    163
<!-- PDF_PAGE: 164 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                      Data-Over-Cable Service Interface Specifications


                                                        Table 78 - Event Format and Content

                                   CM                    Event                                               Error         Event    Notification
  Process      Sub-Process       Priority               Message                 Message Notes and Detail    Code Set        ID        Name
                                                             Authentication and Encryption
                                             <Reserved>                                                                0
 BPKM       AUTH-FSM           Warning       Auth Reject - No                                               B301.2     66030102 CM:
                                             Information<TAGS>                                                                  docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Reject - Unauthorized                                     B301.3     66030103 CM:
                                             CM<TAGS>                                                                           docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Reject - Unauthorized                                     B301.4     66030104 CM:
                                             SAID<TAGS>                                                                         docsIf3CmEventNotif
 BPKM       AUTH-FSM           Error         Auth Reject - Permanent                                        B301.8     66030108 CM:
                                             Authorization Failure<TAGS>                                                        docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Reject - Time of Day not                                  B301.9     66030109 CM:
                                             acquired<TAGS>                                                                     docsIf3CmEventNotif
 BPKM       AUTH-FSM           Informational Auth Reject - EAE                                              B301.10    66030110
                                             disabled<TAGS>
 BPKM       AUTH-FSM           Alert         CM Certificate Error<TAGS>                                     B301.11    66030111 CM:
                                                                                                                                docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Invalid - No                                              B302.2     66030202 CM:
                                             Information<TAGS>                                                                  docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Invalid - Unauthorized                                    B302.3     66030203 CM:
                                             CM<TAGS>                                                                           docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Invalid - Unsolicited<TAGS>                               B302.5     66030205 CM:
                                                                                                                                docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Invalid - Invalid Key                                     B302.6     66030206 CM:
                                             Sequence Number<TAGS>                                                              docsIf3CmEventNotif
 BPKM       AUTH-FSM           Warning       Auth Invalid - Message (Key                                    B302.7     66030207 CM:
                                             Request) Authentication                                                            docsIf3CmEventNotif
                                             Failure<TAGS>
 BPKM       AUTH-FSM           Warning       Unsupported Crypto                                             B303.0     66030300 CM:
                                             Suite<TAGS>                                                                        docsIf3CmEventNotif
 BPKM       AUTH-FSM           Informational Authorized<TAGS>                                               B401.0     66040100 CM:
                                                                                                                                docsIf3CmEventNotif
 BPKM       AUTH-FSM           Informational Auth Pend<TAGS>                                                B402.0     66040200 CM:
                                                                                                                                docsIf3CmEventNotif
 BPKM       AUTH-FSM           Informational Auth Comp<TAGS>                                                B403.0     66040300 CM:
                                                                                                                                docsIf3CmEventNotif




164                                                                  CableLabs                                                            04/29/26
<!-- PDF_PAGE: 165 -->
Cable Modem Operations Support System Interface Specification                                                         CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                    Event                                                    Error      Event       Notification
   Process       Sub-Process      Priority               Message                   Message Notes and Detail       Code Set     ID           Name
 BPKM         AUTH-FSM          Informational Stop<TAGS>                                                          B404.0     66040400 CM:
                                                                                                                                      docsIf3CmEventNotif
 BPKM         TEK-FSM           Warning       Key Reject - No                                                     B501.2     66050102 CM:
                                              Information<TAGS>                                                                       docsIf3CmEventNotif
 BPKM         TEK-FSM           Warning       Key Reject - Unauthorized                                           B501.3     66050103 CM:
                                              SAID<TAGS>                                                                              docsIf3CmEventNotif
 BPKM         TEK-FSM           Warning       TEK Invalid - No                                                    B502.3     66050203 CM:
                                              Information<TAGS>                                                                       docsIf3CmEventNotif
 BPKM         TEK-FSM           Warning       TEK Invalid - Invalid Key                                           B502.6     66050206 CM:
                                              Sequence Number<TAGS>                                                                   docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Informational SA Map State Machine                                                B601.0     66060100 CM:
                                              Started<TAGS>                                                                           docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Warning       Unsupported Crypto                                                  B602.0     66060200 CM:
                                              Suite<TAGS>                                                                             docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Error         Map Request Retry                                                   B603.0     66060300 CM:
                                              Timeout<TAGS>                                                                           docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Informational Unmap<TAGS>                                                         B604.0     66060400 CM:
                                                                                                                                      docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Informational Map Reject - Downstream Traffic                                     B605.10    66060510 CM:
                                              Flow Not Mapped to BPI+ SAID                                                            docsIf3CmEventNotif
                                              (EC=8)<TAGS>
 Dynamic SA   SA MAP-FSM        Warning       Map Reject - Not Authorized for                                     B605.9     66060509 CM:
                                              Requested Downstream Traffic                                                            docsIf3CmEventNotif
                                              Flow (EC=7)<TAGS>
 Dynamic SA   SA MAP-FSM        Warning       Mapped to Existing SAID<TAGS>                                       B606.0     66060600 CM:
                                                                                                                                      docsIf3CmEventNotif
 Dynamic SA   SA MAP-FSM        Warning       Mapped to New SAID<TAGS>                                            B607.0     66060700 CM:
                                                                                                                                      docsIf3CmEventNotif
                                                                       DBC and DCC
 DBC          DBC Request       Warning       CMTS Bad DBC - confirmation       P1=<Confirmation Code>            C501.0     67050100
                                              code <P1>: <P2><TAGS>             P2=<Confirmation> See Annex C.4
                                                                                Confirmation Code
 DBC          DBC Request       Warning       DBC- denied - confirmation code   P1=<Confirmation Code>            C502.0     67050200
                                              <P1>: <P2><TAGS>                  P2=<Confirmation> See Annex C.4
                                                                                Confirmation Code
 DBC          DBC Request       Warning       DBC-REQ Mismatch Between                                            C503.0     67050300
                                              Calculated Value for P1.6hi
                                              Compared to CCAP Provided
                                              Value<TAGS>




04/29/26                                                              CableLabs                                                                         165
<!-- PDF_PAGE: 166 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                              Data-Over-Cable Service Interface Specifications



                                      CM                   Event                                                       Error      Event       Notification
  Process      Sub-Process          Priority              Message                   Message Notes and Detail          Code Set     ID           Name
 DBC        DBC Acknowledgement   Error        DBC-ACK not received<TAGS>                                             C701.0     67070100
 DBC        DBC Acknowledgement   Notice       Bad CMTS DBC-ACK:                 P1="unspecified reason" | "unknown   C702.0     67070200
                                               <P1><TAGS>                        transaction ID" | "authentication
                                                                                 failure" | "msg syntax error"
 DCC        DCC Request           Error        DCC rejected already                                                   C201.0     67020100 CM:
                                               there<TAGS>                                                                                docsIf3CmEventNotif
 DCC        DCC Request           Informational DCC depart old<TAGS>                                                  C202.0     67020200 CM:
                                                                                                                                          docsIf3CmEventNotif
 DCC        DCC Request           Informational DCC arrive new<TAGS>                                                  C203.0     67020300 CM:
                                                                                                                                          docsIf3CmEventNotif
 DCC        DCC Request           Critical     DCC aborted unable to acquire                                          C204.0     67020400
                                               new downstream
                                               channel<TAGS>
 DCC        DCC Request           Critical     DCC aborted no UCD for new                                             C205.0     67020500
                                               upstream channel<TAGS>
 DCC        DCC Request           Critical     DCC aborted unable to                                                  C206.0     67020600
                                               communicate on new upstream
                                               channel<TAGS>
 DCC        DCC Request           Error        DCC rejected unspecified                                               C207.0     67020700 CM:
                                               reason<TAGS>                                                                               docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected permanent - DCC                                           C208.0     67020800 CM:
                                               not supported<TAGS>                                                                        docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected service flow not                                          C209.0     67020900 CM:
                                               found<TAGS>                                                                                docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected required parameter                                        C210.0     67021000 CM:
                                               not present<TAGS>                                                                          docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected authentication                                            C211.0     67021100 CM:
                                               failure<TAGS>                                                                              docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected multiple                                                  C212.0     67021200 CM:
                                               errors<TAGS>                                                                               docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected, duplicate SF                                             C215.0     67021500 CM:
                                               reference-ID or index in                                                                   docsIf3CmEventNotif
                                               message<TAGS>
 DCC        DCC Request           Error        DCC rejected parameter invalid                                         C216.0     67021600 CM:
                                               for context<TAGS>                                                                          docsIf3CmEventNotif
 DCC        DCC Request           Error        DCC rejected message syntax                                            C217.0     67021700 CM:
                                               error<TAGS>                                                                                docsIf3CmEventNotif




166                                                                    CableLabs                                                                    04/29/26
<!-- PDF_PAGE: 167 -->
Cable Modem Operations Support System Interface Specification                                                           CM-SP-CM-OSSIv4.0-I13-260429



                                       CM                   Event                                                    Error      Event       Notification
   Process      Sub-Process          Priority              Message                       Message Notes and Detail   Code Set     ID           Name
 DCC         DCC Request           Error        DCC rejected message too                                            C218.0     67021800 CM:
                                                big<TAGS>                                                                               docsIf3CmEventNotif
 DCC         DCC Request           Error        DCC rejected 2.0 mode                                               C219.0     67021900 CM:
                                                disabled<TAGS>                                                                          docsIf3CmEventNotif
 DCC         DCC Acknowledgement   Error        DCC-ACK not received<TAGS>                                          C401.0     67040100 CM:
                                                                                                                                        docsIf3CmEventNotif
 DCC         DCC Acknowledgement   Error        DCC-ACK rejected unspecified                                        C402.0     67040200 CM:
                                                reason<TAGS>                                                                            docsIf3CmEventNotif
 DCC         DCC Acknowledgement   Error        DCC-ACK rejected unknown                                            C403.0     67040300 CM:
                                                transaction ID<TAGS>                                                                    docsIf3CmEventNotif
 DCC         DCC Acknowledgement   Error        DCC-ACK rejected authentication                                     C404.0     67040400 CM:
                                                failure<TAGS>                                                                           docsIf3CmEventNotif
 DCC         DCC Acknowledgement   Error        DCC-ACK rejected message                                            C405.0     67040500 CM:
                                                syntax error<TAGS>                                                                      docsIf3CmEventNotif
                                                                      DHCP, TOD, and TFTP
 DHCP                              Error        DHCP RENEW sent - No                  P1=IPv4 or IPv6               D101.0     68010100
                                                response for <P1><TAGS>
 DHCP                              Error        DHCP REBIND sent - No                 P1=IPv4 or IPv6               D102.0     68010200
                                                response for <P1><TAGS>
 DHCP                              Error        DHCP RENEW WARNING - Field P1=v4                                    D103.0     68010300
                                                invalid in response <P1>
                                                option<TAGS>
 DHCP                              Critical     DHCP RENEW FAILED - Critical                                        D103.1     68010301
                                                field invalid in response
 DHCP                              Error        DHCP REBIND WARNING -                                               D104.0     68010400
                                                Field invalid in response <TAGS>
 DHCP                              Critical     DHCP REBIND FAILED - Critical                                       D104.1     68010401
                                                field invalid in response
 DHCP                              Notice       DHCP Reconfigure                                                    D105.0     68010500
                                                received<TAGS>
 DHCP                              Notice       DHCP Renew - lease parameters P1 = list of params that changed at   D106.0     68010600
                                                <P1> modified<TAGS>           renew
 DHCP                              Error        Primary lease failed, IPv4 fallback                                 D107.0     68010700
                                                initiated<TAGS>
 Init        DHCP                  Critical     DHCP FAILED - Discover sent,                                        D01.0      68000100
                                                no offer received<TAGS>
 Init        DHCP                  Critical     DHCP FAILED - Request sent,                                         D02.0      68000200
                                                No response<TAGS>




04/29/26                                                                 CableLabs                                                                        167
<!-- PDF_PAGE: 168 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                          Data-Over-Cable Service Interface Specifications



                                   CM                    Event                                                     Error      Event     Notification
   Process       Sub-Process     Priority               Message                   Message Notes and Detail        Code Set     ID         Name
 Init        DHCP              Warning      DHCP WARNING - Non-critical                                          D03.0       68000300
                                            field invalid in response <TAGS>
 Init        DHCP              Critical     DHCP FAILED - Critical field                                         D03.1       68000301
                                            invalid in response <TAGS>
 Init        DHCP              Critical     DHCP failed - RS sent, no RA                                         D12.0       68001200
                                            received<TAGS>
 Init        DHCP              Critical     DHCP Failed - Invalid                                                D12.1       68001201
                                            RA<TAGS>
 Init        DHCP              Critical     DHCP failed - DHCP Solicit sent,                                     D12.2       68001202
                                            No DHCP Advertise
                                            received<TAGS>
 Init        DHCP              Critical     DHCP failed - DHCP Request                                           D12.3       68001203
                                            sent, No DHCP REPLY
                                            received<TAGS>
 Init        DHCP              Error        Primary address acquired,                                            D12.4       68001204
                                            secondary failed<TAGS>
 Init        DHCP              Error        Primary address failed,                                              D12.5       68001205
                                            secondary active<TAGS>
 Init        IPv6 Address      Critical     Link-Local address failed                                            D13.1       68001301
             Acquisition                    DAD<TAGS>
 Init        IPv6 Address      Critical     DHCP lease address failed                                            D13.2       68001302
             Acquisition                    DAD<TAGS>
 Init        TOD               Warning      ToD request sent - No Response                                       D04.1       68000401
                                            received<TAGS>
 Init        TOD               Warning      ToD Response received - Invalid                                      D04.2       68000402
                                            data format<TAGS>
 Init        TFTP              Critical     TFTP failed - Request sent - No                                      D05.0       68000500
                                            Response<TAGS>
 Init        TFTP              Critical     TFTP failed - configuration file   For SYSLOG only: append: File name D06.0      68000600
                                            NOT FOUND<TAGS>                    = <P1> P1 = requested file name
 Init        TFTP              Critical     TFTP Failed - OUT OF ORDER                                           D07.0       68000700
                                            packets<TAGS>
 Init        TFTP              Critical     TFTP file complete - but failed    For SYSLOG only: append: File name D08.0      68000800
                                            Message Integrity check            = <P1> P1 = file name of TFTP file
                                            MIC<TAGS>
 Init        TFTP              Critical     TFTP file complete - but missing                                     D09.0       68000900
                                            mandatory TLV<TAGS>
 Init        TFTP              Critical     TFTP Failed - file too big<TAGS>                                     D10.0       68001000




168                                                                   CableLabs                                                              04/29/26
<!-- PDF_PAGE: 169 -->
Cable Modem Operations Support System Interface Specification                                                                  CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                    Event                                                             Error      Event       Notification
   Process      Sub-Process       Priority               Message                     Message Notes and Detail              Code Set     ID           Name
 Init        TFTP               Critical      TFTP Request Retries exceeded, For SYSLOG only: append: File name D11.1                 68001101
                                              CM unable to register          = <P1> P1 = file name of TFTP file
 Init        TFTP               Critical      Config File Rejected due to         P1 = First TLV encoding (e.g., 53, 54,   D11.2      68001102 CM:
                                              Invalid or Unexpected TLV <P1>      60, etc.) which caused config file                           docsIf3CmEventNotif
                                              encoding<TAGS>                      rejection
 TOD                            Error         ToD request sent - No Response                                               D04.3      68000403 CM:
                                              received<TAGS>                                                                                   docsIf3CmEventNotif
 TOD                            Error         ToD Response received - Invalid                                              D04.4      68000404 CM:
                                              data format<TAGS>                                                                                docsIf3CmEventNotif
                                                                        SSH Services
 SSH         SSH INIT           Information   SSH INIT via config file; Enabled   P1 = EnabledInterfaces of the            D500.00    68050000
 SERVICES                                     Interface: <P1>; Connection         SshServer object
                                              Timeout: <P2>; <TAGS>;              P2 = NewConnectionTimeout of the
                                                                                  SshServer object
 SSH         SSH OPERATIONS     Information   SSH service disabled for all new                                             D501.00    68050100
 SERVICES                                     connections, new connections
                                              timeout expired; <TAGS>;
 SSH         SSH OPERATIONS     Notice        SSH connection rejected, user       P1 = SSH Username                        D501.01    68050001
 SERVICES                                     credential error; User: <P1>;       P2 = User Credential Type
                                              Credential Type: <P2>; <TAGS>;
 SSH         SSH OPERATIONS     Information   SSH connection successful via       P1 = SSH Username                        D501.02    68050002
 SERVICES                                     user login; User: <P1>;             P2 = User Credential Type
                                              Credential Type: <P2>; <TAGS>;
 SSH         SSH OPERATIONS     Information   Existing SSH connection closed,     P1 = SSH Username                        D501.03    68050003
 SERVICES                                     inactivity timeout expired; User:
                                              <P1>; <TAGS>;
 SSH         SSH GENERAL        Warning       SSH connection rejected, SSH      P1 = SSH Username                          D502.00    68050200
 SERVICES    FAILURE                          protocol failure; User: <P1>; SSH P2 = SSH Error from
                                              Error: <P2>; <TAGS>;
                                                                                Table 82
 SSH         SSH Remote AUTH    Notice        SSH connection rejected, server P1 = SCCA REST API URL                       D503.00    68050300
 SERVICES    Failure                          URL misconfiguration; Auth       P2 = SSH Username
                                              Server: <P1>; User: <P2>; Config
                                                                               P3 = Config File Name
                                              File: <P3>; <TAGS>;
 SSH         SSH Remote AUTH    Notice        SSH connection rejected, TLS        P1 = Server Address                      D503.01    68050301
 SERVICES    Failure                          server certificate error; Auth      P2 = SSH Username
                                              Server: <P1>; User: <P2>; Cert
                                                                                   P3 = Certificate Error from
                                              Error: <P3>; Cert Level: <P4>;
                                              <TAGS>;                             Table 80
                                                                                  P4 = Certificate Level from Table 79




04/29/26                                                               CableLabs                                                                                 169
<!-- PDF_PAGE: 170 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                              Data-Over-Cable Service Interface Specifications



                                   CM                    Event                                                          Error      Event       Notification
  Process      Sub-Process       Priority               Message                    Message Notes and Detail            Code Set     ID           Name
 SSH        SSH Remote AUTH    Notice       SSH connection rejected, TLS       P1 = Server Address                 D503.02        68050302
 SERVICES   Failure                         protocol error; Auth Server: <P1>; P2 = SSH Username
                                            User: <P2>; TLS Error: <P3>;
                                                                                P3 = TLS Error from
                                            Retry Number: <P4>; <TAGS>;
                                                                               Table 81
                                                                               P4 = Maximum Retry number (default:
                                                                               3)
 SSH        SSH CDS Download   Warning      BPI+ EAE not used; Config File:     P1 = Config File Name                  D504.00    68050400
 SERVICES                                   <P1>; <TAGS>;
 SSH        SSH CDS Download   Notice       SSH connection rejected, server     P1 = CDS Download URL                  D504.01    68050401
 SERVICES                                   URL misconfiguration; CDS           P2 = Config File Name
                                            Server: <P1>; Config File: <P2>;
                                            <TAGS>;
 SSH        SSH CDS Download   Notice       SSH connection rejected, TLS        P1 = Server Address                    D504.02    68050402
 SERVICES                                   server certificate error; CDS        P2 = Certificate Error from
                                            Server: <P1>; Cert Error: <P2>;
                                                                                Table 80
                                            Cert Level: <P3>; <TAGS>;
                                                                                P3 = Certificate Level from Table 79
 SSH        SSH CDS Download   Notice       SSH connection rejected, TLS        P1 = Server Address                 D504.03       68050403
 SERVICES                                   protocol error; CDS Server:          P2 = TLS Error from
                                            <P1>; TLS Error: <P2>; Retry
                                                                                Table 81
                                            Number: <P3>; <TAGS>;
                                                                                P3 = Maximum Retry number (default:
                                                                                3)
 SSH        SSH CDS Download   Notice       CDS has been successfully           P1 = CDS Name                          D504.04    68050404
 SERVICES                                   downloaded; CDS: <P1>; CDS          P2 = SSH CDS Download URL
                                            Server: <P2>; <TAGS>;
 SSH        SSH CDS Download   Error        Authentication failure, unable to   P1 = CDS Name                          D504.05    68050405 docsIf3CmEventNotif
 SERVICES                                   decrypt CDS; CDS: <P1>; CDS         P2 = SSH CDS Download URL
                                            Server: <P2>; <TAGS>;
 SSH        SSH CDS Download   Error        Authentication failure, failed to   P1 = CDS Name                          D504.06    68050406 docsIf3CmEventNotif
 SERVICES                                   verify CDS signature; CDS:          P2 = SSH CDS Download URL
                                            <P1>; CDS Server: <P2>;
                                            <TAGS>;
 SSH        SSH CDS Download   Error        Authentication failure, unable to   P1 = CDS Name                          D504.07    68050407 docsIf3CmEventNotif
 SERVICES                                   install user credentials; CDS:      P2 = SSH CDS Download URL
                                            <P1>; CDS Server: <P2>;
                                            <TAGS>;
 SSH        SSH CDS Download   Notice       CDS has been successfully                                                  D504.08    68050408
 SERVICES                                   removed; <TAGS>;




170                                                                   CableLabs                                                                      04/29/26
<!-- PDF_PAGE: 171 -->
Cable Modem Operations Support System Interface Specification                                                           CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                   Event                                                       Error      Event       Notification
   Process      Sub-Process       Priority              Message                   Message Notes and Detail          Code Set     ID           Name
                                                               Secure Software Download
 SW Upgrade SW UPGRADE INIT     Notice       SW Download INIT - Via NMS        Other than Local Log, append:        E101.0     69010100 CM:
                                                                               SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                               P2><TAGS>

                                                                               P1 = SW file name
                                                                               P2 = SW Download server IP address
 SW Upgrade SW UPGRADE INIT     Notice       SW Download INIT - Via Config     Other than Local Log, append:        E102.0     69010200 CM:
                                             file <P1>                         SW file: <P2> - SW server: <                             docsIf3CmEventNotif
                                                                               P3><TAGS>

                                                                               P1 = CM config file name
                                                                               P2 = SW file name
                                                                               P3 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        SW Upgrade Failed during          Other than Local Log, append:        E103.0     69010300 CM:
            GENERAL FAILURE                  download - Max retry exceed (3)   SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                               P2><TAGS>

                                                                               P1 = SW file name
                                                                               P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        SW Upgrade Failed Before          Other than Local Log, append:        E104.0     69010400 CM:
            GENERAL FAILURE                  Download - Server not Present     SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                               P2><TAGS>

                                                                               P1 = SW file name
                                                                               P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        SW upgrade Failed before          Other than Local Log, append:        E105.0     69010500 CM:
            GENERAL FAILURE                  download - File not Present       SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                               P2><TAGS>

                                                                               P1 = SW file name
                                                                               P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        SW upgrade Failed before          Other than Local Log, append:        E106.0     69010600 CM:
            GENERAL FAILURE                  download - TFTP Max Retry         SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                             Exceeded                          P2><TAGS>

                                                                               P1 = SW file name
                                                                               P2 = SW Download server IP address




04/29/26                                                             CableLabs                                                                            171
<!-- PDF_PAGE: 172 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                           Data-Over-Cable Service Interface Specifications



                                   CM                   Event                                                       Error      Event       Notification
  Process      Sub-Process       Priority              Message                   Message Notes and Detail          Code Set     ID           Name
 SW Upgrade SW UPGRADE         Error        SW upgrade Failed after           Other than Local Log, append:        E107.0     69010700 CM:
            GENERAL FAILURE                 download -Incompatible SW file    SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Error        SW upgrade Failed after           Other than Local Log, append:        E108.0     69010800 CM:
            GENERAL FAILURE                 download - SW File corruption     SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Error        Disruption during SW download -   Other than Local Log, append:        E109.0     69010900 CM:
            GENERAL FAILURE                 Power Failure                     SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Error        Disruption during SW download -   Other than Local Log, append:        E110.0     69011000 CM:
            GENERAL FAILURE                 RF removed                        SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Notice       SW download Successful - Via      Other than Local Log, append:        E111.0     69011100 CM:
            SUCCESS                         NMS                               SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Notice       SW download Successful - Via      Other than Local Log, append:        E112.0     69011200 CM:
            SUCCESS                         Config file                       SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address
 SW Upgrade SW UPGRADE         Error        Improper Code File Controls       Other than Local Log, append:        E201.0     69020100 CM:
            GENERAL FAILURE                                                   SW file: <P1> - SW server: <                             docsIf3CmEventNotif
                                                                              P2><TAGS>

                                                                              P1 = SW file name
                                                                              P2 = SW Download server IP address




172                                                                CableLabs                                                                     04/29/26
<!-- PDF_PAGE: 173 -->
Cable Modem Operations Support System Interface Specification                                                                 CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                   Event                                                            Error      Event       Notification
   Process      Sub-Process       Priority              Message                    Message Notes and Detail              Code Set     ID           Name
 SW Upgrade SW UPGRADE          Error        Code File Manufacturer CVC         Other than Local Log, append:           E202.0      69020200 CM:
            GENERAL FAILURE                  Validation Failure                 SW file: <P1> - SW server: <                                 docsIf3CmEventNotif
                                                                                P2><TAGS>

                                                                                P1 = SW file name
                                                                                P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        Code File Manufacturer CVS         Other than Local Log, append:           E203.0      69020300 CM:
            GENERAL FAILURE                  Validation Failure                 SW file: <P1> - SW server: <                                 docsIf3CmEventNotif
                                                                                P2><TAGS>

                                                                                P1 = SW file name
                                                                                P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        Code File Co-Signer CVC            Other than Local Log, append:           E204.0      69020400 CM:
            GENERAL FAILURE                  Validation Failure                 SW file: <P1> - SW server: <                                 docsIf3CmEventNotif
                                                                                P2><TAGS>

                                                                                P1 = SW file name
                                                                                P2 = SW Download server IP address
 SW Upgrade SW UPGRADE          Error        Code File Co-Signer CVS            Other than Local Log, append:           E205.0      69020500 CM:
            GENERAL FAILURE                  Validation Failure                 SW file: <P1> - SW server: <                                 docsIf3CmEventNotif
                                                                                P2><TAGS>

                                                                                P1 = SW file name
                                                                                P2 = SW Download server IP address
 SW Upgrade VERIFICATION OF     Error        Improper Configuration File CVC    Other than Local Log, append:             E206.0    69020600 CM:
            CVC                              Format                             Config file: <P1> - Config file server: <                    docsIf3CmEventNotif
                                                                                P2><TAGS>

                                                                                P1 = Config file name
                                                                                P2 = Config file server IP address
 SW Upgrade VERIFICATION OF     Error        Configuration File CVC Validation Other than Local Log, append:             E207.0     69020700 CM:
            CVC                              Failure                           Config file: <P1> - Config file server: <                     docsIf3CmEventNotif
                                                                               P2><TAGS>

                                                                                P1 = Config file name
                                                                                P2 = Config file server IP address
 SW Upgrade VERIFICATION OF     Error        Improper SNMP CVC Format           Other than local Log, append: SNMP      E208.0      69020800 CM:
            CVC                                                                 Manager: <P1><TAGS>                                          docsIf3CmEventNotif

                                                                                P1= IP Address of SNMP Manager




04/29/26                                                             CableLabs                                                                                 173
<!-- PDF_PAGE: 174 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                                Data-Over-Cable Service Interface Specifications



                                        CM                   Event                                                        Error      Event       Notification
   Process       Sub-Process          Priority              Message                    Message Notes and Detail          Code Set     ID           Name
 SW Upgrade VERIFICATION OF         Error        SNMP CVC Validation Failure        Other than local Log, append: SNMP   E209.0     69020900 CM:
            CVC                                                                     Manager: <P1><TAGS>                                      docsIf3CmEventNotif

                                                                                    P1= IP Address of SNMP Manager
                                                                     Registration and TLV-11
 Init        REGISTRATION           Critical     REG-RSP - invalid format or not                                         I01.0      73000100
             RESPONSE                            recognized;<TAGS>
 Init        REGISTRATION           Critical     REG RSP not received<TAGS>                                              I02.0      73000200
             RESPONSE
 Init        REGISTRATION           Critical     REG RSP bad SID <P1><TAGS> P1 = SID which was reject                    I03.0      73000300
             RESPONSE
 Init        1.1 and 2.0 SPECIFIC   Critical     REG RSP contains service flow      P1 = Service Flow ID                 I251.0     73025100
             REGISTRATION                        parameters that CM cannot
             RESPONSE                            support <P1><TAGS>
 Init        1.1 and 2.0 SPECIFIC   Critical     REG RSP contains classifier        P1 = Service Flow ID                 I251.1     73025101
             REGISTRATION                        parameters that CM cannot
             RESPONSE                            support <P1><TAGS>
 Init        1.1 and 2.0 SPECIFIC   Critical     Registration RSP rejected                                               I251.3     73025103
             REGISTRATION                        unspecified reason<TAGS>
             RESPONSE
 Init        1.1 and 2.0 SPECIFIC   Critical     Registration RSP rejected          P1 = message                         I251.4     73025104
             REGISTRATION                        message syntax error
             RESPONSE                            <P1><TAGS>
 Init        1.1 and 2.0 SPECIFIC   Critical     Registration RSP rejected          P1 = # of characters                 I251.5     73025105
             REGISTRATION                        message too big <P1><TAGS>
             RESPONSE
 Init        2.0 SPECIFIC           Warning      REG-RSP received after REG-                                             I261.0     73026100
             REGISTRATION                        ACK. Returning to 1.x transmit
             RESPONSE                            mode<TAGS>
 Init        TLV-11 PARSING         Notice       TLV-11 - unrecognized                                                   I401.0     73040100 CM:
                                                 OID<TAGS>                                                                                   docsIf3CmEventNotif
 Init        TLV-11 PARSING         Critical     TLV-11 - Illegal Set operation                                          I402.0     73040200 CM:
                                                 failed<TAGS>                                                                                docsIf3CmEventNotif
 Init        TLV-11 PARSING         Critical     TLV-11 - Failed to set duplicate                                        I403.0     73040300 CM:
                                                 elements<TAGS>                                                                              docsIf3CmEventNotif
 Init        Waiting for REG-RSP or Error        T6 Timeout and retries                                                  I271.0     73027100
             REG-RSP-MP                          exceeded<TAGS>
 Init        CM Complete            Error        Cannot create US Primary                                                I501.0     73050100
             Registration                        Service Flow<TAGS>




174                                                                       CableLabs                                                                    04/29/26
<!-- PDF_PAGE: 175 -->
Cable Modem Operations Support System Interface Specification                                                         CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                   Event                                                    Error      Event     Notification
   Process      Sub-Process       Priority              Message                       Message Notes and Detail   Code Set     ID         Name
 Init        CM Complete        Notice       Received REG-RSP while in                                           I502.0     73050200
             Registration                    REG-HOLD1 state<TAGS>
 Init        CM Complete        Notice       Received REG-RSP while in                                           I503.0     73050300
             Registration                    REG-HOLD2 state<TAGS>
 Init        CM Complete        Warning      REG-RSP-MP Mismatch                                                 I504.0     73050400
             Registration                    Between Calculated Value for
                                             P1.6hi Compared to CCAP
                                             Provided Value<TAGS>
                                                                            General
 Init                           Informational A transmit opportunity was                                         N01.0      78000100
                                              missed because the MAP arrived
                                              too late.
                                                                         Ranging
 Init        RANGING            Critical     No Maintenance Broadcasts for                                       R01.0      82000100
                                             Ranging opportunities received -
                                             T2 time-out<TAGS>
 Init        RANGING            Critical     No Ranging Response received -                                      R02.0      82000200
                                             T3 time-out<TAGS>
 Init        RANGING            Critical     Ranging Request Retries                                             R03.0      82000300
                                             exhausted<TAGS>
 Init        RANGING            Critical     Received Response to Broadcast                                      R04.0      82000400
                                             Maintenance Request, But no
                                             Unicast Maintenance
                                             opportunities received - T4 time
                                             out<TAGS>
 Init        RANGING            Critical     Started Unicast Maintenance                                         R05.0      82000500
                                             Ranging - No Response received
                                             - T3 time-out<TAGS>
 Init        RANGING            Critical     Unicast Maintenance Ranging                                         R06.0      82000600
                                             attempted - No response -
                                             Retries exhausted<TAGS>
 Init        RANGING            Critical     Unicast Ranging Received Abort                                      R07.0      82000700
                                             Response - Re-initializing
                                             MAC<TAGS>
 Init        RANGING            Critical     16 consecutive T3 timeouts while    P1 = Upstream Channel ID        R08.0      82000800
                                             trying to range on upstream
                                             channel <P1><TAGS>
 Init        RANGING            Warning      B-INIT-RNG Failure - Retries                                        R09.0      82000900
                                             exceeded<TAGS>




04/29/26                                                             CableLabs                                                                       175
<!-- PDF_PAGE: 176 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                      Data-Over-Cable Service Interface Specifications



                                   CM                   Event                                                Error      Event       Notification
  Process      Sub-Process       Priority              Message                  Message Notes and Detail    Code Set     ID           Name
 Station     RANGING           Warning      RNG-RSP CCAP Commanded                                          R10.0      82001000
 Maintenance                                Power Exceeds Value for
                                            Pmax<TAGS>
 Station     RANGING           Warning      RNG-RSP CCAP Commanded                                          R11.0      82001100
 Maintenance                                Power Exceeds Value
                                            Corresponding to the Top of the
                                            DRW<TAGS>
 Station     RANGING           Warning      RNG-RSP CCAP Commanded                                          R12.0      82001200
 Maintenance                                Power in Excess of 6 dB Below
                                            the Value Corresponding to the
                                            Top of the DRW<TAGS>
                                                                    Dynamic Services
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected -                                          S01.0      83000100 CM:
 SERVICES   REQUEST                         Unspecified reason<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected -                                          S01.1      83000101 CM:
 SERVICES   REQUEST                         Unrecognized configuration                                                          docsIf3CmEventNotif
                                            setting<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Classifier                               S01.10     83000110 CM:
 SERVICES   REQUEST                         not found<TAGS>                                                                     docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Classifier                               S01.11     83000111 CM:
 SERVICES   REQUEST                         exists<TAGS>                                                                        docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Duplicated                               S01.14     83000114 CM:
 SERVICES   REQUEST                         reference-ID or index in                                                            docsIf3CmEventNotif
                                            message<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Multiple                                 S01.15     83000115 CM:
 SERVICES   REQUEST                         upstream flows<TAGS>                                                                docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Multiple                                 S01.16     83000116 CM:
 SERVICES   REQUEST                         downstream flows<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Classifier                               S01.17     83000117 CM:
 SERVICES   REQUEST                         for another flow<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Parameter                                S01.19     83000119 CM:
 SERVICES   REQUEST                         invalid for context<TAGS>                                                           docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Temporary                                S01.2      83000102 CM:
 SERVICES   REQUEST                         no resource<TAGS>                                                                   docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected -                                          S01.20     83000120 CM:
 SERVICES   REQUEST                         Authorization failure<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add rejected - Major                                    S01.21     83000121 CM:
 SERVICES   REQUEST                         service flow error<TAGS>                                                            docsIf3CmEventNotif




176                                                                  CableLabs                                                            04/29/26
<!-- PDF_PAGE: 177 -->
Cable Modem Operations Support System Interface Specification                                                  CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                   Event                                              Error      Event       Notification
   Process      Sub-Process       Priority              Message                 Message Notes and Detail   Code Set     ID           Name
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Major                                  S01.22     83000122 CM:
 SERVICES    REQUEST                         classifier error<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Multiple                               S01.24     83000124 CM:
 SERVICES    REQUEST                         major errors<TAGS>                                                                docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Message                                S01.25     83000125 CM:
 SERVICES    REQUEST                         syntax error<TAGS>                                                                docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Message                                S01.26     83000126 CM:
 SERVICES    REQUEST                         too big<TAGS>                                                                     docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Temporary                              S01.27     83000127 CM:
 SERVICES    REQUEST                         DCC<TAGS>                                                                         docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Permanent                              S01.3      83000103 CM:
 SERVICES    REQUEST                         administrative<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Required                               S01.4      83000104 CM:
 SERVICES    REQUEST                         parameter not present<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Header                                 S01.5      83000105 CM:
 SERVICES    REQUEST                         suppression setting not                                                           docsIf3CmEventNotif
                                             supported<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Service                                S01.6      83000106 CM:
 SERVICES    REQUEST                         flow exists<TAGS>                                                                 docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - HMAC                                   S01.7      83000107 CM:
 SERVICES    REQUEST                         Auth failure<TAGS>                                                                docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Add                                    S01.8      83000108 CM:
 SERVICES    REQUEST                         aborted<TAGS>                                                                     docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add rejected - Multiple                               S01.9      83000109 CM:
 SERVICES    REQUEST                         errors<TAGS>                                                                      docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                     S02.0      83000200 CM:
 SERVICES    REQUEST                         Unspecified reason<TAGS>                                                          docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                     S02.1      83000201 CM:
 SERVICES    REQUEST                         Unrecognized configuration                                                        docsIf3CmEventNotif
                                             setting<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                     S02.10     83000210 CM:
 SERVICES    REQUEST                         Classifier not found<TAGS>                                                        docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                     S02.11     83000211 CM:
 SERVICES    REQUEST                         Classifier exists<TAGS>                                                           docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                     S02.14     83000214 CM:
 SERVICES    REQUEST                         Duplicated reference-ID or index                                                  docsIf3CmEventNotif
                                             in message<TAGS>




04/29/26                                                             CableLabs                                                                   177
<!-- PDF_PAGE: 178 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                      Data-Over-Cable Service Interface Specifications



                                   CM                   Event                                                Error      Event       Notification
  Process      Sub-Process       Priority              Message                  Message Notes and Detail    Code Set     ID           Name
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.15     83000215 CM:
 SERVICES   REQUEST                         Multiple upstream flows<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.16     83000216 CM:
 SERVICES   REQUEST                         Multiple downstream                                                                 docsIf3CmEventNotif
                                            flows<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.17     83000217 CM:
 SERVICES   REQUEST                         Classifier for another                                                              docsIf3CmEventNotif
                                            flow<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected - Invalid                               S02.19     83000219 CM:
 SERVICES   REQUEST                         parameter for context<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.2      83000202 CM:
 SERVICES   REQUEST                         Temporary no resource<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.20     83000220 CM:
 SERVICES   REQUEST                         Authorization failure<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected - Major                                 S02.21     83000221 CM:
 SERVICES   REQUEST                         service flow error<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected - Major                                 S02.22     83000222 CM:
 SERVICES   REQUEST                         classifier error<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.24     83000224 CM:
 SERVICES   REQUEST                         Multiple major errors<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.25     83000225 CM:
 SERVICES   REQUEST                         Message syntax error<TAGS>                                                          docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.26     83000226 CM:
 SERVICES   REQUEST                         Message too big<TAGS>                                                               docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.27     83000227 CM:
 SERVICES   REQUEST                         Temporary DCC<TAGS>                                                                 docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.3      83000203 CM:
 SERVICES   REQUEST                         Permanent                                                                           docsIf3CmEventNotif
                                            administrative<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.4      83000204 CM:
 SERVICES   REQUEST                         Requester not owner of service                                                      docsIf3CmEventNotif
                                            flow<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.5      83000205 CM:
 SERVICES   REQUEST                         Service flow not found<TAGS>                                                        docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change rejected -                                       S02.6      83000206 CM:
 SERVICES   REQUEST                         Required parameter not                                                              docsIf3CmEventNotif
                                            present<TAGS>




178                                                                 CableLabs                                                             04/29/26
<!-- PDF_PAGE: 179 -->
Cable Modem Operations Support System Interface Specification                                                   CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                   Event                                               Error      Event       Notification
   Process      Sub-Process       Priority              Message                  Message Notes and Detail   Code Set     ID           Name
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                      S02.7      83000207 CM:
 SERVICES    REQUEST                         Header suppression setting not                                                     docsIf3CmEventNotif
                                             supported<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected - HMAC                                 S02.8      83000208 CM:
 SERVICES    REQUEST                         Auth failure<TAGS>                                                                 docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change rejected -                                      S02.9      83000209 CM:
 SERVICES    REQUEST                         Multiple errors<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete rejected -                                      S03.0      83000300 CM:
 SERVICES    REQUEST                         Unspecified reason<TAGS>                                                           docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete rejected -                                      S03.1      83000301 CM:
 SERVICES    REQUEST                         Requester not owner of service                                                     docsIf3CmEventNotif
                                             flow<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete rejected - Service                              S03.2      83000302 CM:
 SERVICES    REQUEST                         flow not found<TAGS>                                                               docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete rejected - HMAC                                 S03.3      83000303 CM:
 SERVICES    REQUEST                         Auth failure<TAGS>                                                                 docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete rejected -                                      S03.4      83000304 CM:
 SERVICES    REQUEST                         Message syntax error<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.0     83010100 CM:
 SERVICES    RESPONSE                        Invalid transaction ID<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add aborted - No                                       S101.1     83010101 CM:
 SERVICES    RESPONSE                        RSP<TAGS>                                                                          docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.11    83010111 CM:
 SERVICES    RESPONSE                        Duplicate reference_ID or index                                                    docsIf3CmEventNotif
                                             in message<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.12    83010112 CM:
 SERVICES    RESPONSE                        Classifier for another                                                             docsIf3CmEventNotif
                                             flow<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.13    83010113 CM:
 SERVICES    RESPONSE                        Parameter invalid for                                                              docsIf3CmEventNotif
                                             context<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.14    83010114 CM:
 SERVICES    RESPONSE                        Major service flow error<TAGS>                                                     docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.15    83010115 CM:
 SERVICES    RESPONSE                        Major classifier error<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.17    83010117 CM:
 SERVICES    RESPONSE                        Multiple major errors<TAGS>                                                        docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Add Response rejected -                                S101.18    83010118 CM:
 SERVICES    RESPONSE                        Message too big<TAGS>                                                              docsIf3CmEventNotif




04/29/26                                                             CableLabs                                                                    179
<!-- PDF_PAGE: 180 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                       Data-Over-Cable Service Interface Specifications



                                   CM                    Event                                                Error      Event       Notification
  Process      Sub-Process       Priority               Message                  Message Notes and Detail    Code Set     ID           Name
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.2     83010102 CM:
 SERVICES   RESPONSE                        HMAC Auth failure<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.3     83010103 CM:
 SERVICES   RESPONSE                        Message syntax error<TAGS>                                                           docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.4     83010104 CM:
 SERVICES   RESPONSE                        Unspecified reason<TAGS>                                                             docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.5     83010105 CM:
 SERVICES   RESPONSE                        Unrecognized configuration                                                           docsIf3CmEventNotif
                                            setting<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.6     83010106 CM:
 SERVICES   RESPONSE                        Required parameter not                                                               docsIf3CmEventNotif
                                            present<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.7     83010107 CM:
 SERVICES   RESPONSE                        Service Flow exists<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.8     83010108 CM:
 SERVICES   RESPONSE                        Multiple errors<TAGS>                                                                docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Add Response rejected -                                  S101.9     83010109 CM:
 SERVICES   RESPONSE                        Classifier exists<TAGS>                                                              docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.0     83010200 CM:
 SERVICES   RESPONSE                        rejected - Invalid transaction                                                       docsIf3CmEventNotif
                                            ID<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change aborted - No                                      S102.1     83010201 CM:
 SERVICES   RESPONSE                        RSP<TAGS>                                                                            docsIf3CmEventNotif
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.10    83010210 CM:
 SERVICES   RESPONSE                        rejected - Duplicated reference-ID                                                   docsIf3CmEventNotif
                                            or index in<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.11    83010211 CM:
 SERVICES   RESPONSE                        rejected - Invalid parameter for                                                     docsIf3CmEventNotif
                                            context<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.12    83010212 CM:
 SERVICES   RESPONSE                        rejected - Major classifier                                                          docsIf3CmEventNotif
                                            error<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.14    83010214 CM:
 SERVICES   RESPONSE                        rejected - Multiple Major                                                            docsIf3CmEventNotif
                                            errors<TAGS>
 DYNAMIC    DYNAMIC SERVICE    Error        Service Change Response                                          S102.15    83010215 CM:
 SERVICES   RESPONSE                        rejected - Message too                                                               docsIf3CmEventNotif
                                            big<TAGS>




180                                                                   CableLabs                                                            04/29/26
<!-- PDF_PAGE: 181 -->
Cable Modem Operations Support System Interface Specification                                                   CM-SP-CM-OSSIv4.0-I13-260429



                                    CM                    Event                                              Error      Event       Notification
   Process      Sub-Process       Priority               Message                 Message Notes and Detail   Code Set     ID           Name
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.2     83010202 CM:
 SERVICES    RESPONSE                        rejected - HMAC Auth                                                               docsIf3CmEventNotif
                                             failure<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.3     83010203 CM:
 SERVICES    RESPONSE                        rejected - Message syntax                                                          docsIf3CmEventNotif
                                             error<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.4     83010204 CM:
 SERVICES    RESPONSE                        rejected - Unspecified                                                             docsIf3CmEventNotif
                                             reason<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.5     83010205 CM:
 SERVICES    RESPONSE                        rejected - Unrecognized                                                            docsIf3CmEventNotif
                                             configuration setting<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.6     83010206 CM:
 SERVICES    RESPONSE                        rejected - Required parameter not                                                  docsIf3CmEventNotif
                                             present<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.7     83010207 CM:
 SERVICES    RESPONSE                        rejected - Multiple errors<TAGS>                                                   docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE    Error        Service Change Response                                        S102.8     83010208 CM:
 SERVICES    RESPONSE                        rejected - Classifier                                                              docsIf3CmEventNotif
                                             exists<TAGS>
 DYNAMIC     DYNAMIC SERVICE    Error        Service Delete Response                                        S103.0     83010300 CM:
 SERVICES    RESPONSE                        rejected - Invalid transaction                                                     docsIf3CmEventNotif
                                             ID<TAGS>
 DYNAMIC     DYNAMIC SERVICE Error           Service Add Response rejected -                                S201.0     83020100 CM:
 SERVICES    ACKNOWLEDGEMENT                 Invalid Transaction ID<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Add Aborted - No                                       S201.1     83020101 CM:
 SERVICES    ACKNOWLEDGEMENT                 ACK<TAGS>                                                                          docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Add ACK rejected -                                     S201.2     83020102 CM:
 SERVICES    ACKNOWLEDGEMENT                 HMAC auth failure<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Add ACK rejected -                                     S201.3     83020103 CM:
 SERVICES    ACKNOWLEDGEMENT                 Message syntax error<TAGS>                                                         docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Change ACK rejected -                                  S202.0     83020200 CM:
 SERVICES    ACKNOWLEDGEMENT                 Invalid transaction ID<TAGS>                                                       docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Change Aborted - No                                    S202.1     83020201 CM:
 SERVICES    ACKNOWLEDGEMENT                 ACK<TAGS>                                                                          docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Change ACK rejected -                                  S202.2     83020202 CM:
 SERVICES    ACKNOWLEDGEMENT                 HMAC Auth failure<TAGS>                                                            docsIf3CmEventNotif
 DYNAMIC     DYNAMIC SERVICE Error           Service Change ACK rejected -                                  S202.3     83020203 CM:
 SERVICES    ACKNOWLEDGEMENT                 Message syntax error<TAGS>                                                         docsIf3CmEventNotif




04/29/26                                                               CableLabs                                                                  181
<!-- PDF_PAGE: 182 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                      Data-Over-Cable Service Interface Specifications



                                   CM                   Event                                                Error      Event      Notification
   Process      Sub-Process      Priority              Message                  Message Notes and Detail    Code Set     ID          Name
                                                                  Channel Acquisition
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T01.0      84000100
             ACQUISITION                    failure - Failed to acquire
                                            QAM/QPSK symbol
                                            timing;<TAGS>
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T02.0      84000200
             ACQUISITION                    failure - Failed to acquire FEC
                                            framing<TAGS>
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T02.1      84000201
             ACQUISITION                    failure, Acquired FEC framing -
                                            Failed to acquire MPEG2
                                            Sync<TAGS>
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T03.0      84000300
             ACQUISITION                    failure - Failed to acquire MAC
                                            framing<TAGS>
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T04.0      84000400
             ACQUISITION                    failure - Failed to receive MAC
                                            SYNC frame within time-out
                                            period<TAGS>
 Init        DOWNSTREAM        Critical     SYNC Timing Synchronization                                     T05.0      84000500
             ACQUISITION                    failure - Loss of Sync<TAGS>
 Init        DOWNSTREAM        Error        RCS Primary DS Failure<TAGS>                                    T06.0      84000600
             ACQUISITION
 Init        DOWNSTREAM        Warning      RCS Partial Service<TAGS>                                       T07.0      84000700
             ACQUISITION
 Init        RCP and RCC       Error        RCP-ID in RCC not                                               T101.0     84010100
                                            supported<TAGS>
 Init        RCP and RCC       Error        More than one RCP-ID included                                   T102.0     84010200
                                            in RCC<TAGS>
 Init        RCP and RCC       Error        Receive Module Index missing in                                 T103.0     84010300
                                            RCC<TAGS>
 Init        RCP and RCC       Error        RCC contains a Receive Module                                   T104.0     84010400
                                            Index which is not
                                            supported<TAGS>
 Init        RCP and RCC       Error        Receive channel center                                          T105.0     84010500
                                            frequency not within allowed
                                            range of center frequencies for
                                            Receive Module<TAGS>




182                                                                  CableLabs                                                          04/29/26
<!-- PDF_PAGE: 183 -->
Cable Modem Operations Support System Interface Specification                                                     CM-SP-CM-OSSIv4.0-I13-260429



                                       CM                   Event                                             Error      Event     Notification
   Process       Sub-Process         Priority              Message                Message Notes and Detail   Code Set     ID         Name
 Init        RCP and RCC           Error        Receive Module first channel                                 T106.0     84010600
                                                center frequency not within
                                                allowed range of center
                                                frequencies<TAGS>
 Init        RCP and RCC           Error        Receive Module first channel                                 T107.0     84010700
                                                center frequency not present in
                                                RCC<TAGS>
 Init        RCP and RCC           Error        No primary downstream channel                                T108.0     84010800
                                                assignment in RCC<TAGS>
 Init        RCP and RCC           Error        More than one primary                                        T109.0     84010900
                                                downstream channel assignment
                                                present in RCC<TAGS>
 Init        RCP and RCC           Error        Receive Module connectivity                                  T110.0     84011000
                                                encoding in RCC Requires
                                                configuration not
                                                supported<TAGS>
 Init        RCP and RCC           Error        Receive channel index in RCC                                 T111.0     84011100
                                                not supported by CM<TAGS>
 Init        RCP and RCC           Error        Center frequency in RCC not a                                T112.0     84011200
                                                multiple of 62500 Hz<TAGS>
 Init        MDD                   Error        Missing Mandatory MDD TLV on                                 T201.0     84020100
                                                primary DS Channel<TAGS>
 Init        MDD                   Warning      Lost MDD Timeout<TAGS>                                       T202.0     84020200
 Init        MDD                   Warning      MDD message timeout<TAGS>                                    T203.0     84020300
 Init        OBTAIN UPSTREAM       Critical     No UCDs Received -                                           U01.0      85000100
             PARAMETERS                         Timeout;<TAGS>
 Init        OBTAIN UPSTREAM       Critical     UCD invalid or channel                                       U02.0      85000200
             PARAMETERS                         unusable<TAGS>
 Init        OBTAIN UPSTREAM       Critical     UCD & SYNC valid - NO MAPS                                   U04.0      85000400
             PARAMETERS                         for this channel<TAGS>
 Init        OBTAIN UPSTREAM       Critical     US channel wide parameters not                               U06.0      85000600
             PARAMETERS                         set before Burst
                                                Descriptors<TAGS>
 Init        Acquire CM Transmit   Error        TCS Fail on all Upstream                                     U101.0     85010100
             Channels                           Channels<TAGS>
 Init        Acquire CM Transmit   Warning      TCS Partial Service<TAGS>                                    U102.0     85010200
             Channels




04/29/26                                                                 CableLabs                                                               183
<!-- PDF_PAGE: 184 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                                 Data-Over-Cable Service Interface Specifications



                                       CM                    Event                                                       Error      Event       Notification
   Process       Sub-Process         Priority               Message                    Message Notes and Detail         Code Set     ID           Name
 Init        Acquire CM Transmit   Warning       Initializing Channel Timeout                                           U103.0     85010300
             Channels                            Expires Time the CM can perform
                                                 initial ranging on all upstream
                                                 channels in the TCS has
                                                 expired<TAGS>
 Init        Acquire CM Extended   Warning       Extended TCS Partial Service;     P1 = Channel ID                      U102.1     85010201
             Transmit Channels                   Chan ID: <P1>; Mode: <P2>;        P2 = "FDX" | "FDD"
                                                 <TAGS>;
 Init        Acquire CM Extended   Error         Extended TCS fail on all          P1 = Channel ID                      U101.1     85010101
             Transmit Channels                   upstream channels; Chan ID:       P2 = "FDX" | "FDD"
                                                 <P1>; Mode: <P2>; <TAGS>;
                                                                             CM-CTRL
 CM-CTRL     CM-CTRL               Debug         CM-CTRL - Command: <P1>           P1 =                                 L01.0      76000100 CM:
                                                 (if P1= mute                      mute, or                                                 docsIf3CmEventNotif
                                                 Add                               cmReinit, or
                                                 Interval: <P2>                    forwarding
                                                 ChannelID: <P3>)                  P2= mute interval,
                                                 (If P1 = forwarding               Value 0 indicate
                                                 Add                               unmute operation
                                                 Action: <P4>)                     P3= Channel ID or 0
                                                 <TAGS>
                                                                                   P4 = enable, or disable
 CM-CTRL     CM-CTRL               Debug         CM-CTRL- Invalid message                                               L02.0      76000200 CM:
                                                 format<TAGS>                                                                               docsIf3CmEventNotif
                                                                      Energy Management
 EM          EM-REQ                Informational EM-RSP not received<TAGS>                                              L101.0     76010100
 EM          EM-REQ                Warning       EM-REQ retries                                                         L102.0     76010200
                                                 exhausted<TAGS>
 EM          EM-REQ                Informational EM-RSP received, Reject           <P1> = time to defer (seconds)       L103.0     76010300
                                                 Temporary, deferring for <P1>
                                                 seconds<TAGS>
 EM          EM-REQ                Warning       EM-RSP received, Reject                                                L104.0     76010400
                                                 Permanent<TAGS>
 EM          DBC                   Informational CM entered EM 1x1 mode;           P1=Unknown, Activity Detection,      L113.0     76011300 CM:
                                                 Reason: <P1><TAGS>                eSAFE, CMTS Initiated                                    docsIf3CmEventNotif
 EM          DBC                   Informational CM exited EM 1x1 mode<TAGS>                                            L114.0     76011400 CM:
                                                                                                                                            docsIf3CmEventNotif
 EM          Activity Detection    Informational EM 1x1 Activity Detection         P1=Upstream entry, Downstream         L115.0    76011500 CM:
                                                 Threshold crossed;                entry, Upstream exit, Downstream exit                    docsIf3CmEventNotif
                                                 Reason:<P1><TAGS>




184                                                                      CableLabs                                                                    04/29/26
<!-- PDF_PAGE: 185 -->
Cable Modem Operations Support System Interface Specification                                                                  CM-SP-CM-OSSIv4.0-I13-260429



                                     CM                   Event                                                           Error      Event       Notification
   Process         Sub-Process     Priority              Message                   Message Notes and Detail              Code Set     ID           Name
 EM          EM-REQ              Informational EM-REQ Sent<TAGS>                                                         L116.0     76011600
                                                                        CM-STATUS
 Status      Status              Notice       CM-STATUS message sent.           P1: CM-STATUS event type code.            J101.0    74010100 CM:
                                              Event Type Code: <P1>; Chan       P2: Upstream/Downstream channel                              docsIf3CmEventNotif
                                              ID: <P2>; DSID: <P3>; MAC         ID; "N/A" if not provided for this status
                                              Addr: <P4>; OFDM/OFDMA            message type.
                                              Profile ID: <P5>.<TAGS>           P3: DSID; "N/A" if not provided for this
                                                                                status message type.
                                                                                P4: MAC Address; "N/A" if not
                                                                                provided for this status message type.
                                                                                P5: OFDM or OFDMA Profile ID;
                                                                                "N/A" if not provided for this status
                                                                                message type.
                                                                      Profile Change
 Profile     Profile Change      Notice       DS profile assignment change.     P1: Downstream channel ID                C616.0     67061600 CM:
                                              DS Chan ID: <P1>; Previous        P2: Previous OFDM Profile ID                                 docsIf3CmEventNotif
                                              Profile: <P2>; New Profile:
                                                                                P3: New OFDM Profile ID
                                              <P3>.<TAGS>
 Profile     Profile Change      Notice       US profile assignment change.     P1: Upstream channel ID                  C616.1     67061601 CM:
                                              US Chan ID: <P1>; Previous        P2: Previous OFDMA Profile ID                                docsIf3CmEventNotif
                                              Profile: <P2>; New Profile:
                                                                                P3: New OFDMA Profile ID
                                              <P3>.<TAGS>
 Profile     Profile Update      Info         DS profile config update. DS      P1: Downstream channel ID                C616.2     67061602 CM:
                                              Chan ID: <P1>.<TAGS>                                                                           docsIf3CmEventNotif
 Profile     Profile Update      Info         US profile config update. US      P1: Upstream channel ID                  C616.3     67061603 CM:
                                              Chan ID: <P1>.<TAGS>                                                                           docsIf3CmEventNotif
                                                                     RxMER Reporting
 RxMER       Cancelled Request   Error        DBC or Primary Backup interface                                            C860.0     86010100
                                              Index modified

                                                                 FDX Extended Spectrum

 FDX         ECT                 Info         Echo Cancellation Training        P1: RBA Subband Direction Set from       X000.01    90000001
                                              Converged; RBA Subband            CmFdxStatus::RbaSubbandDirection
                                              Direction Set: <P1>;<TAGS>;       Set

 FDX         ECT                 Warning      Loss of Echo Cancellation         P1: RBA Subband Direction Set from       x.000.02   90000002
                                              Convergence; RBA Subband
                                                                                CmFdxStatus::RbaSubbandDirection
                                              Direction Set: <P1>; ECT
                                                                                Set P2 = "Initial" | "Periodic"
                                              Method: <P2>;<TAGS>;




04/29/26                                                              CableLabs                                                                                185
<!-- PDF_PAGE: 186 -->
CM-SP-CM-OSSIv4.0-I13-260429                                                                                           Data-Over-Cable Service Interface Specifications



                                                 CM                    Event                                                          Error          Event    Notification
   Process         Sub-Process                 Priority               Message                  Message Notes and Detail              Code Set         ID        Name
  FDX          Reinitialization            Notice         FDX Reset; RBA Subband           P1: RBA Subband Direction Set from        x.001.00      90000100
                                                          Direction Set: <P1>;<TAGS>;      CmFdxStatus::RbaSubbandDirection
                                                                                           Set

The following tables contain parameters for security events found in Table 78.
                                                                    Table 79 - Certificate Level Parameters

                                               Error Strings                                             Extended Description
                         End Entity Certificate                                 The error generated while validating the end entity certificate
                         Intermediate CA certificate                            The error generated while validating the intermediate CA certificate
                         Root CA certificate                                    The error generated while validating the root CA certificate


                                                                    Table 80 - Certificate Error Parameters

                                               Error Strings                                             Extended Description
                         Incomplete DOCSIS 4.0 Certificate chain                Certificate is missing
                         Certificate is expired                                 Certificate is expired
                         Certificate is revoked                                 Certificate is revoked via CRL
                                                                                Certificate is revoked via OCSP
                         Cannot fetch revocation info                           Missing URL in certificate or configuration
                                                                                Misconfigured URL or network unavailability
                         Certificate has Invalid format                         Certificate has improper format
                         Certificate is not trusted                             Certificate does not chain to a valid trusted root
                         Domain mismatch                                        The certificate does not contain the FQDN or the server IP address
                         Certificate public key algorithm is not supported      The algorithm of the public key inside this certificate is not supported


                                                                  Table 81 - TLS Protocol Error Parameters

                                               Error Strings                                             Extended Description
                         TLS version incompatibility                            SSH client and server use incompatible TLS version
                         TLS cipher suites incompatibility                      SSH client and server use incompatible cipher suites




186                                                                             CableLabs                                                                          04/29/26
<!-- PDF_PAGE: 187 -->
Cable Modem Operations Support System Interface Specification                                                                            CM-SP-CM-OSSIv4.0-I13-260429


                                                                 Table 82 - SSH Protocol Error Parameters

                                              Error Strings                                              Extended Description
                        SSH version incompatibility                               SSH client and server use incompatible SSH version
                        SSH cipher suites incompatibility                         SSH client and server use incompatible cipher suites



C.1 Deprecated Events
Table 83 in this annex lists deprecated events, including any associated syslog and SNMP trap notifications for the events, for a DOCSIS 4.0-compliant CM.
Implementation of deprecated events is optional.
                                                                         Table 83 - Deprecated Events

                                       CM                           Event                             Message Notes                Error      Event        Notification
  Process         Sub-Process        Priority                      Message                              and Detail                Code Set     ID            Name
                                                                          Authentication and Encryption
  Init (BPI+)   DOCSIS 1.0         Error           Missing BP Configuration Setting TLV        P1 = missing required TLV Type B101.0         66010100 CM:
                CONFIG FILE                        Type: <P1><TAGS>                                                                                   docsIf3CmEventNotif
  Init (BPI+)   DOCSIS 1.0         Alert           Invalid BP Configuration Setting Value:     P1=The TLV Value for P2.         B102.0       66010200 CM:
                CONFIG FILE                        <P1> for Type: <P2><TAGS>                   P2 = The first Configuration TLV                       docsIf3CmEventNotif
                                                                                               Type that contain invalid value.
                                                                                        UCC
  UCC           UCC Request        Error           UCC-REQ received with invalid or out of                                       C01.0       67000100
                                                   range US channel ID<TAGS>
  UCC           UCC Request        Error           UCC-REQ received unable to send UCC-                                          C02.0       67000200
                                                   RSP<TAGS>
                                                                              DHCP, TOD, and TFTP
  Init          TFTP               Critical        TFTP file complete- but does not enable     For SYSLOG only: append: File     D11.0       68001100
                                                   2.0 Mode - conflicts with current US        name = <P1> P1 = file name of
                                                   channel type<TAGS>                          TFTP file




04/29/26                                                                          CableLabs                                                                              187
