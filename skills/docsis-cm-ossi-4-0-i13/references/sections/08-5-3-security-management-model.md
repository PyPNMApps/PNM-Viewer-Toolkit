# 8.5.3 - Security Management UML Information Model

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 86-111
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 86 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


8.5.3     Security Management UML Information Model
This section defines the UML Information Model for CM security management functions.
The CM objects for Security Management are shown in Figure 8.




                             Figure 8 - CM Security Management Information Model


8.5.3.1    DocsSec
The DocsSec object serves as the root of the CM Security Management Information Model.
                                      Table 39 - DocsSec Object Associations

  Associated Object                                                       Near-End        Far-End
       Name                                   Type                        Multiplicity   Multiplicity   Label
 BpiPlusManagement       Directed composition to BpiPlusManagement            1              1
 CertificateManagement   Directed composition to CertificateManagement        1              1
 SsdManagement           Directed composition to SsdManagement                1              1
 SshKeyManagement        Directed composition to SshKeyManagement             1              1




86                                                   CableLabs                                         04/29/26
<!-- PDF_PAGE: 87 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv4.0-I13-260429


8.5.3.2    BpiPlusManagement
The BpiPlusManagement object describes the control of the BPI+V1 and V2 functions.
References: [RFC 4131]
                                Table 40 - BpiPlusManagement Object Associations

  Associated Object                                                            Near-End        Far-End
       Name                                   Type                             Multiplicity   Multiplicity   Label
 BpiPlusBase             Directed composition to BpiPlusBase                        1                1
 BpiPlusTek              Directed composition to BpiPlusTek                         1             1..*
 BpiPlusV2Cfg            Directed composition to BpiPlusV2Cfg                       1                1
 BpiPlusCryptoSuites     Directed composition to BpiPlusCryptoSuites                1             1..*



8.5.3.3    BpiPlusBase
The BpiPlusBase object describes the basic and authorization-related BPI+ attributes.
References: docsBpi2CmBaseTable [RFC 4131]
                                      Table 41 - BpiPlusBase Object Attributes

                                                                                                             Default
    Attribute Name               Type         Access               Type Constraints                Units      Value
CmPrivacyEnable           Boolean            R/O                                               N/A
CmPublicKey               HexBinary          R/O          SIZE (0..524)                        N/A
CmAuthState               Enum               R/O          start(1),                            N/A
                                                          authWait(2),
                                                          authorized(3),
                                                          reauthWait(4),
                                                          authRejectWait(5),
                                                          silent(6)
CmAuthKeySeqNum           Int                R/O          0..15                                N/A
CmAuthExpiresOld          DateTime           R/O          SIZE(11)                             N/A
CmAuthExpiresNew          DateTime           R/O          SIZE(11)                             N/A
CmAuthReset               Boolean            R/W                                               N/A
CmAuthGraceTime           Int                R/O          1..6047999                           seconds
CmTekGraceTime            Int                R/O          1..302399                            seconds
CmAuthWaitTimeout         Int                R/O          1..30                                seconds
CmReauthWaitTimeout       Int                R/O          1..30                                seconds
CmOpWaitTimeout           Int                R/O          1..10                                seconds
CmRekeyWaitTimeout        Int                R/O          1..10                                seconds
CmAuthRejectWaitTimeout Int                  R/O          1..600                               seconds
CmSAMapWaitTimeout        Int                R/O          1..10                                seconds
CmSAMapMaxRetries         Int                R/O          1..10                                retries
CmAuthInfos               Counter32          R/O                                               N/A
CmAuthRequests            Counter32          R/O                                               N/A
CmAuthReplies             Counter32          R/O                                               N/A
CmAuthRejects             Counter32          R/O                                               N/A
CmAuthInvalids            Counter32          R/O                                               N/A




04/29/26                                             CableLabs                                                        87
<!-- PDF_PAGE: 88 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


                                                                                                               Default
     Attribute Name                 Type       Access                Type Constraints             Units         Value
CmAuthRejectErrorCode      Enum              R/O          none(1),                             N/A
                                                          unknown(2),
                                                          unauthorizedCm(3),
                                                          unauthorizedSaid(4),
                                                          permanentAuthorizationFailure(8),
                                                          timeOfDayNotAcquired(11),
                                                          eaeDisabled(12),
                                                          unsupportedBpiVer(13)
CmAuthRejectErrorString    String            R/O          SIZE (0..128)                        N/A
CmAuthInvalidErrorCode     Enum              R/O          none(1),                             N/A
                                                          unknown(2),
                                                          unauthorizedCm(3),
                                                          unsolicited(5),
                                                          invalidKeySequence(6),
                                                          keyRequestAuthenticationFailure(7)
CmAuthInvalidErrorString   String            R/O          SIZE (0..128)                        N/A
CmBpiPlusVer               Enum              R/O          none(1),                             N/A
                                                          v1(2),
                                                          v2(3)
CmAuthStatusInfo           UnsignedByte      R/O                                               N/A



8.5.3.3.1      CmPrivacyEnable
This attribute identifies whether this CM is provisioned to run Baseline Privacy Plus.
References: docsBpi2CmPrivacyEnable [RFC 4131]

8.5.3.3.2      CmPublicKey
The value of this attribute is a DER-encoded RSAPublicKey ASN.1 type string, as defined in [X.509], corresponding
to the public key of the CM. This is the zero-length value if the CMTS does not retain the public key. This attribute
is only used in BPI+V1 mode.
References: docsBpi2CmPublicKey [RFC 4131]

8.5.3.3.3      CmAuthState
The value of this attribute is the state of the CM authorization FSM. The start state indicates that FSM is in its initial
state. The possible values for this attribute are listed below.
     •   'start' - This is the initial state of the FSM. No resources are assigned to or used by the FSM, all timers are
         off, and no processing is scheduled.
     •   'authWait' - This is the state of the FSM when the CM has sent both an Authentication Information and an
         Authorize Request message and is waiting for the reply.
     •   'authorized' - This is the state of the FSM when the CM has received an Authorization Reply message and
         has a valid Authorization Key and the list of SAIDs. Transition into this state triggers the creation of one
         TEK FSM for each of the CM's privacy-enabled SAIDs.
     •   'reauthWait' - This is the state of the FSM when the CM has sent an outstanding reauthorization request to
         CMTS and is waiting for a response.
     •   'authRejectWait' - This is the state of the FSM when the CM has received an Authorization Reject message
         in response to its last Authorization Request and the error code indicated that the error was NOT permanent
         or that EAE is not disabled.



88                                                   CableLabs                                                 04/29/26
<!-- PDF_PAGE: 89 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


    •    'silent' - This is the state of the FSM when the CM has received an Authorization Reject message in
         response to its last Authorization Request and the error code indicated that the error was permanent (e.g.,
         Device Certificate is revoked) or the CM and CMTS fail to properly authenticate.
References: docsBpi2CmAuthState [RFC 4131]

8.5.3.3.4     CmAuthKeySeqNum
The value of this attribute is the most recent authorization key sequence number for the CM authorization FSM.
References: docsBpi2CmAuthKeySequenceNumber [RFC 4131]

8.5.3.3.5     CmAuthExpiresOld
The value of this attribute is the actual clock time for expiration of the immediate predecessor of the most recent
authorization key for the CM authorization FSM. If this FSM has only one authorization key, then the value is the
time of activation of this FSM.
References: docsBpi2CmAuthExpiresOld [RFC 4131]

8.5.3.3.6     CmAuthExpiresNew
The value of this attribute is the actual clock time for expiration of the most recent authorization key for this FSM.
References: docsBpi2CmAuthExpiresNew [RFC 4131]

8.5.3.3.7     CmAuthReset
When the value of this attribute is 'true', the CM, generates a Reauthorize event in the CM authorization FSM.
References: docsBpi2CmAuthReset [RFC 4131]

