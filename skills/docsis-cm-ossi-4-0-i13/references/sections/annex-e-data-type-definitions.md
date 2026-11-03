# Annex E - DOCSIS 4.0 Data Type Definitions

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 248-251
Classification: Normative (explicit)

<!-- PDF_PAGE: 248 -->
CM-SP-CM-OSSIv4.0-I13-260429                                  Data-Over-Cable Service Interface Specifications


Annex E            DOCSIS 4.0 Data Type Definitions (Normative)
E.1 Overview
This specification has requirements for the SNMP protocol for network management functions.
This Annex includes the data type definitions for the information models defined for use in DOCSIS 4.0. The
Unified Modeling Language (UML) is used for modeling the management requirements in DOCSIS 4.0. The data
types defined in this Annex are mapped for use with SNMP.
Basic UML notation used in this specification and explained in [UML Guidelines].

E.2 Data Type Mapping
XML is becoming the standard for data definition models. With XML data transformations can be done with or
without a model (DTD or Schema definition). DTDs and XML schemas provides additional data validation layer to
the applications exchanging XML data. There are several models to map formal notation constructs like ASN.1 to
XML [ITU-TX.692], UML to XML, or XML by itself can be used for modeling purposes.
Each area of data information interest approaches XML and defines data models and/or data containment structures
and data types. Similarly, SNMP took and modified a subset of ASN.1 for defining the Structured Management
Information SMIv1 and SMIv2.
Due to the lack of a unified data model and data types for Network Management a neutral model would be
appropriated to allow capturing specific requirements and methodologies from existing protocols and allow forward
or reverse engineering of those standards like SNMP to the general information model and vice versa.

E.2.1      Data Type Requirements and Classification
The information model has to provide seamless translation for SMIv2 requirements, in particular when creating MIB
modules based on the information model, this specification needs to provide full support of [RFC 2578],
[RFC 2579] and the clarifications and recommendations of [RFC 4181].
Thus, there are two data type groups defined for modeling purposes and mapping to protocol data notation
roundtrip:
      1.   General Data types
           Required data types to cover all the management syntax and semantic requirement for all OSSI supported
           data models. In this category are data types defined in SNMP SMIv2 [RFC 2578], [IPDR/XDR], and
           [IPDR/SSDG].
      2.   Extended Data types
           Management protocols specialization based on frequent usage or special semantics. Required data types to
           cover all the syntax requirement for all OSSI supported data models. In this category are SNMP
           TEXTUAL-CONVENTION clauses [RFC 2579] of mandatory or recommended usage by [RFC 2579] and
           [RFC 4181] when modeling for SNMP MIB modules.

E.2.2      Data Type Mapping Methodology
The specification "XML Schema Part 2: Data types Second Edition" is based on [ISO11404] which provides a
language-independent data types (see XML Schema reference). The mapping proposed below uses a subset of the
XML schema data types to cover both SNMP forward and reverse engineering. Any additional protocol being added
should be feasible to provide the particular mappings.
SMIv2 has an extensive experience of data types for management purposes, for illustration consider Counter32 and
Counter64 SMIv2 types [RFC 2578]. The XML schema data types makes no distinction of derived 'decimal' types
and the semantics that are associated to counters, e.g., counters do not necessarily start at 0.
Since the information model needs to cover the mapping of objects to SNMP, the mapping in Section E.2.4 is
heavily based on most common SNMP TEXTUAL-CONVENTION descriptors [RFC 2579] and others IETF



248                                                 CableLabs                                             04/29/26
<!-- PDF_PAGE: 249 -->
Cable Modem Operations Support System Interface Specification                    CM-SP-CM-OSSIv4.0-I13-260429


commonly used type definitions as well as DOCSIS already defined types in MIB modules required by this
specification.
Most of the SNMP information associated to data types are reduced to size and range constraints and specialized
enumerations.

E.2.3   General Data Types
Table 120 represents the mapping between the OSSI information model General Types and their equivalent
representation for SNMP MIB Modules. The permitted values for the data types are indicated in terms of value
ranges and string length when applicable. The IM Data Type column includes the data types to map to SNMP, using
the appropriate type in the corresponding protocol if applicable or available. The SNMP Mapping references to
SNMP data types are defined in [RFC 2578] or as described below.
Note that SNMP does not provide float, double or long XML-Schema data types. Also, SNMP might map a type to a
SNMP subtyped value. For example, UnsignedByte data type maps to Unsigned32 subtyped to the appropriate range
indicated by the Permitted Values (0..255 in this case). Other data types are mapped to SNMP TEXTUAL-
CONVENTIONS as indicated by the references.
                                             Table 120 - General Data Types

                             XML-Schema
           IM Data Type       Data Type              Permitted Values                SNMP Mapping
           Boolean           Boolean           true = 1                       TruthValue [RFC 2579]
                                               false = 0
           Counter32         unsignedInt                                      Counter32
           Counter64         unsignedLong                                     Counter64
           DateTime          dateTime                                         DateAndTime
           Enum              int               -2147483648..2147483647        INTEGER
           EnumBits          hexBinary                                        BITS
           HexBinary         hexBinary                                        OCTET STRING
           InetAddress                                                        InetAddress [RFC 4001]
           (Deprecated)
           InetAddressType                                                    InetAddressType [RFC 4001]
           (Deprecated)
           Int               int               -2147483648..2147483647        Integer32
           IpAddress                           IPv4 Address or IPv6 Address   InetAddress + InetAddressType
                                                                              [RFC 4001]
           MacAddress        hexBinary         SIZE (6)                       MacAddress
           String            string                                           SnmpAdminString [RFC 3411]
           UnsignedByte      unsignedByte      0..255                         Unsigned32
           UnsignedInt       unsignedInt       0..4294967295                  Unsigned32
           UnsignedLong      unsignedLong      0..18446744073709551615        CounterBasedGauge64
                                                                              [RFC 2856]
           UnsignedShort     unsignedShort     0..65535                       Unsigned32



