use super::super::prelude::*;

#[derive(Debug)]
pub enum LLVMOpaquePassManagerBuilder {}

pub type LLVMPassManagerBuilderRef = *mut LLVMOpaquePassManagerBuilder;

extern "C" {
    pub fn LLVMPassManagerBuilderCreate() -> LLVMPassManagerBuilderRef;
    pub fn LLVMPassManagerBuilderDispose(PMB: LLVMPassManagerBuilderRef);
    pub fn LLVMPassManagerBuilderSetOptLevel(
        PMB: LLVMPassManagerBuilderRef,
        OptLevel: ::libc::c_uint,
    );
    pub fn LLVMPassManagerBuilderSetSizeLevel(
        PMB: LLVMPassManagerBuilderRef,
        SizeLevel: ::libc::c_uint,
    );
    pub fn LLVMPassManagerBuilderSetDisableUnitAtATime(
        PMB: LLVMPassManagerBuilderRef,
        Value: LLVMBool,
    );
    pub fn LLVMPassManagerBuilderSetDisableUnrollLoops(
        PMB: LLVMPassManagerBuilderRef,
        Value: LLVMBool,
    );
    pub fn LLVMPassManagerBuilderSetDisableSimplifyLibCalls(
        PMB: LLVMPassManagerBuilderRef,
        Value: LLVMBool,
    );
    pub fn LLVMPassManagerBuilderUseInlinerWithThreshold(
        PMB: LLVMPassManagerBuilderRef,
        Threshold: ::libc::c_uint,
    );
    pub fn LLVMPassManagerBuilderPopulateFunctionPassManager(
        PMB: LLVMPassManagerBuilderRef,
        PM: LLVMPassManagerRef,
    );
    pub fn LLVMPassManagerBuilderPopulateModulePassManager(
        PMB: LLVMPassManagerBuilderRef,
        PM: LLVMPassManagerRef,
    );
}
