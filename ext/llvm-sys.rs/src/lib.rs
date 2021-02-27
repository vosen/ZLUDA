//! Bindings to LLVM's C API.
//!
//! Refer to the [LLVM documentation](http://llvm.org/docs/) for more
//! information.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

extern crate libc;

use self::prelude::*;

#[derive(Debug)]
pub enum LLVMMemoryBuffer {}

#[derive(Debug)]
pub enum LLVMContext {}

#[derive(Debug)]
pub enum LLVMModule {}

#[derive(Debug)]
pub enum LLVMType {}

#[derive(Debug)]
pub enum LLVMValue {}

#[derive(Debug)]
pub enum LLVMBasicBlock {}

#[derive(Debug)]
pub enum LLVMOpaqueMetadata {}

#[derive(Debug)]
pub enum LLVMOpaqueNamedMDNode {}

#[derive(Debug)]
pub enum LLVMOpaqueValueMetadataEntry {}

#[derive(Debug)]
pub enum LLVMBuilder {}

#[derive(Debug)]
pub enum LLVMOpaqueDIBuilder {}

#[derive(Debug)]
pub enum LLVMModuleProvider {}

#[derive(Debug)]
pub enum LLVMPassManager {}

#[derive(Debug)]
pub enum LLVMPassRegistry {}

#[derive(Debug)]
pub enum LLVMUse {}

#[derive(Debug)]
pub enum LLVMDiagnosticInfo {}

#[derive(Debug)]
pub enum LLVMComdat {}

#[derive(Debug)]
pub enum LLVMOpaqueModuleFlagEntry {}

#[derive(Debug)]
pub enum LLVMOpaqueJITEventListener {}

#[derive(Debug)]
pub enum LLVMOpaqueAttributeRef {}

/// Core types used throughout LLVM.
///
/// In most cases you will want to `use llvm::prelude::*`.
pub mod prelude {
    pub type LLVMBool = ::libc::c_int;
    pub type LLVMMemoryBufferRef = *mut super::LLVMMemoryBuffer;
    pub type LLVMContextRef = *mut super::LLVMContext;
    pub type LLVMModuleRef = *mut super::LLVMModule;
    pub type LLVMTypeRef = *mut super::LLVMType;
    pub type LLVMValueRef = *mut super::LLVMValue;
    pub type LLVMBasicBlockRef = *mut super::LLVMBasicBlock;
    pub type LLVMMetadataRef = *mut super::LLVMOpaqueMetadata;
    pub type LLVMNamedMDNodeRef = *mut super::LLVMOpaqueNamedMDNode;
    pub type LLVMValueMetadataEntry = *mut super::LLVMOpaqueValueMetadataEntry;
    pub type LLVMBuilderRef = *mut super::LLVMBuilder;
    pub type LLVMDIBuilderRef = *mut super::LLVMOpaqueDIBuilder;
    pub type LLVMModuleProviderRef = *mut super::LLVMModuleProvider;
    pub type LLVMPassManagerRef = *mut super::LLVMPassManager;
    pub type LLVMPassRegistryRef = *mut super::LLVMPassRegistry;
    pub type LLVMUseRef = *mut super::LLVMUse;
    pub type LLVMDiagnosticInfoRef = *mut super::LLVMDiagnosticInfo;
    pub type LLVMComdatRef = *mut super::LLVMComdat;
    pub type LLVMModuleFlagEntry = *mut super::LLVMOpaqueModuleFlagEntry;
    pub type LLVMJITEventListenerRef = *mut super::LLVMOpaqueJITEventListener;
    pub type LLVMAttributeRef = *mut super::LLVMOpaqueAttributeRef;
}

pub mod analysis;
pub mod bit_reader;
pub mod bit_writer;
pub mod blake3;
pub mod comdat;
pub mod core;
pub mod debuginfo;
pub mod disassembler;
pub mod error;
pub mod error_handling;
pub mod execution_engine;
pub mod initialization;
pub mod ir_reader;
pub mod linker;
pub mod lto;
pub mod object;
pub mod orc2;
pub mod remarks;
pub mod support;
pub mod target;
pub mod target_machine;

