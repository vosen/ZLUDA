use crate::{
    log::{self},
    ErrorEntry, FnCallLog, Settings,
};
use cuda_types::cuda::*;
use goblin::{elf, elf32, elf64};
use rustc_hash::FxHashMap;
use std::{
    alloc::Layout,
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
    pub(crate) parsed_libraries: FxHashMap<SendablePtr, ParsedModule>,
    pub(crate) submodules: FxHashMap<CUmodule, CUlibrary>,
    pub(crate) kernels: FxHashMap<CUfunction, SavedKernel>,
    library_counter: usize,
    pub(crate) enqueue_counter: usize,
    pub(crate) override_cc: Option<(u32, u32)>,
}

pub(crate) struct ParsedModule {
    pub kernels: FxHashMap<String, Vec<Layout>>,
}

pub(crate) struct SavedKernel {
    pub name: String,
    pub owner: SendablePtr,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct SendablePtr(*mut c_void);

unsafe impl Send for SendablePtr {}
unsafe impl Sync for SendablePtr {}

impl StateTracker {
    pub(crate) fn new(settings: &Settings) -> Self {
        StateTracker {
            writer: DumpWriter::new(settings.dump_dir.clone()),
            parsed_libraries: FxHashMap::default(),
            submodules: FxHashMap::default(),
            kernels: FxHashMap::default(),
            library_counter: 0,
            enqueue_counter: 0,
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
                fn_logger.log(log::ErrorEntry::Utf8Error(err));
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
        self.record_new_library(module.0.cast(), read_buff.as_ptr() as *const _, fn_logger);
        Ok(())
    }

    pub(crate) fn record_new_library(
        &mut self,
        handle: *mut c_void,
        raw_image: *const c_void,
        fn_logger: &mut FnCallLog,
    ) {
        fn overwrite<T>(current: &mut Option<T>, value: Option<T>) {
            if value.is_some() {
                *current = value;
            }
        }
        let mut kernel_arguments = None;
        self.library_counter += 1;
        let code_ref = fn_logger.try_return(|| {
            unsafe { zluda_common::CodeLibraryRef::try_load(raw_image) }
                .map_err(ErrorEntry::Utf8Error)
        });
        let code_ref = unwrap_some_or!(code_ref, return);
        unsafe {
            code_ref.iterate_modules(|index, module| match module {
                Err(err) => fn_logger.log(ErrorEntry::from(err)),
                Ok(zluda_common::CodeModuleRef::Elf(elf)) => match get_elf_size(elf) {
                    Some(len) => {
                        let elf_image = std::slice::from_raw_parts(elf.cast::<u8>(), len);
                        overwrite(
                            &mut kernel_arguments,
                            self.record_new_submodule(index, elf_image, fn_logger, "elf"),
                        );
                    }
                    None => fn_logger.log(log::ErrorEntry::UnsupportedModule {
                        handle,
                        raw_image: elf,
                        kind: "ELF",
                    }),
                },
                Ok(zluda_common::CodeModuleRef::Archive(archive)) => {
                    fn_logger.log(log::ErrorEntry::UnsupportedModule {
                        handle,
                        raw_image: archive,
                        kind: "archive",
                    })
                }
                Ok(zluda_common::CodeModuleRef::File(file)) => {
                    if let Some(buffer) = fn_logger
                        .try_(|_| file.get_or_decompress_content().map_err(ErrorEntry::from))
                    {
                        overwrite(
                            &mut kernel_arguments,
                            self.record_new_submodule(index, &*buffer, fn_logger, file.kind()),
                        );
                    }
                }
                Ok(zluda_common::CodeModuleRef::Text(ptx)) => {
                    overwrite(
                        &mut kernel_arguments,
                        self.record_new_submodule(index, ptx.as_bytes(), fn_logger, "ptx"),
                    );
                }
            });
        };
        self.parsed_libraries.insert(
            SendablePtr(handle),
            ParsedModule {
                kernels: kernel_arguments.unwrap_or_default(),
            },
        );
    }

    #[must_use]
    pub(crate) fn record_new_submodule(
        &mut self,
        index: Option<(usize, Option<usize>)>,
        submodule: &[u8],
        fn_logger: &mut FnCallLog,
        type_: &'static str,
    ) -> Option<FxHashMap<String, Vec<Layout>>> {
        fn_logger.try_(|fn_logger| {
            self.writer
                .save_module(fn_logger, self.library_counter, index, submodule, type_)
                .map_err(ErrorEntry::IoError)
        });
        if type_ == "ptx" {
            match CString::new(submodule) {
                Err(e) => {
                    fn_logger.log(log::ErrorEntry::NulInsideModuleText(e));
                    None
                }
                Ok(submodule_cstring) => match submodule_cstring.to_str() {
                    Err(e) => {
                        fn_logger.log(log::ErrorEntry::Utf8Error(e));
                        None
                    }
                    Ok(submodule_text) => Some(self.try_parse_and_record_kernels(
                        fn_logger,
                        self.library_counter,
                        index,
                        submodule_text,
                    )),
                },
            }
        } else {
            None
        }
    }

    fn try_parse_and_record_kernels<'input>(
        &mut self,
        fn_logger: &mut FnCallLog,
        module_index: usize,
        submodule_index: Option<(usize, Option<usize>)>,
        module_text: &'input str,
    ) -> FxHashMap<String, Vec<Layout>> {
        let (errors, params) = ptx_parser::parse_for_errors_and_params(module_text);
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
        params
    }

