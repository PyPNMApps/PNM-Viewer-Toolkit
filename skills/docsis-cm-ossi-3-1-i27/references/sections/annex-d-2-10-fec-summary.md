# Annex D.2.10 - CmDsOfdmFecSummary

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 179-181
Classification: Normative (explicit)

<!-- PDF_PAGE: 179 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


              MeasuredAvgMer
This attribute is the arithmetic mean of all MER dB values measured over all of the subcarriers in the Modulated
Spectrum. That is, the average is taken of all the dB values.

              AverageMerMargin
This attribute represents the difference between the MeasuredAvgMer and the ReqAvgMer.

              MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the data is ready for evaluation.

              Example
The CM calculates the Required Average MER based on the bit loading for the profile and the Required MER per
Modulation Order provided in the CmDsOfdmRequiredQamMer table. The CM then measures the RxMER per
subcarrier and calculates the Average MER for the Active Subcarriers used in the Profile and stores the value as
MeasuredAvgMer.
Attributes used in this example:
    MeasuredAvgMer - average of RxMER per Subcarrier values in dB
    ReqAvgMer - assigned or derived from profile modulation orders and CmDsOfdmRequiredQamMer table
    ThreshldOffset - number subtracted from RequiredQamMer to calculate NumSubcarBelowThrshld
    AverageMerMargin - Result of subtracting ReqAvgMer from MeasuredAvgMer
    NumSubcarriersBelowThrshld - total number of subcarriers for which the RxMER Per Subcarrier is less than
    the RequiredQamMer value minus the ThreshldOffset
    Assumptions: Candidate Profile has 3800 Active Subcarriers in the Modulated Spectrum. The bit loading of
    1900 of the subcarriers is 4096-QAM with a Required MER of 41 dB. The remainder of the 1900 subcarriers
    are 2048-QAM with Required MER of 38 dB. The Required Average MER is calculated as (41 * 1900 + 38 *
    1900)/3800 = 39.5 dB. If the MeasuredAvgMer Per Subcarrier = 42 dB, then the AverageMerMargin is 42 –
    39.5 = 2.5 (250 hundredthsdB).
    Further, if 10 of the 4096-QAM and 10 of the 2048-QAM subcarriers have a Measured RxMER value equal to
    36 dB and the ThreshldOffset is set to 4 dB, then the CM will report a value of 10 for the
    NumSubcarriersBelowThrshld since the criteria for counting the NumSubcarriersBelowThrshld for 4096-QAM
    would be 41-4=37, while the criteria for counting the NumSubcarriersBelowThrshld for 2048-QAM would be
    37-4 = 33 dB.

D.2.10 CmDsOfdmFecSummary
The purpose of this item is to provide a series of codeword error rate measurements on a per profile basis over a set
period of time.
This table will have a row for each ifIndex for the modem.
                              Table 70 - CmDsOfdmFecSummary Object Attributes

       Attribute Name              Type          Access        Type Constraints         Units      Default Value
    IfIndex                 InterfaceIndex     Key
    SummaryType             Enum               R/W           other(1),                             interval10min(2)
                                                             interval10min(2),
                                                             interval24hr(3)
    FileEnable              Boolean            R/W                                                 false
    MeasStatus              MeasStatusType     R/O
    FileName                AdminString        R/W           SIZE (1..255)                         empty string




02/19/25                                             CableLabs®                                                       179
<!-- PDF_PAGE: 180 -->
CM-SP-CM-OSSIv3.1-I27-250219                                      Data-Over-Cable Service Interface Specifications


           IfIndex
This attribute is the interface index of the downstream interface and is a key to provide an index into the table.

           SummaryType
This attribute is the type of summary test to be performed. If set to interval10min(2), when enabled, the CM MUST
return the codeword data samples recorded once per second over the most recent 10-minute period for a maximum
of 600 samples. If set to interval24hr(3), when enabled, the CM MUST return the codeword data samples recorded
once per minute over the most recent 24-hour period for a maximum of 1440 samples. The CM begins collecting
samples in the background when it becomes operational. The CM returns the collected data when the test is enabled.
If the CM has not been in the operational state for the entire SummaryType period, it will not have collected the full
data set for the SummaryType period. In this case, the CM will return the data that it has available since becoming
operational, which will be less than the maximum size permitted for the SummaryType period.
Data samples are cleared on CM reset or power cycle. The CM MUST maintain the FEC Summary Data through a
Re-Init MAC. When the CM is maintaining FEC Summary data across a Re-Init MAC operation, there will be a gap
in the CodewordSetTimeStamp values and the total time for interval10min and interval24hr, SummaryTypes could
be greater than 10 minutes or 24 hours, respectively. It is possible that the DS Channel associated with the ifIndex
for which FEC Summary data is being collected could change due to a Re-Init MAC event. If this unlikely event
were to occur, the CM will report the DS Channel Id currently associated with the ifIndex, in the header of the file
being sent to the PNM server.
References: [PHYv3.1] Downstream FEC Statistics section

           FileEnable
