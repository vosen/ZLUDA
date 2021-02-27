//! Vectorization transformations of LLVM IR.

use super::super::prelude::*;

extern "C" {
    pub fn LLVMAddLoopVectorizePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddSLPVectorizePass(PM: LLVMPassManagerRef);
}
