# Annex D.5 - Bulk Data Transfer

Source: CM-SP-CM-OSSIv4.0-I13-260429
PDF pages: 244-247
Classification: Normative (explicit)

<!-- PDF_PAGE: 244 -->
CM-SP-CM-OSSIv4.0-I13-260429                                     Data-Over-Cable Service Interface Specifications


D.5 Bulk Data Transfer
Proactive Network Maintenance, and potentially other applications, may generate data files that need to be
transferred to a server. The Bulk-Data Transfer mechanism defines file storage requirements, destination address
and a mechanism to initiate a transfer. The transfer of the bulk data file may be initiated automatically on the file
creation or on demand. This section defines the Bulk-Data capability requirements.

D.5.1    CM Bulk Data Transfer Requirements
The CM MUST act as a TFTP client and implement the TFTP protocol over UDP per [RFC 1350] to transfer Bulk-
Data files.
The CM MUST initiate the TFTP connection on the standard TFTP-assigned port (69).
The CM MUST use the 'octet' TFTP transfer mode to perform a TFTP 'write' to the specified address.
The CM MUST include the TFTP Blocksize option [RFC 2348] when establishing a TFTP connection.
The CM MUST request a blocksize of 1448 if using TFTP over IPv4. The CM MUST request a blocksize of 1428 if
using TFTP over IPv6.
The CM MUST change the value of the UploadStatus attribute in the CmBulkDataFile object to reflect the status of
the upload. There are no requirements for the CM to automatically retry the transfer.

D.5.2    CM Data-File and Storage Requirements
The CM MUST be able to store data files that contain at least 64 kilobytes of data. If the CM is commanded to
collect data that exceeds its data size capability, the CM MAY stop collecting data and report an error to the
application that commanded it to collect data. The application may further report the error if so defined.
The CM MUST be able to store a minimum of four data files.
If the CM is commanded to collect Bulk-Data and the memory allocated for the Bulk-Data file is full, the CM
MUST overwrite the oldest data file with the most recently collected data file. The Bulk-Data Transfer mechanism
has no notion of file types; hence, it will always replace the oldest file regardless of the type of data contained in the
file.
If the oldest data file is currently being uploaded, and this file would need to be deleted to make space for a new data
file (i.e., the CM has already stored its maximum limit of files), the CM MUST set the value of the
'PnmCmCtlStatus' object to a value of 'tempReject(4)' while the TFTP Upload is in progress. This serves to limit a
new test from generating a data file that would overwrite a data file in use.
The CM MUST retain Bulk Data files in the allocated memory unless it is commanded to delete the file, or the file is
overwritten with a new file. The CM MAY retain the Bulk Data files across reboot or reset or power cycle.




                                  Figure 13 - Bulk Data Upload Information Model




244                                                  CableLabs                                                 04/29/26
<!-- PDF_PAGE: 245 -->
Cable Modem Operations Support System Interface Specification                       CM-SP-CM-OSSIv4.0-I13-260429


