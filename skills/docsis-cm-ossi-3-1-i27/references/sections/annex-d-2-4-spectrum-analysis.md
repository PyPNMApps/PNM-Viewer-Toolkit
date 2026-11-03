# Annex D.2.4 - CM Spectrum Analysis Objects

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 157-161
Classification: Normative (explicit)

<!-- PDF_PAGE: 157 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


           PnmCmCtlStatus
This attribute represents the overall status of the PNM test platform.
Individual tests within the PNM test suite have their own specific objects to start and stop. For each test, defined by
the data-type enumeration for 'PnmCmCtlTest', the CM MUST first check the status of this object before starting. If
this object is set to any value other than 'ready', the CM MUST NOT start the test. If the CM was not able to start the
test and the test has a 'MeasStatusType' object, the CM MUST set 'MeasStatusType' to a value of
'resourceUnavailable'.
If the test is allowed to start, the CM MUST change the value of this object to 'testInProgress'. When the test is no
longer in progress, the CM MUST change the value of this object to a value other than 'testInProgress'.
This mechanism serves to allow only one instance of only one test to run at a time. There may be some tests that are
not included by this control object. That is, if any test is not defined by the data-type enumeration for
'PnmCmCtlTest', then it is not included (unless otherwise stated by the test definition) and is not required to check or
to change this object. If a test is not included or covered by this set of objects, then it is allowed to run regardless of
any other tests in progress.
The possible values for this object are:
    'other' - Any condition not otherwise defined
    'ready' - The PNM platform is capable and ready to support a test; a test can be initiated
    'testInProgress' - A PNM test is currently in progress
    'tempReject' - A temporary condition exists that prohibits a test from starting, e.g., DBC

D.2.4     CM Spectrum Analysis Objects
This group of objects provides a CM downstream spectrum analysis function. Each measurement is a data collection
event that provides the energy content of the signal at each frequency within a specified range. The result of a
measurement is a table consisting of one or more rows. Each row corresponds to a capture of spectral data across a
specified segment bandwidth. The frequency range of each segment is divided into bins, which are a discrete set of
evenly spaced frequencies across the band. The width of each bin (resolution bandwidth) is generally equal to or
slightly greater than the spacing between bins. Each bin has an associated amplitude value in the table, which
represents the amount of energy measured in that frequency bin. The segments are constrained to be contiguous; that
is, the start frequency of each segment equals the end frequency of the previous segment plus the bin spacing. Thus,
the concatenation of all segments results in a wideband spectral analysis. The measurement table is updated at a rate
that is vendor-specific. The measurement generally occurs prior to the point at which the received signal is
demodulated. The measurement spectrum may or may not include the effects of receiver processing such as gain
control, RF filtering, and matched filtering.
The CM MUST implement the CmSpectrumAnalysisCtrlCmd object.
The CM MUST implement the CmSpectrumAnalysisMeas object.

           CmSpectrumAnalysisCtrlCmd
This object is used to configure the frequency spectral analysis in the CM.
                            Table 57 - CmSpectrumAnalysisCtrlCmd Object Attributes

        Attribute Name                Type         Access     Type Constraints                Units              Default
 Enable                         Boolean           R/W                                                          false
 InactivityTimeout              Int               R/W         0..86400              seconds                    300
 FirstSegmentCenterFrequency    UnsignedInt       R/W                               Hz                         93000000
 LastSegmentCenterFrequency     UnsignedInt       R/W                               Hz                         993000000
 SegmentFrequencySpan           UnsignedInt       R/W         1000000..900000000    Hz                         7500000
 NumBinsPerSegment              UnsignedShort     R/W         2..2048               bins-per-segment           256




02/19/25                                              CableLabs®                                                       157
<!-- PDF_PAGE: 158 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


       Attribute Name                Type          Access     Type Constraints                Units                    Default
 EquivalentNoiseBandwidth       UnsignedShort     R/W         50..500                Hundredths of bin spacing     150
 WindowFunction                 Enum              R/W         other(0),
                                                              hann(1),
                                                              blackmanHarris(2),
                                                              rectangular(3),
                                                              hamming(4),
                                                              flatTop(5),
                                                              gaussian(6),
                                                              chebyshev(7)
 NumberOfAverages               UnsignedShort     R/W         1..1000                                              1
 FileEnable                     Boolean           R/W                                                              false
 MeasStatus                     MeasStatusType    R/O
 FileName                       AdminString       R/W         SIZE(1..255)



