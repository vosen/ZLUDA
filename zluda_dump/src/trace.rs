use crate::{
    log::{self, UInt},
    trace, ErrorEntry, FnCallLog, Settings,
};
use cuda_types::{
    cuda::*,
    dark_api::{FatbinFileHeader, FatbinFileHeaderFlags, FatbinHeader, FatbincWrapper},
};
use rustc_hash::FxHashMap;
use std::{
    borrow::Cow,
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    ptr,
};
use unwrap_or::unwrap_some_or;

// This struct is the heart of CUDA state tracking, it:
// * receives calls from the probes about changes to CUDA state
// * records updates to the state change
// * writes out relevant state change and details to disk and log
pub(crate) struct StateTracker {
    writer: DumpWriter,
    pub(crate) libraries: FxHashMap<CUlibrary, CodePointer>,
    modules: FxHashMap<CUmodule, Option<ParsedModule>>,
    module_counter: usize,
    submodule_counter: usize,
    last_module_version: Option<usize>,
    pub(crate) override_cc: Option<(u32, u32)>,
}

#[derive(Clone, Copy)]
pub(crate) struct CodePointer(pub *const c_void);

unsafe impl Send for CodePointer {}
unsafe impl Sync for CodePointer {}

impl StateTracker {
    pub(crate) fn new(settings: &Settings) -> Self {
        StateTracker {
            writer: DumpWriter::new(settings.dump_dir.clone()),
            libraries: FxHashMap::default(),
            modules: FxHashMap::default(),
            module_counter: 0,
            submodule_counter: 0,
            last_module_version: None,
            override_cc: settings.override_cc,
        }
    }

    pub(crate) fn record_new_module_file(
        &mut self,
        module: CUmodule,
        file_name: *const i8,
        fn_logger: &mut FnCallLog,
    ) {
        let file_name = match unsafe { CStr::from_ptr(file_name) }.to_str() {
            Ok(f) => f,
            Err(err) => {
                fn_logger.log(log::ErrorEntry::MalformedModulePath(err));
                return;
            }
        };
        let maybe_io_error = self.try_record_new_module_file(module, fn_logger, file_name);
        fn_logger.log_io_error(maybe_io_error)
    }

    fn try_record_new_module_file(
        &mut self,
        module: CUmodule,
        fn_logger: &mut FnCallLog,
        file_name: &str,
    ) -> io::Result<()> {
        let mut module_file = fs::File::open(file_name)?;
        let mut read_buff = Vec::new();
        module_file.read_to_end(&mut read_buff)?;
        self.record_new_module(module, read_buff.as_ptr() as *const _, fn_logger);
        Ok(())
    }

    pub(crate) fn record_new_submodule(
        &mut self,
        module: CUmodule,
        submodule: &[u8],
        fn_logger: &mut FnCallLog,
        type_: &'static str,
    ) {
        if !self.modules.contains_key(&module) {
            self.module_counter += 1;
            self.submodule_counter = 0;
            self.modules.insert(module, None);
        }
        self.submodule_counter += 1;
        fn_logger.log_io_error(self.writer.save_module(
            self.module_counter,
            Some(self.submodule_counter),
            submodule,
            type_,
        ));
        if type_ == "ptx" {
            match CString::new(submodule) {
                Err(e) => fn_logger.log(log::ErrorEntry::NulInsideModuleText(e)),
                Ok(submodule_cstring) => match submodule_cstring.to_str() {
                    Err(e) => fn_logger.log(log::ErrorEntry::NonUtf8ModuleText(e)),
                    Ok(submodule_text) => self.try_parse_and_record_kernels(
                        fn_logger,
                        self.module_counter,
                        Some(self.submodule_counter),
                        submodule_text,
                    ),
                },
            }
        }
    }

