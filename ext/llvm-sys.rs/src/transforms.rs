use super::*;

// Util
extern "C" {
    pub fn LLVMAddLowerSwitchPass(PM: LLVMPassManagerRef );

pub fn LLVMAddPromoteMemoryToRegisterPass(PM: LLVMPassManagerRef );
}
