# Annex D.2.9 - CmDsOfdmMerMargin

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 216-217
Classification: Normative (explicit)

<!-- PDF_PAGE: 216 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


D.2.8.10.1.2 Minor Version
The vendor-specified version information. The default value is zero if no vendor version is assigned.

D.2.8.10.1.3 Capture Time
The epoch time (also known as 'unix time') is defined as the number of seconds that have elapsed since midnight
Coordinated Universal Time (UTC),Thursday, 1 January 1970.

D.2.8.10.1.4 DS Channel Id
This element represents the channel Id of the downstream channel for which the RxMer data apply.

D.2.8.10.1.5 CM MAC Address
The CM MAC Address.

D.2.8.10.1.6 Subcarrier Zero Frequency in Hz
This element is the center frequency of subcarrier zero of the OFDM channel.

D.2.8.10.1.7 FirstActiveSubcarrierIndex
This element is the subcarrier index of the lowest subcarrier in the Encompassed Spectrum of the channel.

D.2.8.10.1.8 Subcarrier Spacing in kHz
This element is the OFDM subcarrier spacing.

D.2.8.10.1.9 Length in Bytes of RxMER Data
This element is the number of bytes of the RxMerData.

D.2.9       CmDsOfdmMerMargin
The purpose of this item is to provide an estimate of the MER margin available on the downstream data channel
with respect to a modulation profile. The profile may be a profile that the modem has already been assigned or a
candidate profile. This is similar to the MER Margin reported in the OPT-RSP Message [MULPIv4.0]. The CM
calculates the Required Average MER for the profile based on the bit loading for the profile and the Required MER
per Modulation Order provided in the CmDsOfdmRequiredQamMer Table. For profiles with mixed modulation
orders, this value is computed as an arithmetic mean of the required MER values for each non-excluded subcarrier in
the Modulated Spectrum. The CM then measures the RxMER per subcarrier and calculates the Average MER for
the Active Subcarriers used in the Profile and stores the value as MeasuredAvgMer. The Operator may also compute
the value for Required Average MER for the profile and set that value for the test. The CM also counts the number
of MER per Subcarrier values that are below the threshold determined by the CmDsOfdmRequiredQamMer and the
ThrshldOffset. The CM reports that value as NumSubcarriersBelowThrshld. This table will have a row for each
ifIndex for the modem.
                               Table 102 - CmDsOfdmMerMargin Object Attributes

         Attribute Name               Type           Access      Type Constraints        Units          Default Value
  IfIndex                       InterfaceIndex     Key
  ProfileId                     UnsignedByte       R/W          0..15                                   0
  ThrshldOffset                 UnsignedByte       R/W                                quarterDb         0
  MeasEnable                    Boolean            R/W                                                  False
  NumSymPerSubcarToAvg          UnsignedShort      R/W                                                  8
  ReqAvgMer                     UnsignedByte       R/W                                quarterDb         0
  NumSubcarBelowThrshld         UnsignedShort      R/O
  MeasuredAvgMer                UnsignedInt        R/O                                HundredthsDb




216                                                CableLabs                                                   04/29/26
<!-- PDF_PAGE: 217 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


        Attribute Name                  Type            Access       Type Constraints           Units      Default Value
  AverageMerMargin                Int                 R/O                                  HundredthsDb
  MeasStatus                      MeasStatusType      R/O



           IfIndex
This attribute is the interface index of the downstream channel and is a key to provide an index into the table.

           DsProfileId
This attribute represents the Downstream Profile ID of the Profile. The profile may be a profile that the modem has
already been assigned or a candidate profile. This value can only be changed while a test is not in progress. An
attempt to set this value while the value of MeasStatus is 'busy' will return 'inconsistentValue'.

           ThreshldOffset
This attribute represents the number of dB below the CmDsOfdmRequiredQamMer value for a given modulation
order that is likely to cause uncorrectable errors. Measurements of Subcarrier MER that are this number of dB or
more below the CmDsOfdmRequiredQamMer for the Subcarrier for the profile being tested, will cause the CM to
increment the count of the NumOfSubcarBelowThreshld attribute.
This value can only be changed while a test is not in progress. An attempt to set this value while the value of
MeasStatus is 'busy' will return 'inconsistentValue'.

           MeasEnable
This attribute causes the CM to begin measurement. When the measurement is complete, the MeasEnable attribute is
set internally to 'false' by the CM.
This value is only allowed to be set to 'true' if the value of 'MeasStatus' is a value other than 'busy' for this row AND
for any row in the table. That is, only one row in the table is allowed to be 'true' at the same time. Setting this value
to 'true' will change the value of 'MeasStatus' to 'busy'.
Setting this object to a value of 'false' instructs the CM to stop the measurement.
This object returns 'true' if the CM is actively taking a measurement; otherwise it returns 'false'.
This object returns 'inconsistentValue' if set to 'true' while the value of 'MeasStatus' is a value of 'busy' for this row
OR for any row in the table. That is, only one row in the table is allowed to be 'true' at the same time.
This object returns 'inconsistentValue' if set to 'true' while the CM is in DOCSIS Light Sleep (DLS) mode or the CM
is in battery-backup mode.
Setting this object to a value of 'true' will change the value of the 'MeasStatus' to 'busy'.

           NumSymbolsPerSubcarrierToAverage
This attribute represents the number of symbols that will be used in the calculation of the average MER per
subcarrier. This value can only be changed while a test is not in progress. An attempt to set this value while the
value of 'MeasStatus' is 'busy' will return 'inconsistentValue'.

           ReqAvgMer
This attribute represents the minimum required average MER. This value can either be computed by the entity
requesting the test or be computed by the test when the attribute uses the default value 0. The CM subtracts this
value from the MeasuredAvgMer to obtain the AverageMerMargin. If this value is not provided by the PNM server
(i.e., it is the default value of zero), the CM computes the value as the average of the required MER for all of the
subcarriers, based on the values in the CmDsOfdmRequiredQamMer table, for the Modulation order used for each
of the subcarriers for the profile being analyzed. For profiles with mixed modulation orders, this value is computed




04/29/26                                              CableLabs                                                       217
