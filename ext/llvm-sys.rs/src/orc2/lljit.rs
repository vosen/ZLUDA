use super::*;
use error::LLVMErrorRef;
use prelude::*;

pub type LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction =
    extern "C" fn(
        Ctx: *mut ::libc::c_void,
        ES: LLVMOrcExecutionSessionRef,
        Triple: *const ::libc::c_char,
    ) -> LLVMOrcObjectLayerRef;

#[derive(Debug)]
pub enum LLVMOrcOpaqueLLJITBuilder {}
pub type LLVMOrcLLJITBuilderRef = *mut LLVMOrcOpaqueLLJITBuilder;

#[derive(Debug)]
pub enum LLVMOrcOpaqueLLJIT {}
pub type LLVMOrcLLJITRef = *mut LLVMOrcOpaqueLLJIT;

extern "C" {
    pub fn LLVMOrcCreateLLJITBuilder() -> LLVMOrcLLJITBuilderRef;
    pub fn LLVMOrcDisposeLLJITBuilder(Builder: LLVMOrcLLJITBuilderRef);
    pub fn LLVMOrcLLJITBuilderSetJITTargetMachineBuilder(
        Builder: LLVMOrcLLJITBuilderRef,
        JTMB: LLVMOrcJITTargetMachineBuilderRef,
    );
    pub fn LLVMOrcLLJITBuilderSetObjectLinkingLayerCreator(
        Builder: LLVMOrcLLJITBuilderRef,
        F: LLVMOrcLLJITBuilderObjectLinkingLayerCreatorFunction,
        Ctx: *mut ::libc::c_void,
    );
    pub fn LLVMOrcCreateLLJIT(
        Result: *mut LLVMOrcLLJITRef,
        Builder: LLVMOrcLLJITBuilderRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcDisposeLLJIT(J: LLVMOrcLLJITRef) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITGetExecutionSession(J: LLVMOrcLLJITRef) -> LLVMOrcExecutionSessionRef;
    pub fn LLVMOrcLLJITGetMainJITDylib(J: LLVMOrcLLJITRef) -> LLVMOrcJITDylibRef;
    pub fn LLVMOrcLLJITGetTripleString(J: LLVMOrcLLJITRef) -> *const ::libc::c_char;
    pub fn LLVMOrcLLJITGetGlobalPrefix(J: LLVMOrcLLJITRef) -> ::libc::c_char;
    pub fn LLVMOrcLLJITMangleAndIntern(
        J: LLVMOrcLLJITRef,
        UnmangledName: *const ::libc::c_char,
    ) -> LLVMOrcSymbolStringPoolEntryRef;
    pub fn LLVMOrcLLJITAddObjectFile(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcJITDylibRef,
        ObjBuffer: LLVMMemoryBufferRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITAddObjectFileWithRT(
        J: LLVMOrcLLJITRef,
        RT: LLVMOrcResourceTrackerRef,
        ObjBuffer: LLVMMemoryBufferRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITAddLLVMIRModule(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcJITDylibRef,
        TSM: LLVMOrcThreadSafeModuleRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITAddLLVMIRModuleWithRT(
        J: LLVMOrcLLJITRef,
        JD: LLVMOrcResourceTrackerRef,
        TSM: LLVMOrcThreadSafeModuleRef,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITLookup(
        J: LLVMOrcLLJITRef,
        Result: *mut LLVMOrcExecutorAddress,
        Name: *const ::libc::c_char,
    ) -> LLVMErrorRef;
    pub fn LLVMOrcLLJITGetObjLinkingLayer(J: LLVMOrcLLJITRef) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcLLJITGetObjTransformLayer(J: LLVMOrcLLJITRef) -> LLVMOrcObjectTransformLayerRef;
    pub fn LLVMOrcLLJITGetIRTransformLayer(J: LLVMOrcLLJITRef) -> LLVMOrcIRTransformLayerRef;
    pub fn LLVMOrcLLJITGetDataLayoutStr(J: LLVMOrcLLJITRef) -> *const ::libc::c_char;
}
