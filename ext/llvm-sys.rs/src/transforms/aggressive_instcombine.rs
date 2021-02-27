use prelude::*;

extern "C" {
    pub fn LLVMAddAggressiveInstCombinerPass(PM: LLVMPassManagerRef);
}