    pub(crate) fn record_module_in_library(&mut self, module: CUmodule, library: CUlibrary) {
        self.submodules.insert(module, library);
    }

    pub(crate) fn record_function_from_module(
        &mut self,
        fn_logger: &mut FnCallLog,
        func: CUfunction,
        hmod: CUmodule,
        name: *const i8,
    ) {
        let owner = match self.submodules.get(&hmod) {
            Some(m) => m.0.cast::<c_void>(),
            None => hmod.0.cast::<c_void>(),
        };
        self.record_function_from_impl(fn_logger, func, owner, name);
    }

    fn record_function_from_impl(
        &mut self,
        fn_logger: &mut FnCallLog,
        func: CUfunction,
        owner: *mut c_void,
        name: *const i8,
    ) {
        let name = match unsafe { CStr::from_ptr(name) }.to_str() {
            Ok(f) => f,
            Err(err) => {
                fn_logger.log(log::ErrorEntry::Utf8Error(err));
                return;
            }
        };
        let saved_kernel = SavedKernel {
            name: name.to_string(),
            owner: SendablePtr(owner),
        };
        self.kernels.insert(func, saved_kernel);
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
        fn_logger: &mut FnCallLog,
        module_index: usize,
        submodule_index: Option<(usize, Option<usize>)>,
        buffer: &[u8],
        kind: &'static str,
    ) -> io::Result<()> {
        let mut dump_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        let file_name = Self::get_file_name(module_index, submodule_index, kind);
        dump_file.push(&file_name);
        {
            let mut file = File::create_new(dump_file)?;
            file.write_all(buffer)?;
        }
        fn_logger.log(ErrorEntry::SavedModule(file_name));
        Ok(())
    }

    fn save_module_error_log<'input>(
        &self,
        module_index: usize,
        submodule_index: Option<(usize, Option<usize>)>,
        errors: &[ptx_parser::PtxError<'input>],
    ) -> io::Result<()> {
        let mut log_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        log_file.push(Self::get_file_name(module_index, submodule_index, "log"));
        let mut file = File::create_new(log_file)?;
        for error in errors {
            writeln!(file, "{}", error)?;
        }
        Ok(())
    }

    fn get_file_name(
        module_index: usize,
        submodule_index: Option<(usize, Option<usize>)>,
        kind: &str,
    ) -> String {
        match submodule_index {
            None => {
                format!("module_{:04}.{:02}", module_index, kind)
            }
            Some((sub_index, None)) => {
                format!("module_{:04}_{:02}.{}", module_index, sub_index + 1, kind)
            }
            Some((sub_index, Some(subsub_index))) => {
                format!(
                    "module_{:04}_{:02}_{:02}.{}",
                    module_index,
                    sub_index + 1,
                    subsub_index + 1,
                    kind
                )
            }
        }
    }
}
