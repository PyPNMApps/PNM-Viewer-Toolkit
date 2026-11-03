# Annex D.2.3 - PnmCmControl

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 156-156
Classification: Normative (explicit)

<!-- PDF_PAGE: 156 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


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
This mechanism provides the name of the current or last test run and an estimate of the duration of the test. It also
provides an attribute to signal that a test is currently in progress and to limit other tests from starting.
Other PNM tests will work with these objects to control which test has access to the PNM resources.
                                     Table 56 - PnmCmControl Object Attributes

           Attribute Name            Type        Access            Type Constraints               Units      Default
        PnmCmCtlTest              Enum           R/O        other(1),                           N/A         N/A
                                                            dsSpectrumAnalyzer(2)
                                                            dsOfdmSymbolCapture(3),
                                                            dsOfdmChanEstCoef(4),
                                                            dsConstellationDisp(5),
                                                            dsOfdmRxMERPerSubCar(6),
                                                            dsOfdmCodewordErrorRate(7),
                                                            dsHistogram(8),
                                                            usPreEqualizerCoef(9)
        PnmCmCtlTestDuration      UnsignedInt    R/O        N/A                                 seconds     N/A
        PnmCmCtlStatus            Enum           R/O        other(1),                           N/A         N/A
                                                            ready(2),
                                                            testInProgress(3),
                                                            tempReject(4)



           PnmCmCtlTest
This attribute represents the current test. The value could represent the current test in-progress, or if no test is
running, the last test that was attempted.
For any test that is not specifically covered by the enumeration for this object, but manipulates the
'PnmCmCtlStatus' object, the CM MUST set this object to a value of 'other' and update the 'PnmCmCtlTestDuration'
object. The value of this object is not required to persist across device resets. After a reset and before any test is run,
the object will return a value of 'other'.

           PnmCmCtlTestDuration
This attribute represents the number of seconds that the test specified in 'PnmCmCtlTest' spent with a
'PnmCmCtlStatus' of 'testInProgress'. This serves to provide a rough (seconds resolution) estimate of the time spent
under test. If this object is read while the value of 'PnmCmCtlStatus' is 'testInProgress', then the CM MUST return
the number of seconds since the test started in this object. This value is informative only and is not a guarantee of
future performance.


156                                                    CableLabs®                                                 02/19/25
