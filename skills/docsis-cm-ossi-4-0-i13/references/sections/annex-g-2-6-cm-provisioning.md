# Annex G.2.6 - CM Provisioning Objects

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 345-349
Classification: Normative (explicit)

<!-- PDF_PAGE: 345 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


G.2.5.2.6 NumPackets
This attribute counts the total number of data packets of a DSID context forwarded for further processing.

           CmDsidClient
This object contains the client MAC addresses that the CMTS requests that the CM uses to replicate Multicast
DSIDs during registration or during a DBC transaction.
When a DSID is created that includes client MAC addresses, or when client MAC addresses are added to a DSID,
new rows are created to indicate the added client MAC addresses. When a Client MAC address is deleted from a
DSID, the corresponding row is deleted. When a DSID is deleted, all corresponding rows are deleted, too.
References: [MULPIv4.0] DSID Encodings section in the Common Radio Frequency Interface Encodings Annex
                                   Table 186 - CmDsidClient Object Attributes

                                                                       Type
        Attribute Name                   Type          Access        Constraints           Units             Default
Dsid                              Dsid               Key
MacId                             UnsignedShort      Key           1..65535
MacAddr                           MacAddress         R/O



G.2.5.3.1 Dsid
This key defines the DSID with which the client MAC addresses are associated.

G.2.5.3.2 MacId
This key defines a uniquely identified client MAC address associated with the DSID.

G.2.5.3.3 MacAddr
This attribute defines a client MAC address to which multicast traffic labeled with this DSID should be forwarded.

G.2.6     CM Provisioning Objects




                                 Figure 21 - CM Configuration Information Model




04/29/26                                           CableLabs                                                     345
<!-- PDF_PAGE: 346 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


           CmMdCfg
The CmMdCfg object contains MAC domain level control and configuration attributes for the CM.
References: [MULPIv4.0], IP Provisioning Mode Override section
                                      Table 187 - CmMdCfg Object Attributes

              Attribute Name                   Type         Access        Type Constraints         Units        Default
 IfIndex                                   InterfaceIndex   Key        InterfaceIndex of MAC
                                                                       Domain interface
 IpProvMode                                Enum             R/W        ipv4Only(0),                         honorMdd
                                                                       ipv6Only(1),
                                                                       honorMdd(4)
 IpProvModeResetOnChange                   TruthValue       R/W        true(1),                             false
                                                                       false(2)
 IpProvModeResetOnChangeHoldOffTimer       UnsignedInt      R/W        0...300                    seconds   0
 IpProvModeStorageType                     StorageType      R/W        volatile(2),                         nonVolatile
                                                                       nonVolatile(3)



G.2.6.1.1 IfIndex
This key represents the interface index of the MAC Domain to which this instance applies.

G.2.6.1.2 IpProvMode
This attribute specifies whether the CM honors or ignores the CMTS MDD TLV 5.1 setting in order to configure its
IP provisioning mode. The CM relies upon the CMTS to facilitate the successful IP address acquisition
independently of the MDD.
When this attribute is set to 'ipv4Only' the CM will initiate the acquisition of a single IPv4 address for the CM
management stack.
When this attribute is set to 'ipv6Only' the CM will initiate the acquisition of a single IPv6 address for the CM
management stack.
When this attribute is set to 'honorMdd', the CM will initiate the acquisition of an IP address as directed by the MDD
message sent by the CMTS.
References: [MULPIv4.0] IP Initialization Parameters TLV section

G.2.6.1.3 IpProvModeResetOnChange
This attribute determines whether the CM is to automatically reset upon a change to the IpProvMode attribute. The
IpProvModeResetOnChange attribute has a default value of 'false' which means that the CM does not reset upon
change to IpProvMode attribute. When this attribute is set to 'true', the CM resets upon a change to the IpProvMode
attribute.
References: [MULPIv4.0] IP Initialization Parameters TLV section

G.2.6.1.4 IpProvModeResetOnChangeHoldOffTimer
This attribute determines how long a CM with IpProvModeResetOnChange set to 'true' waits to reset. When the
IpProvModeResetOnChange attribute is set to 'true', the CM will decrement from the configured timer value before
resetting. The default value of the IpProvModeResetOnChangeHoldOffTimer is 0 seconds which is equivalent to an
immediate reset.
References: [MULPIv4.0] IP Initialization Parameters TLV section




346                                                   CableLabs                                                04/29/26
<!-- PDF_PAGE: 347 -->
Cable Modem Operations Support System Interface Specification                  CM-SP-CM-OSSIv4.0-I13-260429


G.2.6.1.5 IpProvModeStorageType
This attribute determines if the CM persists the value of IpProvMode across a single reset or across all resets. The
default value of IpProvModeStorageType is 'nonVolatile' which means that the CM persists the value of IpProvMode
across all resets. The CM persists the value of IpProvMode across only a single reset when IpProvModeStorageType
is set to 'volatile'.
References: [MULPIv4.0] IP Initialization Parameters TLV section

              CmEnergyMgt1x1Cfg
