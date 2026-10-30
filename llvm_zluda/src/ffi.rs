use llvm_sys::{prelude::*, *};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMZludaAtomicRMWBinOp {
    LLVMZludaAtomicRMWBinOpXchg = 0,
    LLVMZludaAtomicRMWBinOpAdd = 1,
    LLVMZludaAtomicRMWBinOpSub = 2,
    LLVMZludaAtomicRMWBinOpAnd = 3,
    LLVMZludaAtomicRMWBinOpNand = 4,
    LLVMZludaAtomicRMWBinOpOr = 5,
    LLVMZludaAtomicRMWBinOpXor = 6,
    LLVMZludaAtomicRMWBinOpMax = 7,
    LLVMZludaAtomicRMWBinOpMin = 8,
    LLVMZludaAtomicRMWBinOpUMax = 9,
    LLVMZludaAtomicRMWBinOpUMin = 10,
    LLVMZludaAtomicRMWBinOpFAdd = 11,
    LLVMZludaAtomicRMWBinOpFSub = 12,
    LLVMZludaAtomicRMWBinOpFMax = 13,
    LLVMZludaAtomicRMWBinOpFMin = 14,
    LLVMZludaAtomicRMWBinOpUIncWrap = 15,
    LLVMZludaAtomicRMWBinOpUDecWrap = 16,
}

// Backport from LLVM 19
pub const LLVMZludaFastMathAllowReassoc: ::std::ffi::c_uint = 1 << 0;
pub const LLVMZludaFastMathNoNaNs: ::std::ffi::c_uint = 1 << 1;
pub const LLVMZludaFastMathNoInfs: ::std::ffi::c_uint = 1 << 2;
pub const LLVMZludaFastMathNoSignedZeros: ::std::ffi::c_uint = 1 << 3;
pub const LLVMZludaFastMathAllowReciprocal: ::std::ffi::c_uint = 1 << 4;
pub const LLVMZludaFastMathAllowContract: ::std::ffi::c_uint = 1 << 5;
pub const LLVMZludaFastMathApproxFunc: ::std::ffi::c_uint = 1 << 6;
pub const LLVMZludaFastMathNone: ::std::ffi::c_uint = 0;
pub const LLVMZludaFastMathAll: ::std::ffi::c_uint = LLVMZludaFastMathAllowReassoc
    | LLVMZludaFastMathNoNaNs
    | LLVMZludaFastMathNoInfs
    | LLVMZludaFastMathNoSignedZeros
    | LLVMZludaFastMathAllowReciprocal
    | LLVMZludaFastMathAllowContract
    | LLVMZludaFastMathApproxFunc;

pub type LLVMZludaFastMathFlags = std::ffi::c_uint;

