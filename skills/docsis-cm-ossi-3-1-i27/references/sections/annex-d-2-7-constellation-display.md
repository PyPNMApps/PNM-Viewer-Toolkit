# Annex D.2.7 - CmDsConstDispMeas

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 169-171
Classification: Normative (explicit)

<!-- PDF_PAGE: 169 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


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

D.2.6.12.1.10 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.7    CmDsConstDispMeas
The downstream constellation display provides received QAM constellation points for display. Equalized soft
decisions (I and Q) at the slicer input are collected over time, possibly subsampling to reduce complexity, and made
available for analysis. This measurement is intended for data subcarriers only. Up to 8192 samples are provided for
each query; additional queries can be made to further fill in the plot.
The complex Constellation Display values are expressed as 16-bit two's complement numbers using s2.13 format.
The CM samples are scaled such that the average power of the QAM constellation is approximately 1, in order to
avoid excessive clipping and quantization noise.
The object controls the CM capturing and reporting received soft-decision samples, for a single selected
constellation from the set of profiles it is receiving, within a single OFDM downstream channel.
This table will have a row for each ifIndex for the modem.




02/19/25                                           CableLabs®                                                   169
<!-- PDF_PAGE: 170 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


                                  Table 64 - CmDsConstDispMeas Object Attributes

           Attribute Name                   Type            Access        Type               Units        Default Value
                                                                        Constraints
      IfIndex                     InterfaceIndex            Key
      TriggerEnable               Boolean                   R/W                                          False
      ModulationOrderOffset       UnsignedByte              R/W         0..10                            0
      NumSampleSymbols            UnsignedShort             R/W                                          8192
      SelectedModulationOrder     DsOfdmModulationType      R/O
      MeasStatus                  MeasStatusType            R/O
      FileName                    AdminString               R/W         SIZE (1..255)                    empty string



                IfIndex
This attribute is the interface index of the downstream channel and is a key to provide an index into the table.

                TriggerEnable
Setting this attribute to a value of 'true' instructs the CM to begin collection and storing the constellation points into
the file specified by the FileName attribute.
Setting this attribute to a value of 'false' instructs the CM to stop storing constellation points in the file.
When read, this attribute returns 'true' if the CM is actively storing constellation points in the file. Otherwise, it
returns 'false'.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore this object returns
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
A default value for FileName is defined; thus, this attribute may be set to 'true' without explicitly setting the
FileName value.
Setting this attribute to a value of 'true' will change the value of the MeasStatus to 'busy'.

                ModulationOrderOffset
This attribute specifies an offset from the lowest order modulation for the data subcarriers in any of the profiles in
the downstream channel. If the lowest order modulation order that the CM was receiving was 1024-QAM and the
ModulationOrderOffset was zero, then the CM would capture the soft decision samples for all of the subcarriers
which were using 1024-QAM modulation order. If the ModulationOrderOffset was 1, then the CM would capture
the soft decision samples for all of the subcarriers using the next highest modulation order in use for the profiles in
the downstream channel.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.

                NumSampleSymbols
This attribute tells the CM how many soft decision samples of OFDM subcarriers with the specified modulation
order are captured.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.




170                                                    CableLabs®                                                  02/19/25
<!-- PDF_PAGE: 171 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


           SelectedModulationOrder
This read-only attribute provides the actual Modulation Order that will be used for the Constellation display based
on the selected ModulationOrderOffset.

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
Coordinated Universal Time (UTC),Thursday, 1 January 1970.
Hence, the format would be:
    PNMDSConDisp_<CM MAC address>_<epoch>
    For example: PNMDSConDisp_0010181A2D11_1403405123
The data file is composed of a header plus the Constellation Display Data. The header is composed of ordered fixed-
length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field.
If necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                       Table 65 - Constellation Display File Format

                                                   Element                       Size
                                    File type (value = 504E4E03)        4 bytes
                                    Major Version (Value = 1)           1 byte
                                    Minor Version (Value = 0)           1 byte
                                    Capture Time                        4 bytes
                                    DS Channel Id                       1 byte
                                    CM MAC Address                      6 bytes
                                    Subcarrier zero frequency in Hz     4 bytes
                                    Actual modulation order*            2 bytes
                                    Number of sample symbols            2 bytes
                                    Subcarrier spacing in kHz           1 byte
                                    Length in bytes of display data     4 bytes
                                    Constellation display data          Complex Data

                                *DsOfdmModulationType




02/19/25                                                  CableLabs®                                                  171
