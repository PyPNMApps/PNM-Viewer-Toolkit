# Annex D.2.8 - CmDsOfdmRxMer

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 212-215
Classification: Normative (explicit)

<!-- PDF_PAGE: 212 -->
CM-SP-CM-OSSIv4.0-I13-260429                                    Data-Over-Cable Service Interface Specifications


D.2.7.7.1.11 Constellation Display Data
This element refers to the complexData values of the Constellation Display data. The data is expressed in s2.13
fixed point notation. The CM should adjust the scaling such that the average power of the QAM constellation (not
including pilots), assuming equally probable QAM symbols, is approximately 1, in order to avoid excessive clipping
and quantization noise.

D.2.8     CmDsOfdmRxMer
This object provides measurements of the receive modulation error ratio (RxMER) for each subcarrier.
Each subcarrier RxMER value consists of one byte, which represents the RxMER value with range 0 to 63.5 dB in
0.25 dB steps. If some subcarriers (such as exclusion bands) cannot be measured by the CM, the CM indicates that
condition by reporting a value of 0xFF for the RxMER for those subcarriers. Any measured RxMER value below 0
dB is clipped to 0 dB (0x00), and any measured value above 63.5 dB is clipped to 63.5 dB (0xFE).
The CM has the capability of generating a file containing all subcarrier RxMER values. The CM also reports four
summary metrics (Mean, StdDev, ThresholdRxMerValue and ThresholdRxMerHighestFreq) which can be used to
determine whether to request the file of all subcarrier RxMER values.
Mathematical notation for the calculation of RxMER is provided in the Downstream Receive Modulation Error
Ratio (RxMER) Per Subcarrier section of [PHYv4.0].
This object will have an instance for each ifIndex for the modem.
The operator may choose two methods of operation for this object. The first is obtaining detailed Rx MER statistics
on a per-subcarrier basis. To do this, the operator sets a filename for the data file in the FileName attribute and then
sets the FileEnable attribute to true. The Operator may also choose to perform an ad-hoc measurement of the
summary metrics for this channel. In this case, the operator performs a query of the object or performs individual
queries on one or more of the following attributes.
      •   Mean
      •   StdDev
      •   ThresholdRxMerValue
      •   ThresholdRxMerHighestFreq
When the CM receives a query request on the Mean, StdDev, ThresholdRxMerValue, or
ThresholdRxMerHighestFreq attributes, the CM MUST perform a measurement for the attribute being requested.
When the CM receives a query on the CmDsOfdmRxMer object, the CM MUST perform a measurement for all of
these attributes. The CM MAY update the Mean, StdDev, ThresholdRxMerValue, and ThresholdRxMerHighestFreq
attributes when a query is performed on any of these attributes.
                                   Table 99 - CmDsOfdmRxMer Object Attributes

          Attribute Name              Type         Access       Type Constraints            Units      Default Value
   IfIndex                      InterfaceIndex     Key
   FileEnable                   Boolean            R/W                                                False
   Percentile                   UnsignedByte       R/W                                 percent        2
   Mean                         HundredthsDb       R/O         0..65535                Db
   StdDev                       HundredthsDb       R/O         0..65535                hundredthsDb
   ThresholdRxMerValue          UnsignedByte       R/O                                 quarterDb
   ThresholdRxMerHighestFreq    UnsignedInt        R/O                                 Hz
   MeasStatus                   MeasStatusType     R/O
   FileName                     AdminString        R/W        SIZE (1..255)                           empty string




212                                                  CableLabs                                                04/29/26
<!-- PDF_PAGE: 213 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


           IfIndex
This attribute is the interface index of the downstream channel and is a key to provide an index into the table.

           FileEnable
This attribute causes the CM to begin the RxMer measurement for the purpose of creating a file of RxMer data.
When the measurement is complete the FileEnable attribute is set internally to false by the CM.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, this object returns
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
Setting this value to 'true' will change the value of the MeasStatus to 'busy' while the test is in progress.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
A default value for FileName is defined; thus, this object may be set to 'true' without explicitly setting the FileName
value.
Setting this attribute to a value of 'false' instructs the CM to stop the measurement.
This attribute returns 'true' if the CM is actively taking a measurement; otherwise it returns 'false'.

           Percentile
This attribute specifies the percentile (such as 2nd percentile or 5th percentile) of all active subcarriers in an OFDM
channel at which the ThresholdRxMerValue occurs. That is, (Percentile) % of the subcarriers have RxMER <=
ThresholdRxMerValue.

           Mean
This attribute is the mean of the dB values of the RxMER measurements of all active subcarriers. The mean is
computed directly on the dB values as follows:
    Mean = sum of (RxMER dB values) / number of RxMER values

           StdDev
This attribute is the standard deviation of the dB values of the RxMER measurements of all active subcarriers. The
standard deviation is computed directly on the dB values as follows:
    StdDev = sqrt(sum of (RxMER dB values - RxMER_mean)^2 / number of RxMER values)

           ThresholdRxMerValue
