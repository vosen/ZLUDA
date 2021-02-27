//! The IR reader

use super::prelude::*;

extern "C" {
    /// Read LLVM IR from a memory buffer and convert it to an in-memory Module.
    ///
    /// Returns 0 on success, and an optional human-readable description of any
    /// errors that occurred.
    pub fn LLVMParseIRInContext(
        ContextRef: LLVMContextRef,
        MemBuf: LLVMMemoryBufferRef,
        OutM: *mut LLVMModuleRef,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
}
