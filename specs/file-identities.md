# Central file identity evidence

The registry in `pnm_file_type.rs` recognizes the following raw signatures.
Names and display tags follow PyPNM pnm/parser/pnm_file_type.py at commit
82bbf79fe04a0b5e766dc3eb4d5ca2004008bca9. Raw values appear in file-format tables
in both versioned extracts; this is identity evidence, not decoder support.

| Tag | Raw signature | Meaning |
| --- | --- | --- |
| PNN1 | 504e4e01 | Symbol capture |
| PNN2 | 504e4e02 | Channel estimate |
| PNN3 | 504e4e03 | Constellation display |
| PNN4 | 504e4e04 | RxMER |
| PNN5 | 504e4e05 | Histogram |
| PNN6 | 504e4e06 | Upstream pre-equalization |
| PNN7 | 504e4e07 | Last pre-equalization update |
| PNN8 | 504e4e08 | FEC Summary |
| PNN9 | 504e4e09 | Spectrum analysis |
| PNN10 | 504e4e0a | Modulation profile |
| LLD01 | 4c4c4401 | Latency summary |

Lookup sources: CM-SP-CM-OSSIv3.1-I27-250219 and
CM-SP-CM-OSSIv4.0-I13-260429, Annex D PNM measurement file-format tables.
The exact signature strings locate each row in the versioned full extracts under
skills/docsis-cm-ossi-*/references/cm-ossi-full.txt. RxMER header uses I27 Table 68 /
I13 Table 101; FEC Summary uses I27 Table 71 / I13 Table 104. Latency signature is
I27 Table 83 / I13 Table 116. All listed version-1.0 headers are enabled; symbol payload decoding stays unsupported.

PNN6 and PNN7 share the upstream pre-equalization route, matching PyPNM's
pnm_parameter.py. Their distinct identities select s2.13 and s1.14 respectively. Header support remains
explicit per identity/version. No fallback assumes every PNN type has capture time.

Text tags normalize case, whitespace and numeric padding; binary signatures are
matched exactly. PXX9 (PyPNM internal SNMP spectrum representation) is excluded.
No PDF rendering or blanket DOCSIS compliance is claimed.
