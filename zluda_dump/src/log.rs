use super::CUresult;
use super::Settings;
use crate::format;
use crate::format::CudaDisplay;
use crate::parse_env_var;
use cuda_types::*;
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::ffi::c_void;
use std::ffi::CString;
use std::fmt;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Stderr;
use std::io::Write;
use std::path::PathBuf;
use std::str::Utf8Error;
use zluda_dark_api::AnyUInt;
use zluda_dark_api::FatbinFileKind;
use zluda_dark_api::Lz4DecompressionFailure;
use zluda_dark_api::UnexpectedFieldError;

const LOG_PREFIX: &[u8] = b"[ZLUDA_DUMP] ";

// This type holds all the relevant settings for logging like output path and
// creates objects which match those settings
pub(crate) struct Factory {
    // Fallible emitter is optional emitter to file system, we might lack
    // file permissions or be out of disk space
    fallible_emitter: Option<Box<dyn WriteTrailingZeroAware>>,
    // This is emitter that "always works" (and if it does not, then we don't
    // care). In addition of normal logs it emits errors from fallible emitter
    infallible_emitter: Box<dyn WriteTrailingZeroAware>,
    write_buffer: WriteBuffer,
    // another shared buffer, so we dont't reallocate on every function call
    log_queue: Vec<LogEntry>,
    log_enable: bool,
}

// When writing out to the emitter (file, WinAPI, whatever else) instead of
// writing piece-by-piece it's better to first concatenate everything in memory
// then write out from memory to the slow emitter only once.
// Additionally we might have an unprefixed and prefixed buffer, this struct
// handles this detail
struct WriteBuffer {
    prefixed_buffer: Option<Vec<u8>>,
    unprefixed_buffer: Option<Vec<u8>>,
}

impl WriteBuffer {
    fn new() -> Self {
        WriteBuffer {
            prefixed_buffer: None,
            unprefixed_buffer: None,
        }
    }

    fn init(
        &mut self,
        fallible_emitter: &Option<Box<dyn WriteTrailingZeroAware>>,
        infallible_emitter: &Box<dyn WriteTrailingZeroAware>,
    ) {
        if infallible_emitter.should_prefix() {
            self.prefixed_buffer = Some(Vec::new());
        } else {
            self.unprefixed_buffer = Some(Vec::new());
        }
        if let Some(emitter) = fallible_emitter {
            if emitter.should_prefix() {
                self.prefixed_buffer = Some(Vec::new());
            } else {
                self.unprefixed_buffer = Some(Vec::new());
            }
        }
    }

    fn all_buffers(&mut self) -> impl Iterator<Item = &mut Vec<u8>> {
        self.prefixed_buffer
            .as_mut()
            .into_iter()
            .chain(self.unprefixed_buffer.as_mut().into_iter())
    }

    fn start_line(&mut self) {
        if let Some(buffer) = &mut self.prefixed_buffer {
            buffer.extend_from_slice(LOG_PREFIX);
        }
    }

    fn end_line(&mut self) {
        for buffer in self.all_buffers() {
            buffer.push(b'\n');
        }
    }

    fn write(&mut self, s: &str) {
        for buffer in self.all_buffers() {
            buffer.extend_from_slice(s.as_bytes());
        }
    }

    fn finish(&mut self) {
        for buffer in self.all_buffers() {
            buffer.push(b'\0');
        }
    }

    fn undo_finish(&mut self) {
        for buffer in self.all_buffers() {
            buffer.truncate(buffer.len() - 1);
        }
    }

    fn send_to(&self, log_emitter: &mut Box<dyn WriteTrailingZeroAware>) -> Result<(), io::Error> {
        if log_emitter.should_prefix() {
            log_emitter.write_zero_aware(
                &*self
                    .prefixed_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
            )
        } else {
            log_emitter.write_zero_aware(
                &*self
                    .unprefixed_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
            )
        }
    }

    fn reset(&mut self) {
        for buffer in self.all_buffers() {
            unsafe { buffer.set_len(0) };
        }
    }
}