8.5.3.3.8     CmAuthGraceTime
The value of this attribute is the grace time for an authorization key in seconds. A CM is expected to start trying to
get a new authorization key beginning AuthGraceTime seconds before the most recent authorization key actually
expires.
References: docsBpi2CmAuthGraceTime [RFC 4131]

8.5.3.3.9     CmTekGraceTime
The value of this attribute is the grace time for the TEK in seconds. The CM is expected to start trying to acquire a
new TEK beginning TEK GraceTime seconds before the expiration of the most recent TEK.
References: docsBpi2CmTEKGraceTime [RFC 4131]

8.5.3.3.10    CmAuthWaitTimeout
The value of this attribute is the Authorize Wait Timeout in seconds.
References: docsBpi2CmAuthWaitTimeout [RFC 4131]

8.5.3.3.11    CmReauthWaitTimeout
The value of this attribute is the Reauthorize Wait Timeout in seconds.
References: docsBpi2CmReauthWaitTimeout [RFC 4131]

8.5.3.3.12    CmOpWaitTimeout
The value of this attribute is the Operational Wait Timeout in seconds.
References: docsBpi2CmOpWaitTimeout [RFC 4131]



04/29/26                                            CableLabs                                                         89
<!-- PDF_PAGE: 90 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


8.5.3.3.13    CmReKeyWaitTimeout
The value of this attribute is the Rekey Wait Timeout in seconds.
References: docsBpi2CmRekeyWaitTimeout [RFC 4131]

8.5.3.3.14    CmAuthRejectWaitTimeout
The value of this attribute is the Authorization Reject Wait Timeout in seconds.
References: docsBpi2CmAuthRejectWaitTimeout [RFC 4131]

8.5.3.3.15    CmSAMapWaitTimeout
The value of this attribute is retransmission interval, in seconds, of SA Map Requests from the MAP Wait state.
References: docsBpi2CmSAMapWaitTimeout [RFC 4131]

8.5.3.3.16    CmSAMapMaxRetries
The value of this attribute is the maximum number of Map Request retries allowed.
References: docsBpi2CmSAMapMaxRetries [RFC 4131]

8.5.3.3.17    CmAuthInfos
The value of this attribute is the number of times the CM has transmitted an Authentication Information message.
Discontinuities in the value of this counter can occur at re-initialization of the management system and at other
times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmAuthentInfos [RFC 4131]

8.5.3.3.18    CmAuthRequests
The value of this attribute is the number of times the CM has transmitted an Authorization Request message.
Discontinuities in the value of this counter can occur at re-initialization of the management system and at other
times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmAuthRequests [RFC 4131]

8.5.3.3.19    CmAuthReplies
The value of this attribute is the number of times the CM has transmitted an Authentication Reply message.
Discontinuities in the value of this counter can occur at re-initialization of the management system and at other
times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmAuthReplies [RFC 4131]

8.5.3.3.20    CmAuthRejects
The value of this attribute is the number of times the CM has transmitted an Authentication Reject message.
Discontinuities in the value of this counter can occur at re-initialization of the management system and at other
times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmAuthRejects [RFC 4131]

8.5.3.3.21    CmAuthInvalids
The value of this attribute is the count of times the CM has received an Authorization Invalid message.
Discontinuities in the value of this counter can occur at re-initialization of the management system and at other
times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmAuthInvalids [RFC 4131]




90                                                  CableLabs                                               04/29/26
<!-- PDF_PAGE: 91 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


8.5.3.3.22    CmAuthRejectErrorCode
The value of this attribute is the enumerated description of the Error-Code in the most recent Authorization Reject
message received by the CM. The possible values for this attribute are listed below.
    •    'none' - No Authorization Reject message has been received since reboot.
    •    'unknown' - The last Error-Code value was 0 (i.e., no information).
    •    'unauthorizedCm' - The CM is unauthorized.
    •    'unauthorizedSaid' - The SAID is unauthorized.
    •    'permanentAuthorizationFailure' - The error conditions affecting the BPKM authorization exchange. These
         include the following:
         •   an unknown manufacturer, i.e., the CMTS does not have the CA certificate belonging to the issuer of a
             CM Device Certificate;
         •   CM Device Certificate has an invalid signature;
         •   ASN.1 parsing failure during verification of CM Device Certificate;
         •   CM Device Certificate is revoked (see [SECv4.0] Certificate Revocation section);
         •   inconsistencies between certificate data and data in accompanying BPKM attributes; and
         •   the CM and CMTS have incompatible security capabilities.
    •    'timeOfDayNotAcquired' - Time of day is not acquired.
    •    'eaeDisabled' - EAE is disabled.
    •    'unsupportedBpiVer' - The BPI+ version used by CM is not supported.
References: docsBpi2CmAuthRejectErrorCode [RFC 4131]; Section 7.1.3.19, "Requirements for DOCSIS Baseline
Privacy Plus MIB (RFC 4131)"

8.5.3.3.23    CmAuthRejectErrorString
The value of this attribute is text string in the most recent Authorization Reject message received by the CM. This is
a zero length string if no Authorization Reject message has been received since reboot.
References: docsBpi2CmAuthRejectErrorString [RFC 4131]

8.5.3.3.24    CmAuthInvalidErrorCode
The value of this attribute is the enumerated description of the Error-Code in the most recent Authorization Invalid
message received by the CM. The possible values for this attribute are listed below.
    •    'none' - No Authorization Invalid message has been received since reboot.
    •    'unknown' - The last Error-Code value was 0 (i.e., no information).
    •    'unauthorizedCm' - The CM is unauthorized.
    •    'unsolicited' - The CMTS sends an unsolicited Authorization Invalid message to a CM, forcing an Auth
         Invalid event.
    •    'invalidKeySequence' - The Key Sequence Number is invalid.
    •    'keyRequestAuthenticationFailure' - The Key Request Message fails authentication with the CMTS.
References: docsBpi2CmAuthInvalidErrorCode [RFC 4131]




04/29/26                                            CableLabs                                                        91
<!-- PDF_PAGE: 92 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


8.5.3.3.25     CmAuthInvalidErrorString
The value of this attribute is text string in the most recent Authorization Invalid message received by the CM. This is
a zero-length string if no Authorization Invalid message has been received since reboot.
References: docsBpi2CmAuthInvalidErrorString [RFC 4131]

8.5.3.3.26     CmBpiPlusVer
The value of this attribute shows the BPI+ version used by the CM for authentication.
     •    'none' - BPI+ is not used by the CM.
     •    'v1' - BPI+V1
     •    'v2' - BPI+V2
References: Baseline Privacy Key Management (BPKM) Protocol [SECv4.0]

8.5.3.3.27     CmAuthStatusInfo
The value of this attribute is the number of times the CM has transmitted an Authentication Status Info message.
This attribute is for BPI+V2 mode only.
For a non-BPI+V2 CM, the value of this attribute needs to be '0'. If this is a BPI+V2 CM, this attribute needs to be a
non-zero value.
The value of this attribute is not preserved after the CM power-cycles or resets.
References: Baseline Privacy Key Management (BPKM) Protocol [SECv4.0]

8.5.3.4    BpiPlusTek
The BpiPlusTek object describes the attributes of each CM Traffic Encryption Key (TEK) association. The CM
maintains (no more than) one TEK association per SAID.
References: docsBpi2CmTEKTable [RFC 4131]
                                       Table 42 - BpiPlusTek Object Attributes

                                                                                                            Default
     Attribute Name               Type        Access               Type Constraints            Units         Value
CmTekSAId                   UnsignedShort   Key          1..16383                           N/A
CmTekSAType                 Enum            R/O          none(1),                           N/A
                                                         primary(2),
                                                         static(3),
                                                         dynamic(4)
CmTekDataEncryptAlg         Enum            R/O          none(1),                           N/A
                                                         des56CbcMode(2),
                                                         des40CbcMode(3),
                                                         aes128CbcMode(4),
                                                         aes256CbcMode(5)
