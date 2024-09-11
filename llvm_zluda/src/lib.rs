pub mod inkwell {
    pub use inkwell::*;
}
pub mod llvm {
    use llvm_sys::prelude::*;
    pub use llvm_sys::*;
    extern "C" {
        pub fn LLVMZludaBuildAlloca(
            B: LLVMBuilderRef,
            Ty: LLVMTypeRef,
            AddrSpace: u32,
            Name: *const i8,
        ) -> LLVMValueRef;
    }
}
