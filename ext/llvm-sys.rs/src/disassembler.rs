//! A disassembler library.

#![allow(non_upper_case_globals, non_snake_case)]

#[derive(Debug)]
pub enum LLVMOpaqueDisasmContext {}

pub type LLVMDisasmContextRef = *mut LLVMOpaqueDisasmContext;

pub type LLVMOpInfoCallback = Option<
    extern "C" fn(
        DisInfo: *mut ::libc::c_void,
        PC: u64,
        Offset: u64,
        OpSize: u64,
        InstSize: u64,
        TagType: ::libc::c_int,
        TagBuf: *mut ::libc::c_void,
    ) -> ::libc::c_int,
>;

#[repr(C)]
#[derive(Debug)]
pub struct LLVMOpInfoSymbol1 {
    /// 1 if this symbol is present.
    pub Present: u64,
    /// Symbol name if not NULL.
    pub Name: *const ::libc::c_char,
    /// Symbol value if name is NULL.
    pub Value: u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct Struct_LLVMOpInfo1 {
    pub AddSymbol: LLVMOpInfoSymbol1,
    pub SubtractSymbol: LLVMOpInfoSymbol1,
    pub Value: u64,
    pub VariantKind: u64,
}

pub const LLVMDisassembler_VariantKind_None: u64 = 0;
pub const LLVMDisassembler_VariantKind_ARM_HI16: u64 = 1;
pub const LLVMDisassembler_VariantKind_ARM_LO16: u64 = 2;
pub const LLVMDisassembler_VariantKind_ARM64_PAGE: u64 = 1;
pub const LLVMDisassembler_VariantKind_ARM64_PAGEOFF: u64 = 2;
pub const LLVMDisassembler_VariantKind_ARM64_GOTPAGE: u64 = 3;
pub const LLVMDisassembler_VariantKind_ARM64_GOTPAGEOFF: u64 = 4;
pub const LLVMDisassembler_VariantKind_ARM64_TLVP: u64 = 5;
pub const LLVMDisassembler_VariantKind_ARM64_TLVOFF: u64 = 6;

/// No input reference type or no output reference type.
pub const LLVMDisassembler_ReferenceType_InOut_None: u64 = 0;

/// The input reference is from a branch instruction.
pub const LLVMDisassembler_ReferenceType_In_Branch: u64 = 1;
/// The input reference is from a PC relative load instruction.
pub const LLVMDisassembler_ReferenceType_In_PCrel_Load: u64 = 2;

/// The input reference is from an ARM64::ADRP instruction.
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADRP: u64 = 0x100000001;
/// The input reference is from an ARM64::ADDXri instruction.
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADDXri: u64 = 0x100000002;
/// The input reference is from an ARM64::LDRXui instruction.
pub const LLVMDisassembler_ReferenceType_In_ARM64_LDRXui: u64 = 0x100000003;
/// The input reference is from an ARM64::LDRXl instruction.
pub const LLVMDisassembler_ReferenceType_In_ARM64_LDRXl: u64 = 0x100000004;
/// The input reference is from an ARM64::ADR instruction.
pub const LLVMDisassembler_ReferenceType_In_ARM64_ADR: u64 = 0x100000005;

/// The output reference is to as symbol stub.
pub const LLVMDisassembler_ReferenceType_Out_SymbolStub: u64 = 1;
/// The output reference is to a symbol address in a literal pool.
pub const LLVMDisassembler_ReferenceType_Out_LitPool_SymAddr: u64 = 2;
/// The output reference is to a cstring address in a literal pool.
pub const LLVMDisassembler_ReferenceType_Out_LitPool_CstrAddr: u64 = 3;

/// The output reference is to a Objective-C CoreFoundation string.
pub const LLVMDisassembler_ReferenceType_Out_Objc_CFString_Ref: u64 = 4;
/// The output reference is to a Objective-C message.
pub const LLVMDisassembler_ReferenceType_Out_Objc_Message: u64 = 5;
/// The output reference is to a Objective-C message ref.
pub const LLVMDisassembler_ReferenceType_Out_Objc_Message_Ref: u64 = 6;
/// The output reference is to a Objective-C selector ref.
pub const LLVMDisassembler_ReferenceType_Out_Objc_Selector_Ref: u64 = 7;
/// The output reference is to a Objective-C class ref.
pub const LLVMDisassembler_ReferenceType_Out_Objc_Class_Ref: u64 = 8;
/// The output reference is to a C++ symbol name.
pub const LLVMDisassembler_ReferenceType_DeMangled_Name: u64 = 9;

/// The option to produce marked up assembly.
pub const LLVMDisassembler_Option_UseMarkup: u64 = 1;
/// The option to print immediates as hex.
pub const LLVMDisassembler_Option_PrintImmHex: u64 = 2;
/// The option use the other assembler printer variant
pub const LLVMDisassembler_Option_AsmPrinterVariant: u64 = 4;
/// The option to set comment on instructions
pub const LLVMDisassembler_Option_SetInstrComments: u64 = 8;
/// The option to print latency information alongside instructions
pub const LLVMDisassembler_Option_PrintLatency: u64 = 16;

pub type LLVMSymbolLookupCallback = Option<
    extern "C" fn(
        DisInfo: *mut ::libc::c_void,
        ReferenceValue: u64,
        ReferenceType: *mut u64,
        ReferencePC: u64,
        ReferenceName: *mut *const ::libc::c_char,
    ) -> *const ::libc::c_char,
>;

extern "C" {
    pub fn LLVMCreateDisasm(
        TripleName: *const ::libc::c_char,
        DisInfo: *mut ::libc::c_void,
        TagType: ::libc::c_int,
        GetOpInfo: LLVMOpInfoCallback,
        SymbolLookUp: LLVMSymbolLookupCallback,
    ) -> LLVMDisasmContextRef;
    pub fn LLVMCreateDisasmCPU(
        Triple: *const ::libc::c_char,
        CPU: *const ::libc::c_char,
        DisInfo: *mut ::libc::c_void,
        TagType: ::libc::c_int,
        GetOpInfo: LLVMOpInfoCallback,
        SymbolLookUp: LLVMSymbolLookupCallback,
    ) -> LLVMDisasmContextRef;
    pub fn LLVMCreateDisasmCPUFeatures(
        Triple: *const ::libc::c_char,
        CPU: *const ::libc::c_char,
        Features: *const ::libc::c_char,
        DisInfo: *mut ::libc::c_void,
        TagType: ::libc::c_int,
        GetOpInfo: LLVMOpInfoCallback,
        SymbolLookUp: LLVMSymbolLookupCallback,
    ) -> LLVMDisasmContextRef;
    pub fn LLVMSetDisasmOptions(DC: LLVMDisasmContextRef, Options: u64) -> ::libc::c_int;
    pub fn LLVMDisasmDispose(DC: LLVMDisasmContextRef);
    pub fn LLVMDisasmInstruction(
        DC: LLVMDisasmContextRef,
        Bytes: *mut u8,
        BytesSize: u64,
        PC: u64,
        OutString: *mut ::libc::c_char,
        OutStringSize: ::libc::size_t,
    ) -> ::libc::size_t;
}
