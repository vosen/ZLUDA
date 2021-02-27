//! Remark diagnostics library.
use prelude::LLVMBool;

#[repr(C)]
pub enum LLVMRemarkType {
    LLVMRemarkTypeUnknown,
    LLVMRemarkTypePassed,
    LLVMRemarkTypeMissed,
    LLVMRemarkTypeAnalysis,
    LLVMRemarkTypeAnalysisFPCommute,
    LLVMRemarkTypeAnalysisAliasing,
    LLVMRemarkTypeFailure,
}

pub enum LLVMRemarkOpaqueString {}

/// String containing a buffer and a length. The buffer is not guaranteed to be zero-terminated.
pub type LLVMRemarkStringRef = *mut LLVMRemarkOpaqueString;

extern "C" {
    /// Returns the buffer holding the string.
    pub fn LLVMRemarkStringGetData(String: LLVMRemarkStringRef) -> *const ::libc::c_char;

    /// Returns the size of the string.
    pub fn LLVMRemarkStringGetLen(String: LLVMRemarkStringRef) -> u32;
}

pub enum LLVMRemarkOpaqueDebugLoc {}

/// DebugLoc containing File, Line and Column.
pub type LLVMRemarkDebugLocRef = *mut LLVMRemarkOpaqueDebugLoc;

extern "C" {
    /// Return the path to the source file for a debug location.
    pub fn LLVMRemarkDebugLocGetSourceFilePath(DL: LLVMRemarkDebugLocRef) -> LLVMRemarkStringRef;

    /// Return the line in the source file for a debug location.
    pub fn LLVMRemarkDebugLocGetSourceLine(DL: LLVMRemarkDebugLocRef) -> u32;

    /// Return the column in the source file for a debug location.
    pub fn LLVMRemarkDebugLocGetSourceColumn(DL: LLVMRemarkDebugLocRef) -> u32;
}

pub enum LLVMRemarkOpaqueArg {}

/// Element of the "Args" list. The key might give more information about what
/// the semantics of the value are, e.g. "Callee" will tell you that the value
/// is a symbol that names a function.
pub type LLVMRemarkArgRef = *mut LLVMRemarkOpaqueArg;

extern "C" {
    /// Returns the key of an argument. The key defines what the value is, and the
    /// same key can appear multiple times in the list of arguments.
    pub fn LLVMRemarkArgGetKey(Arg: LLVMRemarkArgRef) -> LLVMRemarkStringRef;

    /// Returns the value of an argument. This is a string that can contain newlines.
    pub fn LLVMRemarkArgGetValue(Arg: LLVMRemarkArgRef) -> LLVMRemarkStringRef;

    /// Returns the debug location that is attached to the value of this argument.
    pub fn LLVMRemarkArgGetDebugLoc(Arg: LLVMRemarkArgRef) -> LLVMRemarkDebugLocRef;
}

pub enum LLVMRemarkOpaqueEntry {}
/// A remark emitted by the compiler.
pub type LLVMRemarkEntryRef = *mut LLVMRemarkOpaqueEntry;

extern "C" {
    /// Free the resources used by the remark entry.
    pub fn LLVMRemarkEntryDispose(Remark: LLVMRemarkEntryRef);

    /// The type of the remark. For example, it can allow users to only keep the
    /// missed optimizations from the compiler.
    pub fn LLVMRemarkEntryGetType(Remark: LLVMRemarkEntryRef) -> LLVMRemarkType;

    /// Get the name of the pass that emitted this remark.
    pub fn LLVMRemarkEntryGetPassName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

    /// Get an identifier of the remark.
    pub fn LLVMRemarkEntryGetRemarkName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

    /// Get the name of the function being processed when the remark was emitted.
    pub fn LLVMRemarkEntryGetFunctionName(Remark: LLVMRemarkEntryRef) -> LLVMRemarkStringRef;

    /// Returns the debug location that is attached to this remark.
    pub fn LLVMRemarkEntryGetDebugLoc(Remark: LLVMRemarkEntryRef) -> LLVMRemarkDebugLocRef;

    /// Return the hotness of the remark.
    pub fn LLVMRemarkEntryGetHotness(Remark: LLVMRemarkEntryRef) -> u64;

    /// The number of arguments the remark holds.
    pub fn LLVMRemarkEntryGetNumArgs(Remark: LLVMRemarkEntryRef) -> u32;

    /// Get a new iterator to iterate over a remark's argument.
    pub fn LLVMRemarkEntryGetFirstArg(Remark: LLVMRemarkEntryRef) -> LLVMRemarkArgRef;

    /// Get the next argument in Remark from the position of It.
    pub fn LLVMRemarkEntryGetNextArg(
        It: LLVMRemarkArgRef,
        Remark: LLVMRemarkEntryRef,
    ) -> LLVMRemarkArgRef;
}

pub enum LLVMRemarkOpaqueParser {}
pub type LLVMRemarkParserRef = *mut LLVMRemarkOpaqueParser;

extern "C" {
    /// Creates a remark parser that can be used to parse the buffer located in
    /// Buf of size Size bytes.
    pub fn LLVMRemarkParserCreateYAML(Buf: *const ::libc::c_void, Size: u64)
        -> LLVMRemarkParserRef;

    pub fn LLVMRemarkParserCreateBitstream(
        Buf: *const ::libc::c_void,
        Size: u64,
    ) -> LLVMRemarkParserRef;

    /// Returns the next remark in the file.
    pub fn LLVMRemarkParserGetNext(Parser: LLVMRemarkParserRef) -> LLVMRemarkEntryRef;

    /// Returns `1` if the parser encountered an error while parsing the buffer.
    pub fn LLVMRemarkParserHasError(Parser: LLVMRemarkParserRef) -> LLVMBool;

    /// Returns a null-terminated string containing an error message.
    pub fn LLVMRemarkParserGetErrorMessage(Parser: LLVMRemarkParserRef) -> *const ::libc::c_char;

    pub fn LLVMRemarkParserDispose(Parser: LLVMRemarkParserRef);
}

pub const REMARKS_API_VERSION: u32 = 1;

extern "C" {
    /// Returns the version of the remarks library.
    pub fn LLVMRemarkVersion() -> u32;
}
