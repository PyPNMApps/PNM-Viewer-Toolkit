# Annex D.1 - Extended Network Monitoring Requirements - Overview

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 188-188
Classification: Normative (explicit)

<!-- PDF_PAGE: 188 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


Annex D            Extended Network Monitoring Requirements (Normative)
D.1 Overview
This annex addresses
      •   Proactive Network Maintenance and Enhanced Signal Quality Monitoring requirements for plant conditions
          and
      •   reporting of Service Flow Latency statistics.

D.1.1     PNM
PNM tests require downstream connectivity to provide accurate results. Therefore, if there are changes in the system
during an active PNM test that modify the RCS of the CM or errors in the system that cause the CM to switch the
primary downstream channel to a back-up primary downstream channel, the data collected by the PNM test would
be invalidated. The CM MUST abort all active PNM tests if the CM receives a DBC-REQ message changing its
RCS while one or more PNM tests are active. The CM MUST abort all active PNM tests if a loss of downstream
connectivity occurs that causes the CM to switch its primary downstream channel to its backup primary downstream
channel when one or more PNM tests are active.

D.1.2     Latency Reporting
The CM is capable of calculating the estimated queueing delay on a per service flow basis. The operator can enable
statistics gathering and reporting for these latency estimates for individual service flows. These statistics are
reported in the form of a configurable histogram.

D.2 Proactive Network Maintenance Information Model
This section defines the Proactive Network Maintenance objects including the associated attributes.




188                                                  CableLabs                                           04/29/26
