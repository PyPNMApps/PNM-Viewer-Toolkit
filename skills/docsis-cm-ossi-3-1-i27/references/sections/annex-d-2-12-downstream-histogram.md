# Annex D.2.12 - CmDsHist

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 183-186
Classification: Normative (explicit)

<!-- PDF_PAGE: 183 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


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

D.2.12 CmDsHist
The purpose of the downstream histogram is to provide a measurement of nonlinear effects in the channel such as
amplifier compression and laser clipping. For example, laser clipping causes one tail of the histogram to be
truncated and replaced with a spike. The CM captures the histogram of time domain samples at the wideband front
end of the receiver (full downstream band). The histogram is two-sided; that is, it encompasses values from far-
negative to far-positive values of the samples. The histogram will have 255 or 256 equally spaced bins. These bins
typically correspond to the 8 MSBs of the wideband analog-to-digital converter (ADC) for the case of 255 or 256
bins. The histogram dwell count, a 32-bit unsigned integer, is the number of samples observed while counting hits
for a given bin, and may have the same value for all bins. The histogram hit count, a 32-bit unsigned integer, is the
number of samples falling in a given bin. The CM reports the dwell count per bin and the hit count per bin. When
enabled, the CM computes a histogram with a dwell of at least 10 million samples at each bin in 30 seconds or less.
The CM continues accumulating histogram samples until it is restarted, disabled or times out. If the highest dwell
count approaches its 32-bit overflow value, the CM stops counting and sets the MeasStatus attribute to




02/19/25                                            CableLabs®                                                    183
<!-- PDF_PAGE: 184 -->
CM-SP-CM-OSSIv3.1-I27-250219                                         Data-Over-Cable Service Interface Specifications


'sampleReady'. The CM reports the capture time of the histogram measurement in the Header Elements of the file
for TFTP download using the 32-bit Universal Time Code.
This table will have a row for each docsCableMaclayer type ifIndex for the modem.
                                          Table 73 - CmDsHist Object Attributes

        Attribute Name                Type           Access      Type Constraints            Units          Default Value
      IfIndex                   InterfaceIndex       Key
      Enable                    Boolean              R/W                                                   false
      Timeout                   UnsignedShort        R/W                                 Seconds           1800
      MeasStatus                MeasStatusType       R/O
      FileName                  AdminString          R/O         SIZE (0..255)                             empty string


            IfIndex
This attribute is the interface index of the RF MAC Interface and is a key to provide an index into the table.

            Enable
Setting this attribute to a value of 'true' instructs the CM to begin collection of histogram data and when enabled, the
CM continues producing new data at its own rate.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, this object returns
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
Setting this object to a value of 'false' instructs the CM to stop the collection of histogram data.
This object returns 'true' if the CM is actively collecting histogram data. Otherwise, it returns 'false'.
This object returns 'inconsistentValue' if set to 'true' while the value of MeasStatus is a value of 'busy' for this row or
for any row in the table. That is, only one row in the table is allowed to be 'true' at the same time.
This object returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the CM
is in battery-backup mode.
Setting this object to a value of 'true' will change the value of the MeasStatus to 'busy'.
A restart may be accomplished by setting this attribute to 'false' and then back to 'true'.

            Timeout
This attribute sets a seconds time-out timer for capturing histogram data. This attribute is used to automatically clear
the Enable attribute when the timeout expires. If the value of this attribute is zero, the CM will collect data until the
value is changed, stopped, or until any dwell counter approaches its 32-bit rollover value. If the Timeout attribute is
re-written while Enable is 'true', the CM will restart the timeout timer with the new Timeout value and continue
collecting data. When the timeout expires, the Enable attribute will be set to 'false' and the capture will stop. When
this happens, the data collected up to this point will be saved in the file defined by the DataFileName and the value
of MeasStatus set to 'sampleReady'.
Setting this value does not start a capture. Captures can only be started by setting the Enable attribute.
If this attribute is written while the Enable object is 'true', the timer is restarted. If this attribute is set to a value of
'zero', there is no timeout and the collection of data will continue indefinitely.
This attribute returns the value with which it was last set.

            MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.




