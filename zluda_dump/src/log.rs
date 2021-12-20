use crate::cuda::CUmodule;
use crate::cuda::CUuuid;
use crate::format;
use crate::format::FormatCudaObject;

use super::CUresult;
use super::Settings;
use std::borrow::Cow;
use std::error::Error;
use std::ffi::c_void;
use std::ffi::FromBytesWithNulError;
use std::ffi::NulError;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Stderr;
use std::io::Write;
use std::path::PathBuf;
use std::str::Utf8Error;

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
        let debug_emitter = os::new_debug_logger();
        Factory {
            infallible_emitter: debug_emitter,
            fallible_emitter: None,
            write_buffer: WriteBuffer::new(),
            log_queue: Vec::new(),
        }
    }

    fn initalize_fallible_emitter(
        settings: &Settings,
    ) -> std::io::Result<Option<Box<dyn WriteTrailingZeroAware>>> {
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
    ) -> (FunctionLogger, Settings) {
        let mut first_logger = self.get_logger(func);
        let settings = Settings::read_and_init(&mut first_logger);
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

    pub(crate) fn get_logger(&mut self, func: &'static str) -> FunctionLogger {
        FunctionLogger {
            result: None,
            name: CudaFunctionName::Normal(func),
            fallible_emitter: &mut self.fallible_emitter,
            infallible_emitter: &mut self.infallible_emitter,
            write_buffer: &mut self.write_buffer,
            log_queue: &mut self.log_queue,
            finished_writing_args: false,
            args_to_write: 0,
        }
    }

    pub(crate) fn get_logger_dark_api(&mut self, guid: CUuuid, index: usize) -> FunctionLogger {
        FunctionLogger {
            result: None,
            name: CudaFunctionName::Dark { guid, index },
            fallible_emitter: &mut self.fallible_emitter,
            infallible_emitter: &mut self.infallible_emitter,
            write_buffer: &mut self.write_buffer,
            log_queue: &mut self.log_queue,
            finished_writing_args: false,
            args_to_write: 0,
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
    write_buffer: &'a mut WriteBuffer,
    log_queue: &'a mut Vec<LogEntry>,
    args_to_write: usize,
    finished_writing_args: bool,
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

    fn flush_log_queue_to_write_buffer(&mut self) {
        // TODO: remove this once everything has been converted to dtailed logging
        if !self.finished_writing_args {
            self.begin_writing_arguments(0);
            self.write_buffer.write("...) -> ");
        }
        if let Some(result) = self.result {
            match format::stringify_CUresult(result) {
                Some(text) => self.write_buffer.write(text),
                None => write!(self.write_buffer, "{}", result.0).unwrap(),
            }
        } else {
            self.write_buffer.write("(UNKNOWN)");
        };
        self.write_buffer.end_line();
        for entry in self.log_queue.iter() {
            write!(self.write_buffer, "    {}", entry).unwrap_or_else(|_| unreachable!());
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

    pub(crate) fn begin_writing_arguments(&mut self, len: usize) {
        self.args_to_write = len;
        match self.name {
            CudaFunctionName::Normal(fn_name) => self.write_buffer.write(fn_name),
            CudaFunctionName::Dark { guid, index } => {
                guid.write_post_execution(CUresult::CUDA_SUCCESS, &mut self.write_buffer);
                write!(&mut self.write_buffer, "::{}", index).ok();
            }
        }
        self.write_buffer.write("(")
    }

    pub(crate) fn write_single_argument<'x>(
        &mut self,
        result: CUresult,
        arg: impl FormatCudaObject,
    ) {
        self.args_to_write -= 1;
        arg.write_post_execution(result, self.write_buffer);
        if self.args_to_write != 0 {
            self.write_buffer.write(", ")
        }
    }

    pub(crate) fn end_writing_arguments(&mut self) {
        self.write_buffer.write(") -> ");
        self.finished_writing_args = true;
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
    ErrorBox(Box<dyn Error>),
    UnsupportedModule {
        module: CUmodule,
        raw_image: *const c_void,
        kind: &'static str,
    },
    MalformedFunctionName(Utf8Error),
    FunctionParameter {
        name: &'static str,
        value: String,
    },
    MalformedModulePath(Utf8Error),
    NonUtf8ModuleText(Utf8Error),
    NulInsideModuleText(NulError),
    ModuleParsingError(String),
    Lz4DecompressionFailure,
    UnknownExportTableFn,
    UnknownModule(CUmodule),
    UnexpectedArgument {
        arg_name: &'static str,
        expected: Vec<UInt>,
        observed: UInt,
    },
    UnexpectedBinaryField {
        field_name: &'static str,
        expected: Vec<UInt>,
        observed: UInt,
    },
}

impl Display for LogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogEntry::IoError(e) => e.fmt(f),
            LogEntry::CreatedDumpDirectory(dir) => {
                write!(
                    f,
                    "Created dump directory {} ",
                    dir.as_os_str().to_string_lossy()
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
                    kind, module, raw_image
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
            LogEntry::NulInsideModuleText(e) => e.fmt(f),
            LogEntry::Lz4DecompressionFailure => write!(f, "LZ4 decompression failure"),
            LogEntry::UnknownExportTableFn => write!(f, "Unknown export table function"),
            LogEntry::UnknownModule(hmod) => write!(f, "Unknown module {:?}", hmod),
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
            LogEntry::UnexpectedArgument {
                arg_name,
                expected,
                observed,
            } => write!(
                f,
                "Unexpected argument {}. Expected one of: {{{}}}, observed: {}",
                arg_name,
                expected
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
                observed
            ),
            LogEntry::MalformedFunctionName(e) => e.fmt(f),
            LogEntry::FunctionParameter { name, value } => write!(f, "{}: {}", name, value),
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum UInt {
    U16(u16),
    U32(u32),
    USize(usize),
}

impl Display for UInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UInt::U16(x) => write!(f, "{:#x}", x),
            UInt::U32(x) => write!(f, "{:#x}", x),
            UInt::USize(x) => write!(f, "{:#x}", x),
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
    use std::{borrow::Cow, cell::RefCell, io, rc::Rc, str};

    use super::{FunctionLogger, LogEntry, WriteTrailingZeroAware};
    use crate::{log::WriteBuffer, CUresult};

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
            name: Cow::Borrowed("cuInit"),
            infallible_emitter: &mut infallible_emitter,
            fallible_emitter: &mut fallible_emitter,
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
        assert_eq!(result_lines[0], "cuInit(...) -> 0x0");
        assert!(result_lines[1].starts_with("    "));
        assert!(result_lines[2].starts_with("    "));
        assert!(result_lines[3].starts_with("    "));
        assert!(result_lines[4].starts_with("    "));
    }
}
