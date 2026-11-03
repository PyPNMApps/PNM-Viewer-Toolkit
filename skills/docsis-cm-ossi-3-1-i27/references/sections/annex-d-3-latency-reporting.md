# Annex D.3 - Latency Reporting

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 195-200
Classification: Normative (explicit)

<!-- PDF_PAGE: 195 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


D.2.14.4.1.9 Subcarrier Spacing in kHz
This element is the OFDM subcarrier spacing.

D.2.14.4.1.10 Length in Bytes of Modulation Profile Data
This element is the number of bytes of the Modulation Profile Data.

D.2.14.4.1.11 Modulation Profile Data
This element represents the Modulation Profile Data defined in Section D.2.1.7.

D.3 Latency Reporting
This section defines the object models supporting Latency Reporting. CCAP and cable modem features and
capabilities can be leveraged to enable measurement and reporting of latency estimates through each of the
downstream service flows. With this information, operations personnel can monitor latency trends and adjust
network configurations as appropriate.
This section defines the Latency statistics that provide histogram counts, maximum latencies, etc., for each enabled
service flow. Statistics files can be enabled for TFTP /bulk file upload via the bulk data transfer mechanisms defined
in Section D.5.

D.3.1    CmLatencyRpt
This object provides configuration of the CM upstream latency reporting. This enables the creation of latency report
files, which consist of a series of latency measurements on a per Service Flow basis over a provisioned period of
time.




                                   Figure 9 - Latency Report Information Model


The configuration attributes are described below.




02/19/25                                            CableLabs®                                                    195
<!-- PDF_PAGE: 196 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


                                     Table 81 - CmLatencyRpt Object Attributes

            Attribute Name                     Type          Access        Type Constraints         Units        Default
                                                                                                                  Value
 IfIndex                              InterfaceIndex        Key         MAC Domain
 SnapshotDuration                     UnsignedInt           R/W         1..3600                 Seconds     60
 NumSnapshots                         UnsignedShort         R/W         1..2000                             10
 NumFiles                             UnsignedByte          R/W         0..255                  -           0
 MeasStatus                           MeasStatusType        R/O
 Filename                             String                R/W         SIZE (0..231)                       empty string



            IfIndex
This attribute represents the interface index of the MAC Domain.

            SnapshotDuration
This attribute represents the measurement duration of service flow latency estimates. One row of statistics per
service flow is captured during this interval.

            NumSnapshots
This attribute represents the number of Snapshots batched into a file. The total duration represented by the file is the
product of the SnapshotDuration and NumSnapshots. Two examples follow.
      •    If SnapshotDuration is set to 1 second, and NumSnapshots is set to 10, when enabled, the CM returns the
           latency data samples recorded once per second over the next 10-sec period.
      •    If SnapshotDuration is set to 300 seconds (5 mins), and NumSnapshots is set to 288, when enabled, the CM
           returns the latency data samples recorded once per 5-minute interval over the next 24-hour period.
Certain combinations of SnapshotDuration and NumSnapshots could result in an excessive file generation rate or
excessive file size. The CM MAY reject CmLatencyRpt attribute settings that cause an excessive processing load or
resource requirement.

            NumFiles
This attribute represents the number of file uploads performed as defined in Table 82 below.
                                               Table 82 - NumFiles Definition

                                                      NumFiles Attribute
                           0               File generation disabled
                           1..254          Number of files to generate. Counts down from this
                                           value to zero
                           255             Unlimited file generation


The CM MUST start generating a latency report file when this value is set to a non-zero value. The CM MUST
close the current latency report file and decrement NumFiles by 1 when the SnapshotDuration * NumSnapshots
period is complete. The CM MUST close the current latency report file and discontinue file generation if the
NumFiles attribute is set to zero or decrements to zero.

            MeasStatus
This attribute is used to determine the status of the measurement.




196                                                     CableLabs®                                                02/19/25
<!-- PDF_PAGE: 197 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


           Filename
This attribute is the prefix for the name of the file that the CM creates to store latency histogram snapshots. This
value can only be changed while a test is not in progress. The CM MUST return 'inconsistentValue' when it receives
an attempt to set this value while the value of MeasStatus is 'busy'.
If the value of this object is the default value (empty string), then a default filename 'UsLatencySum' will be used.
Otherwise, the value set will be used as the filename prefix as described below.
The CM MUST store the test results in a file with a name consisting of the Filename plus the CM MAC Address
plus the 'epoch time', using the format <Filename><CM MAC address><epoch>, with underscores between
<Filename> and <CM MAC address> and <epoch> having significance. The epoch time (also known as 'unix time')
is defined as the number of seconds that have elapsed since midnight Coordinated Universal Time (UTC), Thursday,
1 January 1970.
Hence, the default format would be:
    UsLatencySum_<CM MAC address>_<epoch>
    For example: USLatencySum_0010181A2D11_1403405123