impl Write for WriteBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(buffer) = &mut self.prefixed_buffer {
            buffer.extend_from_slice(buf);
        }
        if let Some(buffer) = &mut self.unprefixed_buffer {
            buffer.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Factory {
    pub(crate) fn new() -> Self {
        let log_enable = parse_env_var::<bool, _>("ZLUDA_LOG_ENABLE", &mut |_| {}).unwrap_or(true);
        let infallible_emitter = if !log_enable {
            Box::new(NullLog)
        } else {
            os::new_debug_logger()
        };
        Factory {
            infallible_emitter,
            fallible_emitter: None,
            write_buffer: WriteBuffer::new(),
            log_queue: Vec::new(),
            log_enable,
        }
    }

    fn initalize_fallible_emitter(
        settings: &Settings,
    ) -> std::io::Result<Option<Box<dyn WriteTrailingZeroAware>>> {
        if !settings.log_enabled {
            return Ok(None);
        }
        settings
            .dump_dir
            .as_ref()
            .map(|path| {
                Ok::<_, std::io::Error>(Box::new(File::create(path.to_path_buf().join("log.txt"))?)
                    as Box<dyn WriteTrailingZeroAware>)
            })
            .transpose()
    }

    // We load settings during first function call, since during that time we
    // also create one of the loggers, what do we do about errors encountered
    // at that time? We log them to the newly created logger, but to make it
    // "nice" we do both of those in a single function
    // An alternative would be to have something like this:
    //   let mut factory = Factory::new();
    //   let mut cuInitLog = factory.get_logger("cuInit");
    //   cuInitLog.load_settings(&settings);
    // which is a bit nonsensical
    pub(crate) fn get_first_logger_and_init_settings(
        &mut self,
        func: &'static str,
        arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
    ) -> (FunctionLogger, Settings) {
        let log_enabled = self.log_enable;
        let mut first_logger = self.get_logger(func, arguments_writer);
        let settings = Settings::read_and_init(log_enabled, &mut first_logger);
        match Self::initalize_fallible_emitter(&settings) {
            Ok(fallible_emitter) => {
                *first_logger.fallible_emitter = fallible_emitter;
            }
            Err(err) => first_logger.log(LogEntry::IoError(err)),
        }
        first_logger.write_buffer.init(
            first_logger.fallible_emitter,
            first_logger.infallible_emitter,
        );
        (first_logger, settings)
    }

    pub(crate) fn get_logger(
        &mut self,
        func: &'static str,
        arguments_writer: Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>,
    ) -> FunctionLogger {
        FunctionLogger {
            result: None,
            name: CudaFunctionName::Normal(func),
            fallible_emitter: &mut self.fallible_emitter,
            infallible_emitter: &mut self.infallible_emitter,
            write_buffer: &mut self.write_buffer,
            log_queue: &mut self.log_queue,
            arguments_writer: Some(arguments_writer),
        }
    }

    pub(crate) fn get_logger_dark_api(
        &mut self,
        guid: CUuuid,
        index: usize,
        arguments_writer: Option<Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>>,
    ) -> FunctionLogger {
        FunctionLogger {
            result: None,
            name: CudaFunctionName::Dark { guid, index },
            fallible_emitter: &mut self.fallible_emitter,
            infallible_emitter: &mut self.infallible_emitter,
            write_buffer: &mut self.write_buffer,
            log_queue: &mut self.log_queue,
            arguments_writer,
        }
    }
}

enum CudaFunctionName {
    Normal(&'static str),
    Dark { guid: CUuuid, index: usize },
}

// This encapsulates log output for a single function call.
// It's a separate struct and not just a plain function for two reasons:
// * While we want to always display return code before logging errors,
//   logging errors might come before return code is returned
// * We want to handle panics gracefully with Drop
pub(crate) struct FunctionLogger<'a> {
    pub(crate) result: Option<CUresult>,
    name: CudaFunctionName,
    infallible_emitter: &'a mut Box<dyn WriteTrailingZeroAware>,
    fallible_emitter: &'a mut Option<Box<dyn WriteTrailingZeroAware>>,
    arguments_writer: Option<Box<dyn FnMut(&mut dyn std::io::Write) -> std::io::Result<()>>>,
    write_buffer: &'a mut WriteBuffer,
    log_queue: &'a mut Vec<LogEntry>,
}

impl<'a> FunctionLogger<'a> {
    pub(crate) fn log(&mut self, l: LogEntry) {
        self.log_queue.push(l);
    }

    pub(crate) fn log_io_error(&mut self, error: io::Result<()>) {
        if let Err(e) = error {
            self.log_queue.push(LogEntry::IoError(e));
        }
    }

    pub(crate) fn log_unwrap<T>(&mut self, err: Result<T, LogEntry>) -> Option<T> {
        match err {
            Ok(t) => Some(t),
            Err(e) => {
                self.log(e);
                None
            }
        }
    }

    pub(crate) fn log_fn(&mut self, f: impl FnOnce(&mut Self) -> Result<(), LogEntry>) {
        let result = f(self);
        if let Err(e) = result {
            self.log(e)
        }
    }

    fn flush_log_queue_to_write_buffer(&mut self) {
        self.write_buffer.start_line();
        match self.name {
            CudaFunctionName::Normal(fn_name) => self.write_buffer.write(fn_name),
            CudaFunctionName::Dark { guid, index } => {
                format::CudaDisplay::write(&guid, "", 0, &mut self.write_buffer).ok();
                write!(&mut self.write_buffer, "::{}", index).ok();
            }
        }
        match &mut self.arguments_writer {
            Some(arg_writer) => {
                arg_writer(&mut self.write_buffer).ok();
            }
            None => {
                self.write_buffer.write_all(b"(...)").ok();
            }
        }
        self.write_buffer.write_all(b" -> ").ok();
        if let Some(result) = self.result {
            format::CudaDisplay::write(&result, "", 0, self.write_buffer).ok();
        } else {
            self.write_buffer.write_all(b"UNKNOWN").ok();
        };
        self.write_buffer.end_line();
        for entry in self.log_queue.iter() {
            write!(self.write_buffer, "    {}", entry).ok();
            self.write_buffer.end_line();
        }
        self.write_buffer.finish();
    }

    // This is a dirty hack: we call it at the point where our write buffer is
    // already finalized and squeeze the error produced by the previous emitter
    fn hack_squeeze_in_additional_error(&mut self, entry: LogEntry) {
        self.write_buffer.undo_finish();
        write!(self.write_buffer, "    {}", entry).unwrap_or_else(|_| unreachable!());
        self.write_buffer.end_line();
        self.write_buffer.finish();
    }
}

impl<'a> Drop for FunctionLogger<'a> {
    fn drop(&mut self) {
        self.flush_log_queue_to_write_buffer();
        let error_from_writing_to_fallible_emitter = match self.fallible_emitter {
            Some(emitter) => self.write_buffer.send_to(emitter),
            None => Ok(()),
        };
        if let Err(e) = error_from_writing_to_fallible_emitter {
            self.hack_squeeze_in_additional_error(LogEntry::IoError(e))
        }
        self.write_buffer.send_to(self.infallible_emitter).ok();
        self.write_buffer.reset();
        self.log_queue.truncate(0);
    }
}

// Structured log type. We don't want frontend to care about log formatting
pub(crate) enum LogEntry {
    IoError(io::Error),
    CreatedDumpDirectory(PathBuf),
    SideBySideStart(CString),
    ErrorBox(Box<dyn Error>),
    UnsupportedModule {
        module: Option<CUmodule>,
        raw_image: *const c_void,
        kind: FatbinFileKind,
    },
    MalformedModulePath(Utf8Error),
    NonUtf8ModuleText(Utf8Error),
    ModuleParsingError(String),
    Lz4DecompressionFailure,
    UnknownExportTableFn,
    UnexpectedBinaryField {
        field_name: &'static str,
        expected: Vec<AnyUInt>,
        observed: AnyUInt,
    },
    UnknownModule(CUmodule),
    UnknownFunction(CUfunction, CUmodule, String),
    UnknownFunctionUse(CUfunction),
    NoCudaFunction(Cow<'static, str>),
    CudaError(cuda_types::CUresult),
    ArgumentMismatch {
        devptr: *mut c_void,
        diff_count: usize,
        total_count: usize,
    },
    UnknownTexref(CUtexref),
    EnvVarError(env::VarError),
    MalformedEnvVar {
        key: &'static str,
        value: String,
        err: Box<dyn Display>,
    },
    ExportTableLengthGuess(usize),
    ExportTableLengthMismatch {
        expected: usize,
        observed: usize,
    },
    IntegrityHashOverride {
        before: u128,
        after: u128,
    },
    WrappedContext,
}

impl Display for LogEntry {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::fmt::Result {
        match self {
            LogEntry::IoError(e) => e.fmt(f),
            LogEntry::CreatedDumpDirectory(dir) => {
                write!(
                    f,
                    "Created dump directory {}",
                    dir.as_os_str().to_string_lossy()
                )
            }
            LogEntry::SideBySideStart(device) => {
                write!(
                    f,
                    "Side-by-side secondary device: {}",
                    device.to_string_lossy()
                )
            }
            LogEntry::ErrorBox(e) => e.fmt(f),
            LogEntry::UnsupportedModule {
                module,
                raw_image,
                kind,
            } => {
                write!(
                    f,
                    "Unsupported {} module {:?} loaded from module image {:?}",
                    human_readable(*kind),
                    module,
                    raw_image
                )
            }
            LogEntry::MalformedModulePath(e) => e.fmt(f),
            LogEntry::NonUtf8ModuleText(e) => e.fmt(f),
            LogEntry::ModuleParsingError(file_name) => {
                write!(
                    f,
                    "Error parsing module, log has been written to {}",
                    file_name
                )
            }
            LogEntry::Lz4DecompressionFailure => write!(f, "LZ4 decompression failure"),
            LogEntry::UnknownExportTableFn => write!(f, "Unknown export table function"),
            LogEntry::UnexpectedBinaryField {
                field_name,
                expected,
                observed,
            } => write!(
                f,
                "Unexpected field {}. Expected one of: {{{}}}, observed: {}",
                field_name,
                expected
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                observed
            ),
            LogEntry::UnknownModule(module) => as_io_write(f, |f| {
                f.write_all(b"Unknown module ")?;
                CudaDisplay::write(module, "", 0, f)
            }),
            LogEntry::UnknownFunction(func, module, name) => as_io_write(f, |f| {
                f.write_all(b"Unknown function ")?;
                CudaDisplay::write(func, "", 0, f)?;
                write!(f, " ({}) in module ", name)?;
                CudaDisplay::write(module, "", 0, f)
            }),
            LogEntry::UnknownFunctionUse(func) => as_io_write(f, |f| {
                f.write_all(b"Unknown function ")?;
                CudaDisplay::write(func, "", 0, f)
            }),
            LogEntry::NoCudaFunction(fn_name) => {
                write!(f, "Could not resolve CUDA function {}", fn_name)
            }
            LogEntry::CudaError(error) => as_io_write(f, |f| {
                f.write_all(b"CUDA error ")?;
                CudaDisplay::write(error, "", 0, f)
            }),
            LogEntry::ArgumentMismatch {
                devptr,
                diff_count,
                total_count,
            } => {
                let total_percentage = (*diff_count as f64 / *total_count as f64) * 100f64;
                write!(
                    f,
                    "Mismatched elements for argument {:p}. Mismatched elements: {} / {} ({}%)",
                    *devptr, diff_count, total_count, total_percentage
                )
            }
            LogEntry::UnknownTexref(texref) => as_io_write(f, |f| {
                f.write_all(b"Unknown texture references ")?;
                CudaDisplay::write(texref, "", 0, f)
            }),
            LogEntry::EnvVarError(err) => {
                write!(f, "Error reading environment variable: {}", err)
            }
            LogEntry::MalformedEnvVar { key, value, err } => {
                write!(
                    f,
                    "Error parsing environment variable with key {} and value {}: {}",
                    key, value, err
                )
            }
            LogEntry::ExportTableLengthGuess(length) => {
                write!(
                    f,
                    "Unknown export table length, guessed length: {}",
                    *length
                )
            }
            LogEntry::ExportTableLengthMismatch { expected, observed } => {
                write!(
                    f,
                    "Export table length mismatch. Expected: {}, observed: {}",
                    *expected, *observed
                )
            }
            LogEntry::IntegrityHashOverride { before, after } => {
                write!(
                    f,
                    "Overriding integrity hash. Before: {before:032x}, after: {after:032x}"
                )
            }
            LogEntry::WrappedContext => write!(f, "Observed wrapped context"),
        }
    }
}

fn as_io_write<'a, 'b: 'a>(
    formatter: &'a mut fmt::Formatter<'b>,
    func: impl FnOnce(&mut IoFormatter<'a, 'b>) -> io::Result<()>,
) -> fmt::Result {
    let mut formatter = IoFormatter { formatter };
    func(&mut formatter).map_err(|_| fmt::Error)
}

struct IoFormatter<'a, 'b: 'a> {
    formatter: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> fmt::Write for IoFormatter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.formatter.write_str(s)
    }

    fn write_char(&mut self, c: char) -> fmt::Result {
        self.formatter.write_char(c)
    }

    fn write_fmt(&mut self, args: fmt::Arguments) -> fmt::Result {
        self.formatter.write_fmt(args)
    }
}