The CmEnergyMgt1x1Cfg object provides configuration state information on the CM for the Energy Management
1x1 Mode feature.
The values of these attributes are not persisted across reinitialization.
References: [MULPIv4.0] Energy Management 1x1 Mode Encodings section
                                 Table 188 - CmEnergyMgt1x1Cfg Object Attributes

     Attribute Name               Type             Access         Type Constraints      Units         Default
  Direction                 IfDirection         Key
  EntryBitrateThrshld       UnsignedInt         R/W                                  bps          Vendor-specific
  EntryTimeThrshld          UnsignedShort       R/W               1..65535           seconds      Vendor-specific
  ExitBitrateThrshld        UnsignedInt         R/W                                  bps          Vendor-specific
  ExitTimeThrshld           UnsignedShort       R/W               1..65535           seconds      Vendor-specific



G.2.6.2.1 Direction
This key attribute indicates whether the threshold applies to the upstream or downstream.

G.2.6.2.2 EntryBitrateThrshld
This attribute specifies the upstream or downstream bitrate threshold (in bps) below which the CM will request to
enter Energy Management 1x1 Mode operation.

G.2.6.2.3 EntryTimeThrshld
This attribute specifies the number of consecutive seconds that the upstream or downstream data rate needs to
remain below the Upstream or Downstream Entry Bitrate Threshold in order to determine that a transition to Energy
Management 1x1 Mode is required.

G.2.6.2.4 ExitBitrateThrshld
This attribute specifies the upstream or downstream bitrate threshold (in bps) above which the CM will request to
leave Energy Management 1x1 Mode operation.

G.2.6.2.5 ExitTimeThrshld
This attribute specifies the number of consecutive seconds that the upstream or downstream data rate needs to
remain above the Upstream or Downstream Exit Bitrate Threshold in order to determine that a transition out of
Energy Management 1x1 Mode is required.

              CmEmDlsCfg
The CmEmDlsCfg object provides configuration state information on the CM for the Energy Management DLS
Mode feature.
The values of these attributes are not persisted across reinitialization.
References: [MULPIv4.0] Energy Management DLS Mode Encodings section


04/29/26                                              CableLabs                                                347
<!-- PDF_PAGE: 348 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


                                    Table 189 - CmEmDlsCfg Object Attributes

      Attribute Name            Type               Access          Type Constraints        Units            Default
  Direction               IfDirection           Key
  EntryBitrateThrshld     UnsignedInt           R/W                                     bps             Vendor-specific
  EntryTimeThrshld        UnsignedShort         R/W               1..65535              seconds         Vendor-specific
  ExitBitrateThrshld      UnsignedInt           R/W                                     bps             Vendor-specific
  ExitTimeThrshld         UnsignedShort         R/W               1..65535              seconds         Vendor-specific



G.2.6.3.1 Direction
This key attribute indicates whether the threshold applies to the upstream or downstream.

G.2.6.3.2 EntryBitrateThrshld
This attribute specifies the upstream or downstream bitrate threshold (in bps) below which the CM will request to
enter Energy Management DLS Mode operation.

G.2.6.3.3 EntryTimeThrshld
This attribute specifies the number of consecutive seconds that the upstream or downstream data rate needs to
remain below the Upstream or Downstream Entry Bitrate Threshold in order to determine that a transition to Energy
Management DLS Mode is required.

G.2.6.3.4 ExitBitrateThrshld
This attribute specifies the upstream or downstream bitrate threshold (in bps) above which the CM will request to
leave Energy Management DLS Mode operation.

G.2.6.3.5 ExitTimeThrshld
This attribute specifies the number of consecutive seconds that the upstream or downstream data rate needs to
remain above the Upstream or Downstream Exit Bitrate Threshold in order to determine that a transition out of
Energy Management DLS Mode is required.

              CmMac
The CmMac object contains attributes of each CM MAC interface.
References: [RFC 4546] docsIfCmMacTable
                                         Table 190 - CmMac Object Attributes

   Attribute Name           Type          Access           Type Constraints                   Units             Default
 IfIndex                InterfaceIndex    Key         InterfaceIndex of MAC Domain
                                                      interface
 CmtsAddr               MacAddress        R/O
 Capabilities           EnumBits          R/O         atmCells(0),                                            false
                                                      concatenation(1)
 RangingRespTimeout     TimeTicks         R/W                                                                 20
 RangingTimeout         TimeInterval      R/W                                        HundredOfSeconds         20



G.2.6.4.1 IfIndex
This key attribute represents the interface index of the MAC Domain to which this instance applies.




348                                                   CableLabs                                                   04/29/26
<!-- PDF_PAGE: 349 -->
Cable Modem Operations Support System Interface Specification                   CM-SP-CM-OSSIv4.0-I13-260429


G.2.6.4.2 CmtsAddr
This attribute identifies the CMTS that is believed to control this MAC domain. At the CM, this will be the source
address from SYNC, MAP, and other MAC-layer messages. If the CMTS is unknown, returns 00-00-00-00-00-00.

G.2.6.4.3 Capabilities
This attribute identifies the capabilities of the MAC implementation at this interface. Note that packet transmission
is always supported. Therefore, there is no specific bit required to explicitly indicate this capability.

G.2.6.4.4 RangingRespTimeout
This attribute identifies the waiting time for a Ranging Response packet. This attribute has been obsoleted and
replaced by RangingTimeout to correct the typing to TimeInterval.

G.2.6.4.5 RangingTimeout
This attribute identifies the waiting time for a Ranging Response packet. This attribute replaces the obsoleted
RangingRespTimeout attribute.




04/29/26                                            CableLabs                                                    349
