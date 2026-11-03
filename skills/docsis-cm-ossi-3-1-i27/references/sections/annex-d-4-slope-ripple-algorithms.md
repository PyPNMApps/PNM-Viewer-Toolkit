# Annex D.4 - Slope and Ripple Algorithms

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 201-204
Classification: Normative (explicit)

<!-- PDF_PAGE: 201 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv3.1-I27-250219


56CCA72F 00000000000000000B 000000000000000B … 000003E9 0000003E 00000000
56CCA730 00000000000000000D 000000000000000D … 0000041A 0000003F 00000000
…

D.4 Slope and Ripple Algorithms
D.4.1    Overview
This section provides details to specify the summary metrics for downstream channel estimate coefficients and
upstream pre-equalizer coefficients. The summary metrics are the slope, mean, RMS ripple, and peak-to-peak ripple,
for both the magnitude and group delay of the coefficients. The purpose of these summary metrics is to avoid having
to send all 2K/4K/8K coefficients on every query. These definitions are intended to replace the less-specific
reference in the OSSI spec to the SCTE Measurement Recommended Practices document [SCTE RP].
To provide a basis for the analysis, downstream channel estimate coefficient data was taken from a DOCSIS 3.1
OFDM downstream receiver, with channel start frequency of 543.6 MHz, subcarrier spacing of 50 kHz, and 1880
subcarriers over a bandwidth of 94 MHz. The channel was nominally AWGN with high SNR, although there is
some slope and ripple due to the test setup.

D.4.2    Best-Fit Equations
A best-fit line to a set of real-valued data {xi, yi} may be computed using the following equations, which appear in
section 6.5, Nominal Relative Carrier Power Levels and Carrier Level Variations, and Appendix, of [SCTE RP].




where                  and                  are the means of x and y, respectively.


The best-fit line has samples




The residual samples after removing the fit line is




The RMS and peak-to-peak ripple are computed based on the residual values as follows:




The above equations are general and apply to both the magnitude and group delay responses. The data points {xi, yi}
need not be equally spaced on the x (frequency) axis; that is, the equations continue to be valid if there are missing
values of the channel response due to, for example, excluded subcarriers.



02/19/25                                              CableLabs®                                                  201
<!-- PDF_PAGE: 202 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


D.4.3     Magnitude Summary Metrics
We are given a set of NC complex channel estimate coefficients



where each channel estimate coefficient corresponds to a subcarrier frequency. The subcarrier frequencies for which
channel estimates are provided need not be contiguous. To compute the magnitude summary metrics, the inputs
applied to the best-fit equations are NC, the number of subcarriers for which channel estimate data is provided; x, the
list of subcarrier frequencies in MHz; and y, the list of channel estimate coefficient magnitudes in dB for each
subcarrier. The y values are computed using



For the example data, the log-magnitude-vs-frequency response of the channel estimate coefficients read out from
the receiver is plotted in Figure 10 in blue.




              Figure 10 - Frequency Response of Channel Estimate Coefficient, with Best-Fit Line


The calculations for the best-fit line for magnitude are performed using the above best-fit equations directly on the
dB values of y. The best-fit line is plotted in Figure 10 in red. The summary metrics for magnitude are the slope m in
dB/MHz, mean amplitude ӯ = Amean in dB, RMS ripple Rrms in dB, and the peak-to-peak ripple Rpp in dB.
The metrics for the above example data are
      m = -0.0128 dB/MHz
      Amean = -20.2224 dB




202                                                 CableLabs®                                               02/19/25
<!-- PDF_PAGE: 203 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv3.1-I27-250219


    Rrms = 0.2658 dB
    Rpp = 1.2379 dB

D.4.4    Group Delay Summary Metrics
Group delay is the time delay experienced by a modulated signal, such as a QAM or OFDM/OFDMA downstream
or upstream signal, as it passes through a transmission system such as the cable plant. If the group delay is different
for different frequency components across the signal bandwidth, it causes distortion of the signal. Thus measuring
the group delay vs frequency provides important information about plant performance.
Given the Nc complex channel estimate coefficients



the group delay is computed as follows. The angle in radians of each channel coefficient is computed as



The general definition for group delay is




where τ is the group delay in seconds, is the unwrapped phase in radians, and ω is the frequency in radians per
second. In our case, we are not given a continuous function of radian frequency ω since the channel estimate
coefficients are defined at discrete frequencies corresponding to each subcarrier for which channel estimate data is
provided. We can approximate the group delay using phase and frequency differences from subcarrier to subcarrier
in place of the derivative. This gives the formula for group delay in ns as




where Δf is the subcarrier spacing in Hz, the phase samples are in radians, and the phase difference       is defined
modulo 2π as



with the integer k selected such that



This modulo operation on the phase differences effectively unwraps the phase, preventing jumps at 2π interval
boundaries that could cause spikes in the group delay.
The frequency value associated with the group delay sample computed on two adjacent subcarriers with respective
frequencies fi and fi + Δf is



that is, halfway between the two adjacent subcarriers.
Since we are using phase differences from subcarrier to subcarrier to compute the group delay, the number of group
delay values Ng will be less than the number of channel estimate coefficients Nc. In addition, each isolated missing
channel coefficient will cause two missing group delay values. While more complex processing could provide
estimates of the group delay at the missing subcarriers, the approach described herein is considered adequate to
provide meaningful summary metrics.




02/19/25                                            CableLabs®                                                      203
<!-- PDF_PAGE: 204 -->
CM-SP-CM-OSSIv3.1-I27-250219                                   Data-Over-Cable Service Interface Specifications


Having computed the group delay for each pair of consecutive channel coefficients, we apply the above best-fit
equations to compute the summary metrics. The inputs to the best-fit equations for group delay are Ng, the number
of group delay values; x = {fgi}, the list of frequencies in MHz corresponding to the group delay values; and y = {τi},
the list of group delay values in ns. The outputs are the slope m in ns/MHz, mean group delay ӯ = Gmean in ns, RMS
ripple Rrms in ns, and peak-to-peak ripple Rpp in ns.
The group-delay-vs-frequency response for the above example channel is plotted in Figure 11 with the best-fit line
in red.




             Figure 11 - Group Delay Response of Channel Estimate Coefficients, with Best-Fit Line


The summary group delay metrics for the example data are
      m = -0.0012 ns/MHz
      Gmean = 1808.6 ns
      Rrms = 4.0300 ns
      Rpp = 25.4990 ns

D.4.5     Group Delay Summary Metrics Example
A numerical example of the calculations for the summary metrics is provided in [Slope/Ripple]. The referenced
spreadsheet contains an example with channel start frequency of 543.6 MHz, subcarrier spacing of 50 kHz, and 16
channel estimate coefficients corresponding to 16 active subcarriers, plus one excluded subcarrier at 544.05 MHz.
The cells in the spreadsheet contain formulas and results for intermediate computations, as well as the final
summary metrics (MIB outputs). The second tab in the spreadsheet has plots for amplitude and group delay,
including the best-fit line for each.




204                                                 CableLabs®                                               02/19/25
