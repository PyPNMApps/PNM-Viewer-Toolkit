# Annex D.2.6 - CmDsOfdmChanEstimateCoef

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 205-208
Classification: Normative (explicit)

<!-- PDF_PAGE: 205 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


D.2.5.10.1.6 Subcarrier Zero Frequency in Hz
The center frequency of subcarrier zero of the OFDM channel.

D.2.5.10.1.7 SampleRate in Hz
This element is a copy of the SampleRate attribute.

D.2.5.10.1.8 FFT Size
This element is a copy of the FftLength attribute.

D.2.5.10.1.9 TriggerGroup ID
This element is a copy of the TriggerGroupId attribute.

D.2.5.10.1.10 Transaction ID
This element is a copy of the TransactionId attribute.

D.2.5.10.1.11 Length (in Bytes) of Captured Data
This element indicates the size of the complexData which follows.

D.2.5.10.1.12 Capture Data
This element refers to the complexData values of the CM Symbol Capture. The data is expressed in s3.12 fixed
point notation. The CM should adjust the scaling such that the average power of the samples is approximately 1, in
order to avoid excessive clipping and quantization noise.

D.2.6      CmDsOfdmChanEstimateCoef
The purpose of this table is for the CM to report its estimate of the downstream channel response. The reciprocals of
the channel response coefficients are typically used by the CM as its frequency-domain downstream equalizer
coefficients. The channel estimate consists of a single complex value per subcarrier. The channel response
coefficients are expressed as 16-bit two's complement numbers using 2.13 format. The CM samples are scaled such
that the average power of the samples is approximately 1, in order to avoid excessive clipping and quantization
noise.
Summary metrics (slope, ripple, and mean) are defined in order to avoid having to send all coefficients on every
query. The summary metrics are calculated when the corresponding MIB is queried. A Coefficient filename and
trigger are provided to obtain the channel coefficients.
This table will have a row for each ifIndex for the modem.
                           Table 95 - CmDsOfdmChanEstimateCoef Object Attributes

                                                                                                           Default
     Attribute Name              Type           Access         Type Constraints               Units        Value
 IfIndex                   InterfaceIndex     Key
 TriggerEnable             Boolean            R/W                                                        False
 AmplitudeRipplePkToPk     ThousandthdB       R/O                                        dB
 AmplitudeRippleRms        ThousandthdB       R/O                                        dB
 AmplitudeSlope            Int                R/O                                        ThousandthdB/
                                                                                         MHz
 AmplitudeMean             ThousandthdB       R/O                                        dB
 GroupDelayRipplePkToP     UnsignedInt        R/O                                        0.001 nsec
 k
 GroupDelayRippleRms       UnsignedInt        R/O                                        0.001 nsec




04/29/26                                              CableLabs                                                   205
<!-- PDF_PAGE: 206 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


                                                                                                                 Default
      Attribute Name              Type            Access           Type Constraints              Units            Value
 GroupDelaySlope            Int                 R/O                                          0.001
                                                                                             nsec/MHz
 GroupDelayMean             Int                 R/O                                          0.001 nsec
 MeasStatus                 MeasStatusType      R/O
 FileName                   AdminString         R/W           SIZE (0..255)                                  empty string



            IfIndex
This attribute is the interface index of the OFDM downstream channel interface and is a key to provide an index into
the table.

            TriggerEnable
Setting this object to a value of 'true' instructs the CM to begin collection and storing the channel estimate
coefficients into the file specified by the FileName object.
Setting this object to a value of 'false' instructs the CM to stop storing channel estimate coefficients in the file. When
read, this object returns 'true' if the CM is actively storing channel estimate coefficients in the file. Otherwise, it
returns 'false'.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, this object returns
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
A default value for FileName is defined; thus, this object may be set to 'true' without explicitly setting that value.
Setting this attribute to a value of 'true' will change the value of MeasStatus to 'busy'.

            AmplitudeRipplePkToPk
This attribute represents the value of the peak to peak ripple in the magnitude of the equalizer coefficients
[PHYv4.0]. The slope component calculated for the AmplitudeSlope is subtracted from the frequency domain data
and the peak to peak ripple is calculated from the resultant data. This attribute represents the ripple across the entire
OFDM channel.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

            AmplitudeRippleRms
This attribute represents the value of the RMS ripple in the magnitude of the equalizer coefficients. The slope
component calculated for the AmplitudeSlope is subtracted from the frequency domain data and the RMS ripple is
calculated from the resultant data. This attribute represents the ripple across the entire OFDM channel.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

            AmplitudeSlope