D.3.2    Latency Performance Reporting File Format
The data file is composed of a header plus the Latency Data. The header is composed of ordered fixed-length fields.
Unless otherwise specified, the header fields contain hex values that are right-justified within the field. If necessary,
the field is left-padded with zero values.
Syntax of the file is as follows:
                                    Table 83 - Upstream Latency Summary File Format

                                            Element                        Size or Data Type
                         File type (value = 4C4C4401)                    4 bytes
                         Major Version (Value = 1)                       1 byte
                         Minor Version                                   1 byte
                         CM MAC Address                                  6 bytes
                         Number of LatencySummaryData objects (n)        1 byte
                         Latency Data                                    n*LatencySummaryData



           File Header Element Definitions

D.3.2.1.1 Major Version
This element represents the current file header version. The current defined Major Version is 1. The version is
incremented by one when the file header format is modified by specification.

D.3.2.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.3.2.1.3 Number of LatencySummaryData Objects
This number indicates how many instances of the LatencySummaryData type are included in the file. The CM
MUST include an instance of LatencySummaryData for every service flow for which latency histogram calculation
is turned on for any portion of the file reporting period. In addition, when the latency histogram calculation is
disabled for a service flow during the file reporting period, the CM MUST conclude the current
LatencySummaryData instance using the current time (as Snapshot End Timestamp) and histogram counts.




02/19/25                                                CableLabs®                                                   197
<!-- PDF_PAGE: 198 -->
CM-SP-CM-OSSIv3.1-I27-250219                                       Data-Over-Cable Service Interface Specifications


D.3.2.1.4 LatencyData
The set of Latency data encoded using the LatencySummaryData type described below. There is at least one set of
data per histogram enabled Service Flow.

          LatencySummaryData Type
This data type represents the estimated latency of packets for a service flow. The number of sanctioned packets and
maximum measured latency during the measured interval are also reported.
Multiple instances of this data type are included in the file as indicated by the Number of LatencySummaryData
objects header element for the file. In addition, if bin edges for a Service Flow are changed while Reporting is
active, there will be multiple LatencySummaryData instances for the same Service Flow.
                                              Table 84 - LatencySummaryData

                                                 Element                              Size
                               SfId                                         4 bytes
                               SfLabel                                      16 bytes
                               Number of Bin Edges                          1 byte
                               Array of Bin Edge Definitions                2bytes * Number of
                                                                            Bin Edges
                               NumSnapshots                                 2 bytes
                               First Snapshot Start Timestamp               4 bytes
                               Latency Snapshot Entry Format (1 entry per snapshot)
                                   Snapshot End Timestamp                   4 bytes
                                   Array of Bin counts                      8bytes * (Number of
                                                                            Bin Edges+1)
                                   MaxLatency                               4 bytes
                                   NumHistogramUpdates                      4 bytes
                                   SanctionedPkts                           4 bytes
                                   DroppedPkts                              4 bytes
                                   TotalEct0Pkts                            4 bytes
                                   Reserved                                 4 bytes
                                   TotalEct1Pkts                            4 bytes
                                   CeMarkedEct1Pkts                         4 bytes



D.3.2.2.1 SfId
The Service Flow identifier.

D.3.2.2.2 SfLabel
The Service Class Name signaled to the CM or the label configured by the operator by setting the SfLabel
management object.

D.3.2.2.3 NumberOfBinEdges - 1 byte
The number of bin edges defined for the latency histogram for measurement on this Service Flow. The number of
bin edges is one less than the number of bins used in the measurement, e.g., 9 bin edges will give 10 bins for
measurement.

D.3.2.2.4 Array of Bin Edge Definitions - 2 bytes * NumberOfBinEdges
The array of bin edges defined for this latency histogram measurement on this Service Flow. This element is
expressed as an array of bin edges expressed as 16-bit unsigned integers encoding latency values with a resolution of



198                                                      CableLabs®                                        02/19/25
<!-- PDF_PAGE: 199 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv3.1-I27-250219


0.01 ms. This can cover a range of latency values from 0 ms to 655 ms. A properly formatted array specifies bin
edges in monotonically increasing order, and operation is undefined otherwise.

D.3.2.2.5 NumSnapshots - 2 bytes
The number of Latency Snapshot Entries for this Service Flow.

D.3.2.2.6 First Snapshot Start Timestamp - 4 bytes
The start time for the first Snapshot Entry for this instance.

