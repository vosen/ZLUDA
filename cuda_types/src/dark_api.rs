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
    pub const MAGIC: c_uint = 0x466243B1;
    const VERSION_V1: c_uint = 0x1;
    pub const VERSION_V2: c_uint = 0x2;

    pub fn new<'a, T: Sized>(ptr: &*const T) -> Result<&'a Self, ParseError> {
        unsafe { ptr.cast::<Self>().as_ref() }
            .ok_or(ParseError::NullPointer("FatbincWrapper"))
            .and_then(|ptr| {
                ParseError::check_fields("FATBINC_MAGIC", ptr.magic, [Self::MAGIC])?;
                ParseError::check_fields(
                    "FATBINC_VERSION",
                    ptr.version,
                    [Self::VERSION_V1, Self::VERSION_V2],
                )?;
                Ok(ptr)
            })
    }
}

impl FatbinHeader {
    const MAGIC: c_uint = 0xBA55ED50;
    const VERSION: c_ushort = 0x01;

    pub fn new<'a, T: Sized>(ptr: &'a *const T) -> Result<&'a Self, ParseError> {
        unsafe { ptr.cast::<Self>().as_ref() }
            .ok_or(ParseError::NullPointer("FatbinHeader"))
            .and_then(|ptr| {
                ParseError::check_fields("FATBIN_MAGIC", ptr.magic, [Self::MAGIC])?;
                ParseError::check_fields("FATBIN_VERSION", ptr.version, [Self::VERSION])?;
                Ok(ptr)
            })
    }

    pub unsafe fn get_content<'a>(&'a self) -> &'a [u8] {
        let start = std::ptr::from_ref(self)
            .cast::<u8>()
            .add(self.header_size as usize);
        std::slice::from_raw_parts(start, self.files_size as usize)
    }
}

impl FatbinFileHeader {
    pub const HEADER_KIND_PTX: c_ushort = 0x01;
    pub const HEADER_KIND_ELF: c_ushort = 0x02;
    const HEADER_VERSION_CURRENT: c_ushort = 0x101;

    pub fn new_ptx<T: Sized>(ptr: *const T) -> Result<Option<&'static Self>, ParseError> {
        unsafe { ptr.cast::<Self>().as_ref() }
            .ok_or(ParseError::NullPointer("FatbinFileHeader"))
            .and_then(|ptr| {
                ParseError::check_fields(
                    "FATBIN_FILE_HEADER_VERSION_CURRENT",
                    ptr.version,
                    [Self::HEADER_VERSION_CURRENT],
                )?;
                match ptr.kind {
                    Self::HEADER_KIND_PTX => Ok(Some(ptr)),
                    Self::HEADER_KIND_ELF => Ok(None),
                    _ => Err(ParseError::UnexpectedBinaryField {
                        field_name: "FATBIN_FILE_HEADER_KIND",
                        observed: ptr.kind.into(),
                        expected: vec![Self::HEADER_KIND_PTX.into(), Self::HEADER_KIND_ELF.into()],
                    }),
                }
            })
    }

    pub unsafe fn next<'a>(slice: &'a mut &[u8]) -> Result<Option<&'a Self>, ParseError> {
        if slice.len() < std::mem::size_of::<Self>() {
            return Ok(None);
        }
        let this = &*slice.as_ptr().cast::<Self>();
        let next_element = slice
            .split_at_checked(this.header_size as usize + this.padded_payload_size as usize)
            .map(|(_, next)| next);
        *slice = next_element.unwrap_or(&[]);
        ParseError::check_fields(
            "FATBIN_FILE_HEADER_VERSION_CURRENT",
            this.version,
            [Self::HEADER_VERSION_CURRENT],
        )?;
        Ok(Some(this))
    }

    pub unsafe fn get_payload<'a>(&'a self) -> &'a [u8] {
        let start = std::ptr::from_ref(self)
            .cast::<u8>()
            .add(self.header_size as usize);
        std::slice::from_raw_parts(start, self.payload_size as usize)
    }

    pub unsafe fn decompress<'a>(&'a self) -> Result<Vec<u8>, ParseError> {
        let payload = if self.flags.contains(FatbinFileHeaderFlags::CompressedLz4) {
            unsafe { decompress_lz4(self) }?
        } else if self.flags.contains(FatbinFileHeaderFlags::CompressedZstd) {
            unsafe { decompress_zstd(self) }?
        } else {
            unsafe { self.get_payload().to_vec() }
        };

        Ok(payload)
    }
}

pub enum ParseError {
    NullPointer(&'static str),
    UnexpectedBinaryField {
        field_name: &'static str,
        observed: u32,
        expected: Vec<u32>,
    },
    Lz4DecompressionFailure,
    ZstdDecompressionFailure(usize),
}

impl ParseError {
    pub(crate) fn check_fields<const N: usize, T: Into<u32> + Eq + Copy>(
        name: &'static str,
        observed: T,
        expected: [T; N],
    ) -> Result<(), Self> {
        if expected.contains(&observed) {
            Ok(())
        } else {
            let observed = observed.into();
            let expected = expected.into_iter().map(Into::into).collect();
            Err(ParseError::UnexpectedBinaryField {
                field_name: name,
                expected,
                observed,
            })
        }
    }
}

const MAX_MODULE_DECOMPRESSION_BOUND: usize = 64 * 1024 * 1024;

pub unsafe fn decompress_lz4(file: &FatbinFileHeader) -> Result<Vec<u8>, ParseError> {
    let decompressed_size = usize::max(1024, (*file).uncompressed_payload as usize);
    let mut decompressed_vec = vec![0u8; decompressed_size];
    loop {
        match lz4_sys::LZ4_decompress_safe(
            file.get_payload().as_ptr() as *const _,
            decompressed_vec.as_mut_ptr() as *mut _,
            (*file).payload_size as _,
            decompressed_vec.len() as _,
        ) {
            error if error < 0 => {
                let new_size = decompressed_vec.len() * 2;
                if new_size > MAX_MODULE_DECOMPRESSION_BOUND {
                    return Err(ParseError::Lz4DecompressionFailure);
                }
                decompressed_vec.resize(decompressed_vec.len() * 2, 0);
            }
            real_decompressed_size => {
                decompressed_vec.truncate(real_decompressed_size as usize);
                return Ok(decompressed_vec);
            }
        }
    }
}

pub unsafe fn decompress_zstd(file: &FatbinFileHeader) -> Result<Vec<u8>, ParseError> {
    let mut result = Vec::with_capacity(file.uncompressed_payload as usize);
    let payload = file.get_payload();
    dbg!((payload.len(), file.uncompressed_payload, file.payload_size));
    match zstd_safe::decompress(&mut result, payload) {
        Ok(actual_size) => {
            result.truncate(actual_size);
            Ok(result)
        }
        Err(err) => Err(ParseError::ZstdDecompressionFailure(err)),
    }
}