This attribute represents the slope in 0.001 dB per MHz in the magnitude of the equalizer coefficients. The slope is
calculated as the slope of a linear least squares fit of the frequency domain data. This attribute represents the slope
across the entire OFDM channel.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

            AmplitudeMean
This attribute represents the Mean, in 0.001 dB, of the magnitude of the equalizer coefficients.




206                                                   CableLabs                                                 04/29/26
<!-- PDF_PAGE: 207 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


           GroupDelayRipplePkToPk
This attribute represents the peak to peak Group Delay Ripple expressed in units of 0.001 nsec. This attribute
represents the group delay variation across the entire OFDM channel. The slope component calculated for the
GroupDelaySlope is subtracted from the frequency domain data and the peak-to-peak ripple is calculated from the
resulting data.

           GroupDelayRippleRMS
This attribute represents the RMS value of the Group Delay Ripple expressed in units of 0.001 nsec. This attribute
represents the group delay variation across the entire OFDM channel. The slope component calculated for the
GroupDelaySlope is subtracted from the frequency domain data and the RMS ripple is calculated from the resulting
data. This attribute is not stored in the data file.

           GroupDelaySlope
This attribute represents the slope in 0.001 nsec per MHz in the group delay of the equalizer coefficients. This
attribute represents the slope across the entire OFDM channel.
NOTE: An algorithm for calculating the ripple and slope for these measurements is provided in Annex D.4.

           GroupDelayMean
This attribute represents the mean of the group delay, in units of 0.001 nsec.

           MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.

           FileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. The data is stored as
16-bit integers for the I and Q data.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.
If the value of this object is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, it is generated as the test name plus the CM MAC Address plus the 'epoch time'.
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970. Hence, the format would be:
    PNMChEstCoef_<CM MAC address>_<epoch>
For example: PNMChEstCoef_0010181A2D11_1403405123
The data file is composed of a header plus the Chan Estimate Coefficient Data. The header is composed of ordered
fixed-length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the
field. If necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                Table 96 - Channel Estimate Coefficient File Format

                                                  Element                        Size
                                    File type (value = 504E4E02)        4 bytes
                                    Major Version (Value = 1)           1 byte
                                    Minor Version (Value = 0)           1 byte



04/29/26                                                 CableLabs                                                   207
<!-- PDF_PAGE: 208 -->
CM-SP-CM-OSSIv4.0-I13-260429                                         Data-Over-Cable Service Interface Specifications


                                               Element                             Size
                               Capture Time                               4 bytes
                               DS Channel Id                              1 byte
                               CM MAC Address                             6 bytes
                               Subcarrier zero frequency in Hz            4 bytes
                               FirstActiveSubcarrierIndex                 2 bytes
                               Subcarrier spacing in kHz                  1 byte
                               Length in bytes of coefficient data        4 bytes
                               Chan estimate coefficient data             ComplexData



D.2.6.12.1 File Header Element Definitions

D.2.6.12.1.1 Capture Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC),Thursday, 1 January 1970.

D.2.6.12.1.2 DS Channel Id
This element represents the channel Id of the downstream channel for which the Channel Estimate data apply.

D.2.6.12.1.3 CM MAC Address
The CM MAC Address.

D.2.6.12.1.4 Subcarrier Zero Frequency in Hz
This element is the center frequency of subcarrier zero of the OFDM channel.

D.2.6.12.1.5 Subcarrier Spacing in kHz
This element is the subcarrier spacing for the OFDM channel - 25 or 50 kHz.

D.2.6.12.1.6 FirstActiveSubcarrierIndex
This element is the subcarrier index of the lowest subcarrier in the Encompassed Spectrum of the channel.

D.2.6.12.1.7 Length in Bytes of Coefficient Data
This element indicates the size of the complexData which follows.

D.2.6.12.1.8 Chan Estimate Coefficient Data
This element refers to the complexData values of the CmOfdmChanEstimateCoef object. These data elements
typically represent the reciprocal of CM DS frequency-domain equalizer coefficients with one complex value for
each subcarrier within the Encompassed Spectrum. The data is expressed in s2.13 fixed point notation. The CM
should adjust the scaling such that the average power of the samples is approximately 1, in order to avoid excessive
clipping and quantization noise. If there are excluded subcarriers within the Encompassed Spectrum, the CM MUST
report a value of 0x8000 for both of the I and Q values for those excluded subcarriers. If the CM calculates 0x8000
for both of the I and Q values for an active subcarrier, then the CM MUST report a value of 0x8001 for both of the I
and Q values for that subcarrier.

D.2.6.12.1.9 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.




208                                                  CableLabs                                             04/29/26
