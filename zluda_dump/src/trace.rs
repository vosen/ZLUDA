use crate::log::{FunctionLogger, LogEntry};
use crate::{dark_api, log, side_by_side, Settings};
use cuda_types::*;
use ptx::{ast, DisplayParseError, ModuleParserExt};
use ptx::{ast::PtxError, Token};
use serde::{Serialize, Serializer};
use std::alloc::Layout;
use std::ffi::CString;
use std::sync::Arc;
use std::{
    collections::HashMap,
    ffi::CStr,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    rc::Rc,
};
use zluda_dark_api::{CUmoduleContent, FatbinFileKind};

// This struct is the heart of CUDA state tracking, it:
// * receives calls from the probes about changes to CUDA state
// * records updates to the state change
// * writes out relevant state change and details to disk and log
pub(crate) struct StateTracker {
    pub(crate) writer: DumpWriter,
    module_counter: usize,
    pub(crate) modules: HashMap<CUmodule, ParsedModule>,
    pub(crate) functions: HashMap<CUfunction, RecordedFunction>,
    pub(crate) texrefs: HashMap<CUtexref, RecordedTexref>,
    pub(crate) dark_api: dark_api::DarkApiState,
    pub(crate) override_cc_major: Option<u32>,
}

fn visit_cumodule_content(
    this: CUmoduleContent,
    cu_module: Option<CUmodule>,
    fn_logger: &mut log::FunctionLogger,
    mut on_file: impl FnMut(&mut log::FunctionLogger, FatbinFileKind, CUmoduleFileIndex, &[u8]),
) {
    match this {
        CUmoduleContent::Elf(buffer) => {
            let buffer = unsafe { hip_common::elf::as_slice(buffer) };
            on_file(
                fn_logger,
                FatbinFileKind::Elf,
                CUmoduleFileIndex::Single,
                buffer,
            );
        }
        CUmoduleContent::Archive(buffer) => {
            on_file(
                fn_logger,
                FatbinFileKind::Archive,
                CUmoduleFileIndex::Single,
                buffer,
            );
        }
        CUmoduleContent::File(file_name) => {
            fn_logger.log_fn(|fn_logger| {
                let file_name = unsafe { CStr::from_ptr(file_name) };
                let file_name = file_name.to_str().map_err(LogEntry::MalformedModulePath)?;
                let buffer = std::fs::read(file_name)?;
                let (kind, _) = unsafe { try_get_kind_slice(cu_module, buffer.as_ptr())? };
                on_file(fn_logger, kind, CUmoduleFileIndex::Single, &buffer);
                Ok(())
            });
        }
        CUmoduleContent::RawText(buffer) => {
            let kind_slice = unsafe { try_get_kind_slice(cu_module, buffer) };
            if let Some((kind, buffer)) = fn_logger.log_unwrap(kind_slice) {
                on_file(fn_logger, kind, CUmoduleFileIndex::Single, buffer);
            }
        }
        CUmoduleContent::Fatbin(zluda_dark_api::CudaFatbin::Version1(module)) => {
            visit_module(fn_logger, &module, &mut on_file, true, None);
        }
        CUmoduleContent::Fatbin(zluda_dark_api::CudaFatbin::Version2 {
            post_link,
            pre_link,
        }) => {
            visit_module(fn_logger, &post_link, &mut on_file, false, None);
            for (sub_index, sub_module) in pre_link.iter().enumerate() {
                visit_module(fn_logger, sub_module, &mut on_file, false, Some(sub_index));
            }
        }
    }
}

unsafe fn try_get_kind_slice(
    module: Option<CUmodule>,
    raw_image: *const u8,
) -> Result<(FatbinFileKind, &'static [u8]), LogEntry> {
    Ok(
        if *(raw_image as *const [u8; 4]) == *goblin::elf64::header::ELFMAG {
            let slice = hip_common::elf::as_slice(raw_image);
            (FatbinFileKind::Elf, slice)
        } else if *(raw_image as *const [u8; 8]) == *goblin::archive::MAGIC {
            return Err(LogEntry::UnsupportedModule {
                module,
                raw_image: raw_image.cast(),
                kind: FatbinFileKind::Archive,
            });
        } else {
            let mut current = raw_image;
            loop {
                if *current == 0 {
                    break;
                }
                current = current.add(1);
            }
            (
                FatbinFileKind::Ptx,
                std::slice::from_raw_parts(raw_image, current.offset_from(raw_image) as usize),
            )
        },
    )
}