CmTekDataAuthentAlg         Enum            R/O          none(1)                            N/A
CmTekState                  Enum            R/O          start(1),                          N/A
                                                         opWait(2),
                                                         opReauthWait(3),
                                                         operational(4),
                                                         rekeyWait(5),
                                                         rekeyReauthWait(6)
CmTekKeySeqNum              Int             R/O          0..15                              N/A
CmTekExpiresOld             DateTime        R/O          SIZE(11)                           N/A




92                                                  CableLabs                                               04/29/26
<!-- PDF_PAGE: 93 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


                                                                                                       Default
     Attribute Name                  Type      Access              Type Constraints          Units      Value
CmTekExpiresNew                 DateTime      R/O          SIZE(11)                        N/A
CmTekKeyRequests                Counter32     R/O                                          N/A
CmTekKeyReplies                 Counter32     R/O                                          N/A
CmTekKeyRejects                 Counter32     R/O                                          N/A
CmTekInvalids                   Counter32     R/O                                          N/A
CmTekAuthPends                  Counter32     R/O                                          N/A
CmTekKeyRejectErrorCode         Enum          R/O          none(1),                        N/A
                                                           unknown(2),
                                                           unauthorizedSaid(4)
CmTekKeyRejectErrorString String              R/O          SIZE (0..128)                   N/A
CmTekInvalidErrorCode           Enum          R/O          none(1),                        N/A
                                                           unknown(2),
                                                           invalidKeySequence(6)
CmTekInvalidErrorString         String        R/O          SIZE (0..128)                   N/A



8.5.3.4.1       CmTekSAId
The value of this key attribute is the DOCSIS Security Association ID (SAID).
References: docsBpi2CmTEKSAId [RFC 4131]

8.5.3.4.2       CmTekSAType
The value of this attribute is the type of security association.
    •    'none' - SA is empty.
    •    'primary' - Primary SA
    •    'static' - Static SA
    •    'dynamic' - Dynamic SA
References: docsBpi2CmTEKSAType [RFC 4131]

8.5.3.4.3       CmTekDataEncryptAlg
The value of this attribute is the data encryption algorithm for this SAID.
    •    'none' - No data encryption algorithm is used.
    •    'des56CbcMode' - CBC-Mode, 56-bit DES
    •    'des40CbcMode' - CBC-Mode, 40-bit DES
    •    'aes128CbcMode' - CBC-Mode, 128-bit block, 128-bit key AES
    •    'aes256CbcMode' - CBC-Mode, 128-bit block, 256-bit key AES
References: docsBpi2CmTEKDataEncryptAlg [RFC 4131]

8.5.3.4.4       CmTekDataAuthentAlg
The value of this attribute is the data authentication algorithm for this SAID.
    •    'none' - No data authentication algorithm is used.
References: docsBpi2CmTEKDataAuthentAlg [RFC 4131]



04/29/26                                              CableLabs                                                 93
<!-- PDF_PAGE: 94 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


8.5.3.4.5     CmTekState
The value of this attribute is the state of the indicated TEK FSM. The start(1) state indicates that the CM TEK FSM
is in its initial state. The possible values for this attribute are listed below.
     •   'start' - This is the initial state of the FSM. No resources are assigned to or used by the FSM, all timers are
         off, and no processing is scheduled.
     •   'opWait' - The TEK state machine has sent its initial Key Request for its SAID's keying material (TEK and
         CBC IV) and is waiting for a reply from the CMTS.
     •   'opReauthWait' - The Authorization state machine is in a reauthorization cycle, and the CM does not have
         valid keying material for this SAID.
     •   'operational' - The CM has valid keying material for the associated SAID.
     •   'rekeyWait' - The TEK Refresh Timer has expired, and the CM has requested a key update for this SAID to
         replace the older of the two TEKs.
     •   'rekeyReauthWait' - The CM has valid traffic keying material for this SAID and has an outstanding request
         for the latest keying material, and the Authorization state machine has initiated a reauthorization cycle.
References: docsBpi2CmTEKState [RFC 4131]

8.5.3.4.6     CmTekKeySeqNum
The value of this attribute is the most recent TEK key sequence number for this TEK FSM.
References: docsBpi2CmTEKKeySequenceNumber [RFC 4131]

8.5.3.4.7     CmTekExpiresOld
The value of this attribute is actual clock time for expiration of the immediate predecessor of the most recent TEK
for this FSM. If this FSM has only one TEK, then the value is the time of activation of this FSM.
References: docsBpi2CmTEKExpiresOld [RFC 4131]

8.5.3.4.8     CmTekExpiresNew
The value of this attribute is the actual clock time for expiration of the most recent TEK for this FSM.
References: docsBpi2CmTEKExpiresNew [RFC 4131]

8.5.3.4.9     CmTekKeyRequests
The value of this attribute is the number of times the CM has transmitted a Key Request message. Discontinuities in
the value of this counter can occur at re-initialization of the management system.
References: docsBpi2CmTEKKeyRequests [RFC 4131]

8.5.3.4.10    CmTekKeyReplies
The value of this attribute is the number of times the CM has received a Key Reply message, including a message
whose authentication failed. Discontinuities in the value of this counter can occur at re-initialization of the
management system and at other times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmTEKKeyReplies [RFC 4131]

8.5.3.4.11    CmTekKeyRejects
The value of this attribute is the number of times the CM has received a Key Reject message, including a message
whose authentication failed. Discontinuities in the value of this counter can occur at re-initialization of the
management system and at other times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmTEKKeyRejects [RFC 4131]


94                                                   CableLabs                                                 04/29/26
<!-- PDF_PAGE: 95 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


8.5.3.4.12     CmTekKeyInvalids
The value of this attribute is the number of times the CM has received a Key Reply message, including a message
whose authentication failed. Discontinuities in the value of this counter can occur at re-initialization of the
management system and at other times as indicated by the value of ifCounterDiscontinuityTime.
References: docsBpi2CmTEKInvalids [RFC 4131]

8.5.3.4.13     CmTekAuthPends
The value of this attribute is the count of times an Authorization Pending (Auth Pend) event occurred in the TEK
FSM. Discontinuities in the value of this counter can occur at re-initialization of the management system.
References: docsBpi2CmTEKAuthPends [RFC 4131]

8.5.3.4.14     CmTekKeyRejectErrorCode
The value of this attribute is the enumerated description of the Error-Code in the most recent Key Reject message
received by the CM. This has the value unknown(2) if the last Error-Code value was 0 and none(1) if no Key Reject
message has been received since registration.
    •     'none' - No key reject message has been received since registration.
    •     'unknown' - Last error code value was zero.
    •     'unauthorizedSaid' - SA ID is unauthorized.
References: docsBpi2CmTEKKeyRejectErrorCode [RFC 4131]

8.5.3.4.15     CmTekKeyRejectErrorString
The value of this attribute is the text string in the most recent Key Reject message received by the CM. This is a zero
length string if no Key Reject message has been received since registration.
References: docsBpi2CmTEKKeyRejectErrorString [RFC 4131]

8.5.3.4.16     CmTekKeyInvalidErrorCode
The value of this attribute is the enumerated description of the Error-Code in the most recent TEK Invalid message
received by the CM. This has the value unknown(2) if the last Error-Code value was 0 and none(1) if no TEK
Invalid message has been received since registration.
    •     'none' - No key invalid message has been received since registration.
    •     'unknown' - Last error code value was zero.
    •     'invalidKeySequence' - Key sequence number is invalid.
References: docsBpi2CmTEKInvalidErrorCode [RFC 4131]

8.5.3.4.17     CmTekKeyInvalidErrorString
The value of this attribute is the text string in the most recent TEK Invalid message received by the CM. This is a
zero length string if no Key Reject message has been received since registration.
References: docsBpi2CmTEKInvalidErrorString [RFC 4131]

8.5.3.5    BpiPlusCryptoSuites
The BpiPlusCryptoSuites object describes the Baseline Privacy Plus cryptographic suite capabilities.
References: docsBpi2CmCryptoSuiteTable [RFC 4131]




