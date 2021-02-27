//! Scalar transformations of LLVM IR.

use super::super::prelude::*;

extern "C" {
    pub fn LLVMAddAggressiveDCEPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddDCEPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddBitTrackingDCEPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddAlignmentFromAssumptionsPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCFGSimplificationPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddDeadStoreEliminationPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddScalarizerPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddMergedLoadStoreMotionPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddGVNPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddNewGVNPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddIndVarSimplifyPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddInstructionCombiningPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddInstructionSimplifyPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddJumpThreadingPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLICMPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopDeletionPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopIdiomPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopRotatePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopRerollPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopUnrollPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLoopUnrollAndJamPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLowerAtomicPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddMemCpyOptPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddPartiallyInlineLibCallsPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddReassociatePass(PM: LLVMPassManagerRef);
    pub fn LLVMAddSCCPPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddScalarReplAggregatesPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddScalarReplAggregatesPassSSA(PM: LLVMPassManagerRef);
    pub fn LLVMAddScalarReplAggregatesPassWithThreshold(
        PM: LLVMPassManagerRef,
        Threshold: ::libc::c_int,
    );
    pub fn LLVMAddSimplifyLibCallsPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddTailCallEliminationPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddDemoteMemoryToRegisterPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddVerifierPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddCorrelatedValuePropagationPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddEarlyCSEPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddEarlyCSEMemSSAPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLowerExpectIntrinsicPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddLowerConstantIntrinsicsPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddTypeBasedAliasAnalysisPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddScopedNoAliasAAPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddBasicAliasAnalysisPass(PM: LLVMPassManagerRef);
    pub fn LLVMAddUnifyFunctionExitNodesPass(PM: LLVMPassManagerRef);
}