This attribute is the RxMER value corresponding to the specified Percentile value. The CM sorts the subcarriers in
ascending order of RxMER, resulting in a post-sorting subcarrier index ranging from 1 to the number of active
subcarriers. If the percentile value corresponds to a non-integer post-sorting subcarrier index, the post-sorting index
is truncated (floor function is applied); that is, the post-sorting index is selected which is the greatest integer less
than or equal to the corresponding percentile value. For example, if there are 3677 active subcarriers and the 2nd
percentile is specified, the CM computes floor(3677*0.02) = 73. That is, the RxMER value of the 73rd subcarrier in
the sorted list is associated with the 2nd percentile.

           ThresholdRxMerHighestFreq
This attribute is the frequency in Hz of the highest-frequency subcarrier having RxMER = ThresholdRxMerValue.

           Example Calculations of ThresholdRxMerValue and ThresholdRxMerHighestFreq
As a first example, assume there are 3800 active subcarriers in the OFDM channel being measured, and the
Percentile attribute is set to 2%. Using a sorting process, the CM finds the ThresholdRxMerValue corresponding to
the specified Percentile. Assume for this example that ThresholdRxMerValue = 25 dB, that is, 2% of the subcarriers



04/29/26                                              CableLabs                                                    213
<!-- PDF_PAGE: 214 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


have RxMER <= 25 dB. The CM also finds the highest frequency subcarrier having RxMER = 25 dB. Assume for
this example that this subcarrier has frequency 702 MHz. The CM reports ThresholdRxMerValue = 25 dB and
ThresholdRxMerHighestFreq = 702 MHz.
As a second more detailed example, consider a simplified case of an OFDM channel with 50 kHz subcarrier spacing
and only 32 active subcarriers; in reality the number of subcarriers in a channel will be in the hundreds or thousands.
To further simplify the example, 1 dB resolution is assumed for RxMER; in reality the RxMER values will have
0.25 dB resolution. Assume the sequence of subcarriers is sorted in ascending order of RxMER as in Table 100.
                     Table 100 - Example of RxMER Summary Statistics with 32 Subcarriers

                                    Subcarrier                            Subcarrier
                                      Index                               Frequency
                                  (Post Sorting)      RxMER (dB)            (MHz)
                                  1                  19                 501.10
                                  2                  20                 501.00
                                  3                  20                 501.05
                                  4                  20                 501.15
                                  5                  21                 501.20
                                  6                  21                 501.25
                                  7                  21                 501.30
                                  8                  22                 501.35
                                  9                  23                 500.95
                                  10                 23                 501.40
                                  11                 23                 501.55
                                  12                 24                 500.90
                                  13                 24                 501.45
                                  14                 25                 500.75
                                  15                 25                 500.80
                                  16                 25                 500.85
                                  17                 25                 501.50
                                  18                 28                 500.70
                                  19                 29                 500.65
                                  20                 30                 500.00
                                  21                 30                 500.50
                                  22                 30                 500.60
                                  23                 31                 500.45
                                  24                 31                 500.55
                                  25                 32                 500.05
                                  26                 33                 500.15
                                  27                 33                 500.40
                                  28                 34                 500.10
                                  29                 34                 500.35
                                  30                 35                 500.25
                                  31                 35                 500.30
                                  32                 36                 500.20


For this example, let the specified Percentile = 20; that is, to find the subcarrier corresponding to the 20th percentile
(in reality, typical values will be 2nd or 5th percentile). The CM computes floor(20% of 32 subcarriers) = 6,
meaning that the RxMER value of the 6th subcarrier in the sorted list corresponds to the specified Percentile; this
subcarrier has an RxMER value of 21 dB. To determine ThresholdRxMerHighestFreq, the CM finds the frequency


214                                                  CableLabs                                                04/29/26
<!-- PDF_PAGE: 215 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


of the highest-frequency subcarrier having RxMER = 21 dB. There are 3 subcarriers with RxMER = 21 dB, and of
these, the one with the highest frequency is the 7th subcarrier in the sorted list, having frequency 501.30 MHz. The
CM reports ThresholdRxMerValue = 21 dB and ThresholdRxMerHighestFreq = 501.30 MHz.

           MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the file is ready for transfer.

           FileName
This attribute is the name of the file at the CM which is to be downloaded by the PNM server.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.
If the value of this attribute is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, it is generated as the test name plus the CM MAC Address plus the 'epoch time'.
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC), Thursday, 1 January 1970.
Hence, the format would be:
    PNMDsMer_<CM MAC address>_<epoch>
For example: PNMDsMer_0010181A2D11_1403405123
The data file is composed of a header plus the Subcarrier RxMER Data. The header is composed of ordered fixed-
length fields. Unless otherwise specified, the header fields contain hex values that are right-justified within the field.
If necessary, the field is left-padded with zero values.
Syntax of the file is as follows:
                                               Table 101 - RxMER File Format

                                                    Element                      Size
                                    File type (value = 504E4E04)        4 bytes
                                    Major Version (Value = 1)           1 byte
                                    Minor Version (Value = 0)           1 byte
                                    Capture Time                        4 bytes
                                    DS Channel Id                       1 byte
                                    CM MAC Address                      6 bytes
                                    Subcarrier zero frequency in Hz     4 bytes
                                    FirstActiveSubcarrierIndex          2 bytes
                                    Subcarrier spacing in kHz           1 byte
                                    Length in bytes of RxMER data       4 bytes
                                    Subcarrier RxMER data               RxMerData



D.2.8.10.1 File Header Element Definitions

D.2.8.10.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.



04/29/26                                                 CableLabs                                                   215
