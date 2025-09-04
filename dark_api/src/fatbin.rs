// This file contains a higher-level interface for parsing fatbins

use std::ptr;

use cuda_types::dark_api::*;

pub enum ParseError {
    NullPointer(&'static str),
    UnexpectedBinaryField {
        field_name: &'static str,
        observed: u32,
        expected: Vec<u32>,
    },
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

pub enum FatbinError {
    ParseFailure(ParseError),
    Lz4DecompressionFailure,
    ZstdDecompressionFailure(usize),
}

pub fn parse_fatbinc_wrapper<T: Sized>(ptr: &*const T) -> Result<&FatbincWrapper, ParseError> {
    unsafe { ptr.cast::<FatbincWrapper>().as_ref() }
        .ok_or(ParseError::NullPointer("FatbincWrapper"))
        .and_then(|ptr| {
            ParseError::check_fields(
                "FATBINC_MAGIC",
                ptr.magic,
                [u32::from_ne_bytes(FatbincWrapper::MAGIC)],
            )?;
            ParseError::check_fields(
                "FATBINC_VERSION",
                ptr.version,
                [FatbincWrapper::VERSION_V1, FatbincWrapper::VERSION_V2],
            )?;
            Ok(ptr)
        })
}

fn parse_fatbin_header<T: Sized>(ptr: &*const T) -> Result<&FatbinHeader, ParseError> {
    unsafe { ptr.cast::<FatbinHeader>().as_ref() }
        .ok_or(ParseError::NullPointer("FatbinHeader"))
        .and_then(|ptr| {
            ParseError::check_fields(
                "FATBIN_MAGIC",
                ptr.magic,
                [u32::from_ne_bytes(FatbinHeader::MAGIC)],
            )?;
            ParseError::check_fields("FATBIN_VERSION", ptr.version, [FatbinHeader::VERSION])?;
            Ok(ptr)
        })
}

pub struct Fatbin<'a> {
    pub wrapper: &'a FatbincWrapper,
}

impl<'a> Fatbin<'a> {
    pub fn new<T>(ptr: &'a *const T) -> Result<Self, FatbinError> {
        let wrapper: &FatbincWrapper =
            parse_fatbinc_wrapper(ptr).map_err(|e| FatbinError::ParseFailure(e))?;

        Ok(Fatbin { wrapper })
    }

    pub fn get_submodules(&self) -> Result<FatbinIter<'a>, FatbinError> {
        match self.wrapper.version {
            FatbincWrapper::VERSION_V2 => Ok(FatbinIter::V2(FatbinSubmoduleIterator {
                fatbins: self.wrapper.filename_or_fatbins as *const *const std::ffi::c_void,
                _phantom: std::marker::PhantomData,
            })),
            FatbincWrapper::VERSION_V1 => {
                let header =
                    parse_fatbin_header(&self.wrapper.data).map_err(FatbinError::ParseFailure)?;
                Ok(FatbinIter::V1(Some(FatbinSubmodule::new(header))))
            }
            version => Err(FatbinError::ParseFailure(
                ParseError::UnexpectedBinaryField {
                    field_name: "FATBINC_VERSION",
                    observed: version,
                    expected: [FatbincWrapper::VERSION_V1, FatbincWrapper::VERSION_V2].into(),
                },
            )),
        }
    }
}

pub struct FatbinSubmodule<'a> {
    pub header: &'a FatbinHeader, // TODO: maybe make private
}

impl<'a> FatbinSubmodule<'a> {
    pub fn new(header: &'a FatbinHeader) -> Self {
        FatbinSubmodule { header }
    }

    pub fn get_files(&self) -> FatbinFileIterator<'a> {
        unsafe { FatbinFileIterator::new(self.header) }
    }
}

pub enum FatbinIter<'a> {
    V1(Option<FatbinSubmodule<'a>>),
    V2(FatbinSubmoduleIterator<'a>),
}

impl<'a> FatbinIter<'a> {
    pub fn next(&mut self) -> Result<Option<FatbinSubmodule<'a>>, ParseError> {
        match self {
            FatbinIter::V1(opt) => Ok(opt.take()),
            FatbinIter::V2(iter) => unsafe { iter.next() },
        }
    }
}

pub struct FatbinSubmoduleIterator<'a> {
    fatbins: *const *const std::ffi::c_void,
    _phantom: std::marker::PhantomData<&'a FatbinHeader>,
}

