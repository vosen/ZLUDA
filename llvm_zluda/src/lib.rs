use llvm_sys::prelude::*;
pub use llvm_sys::*;

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

extern "C" {
    pub fn LLVMZludaBuildAlloca(
        B: LLVMBuilderRef,
        Ty: LLVMTypeRef,
        AddrSpace: u32,
        Name: *const i8,
    ) -> LLVMValueRef;

    pub fn LLVMZludaBuildAtomicRMW(
        B: LLVMBuilderRef,
        op: LLVMZludaAtomicRMWBinOp,
        PTR: LLVMValueRef,
        Val: LLVMValueRef,
        scope: *const i8,
        ordering: LLVMAtomicOrdering,
    ) -> LLVMValueRef;

    pub fn LLVMZludaBuildAtomicCmpXchg(
        B: LLVMBuilderRef,
        Ptr: LLVMValueRef,
        Cmp: LLVMValueRef,
        New: LLVMValueRef,
        scope: *const i8,
        SuccessOrdering: LLVMAtomicOrdering,
        FailureOrdering: LLVMAtomicOrdering,
    ) -> LLVMValueRef;
}