extern "C" {
    pub fn LLVMZludaBuildAlloca(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        AddrSpace: u32,
        Name: *const std::ffi::c_char,
    ) -> LLVMValueRef;

    pub fn LLVMZludaBuildArrayAlloca(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        AddrSpace: u32,
        ArraySize: LLVMValueRef,
        Align: u32,
        Name: *const std::ffi::c_char,
    ) -> LLVMValueRef;

    pub fn LLVMZludaBuildAtomicRMW(
        B: LLVMBuilderRef,
        op: LLVMZludaAtomicRMWBinOp,
        PTR: LLVMValueRef,
        Val: LLVMValueRef,
        scope: *const std::ffi::c_char,
        ordering: LLVMAtomicOrdering,
    ) -> LLVMValueRef;

    pub fn LLVMZludaBuildAtomicCmpXchg(
        B: LLVMBuilderRef,
        Ptr: LLVMValueRef,
        Cmp: LLVMValueRef,
        New: LLVMValueRef,
        scope: *const std::ffi::c_char,
        SuccessOrdering: LLVMAtomicOrdering,
        FailureOrdering: LLVMAtomicOrdering,
    ) -> LLVMValueRef;

    pub fn LLVMZludaSetFastMathFlags(FPMathInst: LLVMValueRef, FMF: LLVMZludaFastMathFlags);

    pub fn LLVMZludaBuildFence(
        B: LLVMBuilderRef,
        ordering: LLVMAtomicOrdering,
        scope: *const std::ffi::c_char,
        Name: *const std::ffi::c_char,
    ) -> LLVMValueRef;

    pub fn LLVMZludaSetAtomic(
        AtomicInst: LLVMValueRef,
        Ordering: LLVMAtomicOrdering,
        SSID: *const std::ffi::c_char,
    );

    #[cfg(feature = "amdgpu")]
    pub fn LLVMZludaLinkWithLLD(
        inputs: u32,
        input_paths: *const *const std::ffi::c_char,
        output_path: *const std::ffi::c_char,
        error_message: *mut *mut std::ffi::c_char,
    ) -> i32;

    pub fn LLVMZludaParseCommandLineOptions(
        argc: i32,
        argv: *const *const std::ffi::c_char,
        error_message: *mut *mut std::ffi::c_char,
    ) -> bool;

    pub fn LLVMZludaLinkUsedDefinitions(
        dest: LLVMModuleRef,
        data: *const std::ffi::c_char,
        size: usize,
        name: *const std::ffi::c_char,
        error: *mut *mut std::ffi::c_char,
    ) -> bool;

    pub fn LLVMZludaUpgradeFunctionAttributes(module: LLVMModuleRef);

    pub fn LLVMZludaAgxLowerAddressSpaces(module: LLVMModuleRef) -> bool;

    pub fn LLVMZludaAgxCanonicalizeFoldedNaNs(module: LLVMModuleRef);

    pub fn LLVMZludaAgxKeepGlobals(module: LLVMModuleRef);

    pub fn LLVMZludaAgxLowerClusters(module: LLVMModuleRef, error: *mut *mut std::ffi::c_char) -> i32;
    pub fn LLVMZludaAgxSpillClusterShared(module: LLVMModuleRef, error: *mut *mut std::ffi::c_char) -> i32;

    pub fn LLVMZludaAirInlineGlobalUsers(module: LLVMModuleRef) -> i32;
    pub fn LLVMZludaAirSharedLayout(module: LLVMModuleRef) -> i32;
    pub fn LLVMZludaSetAirPrimitives(air: i32);
    pub fn LLVMZludaAirStaticRounding(module: LLVMModuleRef, strict: i32) -> i32;
    pub fn LLVMZludaAirCanonicalNaNs(module: LLVMModuleRef) -> i32;
    pub fn LLVMZludaAirCompareFloatBits(module: LLVMModuleRef) -> i32;
    pub fn LLVMZludaAirKernelModule(module: LLVMModuleRef, name: *const std::ffi::c_char) -> LLVMModuleRef;
    pub fn LLVMZludaAirSaltKernels(module: LLVMModuleRef, length: u64);
    pub fn LLVMZludaAirRemapAddressSpaces(module: LLVMModuleRef, error: *mut *mut std::ffi::c_char) -> i32;
    pub fn LLVMZludaAirFunctionPointers(module: LLVMModuleRef, error: *mut *mut std::ffi::c_char) -> i32;
    pub fn LLVMZludaAirGlobalsToBuffer(
        module: LLVMModuleRef,
        json: *mut *mut std::ffi::c_char,
        error: *mut *mut std::ffi::c_char,
    ) -> i32;
}

// The AGX target is not part of upstream LLVM, so llvm-sys has no bindings
// for its initialization functions.
#[cfg(feature = "agx")]
extern "C" {
    pub fn LLVMInitializeAGXTargetInfo();
    pub fn LLVMInitializeAGXTarget();
    pub fn LLVMInitializeAGXTargetMC();
    pub fn LLVMInitializeAGXAsmPrinter();
}

// The AIR target (not upstream either), for the experimental PTX -> AIR -> AGX path.
#[cfg(all(feature = "agx", zluda_air))]
extern "C" {
    pub fn LLVMInitializeAIRTargetInfo();
    pub fn LLVMInitializeAIRTarget();
    pub fn LLVMInitializeAIRTargetMC();
}
