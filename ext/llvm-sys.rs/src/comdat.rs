//! COMDAT
use super::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMComdatSelectionKind {
    /// The linker may choose any COMDAT.
    LLVMAnyComdatSelectionKind,
    /// The data referenced by the COMDAT must be the same.
    LLVMExactMatchComdatSelectionKind,
    /// The linker will choose the largest COMDAT.
    LLVMLargestComdatSelectionKind,
    /// No deduplication is performed.
    LLVMNoDuplicatesComdatSelectionKind,
    /// The data referenced by the COMDAT must be the same size.
    LLVMSameSizeComdatSelectionKind,
}

extern "C" {
    /// Return the Comdat in the module with the specified name. It is created if it didn't already exist.
    pub fn LLVMGetOrInsertComdat(M: LLVMModuleRef, Name: *const ::libc::c_char) -> LLVMComdatRef;

    /// Get the Comdat assigned to the given global object.
    pub fn LLVMGetComdat(V: LLVMValueRef) -> LLVMComdatRef;

    /// Assign the Comdat to the given global object.
    pub fn LLVMSetComdat(V: LLVMValueRef, C: LLVMComdatRef);

    /// Get the conflict resolution selection kind for the Comdat.
    pub fn LLVMGetComdatSelectionKind(C: LLVMComdatRef) -> LLVMComdatSelectionKind;

    /// Set the conflict resolution selection kind for the Comdat.
    pub fn LLVMSetComdatSelectionKind(C: LLVMComdatRef, Kind: LLVMComdatSelectionKind);
}
