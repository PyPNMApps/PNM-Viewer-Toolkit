# Annex D.2.11 - CmDsOfdmRequiredQamMer

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 221-222
Classification: Normative (explicit)

<!-- PDF_PAGE: 221 -->
Cable Modem Operations Support System Interface Specification                CM-SP-CM-OSSIv4.0-I13-260429


D.2.10.5.1.5 SummaryType
This element is a copy of the Summary Type attribute for the MIB.

D.2.10.5.1.6 NumberOfProfiles
This element is number of profiles in the CM for which Codeword summaries are provided. This number indicates
how many instances of the FecSummaryData type are included in the file.
The CM MUST include FEC data arrays for every profile assigned by the CCAP.

D.2.10.5.1.7 FEC Data
The set of FEC data encoded using the FecSummaryData type. There is one set of data per Profile ID as indicated by
the NumberOfProfiles.
Example:
This is an example showing the FECData encoding for two profiles. In this example, there are only 5 codeword sets
for each profile, whereas normally there would be 600 or 1440 depending on the Summary Type. The four columns
of data correspond to the values for the CodewordSetTimeStamp, TotalCodewords, CorrectedCodewords and
UncorrectableCodewords, respectively. The values in the example are expressed as decimal integer values, but they
are encoded using 32-bit hexadecimal notation in the file. The order of codewordSets in the file is from oldest to
newest, meaning that the codewordSet following the Header Elements is the oldest set.
Profile Id = 255
Number of Codeword Sets = 5
  1456252719            5311230          0    0
  1456252720            5268830          0    0
  1456252721            5226430          0    0
  1456252722            5184030          0    0
  1456252723            5140830          0    0

Profile Id = 0
Number of Codeword Sets = 5
  1456252719            221896           0    0
  1456252720            220128           0    0
  1456252721            218353           0    0
  1456252722            216587           0    0
  1456252723            214777           0    0

Hex Notation:
ff 05 56 CC A7 2F 00 51 0A FE 00 00 00 00 00 00 00 00 56 CC A7 30 00 50 65 5E 00 00 00 00 00 00 00 00, etc.

D.2.11 CmDsOfdmRequiredQamMer
The purpose of this item is to provide a target MER value for each downstream OFDM modulation order to be used
in determining the SNR Margin for the Candidate Downstream Profile. The QamMer attributes are expressed in
units of a quarter dB.
                           Table 105 - CmDsOfdmRequiredQamMer Object Attributes

       Attribute Name             Type        Access      Type Constraints          Units        Default Value
    ReqMerQam16            UnsignedInt       R/W                                quarterDb       60
    ReqMerQam64            UnsignedInt       R/W                                quarterDb       84
    ReqMerQam128           UnsignedInt       R/W                                quarterDb       96




04/29/26                                           CableLabs                                                  221
<!-- PDF_PAGE: 222 -->
CM-SP-CM-OSSIv4.0-I13-260429                                 Data-Over-Cable Service Interface Specifications


        Attribute Name         Type          Access       Type Constraints          Units       Default Value
      ReqMerQam256         UnsignedInt     R/W                                 quarterDb        108
      ReqMerQam512         UnsignedInt     R/W                                 quarterDb        122
      ReqMerQam1024        UnsignedInt     R/W                                 quarterDb        136
      ReqMerQam2048        UnsignedInt     R/W                                 quarterDb        148
      ReqMerQam4096        UnsignedInt     R/W                                 quarterDb        164
      ReqMerQam8192        UnsignedInt     R/W                                 quarterDb        184
      ReqMerQam16384       UnsignedInt     R/W                                 quarterDb        208



           ReqMerQam16
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam64
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam128
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam256
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam512
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam1024
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam2048
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam4096
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

           ReqMerQam8192
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.

             ReqMerQam16384
This attribute represents the minimum required MER value for this Modulation Order. It is used in determining the
SNR Margin for the Candidate Downstream Profile.


222                                               CableLabs                                             04/29/26