04/29/26                                            CableLabs                                                        95
<!-- PDF_PAGE: 96 -->
CM-SP-CM-OSSIv4.0-I13-260429                                         Data-Over-Cable Service Interface Specifications


                                   Table 43 - BpiPlusCryptoSuites Object Attributes

     Attribute Name                Type         Access               Type Constraints             Units     Default
                                                                                                             Value
Index                       UnsignedShort     Key          1..1000                            N/A
CmDataEncryptAlg            Enum              R/O          reserved(1),
                                                           des56CbcMode(2),
                                                           des40CbcMode(3),
                                                           aes128CbcMode(4),
                                                           aes256CbcMode(5)
CmDataAuthentAlg            Enum              R/O          none(1),
                                                           reserved(2)



8.5.3.5.1    Index
The value of this attribute is the index for a cryptographic suite.
References: docsBpi2CmCryptoSuiteIndex [RFC 4131]

8.5.3.5.1    CmDataEncryptAlg
The value of this attribute is the data encryption algorithm for this cryptographic suite capability.
     •    'reserved' -This value is reserved for further use.
     •    'des56CbcMode' - CBC-Mode, 56-bit DES
     •    'des40CbcMode' - CBC-Mode, 40-bit DES
     •    'aes128CbcMode' - CBC-Mode, 128-bit block, 128-bit key AES
     •    'aes256CbcMode' - CBC-Mode, 128-bit block, 256-bit key AES
References: docsBpi2CmCryptoSuiteDataEncryptAlg [RFC 4131]

8.5.3.5.1    CmDataAuthentAlg
The value of this attribute is the data authentication algorithm for this cryptographic suite capability.
     •    'none' - No data authentication algorithm is used.
     •    'reserved' -This value is reserved for further use.
References: docsBpi2CmCryptoSuiteDataAuthentAlg [RFC 4131]

8.5.3.6     BpiPlusV2Cfg
TheBpiPlusV2Cfg object describes the control of BPI+V2 on the CM.
References: Baseline Privacy Key Management (BPKM) Protocol [SECv4.0]
                                     Table 44 - BpiPlusV2Cfg Object Associations

  Associated Object                                                           Near-End        Far-End
       Name                                    Type                           Multiplicity   Multiplicity   Label
 CmtsDesignationCfg       Directed composition to CmtsDesignationCfg               1             0..*
 TrustOnFirstUseCfg       Directed composition to TrustOnFirstUseCfg               1             0..1




96                                                    CableLabs                                            04/29/26
<!-- PDF_PAGE: 97 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


                                        Table 45 - BpiPlusV2Cfg Object Attributes

                                                                                                     Default
     Attribute Name                  Type       Access             Type Constraints       Units      Value
CmAuthKeyEcCurve            Enum               R/O        secp256r1(1),
                                                          secp384r1(2),
                                                          secp521r1(3),
                                                          x25519(4),
                                                          x448(5)



8.5.3.6.1      CmAuthKeyEcCurve
The value of this attribute is the supported Elliptic-Curve Groups (ECDHE) for CM to generate an EC Auth Key
while in BPI+V2 mode.
    •     'secp256r1' - also known as NIST P-256 curve
    •     'secp384r1' - also known as NIST P-384 curve
    •     'secp521r1'- also known as NIST P-521 curve
    •     'x25519' - curve25519 with Diffie-Hellman key agreement
    •     'x448' - curve448 with Diffie-Hellman key agreement
References: Baseline Privacy Key Management (BPKM) Protocol [SECv4.0]

8.5.3.7     CmtsDesignationCfg
The CmtsDesignationCfg object contains information the CM uses to provide rules to bind a CM to a CMTS.
Multiple rules can be added to provide the level of granularity desired.
References: CMTS-Designation [SECv4.0]
                                     Table 46 - CmtsDesignationCfg Object Attributes

                                                                                                     Default
     Attribute Name                  Type       Access             Type Constraints       Units       Value
Index                       UnsignedByte       Key                                      N/A
CmtsDesignationType         Enum               R/W        fingerprint(1),               N/A
                                                          cn(2),
                                                          ou(3),
                                                          on(4),
                                                          serialNum(5),
                                                          caFingerprint(6),
                                                          caCN(7),
                                                          caOU(8),
                                                          caON(9),
                                                          caSerialNum(10)
CmtsDesignationAttr         String             R/W        SIZE (0..128)                 N/A



8.5.3.7.1      Index
The value of this key attribute is the index for an instance of this object.




04/29/26                                              CableLabs                                               97
<!-- PDF_PAGE: 98 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


8.5.3.7.2      CmtsDesignationType
The value of this attribute indicates the type of data in the certificates to be checked for CM at the next authorization
session. The possible values for this attribute are listed below.
     •    'fingerprint' - this data type is the fingerprint of CMTS Device Certificate.
     •    'cn' - this data type is the Common Name of CMTS Device Certificate.
     •    'ou' - this data type is the Organizational Unit of CMTS Device Certificate.
     •    'on' - this data type is the Organization Name of CMTS Device Certificate.
     •    'serialNum' - this data type is the serial number of CMTS Device Certificate.
     •    'caFingerprint' - this data type is the fingerprint of CMTS Device CA Certificate.
     •    'caCN' - this data type is the Common Name of CMTS Device CA Certificate.
     •    'caOU' - this data type is the Organizational Unit of CMTS Device CA Certificate.
     •    'caON' - this data type is the Organization Name of CMTS Device CA Certificate.
     •    'caSerialNum' - this data type is the serial number of CMTS Device CA Certificate.
References: CMTS-Designation [SECv4.0]

8.5.3.7.3      CmtsDesignationAttr
The value of this attribute is used when checking the value of specified data type for CM at the next authorization
session.
References: CMTS-Designation [SECv4.0]

8.5.3.8     TrustOnFirstUseCfg
The TrustOnFirstUseCfg object contains information for CM to prevent the unauthorized downgrades. This object
records the successful authentication configuration used during the first connection to a CMTS and is checked and
used in subsequent connections.
References: Trust On First Use (TOFU) [SECv4.0]
                                  Table 47 - TrustOnFirstUseCfg Object Attributes

                                                                                                              Default
     Attribute Name              Type          Access             Type Constraints               Units        Value
AllowedBpiVers              EnumBits          R/W         bpiPlusV1(0),                        N/A
                                                          bpiPlusV2(1)



8.5.3.8.1      AllowedBpiVers
This attribute configures which BPI+ version the CM supported in subsequent connections.
If the 'bpiPlusV1' bit (bit 0) is set to '1', the CM only uses BPI+V1 to connect to a CMTS.
If the 'bpiPlusV2' bit (bit 1) is set to '1', the CM only uses BPI+V2 to connect to a CMTS.
If both Bit 0 and Bit 1 are set, the CM allows access from both BPI+V1 and BPI+V2 to connect to a CMTS.
References: Trust On First Use (TOFU) [SECv4.0]

8.5.3.9     CertificateManagement
The CertificateManagement object describes the set of known Certificate Authority certificates, Device certificates
as well as revocation control and certs status acquired by the CM.



98                                                    CableLabs                                               04/29/26
<!-- PDF_PAGE: 99 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv4.0-I13-260429


References: BPI+ X.509 Certificate Profile and Management [SECv4.0]
                                 Table 48 - CertificateManagement Object Associations

  Associated Object                                                            Near-End        Far-End
       Name                                    Type                            Multiplicity   Multiplicity        Label
 CmIntermediateCaCert      Directed composition to CmIntermediateCaCert             1               1
 CmtsTrustAnchorCert       Directed composition to CmtsTrustAnchorCert              1              0..1
 CmCert                    Directed composition to CmCert                           1               1
 LearnedCmtsCert           Directed composition to LearnedCmtsCert                  1              0..1
 CertRevocationCfg         Directed composition to CertRevocationCfg                1               1



