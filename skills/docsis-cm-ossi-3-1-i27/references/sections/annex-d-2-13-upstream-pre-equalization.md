# Annex D.2.13 - CmUsPreEq

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 187-191
Classification: Normative (explicit)

<!-- PDF_PAGE: 187 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


D.2.13 CmUsPreEq
This object provides access to CM upstream pre-equalizer coefficients. The CM pre-equalizer coefficients and the
CMTS upstream adaptive equalizer coefficient update values, when taken together describe the linear response of
the upstream cable plant for a given CM. During the ranging process, the CMTS computes adaptive equalizer
coefficients based on upstream probes; these coefficients describe the residual channel remaining after any pre-
equalization. The CMTS sends these equalizer coefficients to the CM as a set of Transmit Equalization Adjust
coefficients as part of the ranging process. The CM Pre-Equalizer coefficients are expressed as 16-bit two's
complement numbers using s2.13 format. The power averaged over all coefficients approximately 1, in order to
avoid excessive clipping and quantization noise.
The Pre-Equalizer coefficient update values sent to the CM by the CMTS in the RNG-RSP are expressed as 16-bit
two's complement numbers using S1.14 format.
The CM provides the capability to report its upstream pre-equalizer coefficients (full set or summary) upon request.
The CM also provides the capability to report the most recent set of Transmit Equalization Adjust or Set coefficients
which were applied to produce the reported set of upstream pre-equalizer coefficients. The CM indicates the status
of the most recent Transmit Equalization Adjust or Set coefficients sent to it by the CMTS. If the CM was able to
apply the coefficients, it sets the status to success(2). If the CM was unable to fully apply the adjustments (for
example, due to excess tilt or ripple in the channel), and it was necessary for the CM to clip the coefficients, it sets
the status to clipped(3). If the CM modified the coefficients other than by simply clipping them, it sets the status to
other(1). If for some reason the CM is unable to apply the adjustments at all, it sets the status to rejected(4).
The 'TriggerEnable' attribute is used to create files. Other attributes are updated as read.
This table will have a row for each ifIndex for the modem.
                                       Table 76 - CmUsPreEq Object Attributes

        Attribute Name                Type         Access      Type Constraints                Units       Default
                                                                                                            Value
    IfIndex                     InterfaceIndex    Key
    FileEnable                  Boolean           R/W                                                    False
    AmplitudeRipplePkToPk       ThousandthdB      R/O                                dB
    AmplitudeRippleRMS          ThousandthdB      R/O                                dB
    AmplitudeSlope              Int               R/O                                ThousandthdB/MHz
    AmplitudeMean               ThousandthdB      R/O                                dB
    GroupDelayRipplePkToPk      UnsignedInt       R/O                                0.001 nsec
    GroupDelayRippleRms         UnsignedInt       R/O                                0.001 nsec
    GroupDelaySlope             Int               R/O                                0.001 nsec/MHz
    GroupDelayMean              Int               R/O                                0.001 nsec
    PreEqCoAdjStatus            Enum              R/O          other(1)
                                                               success(2)
                                                               clipped(3)
                                                               rejected(4)
    MeasStatus                  MeasStatusType    R/O
    LastUpdateFileName          AdminString       R/W          SIZE (0..255)                             empty
                                                                                                         string
    FileName                    AdminString       R/W          SIZE (0..255)                             empty
                                                                                                         string



              IfIndex
This attribute is the interface index of the upstream OFDMA interface and is a key to provide an index into the table.




02/19/25                                             CableLabs®                                                      187
<!-- PDF_PAGE: 188 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


           FileEnable
This attribute causes the files defined by the 'Filename' and the 'LastUpdateFilename' attributes to be created. The
files, once created, are available via the 'docsPnmCmBulkFileTable' mechanism. The 'MeasStatus' object can be
checked to determine the status of this attribute.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, this object returns
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
Setting this value to 'true' will change the value of the MeasStatus to 'busy' while the file generation is in progress.
This object returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the CM
is in battery-backup mode.
Default values for Filename and LastUpdateFilename are defined; thus, this object may be set to 'true' without
explicitly setting these values.
This object returns 'true' if the CM is actively generating the files; otherwise, it returns 'false.'
Summary Metrics
The summary metrics described in D.2.13.3 through D.2.13.10 are not calculated unless a specific query of any of
the summary metrics has been performed.

           AmplitudeRipplePkToPk
This attribute represents the value of the peak to peak ripple in the magnitude of the equalizer coefficients. This
attribute represents the ripple across the entire OFDMA channel. This value is not stored in the data files.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

           AmplitudeRippleRMS
This attribute represents the value of the RMS ripple in the magnitude of the equalizer coefficients. This attribute
represents the ripple across the entire OFDMA channel. This value is not stored in the data files.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

           AmplitudeSlope
This attribute represents the slope in 0.001 dB per MHz in the magnitude of the equalizer coefficients. This attribute
represents the slope across the entire OFDMA channel. This value is not stored in the data files.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

           AmplitudeMean
This attribute represents the mean, in 0.001 dB, of the magnitude of the equalizer coefficients.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

           GroupDelayRipplePkToPk
This attribute represents the peak to peak Group Delay Ripple expressed in units of 0.001 nsec. This attribute
represents the group delay variation across the entire OFDMA channel. This value is not stored in the data files. The
slope component calculated for the GroupDelaySlope is subtracted from the frequency domain data and the peak-to-
peak ripple is calculated from the resulting data.

           GroupDelayRippleRMS
This attribute represents the RMS value of the Group Delay Ripple expressed in units of 0.001 nsec. This attribute
represents the group delay variation across the entire OFDMA channel. The slope component calculated for the
GroupDelaySlope is subtracted from the frequency domain data and the RMS ripple is calculated from the resulting
data. This attribute is not stored in the data files.