When this attribute is set to 'true', the CM MUST begin the SummaryType codeword error summary test. While the
test is in progress, the CM MUST return a MeasStatus value of 'busy'. When the measurement is complete, the CM
MUST set the FileEnable attribute to 'false'. If this attribute is set to 'false' during a test, the CM MUST stop the test.
This attribute is subject to the rules specified by the PnmCmCtlStatus attribute. Therefore, the CM MUST return
'inconsistentValue' if set to 'true' while the value of PnmCmCtlStatus is any value other than 'ready'.
This attribute returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the
CM is in battery-backup mode.
A default value for FileName is defined; thus, this object can be set to 'true' without explicitly setting the FileName
value.
The CM MUST return the value of 'true' if it is actively taking a measurement; otherwise it MUST return 'false'.

           MeasStatus
This attribute is used to determine the status of the measurement. The PNM server will query this value to determine
when the data is ready for evaluation.

           FileName
This attribute is the name of the file at the CM which is to be transferred to the PNM server. This value can only be
changed while a test is not in progress. The CM MUST return 'inconsistentValue' when it receives an attempt to set
this value while the value of MeasStatus is 'busy'.
If the value of this object is the default value (empty string), then a default filename value will be used. Otherwise,
the value set will be used as the filename. If a default filename is generated, then that value will be returned in this
object and will represent the filename that was used for the test. All subsequent tests should set this object to a
meaningful value or to an 'empty string' value (to generate a new default filename) before starting a new test.
If a default filename value is used, the CM MUST store the test results in a file with a name consisting of the test
name plus the CM MAC Address plus the 'epoch time'. The epoch time (also known as 'unix time') is defined as the
number of seconds that have elapsed since midnight Coordinated Universal Time (UTC), Thursday, 1 January 1970.




180                                                   CableLabs®                                                 02/19/25
<!-- PDF_PAGE: 181 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


Hence, the format would be:
    PNMFecSum_<CM MAC address>_<epoch>
    For example: PNMFecSum_0010181A2D11_1403405123
The data file is composed of a header plus the FEC Data. The header is composed of ordered fixed-length fields.
Unless otherwise specified, the header fields contain hex values that are right-justified within the field. If necessary,
the field is left-padded with zero values.
Syntax of the file is as follows:
                                    Table 71 - Downstream FEC Summary File Format

                                              Element                      Size or Data Type
                         File type (value = 504E4E08)                    4 bytes
                         Major Version (Value = 1)                       1 byte
                         Minor Version (Value = 0)                       1 byte
                         DS Channel Id                                   1 byte
                         CM MAC Address                                  6 bytes
                         SummaryType                                     1 byte
                         Number of Profiles                              1 byte
                         FEC Data                                        FecSummaryData



D.2.10.5.1 File Header Element Definitions

D.2.10.5.1.1 Major Version
The current file header version assigned the "value". The version is incremented by one when the file header format
is modified by specification. The specification defined version value is 1.

D.2.10.5.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.10.5.1.3 DS Channel Id
This element represents the channel Id of the downstream channel for which these FEC statistics apply.

D.2.10.5.1.4 CM MAC Address
The CM MAC Address.

D.2.10.5.1.5 SummaryType
This element is a copy of the Summary Type attribute for the MIB.

D.2.10.5.1.6 NumberOfProfiles
This element is number of profiles in the CM for which Codeword summaries are provided. This number indicates
how many instances of the FecSummaryData type are included in the file.
The CM MUST include FEC data arrays for every profile assigned by the CCAP.

D.2.10.5.1.7 FEC Data
The set of FEC data encoded using the FecSummaryData type. There is one set of data per Profile ID as indicated by
the NumberOfProfiles.
Example:



02/19/25                                                CableLabs®                                                   181
