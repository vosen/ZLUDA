//! Output of the LLVM bitcode format.

use super::prelude::*;

extern "C" {
    /// Write a module to the specified path.
    ///
    /// Returns 0 on success.
    pub fn LLVMWriteBitcodeToFile(M: LLVMModuleRef, Path: *const ::libc::c_char) -> ::libc::c_int;
    /// Write a module to an open file descriptor.
    ///
    /// Returns 0 on success.
    pub fn LLVMWriteBitcodeToFD(
        M: LLVMModuleRef,
        FD: ::libc::c_int,
        ShouldClose: ::libc::c_int,
        Unbuffered: ::libc::c_int,
    ) -> ::libc::c_int;
    /// Deprecated: use LLVMWriteBitcodeToFD
    pub fn LLVMWriteBitcodeToFileHandle(M: LLVMModuleRef, Handle: ::libc::c_int) -> ::libc::c_int;
    /// Writes a module to a new memory buffer.
    pub fn LLVMWriteBitcodeToMemoryBuffer(M: LLVMModuleRef) -> LLVMMemoryBufferRef;
}
