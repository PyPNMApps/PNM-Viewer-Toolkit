# Annex D.2.5 - CmSymbolCapture

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 162-165
Classification: Normative (explicit)

<!-- PDF_PAGE: 162 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


D.2.4.1.13.12 Length (in Bytes) of SpectrumAnalysis Data
This element indicates the size of the data which follows.

            CmSpectrumAnalysisMeas
This object provides a list of the spectral amplitude measurements taken across the requested range of center
frequencies. The table represents a full scan of the spectrum with each row corresponding to a spectral capture of
one segment of the spectrum.
                              Table 59 - CmSpectrumAnalysisMeas Object Attributes

      Attribute Name               Type             Access       Type Constraints             Units       Default
      Frequency           UnsignedInt               key                                  Hz
      AmplitudeData       BinAmplitudeData          R/O
      TotalSegmentPower   TenthdBmV                 R/O                                  dBmV



D.2.4.2.1 Frequency
This key indicates the center frequency of the spectral analysis segment which is represented by this instance.

D.2.4.2.2 AmplitudeData
This attribute provides a list of the spectral amplitudes as measured at the center frequency specified by the
Frequency attribute.
The frequency bins are ordered from lowest to highest frequencies covering the frequency span. Information about
the center frequency, frequency span, number of bins and resolution bandwidth are included to provide context to
the measurement point.
Bin Amplitudes are reported in units of 0.01dB.

D.2.4.2.3 TotalSegmentPower
This attribute provides the total RF power present in the segment with the center frequency equal to the Frequency
index and the span equal to the SegmentFrequencySpan. The value represents the sum of the spectrum power in all
of the associated bins. The value is computed by summing power (not dB) values and converting the final sum to
TenthdBmV.

D.2.5     CmSymbolCapture
The purpose of downstream symbol capture is to provide partial functionality of a network analyzer to analyze the
response of the cable plant from the CM's perspective.
At the CM, the received I and Q time-domain samples of one full OFDM symbol before the FFT, not including the
guard interval, are captured and made available for analysis. This capture will result in a number of data points equal
to the FFT length in use, time aligned for receiver FFT processing. The number of captured samples can be reduced
for narrower channels if the sampling rate, which is implementation dependent, is reduced. The capture includes a
bit indicating if receiver windowing effects are present in the data. The time domain samples are expressed as 16-bit
two's complement numbers using s3.12 format. The CM samples are scaled such that the average power of the
samples is approximately 1, in order to avoid excessive clipping and quantization noise.
Capturing the input and output of the cable plant is equivalent to a wideband sweep of the channel, which permits
full characterization of the linear and nonlinear response of the downstream plant. The MAC provides signaling via
the PLC Trigger Message to ensure that the same symbol is captured at the CMTS and CM.
The Downstream Symbol Capture is controlled by setting the 'TriggerEnable' attribute. The status of the capture is
obtained by reading the value of the 'MeasStatus' attribute.
This table will have a row for each ifIndex for the modem.



162                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 163 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


                                   Table 60 - CmSymbolCapture Object Attributes

        Attribute Name               Type           Access           Type Constraints              Units       Default
                                                                                                                Value
    ifIndex                    InterfaceIndex      Key
    TriggerEnable              Boolean             R/W                                                     False
    TriggerEnableTimeout       UnsignedInt         R/W         0..3600                           seconds   600
    TriggerGroupId             UnsignedShort       R/W                                                     0
    RxWindowing                Boolean             R/O
    TransactionId              UnsignedByte        R/O
    SampleRate                 UnsignedInt         R/O                                           Hz
    FftLength                  UnsignedInt         R/O         512 | 1024 | 2048 | 4096 | 8192
    MeasStatus                 MeasStatusType      R/O
    FileName                   AdminString         R/W         SIZE (1..255)                               empty string



              ifIndex
This attribute is the interface index of the OFDM downstream channel and is a key to provide an index into the table.

              TriggerEnable
