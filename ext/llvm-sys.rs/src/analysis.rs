//! Various analyses of the LLVM IR.

use super::prelude::*;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLVMVerifierFailureAction {
    /// Print to stderr and abort the process.
    LLVMAbortProcessAction = 0,
    /// Print to stderr and return 1.
    LLVMPrintMessageAction = 1,
    /// Return 1 and print nothing.
    LLVMReturnStatusAction = 2,
}

extern "C" {
    /// Verify that a module is valid, taking the specified action if not.
    ///
    /// Optionally returns a human-readable description of any invalid constructs,
    /// which must be disposed with `LLVMDisposeMessage`.
    pub fn LLVMVerifyModule(
        M: LLVMModuleRef,
        Action: LLVMVerifierFailureAction,
        OutMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
    /// Verify that a single function is valid, taking the specified action.
    ///
    /// Useful for debugging.
    pub fn LLVMVerifyFunction(Fn: LLVMValueRef, Action: LLVMVerifierFailureAction) -> LLVMBool;
    /// Open a ghostview window displaying the CFG of the given function.
    ///
    /// Useful for debugging.
    pub fn LLVMViewFunctionCFG(Fn: LLVMValueRef);
    pub fn LLVMViewFunctionCFGOnly(Fn: LLVMValueRef);
}