188                                                    CableLabs®                                               02/19/25
<!-- PDF_PAGE: 189 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


           GroupDelaySlope
This attribute represents the slope in 0.001 nsec per MHz in the group delay of the equalizer coefficients. This
attribute represents the slope across the entire OFDM channel.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

              GroupDelayMean
This attribute represents the mean of the group delay in units of 0.001 nsec.

              PreEquCoAdjStatus
This attribute represents whether the last set of Pre-Equalization coefficient adjustments were fully applied or were
only partially applied due to excessive ripple or tilt in the Pre-Equalization coefficient values.

              MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.

              LastUpdateFileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. The data represents the
data sent to the CM by the CMTS in the last RNG-RSP that contained Pre-Equalization Adjust or Set values and are
stored as 16-bit integers for the I and Q data.
This object cannot be changed while a file generation is in progress. It will return a value of 'inconsistentValue' if set
while the value of MeasStatus is set to a value of 'busy'.
If the value of this object is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, it is generated as the test name plus the CM MAC Address plus the 'epoch time'.
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.
Hence, the format would be:
    PNMUsPreEqLastUpdate_<CM MAC address>_<epoch>
    For example: PNMUsPreEqLastUpdate_0010181A2D11_1403405123
The data file is composed of a header plus the Pre-EQ Update Data. The header is composed of ordered fixed-length
fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field. If
necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                    Table 77 - Last PreEqualization Update File Format

                                                   Element                       Size
                                    File type (value = 504E4E07)        4 bytes
                                    Major Version (Value = 1)           1 byte
                                    Minor Version (Value = 0)           1 byte
                                    Capture Time                        4 bytes
                                    Upstream Channel Id                 1 byte
                                    CM MAC Address                      6 bytes
                                    CMTS MAC Address                    6 bytes
                                    Subcarrier zero frequency in Hz     4 bytes



02/19/25                                                  CableLabs®                                                  189
<!-- PDF_PAGE: 190 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


                                              Element                          Size
                                FirstActiveSubcarrierIndex            2 bytes
                                Subcarrier spacing in kHz             1 byte
                                Length in bytes of Pre-EQ data        4 bytes
                                Pre-EQ Coefficient Update data        ComplexData



D.2.13.13.1 File Header Element Definitions

D.2.13.13.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.13.13.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.13.13.1.3 Collection Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.

D.2.13.13.1.4 Upstream Channel Id
This element represents the Upstream Channel Id for which the Upstream PreEqualization Coefficient adjustment
values apply.

D.2.13.13.1.5 CM MAC Address
The CM MAC Address.

D.2.13.13.1.6 CMTS MAC Address
This element contains the MAC Address of the downstream channel for which the CM is receiving MAPs and
UCDs.

D.2.13.13.1.7 Subcarrier Zero Frequency in Hz
This element is the center frequency of subcarrier zero of the OFDM channel.

D.2.13.13.1.8 FirstActiveSubcarrierIndex
This element is the subcarrier index of the lowest subcarrier in the Encompassed Spectrum of the channel.

D.2.13.13.1.9 Subcarrier Spacing in kHz
This element is the OFDM subcarrier spacing.

D.2.13.13.1.10 Length in Bytes of PreEq Data
This element is the number of bytes of the PreEqData.

D.2.13.13.1.11 Pre-Eq Coefficient Update Data
This element refers to the complexData values of the Pre-Equalization Coefficient sent to the CM by the CMTS in
the last RNG-RSP message which contained Pre-EQ coefficients. The data is expressed in s1.14 fixed point
notation.




190                                                  CableLabs®                                             02/19/25
<!-- PDF_PAGE: 191 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


              FileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. The data represents the
current set of the Pre-Equalization values for all of the subcarriers within the Encompassed Spectrum and are stored
as 16-bit integers for the I and Q data.
All formats are 16 bits on each of I and Q. 'sm.n' means sign bit, m integer bits, and n fractional bits.
Examples:
    With s1.14 format, the numerical value '1' corresponds to hex pattern 0x4000.
    With s2.13 format, the numerical value '1' corresponds to hex pattern 0x2000.
    With s3.12 format, the numerical value '1' corresponds to hex pattern 0x1000.
Positive or negative values exceeding the number format are clipped on I and Q independently (no rollover).
This object cannot be changed while a file generation is in progress. It will return a value of 'inconsistentValue' if set
while the value of MeasStatus is set to a value of 'busy'.
If the value of this object is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, it is generated as the test name plus the CM MAC Address plus the 'epoch time'.
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970. Hence, the format would be:
    PNMUsPreEq_<CM MAC address>_<epoch>
    For example: PNMUsPreEq_0010181A2D11_1403405123
The data file is composed of a header plus the Pre-EQ Data. The header is composed of ordered fixed-length fields.
Unless otherwise specified, the header fields contain hex values that are right-justified within the field. If necessary,
the field is left-padded with zero values.
Syntax of the file is as follows:
                                    Table 78 - Upstream PreEqualization File Format

                                                  Element                        Size
                                    File type (value = 504E4E06)        4 bytes
                                    Major Version (Value = 1)           1 byte
                                    Minor Version (Value = 0)           1 byte
                                    Collection Time                     4 bytes
                                    Upstream Channel Id                 1 byte
                                    CM MAC Address                      6 bytes
                                    CMTS MAC Address                    6 bytes
                                    Subcarrier zero frequency in Hz     4 bytes
                                    FirstActiveSubcarrierIndex          2 bytes
                                    Subcarrier spacing in kHz           1 byte
                                    Length in bytes of Pre-EQ data      4 bytes
                                    Pre-EQ Coefficient Data             ComplexData




02/19/25                                                  CableLabs®                                                  191