If this attribute is set to a value of 'true', the CM MUST begin looking for the presence of the Trigger Message Block
in the PLC with a Group ID matching the CM's TriggerGroupId. The TriggerEnable is a one-shot enable and the
attribute is internally disabled when a PLC containing a Group ID matching the CM's TriggerGroupId in a Trigger
Message Block is received.
If this attribute is set to a value of 'false', the CM MUST stop looking for the presence of the Trigger Message Block
in the PLC with a Group ID matching the CM's TriggerGroupId. In this case, the value of the MeasStatus attribute
will be set to 'inactive'.
When read, the CM MUST return a value of 'true' if the CM is actively looking for the presence of the Trigger
Message Block in the PLC with a Group ID matching the CM's TriggerGroupId. Otherwise, the CM MUST return
'false'.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, the CM MUST return
'inconsistentValue' if this attribute is set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
Default values for FileName, TriggerEnableTimeout, and TriggerGroupId attributes are defined; thus, this attribute
may be set to 'true' without explicitly setting these values. Care should be taken to ensure these values are correct for
the desired test case.
Setting this attribute to a value of 'true' will change the value of the MeasStatus attribute to 'busy'.

              TriggerEnableTimeout
This attribute is used to disable the TriggerEnable if no PLC containing a Group ID matching the CM's
TriggerGroupId in a Trigger Message Block is received within the timeout period. The CM MUST timeout the test
after TriggerEnableTimeout seconds from the time when the TriggerEnable was set to 'true'. A value of 0 indicates
that no timeout is enforced and the CM MUST enable the test until it completes, which in this case means the trigger
could be enabled indefinitely. This value can only be changed while a test is not in progress. An attempt to set this
value while the value of 'MeasStatus' is 'busy' will return 'inconsistentValue'.




02/19/25                                              CableLabs®                                                         163
<!-- PDF_PAGE: 164 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


           TriggerGroupId
This attribute is used to associate a CM with a group of CMs expected to perform Symbol Capture measurements for
the designated symbol.
This value can only be changed while a test is not in progress. If the CM receives an attempt to set this value while
the value of MeasStatus is 'busy', it MUST return 'inconsistentValue' for this attribute.

           RxWindowing
This attribute is a flag indicating if vendor proprietary receiver windowing was enabled during the capture.

           TransactionId
This attribute is the Transaction ID sent by the CMTS in the Trigger Message Block. The CMTS increments this
field by one on each trigger message that is sent, rolling over at value 255. Prior to completion of a measurement
this attribute has no meaning.

           SampleRate
This attribute is the FFT sample rate in use by the CM for the channel. Typically the sample rate for the downstream
channel will be 204.8 MHz.

           FftLength
This attribute is the FFT length in use by the CM for the channel. Typically this value is 4096 or 8192 for the
Downstream Channel.

           MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.

           Filename
This attribute is the name of the file, at the CM and containing captured symbol data, which is to be downloaded by
the PNM server.
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
PNMSymCap_<CM MAC address>_<epoch>

For example: PNMSymCap_0010181A2D11_1403405123
The data file is composed of a header plus the Symbol Capture Data. The header is composed of ordered fixed-
length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field.
If necessary, the field is left-padded with zero values.
Syntax of the file is as follows:




164                                                  CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 165 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


                                    Table 61 - CM Symbol Capture File Format

                                               Element                        Size
                                File type (value = 504E4E01)         4 bytes
                                Major Version (Value = 1)            1 byte
                                Minor Version (Value = 0)            1 byte
                                Capture Time                         4 bytes
                                DS Channel Id                        1 byte
                                CM MAC                               6 bytes
                                Subcarrier zero frequency in Hz      4 bytes
                                SampleRate in Hz                     4 bytes
                                FFT Size                             4 bytes
                                TriggerGroupId                       2 bytes
                                Transaction ID                       1 byte
                                Length (in bytes) of Capture Data    4 bytes
                                Capture Data                         ComplexData


D.2.5.10.1 File Header Element Definitions

D.2.5.10.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.5.10.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.5.10.1.3 Capture Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC),Thursday, 1 January 1970.

D.2.5.10.1.4 DS Channel Id
This element represents the channel Id of the downstream channel for which the symbol capture data apply.

D.2.5.10.1.5 CM MAC Address
The CM MAC address.

D.2.5.10.1.6 Subcarrier Zero Frequency in Hz
The center frequency of subcarrier zero of the OFDM channel.

D.2.5.10.1.7 SampleRate in Hz
This element is a copy of the SampleRate attribute.

D.2.5.10.1.8 FFT Size
This element is a copy of the FftLength attribute.

D.2.5.10.1.9 TriggerGroup ID
This element is a copy of the TriggerGroupId attribute.




02/19/25                                              CableLabs®                                                  165