impl<'a, 'b> io::Write for IoFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let str =
            std::str::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        self.formatter
            .write_str(str)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

fn human_readable(kind: FatbinFileKind) -> &'static str {
    match kind {
        FatbinFileKind::Ptx => "PTX",
        FatbinFileKind::Elf => "ELF",
        FatbinFileKind::Archive => "Archive",
    }
}

impl From<io::Error> for LogEntry {
    fn from(e: io::Error) -> Self {
        LogEntry::IoError(e)
    }
}

impl From<Lz4DecompressionFailure> for LogEntry {
    fn from(_err: Lz4DecompressionFailure) -> Self {
        LogEntry::Lz4DecompressionFailure
    }
}

impl From<UnexpectedFieldError> for LogEntry {
    fn from(err: UnexpectedFieldError) -> Self {
        LogEntry::UnexpectedBinaryField {
            field_name: err.name,
            expected: err.expected,
            observed: err.observed,
        }
    }
}

// Some of our writers want to have trailing zero (WinAPI debug logger) and some
// don't (everything else), this trait encapsulates that logic
pub(crate) trait WriteTrailingZeroAware {
    fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()>;
    fn flush(&mut self) -> std::io::Result<()>;
    fn should_prefix(&self) -> bool;
}

impl WriteTrailingZeroAware for File {
    fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()> {
        <Self as std::io::Write>::write_all(self, buf.split_last().unwrap().1)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        <Self as std::io::Write>::flush(self)
    }

