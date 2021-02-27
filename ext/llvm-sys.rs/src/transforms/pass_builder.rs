#![allow(non_snake_case)]

use super::super::prelude::*;
use error::LLVMErrorRef;
use target_machine::LLVMTargetMachineRef;

#[derive(Debug)]
pub enum LLVMOpaquePassBuilderOptions {}
pub type LLVMPassBuilderOptionsRef = *mut LLVMOpaquePassBuilderOptions;

extern "C" {
    pub fn LLVMRunPasses(
        M: LLVMModuleRef,
        Passes: *const ::libc::c_char,
        TM: LLVMTargetMachineRef,
        Options: LLVMPassBuilderOptionsRef,
    ) -> LLVMErrorRef;
    pub fn LLVMCreatePassBuilderOptions() -> LLVMPassBuilderOptionsRef;
    pub fn LLVMPassBuilderOptionsSetVerifyEach(
        Options: LLVMPassBuilderOptionsRef,
        VerifyEach: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetDebugLogging(
        Options: LLVMPassBuilderOptionsRef,
        DebugLogging: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopInterleaving(
        Options: LLVMPassBuilderOptionsRef,
        LoopInterleaving: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopVectorization(
        Options: LLVMPassBuilderOptionsRef,
        LoopVectorization: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetSLPVectorization(
        Options: LLVMPassBuilderOptionsRef,
        SLPVectorization: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLoopUnrolling(
        Options: LLVMPassBuilderOptionsRef,
        LoopUnrolling: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetForgetAllSCEVInLoopUnroll(
        Options: LLVMPassBuilderOptionsRef,
        ForgetAllSCEVInLoopUnroll: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaOptCap(
        Options: LLVMPassBuilderOptionsRef,
        LicmMssaOptCap: ::libc::c_uint,
    );
    pub fn LLVMPassBuilderOptionsSetLicmMssaNoAccForPromotionCap(
        Options: LLVMPassBuilderOptionsRef,
        LicmMssaNoAccForPromotionCap: ::libc::c_uint,
    );
    pub fn LLVMPassBuilderOptionsSetCallGraphProfile(
        Options: LLVMPassBuilderOptionsRef,
        CallGraphProfile: LLVMBool,
    );
    pub fn LLVMPassBuilderOptionsSetMergeFunctions(
        Options: LLVMPassBuilderOptionsRef,
        MergeFunctions: LLVMBool,
    );
    pub fn LLVMDisposePassBuilderOptions(Options: LLVMPassBuilderOptionsRef);
}
