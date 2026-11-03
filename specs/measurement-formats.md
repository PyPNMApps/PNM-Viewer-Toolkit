# Measurement decoder evidence

Sources are the versioned **text extracts**, independently inspected for both
issues. The user authorized text when a PDF layout is unambiguous. These records
do not claim visual PDF verification. Local extract hashes are recorded in
`measurement-source-hashes.json`; fixture hashes are in
`tests/fixtures/pypnm/provenance.json`.

| Family | DOCSIS 3.1 I27 | DOCSIS 4.0 I13 | Payload interpretation |
| --- | --- | --- | --- |
| Symbol | Table 61, p165 | Table 94, p204 | Common header only; explicit stub |
| Channel estimate | Table 63, p168–169, D.2.6.12 | Table 96, p207–208, D.2.6.12 | Signed complex s2.13; both components 0x8000 indicate exclusion |
| Constellation | Table 65, p171–172, D.2.7.7 | Table 98, p210–211, D.2.7.7 | Signed complex s2.13 |
| Histogram | Table 74, p185–186, D.2.12.5 | Table 107, p224–225, D.2.12.5 | u32 counts; each array preceded by byte length |
| Last upstream pre-EQ | Table 77, p189–190, D.2.13.13 | Table 110, p229, D.2.13.13 | Signed complex s1.14 |
| Current upstream pre-EQ | Table 78, p191–192, D.2.13.14 | Table 111, p230–231, D.2.13.14 | Signed complex s2.13 |
| FEC | Table 71, p181; D.2.1.6 | Table 104, p220; D.2.1.6 | Profile ID/count followed by 16-byte timestamp/counter records |
| Spectrum | Table 58, p160–161; D.2.1.4 | Table 91, p200–201; D.2.1.4 | Signed i16 hundredths dB referenced to 0 dBmV |
| Modulation | Table 80, p194; D.2.1.7, Tables 53/54 | Table 113, p233; D.2.1.7, Tables 86/87 | Length-delimited profiles with range or alternating skip assignments |
| Latency | Tables 83/84, p197–199; D.3.3 hex example p200 | Tables 116/117, p236–238; D.3.3 | MAC and flows; 16-byte label; u16 edges; u64 bin counts; u32 interval counters |

Check the section/table identifiers in the full extracts for the authoritative
field sequences. Byte order for implemented PyPNM formats is corroborated by
explicit `>`/`!` struct formats in these pinned implementation files under
`src/pypnm/pnm/parser/`:
`CmDsOfdmChanEstimateCoef.py`, `CmDsConstDispMeas.py`, `CmDsHist.py`,
`CmUsOfdmaPreEq.py`, `CmDsOfdmFecSummary.py`, `CmSpectrumAnalysis.py`,
`CmDsOfdmModulationProfile.py`, and `pnm_header.py`.
This is implementation corroboration, not a claim of a general normative
network-byte-order sentence. Signed complex representation and ordering are
specified in D.2.1.2 in both issues. Latency uses the big-endian hex example as
byte-order evidence and the explicit eight-byte bin widths from Tables 84/117;
the example's abbreviated prose does not override those table widths.

Header policy: standard PNN formats have a ten-byte common header. FEC and LLD
have six bytes without a capture timestamp, as their specific format tables
show. All supported routes are restricted to 1.0. Reserved PNM and internal PXX
prefixes are not aliases for PNN. Symbol payloads remain unimplemented.

Validation policy is deliberately stricter than PyPNM paths that silently shorten
arrays or ignore declared lengths: all declared bytes/records must be present,
with no trailing bytes. Application budgets are independent of protocol maxima.
This work validates structural decoding, not capture quality, PHY ranges or all
normative device generation requirements. No statistics are computed.

Verification: nine real fixtures export successfully. All values in the eight
additional fixtures were compared against an independent Python standard-library
`struct` decoder, without importing/running PyPNM. Synthetic tests cover signed
extremes, exclusions, skip assignments, malformed lengths, budgets, latency
u64 counts and every truncated prefix of the synthetic latency capture.