impl<'a> FatbinSubmoduleIterator<'a> {
    pub unsafe fn next(&mut self) -> Result<Option<FatbinSubmodule<'a>>, ParseError> {
        if *self.fatbins != ptr::null() {
            let header = *self.fatbins as *const FatbinHeader;
            self.fatbins = self.fatbins.add(1);
            Ok(Some(FatbinSubmodule::new(header.as_ref().ok_or(
                ParseError::NullPointer("FatbinSubmoduleIterator"),
            )?)))
        } else {
            Ok(None)
        }
    }
}

pub struct FatbinFile<'a> {
    pub header: &'a FatbinFileHeader,
}

impl<'a> FatbinFile<'a> {
    pub fn new(header: &'a FatbinFileHeader) -> Self {
        Self { header }
    }

    pub unsafe fn get_payload(&'a self) -> &'a [u8] {
        let start = std::ptr::from_ref(self.header)
            .cast::<u8>()
            .add(self.header.header_size as usize);
        std::slice::from_raw_parts(start, self.header.payload_size as usize)
    }

    pub unsafe fn decompress(&'a self) -> Result<Vec<u8>, FatbinError> {
        let mut payload = if self
            .header
            .flags
            .contains(FatbinFileHeaderFlags::CompressedLz4)
        {
            unsafe { decompress_lz4(self) }?
        } else if self
            .header
            .flags
            .contains(FatbinFileHeaderFlags::CompressedZstd)
        {
            unsafe { decompress_zstd(self) }?
        } else {
            unsafe { self.get_payload().to_vec() }
        };

        while payload.last() == Some(&0) {
            // remove trailing zeros
            payload.pop();
        }

        Ok(payload)
    }
}

pub struct FatbinFileIterator<'a> {
    file_buffer: &'a [u8],
}

impl<'a> FatbinFileIterator<'a> {
    pub unsafe fn new(header: &'a FatbinHeader) -> Self {
        let start = std::ptr::from_ref(header)
            .cast::<u8>()
            .add(header.header_size as usize);
        let file_buffer = std::slice::from_raw_parts(start, header.files_size as usize);

        Self { file_buffer }
    }

    pub unsafe fn next(&mut self) -> Result<Option<FatbinFile<'a>>, ParseError> {
        if self.file_buffer.len() < std::mem::size_of::<FatbinFileHeader>() {
            return Ok(None);
        }
        let this = &*self.file_buffer.as_ptr().cast::<FatbinFileHeader>();
        let next_element = self
            .file_buffer
            .split_at_checked(this.header_size as usize + this.padded_payload_size as usize)
            .map(|(_, next)| next);
        self.file_buffer = next_element.unwrap_or(&[]);
        ParseError::check_fields(
            "FATBIN_FILE_HEADER_VERSION_CURRENT",
            this.version,
            [FatbinFileHeader::HEADER_VERSION_CURRENT],
        )?;
        Ok(Some(FatbinFile::new(this)))
    }
}

const MAX_MODULE_DECOMPRESSION_BOUND: usize = 64 * 1024 * 1024;

pub unsafe fn decompress_lz4(file: &FatbinFile) -> Result<Vec<u8>, FatbinError> {
    let decompressed_size = usize::max(1024, file.header.uncompressed_payload as usize);
    let mut decompressed_vec = vec![0u8; decompressed_size];
    loop {
        match lz4_sys::LZ4_decompress_safe(
            file.get_payload().as_ptr() as *const _,
            decompressed_vec.as_mut_ptr() as *mut _,
            file.header.payload_size as _,
            decompressed_vec.len() as _,
        ) {
            error if error < 0 => {
                let new_size = decompressed_vec.len() * 2;
                if new_size > MAX_MODULE_DECOMPRESSION_BOUND {
                    return Err(FatbinError::Lz4DecompressionFailure);
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

pub unsafe fn decompress_zstd(file: &FatbinFile) -> Result<Vec<u8>, FatbinError> {
    let mut result = Vec::with_capacity(file.header.uncompressed_payload as usize);
    let payload = file.get_payload();
    match zstd_safe::decompress(&mut result, payload) {
        Ok(actual_size) => {
            result.truncate(actual_size);
            Ok(result)
        }
        Err(err) => Err(FatbinError::ZstdDecompressionFailure(err)),
    }
}