D.5.3     Bulk Data Objects
This section defines objects that are used to manage the Bulk-Data files that the CM (referred to here as "the
device") has been commanded to capture.

            CmBulkDataControl
This object provides the configuration attributes needed for the device to upload Bulk Data files to a server.
                                 Table 118 - CmBulkDataControl Object Attributes

        Attribute Name                 Type               Access      Type Constraints        Units      Default Value
  DestIpAddr                 InetAddress [RFC 4001]      R/W                                            "" (empty string)
  DestIpAddrType             InetAddressType             R/W          unknown(0),                       unknown
                             [RFC 4001]                               ipv4(1),
                                                                      ipv6(2)
  DestPath                   AdminString                 R/W                                            "" (empty string)
  UploadControl              Enum                        R/W          other(1),                         autoUpload(3)
                                                                      noAutoupload(2),
                                                                      autoUpload(3)



D.5.3.1.1 DestIpAddr
This attribute represents the IP address of the server to which the bulk data file is to be sent. This attribute is further
defined by the DestIpAddrType attribute. The CM MUST NOT allow the value of DestIpAddr to change if the value
of PnmCmCtlStatus is any value other than 'ready'.

D.5.3.1.2 DestIpAddrType
This attribute represents the IP address type of the DestIpAddr attribute. This value is of type InetAddressType,
which is defined by [RFC 4001].
A successful connection depends on the value of this attribute being set to an IP Family supported by the device. For
example, if this value is set to IPv6 and the device is operating in an IPv4-only mode, a successful upload will not be
possible. In this case the UploadStatus attribute in the BulkDataFile object would reflect the error. The CM MUST
NOT allow the value of DestIpAddrType to change if the value of PnmCmCtlStatus is any value other than 'ready'.

D.5.3.1.3 DestPath
This attribute represents the path, excluding the filename, at the server to which the bulk data file is to be sent. The
CM MUST NOT allow the value of DestPath to change if the value of PnmCmCtlStatus is any value other than
'ready'. By default, the value of this object is an empty string. If used, this value includes all expected delimiters.
The following examples, excluding the quotes, are valid values:
    "/Directory1/directory2/"
    "/pnm/"

D.5.3.1.4 UploadControl
This attribute controls the action taken by the device when a new bulk data file is generated. The possible values are
defined below.
    noAutoUpload(2) - Bulk Data files are not automatically uploaded by the device. All bulk data files are
    available to be uploaded, on demand, by manipulating the FileControl attribute in the BulkDataFile object for
    that file's row instance.
    autoUpload(3) - When the autoUpload option is selected, the CM MUST automatically upload bulk data files as
    they become available. A file becomes available when a file-generation application completes the file and
    creates a row in the BulkDataFileTable. If this value is set, the bulk data file is automatically uploaded to the
    parameters defined by the DestIpAddr, DestIpAddrType, and DestPath. If the upload fails or additional uploads


04/29/26                                              CableLabs                                                        245
<!-- PDF_PAGE: 246 -->
CM-SP-CM-OSSIv4.0-I13-260429                                      Data-Over-Cable Service Interface Specifications


      are desired, the file can be re-uploaded by manipulating the FileControl attribute in the BulkDataFile object for
      that file's row instance.

              CmBulkDataFile
This table provides the attributes needed for the device to upload a bulk data file to the Server. This object is a table
with a row for each file that is available in the device for upload. The parameters used for the upload are provided
under the CmBulkDataControl object.
The CM MUST create a row for each file that is available for upload. The device could have limited resources to
save captured data files. Therefore, if the number of files exceeds the minimum supported number of files
requirement for the device, newly-created rows can overwrite/replace existing rows as new data files become
available. If a bulk data file is no longer available for upload, the CM MUST remove that file's details from the
CmBulkDataFile table.
                                     Table 119 - CmBulkDataFile Object Attributes

        Attribute Name              Type            Access         Type Constraints          Units        Default Value
  FileIndex                    UnsignedByte      Key
  FileName                     AdminString       R/O
  FileControl                  Enum              R/W              other(1),                              other(1)
                                                                  tftpUpload(2),
                                                                  cancelUpload(3),
                                                                  deleteFile (4)
  FileUploadStatus             Enum              R/O              other(1),
                                                                  availableForUpload(2),
                                                                  uploadInProgress(3),
                                                                  uploadCompleted(4),
                                                                  uploadPending(5),
                                                                  uploadCancelled(6),
                                                                  error(7)



D.5.3.2.1 FileIndex
This attribute is the key for the table.

D.5.3.2.2 FileName
This attribute contains the name of the bulk data file, stored in the device, that is available to be uploaded to the
server. Filenames are defined by the application that creates them.

D.5.3.2.3 FileControl
This attribute controls the action taken by the device regarding the file specified by the FileName attribute. When a
value is written to this attribute for a given row instance, the device is required to take that action on the specified
bulk data file. The possible actions are:
      other(1)—This value is returned when the object is read. This value is not writeable.
      tftpUpload(2) -The CM MUST initiate a TFTP-Write to the server with the parameters specified in the
      'DestIpAddr', 'DestIpAddrType', and 'DestPath' attributes of the BulkDataControl object. This action will change
      the value of the UploadStatus attribute to 'uploadInProgress' while the transfer is ongoing. This object can only
      be set to 'tftpUpload' when the value of the 'UploadStatus' attribute is not set to a value of 'uploadInProgress' for
      this row OR for any row in the table. This limits the upload process to one upload at a time. This object will
      return 'inconsistentValue' for this case.
      cancelUpload(3) - The CM MUST cancel a pending upload or an upload currently in progress on this bulk data
      file. The value of the UploadStatus attribute will be changed to 'uploadCancelled'.



246                                                    CableLabs                                                   04/29/26
<!-- PDF_PAGE: 247 -->
Cable Modem Operations Support System Interface Specification                      CM-SP-CM-OSSIv4.0-I13-260429


    deleteFile(4) - The CM MUST delete the file from its memory and from this table. This object cannot be set to
    deleteFile(4) while an upload is in progress.

D.5.3.2.4 FileUploadStatus
This attribute reflects the status of the bulk data file. The possible values are listed below.
    other(1)—Any condition not covered by the other defined values.
    availableForUpload(2)—The file is available to be uploaded.
    uploadInProgress(3)—The file is currently being uploaded.
    uploadCompleted(4)—The file was successfully uploaded.
    uploadPending(5)—The file has been selected for upload, but a condition does not allow the upload to take
    place. The upload will start when the condition blocking uploads has been removed. For example, another
    upload that is currently in progress could cause this value to be returned.
    uploadCancelled(6)—An upload was cancelled before it completed.
    error(7)—An error occurred, and the file was not successfully uploaded.




04/29/26                                              CableLabs                                                247
