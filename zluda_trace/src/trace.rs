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
use goblin::{elf, elf32, elf64};
use rustc_hash::{FxHashMap, FxHashSet};
use std::{
    borrow::Cow,
    ffi::{c_void, CStr, CString},
    fs::{self, File},
    io::{self, Read, Write},
    mem,
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
        if unsafe { *(raw_image as *const [u8; 4]) } == *elf64::header::ELFMAG {
            self.saved_modules.insert(module);
            match unsafe { get_elf_size(raw_image) } {
                Some(len) => {
                    let elf_image =
                        unsafe { std::slice::from_raw_parts(raw_image.cast::<u8>(), len) };
                    self.record_new_submodule(module, elf_image, fn_logger, "elf");
                }
                None => fn_logger.log(log::ErrorEntry::UnsupportedModule {
                    module,
                    raw_image,
                    kind: "ELF",
                }),
            }
        } else if unsafe { *(raw_image as *const [u8; 8]) } == *goblin::archive::MAGIC {
            self.saved_modules.insert(module);
            // TODO: Figure out how to get size of archive module and write it to disk
            fn_logger.log(log::ErrorEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "archive",
            })
        } else if unsafe { *(raw_image as *const [u8; 4]) } == FatbincWrapper::MAGIC {
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

unsafe fn get_elf_size(start: *const c_void) -> Option<usize> {
    let start = start.cast::<u8>();
    let ei_class = start.add(mem::size_of_val(elf::header::ELFMAG));
    let (header, header_size, is_64bit): (elf::header::Header, _, _) = match *ei_class {
        elf::header::ELFCLASS32 => (
            (*start.cast::<elf32::header::Header>()).into(),
            mem::size_of::<elf32::header::Header>() as u64,
            false,
        ),
        elf::header::ELFCLASS64 => (
            (*start.cast::<elf64::header::Header>()).into(),
            mem::size_of::<elf64::header::Header>() as u64,
            true,
        ),
        _ => return None,
    };
    let mut max_end = header_size;
    max_end = max_end.max(get_max_end_for::<elf::program_header::ProgramHeader>(
        start, header, is_64bit,
    )?);
    max_end = max_end.max(get_max_end_for::<elf::section_header::SectionHeader>(
        start, header, is_64bit,
    )?);
    Some(max_end as usize)
}

unsafe fn get_max_end_for<T: ElfComponent>(
    elf_start: *const u8,
    header: elf::header::Header,
    is_64bit: bool,
) -> Option<u64> {
    if is_64bit && T::header_size(&header) as usize != mem::size_of::<T::Component64>() {
        return None;
    }
    if !is_64bit && T::header_size(&header) as usize != mem::size_of::<T::Component32>() {
        return None;
    }
    if T::is_extended_header(&header) {
        // TODO: support extended headers
        return None;
    }
    let headers_start = T::headers_offset(&header);
    if headers_start == 0 {
        return Some(0);
    }
    let mut max_end = 0;
    for entry_idx in 0..T::headers_count(&header) as u64 {
        let header_start =
            headers_start.checked_add(entry_idx.checked_mul(T::header_size(&header) as u64)?)?;
        let header_end = header_start.checked_add(T::header_size(&header) as u64)?;
        max_end = max_end.max(header_end);
        let component = if is_64bit {
            (*elf_start
                .add(header_start as usize)
                .cast::<T::Component64>())
            .into()
        } else {
            (*elf_start
                .add(header_start as usize)
                .cast::<T::Component32>())
            .into()
        };
        let component_end = component.start().checked_add(component.size())?;
        max_end = max_end.max(component_end);
    }
    Some(max_end)
}

trait ElfComponent: Sized {
    type Component32: Into<Self> + Copy;
    type Component64: Into<Self> + Copy;

    fn is_extended_header(elf_header: &elf::header::Header) -> bool;
    fn header_size(elf_header: &elf::header::Header) -> u16;
    fn headers_offset(elf_header: &elf::header::Header) -> u64;
    fn headers_count(elf_header: &elf::header::Header) -> u64;
    fn start(&self) -> u64;
    fn size(&self) -> u64;
}

impl ElfComponent for elf::program_header::ProgramHeader {
    type Component32 = elf32::program_header::program_header32::ProgramHeader;
    type Component64 = elf64::program_header::program_header64::ProgramHeader;

    fn is_extended_header(elf_header: &elf::header::Header) -> bool {
        const PN_XNUM: u16 = 0xffff;
        elf_header.e_phnum >= PN_XNUM
    }

    fn header_size(elf_header: &elf::header::Header) -> u16 {
        elf_header.e_phentsize
    }

    fn headers_offset(elf_header: &elf::header::Header) -> u64 {
        elf_header.e_phoff
    }

    fn headers_count(elf_header: &elf::header::Header) -> u64 {
        elf_header.e_phnum as u64
    }

    fn start(&self) -> u64 {
        self.p_offset
    }

    fn size(&self) -> u64 {
        self.p_filesz
    }
}

impl ElfComponent for elf::section_header::SectionHeader {
    type Component32 = elf32::section_header::section_header32::SectionHeader;
    type Component64 = elf64::section_header::section_header64::SectionHeader;

    fn is_extended_header(elf_header: &elf::header::Header) -> bool {
        const SHN_LORESERVE: u16 = 0xff00;
        elf_header.e_phnum >= SHN_LORESERVE
    }

    fn header_size(elf_header: &elf::header::Header) -> u16 {
        elf_header.e_shentsize
    }

    fn headers_offset(elf_header: &elf::header::Header) -> u64 {
        elf_header.e_shoff
    }

    fn headers_count(elf_header: &elf::header::Header) -> u64 {
        elf_header.e_shnum as u64
    }

    fn start(&self) -> u64 {
        self.sh_offset
    }

    fn size(&self) -> u64 {
        self.sh_size
    }
}

// This structs writes out information about CUDA execution to the trace dir
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
