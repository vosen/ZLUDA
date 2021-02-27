//! Initialization routines which must be called before using library features.

use super::prelude::*;

extern "C" {
    pub fn LLVMInitializeCore(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeTransformUtils(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeScalarOpts(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeObjCARCOpts(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeVectorization(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeInstCombine(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeAggressiveInstCombiner(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeIPO(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeInstrumentation(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeAnalysis(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeIPA(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeCodeGen(R: LLVMPassRegistryRef);
    pub fn LLVMInitializeTarget(R: LLVMPassRegistryRef);
}