    pub(crate) fn record_new_module(
        &mut self,
        module: CUmodule,
        raw_image: *const c_void,
        fn_logger: &mut FnCallLog,
    ) {
        self.module_counter += 1;
        if unsafe { *(raw_image as *const [u8; 4]) } == *goblin::elf64::header::ELFMAG {
            self.modules.insert(module, None);
            // TODO: Parse ELF and write it to disk
            fn_logger.log(log::ErrorEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "ELF",
            })
        } else if unsafe { *(raw_image as *const [u8; 8]) } == *goblin::archive::MAGIC {
            self.modules.insert(module, None);
            // TODO: Figure out how to get size of archive module and write it to disk
            fn_logger.log(log::ErrorEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "archive",
            })
        } else if unsafe { *(raw_image as *const u32) } == FatbincWrapper::MAGIC {
            unsafe {
                fn_logger.try_(|fn_logger| {
                    trace::record_submodules_from_wrapped_fatbin(
                        module,
                        raw_image as *const FatbincWrapper,
                        fn_logger,
                        self,
                    )
                });
            }
        } else {
            self.record_module_ptx(module, raw_image, fn_logger)
        }
    }

    fn record_module_ptx(
        &mut self,
        module: CUmodule,
        raw_image: *const c_void,
        fn_logger: &mut FnCallLog,
    ) {
        self.modules.insert(module, None);
        let module_text = unsafe { CStr::from_ptr(raw_image as *const _) }.to_str();
        let module_text = match module_text {
            Ok(m) => m,
            Err(utf8_err) => {
                fn_logger.log(log::ErrorEntry::NonUtf8ModuleText(utf8_err));
                return;
            }
        };
        fn_logger.log_io_error(self.writer.save_module(
            self.module_counter,
            None,
            module_text.as_bytes(),
            "ptx",
        ));
        self.try_parse_and_record_kernels(fn_logger, self.module_counter, None, module_text);
    }

    fn try_parse_and_record_kernels(
        &mut self,
        fn_logger: &mut FnCallLog,
        module_index: usize,
        submodule_index: Option<usize>,
        module_text: &str,
    ) {
        let errors = ptx_parser::parse_for_errors(module_text);
        if !errors.is_empty() {
            fn_logger.log(log::ErrorEntry::ModuleParsingError(
                DumpWriter::get_file_name(module_index, submodule_index, "log"),
            ));
            fn_logger.log_io_error(self.writer.save_module_error_log(
                module_index,
                submodule_index,
                &*errors,
            ));
        }
    }
}

struct ParsedModule {
    kernels_args: Option<HashMap<String, Vec<usize>>>,
}

// This structs writes out information about CUDA execution to the dump dir
struct DumpWriter {
    dump_dir: Option<PathBuf>,
}

impl DumpWriter {
    fn new(dump_dir: Option<PathBuf>) -> Self {
        Self { dump_dir }
    }

    fn save_module(
        &self,
        module_index: usize,
        submodule_index: Option<usize>,
        buffer: &[u8],
        kind: &'static str,
    ) -> io::Result<()> {
        let mut dump_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        dump_file.push(Self::get_file_name(module_index, submodule_index, kind));
        let mut file = File::create(dump_file)?;
        file.write_all(buffer)?;
        Ok(())
    }

    fn save_module_error_log<'input>(
        &self,
        module_index: usize,
        submodule_index: Option<usize>,
        errors: &[ptx_parser::PtxError<'input>],
    ) -> io::Result<()> {
        let mut log_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        log_file.push(Self::get_file_name(module_index, submodule_index, "log"));
        let mut file = File::create(log_file)?;
        for error in errors {
            writeln!(file, "{}", error)?;
        }
        Ok(())
    }

    fn get_file_name(module_index: usize, submodule_index: Option<usize>, kind: &str) -> String {
        match submodule_index {
            None => {
                format!("module_{:04}.{:02}", module_index, kind)
            }
            Some(submodule_index) => {
                format!("module_{:04}_{:02}.{}", module_index, submodule_index, kind)
            }
        }
    }
}