    fn should_prefix(&self) -> bool {
        false
    }
}

impl WriteTrailingZeroAware for Stderr {
    fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()> {
        <Self as std::io::Write>::write_all(self, buf.split_last().unwrap().1)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        <Self as std::io::Write>::flush(self)
    }

    fn should_prefix(&self) -> bool {
        true
    }
}

struct NullLog;

impl WriteTrailingZeroAware for NullLog {
    fn write_zero_aware(&mut self, _buf: &[u8]) -> std::io::Result<()> {
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn should_prefix(&self) -> bool {
        false
    }
}

#[cfg(windows)]
mod os {
    use super::WriteTrailingZeroAware;
    use std::{os::windows::prelude::AsRawHandle, ptr};
    use winapi::um::debugapi::OutputDebugStringA;

    struct OutputDebugString {}

    impl WriteTrailingZeroAware for OutputDebugString {
        fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()> {
            unsafe { OutputDebugStringA(buf.as_ptr() as *const _) };
            Ok(())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        fn should_prefix(&self) -> bool {
            true
        }
    }

    pub(crate) fn new_debug_logger() -> Box<dyn WriteTrailingZeroAware> {
        let stderr = std::io::stderr();
        let log_to_stderr = stderr.as_raw_handle() != ptr::null_mut();
        if log_to_stderr {
            Box::new(stderr)
        } else {
            Box::new(OutputDebugString {})
        }
    }
}

#[cfg(not(windows))]
mod os {
    use super::WriteTrailingZeroAware;

