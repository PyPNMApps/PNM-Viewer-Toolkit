# 10 - OSSI for LED Indicators

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 114-115
Classification: Main body (contains normative requirements)

<!-- PDF_PAGE: 114 -->
CM-SP-CM-OSSIv4.0-I13-260429                                   Data-Over-Cable Service Interface Specifications


10 OSSI FOR LED INDICATORS
The CM SHOULD support standard front-panel LEDs (Light Emitting Diodes) that present straightforward
information about the registration state of the CM so as to facilitate efficient customer support operations.

10.1 CM LED Requirements and Operation
A CM SHOULD support LEDs which have three states: 1) unlit, 2) flash, 3) lit solid. A CM LED in the 'flash' state
SHOULD turn on and off with a 50% duty cycle at a frequency not less than 2 cycles per second. A CM SHOULD
support LEDs which light sequentially, following the normal CM initialization procedure specified in [MULPIv4.0].
In this way, the installer can detect a failure that prevents the CM from becoming operational.
A CM SHOULD have a minimum of five externally visible LEDs divided into three functional groups as indicated
below:
      BOX: This group SHOULD have 1 LED labeled as POWER for the BOX status.
      DOCSIS: This group SHOULD have 3 LEDs labeled as DS, US, and ONLINE for the DOCSIS interface status.
      The LEDs in the DOCSIS group SHOULD be in the order: DS, US, and ONLINE, from left to right, or top to
      bottom, as appropriate for the orientation of the device.
      CPE: This group SHOULD have a minimum of 1 LED labeled as LINK for the LINK status. The CM MAY
      have multiple LEDs in the CPE group to represent individual CPE interface types and parameters. These CM
      CPE LEDs MAY be labeled according to their associated interface types.
There is no specific requirement for labeling the functional groups. The overall CM LED distribution SHOULD be
in the order: POWER, DS, US, ONLINE, and LINK.
The CM SHOULD use these LEDs to indicate that the following modes of operation are in progress, or have
completed successfully:
      •   Power on, Software Application Image Validation and Self Test
      •   Scan for Downstream Channel
      •   Resolve CM-SG and Range
      •   Operational
      •   Data Link and Activity
The CM SHOULD operate its LEDs as described in the following sections for each of the above modes of
operation.

10.1.1 Power On, Software Application Image Validation and Self Test
The CM SHOULD, when turned on, place the LEDs, or at least the DOCSIS Group LEDs (DS, US, ONLINE), in
the 'flash' state while the CM performs the system initialization of the Operational System, CM application load, and
any proprietary self-tests. Following the successful completion of the steps above, the CM SHOULD place the
LEDs, or at least the DOCSIS Group LEDs, in the 'lit solid' state for one second, after which the CM places the
POWER LED in the 'lit solid' state. The CM MAY also place the LINK LED in the 'lit solid' state if a CPE device is
properly connected (see Section 10.1.5). If the system initialization, described above, results in a failure, the CM
SHOULD place the LEDs, or at least the DOCSIS Group LEDs in the 'flash' state, in which they should remain.

10.1.2 Scan for Downstream Channel
The CM SHOULD place the DS LED in the 'flash' state as the CM scans for a candidate primary downstream
DOCSIS channel. The CM SHOULD place the DS LED in the 'lit solid' state when the CM MAC layer has
completed synchronization of MPEG framing of the candidate primary downstream channel, as defined in the Cable
Modem Initialization and Reinitialization section of [MULPIv4.0]. The CM SHOULD maintain the 'lit solid' state of
the DS LED as the CM continues the initialization process. The CM SHOULD NOT place the DS LED in the 'flash'



114                                                 CableLabs                                              04/29/26
<!-- PDF_PAGE: 115 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


state when resolving the CM service groups or performing downstream acquisition of CM receive channels in the
registration process as defined in the Cable Modem Initialization and Reinitialization section of [MULPIv4.0].
Whenever the CM restarts CM initialization (which can include scanning for a downstream channel and attempting
to synchronize to a downstream channel), the CM SHOULD place the DS LED in the 'flash' state and the US LED
and ONLINE LED in the 'unlit' state.

10.1.3 Resolve CM-SG and Range
After the CM places the DS LED in the 'lit solid' state, the CM SHOULD place the US LED in the 'flash' state and
the ONLINE LED in the 'unlit' state while the CM is determining CM-SGs and performing initial ranging, until the
CM receives a ranging response message with a ranging status of 'success' from the CMTS. When the CM receives a
ranging response message with a ranging status of 'success' from the CMTS, the CM SHOULD place the US LED in
the 'lit solid' state.
The CM SHOULD maintain the 'lit solid' state of the US LED as the CM continues the initialization process. Unless
the channel used to transmit the registration request message is not in the TCC received in the registration response
message, the CM SHOULD NOT place the US LED in the 'flash' state when performing upstream acquisition of
CM transmit channels in the registration process as defined in the Cable Modem Initialization and Reinitialization
section of [MULPIv4.0]. The CM SHOULD maintain the 'lit solid' state of the US LED when the CM is ranged on
one or more upstream channels.

10.1.4 Operational
After the CM places the US LED in the 'lit solid' state, the CM SHOULD place the ONLINE LED in the 'flash' state
while the CM continues the process towards become operational (this includes performing early authentication,
establishing IP connectivity, and registering with the CMTS, and performing BPI initialization). When the CM is
operational, the CM SHOULD place the ONLINE LED in the 'lit solid' state. Operational is defined according to the
Cable Modem Initialization and Reinitialization section in [MULPIv4.0].
If at any point there is a failure in the registration process that causes the CM to lose its operational state, including
but not limited to loss of the primary downstream channel, ranging, DHCP, configuration file download,
registration, and Baseline Privacy initialization, the CM SHOULD place the ONLINE LED in the 'flash' state.
If the CM becomes operational and the CM configuration file has the Network Access Control Object (NACO) set
to zero (0), the CM SHOULD place the ONLINE LED in the 'unlit' state and place both the DS and US LEDs in the
'flash' state. Refer to the Common Radio Frequency Interface Encodings Annex of [MULPIv4.0] for details on the
Network Access Control Object (NACO).

10.1.5 Data Link and Activity
The CM SHOULD place the LINK LED in the 'lit solid' state when a CPE device is connected and the CM is not
bridging data. The CM SHOULD place the LINK LED in the 'flash' state ONLY when the CM is bridging data
during the CM operational state and NACO set to one (1). The CM SHOULD NOT place the LINK LED in the
'flash' state for data traffic originating or terminating at the CM device itself.
If LINK is detected with a CPE device, the CM MAY set the LINK LED to the 'lit solid' state any time after the
power and self test steps are completed.

10.2 Additional CM Operational Status Visualization Features
The CM MAY change the DOCSIS defined LED behavior when the CM is in a vendor proprietary mode of
operation. The CM MUST NOT have additional LEDs that reveal DOCSIS specific information about the
configuration file content, or otherwise clearly specified (see NACO visualization in Sections 10.1.4 and 10.1.5.

10.2.1 Secure Software Download
The CM SHOULD signal that a Secure Software Download is in process, by setting the DS LED and the US LED to
the 'flash' state, and the ONLINE LED to the 'lit solid' state.



04/29/26                                              CableLabs                                                       115
