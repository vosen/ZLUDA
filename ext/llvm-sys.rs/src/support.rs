use super::prelude::*;

extern "C" {
    pub fn LLVMLoadLibraryPermanently(Filename: *const ::libc::c_char) -> LLVMBool;
    pub fn LLVMParseCommandLineOptions(
        argc: ::libc::c_int,
        argv: *const *const ::libc::c_char,
        Overview: *const ::libc::c_char,
    );
    /// Search all previously loaded dynamic libraries for the named symbol.
    ///
    /// Returns its address if found, otherwise null.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMSearchForAddressOfSymbol(symbolName: *const ::libc::c_char) -> *mut ::libc::c_void;
    /// Permanently add the named symbol with the provided value.
    ///
    /// Symbols added this way are searched before any libraries.
    ///
    /// Added in LLVM 3.7.
    pub fn LLVMAddSymbol(symbolName: *const ::libc::c_char, symbolValue: *mut ::libc::c_void);
}
