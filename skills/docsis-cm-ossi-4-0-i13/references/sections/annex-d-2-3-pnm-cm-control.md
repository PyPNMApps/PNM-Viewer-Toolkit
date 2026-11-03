# Annex D.2.3 - PnmCmControl

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 194-195
Classification: Normative (explicit)

<!-- PDF_PAGE: 194 -->
CM-SP-CM-OSSIv4.0-I13-260429                                       Data-Over-Cable Service Interface Specifications


                                   Table 87 - Subcarrier Skip Modulation Scheme

                                             Element                          Size
                                   Subcarrier Assignment            1 byte (Value 0x01)
                                   Scheme Type
                                   Main Modulation Order            1 byte
                                   Skip Modulation Order            1 byte
                                   Skip Modulation Number of        2 bytes
                                   Subcarriers



D.2.2    PnmCaptureFile
PNM tests usually generate files to report measurements or test results. The results file includes header information
that is common to all types of PNM tests and fields, and data that are specific to the type of PNM test. The abstract
PnmCaptureFile object defines the attributes and format of the header information common to all PNM test files
described below. The File header fields are right-justified within the field and left-padded with zero values if
necessary.
Determination of maximum size of the PNM Capture File is left to vendor implementation.
Table 88 shows the format of the PNM capture file header, applied to any data file created by a PNM test.
                                    Table 88 - PnmCaptureFile Object Attributes

 Attribute Name           Type                 Access           Type Constraints                Units   Default Value
 FileType           HexBinary            R/O                   SIZE (4)                   N/A           N/A
 MajorVersion       UnsignedByte         R/O                   SIZE (1)                   N/A           N/A
 MinorVersion       UnsignedByte         R/O                   SIZE (1)                   N/A           N/A
 CaptureTime        UnsignedInt          R/O                   SIZE (4)                   N/A           N/A



            FileType
A four-byte hexadecimal identifier specific to the type of PNM test that generated the data file. Each FileType is
registered in [CANN]. Filetypes prefixed by PNM (i.e., 504E4D) do not contain the MajorVersion and
MinorVersion header elements. Filetypes prefixed with PNN (i.e., 504E4E) contain the MajorVersion and
MinorVersion header elements.

            MajorVersion
This attribute represents the file header version. This value is incremented by one when the header format is
modified by this specification.

            MinorVersion
This attribute is reserved for vendor-specific and vendor-defined version information.

            CaptureTime
A four-byte hexadecimal field representing the epoch time (also known as 'unix time') which is the number of
seconds that have elapsed since midnight Coordinated Universal Time (UTC), Thursday, 1 January 1970.

D.2.3    PnmCmControl
PNM tests have the potential to stress the limited resources of a CM. Hence, certain PNM tests should not be
allowed to run in parallel. This set of attributes serves to serialize PNM tests within the CM. No data from these
control objects is required to persist across device resets.




194                                                     CableLabs                                            04/29/26
<!-- PDF_PAGE: 195 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


This mechanism provides the name of the current or last test run and an estimate of the duration of the test. It also
provides an attribute to signal that a test is currently in progress and to limit other tests from starting.
Other PNM tests will work with these objects to control which test has access to the PNM resources.
                                     Table 89 - PnmCmControl Object Attributes

    Attribute Name            Type        Access             Type Constraints                  Units       Default Value
 PnmCmCtlTest              Enum          R/O           other(1),                         N/A               N/A
                                                       dsSpectrumAnalyzer(2),
                                                       dsOfdmSymbolCapture(3),
                                                       dsOfdmChanEstCoef(4),
                                                       dsConstellationDisp(5),
                                                       dsOfdmRxMERPerSubCar(6),
                                                       dsOfdmCodewordErrorRate(7),
                                                       dsHistogram(8),
                                                       usPreEqualizerCoef(9)
 PnmCmCtlTestDuration      UnsignedInt   R/O           N/A                               seconds           N/A
 PnmCmCtlStatus            Enum          R/O           other(1),                         N/A               N/A
                                                       ready(2),
                                                       testInProgress(3),
                                                       tempReject(4)



           PnmCmCtlTest
This attribute represents the current PNM test. The value could represent the current test in-progress, or if no test is
running, the last test that was attempted.
For any test that is not specifically covered by the enumeration for this object, but manipulates the
'PnmCmCtlStatus' object, the CM MUST set this object to a value of 'other' and update the 'PnmCmCtlTestDuration'
object. The value of this object is not required to persist across device resets. After a reset and before any test is run,
the object will return a value of 'other'.
Possible values for this object are listed below.
    •    other(1)—Any PNM test not covered by other defined values
    •    dsSpectrumAnalyzer(2)—Downstream Spectrum Analysis described in Annex D.2.4, CM Spectrum
         Analysis Objects
    •    dsOfdmSymbolCapture(3)—Downstream OFDM Symbol Capture described in Annex D.2.5,
         CmSymbolCapture
    •    dsOfdmChanEstCoef(4)—Downstream OFDM Channel Estimate Coefficient described in Annex D.2.6,
         CmDsOfdmChanEstimateCoef
    •    dsConstellationDisp(5)—Downstream Constellation Display described in Annex D.2.7,
         CmDsConstDispMeas
    •    dsOfdmRxMERPerSubCar(6)—Downstream OFDM Receive Modulation Error Ratio per Subcarrier
         described in Annex D.2.8, CmDsOfdmRxMer
    •    dsOfdmCodewordErrorRate(7)—Downstream OFDM Codeword Error Rate described in Annex D.2.10,
         CmDsOfdmFecSummary
    •    dsHistogram(8)—Downstream Histogram described in Annex D.2.12, CmDsHist
    •    usPreEqualizerCoef(9)—Upstream Pre-equalizer Coefficients described in Annex D.2.13, CmUsPreEq




04/29/26                                              CableLabs                                                      195