fn visit_module(
    fn_logger: &mut FunctionLogger,
    module: &zluda_dark_api::FatbinModuleHandle,
    on_file: &mut impl FnMut(&mut FunctionLogger, FatbinFileKind, CUmoduleFileIndex, &[u8]),
    version1: bool,
    sub_index: Option<usize>,
) {
    fn_logger.log_fn(|fn_logger| {
        let module = unsafe { module.get()? };
        match module {
            zluda_dark_api::FatbinModule::Elf(ptr) => {
                let buffer = unsafe { hip_common::elf::as_slice(ptr) };
                on_file(
                    fn_logger,
                    FatbinFileKind::Elf,
                    if version1 {
                        CUmoduleFileIndex::Single
                    } else {
                        if let Some(module) = sub_index {
                            CUmoduleFileIndex::FatbinSub { module, file: 0 }
                        } else {
                            CUmoduleFileIndex::FatbinMain { file: 0 }
                        }
                    },
                    buffer,
                );
            }
            zluda_dark_api::FatbinModule::Files(files) => {
                for (index, maybe_file) in files.enumerate() {
                    if let Some(file) = fn_logger.log_unwrap(maybe_file.map_err(Into::into)) {
                        if let Some(buffer) = fn_logger
                            .log_unwrap(unsafe { file.get_or_decompress() }.map_err(Into::into))
                        {
                            on_file(
                                fn_logger,
                                file.kind,
                                if version1 {
                                    CUmoduleFileIndex::Fatbin { file: index }
                                } else {
                                    if let Some(module) = sub_index {
                                        CUmoduleFileIndex::FatbinSub {
                                            module,
                                            file: index,
                                        }
                                    } else {
                                        CUmoduleFileIndex::FatbinMain { file: index }
                                    }
                                },
                                &buffer,
                            );
                        }
                    }
                }
            }
        }
        Ok(())
    });
}

#[derive(Clone, Copy)]
enum CUmoduleFileIndex {
    Single,
    Fatbin { file: usize },
    FatbinMain { file: usize },
    FatbinSub { module: usize, file: usize },
}

impl StateTracker {
    pub(crate) fn new(settings: &Settings) -> Self {
        StateTracker {
            writer: DumpWriter::new(settings.dump_dir.clone()),
            modules: HashMap::new(),
            functions: HashMap::new(),
            texrefs: HashMap::new(),
            module_counter: 0,
            dark_api: dark_api::DarkApiState::new(),
            override_cc_major: settings.override_cc_major,
        }
    }

    pub(crate) fn record_module(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        cu_module: Option<CUmodule>,
        module: CUmoduleContent,
    ) {
        self.module_counter += 1;
        let mut parsed_sources = (Vec::new(), HashMap::new());
        let mut ptx_parsing = PtxParseControl::Replace;
        visit_cumodule_content(
            module,
            cu_module,
            fn_logger,
            |fn_logger, kind, index, buffer| {
                fn_logger.log_io_error(self.writer.save_module2(
                    self.module_counter,
                    kind,
                    index,
                    buffer,
                ));
                if ptx_parsing != PtxParseControl::Skip && kind == FatbinFileKind::Ptx {
                    ptx_parsing = match index {
                        CUmoduleFileIndex::Single
                        | CUmoduleFileIndex::Fatbin { .. }
                        | CUmoduleFileIndex::FatbinMain { .. } => PtxParseControl::Replace,
                        CUmoduleFileIndex::FatbinSub { module: 0, .. } => {
                            if !parsed_sources.0.is_empty() {
                                PtxParseControl::Skip
                            } else {
                                PtxParseControl::Append
                            }
                        }
                        CUmoduleFileIndex::FatbinSub { .. } => PtxParseControl::Append,
                    };
                    let parsing_result = match ptx_parsing {
                        PtxParseControl::Append => {
                            std::mem::replace(&mut parsed_sources, (Vec::new(), HashMap::new()))
                        }
                        PtxParseControl::Replace => (Vec::new(), HashMap::new()),
                        PtxParseControl::Skip => {
                            return;
                        }
                    };
                    parsed_sources = self.try_parse_ptx(fn_logger, index, parsing_result, &buffer);
                }
            },
        );
        if let Some(cu_module) = cu_module {
            self.modules
                .insert(cu_module, ParsedModule::new(parsed_sources));
        }
    }