8.5.3.10 CmIntermediateCaCert
The CmIntermediateCaCert object describes the set of known CM Intermediate CA certificates: the Device CA
certificates from the new PKI, and the Manufacturer CA certificate from the legacy PKI.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0]
                                  Table 49 - CmIntermediateCaCert Object Attributes

    Attribute Name                          Type                           Access        Type Constraints         Units
 Docs30IntCaCert           DocsX509ASN1DEREncodedCertificate             R/O                                    N/A
 IntCaCert                 DocsX509ASN1DEREncodedCertificate             R/O                                    N/A



8.5.3.10.1    Docs30IntCaCert
This attribute represents the X509 DER-encoded Manufacturer CA certificate that signed the DOCSIS 3.0 CM
Device Certificate.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0],
docsBpi2CmDeviceManufCert [RFC 4131] (Legacy PKI CM certificate functions)

8.5.3.10.2    IntCaCert
This attribute represents the X509 DER-encoded Device CA Certificate that signed the DOCSIS 3.1/4.0 CM Device
Certificate.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0],
docsBpi2Ext31CmDeviceManufCert [DOCS-BPI2EXT-MIB] (DOCSIS 3.1/4.0 PKI CM certificate functions)

8.5.3.11 CmtsTrustAnchorCert
The CmtsTrustAnchorCert object describes the set of learned CMTS Root CA certificate during the BPI+V2
process.
References: Baseline Privacy Key Management (BPKM) protocol [SECv4.0]
                                   Table 50 - CmtsTrustAnchorCert Object Attributes

                                                                                                                      Default
  Attribute Name                        Type                      Access        Type Constraints        Units          Value
DeviceId                String                                  R/O                                N/A
RootCaCert              DocsX509ASN1DEREncodedCertificate       R/O                                N/A


8.5.3.11.1    DeviceId
This attribute represents the unique device identifier of an instance of this object. It should match the value
presented in the CN field of the CMTS Device Certificate (e.g., FQDN or the Device MAC address).


04/29/26                                              CableLabs                                                            99
<!-- PDF_PAGE: 100 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


References: DOCSIS 4.0 Certificates [C-PKI-TI-V1.5]

8.5.3.11.2    CaCert
This attribute represents the X509 DER-encoded Root CA certificate that signed the CMTS Device CA Certificate.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0]

8.5.3.12 CmCert
The CmCert object describes the set of known CM Device Certificates acquired by the CM.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0]
                                         Table 51 - CmCert Object Attributes

                                                                                  Type                            Default
   Attribute Name                      Type                      Access         Constraints         Units          Value
Docs30DeviceCert      DocsX509ASN1DEREncodedCertificate       R/O                                N/A
DeviceCert            DocsX509ASN1DEREncodedCertificate       R/O                                N/A



8.5.3.12.1    Docs30DeviceCert
This attribute represents the X509 DER-encoded DOCSIS 3.0 CM Device Certificate.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0], docsBpi2CmDeviceCmCert
[RFC 4131] (Legacy PKI CM certificate functions)

8.5.3.12.2    DeviceCert
This attribute represents the X509 DER-encoded DOCSIS 3.1/4.0 CM Device Certificate.
References: Cable Modem Certificate Storage and Management in the CM [SECv4.0],
docsBpi2Ext31CmDeviceCmCert [DOCS-BPI2EXT-MIB] (DOCSIS 3.1/4.0 PKI CM certificate functions)

8.5.3.13 LearnedCmtsCert
The LearnedCmtsCert object describes the DOCSIS 4.0 CMTS Device Certificates learned by the CM during the
BPI+V2 process.
References: Baseline Privacy Key Management (BPKM) protocol [SECv4.0]
                                   Table 52 - LearnedCmtsCert Object Attributes

                                                                                  Type                            Default
   Attribute Name                      Type                      Access         Constraints         Units         Value
DeviceId              String                                  R/O                                N/A
DeviceCert            DocsX509ASN1DEREncodedCertificate       R/O                                N/A



8.5.3.13.1    DeviceId
This attribute represents the unique device identifier of an instance of this object. It should match the value
presented in the CN field of the CMTS Device Certificate (e.g., FQDN or the Device MAC address).
References: DOCSIS 4.0 Certificates [C-PKI-TI-V1.5]

8.5.3.13.2    DeviceCert
This attribute represents the X509 DER-encoded DOCSIS 4.0 CMTS Device Certificate.
References: Baseline Privacy Key Management (BPKM) Protocol [SECv4.0]



100                                                  CableLabs                                                   04/29/26
<!-- PDF_PAGE: 101 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv4.0-I13-260429


8.5.3.14 CmCertRevocationCfg
The CmCertRevocationCfg object describes the revocation control on the CM.
References: Authorization Messages Authentication Overview [SECv4.0]
                                   Table 53 - CmCertRevocationCfg Object Associations

  Associated Object                                                        Near-End          Far-End
       Name                                       Type                     Multiplicity     Multiplicity         Label
 CmOnlineCertStatusProt       Directed composition to                            1                  1
 ocolCfg                      CmOnlineCertStatusProtocolCfg


                                    Table 54 - CmCertRevocationCfg Object Attributes

                                                                                  Type                            Default
   Attribute Name                          Type                  Access         Constraints             Units      Value
Method                    Enum                                 R/W           none(1),               N/A         'none'
                                                                             ocsp(2)



8.5.3.14.1        Method
This attribute indicates the used method for performing the revocation checking.
'none' means the revocation checking is disabled on the CM.
      •   'none' - Revocation checking is disabled.
      •   'ocsp' - Use OCSP for revocation checking.
References: Authorization Messages Authentication Overview [SECv4.0]

8.5.3.15 CmOnlineCertStatusProtocolCfg
The CmOnlineCertStatusProtocolCfg object provides the configuration information for CM to perform revocation
checking using Online Cert Status Protocol.
References: Authorization Messages Authentication Overview [SECv4.0]
                               Table 55 - CmOnlineCertStatusProtocolCfg Object Attributes

                                                                                  Type                            Default
  Attribute Name                          Type                   Access         Constraints             Units      Value
Cache                     Enum                                 R/W          none(1),                N/A         'none'
                                                                            good(2),
                                                                            revoked(3),
                                                                            unknown(4)
RefreshInterval           UnsignedInt                          R/W          1..524160               N/A
Url                       String                               R/W          1..2048                 N/A



8.5.3.15.1        Cache
This attribute indicates the cached OCSP status used by CM. If the value of CmCertRevocationCfg::Method is
'none', the CM does not need to check this attribute.
      •   'none' - no OCSP cache
      •   'good' - Certificates in the received CMTS certificate chain are not revoked.
      •   'revoked' - At least one certificate in the received CMTS certificate chain is revoked.



04/29/26                                                 CableLabs                                                      101
<!-- PDF_PAGE: 102 -->
CM-SP-CM-OSSIv4.0-I13-260429                                         Data-Over-Cable Service Interface Specifications


      •     'unknown' - unknown revocation status
References: Authorization Messages Authentication Overview [SECv4.0]

8.5.3.15.2       RefreshInterval
This attribute indicates the refresh interval (minutes) for the cached OCSP status used by the CM. If the value of
CmCertRevocationCfg::Method is 'none', the CM does not need to check this attribute.
References: Authorization Messages Authentication Overview [SECv4.0]

8.5.3.15.3       Url
This attribute is the URL for OCSP communications in checking a certificate's revocation status. This attribute
configures the CM with a URL string to retrieve OCSP information. If the value of this attribute is a zero-length
string, the CM does not need to check the revocation status of a CMTS Device Certificate. If the value of
CmCertRevocationCfg::Method is 'none', the CM does not need to check this attribute.
References: Authorization Messages Authentication Overview [SECv4.0]

8.5.3.16 SsdManagement
The SsdManagement object provides the control of Secure Software Download (SSD) on the CM.
                                       Table 56 - SsdManagement Object Attributes

  Associated Object                                                           Near-End            Far-End
       Name                                      Type                         Multiplicity       Multiplicity         Label
 SsdCfg                    Directed composition to SsdCfg                               1             1
 CodeDownloadControl       Directed composition to CodeDownloadControl                  1             1
 SsdStatus                 Directed composition to SsdStatus                            1             1



