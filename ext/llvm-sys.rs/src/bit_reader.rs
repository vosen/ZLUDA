//! Input of the LLVM bitcode format.

use super::prelude::*;

extern "C" {
    ///// Build a module from the bitcode in the specified memory buffer.
    /////
    ///// Returns 0 on success and the generated module in `OutModule`.
    ///// Optionally returns a human-readable error message in `OutMessage`.
    //#[deprecated(since = "3.8", note = "Use LLVMParseBitcode2")]
    //pub fn LLVMParseBitcode(
    //    MemBuf: LLVMMemoryBufferRef,
    //    OutModule: *mut LLVMModuleRef,
    //    OutMessage: *mut *mut ::libc::c_char,
    //) -> LLVMBool;
    ///// Build a module from the bitcode in the specified memory buffer.
    /////
    ///// Returns the created module in OutModule, returns 0 on success.
    //pub fn LLVMParseBitcode2(
    //    MemBuf: LLVMMemoryBufferRef,
    //    OutModule: *mut LLVMModuleRef,
    //) -> LLVMBool;

    #[deprecated(since = "3.8", note = "Use LLVMParseBitcodeInContext2")]
    pub fn LLVMParseBitcodeInContext(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutModule: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    pub fn LLVMParseBitcodeInContext2(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutModule: *mut LLVMModuleRef,
    ) -> LLVMBool;

    /// Read a module from the specified path, returning a module provider
    /// performing lazy deserialization.
    ///
    /// Returns 0 on success and an optional error message.
    #[deprecated(since = "3.8", note = "Use LLVMGetBitcodeModuleInContext2")]
    pub fn LLVMGetBitcodeModuleInContext(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    /// Read a module from the specified path, returning a module provider
    /// performing lazy deserialization.
    ///
    /// Returns 0 on success.
    pub fn LLVMGetBitcodeModuleInContext2(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
    ) -> LLVMBool;

    #[deprecated(since = "3.8", note = "Use LLVMGetBitcodeModule2")]
    pub fn LLVMGetBitcodeModule(
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    /// Read a module from the specified path.
    ///
    /// Outputs a module provider which performs lazy deserialization.
    /// Returns 0 on success.
    pub fn LLVMGetBitcodeModule2(MemBuf: LLVMMemoryBufferRef, OutM: *mut LLVMModuleRef)
        -> LLVMBool;
}
