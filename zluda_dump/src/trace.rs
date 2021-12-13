use ptx::{ast::PtxError, Token};
use ptx::{DisplayParseError, ModuleParserExt};

use crate::{cuda::CUmodule, dark_api, log, Settings};
use std::{
    collections::HashMap,
    ffi::{c_void, CStr, CString},
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
    rc::Rc,
};

// This struct is the heart of CUDA state tracking, it:
// * receives calls from the probes about changes to CUDA state
// * records updates to the state change
// * writes out relevant state change and details to disk and log
pub(crate) struct StateTracker {
    writer: DumpWriter,
    modules: HashMap<CUmodule, Option<ParsedModule>>,
    module_counter: usize,
    submodule_counter: usize,
    last_module_version: Option<usize>,
    pub(crate) dark_api: dark_api::DarkApiState,
    pub(crate) override_cc_major: Option<u32>,
}

impl StateTracker {
    pub(crate) fn new(settings: &Settings) -> Self {
        StateTracker {
            writer: DumpWriter::new(settings.dump_dir.clone()),
            modules: HashMap::new(),
            module_counter: 0,
            submodule_counter: 0,
            last_module_version: None,
            dark_api: dark_api::DarkApiState::new(),
            override_cc_major: settings.override_cc_major,
        }
    }

    pub(crate) fn record_new_module_file(
        &mut self,
        module: CUmodule,
        file_name: *const i8,
        fn_logger: &mut log::FunctionLogger,
    ) {
        let file_name = match unsafe { CStr::from_ptr(file_name) }.to_str() {
            Ok(f) => f,
            Err(err) => {
                fn_logger.log(log::LogEntry::MalformedModulePath(err));
                return;
            }
        };
        let maybe_io_error = self.try_record_new_module_file(module, fn_logger, file_name);
        fn_logger.log_io_error(maybe_io_error)
    }

    fn try_record_new_module_file(
        &mut self,
        module: CUmodule,
        fn_logger: &mut log::FunctionLogger,
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
        version: Option<usize>,
        submodule: &[u8],
        fn_logger: &mut log::FunctionLogger,
        type_: &'static str,
    ) {
        if !self.modules.contains_key(&module) {
            self.module_counter += 1;
            self.submodule_counter = 0;
            self.modules.insert(module, None);
        }
        if version != self.last_module_version {
            self.submodule_counter = 0;
        }
        self.submodule_counter += 1;
        self.last_module_version = version;
        fn_logger.log_io_error(self.writer.save_module(
            self.module_counter,
            version,
            Some(self.submodule_counter),
            submodule,
            type_,
        ));
        if type_ == "ptx" {
            match CString::new(submodule) {
                Err(e) => fn_logger.log(log::LogEntry::NulInsideModuleText(e)),
                Ok(submodule_cstring) => match submodule_cstring.to_str() {
                    Err(e) => fn_logger.log(log::LogEntry::NonUtf8ModuleText(e)),
                    Ok(submodule_text) => self.try_parse_and_record_kernels(
                        fn_logger,
                        self.module_counter,
                        version,
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
        fn_logger: &mut log::FunctionLogger,
    ) {
        self.module_counter += 1;
        if unsafe { *(raw_image as *const [u8; 4]) } == *goblin::elf64::header::ELFMAG {
            self.modules.insert(module, None);
            // TODO: Parse ELF and write it to disk
            fn_logger.log(log::LogEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "ELF",
            })
        } else if unsafe { *(raw_image as *const [u8; 8]) } == *goblin::archive::MAGIC {
            self.modules.insert(module, None);
            // TODO: Figure out how to get size of archive module and write it to disk
            fn_logger.log(log::LogEntry::UnsupportedModule {
                module,
                raw_image,
                kind: "archive",
            })
        } else {
            self.record_module_ptx(module, raw_image, fn_logger)
        }
    }

    fn record_module_ptx(
        &mut self,
        module: CUmodule,
        raw_image: *const c_void,
        fn_logger: &mut log::FunctionLogger,
    ) {
        self.modules.insert(module, None);
        let module_text = unsafe { CStr::from_ptr(raw_image as *const _) }.to_str();
        let module_text = match module_text {
            Ok(m) => m,
            Err(utf8_err) => {
                fn_logger.log(log::LogEntry::NonUtf8ModuleText(utf8_err));
                return;
            }
        };
        fn_logger.log_io_error(self.writer.save_module(
            self.module_counter,
            None,
            None,
            module_text.as_bytes(),
            "ptx",
        ));
        self.try_parse_and_record_kernels(fn_logger, self.module_counter, None, None, module_text);
    }

    fn try_parse_and_record_kernels(
        &mut self,
        fn_logger: &mut log::FunctionLogger,
        module_index: usize,
        version: Option<usize>,
        submodule_index: Option<usize>,
        module_text: &str,
    ) {
        let (ast, errors) = ptx::ModuleParser::parse_unchecked(module_text);
        if !errors.is_empty() {
            fn_logger.log(log::LogEntry::ModuleParsingError(
                DumpWriter::get_file_name(module_index, version, submodule_index, "log"),
            ));
            fn_logger.log_io_error(self.writer.save_module_error_log(
                module_index,
                version,
                submodule_index,
                &*errors,
            ));
        }
    }

    pub(crate) fn module_exists(&self, hmod: CUmodule) -> bool {
        self.modules.contains_key(&hmod)
    }
}

struct ParsedModule {
    content: Rc<String>,
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
        version: Option<usize>,
        submodule_index: Option<usize>,
        buffer: &[u8],
        kind: &'static str,
    ) -> io::Result<()> {
        let mut dump_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        dump_file.push(Self::get_file_name(
            module_index,
            version,
            submodule_index,
            kind,
        ));
        let mut file = File::create(dump_file)?;
        file.write_all(buffer)?;
        Ok(())
    }

    fn save_module_error_log<'input>(
        &self,
        module_index: usize,
        version: Option<usize>,
        submodule_index: Option<usize>,
        errors: &[ptx::ParseError<usize, Token<'input>, PtxError>],
    ) -> io::Result<()> {
        let mut log_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        log_file.push(Self::get_file_name(
            module_index,
            version,
            submodule_index,
            "log",
        ));
        let mut file = File::create(log_file)?;
        for error in errors {
            let pretty_print_error = DisplayParseError("", error);
            writeln!(file, "{}", pretty_print_error)?;
        }
        Ok(())
    }

    fn get_file_name(
        module_index: usize,
        version: Option<usize>,
        submodule_index: Option<usize>,
        kind: &str,
    ) -> String {
        match (version, submodule_index) {
            (Some(version), Some(submodule_index)) => format!(
                "module_{:04}_v{}_{}.{}",
                module_index, version, submodule_index, kind
            ),
            (Some(version), None) => {
                format!("module_{:04}_v{}.{}", module_index, version, kind)
            }
            (None, Some(submodule_index)) => {
                format!("module_{:04}_{}.{}", module_index, submodule_index, kind)
            }
            (None, None) => format!("module_{:04}.{}", module_index, kind),
        }
    }
}