E.2.4   Extended Data Types
There are two sources of Extended Data Types: Protocol specific data types, and OSSI data types.
SNMP derived types are defined in SNMP MIB Modules. The most important are in [RFC 2579] which is part of
SNMP STD 58 and are considered in many aspects part of the SNMP protocol. Other MIB modules TEXTUAL-
CONVENTION definitions have been adopted and recommended (e.g., [RFC 4181]) for re-usability and semantics
considerations in order to unify management concepts; some relevant RFCs that include common used textual
conventions are [RFC 4001], [RFC 2863], [RFC 3411], and [RFC 3419] among others (see [RFC 4181]).



04/29/26                                                CableLabs                                                249
<!-- PDF_PAGE: 250 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


Table 121 includes the most relevant data types taken from SNMP to provide a direct mapping of the OSSI
information model to SNMP MIB modules. A few have taken a more general name as they are used across the
information models. For example, AdminString comes from [RFC 3411] SnmpAdminString.
In general, when an OSSI information model needs to reference an existing SNMP textual convention for the
purpose of round-trip design from UML to SNMP, these textual conventions can be added to this list. Other sources
of textual conventions not listed here are from MIB modules specific to DOCSIS either as RFCs or Annex
documents in this specification.
OSSI data types are also defined in this specification in the Data Type section of OSSI annexes; for example, Annex
A and Annex G.
                                               Table 121 - Extended Data Types

                                                 XML-Schema      Permitted
                    IM Data Type                  Data Type       Values                     SNMP Mapping
      AdminString                                string         SIZE (0..255)    SnmpAdminString
      DocsEqualizerData                          hexBinary                       DocsEqualizerData [RFC 4546]
      DocsisUpstreamType                         int                             DocsisUpstreamType [RFC 4546]
      DocsX509ASN1DEREncodedCertificate          hexBinary      SIZE (0..4096)   DocsX509ASN1DEREncodedCertificate
                                                                                 [RFC 4131] [RFC 5280]
      Duration                                   unsignedInt    0..2147483647    TimeInterval
      InetPortNumber                             unsignedInt    0..65535         Unsigned32
      PhysAddress                                hexBinary                       PhysAddress
      RowStatus                                  int                             RowStatus
      StorageType                                int                             StorageType
      TAddress                                   hexBinary      SIZE (1..255)    TAddress
      TDomain                                    anyURI                          TDomain
      TenthdB                                    int                             TenthdB [RFC 4546]
      TenthdBmV                                  int                             TenthdBmV [RFC 4546]
      TimeStamp                                  unsignedInt                     TimeStamp



E.2.5     Common Terms Shortened
The following table lists common terms which have been shortened to allow shorter SNMP MIB names. These
shortened names are desired to be used consistently throughout the information models, SNMP MIBs, and IPDR
schemas. However, in some cases it might not be possible to maintain parity with pre-3.0 DOCSIS requirements.
                                         Table 122 - Shortened Common Terms

                                         Original Word                 Shortened Word
                                   Address                      Addr
                                   Aggregate                    Agg
                                   Algorithm                    Alg
                                   Application                  App
                                   Attribute                    Attr
                                   Authorization                Auth
                                   Channel                      Ch
                                   Command                      Cmd
                                   Config*                      Cfg
                                   Control                      Ctrl
                                   Default                      Def
                                   Destination                  Dest




250                                                       CableLabs                                             04/29/26
<!-- PDF_PAGE: 251 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


                                       Original Word                Shortened Word
                               Direction                      Dir
                               Downstream                     Ds
                               Encryption                     Encrypt
                               Equalization                   Eq
                               Frequency                      Freq
                               Group                          Grp
                               Length                         Len
                               Maximum                        Max
                               Minimum                        Min
                               Multicast                      Mcast
                               Provision*                     Prov
                               Receive                        Rx
                               Registration                   Reg
                               Replication                    Repl
                               Request                        Req
                               Resequence                     Reseq
                               Resequencing                   Reseq
                               Response                       Rsp
                               Segment                        Sgmt
                               Sequence                       Seq
                               Service                        Svc
                               ServiceFlow                    Sf
                               Session(s)                     Sess
                               Source                         Src
                               Threshold                      Thrshld
                               Total                          Tot
                               Transmit                       Tx
                               Upstream                       Us
                               * indicates a wildcard



           Exceptions
Data types and managed objects do not consistently use the shortened names. Also, the term ServiceFlowId remains
unchanged. Service and ServiceFlow are often not shortened to retain backward compatibility with QoS managed
objects.




04/29/26                                                CableLabs                                          251