D.2.4.1.1 Enable
This attribute is used to enable or disable the spectrum analyzer feature. Setting this attribute to true triggers the CM
to initiate measurements for the spectrum analyzer feature based on the other configuration attributes for the feature.
By default, the feature is disabled unless explicitly enabled. Note that the feature may be disabled by the system
under certain circumstances if the spectrum analyzer would affect critical services. In such a case, the attribute will
return 'false' when read, and will reject sets to 'true' with an error. Once the feature is enabled, any configuration
operations (e.g., write operations to configuration objects) might not be effective until the feature is re-enabled.

D.2.4.1.2 InactivityTimeout
This attribute controls the length of time after the last spectrum analysis measurement before the feature is
automatically disabled. If set to a value of 0, the feature will remain enabled until it is explicitly disabled.

D.2.4.1.3 FirstSegmentCenterFrequency
This attribute controls the center frequency of the first segment for the spectrum analysis measurement.
The frequency bins for this segment lie symmetrically to the left and right of this center frequency. If the number of
bins in a segment is odd, the segment center frequency lies directly on the center bin. If the number of bins in a
segment is even, the segment center frequency lies halfway between two bins.
Changing the value of this object may result in changes to the CmSpectrumAnalysisMeas object, as described in the
description field for the object.
Note that if this object is set to an invalid value, the device may return an error of inconsistentValue, or may adjust
the value of the object to the closest valid value.

D.2.4.1.4 LastSegmentCenterFrequency
This attribute controls the center frequency of the last segment of the spectrum analysis measurement.
The frequency bins for this segment lie symmetrically to the left and right of this center frequency. If the number of
bins in a segment is odd, the segment center frequency lies directly on the center bin. If the number of bins in a
segment is even, the segment center frequency lies halfway between two bins.
The value of the LastSegmentCenterFrequency is typically equal to the FirstSegmentCenterFrequency plus an
integer number of segment spans as determined by the SegmentFrequencySpan.
Changing the value of this object may result in changes to the CmSpectrumAnalysisMeas object, as described in the
description field for the object.
Note that if this attribute is set to an invalid value, the device may return an error of inconsistentValue, or may adjust
the value of the attribute to the closest valid value.


158                                                   CableLabs®                                                   02/19/25
<!-- PDF_PAGE: 159 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


D.2.4.1.5 SegmentFrequencySpan
This attribute controls the frequency span of each segment (instance) of the CmSpectrumAnalysisMeas object. If set
to a value of 0, then a default span will be chosen based on the hardware capabilities of the device. Segments are
contiguous from the FirstSegmentCenterFrequency to the LastSegmentCenterFrequency and the center frequency
for each successive segment is incremented by the SegmentFrequencySpan. The number of segments is
(LastSegmentCenterFrequency - FirstSegmentCenterFrequency)/SegmentFrequencySpan + 1. A segment is
equivalent to an instance in the CmSpectrumAnalysisMeas object. The chosen SegmentFrequencySpan affects the
number of instances in the CmSpectrumAnalysisMeas object. A more granular SegmentFrequencySpan may
adversely affect the amount of time needed to query the instances in addition to possibly increasing the acquisition
time.
Changing the value of this object may result in changes to the CmSpectrumAnalysisMeas object, as described in the
description field for the object
Note that if this attribute is set to an invalid value, the device may return an error of inconsistentValue, or may adjust
the value of the attribute to the closest valid value.

D.2.4.1.6 NumBinsPerSegment
This attribute controls the number of bins collected by the measurement performed for each segment (instance) of
the CmSpectrumAnalysisMeas object.
Note that if this attribute is set to an invalid value, the device may return an error of inconsistentValue, or may adjust
the value of the attribute to the closest valid value.

D.2.4.1.7 EquivalentNoiseBandwidth
This attribute allows the user to request an equivalent noise bandwidth for the resolution bandwidth filter used in the
spectrum analysis. This corresponds to the spectral width of the window function used when performing a discrete
Fourier transform for the analysis.
The window function which corresponds to a value written to this attribute may be obtained by reading the value of
the WindowFunction attribute.
If an unsupported value is requested, the device may return an error of inconsistentValue, or choose the closest valid
value to the one which is requested. If the closest value is chosen, then a subsequent read of this attribute will return
the actual value that is in use.

D.2.4.1.8 WindowFunction
This attribute controls or indicates the windowing function that will be used when performing the discrete Fourier
transform for the analysis. The WindowFunction and the EquivalentNoiseBandwidth are related. If a particular
WindowFunction is selected, then the EquivalentNoiseBandwidth for the function in use will be reported by the
EquivalentNoiseBandwidth attribute. Alternatively, if an EquivalentNoiseBandwidth value is chosen and a
WindowFunction function representing that EquivalentNoiseBandwidth is defined in the CM, that value will be
reported in the WindowFunction object, or a value of 'other' will be reported. Use of "modern" windowing functions
not yet defined will likely be reported as 'other'.
Note that all window functions may not be supported by all devices. If an attempt is made to set the attribute to an
unsupported window function, or if writing of the WindowFunction object is not supported by an implementation,
an error will be returned.