pub mod transforms {
    pub mod aggressive_instcombine;
    pub mod instcombine;
    pub mod ipo;
    pub mod pass_builder;
    pub mod pass_manager_builder;
    pub mod scalar;
    pub mod util;
    pub mod vectorize;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMOpcode {
    LLVMRet = 1,
    LLVMBr = 2,
    LLVMSwitch = 3,
    LLVMIndirectBr = 4,
    LLVMInvoke = 5,
    LLVMUnreachable = 7,
    LLVMCallBr = 67,
    LLVMFNeg = 66,
    LLVMAdd = 8,
    LLVMFAdd = 9,
    LLVMSub = 10,
    LLVMFSub = 11,
    LLVMMul = 12,
    LLVMFMul = 13,
    LLVMUDiv = 14,
    LLVMSDiv = 15,
    LLVMFDiv = 16,
    LLVMURem = 17,
    LLVMSRem = 18,
    LLVMFRem = 19,
    LLVMShl = 20,
    LLVMLShr = 21,
    LLVMAShr = 22,
    LLVMAnd = 23,
    LLVMOr = 24,
    LLVMXor = 25,
    LLVMAlloca = 26,
    LLVMLoad = 27,
    LLVMStore = 28,
    LLVMGetElementPtr = 29,
    LLVMTrunc = 30,
    LLVMZExt = 31,
    LLVMSExt = 32,
    LLVMFPToUI = 33,
    LLVMFPToSI = 34,
    LLVMUIToFP = 35,
    LLVMSIToFP = 36,
    LLVMFPTrunc = 37,
    LLVMFPExt = 38,
    LLVMPtrToInt = 39,
    LLVMIntToPtr = 40,
    LLVMBitCast = 41,
    LLVMAddrSpaceCast = 60,
    LLVMICmp = 42,
    LLVMFCmp = 43,
    LLVMPHI = 44,
    LLVMCall = 45,
    LLVMSelect = 46,
    LLVMUserOp1 = 47,
    LLVMUserOp2 = 48,
    LLVMVAArg = 49,
    LLVMExtractElement = 50,
    LLVMInsertElement = 51,
    LLVMShuffleVector = 52,
    LLVMExtractValue = 53,
    LLVMInsertValue = 54,
    LLVMFreeze = 68,
    LLVMFence = 55,
    LLVMAtomicCmpXchg = 56,
    LLVMAtomicRMW = 57,
    LLVMResume = 58,
    LLVMLandingPad = 59,
    LLVMCleanupRet = 61,
    LLVMCatchRet = 62,
    LLVMCatchPad = 63,
    LLVMCleanupPad = 64,
    LLVMCatchSwitch = 65,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMTypeKind {
    LLVMVoidTypeKind = 0,
    LLVMHalfTypeKind = 1,
    LLVMFloatTypeKind = 2,
    LLVMDoubleTypeKind = 3,
    LLVMX86_FP80TypeKind = 4,
    LLVMFP128TypeKind = 5,
    LLVMPPC_FP128TypeKind = 6,
    LLVMLabelTypeKind = 7,
    LLVMIntegerTypeKind = 8,
    LLVMFunctionTypeKind = 9,
    LLVMStructTypeKind = 10,
    LLVMArrayTypeKind = 11,
    LLVMPointerTypeKind = 12,
    LLVMVectorTypeKind = 13,
    LLVMMetadataTypeKind = 14,
    LLVMX86_MMXTypeKind = 15,
    LLVMTokenTypeKind = 16,
    LLVMScalableVectorTypeKind = 17,
    LLVMBFloatTypeKind = 18,
    LLVMX86_AMXTypeKind = 19,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMLinkage {
    LLVMExternalLinkage = 0,
    LLVMAvailableExternallyLinkage = 1,
    LLVMLinkOnceAnyLinkage = 2,
    LLVMLinkOnceODRLinkage = 3,
    LLVMLinkOnceODRAutoHideLinkage = 4,
    LLVMWeakAnyLinkage = 5,
    LLVMWeakODRLinkage = 6,
    LLVMAppendingLinkage = 7,
    LLVMInternalLinkage = 8,
    LLVMPrivateLinkage = 9,
    LLVMDLLImportLinkage = 10,
    LLVMDLLExportLinkage = 11,
    LLVMExternalWeakLinkage = 12,
    LLVMGhostLinkage = 13,
    LLVMCommonLinkage = 14,
    LLVMLinkerPrivateLinkage = 15,
    LLVMLinkerPrivateWeakLinkage = 16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMVisibility {
    LLVMDefaultVisibility = 0,
    LLVMHiddenVisibility = 1,
    LLVMProtectedVisibility = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMUnnamedAddr {
    /// Address of the GV is significant.
    LLVMNoUnnamedAddr,
    /// Address of the GV is locally insignificant.
    LLVMLocalUnnamedAddr,
    /// Address of the GV is globally insignificant.
    LLVMGlobalUnnamedAddr,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMDLLStorageClass {
    LLVMDefaultStorageClass = 0,
    LLVMDLLImportStorageClass = 1,
    LLVMDLLExportStorageClass = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMCallConv {
    LLVMCCallConv = 0,
    LLVMFastCallConv = 8,
    LLVMColdCallConv = 9,
    LLVMGHCCallConv = 10,
    LLVMHiPECallConv = 11,
    LLVMWebKitJSCallConv = 12,
    LLVMAnyRegCallConv = 13,
    LLVMPreserveMostCallConv = 14,
    LLVMPreserveAllCallConv = 15,
    LLVMSwiftCallConv = 16,
    LLVMCXXFASTTLSCallConv = 17,
    LLVMX86StdcallCallConv = 64,
    LLVMX86FastcallCallConv = 65,
    LLVMARMAPCSCallConv = 66,
    LLVMARMAAPCSCallConv = 67,
    LLVMARMAAPCSVFPCallConv = 68,
    LLVMMSP430INTRCallConv = 69,
    LLVMX86ThisCallCallConv = 70,
    LLVMPTXKernelCallConv = 71,
    LLVMPTXDeviceCallConv = 72,
    LLVMSPIRFUNCCallConv = 75,
    LLVMSPIRKERNELCallConv = 76,
    LLVMIntelOCLBICallConv = 77,
    LLVMX8664SysVCallConv = 78,
    LLVMWin64CallConv = 79,
    LLVMX86VectorCallCallConv = 80,
    LLVMHHVMCallConv = 81,
    LLVMHHVMCCallConv = 82,
    LLVMX86INTRCallConv = 83,
    LLVMAVRINTRCallConv = 84,
    LLVMAVRSIGNALCallConv = 85,
    LLVMAVRBUILTINCallConv = 86,
    LLVMAMDGPUVSCallConv = 87,
    LLVMAMDGPUGSCallConv = 88,
    LLVMAMDGPUPSCallConv = 89,
    LLVMAMDGPUCSCallConv = 90,
    LLVMAMDGPUKERNELCallConv = 91,
    LLVMX86RegCallCallConv = 92,
    LLVMAMDGPUHSCallConv = 93,
    LLVMMSP430BUILTINCallConv = 94,
    LLVMAMDGPULSCallConv = 95,
    LLVMAMDGPUESCallConv = 96,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMValueKind {
    LLVMArgumentValueKind,
    LLVMBasicBlockValueKind,
    LLVMMemoryUseValueKind,
    LLVMMemoryDefValueKind,
    LLVMMemoryPhiValueKind,

    LLVMFunctionValueKind,
    LLVMGlobalAliasValueKind,
    LLVMGlobalIFuncValueKind,
    LLVMGlobalVariableValueKind,
    LLVMBlockAddressValueKind,
    LLVMConstantExprValueKind,
    LLVMConstantArrayValueKind,
    LLVMConstantStructValueKind,
    LLVMConstantVectorValueKind,
    LLVMUndefValueValueKind,
    LLVMConstantAggregateZeroValueKind,
    LLVMConstantDataArrayValueKind,
    LLVMConstantDataVectorValueKind,
    LLVMConstantIntValueKind,
    LLVMConstantFPValueKind,
    LLVMConstantPointerNullValueKind,
    LLVMConstantTokenNoneValueKind,

    LLVMMetadataAsValueValueKind,
    LLVMInlineAsmValueKind,

    LLVMInstructionValueKind,
    LLVMPoisonValueKind,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMIntPredicate {
    LLVMIntEQ = 32,
    LLVMIntNE = 33,
    LLVMIntUGT = 34,
    LLVMIntUGE = 35,
    LLVMIntULT = 36,
    LLVMIntULE = 37,
    LLVMIntSGT = 38,
    LLVMIntSGE = 39,
    LLVMIntSLT = 40,
    LLVMIntSLE = 41,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMRealPredicate {
    LLVMRealPredicateFalse = 0,
    LLVMRealOEQ = 1,
    LLVMRealOGT = 2,
    LLVMRealOGE = 3,
    LLVMRealOLT = 4,
    LLVMRealOLE = 5,
    LLVMRealONE = 6,
    LLVMRealORD = 7,
    LLVMRealUNO = 8,
    LLVMRealUEQ = 9,
    LLVMRealUGT = 10,
    LLVMRealUGE = 11,
    LLVMRealULT = 12,
    LLVMRealULE = 13,
    LLVMRealUNE = 14,
    LLVMRealPredicateTrue = 15,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMLandingPadClauseTy {
    LLVMLandingPadCatch = 0,
    LLVMLandingPadFilter = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMThreadLocalMode {
    LLVMNotThreadLocal = 0,
    LLVMGeneralDynamicTLSModel = 1,
    LLVMLocalDynamicTLSModel = 2,
    LLVMInitialExecTLSModel = 3,
    LLVMLocalExecTLSModel = 4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMAtomicOrdering {
    LLVMAtomicOrderingNotAtomic = 0,
    LLVMAtomicOrderingUnordered = 1,
    LLVMAtomicOrderingMonotonic = 2,
    LLVMAtomicOrderingAcquire = 4,
    LLVMAtomicOrderingRelease = 5,
    LLVMAtomicOrderingAcquireRelease = 6,
    LLVMAtomicOrderingSequentiallyConsistent = 7,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMAtomicRMWBinOp {
    LLVMAtomicRMWBinOpXchg = 0,
    LLVMAtomicRMWBinOpAdd = 1,
    LLVMAtomicRMWBinOpSub = 2,
    LLVMAtomicRMWBinOpAnd = 3,
    LLVMAtomicRMWBinOpNand = 4,
    LLVMAtomicRMWBinOpOr = 5,
    LLVMAtomicRMWBinOpXor = 6,
    LLVMAtomicRMWBinOpMax = 7,
    LLVMAtomicRMWBinOpMin = 8,
    LLVMAtomicRMWBinOpUMax = 9,
    LLVMAtomicRMWBinOpUMin = 10,
    LLVMAtomicRMWBinOpFAdd = 11,
    LLVMAtomicRMWBinOpFSub = 12,
    LLVMAtomicRMWBinOpFMax = 13,
    LLVMAtomicRMWBinOpFMin = 14,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMDiagnosticSeverity {
    LLVMDSError = 0,
    LLVMDSWarning = 1,
    LLVMDSRemark = 2,
    LLVMDSNote = 3,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMInlineAsmDialect {
    LLVMInlineAsmDialectATT,
    LLVMInlineAsmDialectIntel,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMModuleFlagBehavior {
    /// Emits an error if two values disagree, otherwise the resulting value is that of the operands.
    LLVMModuleFlagBehaviorError,
    /// Emits a warning if two values disagree. The result value will be the operand for the flag from the first module being linked.
    LLVMModuleFlagBehaviorWarning,
    /// Adds a requirement that another module flag be present and have a specified value after linking is performed. The value must be a metadata pair, where the first element of the pair is the ID of the module flag to be restricted, and the second element of the pair is the value the module flag should be restricted to. This behavior can be used to restrict the allowable results (via triggering of an error) of linking IDs with the **Override** behavior.
    LLVMModuleFlagBehaviorRequire,
    /// Uses the specified value, regardless of the behavior or value of the other module. If both modules specify **Override**, but the values differ, an error will be emitted.
    LLVMModuleFlagBehaviorOverride,
    /// Appends the two values, which are required to be metadata nodes.
    LLVMModuleFlagBehaviorAppend,
    /// Appends the two values, which are required to be metadata nodes. However, duplicate entries in the second list are dropped during the append operation.
    LLVMModuleFlagBehaviorAppendUnique,
}

pub const LLVMAttributeReturnIndex: ::libc::c_uint = 0;
pub const LLVMAttributeFunctionIndex: ::libc::c_uint = !0; // -1
/// Either LLVMAttributeReturnIndex, LLVMAttributeFunctionIndex, or a parameter
/// number from 1 to N.
pub type LLVMAttributeIndex = ::libc::c_uint;

pub type LLVMDiagnosticHandler =
    Option<extern "C" fn(arg1: LLVMDiagnosticInfoRef, arg2: *mut ::libc::c_void)>;
pub type LLVMYieldCallback = Option<extern "C" fn(arg1: LLVMContextRef, arg2: *mut ::libc::c_void)>;

#[cfg(all(not(doc), not(feature = "no-llvm-linking"), LLVM_SYS_NOT_FOUND))]
std::compile_error!(concat!(
    "No suitable version of LLVM was found system-wide or pointed
       to by LLVM_SYS_",
    env!("CARGO_PKG_VERSION_MAJOR"),
    "_PREFIX.

       Consider using `llvmenv` to compile an appropriate copy of LLVM, and
       refer to the llvm-sys documentation for more information.

       llvm-sys: https://crates.io/crates/llvm-sys
       llvmenv: https://crates.io/crates/llvmenv"
));
