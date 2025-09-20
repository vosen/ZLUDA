use super::Settings;
use crate::trace::SendablePtr;
use crate::FnCallLog;
use crate::LogEntry;
use cuda_types::cuda::*;
use std::error::Error;
use std::ffi::c_void;
use std::ffi::NulError;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Stderr;
use std::io::Write;
use std::path::PathBuf;
use std::str::Utf8Error;

const LOG_PREFIX: &[u8] = b"[ZLUDA_TRACE] ";

pub(crate) struct Writer {
    // Fallible emitter is an optional emitter to the file system, we might lack
    // file permissions or be out of disk space
    fallible_emitter: Option<Box<dyn WriteTrailingZeroAware + Send>>,
    // This is emitter that "always works" (and if it does not, then we don't
    // care). In addition of normal logs it emits errors from fallible emitter
    infallible_emitter: Box<dyn WriteTrailingZeroAware + Send>,
    // This object could be recreated every time, but it's slightly better for performance to
    // reuse the allocations by keeping the object in globals
    write_buffer: WriteBuffer,
}

impl Writer {
    pub(crate) fn new() -> Self {
        let debug_emitter = os::new_debug_logger();
        Self {
            infallible_emitter: debug_emitter,
            fallible_emitter: None,
            write_buffer: WriteBuffer::new(),
        }
    }

    pub(crate) fn late_init(&mut self, settings: &Settings) -> Result<(), ErrorEntry> {
        self.fallible_emitter = settings
            .dump_dir
            .as_ref()
            .map(|path| {
                Ok::<_, std::io::Error>(Box::new(File::create(path.to_path_buf().join("log.txt"))?)
                    as Box<dyn WriteTrailingZeroAware + Send>)
            })
            .transpose()
            .map_err(ErrorEntry::IoError)?;
        self.write_buffer
            .init(&self.fallible_emitter, &self.infallible_emitter);
        Ok(())
    }

    pub(crate) fn write_and_flush(&mut self, log_root: &mut FnCallLog) {
        self.write_all_from_depth(0, log_root);
        self.write_buffer.finish();
        let error_from_writing_to_fallible_emitter = match self.fallible_emitter {
            Some(ref mut emitter) => self.write_buffer.send_to_and_flush(emitter),
            None => Ok(()),
        };
        if let Err(e) = error_from_writing_to_fallible_emitter {
            self.hack_squeeze_in_additional_error(ErrorEntry::IoError(e))
        }
        self.write_buffer
            .send_to_and_flush(&mut self.infallible_emitter)
            .ok();
        self.write_buffer.reset();
        log_root.reset();
    }

    fn write_all_from_depth(&mut self, depth: usize, fn_call: &FnCallLog) {
        self.write_call(depth, fn_call);
        for sub in fn_call.subcalls.iter() {
            match sub {
                LogEntry::FnCall(fn_call) => self.write_all_from_depth(depth + 1, fn_call),
                LogEntry::Error(err) => self.write_error(depth + 1, err),
            }
        }
    }

    fn write_call(&mut self, depth: usize, call: &FnCallLog) {
        self.write_buffer.start_line(depth);
        write!(self.write_buffer, "{}", call.name).ok();
        match call.args {
            Some(ref args) => {
                self.write_buffer.write_all(args).ok();
            }
            None => {
                self.write_buffer.write_all(b"(...)").ok();
            }
        }
        self.write_buffer.write_all(b" -> ").ok();
        if let Some(ref result) = call.output {
            self.write_buffer.write_all(result).ok();
        } else {
            self.write_buffer.write_all(b"UNKNOWN").ok();
        };
        self.write_buffer.end_line();
    }

    fn write_error(&mut self, depth: usize, error: &ErrorEntry) {
        self.write_buffer.start_line(depth);
        write!(self.write_buffer, "{}", error).ok();
        self.write_buffer.end_line();
    }

