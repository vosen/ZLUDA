use super::super::prelude::*;

extern "C" {
    pub fn LLVMAddInstructionCombiningPass(PM: LLVMPassManagerRef);
}
