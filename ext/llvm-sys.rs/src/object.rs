//! Object file reading and writing

use super::prelude::*;

#[derive(Debug)]
pub enum LLVMOpaqueSectionIterator {}

pub type LLVMSectionIteratorRef = *mut LLVMOpaqueSectionIterator;

#[derive(Debug)]
pub enum LLVMOpaqueSymbolIterator {}

pub type LLVMSymbolIteratorRef = *mut LLVMOpaqueSymbolIterator;

#[derive(Debug)]
pub enum LLVMOpaqueRelocationIterator {}

pub type LLVMRelocationIteratorRef = *mut LLVMOpaqueRelocationIterator;

#[derive(Debug)]
pub enum LLVMOpaqueBinary {}

pub type LLVMBinaryRef = *mut LLVMOpaqueBinary;

#[repr(C)]
#[derive(Debug)]
pub enum LLVMBinaryType {
    /// Archive file
    LLVMBinaryTypeArchive,
    /// Mach-O Universal Binary file
    LLVMBinaryTypeMachOUniversalBinary,
    /// COFF Import file
    LLVMBinaryTypeCOFFImportFile,
    /// LLVM IR
    LLVMBinaryTypeIR,
    /// Windows resource (.res) file
    LLVMBinaryTypeWinRes,
    /// COFF Object file
    LLVMBinaryTypeCOFF,
    /// ELF 32-bit, little endian
    LLVMBinaryTypeELF32L,
    /// ELF 32-bit, big endian
    LLVMBinaryTypeELF32B,
    /// ELF 64-bit, little endian
    LLVMBinaryTypeELF64L,
    /// ELF 64-bit, big endian
    LLVMBinaryTypeELF64B,
    /// MachO 32-bit, little endian
    LLVMBinaryTypeMachO32L,
    /// MachO 32-bit, big endian
    LLVMBinaryTypeMachO32B,
    /// MachO 64-bit, little endian
    LLVMBinaryTypeMachO64L,
    /// MachO 64-bit, big endian
    LLVMBinaryTypeMachO64B,
    /// Web assembly
    LLVMBinaryTypeWasm,
    /// Offloading fatbinary
    LLVMBinaryTypeOffload,
}

#[deprecated(since = "LLVM 9.0")]
pub enum LLVMOpaqueObjectFile {}

#[allow(deprecated)]
#[deprecated(since = "LLVM 9.0")]
pub type LLVMObjectFileRef = *mut LLVMOpaqueObjectFile;

