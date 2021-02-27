pub use llvm_sys::*;

#[allow(non_upper_case_globals)]
pub mod zluda {
    use bitflags::bitflags;
    use llvm_sys::prelude::*;
    use llvm_sys::*;

    bitflags! {
        #[repr(transparent)]
        pub struct FastMathFlags: u32 {
            const AllowReassoc      = 0b00000001;
            const NoNaNs            = 0b00000010;
            const NoInfs            = 0b00000100;
            const NoSignedZeros     = 0b00001000;
            const AllowReciprocal   = 0b00010000;
            const AllowContract     = 0b00100000;
            const ApproxFunc        = 0b01000000;
        }
    }

    extern "C" {
        pub fn LLVMZludaBuildAtomicRMW(
            B: LLVMBuilderRef,
            op: LLVMAtomicRMWBinOp,
            PTR: LLVMValueRef,
            Val: LLVMValueRef,
            scope: *const i8,
            ordering: LLVMAtomicOrdering,
            Align: u32,
        ) -> LLVMValueRef;

        pub fn LLVMZludaBuildAtomicCmpXchg(
            B: LLVMBuilderRef,
            PTR: LLVMValueRef,
            Cmp: LLVMValueRef,
            New: LLVMValueRef,
            scope: *const i8,
            SuccessOrdering: LLVMAtomicOrdering,
            FailureOrdering: LLVMAtomicOrdering,
            Align: u32,
        ) -> LLVMValueRef;

        pub fn LLVMZludaSetFastMathFlags(Val: LLVMValueRef, Flags: FastMathFlags);

        pub fn LLVMZludaBuildFence(
            B: LLVMBuilderRef,
            ordering: LLVMAtomicOrdering,
            scope: *const i8,
            Name: *const i8,
        ) -> LLVMValueRef;

        pub fn LLVMZludaBuildAlloca(
            B: LLVMBuilderRef,
            Ty: LLVMTypeRef,
            AddrSpace: u32,
            Name: *const i8,
        ) -> LLVMValueRef;

        pub fn LLVMZludaSetAtomic(
            C: LLVMContextRef,
            Val: LLVMValueRef,
            Ordering: LLVMAtomicOrdering,
            scope: *const i8,
        );
    }
}
