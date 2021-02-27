use super::super::prelude::*;

extern "C" {
    pub fn LLVMAddLowerSwitchPass(PM: LLVMPassManagerRef);

    pub fn LLVMAddPromoteMemoryToRegisterPass(PM: LLVMPassManagerRef);

    pub fn LLVMAddAddDiscriminatorsPass(PM: LLVMPassManagerRef);
}