8.5.3.17 SsdCfg
The SsdCfg object contains Secure Software Download (SSD) control attributes for the CM.
References: docsDevSoftware [RFC 4639]
                                           Table 57 - SsdCfg Object Attributes

          Attribute Name                  Type                   Access                 Type Constraints              Units
SwFilename                    String                           R/W           SIZE (0..64)                       N/A
SwServerAddress               IpAddress                        R/W                                              N/A
TransportProtocol             Enum                             R/W           tftp(1),                           N/A
                                                                             http(2)
SwAdminControl                Enum                             R/W           upgradeFromMgt(1),                 N/A
                                                                             allowProvisioningUpgrade(2),
                                                                             ignoreProvisioningUpgrade(3)



8.5.3.17.1       SwFileName
This attribute is the filename of the software image to be downloaded.
References: docsDevSwFilename [RFC 4639]

8.5.3.17.2       SwServerAddress
This attribute is the address of the TFTP or HTTP server used for software upgrades. If the TFTP/HTTP server is
unknown, return the zero-length address string.



102                                                     CableLabs                                                    04/29/26
<!-- PDF_PAGE: 103 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


References: docsDevSwServerAddress [RFC 4639]

8.5.3.17.3     TransportProtocol
This attribute is the transport protocol (TFTP or HTTP) to be used for software upgrades.
References: docsDevSwServerTransportProtocol [RFC 4639]

8.5.3.17.4     SwAdminControl
This attribute is the status of the SSD administration. The possible values for this attribute are listed below.
    •     'upgradeFromMgt' - If set to upgradeFromMgt, the device will initiate a TFTP or HTTP software image
          download. This indicates that a software download is currently in progress, and that the device will reboot
          after successfully receiving an image. If the download process is interrupted (e.g., by a reset or power
          failure), the device will load the previous image and, after re-initialization, continue to attempt loading the
          image specified in docsDevSwFilename.
    •     'allowProvisioningUpgrade' - If set to allowProvisioningUpgrade, the device will use the software version
          information supplied by the provisioning server when next rebooting (this does not cause a reboot).
    •     'ignoreProvisioningUpgrade' - If set to ignoreProvisioningUpgrade, the device will disregard software
          image upgrade information from the provisioning server.
References: docsDevSwAdminStatus [RFC 4639]

8.5.3.18 CodeDownloadControl
The CodeDownloadControl object contains code download control attributes for the CM.
References: docsBpi2Ext31CodeDownloadControl [DOCS-BPI2EXT-MIB]
                                 Table 58 - CodeDownloadControl Object Attributes

        Attribute Name                    Type                Access             Type Constraints                  Units
UpdateCvcChain                HexBinary                   R/W                                               N/A
MfgOrgName                    AdminString                 R/O                                               N/A
MfgCodeAccessStart            DateTime                    R/O              SIZE(11)                         N/A
MfgCvcAccessStart             DateTime                    R/O              SIZE(11)                         N/A
CoSignerOrgName               AdminString                 R/O                                               N/A
CoSignerCodeAccessStart       DateTime                    R/O              SIZE(11)                         N/A
CoSignerCvcAccessStart        DateTime                    R/O              SIZE(11)                         N/A
OcspCvcResponse               String                      R/W              SIZE(0..255)                     N/A



8.5.3.18.1     UpdateCvcChain
This attribute is a degenerate PKCS7 signedData structure that contains the CVC and the CVC CA certificate chain
in the certificates field. Setting this object triggers the device to verify the CVC and update the cvcAccessStart
values associated with the new PKI defined by DOCSIS 4.0. The content of this object is then discarded. If the
device is not enabled to upgrade code files, or if the CVC verification fails, the CVC will be rejected. Reading this
object always returns the zero-length OCTET STRING.
References: docsBpi2Ext31CodeUpdateCvcChain [DOCS-BPI2EXT-MIB]

8.5.3.18.2     MfgOrgName
This attribute is the device manufacturer's organizationName used to validate the code verification certificate issued
from the new PKI defined in DOCSIS 4.0.
References: docsBpi2Ext31CodeMfgOrgName [DOCS-BPI2EXT-MIB]


04/29/26                                              CableLabs                                                       103
<!-- PDF_PAGE: 104 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


8.5.3.18.3     MfgCodeAccessStart
This attribute is the device manufacturer's current codeAccessStart value used with the new PKI defined in DOCSIS
3.1. This value will always refer to Greenwich Mean Time (GMT), and the value is required to contain TimeZone
information (fields 8–10).
References: docsBpi2Ext31CodeMfgCodeAccessStart [DOCS-BPI2EXT-MIB]

8.5.3.18.4     MfgCvcAccessStart
This attribute is the device manufacturer's current cvcAccessStart value used with the new PKI defined in DOCSIS
4.0. This value will always refer to Greenwich Mean Time (GMT), and the value is required to contain TimeZone
information (fields 8–10).
References: docsBpi2Ext31CodeMfgCvcAccessStart [DOCS-BPI2EXT-MIB]

8.5.3.18.5     CoSignerOrgName
This attribute is the co-signer's organizationName used to validate the code verification certificate issued from the
new PKI defined in DOCSIS 4.0. The value is a zero-length string if the co-signer is not specified.
References: docsBpi2Ext31CodeCoSignerOrgName [DOCS-BPI2EXT-MIB]

8.5.3.18.6     CoSignerCodeAccessStart
This attribute is the co-signer's current codeAccessStart value used with the new PKI defined in DOCSIS 4.0. This
value will always refer to Greenwich Mean Time (GMT), and the value is required to contain TimeZone information
(fields 8–10). If CoSignerOrgName is a zero-length string, the value of this object is meaningless.
References: docsBpi2Ext31CodeCoSignerCodeAccessStart [DOCS-BPI2EXT-MIB]

8.5.3.18.7     CoSignerCvcAccessStart
This attribute is the co-signer's current cvcAccessStart value used with the new PKI defined in DOCSIS 4.0. This
value will always refer to Greenwich Mean Time (GMT), and the value is required to contain TimeZone information
(fields 8–10). If CoSignerOrgName is a zero-length string, the value of this object is meaningless.
References: docsBpi2Ext31CodeCoSignerCvcAccessStart [DOCS-BPI2EXT-MIB]

8.5.3.18.8     OcspCvcResponse
This optional attribute is the Revocation Information for the CVC chains.
References: Network Initialization [SECv4.0]

8.5.3.19 SsdStatus
The SsdStatus object contains Secure Software Download (SSD) Status attributes for the CM.
                                       Table 59 - SsdStatus Object Attributes

      Attribute Name                   Type                 Access             Type Constraints                 Units
SwOperStatus                 Enum                        R/O             inProgress(1),                   N/A
                                                                         completeFromProvisioning(2),
                                                                         completeFromMgt(3),
                                                                         failed(4),
                                                                         idle(5),
                                                                         cvcVerified(6),
                                                                         cvcRejected(7),
                                                                         codeFileVerified(8),
                                                                         codeFileRejected(9),
                                                                         unknown(10)




104                                                 CableLabs                                                  04/29/26
<!-- PDF_PAGE: 105 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


        Attribute Name                   Type                 Access              Type Constraints                  Units
SwCurrentVers                 AdminString                  R/O              SIZE(11)                          N/A
SwStatusCode                  Enum                         R/O              configFileCvcVerified(1),         N/A
                                                                            configFileCvcRejected(2),
                                                                            snmpCvcVerified(3),
                                                                            snmpFileCvcRejected(4),
                                                                            codeFileVerified(5),
                                                                            codeFileRejected(6),
                                                                            other(7)
SwStatusString                String                       R/O                                                N/A