    fn try_parse_ptx(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        file_index: CUmoduleFileIndex,
        (mut kernel_text, mut kernels): (Vec<String>, HashMap<String, Vec<Layout>>),
        buffer: &[u8],
    ) -> (Vec<String>, HashMap<String, Vec<Layout>>) {
        let module_text = match std::str::from_utf8(buffer).map_err(LogEntry::NonUtf8ModuleText) {
            Ok(m) => m,
            Err(err) => {
                fn_logger.log(err);
                return (kernel_text, kernels);
            }
        };
        let (ast, errors) = ptx::ModuleParser::parse_unchecked(module_text);
        if !errors.is_empty() {
            fn_logger.log(log::LogEntry::ModuleParsingError(
                DumpWriter::get_file_name2(self.module_counter, file_index, "log"),
            ));
            fn_logger.log_io_error(self.writer.save_module_error_log(
                module_text,
                self.module_counter,
                file_index,
                &*errors,
            ));
        } else {
            for directive in ast.directives {
                match directive {
                    ast::Directive::Method(
                        _,
                        ast::Function {
                            func_directive:
                                ast::MethodDeclaration {
                                    name: ast::MethodName::Kernel(kernel_name),
                                    input_arguments,
                                    ..
                                },
                            ..
                        },
                    ) => {
                        kernels.insert(
                            kernel_name.to_string(),
                            input_arguments.iter().map(|arg| arg.layout()).collect(),
                        );
                    }
                    _ => {}
                }
            }
            let mut zero_terminated_kernel_text = module_text.to_string();
            zero_terminated_kernel_text.push('\0');
            kernel_text.push(zero_terminated_kernel_text);
        }
        (kernel_text, kernels)
    }

    pub(crate) unsafe fn record_function(
        &mut self,
        func: CUfunction,
        module: CUmodule,
        raw_name: *const ::std::os::raw::c_char,
        fn_logger: &mut log::FunctionLogger,
    ) {
        let name = CStr::from_ptr(raw_name).to_string_lossy().to_string();
        let maybe_parsed_func = if let Some(parsed_mod) = self.modules.get(&module) {
            if let Some(args) = parsed_mod.kernels_args.get(&name) {
                Some(RecordedFunction {
                    name: Arc::new(name.clone()),
                    module,
                    parsed: Some(ParsedFunction {
                        text: parsed_mod.sources.clone(),
                        explicit_arguments: args.clone(),
                    }),
                })
            } else {
                // We don't know about this function
                fn_logger.log(LogEntry::UnknownFunction(func, module, name.clone()));
                None
            }
        } else {
            // We don't know about this module
            fn_logger.log(LogEntry::UnknownModule(module));
            None
        };
        let parsed_func = maybe_parsed_func.unwrap_or_else(move || RecordedFunction {
            name: Arc::new(name),
            module,
            parsed: None,
        });
        self.functions.insert(func, parsed_func);
    }

    pub(crate) fn record_texref(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        name: *const ::std::os::raw::c_char,
        texref: CUtexref,
        hmod: CUmodule,
    ) {
        let parsed_module = match self.modules.get_mut(&hmod) {
            Some(m) => m,
            _ => {
                fn_logger.log(LogEntry::UnknownModule(hmod));
                return;
            }
        };
        parsed_module
            .texrefs
            .insert(unsafe { CStr::from_ptr(name) }.to_owned(), texref);
        self.texrefs
            .insert(texref, RecordedTexref { address: None });
    }

    pub(crate) fn get_texrefs(
        &self,
        module: CUmodule,
    ) -> Result<impl Iterator<Item = (&CStr, CUtexref, TexrefAddress)> + '_, LogEntry> {
        let module = match self.modules.get(&module) {
            Some(m) => m,
            _ => return Err(LogEntry::UnknownModule(module)),
        };
        let all_texrefs = &self.texrefs;
        Ok(module
            .texrefs
            .iter()
            .map(move |(name, cu_texref)| {
                let cu_texref = *cu_texref;
                all_texrefs
                    .get(&cu_texref)
                    .copied()
                    .map(move |recorded| {
                        recorded
                            .address
                            .map(|address| (name.as_c_str(), cu_texref, address))
                    })
                    .into_iter()
            })
            .flatten()
            .flatten())
    }

    pub(crate) fn get_globals(
        &self,
        module: CUmodule,
    ) -> Result<impl Iterator<Item = (&CStr, CUdeviceptr)> + '_, LogEntry> {
        let module = match self.modules.get(&module) {
            Some(m) => m,
            _ => return Err(LogEntry::UnknownModule(module)),
        };
        Ok(module
            .globals
            .iter()
            .map(|(name, ptr)| (name.as_c_str(), *ptr)))
    }

    pub(crate) fn record_texref_address(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        tex_ref: CUtexref,
        pitch: Option<TexrefAddress>,
    ) {
        fn_logger.log_fn(|_| {
            let recorded_texref = self
                .texrefs
                .get_mut(&tex_ref)
                .ok_or(LogEntry::UnknownTexref(tex_ref))?;
            recorded_texref.address = pitch;
            Ok(())
        });
    }

    pub(crate) fn remove_texref_address(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        tex_ref: CUtexref,
    ) {
        self.record_texref_address(fn_logger, tex_ref, None)
    }

    pub(crate) fn record_global(
        &mut self,
        fn_logger: &mut FunctionLogger,
        hmod: CUmodule,
        name: *const i8,
        dptr: CUdeviceptr,
    ) {
        fn_logger.log_fn(|_| {
            let recorded_module = self
                .modules
                .get_mut(&hmod)
                .ok_or(LogEntry::UnknownModule(hmod))?;
            let name = unsafe { CStr::from_ptr(name) }.to_owned();
            recorded_module.globals.insert(name, dptr);
            Ok(())
        });
    }
}

