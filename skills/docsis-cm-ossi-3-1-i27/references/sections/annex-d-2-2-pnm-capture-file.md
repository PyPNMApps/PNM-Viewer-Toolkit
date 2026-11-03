# Annex D.2.2 - PnmCaptureFile

Source: CM-SP-CM-OSSIv3.1-I27-250219
PDF pages: 155-155
Classification: Normative (explicit)

<!-- PDF_PAGE: 155 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv3.1-I27-250219


D.2.1.7.3.2 Subcarrier Skip Modulation Scheme
This attribute indicates whether the skip modulation method is used. If true, the modulation order of the subcarriers
in the range is alternating between the Main Modulation and Skip Modulation.

D.2.1.7.3.2.1 Main Modulation Order - 1 Byte
This attribute indicates the main modulation order 2x. The main modulation is the modulation order of the first, the
third, the fifth, etc., subcarriers in the range.

D.2.1.7.3.2.2 Skip Modulation Order - 1 Byte
This attribute indicates the modulation order 2x for every other subcarrier in the range.

D.2.1.7.3.2.3 Skip Modulation Number of Subcarriers - 2 Bytes
The total number of subcarriers including both the Main Modulation and Skip Modulation.
                                    Table 54 - Subcarrier Skip Modulation Scheme

                                               Element                           Size
                                  Subcarrier Assignment Scheme Type    1 byte (Value 0x01)
                                  Main Modulation Order                1 byte
                                  Skip Modulation Order                1 byte
                                  Skip Modulation Number of            2 bytes
                                  Subcarriers



D.2.2    PnmCaptureFile
PNM tests usually generate files to report measurements or test results. The results file includes header information
that is common to all types of PNM tests and fields, and data that are specific to the type of PNM test. The abstract
PnmCaptureFile object defines the attributes and format of the header information common to all PNM test files
described below. The File header fields are right-justified within the field and left-padded with zero values if
necessary.
Determination of maximum size of the PNM Capture File is left to vendor implementation.
Table 55 shows the format of the PNM capture file header, applied to any data file created by a PNM test.
                                     Table 55 - PnmCaptureFile Object Attributes

        Attribute         Type              Access             Type Constraints               Units   Default Value
         Name

    FileType          HexBinary          R/O                SIZE (4)                    N/A           N/A
    MajorVersion      UnsignedByte       R/O                SIZE (1)                    N/A           N/A
    MinorVersion      UnsignedByte       R/O                SIZE (1)                    N/A           N/A
    CaptureTime       UnsignedInt        R/O                SIZE (4)                    N/A           N/A



           FileType
A four-byte hexadecimal identifier specific to the type of PNM test that generated the data file. Each FileType is
registered in [CANN]. Filetypes prefixed by PNM (i.e., 504E4D) do not contain the MajorVersion and
MinorVersion header elements. Filetypes prefixed with PNN (i.e., 504E4E) contain the MajorVersion and
MinorVersion header elements.




02/19/25                                                  CableLabs®                                                 155
