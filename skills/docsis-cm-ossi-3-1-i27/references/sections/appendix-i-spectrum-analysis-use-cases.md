# Appendix I - Spectrum Analysis Use Cases

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 303-305
Classification: Informative (explicit)

<!-- PDF_PAGE: 303 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv3.1-I27-250219


Appendix I             Spectrum Analysis Use Cases (Informative)
This appendix describes several use cases where the Signal Quality Monitoring features introduced in DOCSIS 3.0
can be utilized to manage the HFC plant.
To maintain the HFC network in optimal conditions constant monitoring of the physical characteristics is desired.
This practice helps in the early detection of plant problems. These problems, if not properly corrected could cause
degradation of services that are offered over the DOCSIS network. The RF impairments may often be the root cause
of the problem affecting the quality of services offered over DOCSIS. These impairments result in excessive
logging, and poor statistics indicating a lower quality of experience for customer of the services.
Ideally, rather than inferring the presence of RF impairments in the HFC from DOCSIS MAC statistics (for
example), the use of Signaling Quality measurement equipment dedicated to monitor the HFC spectrum is desired.
However, the cost of such equipment and its associated management and operation may not be justifiable. Instead,
active network elements such as CMTSs have evolved their capabilities to report RF measurements using an SNMP
management interface. The main advantage of this approach is the constant availability of information across the
network. Such information can be correlated to determine e.g., a group of CMs with a common tap in the HFC path
reporting the same measurements problem. The signal monitoring approach is similar to how specialized equipment
is used to further isolate the problems based on the coarse measurements from a CMTS.
This appendix describes use cases for two main categories of the Enhanced Signaling Quality Monitoring features of
DOCSIS 3.0:
    •    Normalization of RF Impairment Measurements
    •    Spectrum Amplitude Measurements for Upstream Interfaces

I.1 Normalization of RF Impairment Measurements

DOCSIS [RFC 4546] provides SNR (Signal-to-Noise) measurement. SNR among other measurements are available
on a per CM basis and per interface.
SNR values reported may not be uniform amongst different CMTS vendors. Therefore, it might not be possible to
compare and analyze information from different devices to determine the HFC plant conditions.
Major contributors to impairments in the DOCSIS channels are linear distortion, non-linear distortion, impulse noise
and ingress noise.
DOCSIS pre-equalization provides a mechanism to correct the linear distortion of each individual CM transmission.
Ingress noise robustness has no specification requirements beyond the assumed RF plant conditions in [PHYv3.1].
However, vendors have provided mechanisms to mitigate noise and ingress interference in plants that have more
severe noise conditions than the ones assumed in the [PHYv3.1] specification.
The available RF measurements in DOCSIS 3.0 are listed in Table 144 where the DOCSIS 3.0 added features are
indicated in bold text and are the basis for the use cases of this section. In general, downstream RF measurements
are performed by individual CMs while the upstream measurements are performed by the CMTS either at an
interface or at a CM level. Based on CMTS and CM interactions, the CM provides an indirect measure of the
distortion in the upstream channel through its pre-equalization coefficients.
                                    Table 144 - RF Management Statistics Available

           CM (Downstream                  CMTS (Upstream Measurements)                Measurements Categories
            Measurements)
 SNR                                     SNR                                       Noise conditions
 RxMER                                   RxMER
                                         CNIR
                                         Expected Received Power                   Power level
 Correctable/uncorrectable errors        Correctable/uncorrectable errors per CM   FEC performance statistics




02/19/25                                              CableLabs®                                                 303
<!-- PDF_PAGE: 304 -->
CM-SP-CM-OSSIv3.1-I27-250219                                        Data-Over-Cable Service Interface Specifications


          CM (Downstream                    CMTS (Upstream Measurements)                    Measurements Categories
           Measurements)
                                          Correctable/uncorrectable errors per US
                                          interface
 Downstream micro-reflections             Upstream micro-reflections per CM             Linear distortion
 CM post-equalization data                CM pre-equalization1
 Note:
 1
  CM may provide more accurate pre-equalization coefficient than what the CMTS is able to calculate.

The following use cases refer to the noise measurement enhancements for DOCSIS 3.0.

I.1.1 Use Case 1: Figure of Merit Estimation for Logical Upstream Channel

