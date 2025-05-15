match (name, flag) {
    (b"cuArray3DCreate", 0) => {
        if version >= 3020 {
            return cuArray3DCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArray3DCreate", 1) => {
        if version >= 3020 {
            return cuArray3DCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArray3DCreate", 2) => {
        if version >= 3020 {
            return cuArray3DCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArray3DGetDescriptor", 0) => {
        if version >= 3020 {
            return cuArray3DGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArray3DGetDescriptor", 1) => {
        if version >= 3020 {
            return cuArray3DGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArray3DGetDescriptor", 2) => {
        if version >= 3020 {
            return cuArray3DGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArray3DGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArrayCreate", 0) => {
        if version >= 3020 {
            return cuArrayCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArrayCreate", 1) => {
        if version >= 3020 {
            return cuArrayCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArrayCreate", 2) => {
        if version >= 3020 {
            return cuArrayCreate_v2 as _;
        }
        if version >= 2000 {
            return cuArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuArrayDestroy", 0) => {
        if version >= 2000 {
            return cuArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuArrayDestroy", 1) => {
        if version >= 2000 {
            return cuArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuArrayDestroy", 2) => {
        if version >= 2000 {
            return cuArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetDescriptor", 0) => {
        if version >= 3020 {
            return cuArrayGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArrayGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetDescriptor", 1) => {
        if version >= 3020 {
            return cuArrayGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArrayGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetDescriptor", 2) => {
        if version >= 3020 {
            return cuArrayGetDescriptor_v2 as _;
        }
        if version >= 2000 {
            return cuArrayGetDescriptor as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetMemoryRequirements", 0) => {
        if version >= 11060 {
            return cuArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetMemoryRequirements", 1) => {
        if version >= 11060 {
            return cuArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetMemoryRequirements", 2) => {
        if version >= 11060 {
            return cuArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetPlane", 0) => {
        if version >= 11020 {
            return cuArrayGetPlane as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetPlane", 1) => {
        if version >= 11020 {
            return cuArrayGetPlane as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetPlane", 2) => {
        if version >= 11020 {
            return cuArrayGetPlane as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetSparseProperties", 0) => {
        if version >= 11010 {
            return cuArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetSparseProperties", 1) => {
        if version >= 11010 {
            return cuArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuArrayGetSparseProperties", 2) => {
        if version >= 11010 {
            return cuArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessCheckpoint", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessCheckpoint as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessCheckpoint", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessCheckpoint as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessCheckpoint", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessCheckpoint as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetRestoreThreadId", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessGetRestoreThreadId as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetRestoreThreadId", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessGetRestoreThreadId as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetRestoreThreadId", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessGetRestoreThreadId as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetState", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessGetState as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetState", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessGetState as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessGetState", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessGetState as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessLock", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessLock as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessLock", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessLock as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessLock", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessLock as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessRestore", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessRestore as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessRestore", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessRestore as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessRestore", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessRestore as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessUnlock", 0) => {
        if version >= 12080 {
            return cuCheckpointProcessUnlock as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessUnlock", 1) => {
        if version >= 12080 {
            return cuCheckpointProcessUnlock as _;
        }
        usize::MAX as _
    }
    (b"cuCheckpointProcessUnlock", 2) => {
        if version >= 12080 {
            return cuCheckpointProcessUnlock as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttribute", 0) => {
        if version >= 12010 {
            return cuCoredumpGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttribute", 1) => {
        if version >= 12010 {
            return cuCoredumpGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttribute", 2) => {
        if version >= 12010 {
            return cuCoredumpGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttributeGlobal", 0) => {
        if version >= 12010 {
            return cuCoredumpGetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttributeGlobal", 1) => {
        if version >= 12010 {
            return cuCoredumpGetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpGetAttributeGlobal", 2) => {
        if version >= 12010 {
            return cuCoredumpGetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttribute", 0) => {
        if version >= 12010 {
            return cuCoredumpSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttribute", 1) => {
        if version >= 12010 {
            return cuCoredumpSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttribute", 2) => {
        if version >= 12010 {
            return cuCoredumpSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttributeGlobal", 0) => {
        if version >= 12010 {
            return cuCoredumpSetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttributeGlobal", 1) => {
        if version >= 12010 {
            return cuCoredumpSetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCoredumpSetAttributeGlobal", 2) => {
        if version >= 12010 {
            return cuCoredumpSetAttributeGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuCtxAttach", 0) => {
        if version >= 2000 {
            return cuCtxAttach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxAttach", 1) => {
        if version >= 2000 {
            return cuCtxAttach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxAttach", 2) => {
        if version >= 2000 {
            return cuCtxAttach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxCreate", 0) => {
        if version >= 12050 {
            return cuCtxCreate_v4 as _;
        }
        if version >= 11040 {
            return cuCtxCreate_v3 as _;
        }
        if version >= 3020 {
            return cuCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuCtxCreate", 1) => {
        if version >= 12050 {
            return cuCtxCreate_v4 as _;
        }
        if version >= 11040 {
            return cuCtxCreate_v3 as _;
        }
        if version >= 3020 {
            return cuCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuCtxCreate", 2) => {
        if version >= 12050 {
            return cuCtxCreate_v4 as _;
        }
        if version >= 11040 {
            return cuCtxCreate_v3 as _;
        }
        if version >= 3020 {
            return cuCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDestroy", 0) => {
        if version >= 4000 {
            return cuCtxDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDestroy", 1) => {
        if version >= 4000 {
            return cuCtxDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDestroy", 2) => {
        if version >= 4000 {
            return cuCtxDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDetach", 0) => {
        if version >= 2000 {
            return cuCtxDetach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDetach", 1) => {
        if version >= 2000 {
            return cuCtxDetach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDetach", 2) => {
        if version >= 2000 {
            return cuCtxDetach as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDisablePeerAccess", 0) => {
        if version >= 4000 {
            return cuCtxDisablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDisablePeerAccess", 1) => {
        if version >= 4000 {
            return cuCtxDisablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxDisablePeerAccess", 2) => {
        if version >= 4000 {
            return cuCtxDisablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxEnablePeerAccess", 0) => {
        if version >= 4000 {
            return cuCtxEnablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxEnablePeerAccess", 1) => {
        if version >= 4000 {
            return cuCtxEnablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxEnablePeerAccess", 2) => {
        if version >= 4000 {
            return cuCtxEnablePeerAccess as _;
        }
        usize::MAX as _
    }
    (b"cuCtxFromGreenCtx", 0) => {
        if version >= 12040 {
            return cuCtxFromGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuCtxFromGreenCtx", 1) => {
        if version >= 12040 {
            return cuCtxFromGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuCtxFromGreenCtx", 2) => {
        if version >= 12040 {
            return cuCtxFromGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetApiVersion", 0) => {
        if version >= 3020 {
            return cuCtxGetApiVersion as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetApiVersion", 1) => {
        if version >= 3020 {
            return cuCtxGetApiVersion as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetApiVersion", 2) => {
        if version >= 3020 {
            return cuCtxGetApiVersion as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCacheConfig", 0) => {
        if version >= 3020 {
            return cuCtxGetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCacheConfig", 1) => {
        if version >= 3020 {
            return cuCtxGetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCacheConfig", 2) => {
        if version >= 3020 {
            return cuCtxGetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCurrent", 0) => {
        if version >= 4000 {
            return cuCtxGetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCurrent", 1) => {
        if version >= 4000 {
            return cuCtxGetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetCurrent", 2) => {
        if version >= 4000 {
            return cuCtxGetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevResource", 0) => {
        if version >= 12040 {
            return cuCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevResource", 1) => {
        if version >= 12040 {
            return cuCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevResource", 2) => {
        if version >= 12040 {
            return cuCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevice", 0) => {
        if version >= 2000 {
            return cuCtxGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevice", 1) => {
        if version >= 2000 {
            return cuCtxGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetDevice", 2) => {
        if version >= 2000 {
            return cuCtxGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetExecAffinity", 0) => {
        if version >= 11040 {
            return cuCtxGetExecAffinity as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetExecAffinity", 1) => {
        if version >= 11040 {
            return cuCtxGetExecAffinity as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetExecAffinity", 2) => {
        if version >= 11040 {
            return cuCtxGetExecAffinity as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetFlags", 0) => {
        if version >= 7000 {
            return cuCtxGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetFlags", 1) => {
        if version >= 7000 {
            return cuCtxGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetFlags", 2) => {
        if version >= 7000 {
            return cuCtxGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetId", 0) => {
        if version >= 12000 {
            return cuCtxGetId as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetId", 1) => {
        if version >= 12000 {
            return cuCtxGetId as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetId", 2) => {
        if version >= 12000 {
            return cuCtxGetId as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetLimit", 0) => {
        if version >= 3010 {
            return cuCtxGetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetLimit", 1) => {
        if version >= 3010 {
            return cuCtxGetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetLimit", 2) => {
        if version >= 3010 {
            return cuCtxGetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetSharedMemConfig", 0) => {
        if version >= 4020 {
            return cuCtxGetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetSharedMemConfig", 1) => {
        if version >= 4020 {
            return cuCtxGetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetSharedMemConfig", 2) => {
        if version >= 4020 {
            return cuCtxGetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetStreamPriorityRange", 0) => {
        if version >= 5050 {
            return cuCtxGetStreamPriorityRange as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetStreamPriorityRange", 1) => {
        if version >= 5050 {
            return cuCtxGetStreamPriorityRange as _;
        }
        usize::MAX as _
    }
    (b"cuCtxGetStreamPriorityRange", 2) => {
        if version >= 5050 {
            return cuCtxGetStreamPriorityRange as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPopCurrent", 0) => {
        if version >= 4000 {
            return cuCtxPopCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPopCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPopCurrent", 1) => {
        if version >= 4000 {
            return cuCtxPopCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPopCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPopCurrent", 2) => {
        if version >= 4000 {
            return cuCtxPopCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPopCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPushCurrent", 0) => {
        if version >= 4000 {
            return cuCtxPushCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPushCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPushCurrent", 1) => {
        if version >= 4000 {
            return cuCtxPushCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPushCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxPushCurrent", 2) => {
        if version >= 4000 {
            return cuCtxPushCurrent_v2 as _;
        }
        if version >= 2000 {
            return cuCtxPushCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxRecordEvent", 0) => {
        if version >= 12050 {
            return cuCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxRecordEvent", 1) => {
        if version >= 12050 {
            return cuCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxRecordEvent", 2) => {
        if version >= 12050 {
            return cuCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxResetPersistingL2Cache", 0) => {
        if version >= 11000 {
            return cuCtxResetPersistingL2Cache as _;
        }
        usize::MAX as _
    }
    (b"cuCtxResetPersistingL2Cache", 1) => {
        if version >= 11000 {
            return cuCtxResetPersistingL2Cache as _;
        }
        usize::MAX as _
    }
    (b"cuCtxResetPersistingL2Cache", 2) => {
        if version >= 11000 {
            return cuCtxResetPersistingL2Cache as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCacheConfig", 0) => {
        if version >= 3020 {
            return cuCtxSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCacheConfig", 1) => {
        if version >= 3020 {
            return cuCtxSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCacheConfig", 2) => {
        if version >= 3020 {
            return cuCtxSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCurrent", 0) => {
        if version >= 4000 {
            return cuCtxSetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCurrent", 1) => {
        if version >= 4000 {
            return cuCtxSetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetCurrent", 2) => {
        if version >= 4000 {
            return cuCtxSetCurrent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetFlags", 0) => {
        if version >= 12010 {
            return cuCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetFlags", 1) => {
        if version >= 12010 {
            return cuCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetFlags", 2) => {
        if version >= 12010 {
            return cuCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetLimit", 0) => {
        if version >= 3010 {
            return cuCtxSetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetLimit", 1) => {
        if version >= 3010 {
            return cuCtxSetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetLimit", 2) => {
        if version >= 3010 {
            return cuCtxSetLimit as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetSharedMemConfig", 0) => {
        if version >= 4020 {
            return cuCtxSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetSharedMemConfig", 1) => {
        if version >= 4020 {
            return cuCtxSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSetSharedMemConfig", 2) => {
        if version >= 4020 {
            return cuCtxSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSynchronize", 0) => {
        if version >= 2000 {
            return cuCtxSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSynchronize", 1) => {
        if version >= 2000 {
            return cuCtxSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuCtxSynchronize", 2) => {
        if version >= 2000 {
            return cuCtxSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuCtxWaitEvent", 0) => {
        if version >= 12050 {
            return cuCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxWaitEvent", 1) => {
        if version >= 12050 {
            return cuCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuCtxWaitEvent", 2) => {
        if version >= 12050 {
            return cuCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalMemory", 0) => {
        if version >= 10000 {
            return cuDestroyExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalMemory", 1) => {
        if version >= 10000 {
            return cuDestroyExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalMemory", 2) => {
        if version >= 10000 {
            return cuDestroyExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalSemaphore", 0) => {
        if version >= 10000 {
            return cuDestroyExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalSemaphore", 1) => {
        if version >= 10000 {
            return cuDestroyExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuDestroyExternalSemaphore", 2) => {
        if version >= 10000 {
            return cuDestroyExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuDevResourceGenerateDesc", 0) => {
        if version >= 12040 {
            return cuDevResourceGenerateDesc as _;
        }
        usize::MAX as _
    }
    (b"cuDevResourceGenerateDesc", 1) => {
        if version >= 12040 {
            return cuDevResourceGenerateDesc as _;
        }
        usize::MAX as _
    }
    (b"cuDevResourceGenerateDesc", 2) => {
        if version >= 12040 {
            return cuDevResourceGenerateDesc as _;
        }
        usize::MAX as _
    }
    (b"cuDevSmResourceSplitByCount", 0) => {
        if version >= 12040 {
            return cuDevSmResourceSplitByCount as _;
        }
        usize::MAX as _
    }
    (b"cuDevSmResourceSplitByCount", 1) => {
        if version >= 12040 {
            return cuDevSmResourceSplitByCount as _;
        }
        usize::MAX as _
    }
    (b"cuDevSmResourceSplitByCount", 2) => {
        if version >= 12040 {
            return cuDevSmResourceSplitByCount as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceCanAccessPeer", 0) => {
        if version >= 4000 {
            return cuDeviceCanAccessPeer as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceCanAccessPeer", 1) => {
        if version >= 4000 {
            return cuDeviceCanAccessPeer as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceCanAccessPeer", 2) => {
        if version >= 4000 {
            return cuDeviceCanAccessPeer as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceComputeCapability", 0) => {
        if version >= 2000 {
            return cuDeviceComputeCapability as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceComputeCapability", 1) => {
        if version >= 2000 {
            return cuDeviceComputeCapability as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceComputeCapability", 2) => {
        if version >= 2000 {
            return cuDeviceComputeCapability as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGet", 0) => {
        if version >= 2000 {
            return cuDeviceGet as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGet", 1) => {
        if version >= 2000 {
            return cuDeviceGet as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGet", 2) => {
        if version >= 2000 {
            return cuDeviceGet as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetAttribute", 0) => {
        if version >= 2000 {
            return cuDeviceGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetAttribute", 1) => {
        if version >= 2000 {
            return cuDeviceGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetAttribute", 2) => {
        if version >= 2000 {
            return cuDeviceGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetByPCIBusId", 0) => {
        if version >= 4010 {
            return cuDeviceGetByPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetByPCIBusId", 1) => {
        if version >= 4010 {
            return cuDeviceGetByPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetByPCIBusId", 2) => {
        if version >= 4010 {
            return cuDeviceGetByPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetCount", 0) => {
        if version >= 2000 {
            return cuDeviceGetCount as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetCount", 1) => {
        if version >= 2000 {
            return cuDeviceGetCount as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetCount", 2) => {
        if version >= 2000 {
            return cuDeviceGetCount as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDefaultMemPool", 0) => {
        if version >= 11020 {
            return cuDeviceGetDefaultMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDefaultMemPool", 1) => {
        if version >= 11020 {
            return cuDeviceGetDefaultMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDefaultMemPool", 2) => {
        if version >= 11020 {
            return cuDeviceGetDefaultMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDevResource", 0) => {
        if version >= 12040 {
            return cuDeviceGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDevResource", 1) => {
        if version >= 12040 {
            return cuDeviceGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetDevResource", 2) => {
        if version >= 12040 {
            return cuDeviceGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetExecAffinitySupport", 0) => {
        if version >= 11040 {
            return cuDeviceGetExecAffinitySupport as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetExecAffinitySupport", 1) => {
        if version >= 11040 {
            return cuDeviceGetExecAffinitySupport as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetExecAffinitySupport", 2) => {
        if version >= 11040 {
            return cuDeviceGetExecAffinitySupport as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetGraphMemAttribute", 0) => {
        if version >= 11040 {
            return cuDeviceGetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetGraphMemAttribute", 1) => {
        if version >= 11040 {
            return cuDeviceGetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetGraphMemAttribute", 2) => {
        if version >= 11040 {
            return cuDeviceGetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetLuid", 0) => {
        if version >= 10000 {
            return cuDeviceGetLuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetLuid", 1) => {
        if version >= 10000 {
            return cuDeviceGetLuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetLuid", 2) => {
        if version >= 10000 {
            return cuDeviceGetLuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetMemPool", 0) => {
        if version >= 11020 {
            return cuDeviceGetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetMemPool", 1) => {
        if version >= 11020 {
            return cuDeviceGetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetMemPool", 2) => {
        if version >= 11020 {
            return cuDeviceGetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetName", 0) => {
        if version >= 2000 {
            return cuDeviceGetName as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetName", 1) => {
        if version >= 2000 {
            return cuDeviceGetName as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetName", 2) => {
        if version >= 2000 {
            return cuDeviceGetName as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetNvSciSyncAttributes", 0) => {
        if version >= 10020 {
            return cuDeviceGetNvSciSyncAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetNvSciSyncAttributes", 1) => {
        if version >= 10020 {
            return cuDeviceGetNvSciSyncAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetNvSciSyncAttributes", 2) => {
        if version >= 10020 {
            return cuDeviceGetNvSciSyncAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetP2PAttribute", 0) => {
        if version >= 8000 {
            return cuDeviceGetP2PAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetP2PAttribute", 1) => {
        if version >= 8000 {
            return cuDeviceGetP2PAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetP2PAttribute", 2) => {
        if version >= 8000 {
            return cuDeviceGetP2PAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetPCIBusId", 0) => {
        if version >= 4010 {
            return cuDeviceGetPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetPCIBusId", 1) => {
        if version >= 4010 {
            return cuDeviceGetPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetPCIBusId", 2) => {
        if version >= 4010 {
            return cuDeviceGetPCIBusId as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetProperties", 0) => {
        if version >= 2000 {
            return cuDeviceGetProperties as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetProperties", 1) => {
        if version >= 2000 {
            return cuDeviceGetProperties as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetProperties", 2) => {
        if version >= 2000 {
            return cuDeviceGetProperties as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetTexture1DLinearMaxWidth", 0) => {
        if version >= 11010 {
            return cuDeviceGetTexture1DLinearMaxWidth as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetTexture1DLinearMaxWidth", 1) => {
        if version >= 11010 {
            return cuDeviceGetTexture1DLinearMaxWidth as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetTexture1DLinearMaxWidth", 2) => {
        if version >= 11010 {
            return cuDeviceGetTexture1DLinearMaxWidth as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetUuid", 0) => {
        if version >= 11040 {
            return cuDeviceGetUuid_v2 as _;
        }
        if version >= 9020 {
            return cuDeviceGetUuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetUuid", 1) => {
        if version >= 11040 {
            return cuDeviceGetUuid_v2 as _;
        }
        if version >= 9020 {
            return cuDeviceGetUuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGetUuid", 2) => {
        if version >= 11040 {
            return cuDeviceGetUuid_v2 as _;
        }
        if version >= 9020 {
            return cuDeviceGetUuid as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGraphMemTrim", 0) => {
        if version >= 11040 {
            return cuDeviceGraphMemTrim as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGraphMemTrim", 1) => {
        if version >= 11040 {
            return cuDeviceGraphMemTrim as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceGraphMemTrim", 2) => {
        if version >= 11040 {
            return cuDeviceGraphMemTrim as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxGetState", 0) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxGetState as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxGetState", 1) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxGetState as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxGetState", 2) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxGetState as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRelease", 0) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxRelease_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxRelease as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRelease", 1) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxRelease_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxRelease as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRelease", 2) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxRelease_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxRelease as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxReset", 0) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxReset_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxReset as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxReset", 1) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxReset_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxReset as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxReset", 2) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxReset_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxReset as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRetain", 0) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxRetain as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRetain", 1) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxRetain as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxRetain", 2) => {
        if version >= 7000 {
            return cuDevicePrimaryCtxRetain as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxSetFlags", 0) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxSetFlags_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxSetFlags", 1) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxSetFlags_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuDevicePrimaryCtxSetFlags", 2) => {
        if version >= 11000 {
            return cuDevicePrimaryCtxSetFlags_v2 as _;
        }
        if version >= 7000 {
            return cuDevicePrimaryCtxSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceRegisterAsyncNotification", 0) => {
        if version >= 12040 {
            return cuDeviceRegisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceRegisterAsyncNotification", 1) => {
        if version >= 12040 {
            return cuDeviceRegisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceRegisterAsyncNotification", 2) => {
        if version >= 12040 {
            return cuDeviceRegisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetGraphMemAttribute", 0) => {
        if version >= 11040 {
            return cuDeviceSetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetGraphMemAttribute", 1) => {
        if version >= 11040 {
            return cuDeviceSetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetGraphMemAttribute", 2) => {
        if version >= 11040 {
            return cuDeviceSetGraphMemAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetMemPool", 0) => {
        if version >= 11020 {
            return cuDeviceSetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetMemPool", 1) => {
        if version >= 11020 {
            return cuDeviceSetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceSetMemPool", 2) => {
        if version >= 11020 {
            return cuDeviceSetMemPool as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceTotalMem", 0) => {
        if version >= 3020 {
            return cuDeviceTotalMem_v2 as _;
        }
        if version >= 2000 {
            return cuDeviceTotalMem as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceTotalMem", 1) => {
        if version >= 3020 {
            return cuDeviceTotalMem_v2 as _;
        }
        if version >= 2000 {
            return cuDeviceTotalMem as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceTotalMem", 2) => {
        if version >= 3020 {
            return cuDeviceTotalMem_v2 as _;
        }
        if version >= 2000 {
            return cuDeviceTotalMem as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceUnregisterAsyncNotification", 0) => {
        if version >= 12040 {
            return cuDeviceUnregisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceUnregisterAsyncNotification", 1) => {
        if version >= 12040 {
            return cuDeviceUnregisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDeviceUnregisterAsyncNotification", 2) => {
        if version >= 12040 {
            return cuDeviceUnregisterAsyncNotification as _;
        }
        usize::MAX as _
    }
    (b"cuDriverGetVersion", 0) => {
        if version >= 2020 {
            return cuDriverGetVersion as _;
        }
        usize::MAX as _
    }
    (b"cuDriverGetVersion", 1) => {
        if version >= 2020 {
            return cuDriverGetVersion as _;
        }
        usize::MAX as _
    }
    (b"cuDriverGetVersion", 2) => {
        if version >= 2020 {
            return cuDriverGetVersion as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerAcquireFrame", 0) => {
        if version >= 7000 {
            return cuEGLStreamConsumerAcquireFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerAcquireFrame", 1) => {
        if version >= 7000 {
            return cuEGLStreamConsumerAcquireFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerAcquireFrame", 2) => {
        if version >= 7000 {
            return cuEGLStreamConsumerAcquireFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnect", 0) => {
        if version >= 7000 {
            return cuEGLStreamConsumerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnect", 1) => {
        if version >= 7000 {
            return cuEGLStreamConsumerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnect", 2) => {
        if version >= 7000 {
            return cuEGLStreamConsumerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnectWithFlags", 0) => {
        if version >= 8000 {
            return cuEGLStreamConsumerConnectWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnectWithFlags", 1) => {
        if version >= 8000 {
            return cuEGLStreamConsumerConnectWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerConnectWithFlags", 2) => {
        if version >= 8000 {
            return cuEGLStreamConsumerConnectWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerDisconnect", 0) => {
        if version >= 7000 {
            return cuEGLStreamConsumerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerDisconnect", 1) => {
        if version >= 7000 {
            return cuEGLStreamConsumerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerDisconnect", 2) => {
        if version >= 7000 {
            return cuEGLStreamConsumerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerReleaseFrame", 0) => {
        if version >= 7000 {
            return cuEGLStreamConsumerReleaseFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerReleaseFrame", 1) => {
        if version >= 7000 {
            return cuEGLStreamConsumerReleaseFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamConsumerReleaseFrame", 2) => {
        if version >= 7000 {
            return cuEGLStreamConsumerReleaseFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerConnect", 0) => {
        if version >= 7000 {
            return cuEGLStreamProducerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerConnect", 1) => {
        if version >= 7000 {
            return cuEGLStreamProducerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerConnect", 2) => {
        if version >= 7000 {
            return cuEGLStreamProducerConnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerDisconnect", 0) => {
        if version >= 7000 {
            return cuEGLStreamProducerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerDisconnect", 1) => {
        if version >= 7000 {
            return cuEGLStreamProducerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerDisconnect", 2) => {
        if version >= 7000 {
            return cuEGLStreamProducerDisconnect as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerPresentFrame", 0) => {
        if version >= 7000 {
            return cuEGLStreamProducerPresentFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerPresentFrame", 1) => {
        if version >= 7000 {
            return cuEGLStreamProducerPresentFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerPresentFrame", 2) => {
        if version >= 7000 {
            return cuEGLStreamProducerPresentFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerReturnFrame", 0) => {
        if version >= 7000 {
            return cuEGLStreamProducerReturnFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerReturnFrame", 1) => {
        if version >= 7000 {
            return cuEGLStreamProducerReturnFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEGLStreamProducerReturnFrame", 2) => {
        if version >= 7000 {
            return cuEGLStreamProducerReturnFrame as _;
        }
        usize::MAX as _
    }
    (b"cuEventCreate", 0) => {
        if version >= 2000 {
            return cuEventCreate as _;
        }
        usize::MAX as _
    }
    (b"cuEventCreate", 1) => {
        if version >= 2000 {
            return cuEventCreate as _;
        }
        usize::MAX as _
    }
    (b"cuEventCreate", 2) => {
        if version >= 2000 {
            return cuEventCreate as _;
        }
        usize::MAX as _
    }
    (b"cuEventDestroy", 0) => {
        if version >= 4000 {
            return cuEventDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuEventDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuEventDestroy", 1) => {
        if version >= 4000 {
            return cuEventDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuEventDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuEventDestroy", 2) => {
        if version >= 4000 {
            return cuEventDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuEventDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuEventElapsedTime", 0) => {
        if version >= 12080 {
            return cuEventElapsedTime_v2 as _;
        }
        if version >= 2000 {
            return cuEventElapsedTime as _;
        }
        usize::MAX as _
    }
    (b"cuEventElapsedTime", 1) => {
        if version >= 12080 {
            return cuEventElapsedTime_v2 as _;
        }
        if version >= 2000 {
            return cuEventElapsedTime as _;
        }
        usize::MAX as _
    }
    (b"cuEventElapsedTime", 2) => {
        if version >= 12080 {
            return cuEventElapsedTime_v2 as _;
        }
        if version >= 2000 {
            return cuEventElapsedTime as _;
        }
        usize::MAX as _
    }
    (b"cuEventQuery", 0) => {
        if version >= 2000 {
            return cuEventQuery as _;
        }
        usize::MAX as _
    }
    (b"cuEventQuery", 1) => {
        if version >= 2000 {
            return cuEventQuery as _;
        }
        usize::MAX as _
    }
    (b"cuEventQuery", 2) => {
        if version >= 2000 {
            return cuEventQuery as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecord", 0) => {
        if version >= 2000 {
            return cuEventRecord as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecord", 1) => {
        if version >= 2000 {
            return cuEventRecord as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecord", 2) => {
        if version >= 7000 {
            return cuEventRecord_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecordWithFlags", 0) => {
        if version >= 11010 {
            return cuEventRecordWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecordWithFlags", 1) => {
        if version >= 11010 {
            return cuEventRecordWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuEventRecordWithFlags", 2) => {
        if version >= 11010 {
            return cuEventRecordWithFlags_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuEventSynchronize", 0) => {
        if version >= 2000 {
            return cuEventSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuEventSynchronize", 1) => {
        if version >= 2000 {
            return cuEventSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuEventSynchronize", 2) => {
        if version >= 2000 {
            return cuEventSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedBuffer", 0) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedBuffer", 1) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedBuffer", 2) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedMipmappedArray", 0) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedMipmappedArray", 1) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuExternalMemoryGetMappedMipmappedArray", 2) => {
        if version >= 10000 {
            return cuExternalMemoryGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuFlushGPUDirectRDMAWrites", 0) => {
        if version >= 11030 {
            return cuFlushGPUDirectRDMAWrites as _;
        }
        usize::MAX as _
    }
    (b"cuFlushGPUDirectRDMAWrites", 1) => {
        if version >= 11030 {
            return cuFlushGPUDirectRDMAWrites as _;
        }
        usize::MAX as _
    }
    (b"cuFlushGPUDirectRDMAWrites", 2) => {
        if version >= 11030 {
            return cuFlushGPUDirectRDMAWrites as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetAttribute", 0) => {
        if version >= 2020 {
            return cuFuncGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetAttribute", 1) => {
        if version >= 2020 {
            return cuFuncGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetAttribute", 2) => {
        if version >= 2020 {
            return cuFuncGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetModule", 0) => {
        if version >= 11000 {
            return cuFuncGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetModule", 1) => {
        if version >= 11000 {
            return cuFuncGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetModule", 2) => {
        if version >= 11000 {
            return cuFuncGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetName", 0) => {
        if version >= 12030 {
            return cuFuncGetName as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetName", 1) => {
        if version >= 12030 {
            return cuFuncGetName as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetName", 2) => {
        if version >= 12030 {
            return cuFuncGetName as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetParamInfo", 0) => {
        if version >= 12040 {
            return cuFuncGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetParamInfo", 1) => {
        if version >= 12040 {
            return cuFuncGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuFuncGetParamInfo", 2) => {
        if version >= 12040 {
            return cuFuncGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuFuncIsLoaded", 0) => {
        if version >= 12040 {
            return cuFuncIsLoaded as _;
        }
        usize::MAX as _
    }
    (b"cuFuncIsLoaded", 1) => {
        if version >= 12040 {
            return cuFuncIsLoaded as _;
        }
        usize::MAX as _
    }
    (b"cuFuncIsLoaded", 2) => {
        if version >= 12040 {
            return cuFuncIsLoaded as _;
        }
        usize::MAX as _
    }
    (b"cuFuncLoad", 0) => {
        if version >= 12040 {
            return cuFuncLoad as _;
        }
        usize::MAX as _
    }
    (b"cuFuncLoad", 1) => {
        if version >= 12040 {
            return cuFuncLoad as _;
        }
        usize::MAX as _
    }
    (b"cuFuncLoad", 2) => {
        if version >= 12040 {
            return cuFuncLoad as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetAttribute", 0) => {
        if version >= 9000 {
            return cuFuncSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetAttribute", 1) => {
        if version >= 9000 {
            return cuFuncSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetAttribute", 2) => {
        if version >= 9000 {
            return cuFuncSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetBlockShape", 0) => {
        if version >= 2000 {
            return cuFuncSetBlockShape as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetBlockShape", 1) => {
        if version >= 2000 {
            return cuFuncSetBlockShape as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetBlockShape", 2) => {
        if version >= 2000 {
            return cuFuncSetBlockShape as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetCacheConfig", 0) => {
        if version >= 3000 {
            return cuFuncSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetCacheConfig", 1) => {
        if version >= 3000 {
            return cuFuncSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetCacheConfig", 2) => {
        if version >= 3000 {
            return cuFuncSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedMemConfig", 0) => {
        if version >= 4020 {
            return cuFuncSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedMemConfig", 1) => {
        if version >= 4020 {
            return cuFuncSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedMemConfig", 2) => {
        if version >= 4020 {
            return cuFuncSetSharedMemConfig as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedSize", 0) => {
        if version >= 2000 {
            return cuFuncSetSharedSize as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedSize", 1) => {
        if version >= 2000 {
            return cuFuncSetSharedSize as _;
        }
        usize::MAX as _
    }
    (b"cuFuncSetSharedSize", 2) => {
        if version >= 2000 {
            return cuFuncSetSharedSize as _;
        }
        usize::MAX as _
    }
    (b"cuGLCtxCreate", 0) => {
        if version >= 3020 {
            return cuGLCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuGLCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGLCtxCreate", 1) => {
        if version >= 3020 {
            return cuGLCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuGLCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGLCtxCreate", 2) => {
        if version >= 3020 {
            return cuGLCtxCreate_v2 as _;
        }
        if version >= 2000 {
            return cuGLCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGLGetDevices", 0) => {
        if version >= 6050 {
            return cuGLGetDevices_v2 as _;
        }
        if version >= 4010 {
            return cuGLGetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuGLGetDevices", 1) => {
        if version >= 6050 {
            return cuGLGetDevices_v2 as _;
        }
        if version >= 4010 {
            return cuGLGetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuGLGetDevices", 2) => {
        if version >= 6050 {
            return cuGLGetDevices_v2 as _;
        }
        if version >= 4010 {
            return cuGLGetDevices as _;
        }
        usize::MAX as _
    }
    (b"cuGLInit", 0) => {
        if version >= 2000 {
            return cuGLInit as _;
        }
        usize::MAX as _
    }
    (b"cuGLInit", 1) => {
        if version >= 2000 {
            return cuGLInit as _;
        }
        usize::MAX as _
    }
    (b"cuGLInit", 2) => {
        if version >= 2000 {
            return cuGLInit as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObject", 0) => {
        if version >= 3020 {
            return cuGLMapBufferObject_v2 as _;
        }
        if version >= 2000 {
            return cuGLMapBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObject", 1) => {
        if version >= 3020 {
            return cuGLMapBufferObject_v2 as _;
        }
        if version >= 2000 {
            return cuGLMapBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObject", 2) => {
        if version >= 7000 {
            return cuGLMapBufferObject_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObjectAsync", 0) => {
        if version >= 3020 {
            return cuGLMapBufferObjectAsync_v2 as _;
        }
        if version >= 2030 {
            return cuGLMapBufferObjectAsync as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObjectAsync", 1) => {
        if version >= 3020 {
            return cuGLMapBufferObjectAsync_v2 as _;
        }
        if version >= 2030 {
            return cuGLMapBufferObjectAsync as _;
        }
        usize::MAX as _
    }
    (b"cuGLMapBufferObjectAsync", 2) => {
        if version >= 7000 {
            return cuGLMapBufferObjectAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGLRegisterBufferObject", 0) => {
        if version >= 2000 {
            return cuGLRegisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLRegisterBufferObject", 1) => {
        if version >= 2000 {
            return cuGLRegisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLRegisterBufferObject", 2) => {
        if version >= 2000 {
            return cuGLRegisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLSetBufferObjectMapFlags", 0) => {
        if version >= 2030 {
            return cuGLSetBufferObjectMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGLSetBufferObjectMapFlags", 1) => {
        if version >= 2030 {
            return cuGLSetBufferObjectMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGLSetBufferObjectMapFlags", 2) => {
        if version >= 2030 {
            return cuGLSetBufferObjectMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObject", 0) => {
        if version >= 2000 {
            return cuGLUnmapBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObject", 1) => {
        if version >= 2000 {
            return cuGLUnmapBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObject", 2) => {
        if version >= 2000 {
            return cuGLUnmapBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObjectAsync", 0) => {
        if version >= 2030 {
            return cuGLUnmapBufferObjectAsync as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObjectAsync", 1) => {
        if version >= 2030 {
            return cuGLUnmapBufferObjectAsync as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnmapBufferObjectAsync", 2) => {
        if version >= 2030 {
            return cuGLUnmapBufferObjectAsync as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnregisterBufferObject", 0) => {
        if version >= 2000 {
            return cuGLUnregisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnregisterBufferObject", 1) => {
        if version >= 2000 {
            return cuGLUnregisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGLUnregisterBufferObject", 2) => {
        if version >= 2000 {
            return cuGLUnregisterBufferObject as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorName", 0) => {
        if version >= 6000 {
            return cuGetErrorName as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorName", 1) => {
        if version >= 6000 {
            return cuGetErrorName as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorName", 2) => {
        if version >= 6000 {
            return cuGetErrorName as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorString", 0) => {
        if version >= 6000 {
            return cuGetErrorString as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorString", 1) => {
        if version >= 6000 {
            return cuGetErrorString as _;
        }
        usize::MAX as _
    }
    (b"cuGetErrorString", 2) => {
        if version >= 6000 {
            return cuGetErrorString as _;
        }
        usize::MAX as _
    }
    (b"cuGetExportTable", 0) => {
        if version >= 3000 {
            return cuGetExportTable as _;
        }
        usize::MAX as _
    }
    (b"cuGetExportTable", 1) => {
        if version >= 3000 {
            return cuGetExportTable as _;
        }
        usize::MAX as _
    }
    (b"cuGetExportTable", 2) => {
        if version >= 3000 {
            return cuGetExportTable as _;
        }
        usize::MAX as _
    }
    (b"cuGetProcAddress", 0) => {
        if version >= 12000 {
            return cuGetProcAddress_v2 as _;
        }
        if version >= 11030 {
            return cuGetProcAddress as _;
        }
        usize::MAX as _
    }
    (b"cuGetProcAddress", 1) => {
        if version >= 12000 {
            return cuGetProcAddress_v2 as _;
        }
        if version >= 11030 {
            return cuGetProcAddress as _;
        }
        usize::MAX as _
    }
    (b"cuGetProcAddress", 2) => {
        if version >= 12000 {
            return cuGetProcAddress_v2 as _;
        }
        if version >= 11030 {
            return cuGetProcAddress as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddBatchMemOpNode", 0) => {
        if version >= 11070 {
            return cuGraphAddBatchMemOpNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddBatchMemOpNode", 1) => {
        if version >= 11070 {
            return cuGraphAddBatchMemOpNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddBatchMemOpNode", 2) => {
        if version >= 11070 {
            return cuGraphAddBatchMemOpNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddChildGraphNode", 0) => {
        if version >= 10000 {
            return cuGraphAddChildGraphNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddChildGraphNode", 1) => {
        if version >= 10000 {
            return cuGraphAddChildGraphNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddChildGraphNode", 2) => {
        if version >= 10000 {
            return cuGraphAddChildGraphNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddDependencies", 0) => {
        if version >= 12030 {
            return cuGraphAddDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddDependencies", 1) => {
        if version >= 12030 {
            return cuGraphAddDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddDependencies", 2) => {
        if version >= 12030 {
            return cuGraphAddDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEmptyNode", 0) => {
        if version >= 10000 {
            return cuGraphAddEmptyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEmptyNode", 1) => {
        if version >= 10000 {
            return cuGraphAddEmptyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEmptyNode", 2) => {
        if version >= 10000 {
            return cuGraphAddEmptyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventRecordNode", 0) => {
        if version >= 11010 {
            return cuGraphAddEventRecordNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventRecordNode", 1) => {
        if version >= 11010 {
            return cuGraphAddEventRecordNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventRecordNode", 2) => {
        if version >= 11010 {
            return cuGraphAddEventRecordNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventWaitNode", 0) => {
        if version >= 11010 {
            return cuGraphAddEventWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventWaitNode", 1) => {
        if version >= 11010 {
            return cuGraphAddEventWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddEventWaitNode", 2) => {
        if version >= 11010 {
            return cuGraphAddEventWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresSignalNode", 0) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresSignalNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresSignalNode", 1) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresSignalNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresSignalNode", 2) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresSignalNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresWaitNode", 0) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresWaitNode", 1) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddExternalSemaphoresWaitNode", 2) => {
        if version >= 11020 {
            return cuGraphAddExternalSemaphoresWaitNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddHostNode", 0) => {
        if version >= 10000 {
            return cuGraphAddHostNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddHostNode", 1) => {
        if version >= 10000 {
            return cuGraphAddHostNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddHostNode", 2) => {
        if version >= 10000 {
            return cuGraphAddHostNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddKernelNode", 0) => {
        if version >= 12000 {
            return cuGraphAddKernelNode_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddKernelNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddKernelNode", 1) => {
        if version >= 12000 {
            return cuGraphAddKernelNode_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddKernelNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddKernelNode", 2) => {
        if version >= 12000 {
            return cuGraphAddKernelNode_v2 as _;
        }
        if version >= 10000 {
            return cuGraphAddKernelNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemAllocNode", 0) => {
        if version >= 11040 {
            return cuGraphAddMemAllocNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemAllocNode", 1) => {
        if version >= 11040 {
            return cuGraphAddMemAllocNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemAllocNode", 2) => {
        if version >= 11040 {
            return cuGraphAddMemAllocNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemFreeNode", 0) => {
        if version >= 11040 {
            return cuGraphAddMemFreeNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemFreeNode", 1) => {
        if version >= 11040 {
            return cuGraphAddMemFreeNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemFreeNode", 2) => {
        if version >= 11040 {
            return cuGraphAddMemFreeNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemcpyNode", 0) => {
        if version >= 10000 {
            return cuGraphAddMemcpyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemcpyNode", 1) => {
        if version >= 10000 {
            return cuGraphAddMemcpyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemcpyNode", 2) => {
        if version >= 10000 {
            return cuGraphAddMemcpyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemsetNode", 0) => {
        if version >= 10000 {
            return cuGraphAddMemsetNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemsetNode", 1) => {
        if version >= 10000 {
            return cuGraphAddMemsetNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddMemsetNode", 2) => {
        if version >= 10000 {
            return cuGraphAddMemsetNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddNode", 0) => {
        if version >= 12030 {
            return cuGraphAddNode_v2 as _;
        }
        if version >= 12020 {
            return cuGraphAddNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddNode", 1) => {
        if version >= 12030 {
            return cuGraphAddNode_v2 as _;
        }
        if version >= 12020 {
            return cuGraphAddNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphAddNode", 2) => {
        if version >= 12030 {
            return cuGraphAddNode_v2 as _;
        }
        if version >= 12020 {
            return cuGraphAddNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeGetParams", 0) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeGetParams", 1) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeGetParams", 2) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeSetParams", 0) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeSetParams", 1) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphBatchMemOpNodeSetParams", 2) => {
        if version >= 11070 {
            return cuGraphBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphChildGraphNodeGetGraph", 0) => {
        if version >= 10000 {
            return cuGraphChildGraphNodeGetGraph as _;
        }
        usize::MAX as _
    }
    (b"cuGraphChildGraphNodeGetGraph", 1) => {
        if version >= 10000 {
            return cuGraphChildGraphNodeGetGraph as _;
        }
        usize::MAX as _
    }
    (b"cuGraphChildGraphNodeGetGraph", 2) => {
        if version >= 10000 {
            return cuGraphChildGraphNodeGetGraph as _;
        }
        usize::MAX as _
    }
    (b"cuGraphClone", 0) => {
        if version >= 10000 {
            return cuGraphClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphClone", 1) => {
        if version >= 10000 {
            return cuGraphClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphClone", 2) => {
        if version >= 10000 {
            return cuGraphClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphConditionalHandleCreate", 0) => {
        if version >= 12030 {
            return cuGraphConditionalHandleCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphConditionalHandleCreate", 1) => {
        if version >= 12030 {
            return cuGraphConditionalHandleCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphConditionalHandleCreate", 2) => {
        if version >= 12030 {
            return cuGraphConditionalHandleCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphCreate", 0) => {
        if version >= 10000 {
            return cuGraphCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphCreate", 1) => {
        if version >= 10000 {
            return cuGraphCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphCreate", 2) => {
        if version >= 10000 {
            return cuGraphCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDebugDotPrint", 0) => {
        if version >= 11030 {
            return cuGraphDebugDotPrint as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDebugDotPrint", 1) => {
        if version >= 11030 {
            return cuGraphDebugDotPrint as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDebugDotPrint", 2) => {
        if version >= 11030 {
            return cuGraphDebugDotPrint as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroy", 0) => {
        if version >= 10000 {
            return cuGraphDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroy", 1) => {
        if version >= 10000 {
            return cuGraphDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroy", 2) => {
        if version >= 10000 {
            return cuGraphDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroyNode", 0) => {
        if version >= 10000 {
            return cuGraphDestroyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroyNode", 1) => {
        if version >= 10000 {
            return cuGraphDestroyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphDestroyNode", 2) => {
        if version >= 10000 {
            return cuGraphDestroyNode as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeGetEvent", 0) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeGetEvent", 1) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeGetEvent", 2) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeSetEvent", 0) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeSetEvent", 1) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventRecordNodeSetEvent", 2) => {
        if version >= 11010 {
            return cuGraphEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeGetEvent", 0) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeGetEvent", 1) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeGetEvent", 2) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeGetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeSetEvent", 0) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeSetEvent", 1) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphEventWaitNodeSetEvent", 2) => {
        if version >= 11010 {
            return cuGraphEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecBatchMemOpNodeSetParams", 0) => {
        if version >= 11070 {
            return cuGraphExecBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecBatchMemOpNodeSetParams", 1) => {
        if version >= 11070 {
            return cuGraphExecBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecBatchMemOpNodeSetParams", 2) => {
        if version >= 11070 {
            return cuGraphExecBatchMemOpNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecChildGraphNodeSetParams", 0) => {
        if version >= 11010 {
            return cuGraphExecChildGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecChildGraphNodeSetParams", 1) => {
        if version >= 11010 {
            return cuGraphExecChildGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecChildGraphNodeSetParams", 2) => {
        if version >= 11010 {
            return cuGraphExecChildGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecDestroy", 0) => {
        if version >= 10000 {
            return cuGraphExecDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecDestroy", 1) => {
        if version >= 10000 {
            return cuGraphExecDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecDestroy", 2) => {
        if version >= 10000 {
            return cuGraphExecDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventRecordNodeSetEvent", 0) => {
        if version >= 11010 {
            return cuGraphExecEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventRecordNodeSetEvent", 1) => {
        if version >= 11010 {
            return cuGraphExecEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventRecordNodeSetEvent", 2) => {
        if version >= 11010 {
            return cuGraphExecEventRecordNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventWaitNodeSetEvent", 0) => {
        if version >= 11010 {
            return cuGraphExecEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventWaitNodeSetEvent", 1) => {
        if version >= 11010 {
            return cuGraphExecEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecEventWaitNodeSetEvent", 2) => {
        if version >= 11010 {
            return cuGraphExecEventWaitNodeSetEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresSignalNodeSetParams", 0) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresSignalNodeSetParams", 1) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresSignalNodeSetParams", 2) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresWaitNodeSetParams", 0) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresWaitNodeSetParams", 1) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecExternalSemaphoresWaitNodeSetParams", 2) => {
        if version >= 11020 {
            return cuGraphExecExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecGetFlags", 0) => {
        if version >= 12000 {
            return cuGraphExecGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecGetFlags", 1) => {
        if version >= 12000 {
            return cuGraphExecGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecGetFlags", 2) => {
        if version >= 12000 {
            return cuGraphExecGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecHostNodeSetParams", 0) => {
        if version >= 10020 {
            return cuGraphExecHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecHostNodeSetParams", 1) => {
        if version >= 10020 {
            return cuGraphExecHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecHostNodeSetParams", 2) => {
        if version >= 10020 {
            return cuGraphExecHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecKernelNodeSetParams", 0) => {
        if version >= 12000 {
            return cuGraphExecKernelNodeSetParams_v2 as _;
        }
        if version >= 10010 {
            return cuGraphExecKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecKernelNodeSetParams", 1) => {
        if version >= 12000 {
            return cuGraphExecKernelNodeSetParams_v2 as _;
        }
        if version >= 10010 {
            return cuGraphExecKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecKernelNodeSetParams", 2) => {
        if version >= 12000 {
            return cuGraphExecKernelNodeSetParams_v2 as _;
        }
        if version >= 10010 {
            return cuGraphExecKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemcpyNodeSetParams", 0) => {
        if version >= 10020 {
            return cuGraphExecMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemcpyNodeSetParams", 1) => {
        if version >= 10020 {
            return cuGraphExecMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemcpyNodeSetParams", 2) => {
        if version >= 10020 {
            return cuGraphExecMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemsetNodeSetParams", 0) => {
        if version >= 10020 {
            return cuGraphExecMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemsetNodeSetParams", 1) => {
        if version >= 10020 {
            return cuGraphExecMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecMemsetNodeSetParams", 2) => {
        if version >= 10020 {
            return cuGraphExecMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecNodeSetParams", 0) => {
        if version >= 12020 {
            return cuGraphExecNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecNodeSetParams", 1) => {
        if version >= 12020 {
            return cuGraphExecNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecNodeSetParams", 2) => {
        if version >= 12020 {
            return cuGraphExecNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecUpdate", 0) => {
        if version >= 12000 {
            return cuGraphExecUpdate_v2 as _;
        }
        if version >= 10020 {
            return cuGraphExecUpdate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecUpdate", 1) => {
        if version >= 12000 {
            return cuGraphExecUpdate_v2 as _;
        }
        if version >= 10020 {
            return cuGraphExecUpdate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExecUpdate", 2) => {
        if version >= 12000 {
            return cuGraphExecUpdate_v2 as _;
        }
        if version >= 10020 {
            return cuGraphExecUpdate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeGetParams", 0) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeGetParams", 1) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeGetParams", 2) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeSetParams", 0) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeSetParams", 1) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresSignalNodeSetParams", 2) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresSignalNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeGetParams", 0) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeGetParams", 1) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeGetParams", 2) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeSetParams", 0) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeSetParams", 1) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphExternalSemaphoresWaitNodeSetParams", 2) => {
        if version >= 11020 {
            return cuGraphExternalSemaphoresWaitNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetEdges", 0) => {
        if version >= 12030 {
            return cuGraphGetEdges_v2 as _;
        }
        if version >= 10000 {
            return cuGraphGetEdges as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetEdges", 1) => {
        if version >= 12030 {
            return cuGraphGetEdges_v2 as _;
        }
        if version >= 10000 {
            return cuGraphGetEdges as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetEdges", 2) => {
        if version >= 12030 {
            return cuGraphGetEdges_v2 as _;
        }
        if version >= 10000 {
            return cuGraphGetEdges as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetNodes", 0) => {
        if version >= 10000 {
            return cuGraphGetNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetNodes", 1) => {
        if version >= 10000 {
            return cuGraphGetNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetNodes", 2) => {
        if version >= 10000 {
            return cuGraphGetNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetRootNodes", 0) => {
        if version >= 10000 {
            return cuGraphGetRootNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetRootNodes", 1) => {
        if version >= 10000 {
            return cuGraphGetRootNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphGetRootNodes", 2) => {
        if version >= 10000 {
            return cuGraphGetRootNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeGetParams", 0) => {
        if version >= 10000 {
            return cuGraphHostNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeGetParams", 1) => {
        if version >= 10000 {
            return cuGraphHostNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeGetParams", 2) => {
        if version >= 10000 {
            return cuGraphHostNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeSetParams", 0) => {
        if version >= 10000 {
            return cuGraphHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeSetParams", 1) => {
        if version >= 10000 {
            return cuGraphHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphHostNodeSetParams", 2) => {
        if version >= 10000 {
            return cuGraphHostNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiate", 0) => {
        if version >= 11000 {
            return cuGraphInstantiate_v2 as _;
        }
        if version >= 10000 {
            return cuGraphInstantiate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiate", 1) => {
        if version >= 11000 {
            return cuGraphInstantiate_v2 as _;
        }
        if version >= 10000 {
            return cuGraphInstantiate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiate", 2) => {
        if version >= 11000 {
            return cuGraphInstantiate_v2 as _;
        }
        if version >= 10000 {
            return cuGraphInstantiate as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithFlags", 0) => {
        if version >= 11040 {
            return cuGraphInstantiateWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithFlags", 1) => {
        if version >= 11040 {
            return cuGraphInstantiateWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithFlags", 2) => {
        if version >= 11040 {
            return cuGraphInstantiateWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithParams", 0) => {
        if version >= 12000 {
            return cuGraphInstantiateWithParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithParams", 1) => {
        if version >= 12000 {
            return cuGraphInstantiateWithParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphInstantiateWithParams", 2) => {
        if version >= 12000 {
            return cuGraphInstantiateWithParams_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeCopyAttributes", 0) => {
        if version >= 11000 {
            return cuGraphKernelNodeCopyAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeCopyAttributes", 1) => {
        if version >= 11000 {
            return cuGraphKernelNodeCopyAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeCopyAttributes", 2) => {
        if version >= 11000 {
            return cuGraphKernelNodeCopyAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetAttribute", 0) => {
        if version >= 11000 {
            return cuGraphKernelNodeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetAttribute", 1) => {
        if version >= 11000 {
            return cuGraphKernelNodeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetAttribute", 2) => {
        if version >= 11000 {
            return cuGraphKernelNodeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetParams", 0) => {
        if version >= 12000 {
            return cuGraphKernelNodeGetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetParams", 1) => {
        if version >= 12000 {
            return cuGraphKernelNodeGetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeGetParams", 2) => {
        if version >= 12000 {
            return cuGraphKernelNodeGetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetAttribute", 0) => {
        if version >= 11000 {
            return cuGraphKernelNodeSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetAttribute", 1) => {
        if version >= 11000 {
            return cuGraphKernelNodeSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetAttribute", 2) => {
        if version >= 11000 {
            return cuGraphKernelNodeSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetParams", 0) => {
        if version >= 12000 {
            return cuGraphKernelNodeSetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetParams", 1) => {
        if version >= 12000 {
            return cuGraphKernelNodeSetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphKernelNodeSetParams", 2) => {
        if version >= 12000 {
            return cuGraphKernelNodeSetParams_v2 as _;
        }
        if version >= 10000 {
            return cuGraphKernelNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphLaunch", 0) => {
        if version >= 10000 {
            return cuGraphLaunch as _;
        }
        usize::MAX as _
    }
    (b"cuGraphLaunch", 1) => {
        if version >= 10000 {
            return cuGraphLaunch as _;
        }
        usize::MAX as _
    }
    (b"cuGraphLaunch", 2) => {
        if version >= 10000 {
            return cuGraphLaunch_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemAllocNodeGetParams", 0) => {
        if version >= 11040 {
            return cuGraphMemAllocNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemAllocNodeGetParams", 1) => {
        if version >= 11040 {
            return cuGraphMemAllocNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemAllocNodeGetParams", 2) => {
        if version >= 11040 {
            return cuGraphMemAllocNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemFreeNodeGetParams", 0) => {
        if version >= 11040 {
            return cuGraphMemFreeNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemFreeNodeGetParams", 1) => {
        if version >= 11040 {
            return cuGraphMemFreeNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemFreeNodeGetParams", 2) => {
        if version >= 11040 {
            return cuGraphMemFreeNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeGetParams", 0) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeGetParams", 1) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeGetParams", 2) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeSetParams", 0) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeSetParams", 1) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemcpyNodeSetParams", 2) => {
        if version >= 10000 {
            return cuGraphMemcpyNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeGetParams", 0) => {
        if version >= 10000 {
            return cuGraphMemsetNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeGetParams", 1) => {
        if version >= 10000 {
            return cuGraphMemsetNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeGetParams", 2) => {
        if version >= 10000 {
            return cuGraphMemsetNodeGetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeSetParams", 0) => {
        if version >= 10000 {
            return cuGraphMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeSetParams", 1) => {
        if version >= 10000 {
            return cuGraphMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphMemsetNodeSetParams", 2) => {
        if version >= 10000 {
            return cuGraphMemsetNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeFindInClone", 0) => {
        if version >= 10000 {
            return cuGraphNodeFindInClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeFindInClone", 1) => {
        if version >= 10000 {
            return cuGraphNodeFindInClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeFindInClone", 2) => {
        if version >= 10000 {
            return cuGraphNodeFindInClone as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependencies", 0) => {
        if version >= 12030 {
            return cuGraphNodeGetDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependencies", 1) => {
        if version >= 12030 {
            return cuGraphNodeGetDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependencies", 2) => {
        if version >= 12030 {
            return cuGraphNodeGetDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependentNodes", 0) => {
        if version >= 12030 {
            return cuGraphNodeGetDependentNodes_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependentNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependentNodes", 1) => {
        if version >= 12030 {
            return cuGraphNodeGetDependentNodes_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependentNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetDependentNodes", 2) => {
        if version >= 12030 {
            return cuGraphNodeGetDependentNodes_v2 as _;
        }
        if version >= 10000 {
            return cuGraphNodeGetDependentNodes as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetEnabled", 0) => {
        if version >= 11060 {
            return cuGraphNodeGetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetEnabled", 1) => {
        if version >= 11060 {
            return cuGraphNodeGetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetEnabled", 2) => {
        if version >= 11060 {
            return cuGraphNodeGetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetType", 0) => {
        if version >= 10000 {
            return cuGraphNodeGetType as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetType", 1) => {
        if version >= 10000 {
            return cuGraphNodeGetType as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeGetType", 2) => {
        if version >= 10000 {
            return cuGraphNodeGetType as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetEnabled", 0) => {
        if version >= 11060 {
            return cuGraphNodeSetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetEnabled", 1) => {
        if version >= 11060 {
            return cuGraphNodeSetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetEnabled", 2) => {
        if version >= 11060 {
            return cuGraphNodeSetEnabled as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetParams", 0) => {
        if version >= 12020 {
            return cuGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetParams", 1) => {
        if version >= 12020 {
            return cuGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphNodeSetParams", 2) => {
        if version >= 12020 {
            return cuGraphNodeSetParams as _;
        }
        usize::MAX as _
    }
    (b"cuGraphReleaseUserObject", 0) => {
        if version >= 11030 {
            return cuGraphReleaseUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphReleaseUserObject", 1) => {
        if version >= 11030 {
            return cuGraphReleaseUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphReleaseUserObject", 2) => {
        if version >= 11030 {
            return cuGraphReleaseUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRemoveDependencies", 0) => {
        if version >= 12030 {
            return cuGraphRemoveDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphRemoveDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRemoveDependencies", 1) => {
        if version >= 12030 {
            return cuGraphRemoveDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphRemoveDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRemoveDependencies", 2) => {
        if version >= 12030 {
            return cuGraphRemoveDependencies_v2 as _;
        }
        if version >= 10000 {
            return cuGraphRemoveDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRetainUserObject", 0) => {
        if version >= 11030 {
            return cuGraphRetainUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRetainUserObject", 1) => {
        if version >= 11030 {
            return cuGraphRetainUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphRetainUserObject", 2) => {
        if version >= 11030 {
            return cuGraphRetainUserObject as _;
        }
        usize::MAX as _
    }
    (b"cuGraphUpload", 0) => {
        if version >= 11010 {
            return cuGraphUpload as _;
        }
        usize::MAX as _
    }
    (b"cuGraphUpload", 1) => {
        if version >= 11010 {
            return cuGraphUpload as _;
        }
        usize::MAX as _
    }
    (b"cuGraphUpload", 2) => {
        if version >= 11010 {
            return cuGraphUpload_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsEGLRegisterImage", 0) => {
        if version >= 7000 {
            return cuGraphicsEGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsEGLRegisterImage", 1) => {
        if version >= 7000 {
            return cuGraphicsEGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsEGLRegisterImage", 2) => {
        if version >= 7000 {
            return cuGraphicsEGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterBuffer", 0) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterBuffer", 1) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterBuffer", 2) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterBuffer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterImage", 0) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterImage", 1) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsGLRegisterImage", 2) => {
        if version >= 3000 {
            return cuGraphicsGLRegisterImage as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsMapResources", 0) => {
        if version >= 3000 {
            return cuGraphicsMapResources as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsMapResources", 1) => {
        if version >= 3000 {
            return cuGraphicsMapResources as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsMapResources", 2) => {
        if version >= 7000 {
            return cuGraphicsMapResources_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedEglFrame", 0) => {
        if version >= 7000 {
            return cuGraphicsResourceGetMappedEglFrame as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedEglFrame", 1) => {
        if version >= 7000 {
            return cuGraphicsResourceGetMappedEglFrame as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedEglFrame", 2) => {
        if version >= 7000 {
            return cuGraphicsResourceGetMappedEglFrame as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedMipmappedArray", 0) => {
        if version >= 5000 {
            return cuGraphicsResourceGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedMipmappedArray", 1) => {
        if version >= 5000 {
            return cuGraphicsResourceGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedMipmappedArray", 2) => {
        if version >= 5000 {
            return cuGraphicsResourceGetMappedMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedPointer", 0) => {
        if version >= 3020 {
            return cuGraphicsResourceGetMappedPointer_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedPointer", 1) => {
        if version >= 3020 {
            return cuGraphicsResourceGetMappedPointer_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceGetMappedPointer", 2) => {
        if version >= 3020 {
            return cuGraphicsResourceGetMappedPointer_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceGetMappedPointer as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceSetMapFlags", 0) => {
        if version >= 6050 {
            return cuGraphicsResourceSetMapFlags_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceSetMapFlags", 1) => {
        if version >= 6050 {
            return cuGraphicsResourceSetMapFlags_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsResourceSetMapFlags", 2) => {
        if version >= 6050 {
            return cuGraphicsResourceSetMapFlags_v2 as _;
        }
        if version >= 3000 {
            return cuGraphicsResourceSetMapFlags as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsSubResourceGetMappedArray", 0) => {
        if version >= 3000 {
            return cuGraphicsSubResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsSubResourceGetMappedArray", 1) => {
        if version >= 3000 {
            return cuGraphicsSubResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsSubResourceGetMappedArray", 2) => {
        if version >= 3000 {
            return cuGraphicsSubResourceGetMappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnmapResources", 0) => {
        if version >= 3000 {
            return cuGraphicsUnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnmapResources", 1) => {
        if version >= 3000 {
            return cuGraphicsUnmapResources as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnmapResources", 2) => {
        if version >= 7000 {
            return cuGraphicsUnmapResources_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnregisterResource", 0) => {
        if version >= 3000 {
            return cuGraphicsUnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnregisterResource", 1) => {
        if version >= 3000 {
            return cuGraphicsUnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsUnregisterResource", 2) => {
        if version >= 3000 {
            return cuGraphicsUnregisterResource as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterOutputSurface", 0) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterOutputSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterOutputSurface", 1) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterOutputSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterOutputSurface", 2) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterOutputSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterVideoSurface", 0) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterVideoSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterVideoSurface", 1) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterVideoSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGraphicsVDPAURegisterVideoSurface", 2) => {
        if version >= 3010 {
            return cuGraphicsVDPAURegisterVideoSurface as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxCreate", 0) => {
        if version >= 12040 {
            return cuGreenCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxCreate", 1) => {
        if version >= 12040 {
            return cuGreenCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxCreate", 2) => {
        if version >= 12040 {
            return cuGreenCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxDestroy", 0) => {
        if version >= 12040 {
            return cuGreenCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxDestroy", 1) => {
        if version >= 12040 {
            return cuGreenCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxDestroy", 2) => {
        if version >= 12040 {
            return cuGreenCtxDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxGetDevResource", 0) => {
        if version >= 12040 {
            return cuGreenCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxGetDevResource", 1) => {
        if version >= 12040 {
            return cuGreenCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxGetDevResource", 2) => {
        if version >= 12040 {
            return cuGreenCtxGetDevResource as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxRecordEvent", 0) => {
        if version >= 12040 {
            return cuGreenCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxRecordEvent", 1) => {
        if version >= 12040 {
            return cuGreenCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxRecordEvent", 2) => {
        if version >= 12040 {
            return cuGreenCtxRecordEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxStreamCreate", 0) => {
        if version >= 12050 {
            return cuGreenCtxStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxStreamCreate", 1) => {
        if version >= 12050 {
            return cuGreenCtxStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxStreamCreate", 2) => {
        if version >= 12050 {
            return cuGreenCtxStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxWaitEvent", 0) => {
        if version >= 12040 {
            return cuGreenCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxWaitEvent", 1) => {
        if version >= 12040 {
            return cuGreenCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuGreenCtxWaitEvent", 2) => {
        if version >= 12040 {
            return cuGreenCtxWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalMemory", 0) => {
        if version >= 10000 {
            return cuImportExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalMemory", 1) => {
        if version >= 10000 {
            return cuImportExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalMemory", 2) => {
        if version >= 10000 {
            return cuImportExternalMemory as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalSemaphore", 0) => {
        if version >= 10000 {
            return cuImportExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalSemaphore", 1) => {
        if version >= 10000 {
            return cuImportExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuImportExternalSemaphore", 2) => {
        if version >= 10000 {
            return cuImportExternalSemaphore as _;
        }
        usize::MAX as _
    }
    (b"cuInit", 0) => {
        if version >= 2000 {
            return cuInit as _;
        }
        usize::MAX as _
    }
    (b"cuInit", 1) => {
        if version >= 2000 {
            return cuInit as _;
        }
        usize::MAX as _
    }
    (b"cuInit", 2) => {
        if version >= 2000 {
            return cuInit as _;
        }
        usize::MAX as _
    }
    (b"cuIpcCloseMemHandle", 0) => {
        if version >= 4010 {
            return cuIpcCloseMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcCloseMemHandle", 1) => {
        if version >= 4010 {
            return cuIpcCloseMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcCloseMemHandle", 2) => {
        if version >= 4010 {
            return cuIpcCloseMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetEventHandle", 0) => {
        if version >= 4010 {
            return cuIpcGetEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetEventHandle", 1) => {
        if version >= 4010 {
            return cuIpcGetEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetEventHandle", 2) => {
        if version >= 4010 {
            return cuIpcGetEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetMemHandle", 0) => {
        if version >= 4010 {
            return cuIpcGetMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetMemHandle", 1) => {
        if version >= 4010 {
            return cuIpcGetMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcGetMemHandle", 2) => {
        if version >= 4010 {
            return cuIpcGetMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenEventHandle", 0) => {
        if version >= 4010 {
            return cuIpcOpenEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenEventHandle", 1) => {
        if version >= 4010 {
            return cuIpcOpenEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenEventHandle", 2) => {
        if version >= 4010 {
            return cuIpcOpenEventHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenMemHandle", 0) => {
        if version >= 11000 {
            return cuIpcOpenMemHandle_v2 as _;
        }
        if version >= 4010 {
            return cuIpcOpenMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenMemHandle", 1) => {
        if version >= 11000 {
            return cuIpcOpenMemHandle_v2 as _;
        }
        if version >= 4010 {
            return cuIpcOpenMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuIpcOpenMemHandle", 2) => {
        if version >= 11000 {
            return cuIpcOpenMemHandle_v2 as _;
        }
        if version >= 4010 {
            return cuIpcOpenMemHandle as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetAttribute", 0) => {
        if version >= 12000 {
            return cuKernelGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetAttribute", 1) => {
        if version >= 12000 {
            return cuKernelGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetAttribute", 2) => {
        if version >= 12000 {
            return cuKernelGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetFunction", 0) => {
        if version >= 12000 {
            return cuKernelGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetFunction", 1) => {
        if version >= 12000 {
            return cuKernelGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetFunction", 2) => {
        if version >= 12000 {
            return cuKernelGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetLibrary", 0) => {
        if version >= 12050 {
            return cuKernelGetLibrary as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetLibrary", 1) => {
        if version >= 12050 {
            return cuKernelGetLibrary as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetLibrary", 2) => {
        if version >= 12050 {
            return cuKernelGetLibrary as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetName", 0) => {
        if version >= 12030 {
            return cuKernelGetName as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetName", 1) => {
        if version >= 12030 {
            return cuKernelGetName as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetName", 2) => {
        if version >= 12030 {
            return cuKernelGetName as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetParamInfo", 0) => {
        if version >= 12040 {
            return cuKernelGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetParamInfo", 1) => {
        if version >= 12040 {
            return cuKernelGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuKernelGetParamInfo", 2) => {
        if version >= 12040 {
            return cuKernelGetParamInfo as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetAttribute", 0) => {
        if version >= 12000 {
            return cuKernelSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetAttribute", 1) => {
        if version >= 12000 {
            return cuKernelSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetAttribute", 2) => {
        if version >= 12000 {
            return cuKernelSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetCacheConfig", 0) => {
        if version >= 12000 {
            return cuKernelSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetCacheConfig", 1) => {
        if version >= 12000 {
            return cuKernelSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuKernelSetCacheConfig", 2) => {
        if version >= 12000 {
            return cuKernelSetCacheConfig as _;
        }
        usize::MAX as _
    }
    (b"cuLaunch", 0) => {
        if version >= 2000 {
            return cuLaunch as _;
        }
        usize::MAX as _
    }
    (b"cuLaunch", 1) => {
        if version >= 2000 {
            return cuLaunch as _;
        }
        usize::MAX as _
    }
    (b"cuLaunch", 2) => {
        if version >= 2000 {
            return cuLaunch as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernel", 0) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernel", 1) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernel", 2) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernel_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernelMultiDevice", 0) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernelMultiDevice as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernelMultiDevice", 1) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernelMultiDevice as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchCooperativeKernelMultiDevice", 2) => {
        if version >= 9000 {
            return cuLaunchCooperativeKernelMultiDevice as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGrid", 0) => {
        if version >= 2000 {
            return cuLaunchGrid as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGrid", 1) => {
        if version >= 2000 {
            return cuLaunchGrid as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGrid", 2) => {
        if version >= 2000 {
            return cuLaunchGrid as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGridAsync", 0) => {
        if version >= 2000 {
            return cuLaunchGridAsync as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGridAsync", 1) => {
        if version >= 2000 {
            return cuLaunchGridAsync as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchGridAsync", 2) => {
        if version >= 2000 {
            return cuLaunchGridAsync as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchHostFunc", 0) => {
        if version >= 10000 {
            return cuLaunchHostFunc as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchHostFunc", 1) => {
        if version >= 10000 {
            return cuLaunchHostFunc as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchHostFunc", 2) => {
        if version >= 10000 {
            return cuLaunchHostFunc_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernel", 0) => {
        if version >= 4000 {
            return cuLaunchKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernel", 1) => {
        if version >= 4000 {
            return cuLaunchKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernel", 2) => {
        if version >= 7000 {
            return cuLaunchKernel_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernelEx", 0) => {
        if version >= 11060 {
            return cuLaunchKernelEx as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernelEx", 1) => {
        if version >= 11060 {
            return cuLaunchKernelEx as _;
        }
        usize::MAX as _
    }
    (b"cuLaunchKernelEx", 2) => {
        if version >= 11060 {
            return cuLaunchKernelEx_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryEnumerateKernels", 0) => {
        if version >= 12040 {
            return cuLibraryEnumerateKernels as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryEnumerateKernels", 1) => {
        if version >= 12040 {
            return cuLibraryEnumerateKernels as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryEnumerateKernels", 2) => {
        if version >= 12040 {
            return cuLibraryEnumerateKernels as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetGlobal", 0) => {
        if version >= 12000 {
            return cuLibraryGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetGlobal", 1) => {
        if version >= 12000 {
            return cuLibraryGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetGlobal", 2) => {
        if version >= 12000 {
            return cuLibraryGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernel", 0) => {
        if version >= 12000 {
            return cuLibraryGetKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernel", 1) => {
        if version >= 12000 {
            return cuLibraryGetKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernel", 2) => {
        if version >= 12000 {
            return cuLibraryGetKernel as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernelCount", 0) => {
        if version >= 12040 {
            return cuLibraryGetKernelCount as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernelCount", 1) => {
        if version >= 12040 {
            return cuLibraryGetKernelCount as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetKernelCount", 2) => {
        if version >= 12040 {
            return cuLibraryGetKernelCount as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetManaged", 0) => {
        if version >= 12000 {
            return cuLibraryGetManaged as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetManaged", 1) => {
        if version >= 12000 {
            return cuLibraryGetManaged as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetManaged", 2) => {
        if version >= 12000 {
            return cuLibraryGetManaged as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetModule", 0) => {
        if version >= 12000 {
            return cuLibraryGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetModule", 1) => {
        if version >= 12000 {
            return cuLibraryGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetModule", 2) => {
        if version >= 12000 {
            return cuLibraryGetModule as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetUnifiedFunction", 0) => {
        if version >= 12000 {
            return cuLibraryGetUnifiedFunction as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetUnifiedFunction", 1) => {
        if version >= 12000 {
            return cuLibraryGetUnifiedFunction as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryGetUnifiedFunction", 2) => {
        if version >= 12000 {
            return cuLibraryGetUnifiedFunction as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadData", 0) => {
        if version >= 12000 {
            return cuLibraryLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadData", 1) => {
        if version >= 12000 {
            return cuLibraryLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadData", 2) => {
        if version >= 12000 {
            return cuLibraryLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadFromFile", 0) => {
        if version >= 12000 {
            return cuLibraryLoadFromFile as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadFromFile", 1) => {
        if version >= 12000 {
            return cuLibraryLoadFromFile as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryLoadFromFile", 2) => {
        if version >= 12000 {
            return cuLibraryLoadFromFile as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryUnload", 0) => {
        if version >= 12000 {
            return cuLibraryUnload as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryUnload", 1) => {
        if version >= 12000 {
            return cuLibraryUnload as _;
        }
        usize::MAX as _
    }
    (b"cuLibraryUnload", 2) => {
        if version >= 12000 {
            return cuLibraryUnload as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddData", 0) => {
        if version >= 6050 {
            return cuLinkAddData_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddData as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddData", 1) => {
        if version >= 6050 {
            return cuLinkAddData_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddData as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddData", 2) => {
        if version >= 6050 {
            return cuLinkAddData_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddData as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddFile", 0) => {
        if version >= 6050 {
            return cuLinkAddFile_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddFile as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddFile", 1) => {
        if version >= 6050 {
            return cuLinkAddFile_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddFile as _;
        }
        usize::MAX as _
    }
    (b"cuLinkAddFile", 2) => {
        if version >= 6050 {
            return cuLinkAddFile_v2 as _;
        }
        if version >= 5050 {
            return cuLinkAddFile as _;
        }
        usize::MAX as _
    }
    (b"cuLinkComplete", 0) => {
        if version >= 5050 {
            return cuLinkComplete as _;
        }
        usize::MAX as _
    }
    (b"cuLinkComplete", 1) => {
        if version >= 5050 {
            return cuLinkComplete as _;
        }
        usize::MAX as _
    }
    (b"cuLinkComplete", 2) => {
        if version >= 5050 {
            return cuLinkComplete as _;
        }
        usize::MAX as _
    }
    (b"cuLinkCreate", 0) => {
        if version >= 6050 {
            return cuLinkCreate_v2 as _;
        }
        if version >= 5050 {
            return cuLinkCreate as _;
        }
        usize::MAX as _
    }
    (b"cuLinkCreate", 1) => {
        if version >= 6050 {
            return cuLinkCreate_v2 as _;
        }
        if version >= 5050 {
            return cuLinkCreate as _;
        }
        usize::MAX as _
    }
    (b"cuLinkCreate", 2) => {
        if version >= 6050 {
            return cuLinkCreate_v2 as _;
        }
        if version >= 5050 {
            return cuLinkCreate as _;
        }
        usize::MAX as _
    }
    (b"cuLinkDestroy", 0) => {
        if version >= 5050 {
            return cuLinkDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuLinkDestroy", 1) => {
        if version >= 5050 {
            return cuLinkDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuLinkDestroy", 2) => {
        if version >= 5050 {
            return cuLinkDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressFree", 0) => {
        if version >= 10020 {
            return cuMemAddressFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressFree", 1) => {
        if version >= 10020 {
            return cuMemAddressFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressFree", 2) => {
        if version >= 10020 {
            return cuMemAddressFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressReserve", 0) => {
        if version >= 10020 {
            return cuMemAddressReserve as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressReserve", 1) => {
        if version >= 10020 {
            return cuMemAddressReserve as _;
        }
        usize::MAX as _
    }
    (b"cuMemAddressReserve", 2) => {
        if version >= 10020 {
            return cuMemAddressReserve as _;
        }
        usize::MAX as _
    }
    (b"cuMemAdvise", 0) => {
        if version >= 12020 {
            return cuMemAdvise_v2 as _;
        }
        if version >= 8000 {
            return cuMemAdvise as _;
        }
        usize::MAX as _
    }
    (b"cuMemAdvise", 1) => {
        if version >= 12020 {
            return cuMemAdvise_v2 as _;
        }
        if version >= 8000 {
            return cuMemAdvise as _;
        }
        usize::MAX as _
    }
    (b"cuMemAdvise", 2) => {
        if version >= 12020 {
            return cuMemAdvise_v2 as _;
        }
        if version >= 8000 {
            return cuMemAdvise as _;
        }
        usize::MAX as _
    }
    (b"cuMemAlloc", 0) => {
        if version >= 3020 {
            return cuMemAlloc_v2 as _;
        }
        if version >= 2000 {
            return cuMemAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemAlloc", 1) => {
        if version >= 3020 {
            return cuMemAlloc_v2 as _;
        }
        if version >= 2000 {
            return cuMemAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemAlloc", 2) => {
        if version >= 3020 {
            return cuMemAlloc_v2 as _;
        }
        if version >= 2000 {
            return cuMemAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocAsync", 0) => {
        if version >= 11020 {
            return cuMemAllocAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocAsync", 1) => {
        if version >= 11020 {
            return cuMemAllocAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocAsync", 2) => {
        if version >= 11020 {
            return cuMemAllocAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocFromPoolAsync", 0) => {
        if version >= 11020 {
            return cuMemAllocFromPoolAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocFromPoolAsync", 1) => {
        if version >= 11020 {
            return cuMemAllocFromPoolAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocFromPoolAsync", 2) => {
        if version >= 11020 {
            return cuMemAllocFromPoolAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocHost", 0) => {
        if version >= 3020 {
            return cuMemAllocHost_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocHost", 1) => {
        if version >= 3020 {
            return cuMemAllocHost_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocHost", 2) => {
        if version >= 3020 {
            return cuMemAllocHost_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocManaged", 0) => {
        if version >= 6000 {
            return cuMemAllocManaged as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocManaged", 1) => {
        if version >= 6000 {
            return cuMemAllocManaged as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocManaged", 2) => {
        if version >= 6000 {
            return cuMemAllocManaged as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocPitch", 0) => {
        if version >= 3020 {
            return cuMemAllocPitch_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocPitch as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocPitch", 1) => {
        if version >= 3020 {
            return cuMemAllocPitch_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocPitch as _;
        }
        usize::MAX as _
    }
    (b"cuMemAllocPitch", 2) => {
        if version >= 3020 {
            return cuMemAllocPitch_v2 as _;
        }
        if version >= 2000 {
            return cuMemAllocPitch as _;
        }
        usize::MAX as _
    }
    (b"cuMemBatchDecompressAsync", 0) => {
        if version >= 12060 {
            return cuMemBatchDecompressAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemBatchDecompressAsync", 1) => {
        if version >= 12060 {
            return cuMemBatchDecompressAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemBatchDecompressAsync", 2) => {
        if version >= 12060 {
            return cuMemBatchDecompressAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemCreate", 0) => {
        if version >= 10020 {
            return cuMemCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemCreate", 1) => {
        if version >= 10020 {
            return cuMemCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemCreate", 2) => {
        if version >= 10020 {
            return cuMemCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemExportToShareableHandle", 0) => {
        if version >= 10020 {
            return cuMemExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemExportToShareableHandle", 1) => {
        if version >= 10020 {
            return cuMemExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemExportToShareableHandle", 2) => {
        if version >= 10020 {
            return cuMemExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemFree", 0) => {
        if version >= 3020 {
            return cuMemFree_v2 as _;
        }
        if version >= 2000 {
            return cuMemFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemFree", 1) => {
        if version >= 3020 {
            return cuMemFree_v2 as _;
        }
        if version >= 2000 {
            return cuMemFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemFree", 2) => {
        if version >= 3020 {
            return cuMemFree_v2 as _;
        }
        if version >= 2000 {
            return cuMemFree as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeAsync", 0) => {
        if version >= 11020 {
            return cuMemFreeAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeAsync", 1) => {
        if version >= 11020 {
            return cuMemFreeAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeAsync", 2) => {
        if version >= 11020 {
            return cuMemFreeAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeHost", 0) => {
        if version >= 2000 {
            return cuMemFreeHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeHost", 1) => {
        if version >= 2000 {
            return cuMemFreeHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemFreeHost", 2) => {
        if version >= 2000 {
            return cuMemFreeHost as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAccess", 0) => {
        if version >= 10020 {
            return cuMemGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAccess", 1) => {
        if version >= 10020 {
            return cuMemGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAccess", 2) => {
        if version >= 10020 {
            return cuMemGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAddressRange", 0) => {
        if version >= 3020 {
            return cuMemGetAddressRange_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAddressRange", 1) => {
        if version >= 3020 {
            return cuMemGetAddressRange_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAddressRange", 2) => {
        if version >= 3020 {
            return cuMemGetAddressRange_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationGranularity", 0) => {
        if version >= 10020 {
            return cuMemGetAllocationGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationGranularity", 1) => {
        if version >= 10020 {
            return cuMemGetAllocationGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationGranularity", 2) => {
        if version >= 10020 {
            return cuMemGetAllocationGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationPropertiesFromHandle", 0) => {
        if version >= 10020 {
            return cuMemGetAllocationPropertiesFromHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationPropertiesFromHandle", 1) => {
        if version >= 10020 {
            return cuMemGetAllocationPropertiesFromHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetAllocationPropertiesFromHandle", 2) => {
        if version >= 10020 {
            return cuMemGetAllocationPropertiesFromHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetHandleForAddressRange", 0) => {
        if version >= 11070 {
            return cuMemGetHandleForAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetHandleForAddressRange", 1) => {
        if version >= 11070 {
            return cuMemGetHandleForAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetHandleForAddressRange", 2) => {
        if version >= 11070 {
            return cuMemGetHandleForAddressRange as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetInfo", 0) => {
        if version >= 3020 {
            return cuMemGetInfo_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetInfo as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetInfo", 1) => {
        if version >= 3020 {
            return cuMemGetInfo_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetInfo as _;
        }
        usize::MAX as _
    }
    (b"cuMemGetInfo", 2) => {
        if version >= 3020 {
            return cuMemGetInfo_v2 as _;
        }
        if version >= 2000 {
            return cuMemGetInfo as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostAlloc", 0) => {
        if version >= 2020 {
            return cuMemHostAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostAlloc", 1) => {
        if version >= 2020 {
            return cuMemHostAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostAlloc", 2) => {
        if version >= 2020 {
            return cuMemHostAlloc as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetDevicePointer", 0) => {
        if version >= 3020 {
            return cuMemHostGetDevicePointer_v2 as _;
        }
        if version >= 2020 {
            return cuMemHostGetDevicePointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetDevicePointer", 1) => {
        if version >= 3020 {
            return cuMemHostGetDevicePointer_v2 as _;
        }
        if version >= 2020 {
            return cuMemHostGetDevicePointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetDevicePointer", 2) => {
        if version >= 3020 {
            return cuMemHostGetDevicePointer_v2 as _;
        }
        if version >= 2020 {
            return cuMemHostGetDevicePointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetFlags", 0) => {
        if version >= 2030 {
            return cuMemHostGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetFlags", 1) => {
        if version >= 2030 {
            return cuMemHostGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostGetFlags", 2) => {
        if version >= 2030 {
            return cuMemHostGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostRegister", 0) => {
        if version >= 6050 {
            return cuMemHostRegister_v2 as _;
        }
        if version >= 4000 {
            return cuMemHostRegister as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostRegister", 1) => {
        if version >= 6050 {
            return cuMemHostRegister_v2 as _;
        }
        if version >= 4000 {
            return cuMemHostRegister as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostRegister", 2) => {
        if version >= 6050 {
            return cuMemHostRegister_v2 as _;
        }
        if version >= 4000 {
            return cuMemHostRegister as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostUnregister", 0) => {
        if version >= 4000 {
            return cuMemHostUnregister as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostUnregister", 1) => {
        if version >= 4000 {
            return cuMemHostUnregister as _;
        }
        usize::MAX as _
    }
    (b"cuMemHostUnregister", 2) => {
        if version >= 4000 {
            return cuMemHostUnregister as _;
        }
        usize::MAX as _
    }
    (b"cuMemImportFromShareableHandle", 0) => {
        if version >= 10020 {
            return cuMemImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemImportFromShareableHandle", 1) => {
        if version >= 10020 {
            return cuMemImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemImportFromShareableHandle", 2) => {
        if version >= 10020 {
            return cuMemImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemMap", 0) => {
        if version >= 10020 {
            return cuMemMap as _;
        }
        usize::MAX as _
    }
    (b"cuMemMap", 1) => {
        if version >= 10020 {
            return cuMemMap as _;
        }
        usize::MAX as _
    }
    (b"cuMemMap", 2) => {
        if version >= 10020 {
            return cuMemMap as _;
        }
        usize::MAX as _
    }
    (b"cuMemMapArrayAsync", 0) => {
        if version >= 11010 {
            return cuMemMapArrayAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemMapArrayAsync", 1) => {
        if version >= 11010 {
            return cuMemMapArrayAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemMapArrayAsync", 2) => {
        if version >= 11010 {
            return cuMemMapArrayAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolCreate", 0) => {
        if version >= 11020 {
            return cuMemPoolCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolCreate", 1) => {
        if version >= 11020 {
            return cuMemPoolCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolCreate", 2) => {
        if version >= 11020 {
            return cuMemPoolCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolDestroy", 0) => {
        if version >= 11020 {
            return cuMemPoolDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolDestroy", 1) => {
        if version >= 11020 {
            return cuMemPoolDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolDestroy", 2) => {
        if version >= 11020 {
            return cuMemPoolDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportPointer", 0) => {
        if version >= 11020 {
            return cuMemPoolExportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportPointer", 1) => {
        if version >= 11020 {
            return cuMemPoolExportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportPointer", 2) => {
        if version >= 11020 {
            return cuMemPoolExportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportToShareableHandle", 0) => {
        if version >= 11020 {
            return cuMemPoolExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportToShareableHandle", 1) => {
        if version >= 11020 {
            return cuMemPoolExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolExportToShareableHandle", 2) => {
        if version >= 11020 {
            return cuMemPoolExportToShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAccess", 0) => {
        if version >= 11020 {
            return cuMemPoolGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAccess", 1) => {
        if version >= 11020 {
            return cuMemPoolGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAccess", 2) => {
        if version >= 11020 {
            return cuMemPoolGetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAttribute", 0) => {
        if version >= 11020 {
            return cuMemPoolGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAttribute", 1) => {
        if version >= 11020 {
            return cuMemPoolGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolGetAttribute", 2) => {
        if version >= 11020 {
            return cuMemPoolGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportFromShareableHandle", 0) => {
        if version >= 11020 {
            return cuMemPoolImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportFromShareableHandle", 1) => {
        if version >= 11020 {
            return cuMemPoolImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportFromShareableHandle", 2) => {
        if version >= 11020 {
            return cuMemPoolImportFromShareableHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportPointer", 0) => {
        if version >= 11020 {
            return cuMemPoolImportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportPointer", 1) => {
        if version >= 11020 {
            return cuMemPoolImportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolImportPointer", 2) => {
        if version >= 11020 {
            return cuMemPoolImportPointer as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAccess", 0) => {
        if version >= 11020 {
            return cuMemPoolSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAccess", 1) => {
        if version >= 11020 {
            return cuMemPoolSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAccess", 2) => {
        if version >= 11020 {
            return cuMemPoolSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAttribute", 0) => {
        if version >= 11020 {
            return cuMemPoolSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAttribute", 1) => {
        if version >= 11020 {
            return cuMemPoolSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolSetAttribute", 2) => {
        if version >= 11020 {
            return cuMemPoolSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolTrimTo", 0) => {
        if version >= 11020 {
            return cuMemPoolTrimTo as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolTrimTo", 1) => {
        if version >= 11020 {
            return cuMemPoolTrimTo as _;
        }
        usize::MAX as _
    }
    (b"cuMemPoolTrimTo", 2) => {
        if version >= 11020 {
            return cuMemPoolTrimTo as _;
        }
        usize::MAX as _
    }
    (b"cuMemPrefetchAsync", 0) => {
        if version >= 12020 {
            return cuMemPrefetchAsync_v2 as _;
        }
        if version >= 8000 {
            return cuMemPrefetchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemPrefetchAsync", 1) => {
        if version >= 12020 {
            return cuMemPrefetchAsync_v2 as _;
        }
        if version >= 8000 {
            return cuMemPrefetchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemPrefetchAsync", 2) => {
        if version >= 12020 {
            return cuMemPrefetchAsync_v2_ptsz as _;
        }
        if version >= 8000 {
            return cuMemPrefetchAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttribute", 0) => {
        if version >= 8000 {
            return cuMemRangeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttribute", 1) => {
        if version >= 8000 {
            return cuMemRangeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttribute", 2) => {
        if version >= 8000 {
            return cuMemRangeGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttributes", 0) => {
        if version >= 8000 {
            return cuMemRangeGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttributes", 1) => {
        if version >= 8000 {
            return cuMemRangeGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuMemRangeGetAttributes", 2) => {
        if version >= 8000 {
            return cuMemRangeGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuMemRelease", 0) => {
        if version >= 10020 {
            return cuMemRelease as _;
        }
        usize::MAX as _
    }
    (b"cuMemRelease", 1) => {
        if version >= 10020 {
            return cuMemRelease as _;
        }
        usize::MAX as _
    }
    (b"cuMemRelease", 2) => {
        if version >= 10020 {
            return cuMemRelease as _;
        }
        usize::MAX as _
    }
    (b"cuMemRetainAllocationHandle", 0) => {
        if version >= 11000 {
            return cuMemRetainAllocationHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemRetainAllocationHandle", 1) => {
        if version >= 11000 {
            return cuMemRetainAllocationHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemRetainAllocationHandle", 2) => {
        if version >= 11000 {
            return cuMemRetainAllocationHandle as _;
        }
        usize::MAX as _
    }
    (b"cuMemSetAccess", 0) => {
        if version >= 10020 {
            return cuMemSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemSetAccess", 1) => {
        if version >= 10020 {
            return cuMemSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemSetAccess", 2) => {
        if version >= 10020 {
            return cuMemSetAccess as _;
        }
        usize::MAX as _
    }
    (b"cuMemUnmap", 0) => {
        if version >= 10020 {
            return cuMemUnmap as _;
        }
        usize::MAX as _
    }
    (b"cuMemUnmap", 1) => {
        if version >= 10020 {
            return cuMemUnmap as _;
        }
        usize::MAX as _
    }
    (b"cuMemUnmap", 2) => {
        if version >= 10020 {
            return cuMemUnmap as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy", 0) => {
        if version >= 4000 {
            return cuMemcpy as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy", 1) => {
        if version >= 4000 {
            return cuMemcpy as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy", 2) => {
        if version >= 7000 {
            return cuMemcpy_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2D", 0) => {
        if version >= 3020 {
            return cuMemcpy2D_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2D as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2D", 1) => {
        if version >= 3020 {
            return cuMemcpy2D_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2D as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2D", 2) => {
        if version >= 7000 {
            return cuMemcpy2D_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DAsync", 0) => {
        if version >= 3020 {
            return cuMemcpy2DAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2DAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DAsync", 1) => {
        if version >= 3020 {
            return cuMemcpy2DAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2DAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DAsync", 2) => {
        if version >= 7000 {
            return cuMemcpy2DAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DUnaligned", 0) => {
        if version >= 3020 {
            return cuMemcpy2DUnaligned_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2DUnaligned as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DUnaligned", 1) => {
        if version >= 3020 {
            return cuMemcpy2DUnaligned_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy2DUnaligned as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy2DUnaligned", 2) => {
        if version >= 7000 {
            return cuMemcpy2DUnaligned_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3D", 0) => {
        if version >= 3020 {
            return cuMemcpy3D_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy3D as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3D", 1) => {
        if version >= 3020 {
            return cuMemcpy3D_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy3D as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3D", 2) => {
        if version >= 7000 {
            return cuMemcpy3D_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DAsync", 0) => {
        if version >= 3020 {
            return cuMemcpy3DAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy3DAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DAsync", 1) => {
        if version >= 3020 {
            return cuMemcpy3DAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpy3DAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DAsync", 2) => {
        if version >= 7000 {
            return cuMemcpy3DAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DBatchAsync", 0) => {
        if version >= 12080 {
            return cuMemcpy3DBatchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DBatchAsync", 1) => {
        if version >= 12080 {
            return cuMemcpy3DBatchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DBatchAsync", 2) => {
        if version >= 12080 {
            return cuMemcpy3DBatchAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeer", 0) => {
        if version >= 4000 {
            return cuMemcpy3DPeer as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeer", 1) => {
        if version >= 4000 {
            return cuMemcpy3DPeer as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeer", 2) => {
        if version >= 7000 {
            return cuMemcpy3DPeer_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeerAsync", 0) => {
        if version >= 4000 {
            return cuMemcpy3DPeerAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeerAsync", 1) => {
        if version >= 4000 {
            return cuMemcpy3DPeerAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpy3DPeerAsync", 2) => {
        if version >= 7000 {
            return cuMemcpy3DPeerAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAsync", 0) => {
        if version >= 4000 {
            return cuMemcpyAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAsync", 1) => {
        if version >= 4000 {
            return cuMemcpyAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoA", 0) => {
        if version >= 3020 {
            return cuMemcpyAtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoA", 1) => {
        if version >= 3020 {
            return cuMemcpyAtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoA", 2) => {
        if version >= 7000 {
            return cuMemcpyAtoA_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoD", 0) => {
        if version >= 3020 {
            return cuMemcpyAtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoD", 1) => {
        if version >= 3020 {
            return cuMemcpyAtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoD", 2) => {
        if version >= 7000 {
            return cuMemcpyAtoD_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoH", 0) => {
        if version >= 3020 {
            return cuMemcpyAtoH_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoH as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoH", 1) => {
        if version >= 3020 {
            return cuMemcpyAtoH_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoH as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoH", 2) => {
        if version >= 7000 {
            return cuMemcpyAtoH_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoHAsync", 0) => {
        if version >= 3020 {
            return cuMemcpyAtoHAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoHAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoHAsync", 1) => {
        if version >= 3020 {
            return cuMemcpyAtoHAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyAtoHAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyAtoHAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyAtoHAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyBatchAsync", 0) => {
        if version >= 12080 {
            return cuMemcpyBatchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyBatchAsync", 1) => {
        if version >= 12080 {
            return cuMemcpyBatchAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyBatchAsync", 2) => {
        if version >= 12080 {
            return cuMemcpyBatchAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoA", 0) => {
        if version >= 3020 {
            return cuMemcpyDtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoA", 1) => {
        if version >= 3020 {
            return cuMemcpyDtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoA", 2) => {
        if version >= 7000 {
            return cuMemcpyDtoA_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoD", 0) => {
        if version >= 3020 {
            return cuMemcpyDtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoD", 1) => {
        if version >= 3020 {
            return cuMemcpyDtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoD", 2) => {
        if version >= 7000 {
            return cuMemcpyDtoD_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoDAsync", 0) => {
        if version >= 3020 {
            return cuMemcpyDtoDAsync_v2 as _;
        }
        if version >= 3000 {
            return cuMemcpyDtoDAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoDAsync", 1) => {
        if version >= 3020 {
            return cuMemcpyDtoDAsync_v2 as _;
        }
        if version >= 3000 {
            return cuMemcpyDtoDAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoDAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyDtoDAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoH", 0) => {
        if version >= 3020 {
            return cuMemcpyDtoH_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoH as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoH", 1) => {
        if version >= 3020 {
            return cuMemcpyDtoH_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoH as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoH", 2) => {
        if version >= 7000 {
            return cuMemcpyDtoH_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoHAsync", 0) => {
        if version >= 3020 {
            return cuMemcpyDtoHAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoHAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoHAsync", 1) => {
        if version >= 3020 {
            return cuMemcpyDtoHAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyDtoHAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyDtoHAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyDtoHAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoA", 0) => {
        if version >= 3020 {
            return cuMemcpyHtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoA", 1) => {
        if version >= 3020 {
            return cuMemcpyHtoA_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoA as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoA", 2) => {
        if version >= 7000 {
            return cuMemcpyHtoA_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoAAsync", 0) => {
        if version >= 3020 {
            return cuMemcpyHtoAAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoAAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoAAsync", 1) => {
        if version >= 3020 {
            return cuMemcpyHtoAAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoAAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoAAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyHtoAAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoD", 0) => {
        if version >= 3020 {
            return cuMemcpyHtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoD", 1) => {
        if version >= 3020 {
            return cuMemcpyHtoD_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoD as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoD", 2) => {
        if version >= 7000 {
            return cuMemcpyHtoD_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoDAsync", 0) => {
        if version >= 3020 {
            return cuMemcpyHtoDAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoDAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoDAsync", 1) => {
        if version >= 3020 {
            return cuMemcpyHtoDAsync_v2 as _;
        }
        if version >= 2000 {
            return cuMemcpyHtoDAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyHtoDAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyHtoDAsync_v2_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeer", 0) => {
        if version >= 4000 {
            return cuMemcpyPeer as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeer", 1) => {
        if version >= 4000 {
            return cuMemcpyPeer as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeer", 2) => {
        if version >= 7000 {
            return cuMemcpyPeer_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeerAsync", 0) => {
        if version >= 4000 {
            return cuMemcpyPeerAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeerAsync", 1) => {
        if version >= 4000 {
            return cuMemcpyPeerAsync as _;
        }
        usize::MAX as _
    }
    (b"cuMemcpyPeerAsync", 2) => {
        if version >= 7000 {
            return cuMemcpyPeerAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16", 0) => {
        if version >= 3020 {
            return cuMemsetD16_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD16 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16", 1) => {
        if version >= 3020 {
            return cuMemsetD16_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD16 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16", 2) => {
        if version >= 7000 {
            return cuMemsetD16_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16Async", 0) => {
        if version >= 3020 {
            return cuMemsetD16Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16Async", 1) => {
        if version >= 3020 {
            return cuMemsetD16Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD16Async", 2) => {
        if version >= 7000 {
            return cuMemsetD16Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16", 0) => {
        if version >= 3020 {
            return cuMemsetD2D16_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D16 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16", 1) => {
        if version >= 3020 {
            return cuMemsetD2D16_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D16 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16", 2) => {
        if version >= 7000 {
            return cuMemsetD2D16_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16Async", 0) => {
        if version >= 3020 {
            return cuMemsetD2D16Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16Async", 1) => {
        if version >= 3020 {
            return cuMemsetD2D16Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D16Async", 2) => {
        if version >= 7000 {
            return cuMemsetD2D16Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32", 0) => {
        if version >= 3020 {
            return cuMemsetD2D32_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D32 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32", 1) => {
        if version >= 3020 {
            return cuMemsetD2D32_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D32 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32", 2) => {
        if version >= 7000 {
            return cuMemsetD2D32_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32Async", 0) => {
        if version >= 3020 {
            return cuMemsetD2D32Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32Async", 1) => {
        if version >= 3020 {
            return cuMemsetD2D32Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D32Async", 2) => {
        if version >= 7000 {
            return cuMemsetD2D32Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8", 0) => {
        if version >= 3020 {
            return cuMemsetD2D8_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D8 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8", 1) => {
        if version >= 3020 {
            return cuMemsetD2D8_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD2D8 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8", 2) => {
        if version >= 7000 {
            return cuMemsetD2D8_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8Async", 0) => {
        if version >= 3020 {
            return cuMemsetD2D8Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8Async", 1) => {
        if version >= 3020 {
            return cuMemsetD2D8Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD2D8Async", 2) => {
        if version >= 7000 {
            return cuMemsetD2D8Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32", 0) => {
        if version >= 3020 {
            return cuMemsetD32_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD32 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32", 1) => {
        if version >= 3020 {
            return cuMemsetD32_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD32 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32", 2) => {
        if version >= 7000 {
            return cuMemsetD32_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32Async", 0) => {
        if version >= 3020 {
            return cuMemsetD32Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32Async", 1) => {
        if version >= 3020 {
            return cuMemsetD32Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD32Async", 2) => {
        if version >= 7000 {
            return cuMemsetD32Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8", 0) => {
        if version >= 3020 {
            return cuMemsetD8_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD8 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8", 1) => {
        if version >= 3020 {
            return cuMemsetD8_v2 as _;
        }
        if version >= 2000 {
            return cuMemsetD8 as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8", 2) => {
        if version >= 7000 {
            return cuMemsetD8_v2_ptds as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8Async", 0) => {
        if version >= 3020 {
            return cuMemsetD8Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8Async", 1) => {
        if version >= 3020 {
            return cuMemsetD8Async as _;
        }
        usize::MAX as _
    }
    (b"cuMemsetD8Async", 2) => {
        if version >= 7000 {
            return cuMemsetD8Async_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayCreate", 0) => {
        if version >= 5000 {
            return cuMipmappedArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayCreate", 1) => {
        if version >= 5000 {
            return cuMipmappedArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayCreate", 2) => {
        if version >= 5000 {
            return cuMipmappedArrayCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayDestroy", 0) => {
        if version >= 5000 {
            return cuMipmappedArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayDestroy", 1) => {
        if version >= 5000 {
            return cuMipmappedArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayDestroy", 2) => {
        if version >= 5000 {
            return cuMipmappedArrayDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetLevel", 0) => {
        if version >= 5000 {
            return cuMipmappedArrayGetLevel as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetLevel", 1) => {
        if version >= 5000 {
            return cuMipmappedArrayGetLevel as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetLevel", 2) => {
        if version >= 5000 {
            return cuMipmappedArrayGetLevel as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetMemoryRequirements", 0) => {
        if version >= 11060 {
            return cuMipmappedArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetMemoryRequirements", 1) => {
        if version >= 11060 {
            return cuMipmappedArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetMemoryRequirements", 2) => {
        if version >= 11060 {
            return cuMipmappedArrayGetMemoryRequirements as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetSparseProperties", 0) => {
        if version >= 11010 {
            return cuMipmappedArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetSparseProperties", 1) => {
        if version >= 11010 {
            return cuMipmappedArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuMipmappedArrayGetSparseProperties", 2) => {
        if version >= 11010 {
            return cuMipmappedArrayGetSparseProperties as _;
        }
        usize::MAX as _
    }
    (b"cuModuleEnumerateFunctions", 0) => {
        if version >= 12040 {
            return cuModuleEnumerateFunctions as _;
        }
        usize::MAX as _
    }
    (b"cuModuleEnumerateFunctions", 1) => {
        if version >= 12040 {
            return cuModuleEnumerateFunctions as _;
        }
        usize::MAX as _
    }
    (b"cuModuleEnumerateFunctions", 2) => {
        if version >= 12040 {
            return cuModuleEnumerateFunctions as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunction", 0) => {
        if version >= 2000 {
            return cuModuleGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunction", 1) => {
        if version >= 2000 {
            return cuModuleGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunction", 2) => {
        if version >= 2000 {
            return cuModuleGetFunction as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunctionCount", 0) => {
        if version >= 12040 {
            return cuModuleGetFunctionCount as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunctionCount", 1) => {
        if version >= 12040 {
            return cuModuleGetFunctionCount as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetFunctionCount", 2) => {
        if version >= 12040 {
            return cuModuleGetFunctionCount as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetGlobal", 0) => {
        if version >= 3020 {
            return cuModuleGetGlobal_v2 as _;
        }
        if version >= 2000 {
            return cuModuleGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetGlobal", 1) => {
        if version >= 3020 {
            return cuModuleGetGlobal_v2 as _;
        }
        if version >= 2000 {
            return cuModuleGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetGlobal", 2) => {
        if version >= 3020 {
            return cuModuleGetGlobal_v2 as _;
        }
        if version >= 2000 {
            return cuModuleGetGlobal as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetLoadingMode", 0) => {
        if version >= 11070 {
            return cuModuleGetLoadingMode as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetLoadingMode", 1) => {
        if version >= 11070 {
            return cuModuleGetLoadingMode as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetLoadingMode", 2) => {
        if version >= 11070 {
            return cuModuleGetLoadingMode as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetSurfRef", 0) => {
        if version >= 3000 {
            return cuModuleGetSurfRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetSurfRef", 1) => {
        if version >= 3000 {
            return cuModuleGetSurfRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetSurfRef", 2) => {
        if version >= 3000 {
            return cuModuleGetSurfRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetTexRef", 0) => {
        if version >= 2000 {
            return cuModuleGetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetTexRef", 1) => {
        if version >= 2000 {
            return cuModuleGetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleGetTexRef", 2) => {
        if version >= 2000 {
            return cuModuleGetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoad", 0) => {
        if version >= 2000 {
            return cuModuleLoad as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoad", 1) => {
        if version >= 2000 {
            return cuModuleLoad as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoad", 2) => {
        if version >= 2000 {
            return cuModuleLoad as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadData", 0) => {
        if version >= 2000 {
            return cuModuleLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadData", 1) => {
        if version >= 2000 {
            return cuModuleLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadData", 2) => {
        if version >= 2000 {
            return cuModuleLoadData as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadDataEx", 0) => {
        if version >= 2010 {
            return cuModuleLoadDataEx as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadDataEx", 1) => {
        if version >= 2010 {
            return cuModuleLoadDataEx as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadDataEx", 2) => {
        if version >= 2010 {
            return cuModuleLoadDataEx as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadFatBinary", 0) => {
        if version >= 2000 {
            return cuModuleLoadFatBinary as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadFatBinary", 1) => {
        if version >= 2000 {
            return cuModuleLoadFatBinary as _;
        }
        usize::MAX as _
    }
    (b"cuModuleLoadFatBinary", 2) => {
        if version >= 2000 {
            return cuModuleLoadFatBinary as _;
        }
        usize::MAX as _
    }
    (b"cuModuleUnload", 0) => {
        if version >= 2000 {
            return cuModuleUnload as _;
        }
        usize::MAX as _
    }
    (b"cuModuleUnload", 1) => {
        if version >= 2000 {
            return cuModuleUnload as _;
        }
        usize::MAX as _
    }
    (b"cuModuleUnload", 2) => {
        if version >= 2000 {
            return cuModuleUnload as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastAddDevice", 0) => {
        if version >= 12010 {
            return cuMulticastAddDevice as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastAddDevice", 1) => {
        if version >= 12010 {
            return cuMulticastAddDevice as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastAddDevice", 2) => {
        if version >= 12010 {
            return cuMulticastAddDevice as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindAddr", 0) => {
        if version >= 12010 {
            return cuMulticastBindAddr as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindAddr", 1) => {
        if version >= 12010 {
            return cuMulticastBindAddr as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindAddr", 2) => {
        if version >= 12010 {
            return cuMulticastBindAddr as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindMem", 0) => {
        if version >= 12010 {
            return cuMulticastBindMem as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindMem", 1) => {
        if version >= 12010 {
            return cuMulticastBindMem as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastBindMem", 2) => {
        if version >= 12010 {
            return cuMulticastBindMem as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastCreate", 0) => {
        if version >= 12010 {
            return cuMulticastCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastCreate", 1) => {
        if version >= 12010 {
            return cuMulticastCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastCreate", 2) => {
        if version >= 12010 {
            return cuMulticastCreate as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastGetGranularity", 0) => {
        if version >= 12010 {
            return cuMulticastGetGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastGetGranularity", 1) => {
        if version >= 12010 {
            return cuMulticastGetGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastGetGranularity", 2) => {
        if version >= 12010 {
            return cuMulticastGetGranularity as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastUnbind", 0) => {
        if version >= 12010 {
            return cuMulticastUnbind as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastUnbind", 1) => {
        if version >= 12010 {
            return cuMulticastUnbind as _;
        }
        usize::MAX as _
    }
    (b"cuMulticastUnbind", 2) => {
        if version >= 12010 {
            return cuMulticastUnbind as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyAvailableDynamicSMemPerBlock", 0) => {
        if version >= 10020 {
            return cuOccupancyAvailableDynamicSMemPerBlock as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyAvailableDynamicSMemPerBlock", 1) => {
        if version >= 10020 {
            return cuOccupancyAvailableDynamicSMemPerBlock as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyAvailableDynamicSMemPerBlock", 2) => {
        if version >= 10020 {
            return cuOccupancyAvailableDynamicSMemPerBlock as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessor", 0) => {
        if version >= 6050 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessor as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessor", 1) => {
        if version >= 6050 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessor as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessor", 2) => {
        if version >= 6050 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessor as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags", 0) => {
        if version >= 7000 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags", 1) => {
        if version >= 7000 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags", 2) => {
        if version >= 7000 {
            return cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveClusters", 0) => {
        if version >= 11070 {
            return cuOccupancyMaxActiveClusters as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveClusters", 1) => {
        if version >= 11070 {
            return cuOccupancyMaxActiveClusters as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxActiveClusters", 2) => {
        if version >= 11070 {
            return cuOccupancyMaxActiveClusters as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSize", 0) => {
        if version >= 6050 {
            return cuOccupancyMaxPotentialBlockSize as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSize", 1) => {
        if version >= 6050 {
            return cuOccupancyMaxPotentialBlockSize as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSize", 2) => {
        if version >= 6050 {
            return cuOccupancyMaxPotentialBlockSize as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSizeWithFlags", 0) => {
        if version >= 7000 {
            return cuOccupancyMaxPotentialBlockSizeWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSizeWithFlags", 1) => {
        if version >= 7000 {
            return cuOccupancyMaxPotentialBlockSizeWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialBlockSizeWithFlags", 2) => {
        if version >= 7000 {
            return cuOccupancyMaxPotentialBlockSizeWithFlags as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialClusterSize", 0) => {
        if version >= 11070 {
            return cuOccupancyMaxPotentialClusterSize as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialClusterSize", 1) => {
        if version >= 11070 {
            return cuOccupancyMaxPotentialClusterSize as _;
        }
        usize::MAX as _
    }
    (b"cuOccupancyMaxPotentialClusterSize", 2) => {
        if version >= 11070 {
            return cuOccupancyMaxPotentialClusterSize as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetSize", 0) => {
        if version >= 2000 {
            return cuParamSetSize as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetSize", 1) => {
        if version >= 2000 {
            return cuParamSetSize as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetSize", 2) => {
        if version >= 2000 {
            return cuParamSetSize as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetTexRef", 0) => {
        if version >= 2000 {
            return cuParamSetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetTexRef", 1) => {
        if version >= 2000 {
            return cuParamSetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetTexRef", 2) => {
        if version >= 2000 {
            return cuParamSetTexRef as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetf", 0) => {
        if version >= 2000 {
            return cuParamSetf as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetf", 1) => {
        if version >= 2000 {
            return cuParamSetf as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetf", 2) => {
        if version >= 2000 {
            return cuParamSetf as _;
        }
        usize::MAX as _
    }
    (b"cuParamSeti", 0) => {
        if version >= 2000 {
            return cuParamSeti as _;
        }
        usize::MAX as _
    }
    (b"cuParamSeti", 1) => {
        if version >= 2000 {
            return cuParamSeti as _;
        }
        usize::MAX as _
    }
    (b"cuParamSeti", 2) => {
        if version >= 2000 {
            return cuParamSeti as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetv", 0) => {
        if version >= 2000 {
            return cuParamSetv as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetv", 1) => {
        if version >= 2000 {
            return cuParamSetv as _;
        }
        usize::MAX as _
    }
    (b"cuParamSetv", 2) => {
        if version >= 2000 {
            return cuParamSetv as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttribute", 0) => {
        if version >= 4000 {
            return cuPointerGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttribute", 1) => {
        if version >= 4000 {
            return cuPointerGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttribute", 2) => {
        if version >= 4000 {
            return cuPointerGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttributes", 0) => {
        if version >= 7000 {
            return cuPointerGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttributes", 1) => {
        if version >= 7000 {
            return cuPointerGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuPointerGetAttributes", 2) => {
        if version >= 7000 {
            return cuPointerGetAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuPointerSetAttribute", 0) => {
        if version >= 6000 {
            return cuPointerSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuPointerSetAttribute", 1) => {
        if version >= 6000 {
            return cuPointerSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuPointerSetAttribute", 2) => {
        if version >= 6000 {
            return cuPointerSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerInitialize", 0) => {
        if version >= 4000 {
            return cuProfilerInitialize as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerInitialize", 1) => {
        if version >= 4000 {
            return cuProfilerInitialize as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerInitialize", 2) => {
        if version >= 4000 {
            return cuProfilerInitialize as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStart", 0) => {
        if version >= 4000 {
            return cuProfilerStart as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStart", 1) => {
        if version >= 4000 {
            return cuProfilerStart as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStart", 2) => {
        if version >= 4000 {
            return cuProfilerStart as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStop", 0) => {
        if version >= 4000 {
            return cuProfilerStop as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStop", 1) => {
        if version >= 4000 {
            return cuProfilerStop as _;
        }
        usize::MAX as _
    }
    (b"cuProfilerStop", 2) => {
        if version >= 4000 {
            return cuProfilerStop as _;
        }
        usize::MAX as _
    }
    (b"cuSignalExternalSemaphoresAsync", 0) => {
        if version >= 10000 {
            return cuSignalExternalSemaphoresAsync as _;
        }
        usize::MAX as _
    }
    (b"cuSignalExternalSemaphoresAsync", 1) => {
        if version >= 10000 {
            return cuSignalExternalSemaphoresAsync as _;
        }
        usize::MAX as _
    }
    (b"cuSignalExternalSemaphoresAsync", 2) => {
        if version >= 10000 {
            return cuSignalExternalSemaphoresAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAddCallback", 0) => {
        if version >= 5000 {
            return cuStreamAddCallback as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAddCallback", 1) => {
        if version >= 5000 {
            return cuStreamAddCallback as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAddCallback", 2) => {
        if version >= 7000 {
            return cuStreamAddCallback_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAttachMemAsync", 0) => {
        if version >= 6000 {
            return cuStreamAttachMemAsync as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAttachMemAsync", 1) => {
        if version >= 6000 {
            return cuStreamAttachMemAsync as _;
        }
        usize::MAX as _
    }
    (b"cuStreamAttachMemAsync", 2) => {
        if version >= 7000 {
            return cuStreamAttachMemAsync_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBatchMemOp", 0) => {
        if version >= 11070 {
            return cuStreamBatchMemOp_v2 as _;
        }
        if version >= 8000 {
            return cuStreamBatchMemOp as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBatchMemOp", 1) => {
        if version >= 11070 {
            return cuStreamBatchMemOp_v2 as _;
        }
        if version >= 8000 {
            return cuStreamBatchMemOp as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBatchMemOp", 2) => {
        if version >= 11070 {
            return cuStreamBatchMemOp_v2_ptsz as _;
        }
        if version >= 8000 {
            return cuStreamBatchMemOp_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCapture", 0) => {
        if version >= 10010 {
            return cuStreamBeginCapture_v2 as _;
        }
        if version >= 10000 {
            return cuStreamBeginCapture as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCapture", 1) => {
        if version >= 10010 {
            return cuStreamBeginCapture_v2 as _;
        }
        if version >= 10000 {
            return cuStreamBeginCapture as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCapture", 2) => {
        if version >= 10010 {
            return cuStreamBeginCapture_v2_ptsz as _;
        }
        if version >= 10000 {
            return cuStreamBeginCapture_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCaptureToGraph", 0) => {
        if version >= 12030 {
            return cuStreamBeginCaptureToGraph as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCaptureToGraph", 1) => {
        if version >= 12030 {
            return cuStreamBeginCaptureToGraph as _;
        }
        usize::MAX as _
    }
    (b"cuStreamBeginCaptureToGraph", 2) => {
        if version >= 12030 {
            return cuStreamBeginCaptureToGraph_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCopyAttributes", 0) => {
        if version >= 11000 {
            return cuStreamCopyAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCopyAttributes", 1) => {
        if version >= 11000 {
            return cuStreamCopyAttributes as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCopyAttributes", 2) => {
        if version >= 11000 {
            return cuStreamCopyAttributes_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreate", 0) => {
        if version >= 2000 {
            return cuStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreate", 1) => {
        if version >= 2000 {
            return cuStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreate", 2) => {
        if version >= 2000 {
            return cuStreamCreate as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreateWithPriority", 0) => {
        if version >= 5050 {
            return cuStreamCreateWithPriority as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreateWithPriority", 1) => {
        if version >= 5050 {
            return cuStreamCreateWithPriority as _;
        }
        usize::MAX as _
    }
    (b"cuStreamCreateWithPriority", 2) => {
        if version >= 5050 {
            return cuStreamCreateWithPriority as _;
        }
        usize::MAX as _
    }
    (b"cuStreamDestroy", 0) => {
        if version >= 4000 {
            return cuStreamDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuStreamDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuStreamDestroy", 1) => {
        if version >= 4000 {
            return cuStreamDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuStreamDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuStreamDestroy", 2) => {
        if version >= 4000 {
            return cuStreamDestroy_v2 as _;
        }
        if version >= 2000 {
            return cuStreamDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuStreamEndCapture", 0) => {
        if version >= 10000 {
            return cuStreamEndCapture as _;
        }
        usize::MAX as _
    }
    (b"cuStreamEndCapture", 1) => {
        if version >= 10000 {
            return cuStreamEndCapture as _;
        }
        usize::MAX as _
    }
    (b"cuStreamEndCapture", 2) => {
        if version >= 10000 {
            return cuStreamEndCapture_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetAttribute", 0) => {
        if version >= 11000 {
            return cuStreamGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetAttribute", 1) => {
        if version >= 11000 {
            return cuStreamGetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetAttribute", 2) => {
        if version >= 11000 {
            return cuStreamGetAttribute_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCaptureInfo", 0) => {
        if version >= 12030 {
            return cuStreamGetCaptureInfo_v3 as _;
        }
        if version >= 11030 {
            return cuStreamGetCaptureInfo_v2 as _;
        }
        if version >= 10010 {
            return cuStreamGetCaptureInfo as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCaptureInfo", 1) => {
        if version >= 12030 {
            return cuStreamGetCaptureInfo_v3 as _;
        }
        if version >= 11030 {
            return cuStreamGetCaptureInfo_v2 as _;
        }
        if version >= 10010 {
            return cuStreamGetCaptureInfo as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCaptureInfo", 2) => {
        if version >= 12030 {
            return cuStreamGetCaptureInfo_v3_ptsz as _;
        }
        if version >= 11030 {
            return cuStreamGetCaptureInfo_v2_ptsz as _;
        }
        if version >= 10010 {
            return cuStreamGetCaptureInfo_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCtx", 0) => {
        if version >= 12050 {
            return cuStreamGetCtx_v2 as _;
        }
        if version >= 9020 {
            return cuStreamGetCtx as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCtx", 1) => {
        if version >= 12050 {
            return cuStreamGetCtx_v2 as _;
        }
        if version >= 9020 {
            return cuStreamGetCtx as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetCtx", 2) => {
        if version >= 12050 {
            return cuStreamGetCtx_v2_ptsz as _;
        }
        if version >= 9020 {
            return cuStreamGetCtx_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetDevice", 0) => {
        if version >= 12080 {
            return cuStreamGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetDevice", 1) => {
        if version >= 12080 {
            return cuStreamGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetDevice", 2) => {
        if version >= 12080 {
            return cuStreamGetDevice_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetFlags", 0) => {
        if version >= 5050 {
            return cuStreamGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetFlags", 1) => {
        if version >= 5050 {
            return cuStreamGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetFlags", 2) => {
        if version >= 7000 {
            return cuStreamGetFlags_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetGreenCtx", 0) => {
        if version >= 12040 {
            return cuStreamGetGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetGreenCtx", 1) => {
        if version >= 12040 {
            return cuStreamGetGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetGreenCtx", 2) => {
        if version >= 12040 {
            return cuStreamGetGreenCtx as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetId", 0) => {
        if version >= 12000 {
            return cuStreamGetId as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetId", 1) => {
        if version >= 12000 {
            return cuStreamGetId as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetId", 2) => {
        if version >= 12000 {
            return cuStreamGetId_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetPriority", 0) => {
        if version >= 5050 {
            return cuStreamGetPriority as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetPriority", 1) => {
        if version >= 5050 {
            return cuStreamGetPriority as _;
        }
        usize::MAX as _
    }
    (b"cuStreamGetPriority", 2) => {
        if version >= 7000 {
            return cuStreamGetPriority_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamIsCapturing", 0) => {
        if version >= 10000 {
            return cuStreamIsCapturing as _;
        }
        usize::MAX as _
    }
    (b"cuStreamIsCapturing", 1) => {
        if version >= 10000 {
            return cuStreamIsCapturing as _;
        }
        usize::MAX as _
    }
    (b"cuStreamIsCapturing", 2) => {
        if version >= 10000 {
            return cuStreamIsCapturing_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamQuery", 0) => {
        if version >= 2000 {
            return cuStreamQuery as _;
        }
        usize::MAX as _
    }
    (b"cuStreamQuery", 1) => {
        if version >= 2000 {
            return cuStreamQuery as _;
        }
        usize::MAX as _
    }
    (b"cuStreamQuery", 2) => {
        if version >= 7000 {
            return cuStreamQuery_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSetAttribute", 0) => {
        if version >= 11000 {
            return cuStreamSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSetAttribute", 1) => {
        if version >= 11000 {
            return cuStreamSetAttribute as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSetAttribute", 2) => {
        if version >= 11000 {
            return cuStreamSetAttribute_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSynchronize", 0) => {
        if version >= 2000 {
            return cuStreamSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSynchronize", 1) => {
        if version >= 2000 {
            return cuStreamSynchronize as _;
        }
        usize::MAX as _
    }
    (b"cuStreamSynchronize", 2) => {
        if version >= 7000 {
            return cuStreamSynchronize_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamUpdateCaptureDependencies", 0) => {
        if version >= 12030 {
            return cuStreamUpdateCaptureDependencies_v2 as _;
        }
        if version >= 11030 {
            return cuStreamUpdateCaptureDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuStreamUpdateCaptureDependencies", 1) => {
        if version >= 12030 {
            return cuStreamUpdateCaptureDependencies_v2 as _;
        }
        if version >= 11030 {
            return cuStreamUpdateCaptureDependencies as _;
        }
        usize::MAX as _
    }
    (b"cuStreamUpdateCaptureDependencies", 2) => {
        if version >= 12030 {
            return cuStreamUpdateCaptureDependencies_v2_ptsz as _;
        }
        if version >= 11030 {
            return cuStreamUpdateCaptureDependencies_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitEvent", 0) => {
        if version >= 3020 {
            return cuStreamWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitEvent", 1) => {
        if version >= 3020 {
            return cuStreamWaitEvent as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitEvent", 2) => {
        if version >= 7000 {
            return cuStreamWaitEvent_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue32", 0) => {
        if version >= 11070 {
            return cuStreamWaitValue32_v2 as _;
        }
        if version >= 8000 {
            return cuStreamWaitValue32 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue32", 1) => {
        if version >= 11070 {
            return cuStreamWaitValue32_v2 as _;
        }
        if version >= 8000 {
            return cuStreamWaitValue32 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue32", 2) => {
        if version >= 11070 {
            return cuStreamWaitValue32_v2_ptsz as _;
        }
        if version >= 8000 {
            return cuStreamWaitValue32_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue64", 0) => {
        if version >= 11070 {
            return cuStreamWaitValue64_v2 as _;
        }
        if version >= 9000 {
            return cuStreamWaitValue64 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue64", 1) => {
        if version >= 11070 {
            return cuStreamWaitValue64_v2 as _;
        }
        if version >= 9000 {
            return cuStreamWaitValue64 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWaitValue64", 2) => {
        if version >= 11070 {
            return cuStreamWaitValue64_v2_ptsz as _;
        }
        if version >= 9000 {
            return cuStreamWaitValue64_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue32", 0) => {
        if version >= 11070 {
            return cuStreamWriteValue32_v2 as _;
        }
        if version >= 8000 {
            return cuStreamWriteValue32 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue32", 1) => {
        if version >= 11070 {
            return cuStreamWriteValue32_v2 as _;
        }
        if version >= 8000 {
            return cuStreamWriteValue32 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue32", 2) => {
        if version >= 11070 {
            return cuStreamWriteValue32_v2_ptsz as _;
        }
        if version >= 8000 {
            return cuStreamWriteValue32_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue64", 0) => {
        if version >= 11070 {
            return cuStreamWriteValue64_v2 as _;
        }
        if version >= 9000 {
            return cuStreamWriteValue64 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue64", 1) => {
        if version >= 11070 {
            return cuStreamWriteValue64_v2 as _;
        }
        if version >= 9000 {
            return cuStreamWriteValue64 as _;
        }
        usize::MAX as _
    }
    (b"cuStreamWriteValue64", 2) => {
        if version >= 11070 {
            return cuStreamWriteValue64_v2_ptsz as _;
        }
        if version >= 9000 {
            return cuStreamWriteValue64_ptsz as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectCreate", 0) => {
        if version >= 5000 {
            return cuSurfObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectCreate", 1) => {
        if version >= 5000 {
            return cuSurfObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectCreate", 2) => {
        if version >= 5000 {
            return cuSurfObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectDestroy", 0) => {
        if version >= 5000 {
            return cuSurfObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectDestroy", 1) => {
        if version >= 5000 {
            return cuSurfObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectDestroy", 2) => {
        if version >= 5000 {
            return cuSurfObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectGetResourceDesc", 0) => {
        if version >= 5000 {
            return cuSurfObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectGetResourceDesc", 1) => {
        if version >= 5000 {
            return cuSurfObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuSurfObjectGetResourceDesc", 2) => {
        if version >= 5000 {
            return cuSurfObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefGetArray", 0) => {
        if version >= 3000 {
            return cuSurfRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefGetArray", 1) => {
        if version >= 3000 {
            return cuSurfRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefGetArray", 2) => {
        if version >= 3000 {
            return cuSurfRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefSetArray", 0) => {
        if version >= 3000 {
            return cuSurfRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefSetArray", 1) => {
        if version >= 3000 {
            return cuSurfRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuSurfRefSetArray", 2) => {
        if version >= 3000 {
            return cuSurfRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2col", 0) => {
        if version >= 12000 {
            return cuTensorMapEncodeIm2col as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2col", 1) => {
        if version >= 12000 {
            return cuTensorMapEncodeIm2col as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2col", 2) => {
        if version >= 12000 {
            return cuTensorMapEncodeIm2col as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2colWide", 0) => {
        if version >= 12080 {
            return cuTensorMapEncodeIm2colWide as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2colWide", 1) => {
        if version >= 12080 {
            return cuTensorMapEncodeIm2colWide as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeIm2colWide", 2) => {
        if version >= 12080 {
            return cuTensorMapEncodeIm2colWide as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeTiled", 0) => {
        if version >= 12000 {
            return cuTensorMapEncodeTiled as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeTiled", 1) => {
        if version >= 12000 {
            return cuTensorMapEncodeTiled as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapEncodeTiled", 2) => {
        if version >= 12000 {
            return cuTensorMapEncodeTiled as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapReplaceAddress", 0) => {
        if version >= 12000 {
            return cuTensorMapReplaceAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapReplaceAddress", 1) => {
        if version >= 12000 {
            return cuTensorMapReplaceAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTensorMapReplaceAddress", 2) => {
        if version >= 12000 {
            return cuTensorMapReplaceAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectCreate", 0) => {
        if version >= 5000 {
            return cuTexObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectCreate", 1) => {
        if version >= 5000 {
            return cuTexObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectCreate", 2) => {
        if version >= 5000 {
            return cuTexObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectDestroy", 0) => {
        if version >= 5000 {
            return cuTexObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectDestroy", 1) => {
        if version >= 5000 {
            return cuTexObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectDestroy", 2) => {
        if version >= 5000 {
            return cuTexObjectDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceDesc", 0) => {
        if version >= 5000 {
            return cuTexObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceDesc", 1) => {
        if version >= 5000 {
            return cuTexObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceDesc", 2) => {
        if version >= 5000 {
            return cuTexObjectGetResourceDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceViewDesc", 0) => {
        if version >= 5000 {
            return cuTexObjectGetResourceViewDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceViewDesc", 1) => {
        if version >= 5000 {
            return cuTexObjectGetResourceViewDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetResourceViewDesc", 2) => {
        if version >= 5000 {
            return cuTexObjectGetResourceViewDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetTextureDesc", 0) => {
        if version >= 5000 {
            return cuTexObjectGetTextureDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetTextureDesc", 1) => {
        if version >= 5000 {
            return cuTexObjectGetTextureDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexObjectGetTextureDesc", 2) => {
        if version >= 5000 {
            return cuTexObjectGetTextureDesc as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefCreate", 0) => {
        if version >= 2000 {
            return cuTexRefCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefCreate", 1) => {
        if version >= 2000 {
            return cuTexRefCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefCreate", 2) => {
        if version >= 2000 {
            return cuTexRefCreate as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefDestroy", 0) => {
        if version >= 2000 {
            return cuTexRefDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefDestroy", 1) => {
        if version >= 2000 {
            return cuTexRefDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefDestroy", 2) => {
        if version >= 2000 {
            return cuTexRefDestroy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddress", 0) => {
        if version >= 3020 {
            return cuTexRefGetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefGetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddress", 1) => {
        if version >= 3020 {
            return cuTexRefGetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefGetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddress", 2) => {
        if version >= 3020 {
            return cuTexRefGetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefGetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddressMode", 0) => {
        if version >= 2000 {
            return cuTexRefGetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddressMode", 1) => {
        if version >= 2000 {
            return cuTexRefGetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetAddressMode", 2) => {
        if version >= 2000 {
            return cuTexRefGetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetArray", 0) => {
        if version >= 2000 {
            return cuTexRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetArray", 1) => {
        if version >= 2000 {
            return cuTexRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetArray", 2) => {
        if version >= 2000 {
            return cuTexRefGetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetBorderColor", 0) => {
        if version >= 8000 {
            return cuTexRefGetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetBorderColor", 1) => {
        if version >= 8000 {
            return cuTexRefGetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetBorderColor", 2) => {
        if version >= 8000 {
            return cuTexRefGetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFilterMode", 0) => {
        if version >= 2000 {
            return cuTexRefGetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFilterMode", 1) => {
        if version >= 2000 {
            return cuTexRefGetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFilterMode", 2) => {
        if version >= 2000 {
            return cuTexRefGetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFlags", 0) => {
        if version >= 2000 {
            return cuTexRefGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFlags", 1) => {
        if version >= 2000 {
            return cuTexRefGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFlags", 2) => {
        if version >= 2000 {
            return cuTexRefGetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFormat", 0) => {
        if version >= 2000 {
            return cuTexRefGetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFormat", 1) => {
        if version >= 2000 {
            return cuTexRefGetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetFormat", 2) => {
        if version >= 2000 {
            return cuTexRefGetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMaxAnisotropy", 0) => {
        if version >= 5000 {
            return cuTexRefGetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMaxAnisotropy", 1) => {
        if version >= 5000 {
            return cuTexRefGetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMaxAnisotropy", 2) => {
        if version >= 5000 {
            return cuTexRefGetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapFilterMode", 0) => {
        if version >= 5000 {
            return cuTexRefGetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapFilterMode", 1) => {
        if version >= 5000 {
            return cuTexRefGetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapFilterMode", 2) => {
        if version >= 5000 {
            return cuTexRefGetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelBias", 0) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelBias", 1) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelBias", 2) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelClamp", 0) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelClamp", 1) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmapLevelClamp", 2) => {
        if version >= 5000 {
            return cuTexRefGetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmappedArray", 0) => {
        if version >= 5000 {
            return cuTexRefGetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmappedArray", 1) => {
        if version >= 5000 {
            return cuTexRefGetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefGetMipmappedArray", 2) => {
        if version >= 5000 {
            return cuTexRefGetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress", 0) => {
        if version >= 3020 {
            return cuTexRefSetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefSetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress", 1) => {
        if version >= 3020 {
            return cuTexRefSetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefSetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress", 2) => {
        if version >= 3020 {
            return cuTexRefSetAddress_v2 as _;
        }
        if version >= 2000 {
            return cuTexRefSetAddress as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress2D", 0) => {
        if version >= 4010 {
            return cuTexRefSetAddress2D_v3 as _;
        }
        if version >= 3020 {
            return cuTexRefSetAddress2D_v2 as _;
        }
        if version >= 2020 {
            return cuTexRefSetAddress2D as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress2D", 1) => {
        if version >= 4010 {
            return cuTexRefSetAddress2D_v3 as _;
        }
        if version >= 3020 {
            return cuTexRefSetAddress2D_v2 as _;
        }
        if version >= 2020 {
            return cuTexRefSetAddress2D as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddress2D", 2) => {
        if version >= 4010 {
            return cuTexRefSetAddress2D_v3 as _;
        }
        if version >= 3020 {
            return cuTexRefSetAddress2D_v2 as _;
        }
        if version >= 2020 {
            return cuTexRefSetAddress2D as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddressMode", 0) => {
        if version >= 2000 {
            return cuTexRefSetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddressMode", 1) => {
        if version >= 2000 {
            return cuTexRefSetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetAddressMode", 2) => {
        if version >= 2000 {
            return cuTexRefSetAddressMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetArray", 0) => {
        if version >= 2000 {
            return cuTexRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetArray", 1) => {
        if version >= 2000 {
            return cuTexRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetArray", 2) => {
        if version >= 2000 {
            return cuTexRefSetArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetBorderColor", 0) => {
        if version >= 8000 {
            return cuTexRefSetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetBorderColor", 1) => {
        if version >= 8000 {
            return cuTexRefSetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetBorderColor", 2) => {
        if version >= 8000 {
            return cuTexRefSetBorderColor as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFilterMode", 0) => {
        if version >= 2000 {
            return cuTexRefSetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFilterMode", 1) => {
        if version >= 2000 {
            return cuTexRefSetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFilterMode", 2) => {
        if version >= 2000 {
            return cuTexRefSetFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFlags", 0) => {
        if version >= 2000 {
            return cuTexRefSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFlags", 1) => {
        if version >= 2000 {
            return cuTexRefSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFlags", 2) => {
        if version >= 2000 {
            return cuTexRefSetFlags as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFormat", 0) => {
        if version >= 2000 {
            return cuTexRefSetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFormat", 1) => {
        if version >= 2000 {
            return cuTexRefSetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetFormat", 2) => {
        if version >= 2000 {
            return cuTexRefSetFormat as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMaxAnisotropy", 0) => {
        if version >= 5000 {
            return cuTexRefSetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMaxAnisotropy", 1) => {
        if version >= 5000 {
            return cuTexRefSetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMaxAnisotropy", 2) => {
        if version >= 5000 {
            return cuTexRefSetMaxAnisotropy as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapFilterMode", 0) => {
        if version >= 5000 {
            return cuTexRefSetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapFilterMode", 1) => {
        if version >= 5000 {
            return cuTexRefSetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapFilterMode", 2) => {
        if version >= 5000 {
            return cuTexRefSetMipmapFilterMode as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelBias", 0) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelBias", 1) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelBias", 2) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelBias as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelClamp", 0) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelClamp", 1) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmapLevelClamp", 2) => {
        if version >= 5000 {
            return cuTexRefSetMipmapLevelClamp as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmappedArray", 0) => {
        if version >= 5000 {
            return cuTexRefSetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmappedArray", 1) => {
        if version >= 5000 {
            return cuTexRefSetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuTexRefSetMipmappedArray", 2) => {
        if version >= 5000 {
            return cuTexRefSetMipmappedArray as _;
        }
        usize::MAX as _
    }
    (b"cuThreadExchangeStreamCaptureMode", 0) => {
        if version >= 10010 {
            return cuThreadExchangeStreamCaptureMode as _;
        }
        usize::MAX as _
    }
    (b"cuThreadExchangeStreamCaptureMode", 1) => {
        if version >= 10010 {
            return cuThreadExchangeStreamCaptureMode as _;
        }
        usize::MAX as _
    }
    (b"cuThreadExchangeStreamCaptureMode", 2) => {
        if version >= 10010 {
            return cuThreadExchangeStreamCaptureMode as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectCreate", 0) => {
        if version >= 11030 {
            return cuUserObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectCreate", 1) => {
        if version >= 11030 {
            return cuUserObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectCreate", 2) => {
        if version >= 11030 {
            return cuUserObjectCreate as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRelease", 0) => {
        if version >= 11030 {
            return cuUserObjectRelease as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRelease", 1) => {
        if version >= 11030 {
            return cuUserObjectRelease as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRelease", 2) => {
        if version >= 11030 {
            return cuUserObjectRelease as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRetain", 0) => {
        if version >= 11030 {
            return cuUserObjectRetain as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRetain", 1) => {
        if version >= 11030 {
            return cuUserObjectRetain as _;
        }
        usize::MAX as _
    }
    (b"cuUserObjectRetain", 2) => {
        if version >= 11030 {
            return cuUserObjectRetain as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUCtxCreate", 0) => {
        if version >= 3020 {
            return cuVDPAUCtxCreate_v2 as _;
        }
        if version >= 3010 {
            return cuVDPAUCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUCtxCreate", 1) => {
        if version >= 3020 {
            return cuVDPAUCtxCreate_v2 as _;
        }
        if version >= 3010 {
            return cuVDPAUCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUCtxCreate", 2) => {
        if version >= 3020 {
            return cuVDPAUCtxCreate_v2 as _;
        }
        if version >= 3010 {
            return cuVDPAUCtxCreate as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUGetDevice", 0) => {
        if version >= 3010 {
            return cuVDPAUGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUGetDevice", 1) => {
        if version >= 3010 {
            return cuVDPAUGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuVDPAUGetDevice", 2) => {
        if version >= 3010 {
            return cuVDPAUGetDevice as _;
        }
        usize::MAX as _
    }
    (b"cuWaitExternalSemaphoresAsync", 0) => {
        if version >= 10000 {
            return cuWaitExternalSemaphoresAsync as _;
        }
        usize::MAX as _
    }
    (b"cuWaitExternalSemaphoresAsync", 1) => {
        if version >= 10000 {
            return cuWaitExternalSemaphoresAsync as _;
        }
        usize::MAX as _
    }
    (b"cuWaitExternalSemaphoresAsync", 2) => {
        if version >= 10000 {
            return cuWaitExternalSemaphoresAsync_ptsz as _;
        }
        usize::MAX as _
    }
    _ => 0usize as _
}
