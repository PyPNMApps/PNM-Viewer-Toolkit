# Annex D.2.14 - CmDsOfdmModulationProfile

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 192-194
Classification: Normative (explicit)

<!-- PDF_PAGE: 192 -->
CM-SP-CM-OSSIv3.1-I27-250219                                    Data-Over-Cable Service Interface Specifications


D.2.13.14.1 File Header Element Definitions

D.2.13.14.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.13.14.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.13.14.1.3 Collection Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.

D.2.13.14.1.4 Upstream Channel Id
This element represents the Upstream Channel Id for which the Upstream PreEqualization Coefficients apply.

D.2.13.14.1.5 CM MAC Address
The CM MAC Address.

D.2.13.14.1.6 CMTS MAC Address
This element contains the MAC Address of the downstream channel for which the CM is receiving MAPs and
UCDs.

D.2.13.14.1.7 Subcarrier Zero Frequency in Hz
This element is the center frequency of subcarrier zero of the OFDM channel.

D.2.13.14.1.8 FirstActiveSubcarrierIndex
This element is the subcarrier index of the lowest subcarrier in the Encompassed Spectrum of the channel.

D.2.13.14.1.9 Subcarrier Spacing in kHz
This element is the OFDM subcarrier spacing.

D.2.13.14.1.10 Length in Bytes of PreEq Data
This element is the number of bytes of the PreEqData.

D.2.13.14.1.11 Pre-Eq Coefficient Data
This element refers to the complexData values of the CM Pre-Equalization Coefficients. The data is expressed in
s2.13 Fixed Point notation.
Note:    The power averaged over all coefficients is normalized to 1 [PHYv3.1].

D.2.14 CmDsOfdmModulationProfile
The purpose of this object is to provide the per subcarrier bit loading of the modulation profiles.
This table will have a row for each downstream interface ifIndex for the modem.




192                                                 CableLabs®                                              02/19/25
<!-- PDF_PAGE: 193 -->
Cable Modem Operations Support System Interface Specification                     CM-SP-CM-OSSIv3.1-I27-250219


                            Table 79 - CmDsOfdmModulation Profile Object Attributes

       Attribute Name              Type           Access         Type Constraints          Units       Default Value
    IfIndex                  InterfaceIndex      Key
    FileEnable               Boolean             R/W                                                  false
    MeasStatus               MeasStatusType      R/O
    FileName                 AdminString         R/W           SIZE (0..255)                          empty string



              IfIndex
This attribute is the index of the downstream interface and is a key to provide an index into the table.

              FileEnable
When the FileEnable attribute is set to 'true', the CM MUST compile all the modulation profiles that are assigned by
the CMTS. While the compilation is in progress, the CM MUST return a MeasStatus value of 'busy'. When the
measurement is complete, the CM MUST set the FileEnable attribute to 'false'. If the FileEnable attribute is set to
'false' during a compilation, the CM MUST stop the compilation.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
A default value for FileName is defined; thus, this object can be set to 'true' without explicitly setting the FileName
value.
The CM MUST return the FileEnable value of 'true' if it is actively compiling the modulation profile data. The CM
MUST return the FileEnable value of 'false' if it is not actively compiling the modulation profile data.

              MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the data is ready for evaluation.

              FileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. This value can only be
changed while a test is not in progress. The CM MUST return 'inconsistentValue' when it receives an attempt to set
the FileName value while the value of MeasStatus is 'busy'.
If the value of this object is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default FileName value is used, the CM MUST store the test results in a file with a name consisting of the test
name plus the CM MAC Address plus the 'epoch time'. The epoch time (also known as 'unix time') is defined as the
number of seconds that have elapsed since midnight Coordinated Universal Time (UTC), Thursday, 1 January 1970.
Hence, the format would be:
    PNMModProfile_<CM MAC address>_<epoch>
    For example: PNMModProfile_0010181A2D11_1403405123
The data file is composed of a header plus the Modulation Profile Data. The header is composed of ordered fixed-
length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field.
If necessary, the field is left-padded with zero values.




02/19/25                                               CableLabs®                                                     193
<!-- PDF_PAGE: 194 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


                           Table 80 - Downstream OFDM Modulation Profile File Data

                                             Element                       Size or Data Type
                        File type (value = 504E4E0A)                     4 bytes
                        Major Version (Value = 1)                        1 byte
                        Minor Version (Value = 0)                        1 byte
                        Capture Time                                     4 bytes
                        DS Channel Id                                    1 byte
                        CM MAC Address                                   6 bytes
                        Number of Profiles                               1 byte
                        Subcarrier zero frequency in Hz                  4 bytes
                        FirstActiveSubcarrierIndex                       2 bytes
                        Subcarrier spacing in kHz                        1 byte
                        Length in bytes of the Modulation Profile Data   4 bytes
                        Modulation Profile Data                          ModulationProfileData



D.2.14.4.1 File Header Element Definitions

D.2.14.4.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.14.4.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.14.4.1.3 Capture Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.

D.2.14.4.1.4 DS Channel Id
This element represents the channel Id of the downstream channel for which the modulation profile data apply.

D.2.14.4.1.5 CM MAC Address
The CM MAC Address.

D.2.14.4.1.6 Number of Profiles
This element is number of profiles in the CM that indicates how many instances of the Modulation Profile Data type
are included in the file.
The CM MUST count every Modulation Profile assigned by the CCAP, not including NCP.

D.2.14.4.1.7 Subcarrier Zero Frequency in Hz
This element is the center frequency of subcarrier zero of the OFDM channel.

D.2.14.4.1.8 FirstActiveSubcarrierIndex
This element is the subcarrier index of the lowest active subcarrier in the Occupied Bandwidth of the channel.




194                                                    CableLabs®                                          02/19/25