8.5.3.19.1      SwOperStatus
This attribute is the status of the SSD operation. The possible values for this attribute are listed below.
    •     'inProgress' - This status indicates that a TFTP or HTTP download is underway, either as a result of a
          version mismatch at provisioning or as a result of a upgradeFromMgt request.
    •     'completeFromProvisioning' - This status indicates that the last software upgrade was a result of version
          mismatch at provisioning.
    •     'completeFromMgt' - This status indicates that the last software upgrade was a result of setting
          docsDevSwAdminStatus to upgradeFromMgt.
    •     'failed' - This status indicates that the last attempted download failed, ordinarily due to TFTP or HTTP
          timeout.
    •     'idle' - This status indicates that the CM is not running a TFTP or HTTP download.
    •     'cvcVerified' - This status indicates that the CVC chain has been verified and is valid.
    •     'cvcRejected' - This status indicates that the CVC chain has been rejected by the CM.
    •     'codeFileVerified' - This status indicates that the code file has been verified and is valid.
    •     'codeFileRejected' - This status indicates that the code file has been rejected by the CM.
    •     'unknown' - unknown status
References: docsDevSwOperStatus [RFC 4639]

8.5.3.19.2      SwCurrentVers
This attribute is the software version currently operating in the CM.
References: docsDevSwCurrentVers [RFC 4639]

8.5.3.19.3      SwStatusCode
This attribute indicates the result of the latest config file CVC verification, SNMP CVC verification, or code file
verification.
    •     'configFileCvcVerified' - This status indicates that the CVC file received via config file has been verified.
    •     'configFileCvcRejected' - This status indicates that the CVC file received via config file has been rejected.
    •     'snmpCvcVerified' - This status indicates that the CVC file received via SNMP has been verified.
    •     'snmpFileCvcRejected' - This status indicates that the CVC file received via SNMP has been rejected.
    •     'codeFileVerified' - This status indicates the code file has been verified.
    •     'codeFileRejected' - This status indicates the code file has been rejected.



04/29/26                                              CableLabs                                                        105
<!-- PDF_PAGE: 106 -->
CM-SP-CM-OSSIv4.0-I13-260429                                           Data-Over-Cable Service Interface Specifications


      •   'other' - other status
References: docsBpi2CodeDownloadStatusCode [RFC 4131]

8.5.3.19.4       SwStatusString
The value of this attribute indicates the additional information to the status code. The value will include the error
code and error description, which will be defined separately.
References: docsBpi2CodeDownloadStatusString [RFC 4131]

8.5.3.20 SshKeyManagement
The SshKeyManagement object serves as the root for the CM's SSH Key Management functions.
References: docsSecCmSshKeyManagement [DOCS-SEC-MIB]
                                     Table 60 - SshKeyManagement Object Associations

      Associated Object                                                    Near-End            Far-End
           Name                                  Type                      Multiplicity       Multiplicity     Label
 SshServer                         Directed composition to SshServer   1                  1



8.5.3.21 SshServer
The SshServer object contains information about the CM Secure Shell server function.
References: docsSecCmSshServer [DOCS-SEC-MIB]
                                           Table 61 - SshServer Object Associations

      Associated Object                                                    Near-End            Far-End
           Name                                  Type                      Multiplicity       Multiplicity     Label
 CdsFileServer                     Directed composition to             1                  1
                                   CdsFileServer
 SshCmCds                          Directed composition to CdsCfg      1                  1
 SccaServerCfg                     Directed composition to             1                  1
                                   SccaServerCfg




106                                                          CableLabs                                        04/29/26
<!-- PDF_PAGE: 107 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


                                          Table 62 - SshServer Object Attributes

   Attribute                                                                                                  Default
    Name                Type          Access                  Type Constraints                   Units         Value
 EnabledInterfa     EnumBits        R/W           customerFacing(0),                            N/A       'operatorFacin
 ces                                              operatorFacing(1)                                       g'
 Status             Enum            R/O           disconnectedNotAllowed(1),                    N/A
                                                  disconnectedProtocolError(2),
                                                  disconnectedKeyExchangeFailed(3),
                                                  disconnectedReserved(4),
                                                  disconnectedMacError(5),
                                                  disconnectedCompressionError(6),
                                                  disconnectedServiceNotAvailable(7),
                                                  disconnectedProtocolVersionNotSupported(8),
                                                  disconnectedHostKeyNotVerifiable(9),
                                                  disconnectedConnectionLost(10),
                                                  disconnectedByApplication(11),
                                                  disconnectedTooManyConnections(12),
                                                  disconnectedAuthCancelledByUser(13),
                                                  disconnectedNoMoreAuthMethods(14),
                                                  disconnectedIllegalUserName(15),
                                                  connected(16),
                                                  disconnectedUnknown(17)
 PublicKey          HexBinary       R/O                                                         N/A
 NewConnectio       UnsignedShort   R/W           0..28800                                      seconds   0
 nTimeout
 InactivityTime     UnsignedShort   R/W           0..86400                                      seconds   1800
 out
 SshSourceAd        IpAddress       R/W
 drRestriction
 SshSourcePre       InetAddressPr   R/W
 fixRestriction     efixLength



