use ptx::{ast::PtxError, Token};

use crate::{cuda::CUmodule, log, Settings};
use std::{
    collections::HashMap,
    ffi::{c_void, CStr},
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
}

impl StateTracker {
    pub(crate) fn new(settings: &Settings) -> Self {
        StateTracker {
            writer: DumpWriter::new(settings.dump_dir.clone()),
            modules: HashMap::new(),
            module_counter: 0,
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
        let module_text = unsafe { CStr::from_ptr(raw_image as *const _) }.to_str();
        let module_text = match module_text {
            Ok(m) => m,
            Err(utf8_err) => {
                fn_logger.log(log::LogEntry::MalformedModuleText(utf8_err));
                return;
            }
        };
        fn_logger.log_io_error(self.writer.save_module(self.module_counter, module_text));
        let mut errors = Vec::new();
        let ast = ptx::ModuleParser::new().parse(&mut errors, module_text);
        let ast = match (&*errors, ast) {
            (&[], Ok(ast)) => ast,
            (err_vec, res) => {
                fn_logger.log(log::LogEntry::ModuleParsingError(self.module_counter));
                fn_logger.log_io_error(self.writer.save_module_error_log(
                    self.module_counter,
                    err_vec,
                    res.err(),
                ));
                return;
            }
        };
        // TODO: store kernel names and details
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

    fn save_module(&self, index: usize, text: &str) -> io::Result<()> {
        let mut dump_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        dump_file.push(format!("module_{:04}.ptx", index));
        let mut file = File::create(dump_file)?;
        file.write_all(text.as_bytes())?;
        Ok(())
    }

    fn save_module_error_log<'input>(
        &self,
        index: usize,
        recoverable: &[ptx::ParseError<usize, Token<'input>, PtxError>],
        unrecoverable: Option<ptx::ParseError<usize, Token<'input>, PtxError>>,
    ) -> io::Result<()> {
        let mut log_file = match &self.dump_dir {
            None => return Ok(()),
            Some(d) => d.clone(),
        };
        log_file.push(format!("module_{:04}.log", index));
        let mut file = File::create(log_file)?;
        for err in unrecoverable.iter().chain(recoverable.iter()) {
            writeln!(file, "{}", err)?;
        }
        Ok(())
    }
}