extern "C" {
    /// Create a binary file from the given memory buffer.
    pub fn LLVMCreateBinary(
        MemBuf: LLVMMemoryBufferRef,
        Context: LLVMContextRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBinaryRef;
    /// Dispose of a binary file
    pub fn LLVMDisposeBinary(BR: LLVMBinaryRef);

    pub fn LLVMBinaryCopyMemoryBuffer(BR: LLVMBinaryRef) -> LLVMMemoryBufferRef;
    pub fn LLVMBinaryGetType(BR: LLVMBinaryRef) -> LLVMBinaryType;
    pub fn LLVMMachOUniversalBinaryCopyObjectForArch(
        BR: LLVMBinaryRef,
        Arch: *const ::libc::c_char,
        ArchLen: ::libc::size_t,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBinaryRef;

    pub fn LLVMObjectFileCopySectionIterator(BR: LLVMBinaryRef) -> LLVMSectionIteratorRef;
    pub fn LLVMObjectFileIsSectionIteratorAtEnd(
        BR: LLVMBinaryRef,
        SI: LLVMSectionIteratorRef,
    ) -> LLVMBool;
    pub fn LLVMObjectFileCopySymbolIterator(BR: LLVMBinaryRef) -> LLVMSymbolIteratorRef;
    pub fn LLVMObjectFileIsSymbolIteratorAtEnd(
        BR: LLVMBinaryRef,
        SI: LLVMSymbolIteratorRef,
    ) -> LLVMBool;
    pub fn LLVMDisposeSectionIterator(SI: LLVMSectionIteratorRef);

    pub fn LLVMMoveToNextSection(SI: LLVMSectionIteratorRef);
    pub fn LLVMMoveToContainingSection(Sect: LLVMSectionIteratorRef, Sym: LLVMSymbolIteratorRef);
    pub fn LLVMDisposeSymbolIterator(SI: LLVMSymbolIteratorRef);
    pub fn LLVMMoveToNextSymbol(SI: LLVMSymbolIteratorRef);
    pub fn LLVMGetSectionName(SI: LLVMSectionIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSectionSize(SI: LLVMSectionIteratorRef) -> u64;
    pub fn LLVMGetSectionContents(SI: LLVMSectionIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSectionAddress(SI: LLVMSectionIteratorRef) -> u64;
    pub fn LLVMGetSectionContainsSymbol(
        SI: LLVMSectionIteratorRef,
        Sym: LLVMSymbolIteratorRef,
    ) -> LLVMBool;
    pub fn LLVMGetRelocations(Section: LLVMSectionIteratorRef) -> LLVMRelocationIteratorRef;
    pub fn LLVMDisposeRelocationIterator(RI: LLVMRelocationIteratorRef);
    pub fn LLVMIsRelocationIteratorAtEnd(
        Section: LLVMSectionIteratorRef,
        RI: LLVMRelocationIteratorRef,
    ) -> LLVMBool;
    pub fn LLVMMoveToNextRelocation(RI: LLVMRelocationIteratorRef);
    pub fn LLVMGetSymbolName(SI: LLVMSymbolIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetSymbolAddress(SI: LLVMSymbolIteratorRef) -> u64;
    pub fn LLVMGetSymbolSize(SI: LLVMSymbolIteratorRef) -> u64;
    pub fn LLVMGetRelocationOffset(RI: LLVMRelocationIteratorRef) -> u64;
    pub fn LLVMGetRelocationSymbol(RI: LLVMRelocationIteratorRef) -> LLVMSymbolIteratorRef;
    pub fn LLVMGetRelocationType(RI: LLVMRelocationIteratorRef) -> u64;
    pub fn LLVMGetRelocationTypeName(RI: LLVMRelocationIteratorRef) -> *const ::libc::c_char;
    pub fn LLVMGetRelocationValueString(RI: LLVMRelocationIteratorRef) -> *const ::libc::c_char;

    #[allow(deprecated)]
    #[deprecated(since = "LLVM 9.0", note = "Use LLVMCreateBinary instead")]
    pub fn LLVMCreateObjectFile(MemBuf: LLVMMemoryBufferRef) -> LLVMObjectFileRef;
    #[allow(deprecated)]
    #[deprecated(since = "LLVM 9.0", note = "Use LLVMDisposeBinary instead")]
    pub fn LLVMDisposeObjectFile(ObjectFile: LLVMObjectFileRef);
    #[allow(deprecated)]
    #[deprecated(
        since = "LLVM 9.0",
        note = "Use LLVMObjectFileCopySectionIterator instead"
    )]
    pub fn LLVMGetSections(ObjectFile: LLVMObjectFileRef) -> LLVMSectionIteratorRef;
    #[allow(deprecated)]
    #[deprecated(
        since = "LLVM 9.0",
        note = "Use LLVMObjectFileIsSectionIteratorAtEnd instead"
    )]
    pub fn LLVMIsSectionIteratorAtEnd(
        ObjectFile: LLVMObjectFileRef,
        SI: LLVMSectionIteratorRef,
    ) -> LLVMBool;
    #[allow(deprecated)]
    #[deprecated(
        since = "LLVM 9.0",
        note = "Use LLVMObjectFileCopySymbolIterator instead"
    )]
    pub fn LLVMGetSymbols(ObjectFile: LLVMObjectFileRef) -> LLVMSymbolIteratorRef;
    #[allow(deprecated)]
    #[deprecated(
        since = "LLVM 9.0",
        note = "Use LLVMObjectFileIsSymbolIteratorAtEnd instead"
    )]
    pub fn LLVMIsSymbolIteratorAtEnd(
        ObjectFile: LLVMObjectFileRef,
        SI: LLVMSymbolIteratorRef,
    ) -> LLVMBool;
}
