use super::*;

extern "C" {
    pub fn LLVMOrcCreateRTDyldObjectLinkingLayerWithSectionMemoryManager(
        ES: LLVMOrcExecutionSessionRef,
    ) -> LLVMOrcObjectLayerRef;
    pub fn LLVMOrcRTDyldObjectLinkingLayerRegisterJITEventListener(
        RTDyldObjLinkingLayer: LLVMOrcObjectLayerRef,
        Listener: LLVMJITEventListenerRef,
    );
}