pub(crate) unsafe fn record_submodules_from_wrapped_fatbin(
    module: CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    fn_logger: &mut FnCallLog,
    state: &mut StateTracker,
) -> Result<(), ErrorEntry> {
    let fatbinc_wrapper = FatbincWrapper::new(&fatbinc_wrapper).map_err(ErrorEntry::from)?;
    let is_version_2 = fatbinc_wrapper.version == FatbincWrapper::VERSION_V2;
    record_submodules_from_fatbin(module, (*fatbinc_wrapper).data, fn_logger, state)?;
    if is_version_2 {
        let mut current = (*fatbinc_wrapper).filename_or_fatbins as *const *const c_void;
        while *current != ptr::null() {
            record_submodules_from_fatbin(module, *current as *const _, fn_logger, state)?;
            current = current.add(1);
        }
    }
    Ok(())
}

pub(crate) unsafe fn record_submodules_from_fatbin(
    module: CUmodule,
    fatbin_header: *const FatbinHeader,
    logger: &mut FnCallLog,
    state: &mut StateTracker,
) -> Result<(), ErrorEntry> {
    let header = FatbinHeader::new(&fatbin_header).map_err(ErrorEntry::from)?;
    let file = header.get_content();
    record_submodules(module, logger, state, file)?;
    Ok(())
}

unsafe fn record_submodules(
    module: CUmodule,
    fn_logger: &mut FnCallLog,
    state: &mut StateTracker,
    mut file_buffer: &[u8],
) -> Result<(), ErrorEntry> {
    while let Some(file) = FatbinFileHeader::next(&mut file_buffer)? {
        let mut payload = if file.flags.contains(FatbinFileHeaderFlags::CompressedLz4) {
            Cow::Owned(unwrap_some_or!(
                fn_logger.try_return(|| decompress_lz4(file)),
                continue
            ))
        } else if file.flags.contains(FatbinFileHeaderFlags::CompressedZstd) {
            Cow::Owned(unwrap_some_or!(
                fn_logger.try_return(|| decompress_zstd(file)),
                continue
            ))
        } else {
            Cow::Borrowed(file.get_payload())
        };
        match file.kind {
            FatbinFileHeader::HEADER_KIND_PTX => {
                while payload.last() == Some(&0) {
                    // remove trailing zeros
                    payload.to_mut().pop();
                }
                state.record_new_submodule(module, &*payload, fn_logger, "ptx")
            }
            FatbinFileHeader::HEADER_KIND_ELF => {
                state.record_new_submodule(module, &*payload, fn_logger, "elf")
            }
            _ => {
                fn_logger.log(log::ErrorEntry::UnexpectedBinaryField {
                    field_name: "FATBIN_FILE_HEADER_KIND",
                    expected: vec![
                        UInt::U16(FatbinFileHeader::HEADER_KIND_PTX),
                        UInt::U16(FatbinFileHeader::HEADER_KIND_ELF),
                    ],
                    observed: UInt::U16(file.kind),
                });
            }
        }
    }
    Ok(())
}

const MAX_MODULE_DECOMPRESSION_BOUND: usize = 64 * 1024 * 1024;

unsafe fn decompress_lz4(file: &FatbinFileHeader) -> Result<Vec<u8>, ErrorEntry> {
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
                    return Err(ErrorEntry::Lz4DecompressionFailure);
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

unsafe fn decompress_zstd(file: &FatbinFileHeader) -> Result<Vec<u8>, ErrorEntry> {
    let mut result = Vec::with_capacity(file.uncompressed_payload as usize);
    let payload = file.get_payload();
    dbg!((payload.len(), file.uncompressed_payload, file.payload_size));
    match zstd_safe::decompress(&mut result, payload) {
        Ok(actual_size) => {
            result.truncate(actual_size);
            Ok(result)
        }
        Err(err) => Err(ErrorEntry::ZstdDecompressionFailure(err)),
    }
}