    pub(crate) fn new_debug_logger() -> Box<dyn WriteTrailingZeroAware> {
        Box::new(std::io::stderr())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, io, rc::Rc, str};

    use super::{FunctionLogger, LogEntry, WriteTrailingZeroAware};
    use crate::{log::CudaFunctionName, log::WriteBuffer, CUresult};

    struct FailOnNthWrite {
        fail_on: usize,
        counter: usize,
    }

    impl WriteTrailingZeroAware for FailOnNthWrite {
        fn write_zero_aware(&mut self, _: &[u8]) -> std::io::Result<()> {
            self.counter += 1;
            if self.counter >= self.fail_on {
                Err(io::Error::from_raw_os_error(4))
            } else {
                Ok(())
            }
        }

        fn flush(&mut self) -> std::io::Result<()> {
            panic!()
        }

        fn should_prefix(&self) -> bool {
            false
        }
    }

    // Custom type to not trigger trait coherence rules
    #[derive(Clone)]
    struct RcVec<T>(Rc<RefCell<Vec<T>>>);

    impl WriteTrailingZeroAware for RcVec<u8> {
        fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()> {
            let mut vec = self.0.borrow_mut();
            vec.extend_from_slice(buf.split_last().unwrap().1);
            Ok(())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }

        fn should_prefix(&self) -> bool {
            false
        }
    }