#[derive(PartialEq, Eq)]
enum PtxParseControl {
    Skip,
    Append,
    Replace,
}

pub(crate) struct ParsedModule {
    // Note that text includes NULL terminator
    sources: Rc<Vec<String>>,
    kernels_args: HashMap<String, Vec<Layout>>,
    texrefs: HashMap<CString, CUtexref>,
    globals: HashMap<CString, CUdeviceptr>,
}

impl ParsedModule {
    pub(crate) fn new((kernel_text, kernels): (Vec<String>, HashMap<String, Vec<Layout>>)) -> Self {
        ParsedModule {
            sources: Rc::new(kernel_text),
            kernels_args: kernels,
            texrefs: HashMap::new(),
            globals: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct RecordedTexref {
    pub(crate) address: Option<TexrefAddress>,
}

pub(crate) struct RecordedFunction {
    pub(crate) name: Arc<String>,
    pub(crate) module: CUmodule,
    pub(crate) parsed: Option<ParsedFunction>,
}

pub(crate) struct ParsedFunction {
    pub(crate) text: Rc<Vec<String>>,
    pub(crate) explicit_arguments: Vec<Layout>,
}

#[derive(Serialize, Clone, Copy)]
pub(crate) enum TexrefAddress {
    OneD {
        #[serde(skip)]
        pointer: CUdeviceptr,
        bytes: usize,
    },
    TwoD {
        #[serde(skip)]
        pointer: CUdeviceptr,
        width: usize,
        height: usize,
        #[serde(serialize_with = "serialize_array_format")]
        format: CUarray_format,
        channels: u32,
        pitch: usize,
    },
    Array {
        #[serde(skip)]
        array: CUarray,
        flags: u32,
    },
}

pub(crate) fn serialize_array_format<S>(
    array_format: &CUarray_format,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(array_format.0)
}

impl TexrefAddress {
    pub(crate) fn new_2d(desc: CUDA_ARRAY_DESCRIPTOR, pointer: CUdeviceptr, pitch: usize) -> Self {
        Self::TwoD {
            pointer,
            pitch,
            width: desc.Width,
            height: desc.Height,
            format: desc.Format,
            channels: desc.NumChannels,
        }
    }
}

// This structs writes out information about CUDA execution to the dump dir
pub(crate) struct DumpWriter {
    dump_dir: Option<PathBuf>,
}

impl DumpWriter {
    fn new(dump_dir: Option<PathBuf>) -> Self {
        Self { dump_dir }
    }

    fn save_module2(
        &self,
        cu_module_index: usize,
        kind: FatbinFileKind,
        index: CUmoduleFileIndex,
        buffer: &[u8],
    ) -> io::Result<()> {
        let mut dump_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        dump_file.push(Self::get_file_name2(
            cu_module_index,
            index,
            kind.file_extension(),
        ));
        let mut file = File::create(dump_file)?;
        file.write_all(buffer)?;
        Ok(())
    }

    fn get_file_name2(
        cu_module_index: usize,
        module_file: CUmoduleFileIndex,
        kind: &str,
    ) -> String {
        match module_file {
            CUmoduleFileIndex::Single => {
                format!("module_{:04}.{}", cu_module_index, kind)
            }
            CUmoduleFileIndex::Fatbin { file } => {
                format!("module_{:04}_{:02}.{}", cu_module_index, file + 1, kind)
            }
            CUmoduleFileIndex::FatbinMain { file } => {
                format!(
                    "module_{:04}_main_{:02}.{}",
                    cu_module_index,
                    file + 1,
                    kind
                )
            }
            CUmoduleFileIndex::FatbinSub { module, file } => {
                format!(
                    "module_{:04}_sub_{:03}_{:02}.{}",
                    cu_module_index,
                    module + 1,
                    file + 1,
                    kind
                )
            }
        }
    }

    pub(crate) fn save_kernel_launch(
        &self,
        logger: &mut FunctionLogger,
        name: &str,
        module: &str,
        parameters: KernelLaunchParams,
        input: &side_by_side::HostArguments,
        output: &side_by_side::HostArguments,
    ) {
        logger.log_io_error(Self::save_kernel_launch_impl(
            self, name, module, parameters, input, output,
        ))
    }

    fn save_module_error_log<'input>(
        &self,
        module_text: &str,
        cu_module_index: usize,
        module_file: CUmoduleFileIndex,
        errors: &[ptx::ParseError<usize, Token<'input>, PtxError>],
    ) -> io::Result<()> {
        let mut log_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        log_file.push(Self::get_file_name2(cu_module_index, module_file, "log"));
        let mut file = File::create(log_file)?;
        for error in errors {
            let pretty_print_error = unsafe { DisplayParseError::new(error, module_text) };
            writeln!(file, "{}", pretty_print_error)?;
        }
        Ok(())
    }

    fn save_kernel_launch_impl(
        &self,
        name: &str,
        module: &str,
        parameters: KernelLaunchParams,
        input: &side_by_side::HostArguments,
        output: &side_by_side::HostArguments,
    ) -> io::Result<()> {
        let mut dump_dir = if let Some(ref dump_dir) = self.dump_dir {
            dump_dir.clone()
        } else {
            return Ok(());
        };
        dump_dir.push(name);
        let mut suffix = 1;
        while dump_dir.exists() {
            dump_dir.set_file_name(format!("{}_{}", name, suffix));
            suffix += 1;
        }
        fs::create_dir(&dump_dir)?;
        dump_dir.push("kernel_launch.json");
        let allocations = input
            .memory_allocations
            .0
            .iter()
            .filter_map(|(k, v)| {
                v.array_descriptor()
                    .map(|desc| (*k as usize, Array3dDescriptor::new(desc)))
            })
            .collect::<HashMap<_, _>>();
        let kernel_launch = KernelLaunch {
            name,
            parameters,
            arguments: input,
            allocations,
        };
        let file = File::create(&dump_dir)?;
        serde_json::to_writer_pretty(file, &kernel_launch)?;
        dump_dir.set_file_name("module.ptx");
        fs::write(&dump_dir, module)?;
        dump_dir.set_file_name("pre");
        fs::create_dir(&dump_dir)?;
        Self::write_buffers(input, &dump_dir)?;
        dump_dir.set_file_name("post");
        fs::create_dir(&dump_dir)?;
        Self::write_buffers(output, &dump_dir)?;
        Ok(())
    }

    fn write_buffers(
        buffers: &side_by_side::HostArguments,
        dump_dir: &PathBuf,
    ) -> Result<(), io::Error> {
        Ok(for (key, buffer) in buffers.memory_allocations.0.iter() {
            let mut file_path = dump_dir.clone();
            file_path.push((*key as usize).to_string());
            fs::write(file_path, buffer.data())?;
        })
    }
}

#[derive(Serialize)]
#[allow(non_snake_case)]
pub(crate) struct KernelLaunchParams {
    pub(crate) gridDimX: u32,
    pub(crate) gridDimY: u32,
    pub(crate) gridDimZ: u32,
    pub(crate) blockDimX: u32,
    pub(crate) blockDimY: u32,
    pub(crate) blockDimZ: u32,
    pub(crate) sharedMemBytes: u32,
}

#[derive(Serialize)]
struct KernelLaunch<'a> {
    name: &'a str,
    parameters: KernelLaunchParams,
    #[serde(flatten)]
    arguments: &'a side_by_side::HostArguments,
    allocations: HashMap<usize, Array3dDescriptor>,
}

#[derive(Serialize)]
#[allow(non_snake_case)]
struct Array3dDescriptor {
    pub Width: usize,
    pub Height: usize,
    pub Depth: usize,
    #[serde(serialize_with = "serialize_array_format")]
    pub Format: CUarray_format,
    pub NumChannels: u32,
    pub Flags: u32,
}

impl Array3dDescriptor {
    fn new(original: &CUDA_ARRAY3D_DESCRIPTOR) -> Self {
        Self {
            Width: original.Width,
            Height: original.Height,
            Depth: original.Depth,
            Format: original.Format,
            NumChannels: original.NumChannels,
            Flags: original.Flags,
        }
    }
}