184                                                      CableLabs®                                                       02/19/25
<!-- PDF_PAGE: 185 -->
Cable Modem Operations Support System Interface Specification                        CM-SP-CM-OSSIv3.1-I27-250219


           FileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. The data is stored as
32-bit integers for the hit and dwell count values.
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
         PNMHist_<CM MAC address>_<epoch>
         For example: PNMHist_0010181A2D11_1403405123
The data file is composed of a header plus the Histogram Data. The header is composed of ordered fixed-length
fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field. If
necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                    Table 74 - Downstream Histogram File Format

                                                  Element                          Size
                                File type (value = 504E4E05)              4 bytes
                                Major Version (Value = 1)                 1 byte
                                Minor Version (Value = 0)                 1 byte
                                Capture Time                              4 bytes
                                CM MAC Address                            6 bytes
                                Symmetry                                  1 byte
                                Length (in bytes) of Dwell Count Values   4 bytes
                                DwellCount values                         (1-4096) * 4 bytes
                                Length (in bytes) of Hit Count Values     4 bytes
                                HitCount values                           (1-4096) * 4 bytes



D.2.12.5.1 File Header Element Definitions

D.2.12.5.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.12.5.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.12.5.1.3 Capture Time
The time (epoch time) that the file was written. The epoch time (also known as 'unix time') is defined as the number
of seconds that have elapsed since midnight Coordinated Universal Time (UTC), Thursday, 1 January 1970.




02/19/25                                               CableLabs®                                                        185
<!-- PDF_PAGE: 186 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


D.2.12.5.1.4 CM MAC Address
The CM MAC Address.

D.2.12.5.1.5 Symmetry
This attribute is used to indicate whether 256 or 255 bins were used for the measurement.
Even Symmetry = 'false' (default): The histogram has even symmetry about the origin. There is no bin center lying
directly at the origin; rather, two bin centers straddle the origin at 0.5. All bins with indices 0-255 contain valid hit-
count data. The histogram bin centers are offset from the corresponding 8-bit two's complement integer values by
1/2, that is, bin center = two's complement value + 0.5.
Odd Symmetry = 'true': The histogram has odd symmetry about the origin. There is a bin center lying at the origin.
The bin with index 0 is not used and returns the value 0. The bins with indices 1 to 255 contain valid hit-count data.
The histogram bin centers are located on the corresponding 8-bit two's complement integer values.
The following table shows the defined histogram bin centers for the cases of even and odd symmetry.
                                           Table 75 - Histogram Bin Centers

                                 Bin Index           Bin Center Even      Bin Center Odd
                                                        Symmetry            Symmetry
                                0              -127.5                    not used
                                1              -126.5                    -127
                                2              -125.5                    -126
                                ...            ...                       ...
                                127            -0.5                      -1
                                128            0.5                       0
                                129            1.5                       1
                                ...            ...                       ...
                                253            125.5                     125
                                254            126.5                     126
                                255            127.5                     127



D.2.12.5.1.6 Length in Bytes of Dwell Count Values
This element represents the number of Dwell Count Values which follow in the file.

D.2.12.5.1.7 Dwell Count Values
This element represents the Dwell Counts for each bin for the "Current" capture. The value is a sequence of 4-byte
values. If the dwell count for all bins is the same, then only a single value is reported. The value for each bin is
reported as a 32-bit value.

D.2.12.5.1.8 Length in Bytes of Hit Count Values
This element represents the number of Hit Count Values which follow in the file.

D.2.12.5.1.9 Hit Count Values
This element represents the Hit Counts for each bin for the "Current" capture. The value represents a sequence of
4-byte values. If odd symmetry is used, then there will be 255 bins. The value for each bin is reported as a 32-bit
value.




186                                                     CableLabs®                                               02/19/25
