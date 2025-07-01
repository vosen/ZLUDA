use crate::{
    log::{self, UInt},
    trace, ErrorEntry, FnCallLog, Settings,
};
use cuda_types::{
    cuda::*,
    dark_api::{FatbinFileHeader, FatbinFileHeaderFlags, FatbincWrapper},
};
use dark_api::fatbin::{
    decompress_lz4, decompress_zstd, Fatbin, FatbinFileIterator, FatbinSubmodule,
};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};
use unwrap_or::unwrap_some_or;

// This struct is the heart of CUDA state tracking, it:
// * receives calls from the probes about changes to CUDA state
// * records updates to the state change
// * writes out relevant state change and details to disk and log
pub(crate) struct StateTracker {
    writer: DumpWriter,
    pub(crate) libraries: FxHashMap<CUlibrary, CodePointer>,
    saved_modules: FxHashSet<CUmodule>,
    module_counter: usize,
    submodule_counter: usize,
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
            saved_modules: FxHashSet::default(),
            module_counter: 0,
            submodule_counter: 0,
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
        if self.saved_modules.insert(module) {
            self.module_counter += 1;
            self.submodule_counter = 0;
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
            self.saved_modules.insert(module);
            // TODO: Parse ELF and write it to disk
            fn_logger.log(log::ErrorEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "ELF",
            })
        } else if unsafe { *(raw_image as *const [u8; 8]) } == *goblin::archive::MAGIC {
            self.saved_modules.insert(module);
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
        self.saved_modules.insert(module);
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
    let fatbin = Fatbin::new(&fatbinc_wrapper).map_err(ErrorEntry::from)?;
    let mut submodules = fatbin.get_submodules()?;
    while let Some(current) = submodules.next()? {
        record_submodules_from_fatbin(module, current, fn_logger, state)?;
    }
    Ok(())
}

pub(crate) unsafe fn record_submodules_from_fatbin(
    module: CUmodule,
    submodule: FatbinSubmodule,
    logger: &mut FnCallLog,
    state: &mut StateTracker,
) -> Result<(), ErrorEntry> {
    record_submodules(module, logger, state, submodule.get_files())?;
    Ok(())
}

pub(crate) unsafe fn record_submodules(
    module: CUmodule,
    fn_logger: &mut FnCallLog,
    state: &mut StateTracker,
    mut files: FatbinFileIterator,
) -> Result<(), ErrorEntry> {
    while let Some(file) = files.next()? {
        let mut payload = if file
            .header
            .flags
            .contains(FatbinFileHeaderFlags::CompressedLz4)
        {
            Cow::Owned(unwrap_some_or!(
                fn_logger.try_return(|| decompress_lz4(&file).map_err(|e| e.into())),
                continue
            ))
        } else if file
            .header
            .flags
            .contains(FatbinFileHeaderFlags::CompressedZstd)
        {
            Cow::Owned(unwrap_some_or!(
                fn_logger.try_return(|| decompress_zstd(&file).map_err(|e| e.into())),
                continue
            ))
        } else {
            Cow::Borrowed(file.get_payload())
        };
        match file.header.kind {
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
                    observed: UInt::U16(file.header.kind),
                });
            }
        }
    }
    Ok(())
}