    #[test]
    fn error_in_fallible_emitter_is_handled_gracefully() {
        let result = RcVec(Rc::new(RefCell::new(Vec::<u8>::new())));
        let mut infallible_emitter = Box::new(result.clone()) as Box<dyn WriteTrailingZeroAware>;
        let mut fallible_emitter = Some(Box::new(FailOnNthWrite {
            fail_on: 1,
            counter: 0,
        }) as Box<dyn WriteTrailingZeroAware>);
        let mut write_buffer = WriteBuffer::new();
        write_buffer.unprefixed_buffer = Some(Vec::new());
        let mut log_queue = Vec::new();
        let mut func_logger = FunctionLogger {
            result: Some(CUresult::CUDA_SUCCESS),
            name: CudaFunctionName::Normal("cuInit"),
            infallible_emitter: &mut infallible_emitter,
            fallible_emitter: &mut fallible_emitter,
            arguments_writer: None,
            write_buffer: &mut write_buffer,
            log_queue: &mut log_queue,
        };

        func_logger.log(LogEntry::IoError(io::Error::from_raw_os_error(1)));
        func_logger.log(LogEntry::IoError(io::Error::from_raw_os_error(2)));
        func_logger.log(LogEntry::IoError(io::Error::from_raw_os_error(3)));
        drop(func_logger);
        drop(infallible_emitter);

        let result = result.0.borrow_mut();
        let result_str = str::from_utf8(&*result).unwrap();
        let result_lines = result_str.lines().collect::<Vec<_>>();
        assert_eq!(result_lines.len(), 5);
        assert_eq!(result_lines[0], "cuInit(...) -> CUDA_SUCCESS");
        assert!(result_lines[1].starts_with("    "));
        assert!(result_lines[2].starts_with("    "));
        assert!(result_lines[3].starts_with("    "));
        assert!(result_lines[4].starts_with("    "));
    }
}