This Use Case defines a Figure of Merit for Logical Upstream Channel measurement that an operator can use to
periodically collect information to characterize the performance of the HFC part of the Cable distribution network.
To overcome non-uniform SNR measurements, DOCSIS 3.0 defines two measurements: RxMER (Receive
Modulation Error Rate) and CNIR (Carrier to Noise plus Interference Ratio). These provide better indication of the
HFC plant impairments and the corrections achieved by the CMTS through compensation techniques. Combining
RxMER and CNIR, a Figure of Merit of impairment compensation efficiency can be defined when noise or
interference is present.
RxMER measures the average quantization error just prior to FEC, and CNIR measures the carrier to noise plus
interference ratio prior to demodulation. A Figure of Merit of how efficiently interference and distortion is
compensated in a logical channel can be defined as:
                                   Figure of Merit (logical channel) = RxMER - CNIR
The variables from Annex D to retrieve are:
      •   RxMER: docsIf3SignalQualityExtRxMER
      •   CNIR: docsIf3CmtsSignalQualityExtCNIR
The Figure of Merit is relevant when the device is capable of suppressing ingress, thus increasing the RxMER value
with respect to the channel CNIR.
To minimize the uncertainties in measuring the Figure of Merit due to distortion that is unique to individual
upstream paths between a CM and CMTS, it is advisable to operate with pre-equalization on (see
docsIfUpChannelPreEqEnable of [RFC 4546]).

I.1.2 Use Case 2: Figure of Merit Estimation per CM

This Use Case defines a Figure of Merit per CM transmission. Similar to Use Case 1, the operator can periodically
collect information to characterize the performance of CMs in terms of Figure of Merit for the given CMTS the CM
is attached to.
Unlike RxMER, the SNR parameter is unique for each CM. This allows you to define a Figure of Merit on a per CM
basis. A Figure of Merit of how efficiently interference and distortion affecting a CM is compensated can be defined
as:
                     Figure of Merit (CM) = SNR (CM) - CNIR (of the logical upstream channel)
The variables from [DOCS-IF3-MIB] and Annex D to retrieve are:
      •   SNR: docsIf3CmtsCmUsStatusSignalNoise
      •   CNIR: docsIf3CmtsSignalQualityExtCNIR
This Figure of Merit indicates if a CM, through its pre-equalization mechanism, is efficiently compensating the
linear distortion in its upstream path.




304                                                     CableLabs®                                            02/19/25
<!-- PDF_PAGE: 305 -->
Cable Modem Operations Support System Interface Specification               CM-SP-CM-OSSIv3.1-I27-250219


I.1.3 Use Case 3: Absolute Noise and Interference Estimation

Traditionally CMTSs are expected to command the CMs' power transmission so that the CMTS received power is
close to 0 dBmV across all CMs.
This Use Case defines how an operator may derive the absolute value of the noise plus interference (in dBmV) from
the reported value (CNIR in dB) which is a relative measure.
For example, CNIR and ExpectedRxSignalPower can be used to estimate noise and interference levels (N+I) across
the operator's network in dBmV as:
                 N + I = CNIR - ExpectedRxSignalPower (CMs of the logical upstream channel)
Operators may determine the difference between the target and the actual received power at the CMTS using the
following equation:
                          CM Offset Power = CM Rx Power - ExpectedRxSignalPower
The variables from [DOCS-IF3-MIB] and Annex D to retrieve are:
    •   CM Rx Power: docsIf3CmtsCmUsStatusRxPower
    •   ExpectedRxSignalPower: docsIf3CmtsSignalQualityExtExpectedRxSignalPower
Operators may estimate individual CM CNIR by combining the CNIR obtained for the logical channel and the CM
offset power as follows:
                                CM Estimated CNIR = CM Offset Power + CNIR
CM Offset Power: The difference between the actual received CM power level and the expected commanded
received signal power at the CMTS.
The variables from [DOCS-IF3-MIB] and Annex D to retrieve are:
    •   CNIR: docsIf3CmtsSignalQualityExtCNIR
    •   CM Rx Power: docsIf3CmtsCmUsStatusRxPower
    •   Expected Commanded Received Signal Power: docsIf3CmtsSignalQualityExtExpectedRxSignalPower




02/19/25                                         CableLabs®                                                     305