D.2.4.1.9 NumberOfAverages
This attribute controls the number of averages that will be performed on spectral bins. The average will be computed
using the "leaky integrator" method, where reported bin value = alpha*accumulated bin values + (1-alpha)*current
bin value. Alpha is one minus the reciprocal of the number of averages. For example, if N=25, then alpha = 0.96. A
value of 1 indicates no averaging. Re-writing the number of averages will restart the averaging process. If there are
no accumulated values, the accumulators are made equal to the first measured bin amplitudes.




02/19/25                                             CableLabs®                                                      159
<!-- PDF_PAGE: 160 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


The number of averages will be set by writing NumberOfAverages attribute. If an attempt is made to set the attribute
to an unsupported number of averages, an error of inconsistentValue will be returned.

D.2.4.1.10 FileEnable
This attribute, when set to 'true', causes the CM to begin a Spectrum Analysis measurement with the parameters
defined by the CmSpectrumAnalysisCtrlCmd set of attributes. In order to set FileEnable to true, the Enable must
already be set to true.
When the measurement is completed successfully, a file is generated and is made available for transfer and the
MeasStatus attribute is set to 'sampleReady'. The file will contain one complete snapshot of the spectrum data.
Setting this object to a value of 'false' instructs the CM to stop the measurement.

D.2.4.1.11 MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.

D.2.4.1.12 Filename
This attribute is the name of the file, at the CM and containing the spectrum analysis data, which is to be
downloaded by the PNM server.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.
If the value of this object is the DEFVAL (empty string), then a default filename value will be used. Otherwise, the
value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, it is generated as the test name plus the CM MAC Address plus the 'epoch time'.
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.
Hence, the format would be:
      PNMSpecAnData_<CM MAC address>_<epoch>
      For example: PNMSpecAnData_0010181A2D11_1403405123
The data file is composed of a header plus the Spectrum Analysis Data. The header is composed of ordered fixed-
length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field.
If necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                      Table 58 - CM Spectrum Analysis File Format

                                            Element                                Size
                         File type (value = 504E4E09)                    4 bytes
                         Major Version (value = 1)                       1 byte
                         Minor Version                                   1 byte
                         Capture Time                                    4 bytes
                         Channel ID                                      1 byte
                         CM MAC Address                                  6 bytes
                         FirstSegmentCenterFrequency                     4 bytes
                         LastSegmentCenterFrequency                      4 bytes
                         SegmentFrequencySpan                            4 bytes
                         NumBinsPerSegment                               2 bytes




160                                                     CableLabs®                                              02/19/25
<!-- PDF_PAGE: 161 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


                                           Element                               Size
                         EquivalentNoiseBandWidth                      2 bytes
                         WindowFunction                                2 bytes
                         Length (in bytes) of SpectrumAnalysis Data    4 bytes
                         SpectrumAnalysisData                          BinAmplitudeFileData


If the size of the data the CM is commanded to collect would exceed the maximum file-size limit of the data
collection mechanism, the CM will limit the file size accordingly and set the 'MeasStatus' attribute to a value of
'sampleTruncated'. If this occurs, the file will contain valid data, from the beginning of the capture, but will
represent fewer bins than configured for the measurement.

D.2.4.1.13 File Header Element Definitions

D.2.4.1.13.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.4.1.13.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.4.1.13.3 Capture Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC),Thursday, 1 January 1970.

D.2.4.1.13.4 Channel ID
Channel ID is not valid for this file type; must be set to 0.

D.2.4.1.13.5 CM MAC Address
The CM MAC address.

D.2.4.1.13.6 FirstSegmentCenterFrequency
This element is a copy of the FirstSegmentCenterFrequency attribute.

D.2.4.1.13.7 LastSegmentCenterFrequency
This element is a copy of the LastSegmentCenterFrequency attribute.

D.2.4.1.13.8 SegmentFrequencySpan
This element is a copy of the SegmentFrequencySpan attribute.

D.2.4.1.13.9 NumBinsPerSegment
This element is a copy of the NumBinsPerSegment attribute.

D.2.4.1.13.10 EquivalentNoiseBandWidth
This element is a copy of the EquivalentNoiseBandWidth attribute.

D.2.4.1.13.11 WindowFunction
This element is a copy of the attribute.




02/19/25                                              CableLabs®                                                     161