D.3.2.2.7 Latency Snapshot Entry Format
Each Latency Snapshot Entry consists of a value for the Snapshot End TimeStamp, an array of Latency Bin Counts
(NumPktsBin1, NumPktsBin2, … NumPktsBinN,), a Max Latency value, the number of SanctionedPackets, and the
number of histogram updates for a snapshot interval.
    •    Snapshot End TimeStamp - 4 bytes
         The time when this Latency Snapshot ended. The value is expressed in epoch time (also known as 'unix
         time') and is defined as the number of seconds that have elapsed since midnight Coordinated Universal
         Time (UTC), Thursday, 1 January 1970.
    •    Array of Bin counts - (8 bytes * (NumberOfBinEdges + 1) )
         An array representing the histogram bin counts during the Snapshot interval for the service flow. Each bin
         is an 8 Byte value. The length of the array is NumberOfBinEdges+1.
    •    Maximum Latency - 4 bytes
         The maximum latency (in microseconds) measured for this service flow during the Snapshot interval.
    •    Number of Histogram Updates - 4 bytes
         The number of histogram updates on this service flow during the snapshot interval.
    •    Number of SanctionedPackets - 4 bytes
For an Upstream Low Latency Service Flow, this attribute counts the number of packets redirected from the Low
Latency Service Flow to the Classic Service Flow recorded on this Service Flow during the snapshot interval. For
other Service Flow types, this counter reports 0.
    •    Number of Dropped Packets - 4 bytes
         The total number of dropped packets on this service flow during the snapshot interval.
    •    Number of TotalEct0Pkts Packets - 4 bytes
         This attribute represents the count of packets that arrived marked as ECT0.
    •    Number of TotalEct1Pkts Packets - 4 bytes
         This attribute represents the count of packets that arrived marked as ECT1.
    •    Number of CeMarkedEct1Pkts Packets - 4 bytes
         This attribute represents the count of packets that arrived marked as ECT1 and were marked as Congestion
         Experienced (CE) by the CM.

D.3.3    Example
This is an example showing the US Latency Data encoding for 2 Service Flows. In this example, there are only 5
Snapshots for each service flow, whereas normally there could be more depending on the setting (e.g., 2000 sets of
latency data for say 2 second intervals, and 1000 as the number of snapshot intervals).




02/19/25                                              CableLabs®                                                  199
<!-- PDF_PAGE: 200 -->
CM-SP-CM-OSSIv3.1-I27-250219                                     Data-Over-Cable Service Interface Specifications


The columns of data correspond to the values for the Snapshot End TimeStamp, NumPktsBin1, NumPktsBin2, …
NumPktsBinN, Max Latency, Number of histogram updates, SanctionedPackets, respectively.
The values in the example are expressed as decimal integer values, but they are encoded using 32-bit hexadecimal
notation in the file. The order of latency snapshot entries in the file is from oldest to newest, meaning that the
latency snapshot entry following the Header Elements is the oldest set.
Service Flow Id = 1050
SfLabel = 'LLService1'
Number of Bin Edges = 9
Array of Bin Edge definitions = 1,2,5,10,15,20,50,100,400
Number Snapshots = 5
First Snapshot Start Timestamp = 1456252718
                                                                                                 Max       Num         Num
    TimeStamp     Bin1    Bin2    Bin3    Bin4    Bin5   Bin6      Bin7   Bin8   Bin9   Bin10   Latency    Hist      Sanctioned
                                                                                                          Updates       Pkts
    1456252719      10     2303    233     33      3        1        1     1      0       0       57       2575          9
    1456252720      12     2510    233     50      3        1        1     1      0       0       60       2519          10
    1456252721      12     2320    233     30      3        1        1     1      0       0       51       2501          8
    1456252722      10     2430    233     40      3        1        1     0      0       0       21       2708          4
    1456252723      15     2510    233     50      2        1        1     0      0       0       22       2797          2

Service Flow Id = 1051
SfLabel = 'CService1'
Number of Bin Edges = 9
Array of Bin Edge definitions = 1,2,5,10,15,20,50,500,650
Number Snapshots = 5
First Snapshot Start Timestamp = 1456252718
                                                                                                 Max       Num         Num
    TimeStamp     Bin1    Bin2    Bin3   Bin4    Bin5    Bin6      Bin7   Bin8   Bin9   Bin10   Latency    Hist      Sanctioned
                                                                                                          Updates       Pkts
    1456252719      11     11      23     233     2303      33      11     3      1       1      1001       62           0
    1456252720      13     13      25     233     2510      50      13     3      1       1      1050       63           0
    1456252721      14     14      23     233     2320      30      14     3      1       1      1021       62           0
    1456252722      12     12      24     233     2430      40      12     3      1       0       500       63           0
    1456252723      16     16      25     233     2510      50      16     2      1       0       510       62           0


Hex Notation:
0000041A 0000000000004c4c5365727669636531 09 0064 00c8 01F4 03E8 05DC 07D0 1388 2710 9c40 0005 56CCA72E
56CCA72F 00000000000000000A 00000000000008FF … 00000039 00000A0F 00000009
56CCA730 00000000000000000C 00000000000009CE … 0000003C 000009D7 0000000A
…
…
…
0000041B 00000000000000435365727669636531 09 0064 00c8 01F4 03E8 05DC 07D0 1388 C350 FDE8 0005 56CCA72E




200                                                CableLabs®                                              02/19/25