    fn hack_squeeze_in_additional_error(&mut self, entry: ErrorEntry) {
        self.write_buffer.undo_finish();
        write!(self.write_buffer, "    {}", entry).ok();
        self.write_buffer.end_line();
        self.write_buffer.finish();
    }
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
        fallible_emitter: &Option<Box<dyn WriteTrailingZeroAware + Send>>,
        infallible_emitter: &Box<dyn WriteTrailingZeroAware + Send>,
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

    fn start_line(&mut self, depth: usize) {
        if let Some(buffer) = &mut self.prefixed_buffer {
            buffer.extend_from_slice(LOG_PREFIX);
        }
        if depth == 0 {
            return;
        }
        for buffer in self.all_buffers() {
            buffer.extend(std::iter::repeat_n(b' ', depth * 4));
        }
    }

    fn end_line(&mut self) {
        for buffer in self.all_buffers() {
            buffer.push(b'\n');
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

    fn send_to_and_flush(
        &self,
        log_emitter: &mut Box<dyn WriteTrailingZeroAware + Send>,
    ) -> Result<(), io::Error> {
        if log_emitter.should_prefix() {
            log_emitter.write_zero_aware(
                &*self
                    .prefixed_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
            )?;
        } else {
            log_emitter.write_zero_aware(
                &*self
                    .unprefixed_buffer
                    .as_ref()
                    .unwrap_or_else(|| unreachable!()),
            )?;
        }
        log_emitter.flush()
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

#[derive(Clone)]
pub(crate) enum CudaFunctionName {
    Normal(&'static str),
    Dark { guid: CUuuid, index: usize },
}

impl Display for CudaFunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CudaFunctionName::Normal(fn_) => f.write_str(fn_),
            CudaFunctionName::Dark { guid, index } => {
                match ::dark_api::cuda::guid_to_name(guid, *index) {
                    Some((name, fn_)) => match fn_ {
                        Some(fn_) => write!(f, "{{{name}}}::{fn_}"),
                        None => write!(f, "{{{name}}}::{index}"),
                    },
                    None => {
                        let mut temp = Vec::new();
                        format::CudaDisplay::write(guid, "", 0, &mut temp)
                            .map_err(|_| std::fmt::Error::default())?;
                        let temp = String::from_utf8_lossy(&*temp);
                        write!(f, "{temp}::{index}")
                    }
                }
            }
        }
    }
}

pub(crate) enum ErrorEntry {
    IoError(io::Error),
    CreatedDumpDirectory(PathBuf),
    ErrorBox(Box<dyn Error>),
    UnsupportedModule {
        handle: *mut c_void,
        raw_image: *const c_void,
        kind: &'static str,
    },
    FunctionNotFound(CudaFunctionName),
    Utf8Error(Utf8Error),
    NulInsideModuleText(NulError),
    ModuleParsingError(String),
    Lz4DecompressionFailure,
    ZstdDecompressionFailure(usize),
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
    InvalidEnvVar {
        var: &'static str,
        pattern: &'static str,
        value: String,
    },
    UnexpectedExportTableSize {
        expected: usize,
        computed: usize,
    },
    IntegrityCheck {
        original: [u64; 2],
        overriden: [u64; 2],
    },
    NullPointer(&'static str),
    SavedModule(String),
    UnknownFunctionHandle(CUfunction),
    UnknownLibrary(CUfunction, SendablePtr),
    UnknownFunction(CUfunction, SendablePtr, String),
    CudaError(Option<CUerror>),
}

unsafe impl Send for ErrorEntry {}
unsafe impl Sync for ErrorEntry {}

impl From<dark_api::fatbin::ParseError> for ErrorEntry {
    fn from(e: dark_api::fatbin::ParseError) -> Self {
        match e {
            dark_api::fatbin::ParseError::NullPointer(s) => ErrorEntry::NullPointer(s),
            dark_api::fatbin::ParseError::UnexpectedBinaryField {
                field_name,
                observed,
                expected,
            } => ErrorEntry::UnexpectedBinaryField {
                field_name,
                observed: UInt::from(observed),
                expected: expected.into_iter().map(UInt::from).collect(),
            },
        }
    }
}

impl From<dark_api::fatbin::FatbinError> for ErrorEntry {
    fn from(e: dark_api::fatbin::FatbinError) -> Self {
        match e {
            dark_api::fatbin::FatbinError::ParseFailure(parse_error) => parse_error.into(),
            dark_api::fatbin::FatbinError::Lz4DecompressionFailure => {
                ErrorEntry::Lz4DecompressionFailure
            }
            dark_api::fatbin::FatbinError::ZstdDecompressionFailure(c) => {
                ErrorEntry::ZstdDecompressionFailure(c)
            }
        }
    }
}

impl Display for ErrorEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorEntry::IoError(e) => e.fmt(f),
            ErrorEntry::CreatedDumpDirectory(dir) => {
                                                                        write!(
                                                                            f,
                                                                            "Created trace directory {} ",
                                                                            dir.as_os_str().to_string_lossy()
                                                                        )
                                                                    }
            ErrorEntry::ErrorBox(e) => e.fmt(f),
            ErrorEntry::UnsupportedModule {
                                                                        handle,
                                                                        raw_image,
                                                                        kind,
                                                                    } => {
                                                                        write!(
                                                                            f,
                                                                            "Unsupported {} module {:p} loaded from module image {:p}",
                                                                            kind, handle, raw_image
                                                                        )
                                                                    }
            ErrorEntry::Utf8Error(e) => e.fmt(f),
            ErrorEntry::ModuleParsingError(file_name) => {
                                                                        write!(
                                                                            f,
                                                                            "Error parsing module, log has been written to {}",
                                                                            file_name
                                                                        )
                                                                    }
            ErrorEntry::NulInsideModuleText(e) => e.fmt(f),
            ErrorEntry::Lz4DecompressionFailure => write!(f, "LZ4 decompression failure"),
            ErrorEntry::ZstdDecompressionFailure(err_code) => write!(f, "Zstd decompression failure: {}", zstd_safe::get_error_name(*err_code)),
            ErrorEntry::UnexpectedBinaryField {
                                                                        field_name,
                                                                        expected,
                                                                        observed,
                                                                    } => write!(
                                                                        f,
                                                                        "Unexpected field {}. Expected one of: [{}], observed: {}",
                                                                        field_name,
                                                                        expected
                                                                            .iter()
                                                                            .map(|x| x.to_string())
                                                                            .collect::<Vec<_>>()
                                                                            .join(", "),
                                                                        observed
                                                                    ),
            ErrorEntry::UnexpectedArgument {
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
            ErrorEntry::InvalidEnvVar {
                                                                        var,
                                                                        pattern,
                                                                        value,
                                                                    } => write!(
                                                                        f,
                                                                        "Unexpected value of environment variable {var}. Expected pattern: {pattern}, got value: {value}"
                                                                    ),
            ErrorEntry::FunctionNotFound(cuda_function_name) => write!(
                                                                f,
                                                                "No function {cuda_function_name} in the underlying library"
                                                            ),
            ErrorEntry::UnexpectedExportTableSize { expected, computed } => {
                                                        write!(f, "Table length mismatch. Expected: {expected}, got: {computed}")
                                                    }
            ErrorEntry::IntegrityCheck { original, overriden } => {
                                                        write!(f, "Overriding integrity check hash. Original: {original:?}, overriden: {overriden:?}")
                                                    }
            ErrorEntry::NullPointer(type_) => {
                                                write!(f, "Null pointer of type {type_} encountered")
                                            }
            ErrorEntry::SavedModule(file) => write!(f, "Saved module to {file}"),
            ErrorEntry::UnknownFunctionHandle(cuda_function_name) => {
                        write!(f, "Function with unknown provenance: {cuda_function_name:p}")
                    }
            ErrorEntry::UnknownLibrary(cuda_function_name, owner) => {
                        write!(f, "Function with unknown provenance: {cuda_function_name:p}, owner: {owner:p}")
                    }
            ErrorEntry::UnknownFunction(cuda_function_name, owner, name) => {
                        write!(f, "Function with unknown provenance: {cuda_function_name:p}, owner: {owner:p}, name: {name}")
                    }
            ErrorEntry::CudaError(cuerror) => {
                let cuerror = cuerror.map(|e| e.0);
                write!(f, "CUDA error encountered: {cuerror:#?}")
            },
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum UInt {
    U16(u16),
    U32(u32),
    USize(usize),
}

impl From<u16> for UInt {
    fn from(value: u16) -> Self {
        UInt::U16(value)
    }
}

impl From<u32> for UInt {
    fn from(value: u32) -> Self {
        UInt::U32(value)
    }
}

impl From<usize> for UInt {
    fn from(value: usize) -> Self {
        UInt::USize(value)
    }
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

    pub(crate) fn new_debug_logger() -> Box<dyn WriteTrailingZeroAware + Send> {
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

    pub(crate) fn new_debug_logger() -> Box<dyn WriteTrailingZeroAware + Send> {
        Box::new(std::io::stderr())
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorEntry, FnCallLog, WriteTrailingZeroAware};
    use crate::{
        log::{CudaFunctionName, WriteBuffer},
        FnCallLogStack, OuterCallGuard,
    };
    use std::{
        cell::RefCell,
        io, str,
        sync::{Arc, Mutex},
    };

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
    struct ArcVec<T>(Arc<Mutex<Vec<T>>>);

    impl WriteTrailingZeroAware for ArcVec<u8> {
        fn write_zero_aware(&mut self, buf: &[u8]) -> std::io::Result<()> {
            let mut vec = self.0.lock().unwrap();
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
    // TODO: fix this, it should use drop guard for testing.
    // Previously FnCallLog would implement Drop and write to the log
    fn error_in_fallible_emitter_is_handled_gracefully() {
        let result = ArcVec(Arc::new(Mutex::new(Vec::<u8>::new())));
        let infallible_emitter = Box::new(result.clone()) as Box<dyn WriteTrailingZeroAware + Send>;
        let fallible_emitter = Some(Box::new(FailOnNthWrite {
            fail_on: 1,
            counter: 0,
        }) as Box<dyn WriteTrailingZeroAware + Send>);
        let mut write_buffer = WriteBuffer::new();
        write_buffer.unprefixed_buffer = Some(Vec::new());
        let mut writer = super::Writer {
            fallible_emitter,
            infallible_emitter,
            write_buffer,
        };
        let func_logger = FnCallLog {
            name: CudaFunctionName::Normal("cuInit"),
            args: None,
            output: None,
            subcalls: Vec::new(),
        };
        let log_root = FnCallLogStack {
            depth: 1,
            log_root: func_logger,
        };
        let log_root = RefCell::new(log_root);
        let drop_guard = OuterCallGuard {
            writer: &mut writer,
            log_root: &log_root,
        };

        {
            log_root
                .borrow_mut()
                .log_root
                .log(ErrorEntry::IoError(io::Error::from_raw_os_error(1)));
            log_root
                .borrow_mut()
                .log_root
                .log(ErrorEntry::IoError(io::Error::from_raw_os_error(2)));
            log_root
                .borrow_mut()
                .log_root
                .log(ErrorEntry::IoError(io::Error::from_raw_os_error(3)));
        }
        drop(drop_guard);

        let result = result.0.lock().unwrap();
        let result_str = str::from_utf8(&*result).unwrap();
        let result_lines = result_str.lines().collect::<Vec<_>>();
        assert_eq!(result_lines.len(), 5);
        assert_eq!(result_lines[0], "cuInit(...) -> UNKNOWN");
        assert!(result_lines[1].starts_with("    "));
        assert!(result_lines[2].starts_with("    "));
        assert!(result_lines[3].starts_with("    "));
        assert!(result_lines[4].starts_with("    "));
    }
}