8.5.3.21.1        EnabledInterfaces
This attribute configures which interfaces the SSH server function is enabled on in the CM.
If the 'customerFacing' bit (bit 0) is set to '1', the CM allows access from all local (customer premises) network
interfaces/addresses. This includes Ethernet, wireless and MOCA interfaces.
If the 'operatorFacing' bit (bit 1) is set to '1', the CM allows access from all network-facing private
interfaces/addresses (i.e., operator's network).
If both Bit 0 and Bit 1 are set, the CM allows access from both local and private network interfaces/addresses.
See section A.1.2.1.3, SSH Enabled Interfaces, in [SECv4.0] for additional details.
References: docsSecCmSshServerEnabledInterfaces [DOCS-SEC-MIB]

8.5.3.21.2        Status
This attribute reports the status of the connection between the CM SSH server and the SSH client. The possible
values for this attribute are defined in [RFC 4253] and are listed below.
    •     'disconnectedNotAllowed' - SSH client is not allowed to connect to the host.
    •     'disconnectedProtocolError' - SSH client disconnected because of SSH protocol error.
    •     'disconnectedKeyExchangeFailed' - SSH client disconnected because the SSH key exchange failed at the
          SSH transport layer.


04/29/26                                              CableLabs                                                     107
<!-- PDF_PAGE: 108 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


      •   'disconnectedReserved' - Value reserved for future use.
      •   'disconnectedMacError' - SSH client disconnected due to the incompatibility of the Message Authentication
          code algorithm or value.
      •   'disconnectedCompressionError' - SSH client disconnected due to the failure of compression on the packet
          payload when it is required or the incompatibility of the compression algorithm exists.
      •   'disconnectedServiceNotAvailable' - SSH client disconnected because SSH service is not available on the
          server.
      •   'disconnectedProtocolVersionNotSupported' - SSH client disconnected because the SSH protocol version is
          not supported by the server.
      •   'disconnectedHostKeyNotVerifiable' - SSH client disconnected because of using an unverifiable host key.
      •   'disconnectedConnectionLost' - SSH client disconnected because of inactivity.
      •   'disconnectedByApplication' - SSH server disconnected by the SCCA application when performing the
          TLS-based Authentication.
      •   'disconnectedTooManyConnections' - SSH client disconnected because the connections limitation has been
          exceeded.
      •   'disconnectedAuthCancelledByUser' - SSH client disconnected because the authentication is cancelled by
          the user.
      •   'disconnectedNoMoreAuthMethods' - SSH client disconnected because no more authentication methods are
          available.
      •   'disconnectedIllegalUserName' - SSH client disconnected because of an illegal username.
      •   'connected' - Connection between the CM SSH server and the SSH client is active.
      •   'disconnectedUnknown' - SSH client disconnected for unknown or other reason.
References: docsSecCmSshServerStatus [DOCS-SEC-MIB]

8.5.3.21.3     PublicKey
This attribute is the authorized SSH client public key used by the CM to authenticate the client when the client
attempts to set up a CLI SSH connection.
References: docsSecCmSshServerPublicKey [DOCS-SEC-MIB]

8.5.3.21.4     NewConnectionTimeout
This attribute is the new SSH connection timeout provisioned on the CM. When this timeout value is reached, the
CM sets the Enabled attribute to 'false' and stops accepting new SSH connections. Established connections remain
active.
References: docsSecCmSshServerNewConnectionTimeout [DOCS-SEC-MIB]

8.5.3.21.5     InactivityTimeout
This attribute is the SSH inactivity timeout provisioned on the CM. This attribute represents the time at which an
established connection is terminated if there is no activity. Inactivity is defined as the remote side of the connection
timing out and disconnecting. If this attribute is set to zero, the inactivity timeout will be implementation-specific.
References: docsSecCmSshServerInactivityTimeout [DOCS-SEC-MIB]

8.5.3.21.6     SshSourceAddrRestriction
This optional attribute is the SSH source address restriction provisioned on the CM. When this attribute is not
present, the CM enables unrestricted access to the SSH server.


108                                                  CableLabs                                                04/29/26
<!-- PDF_PAGE: 109 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv4.0-I13-260429


References: docsSecCmSshServerSshSourceAddrRestriction [DOCS-SEC-MIB]

8.5.3.21.7    SshSourcePrefixRestriction
This optional attribute is the SSH source address prefix restriction provisioned on the CM. This attribute is a
network/address specifier in CIDR notation that limits the IP addresses where SSH connections can originate.
References: docsSecCmSshServerSshSourcePrefixRestriction [DOCS-SEC-MIB]

8.5.3.22 CdsFileServer
The CdsFileServer object contains information the CM uses to access a CDS server in the operator's network.
References: docsSecCmCdsFileServer [DOCS-SEC-MIB]
                                     Table 63 - CdsFileServer Object Attributes

        Attribute Name               Type          Access          Type Constraints             Units        Default Value
    IpAddr                     IpAddress           R/W                                         N/A
    SshCmCdsDownloadUrl        String              R/W          1..2048                        N/A
    RevocationStatusAction     Enum                R/W          continue(0),                   N/A       'continue'
                                                                reject(1)



8.5.3.22.1    IpAddr
This attribute is the Internet address of the CDS server in the operator's network.
References: docsSecCmCdsFileServerIpAddr [DOCS-SEC-MIB]

8.5.3.22.2    SshCmCdsDownloadUrl
This attribute is the URL of the CDS server in the operator's network.
References: docsSecCmCdsFileServerSshCmCdsDownloadUrl [DOCS-SEC-MIB]

8.5.3.22.3    RevocationStatusAction
This attribute is the action taken by the CM if it does not receive revocation status from the provisioning system
server. The possible values for this object are listed below.
    •    'continue' - Continue operation with the CDS server
    •    'reject' - Reject the connection with the CDS server
References: docsSecCmCdsFileServerRevocationStatusAction [DOCS-SEC-MIB]

8.5.3.23 SshCmCds
The SshCmCds object serves as the root for the CM's SSH Key Management function for configuring the Credential
Data Structure (CDS) for the SNMP-based authentication method.
References: docsSecCmSshCmCds [DOCS-SEC-MIB]
                                    Table 64 - SshCmCds Object Associations

    Associated Object                                                 Near-End                 Far-End
         Name                               Type                      Multiplicity            Multiplicity            Label
 PasswordCredential          Directed composition to              1                    0..*
                             PasswordCredential
 PublicKeyCredential         Directed composition to              1                    0..*
                             PublicKeyCredential




04/29/26                                               CableLabs                                                         109
<!-- PDF_PAGE: 110 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


8.5.3.24 PasswordCredential
The PasswordCredential object contains authorized password credential information for the CM to authenticate SSH
Client CLI connections.
The CM MUST support creation of new instances of the PasswordCredential object and deletion of existing
PasswordCredential object instances.
References: docsSecCmPasswordCredentialTable [DOCS-SEC-MIB]
                                  Table 65 - PasswordCredential Object Attributes

      Attribute Name                       Type                        Access           Type Constraints     Units
  Index                    UnsignedInt                              Key                                     N/A
  UserId                   AdminString                              R/W                                     N/A
  Password                 HexBinary                                R/W              8..128                 N/A
  MacAddr                  MacAddress                               R/W                                     N/A



8.5.3.24.1     Index
This key attribute represents the unique identifier of an instance of this object.
References: docsSecCmPasswordCredentialIndex [DOCS-SEC-MIB]

8.5.3.24.2     UserId
This attribute is the identifier of the user for which the password credential is to be evaluated.
References: docsSecCmPasswordCredentialUserId [DOCS-SEC-MIB]

8.5.3.24.3     Password
This attribute is a string encoded in the [ISO8859-1] character-set and using characters in the range from 0x21 –
0x7E and serving as the credential to be evaluated for the user.
References: docsSecCmPasswordCredentialPassword [DOCS-SEC-MIB]

8.5.3.24.4     MacAddr
This optional attribute is the MAC address assigned to the CM.
References: docsSecCmPasswordCredentialMacAddr [DOCS-SEC-MIB]

8.5.3.25 PublicKeyCredential
The PublicKeyCredential object contains authorized RSA Public Key credential information for the CM to
authenticate SSH Client CLI connections.
The CM MUST support creation of new instances of the PublicKeyCredential object and deletion of existing
PublicKeyCredential object instances.
References: docsSecCmPublicKeyCredentialTable [DOCS-SEC-MIB]
                                  Table 66 - PublicKeyCredential Object Attributes

      Attribute Name                       Type                        Access           Type Constraints     Units
  Index                    UnsignedInt                              Key                                     N/A
  SshPublicKey             HexBinary                                R/W              256..512               N/A
  MacAddr                  MacAddress                               R/W                                     N/A




110                                                  CableLabs                                             04/29/26
<!-- PDF_PAGE: 111 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv4.0-I13-260429


8.5.3.25.1       Index
This key attribute represents the unique identifier of an instance of this object.
References: docsSecCmPublicKeyCredentialIndex [DOCS-SEC-MIB]

8.5.3.25.2       SshPublicKey
This attribute is a string containing a DER-encoded RSA or ECDSA public keys in ASN.1 type, as defined in
[X.509] and serving as the credential to be evaluated for the user.
References: docsSecCmPublicKeyCredentialSshPublicKey [DOCS-SEC-MIB]

8.5.3.25.3       MacAddr
This optional attribute is the MAC address assigned to the CM.
References: docsSecCmPublicKeyCredentialMacAddr [DOCS-SEC-MIB]

8.5.3.26 SccaServerCfg
The SccaServerCfg object contains the information the CM uses to access to SCCA API on HTTPS server in the
operator's network.
References: docsSecCmSccaServerCfg [DOCS-SEC-MIB]
                                     Table 67 - SccaServerCfg Object Attributes

        Attribute Name               Type           Access         Type Constraints         Units   Default Value
    IpAddr                      IpAddress          R/W                                     N/A
    RestApiUrl                  String             R/W          1..2048                    N/A
    RevocationStatusAction      Enum               R/W          continue(0),               N/A      'continue'
                                                                reject(1)



8.5.3.26.1       IpAddr
This attribute is the Internet address of the CDS server in the operator's network.
References: docsSecCmSccaServerCfgIpAddr [DOCS-SEC-MIB]

8.5.3.26.2       RestApiUrl
This attribute is the URL of the SCCA REST API to validate the user credentials.
References: docsSecCmSccaServerCfgRestApiUrl [DOCS-SEC-MIB]

8.5.3.26.3       RevocationStatusAction
This attribute configures the action taken by the CM if it does not receive revocation status from the HTTPS server.
The possible values for this object are listed below.
    •    'continue' - Continue operation with the HTTPS server
    •    'reject' - Reject the connection with the HTTPS server
References: docsSecCmSccaServerCfgRevocationStatusAction [DOCS-SEC-MIB]




04/29/26                                             CableLabs                                                  111
