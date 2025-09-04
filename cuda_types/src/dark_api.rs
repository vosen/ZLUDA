use bitflags::bitflags;
use std::ffi::{c_uint, c_ulonglong, c_ushort, c_void};

/*
fat_cubin:
typedef struct {
  int magic;
  int version;
  const unsigned long long* data;
  void *filename_or_fatbins;  /* version 1: offline filename,
                               * version 2: array of prelinked fatbins */
} __fatBinC_Wrapper_t;

data start with this header:
#define FATBIN_MAGIC 0xBA55ED50U
#define OLD_STYLE_FATBIN_MAGIC 0x1EE55A01U
#define FATBIN_VERSION 0x0001U

struct fatbinary_ALIGN_(8) fatBinaryHeader
{
  unsigned int           magic;   // FATBIN_MAGIC
  unsigned short         version; // FATBIN_VERSION
  unsigned short         headerSize;
  unsigned long long int fatSize; // size of the entire fat binary excluding this header
};

there's binary data after header

*/

#[repr(C)]
pub struct FatbincWrapper {
    pub magic: c_uint,
    pub version: c_uint,
    pub data: *const FatbinHeader,
    pub filename_or_fatbins: *const c_void,
}

#[repr(C, align(8))]
pub struct FatbinHeader {
    pub magic: c_uint,
    pub version: c_ushort,
    pub header_size: c_ushort,
    pub files_size: c_ulonglong, // excluding frame header, size of all blocks framed by this frame
}

#[repr(C)]
pub struct FatbinFileHeader {
    pub kind: c_ushort,
    pub version: c_ushort,
    pub header_size: c_uint,
    pub padded_payload_size: c_uint,
    pub unknown0: c_uint, // check if it's written into separately
    pub payload_size: c_uint,
    pub unknown1: c_uint,
    pub unknown2: c_uint,
    pub sm_version: c_uint,
    pub bit_width: c_uint,
    pub unknown3: c_uint,
    pub flags: FatbinFileHeaderFlags,
    pub unknown5: c_ulonglong,
    pub uncompressed_payload: c_ulonglong,
}

bitflags! {
    pub struct FatbinFileHeaderFlags: u64 {
        const Is64Bit = 0x0000000000000001;
        const Debug = 0x0000000000000002;
        const Linux = 0x0000000000000010;
        const Mac = 0x0000000000000020;
        const Windows = 0x0000000000000040;
        const CompressedLz4 = 0x0000000000002000;
        const CompressedZstd = 0x0000000000008000;

        const _ = !0;
    }
}

impl FatbincWrapper {
    pub const MAGIC: [u8; 4] = [0x46, 0x62, 0x43, 0xB1];
    pub const VERSION_V1: c_uint = 0x1;
    pub const VERSION_V2: c_uint = 0x2;
}

impl FatbinHeader {
    pub const MAGIC: [u8; 4] = [0xBA, 0x55, 0xED, 0x50];
    pub const VERSION: c_ushort = 0x01;
}

impl FatbinFileHeader {
    pub const HEADER_KIND_PTX: c_ushort = 0x01;
    pub const HEADER_KIND_ELF: c_ushort = 0x02;
    pub const HEADER_VERSION_CURRENT: c_ushort = 0x101;
}
