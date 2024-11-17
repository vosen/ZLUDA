#![allow(warnings)]
pub const CUDA_VERSION: u32 = 12040;
pub const CU_IPC_HANDLE_SIZE: u32 = 64;
pub const CU_COMPUTE_ACCELERATED_TARGET_BASE: u32 = 65536;
pub const CU_GRAPH_COND_ASSIGN_DEFAULT: u32 = 1;
pub const CU_GRAPH_KERNEL_NODE_PORT_DEFAULT: u32 = 0;
pub const CU_GRAPH_KERNEL_NODE_PORT_PROGRAMMATIC: u32 = 1;
pub const CU_GRAPH_KERNEL_NODE_PORT_LAUNCH_ORDER: u32 = 2;
pub const CU_MEMHOSTALLOC_PORTABLE: u32 = 1;
pub const CU_MEMHOSTALLOC_DEVICEMAP: u32 = 2;
pub const CU_MEMHOSTALLOC_WRITECOMBINED: u32 = 4;
pub const CU_MEMHOSTREGISTER_PORTABLE: u32 = 1;
pub const CU_MEMHOSTREGISTER_DEVICEMAP: u32 = 2;
pub const CU_MEMHOSTREGISTER_IOMEMORY: u32 = 4;
pub const CU_MEMHOSTREGISTER_READ_ONLY: u32 = 8;
pub const CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL: u32 = 1;
pub const CU_TENSOR_MAP_NUM_QWORDS: u32 = 16;
pub const CUDA_EXTERNAL_MEMORY_DEDICATED: u32 = 1;
pub const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_SKIP_NVSCIBUF_MEMSYNC: u32 = 1;
pub const CUDA_EXTERNAL_SEMAPHORE_WAIT_SKIP_NVSCIBUF_MEMSYNC: u32 = 2;
pub const CUDA_NVSCISYNC_ATTR_SIGNAL: u32 = 1;
pub const CUDA_NVSCISYNC_ATTR_WAIT: u32 = 2;
pub const CU_MEM_CREATE_USAGE_TILE_POOL: u32 = 1;
pub const CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_PRE_LAUNCH_SYNC: u32 = 1;
pub const CUDA_COOPERATIVE_LAUNCH_MULTI_DEVICE_NO_POST_LAUNCH_SYNC: u32 = 2;
pub const CUDA_ARRAY3D_LAYERED: u32 = 1;
pub const CUDA_ARRAY3D_2DARRAY: u32 = 1;
pub const CUDA_ARRAY3D_SURFACE_LDST: u32 = 2;
pub const CUDA_ARRAY3D_CUBEMAP: u32 = 4;
pub const CUDA_ARRAY3D_TEXTURE_GATHER: u32 = 8;
pub const CUDA_ARRAY3D_DEPTH_TEXTURE: u32 = 16;
pub const CUDA_ARRAY3D_COLOR_ATTACHMENT: u32 = 32;
pub const CUDA_ARRAY3D_SPARSE: u32 = 64;
pub const CUDA_ARRAY3D_DEFERRED_MAPPING: u32 = 128;
pub const CU_TRSA_OVERRIDE_FORMAT: u32 = 1;
pub const CU_TRSF_READ_AS_INTEGER: u32 = 1;
pub const CU_TRSF_NORMALIZED_COORDINATES: u32 = 2;
pub const CU_TRSF_SRGB: u32 = 16;
pub const CU_TRSF_DISABLE_TRILINEAR_OPTIMIZATION: u32 = 32;
pub const CU_TRSF_SEAMLESS_CUBEMAP: u32 = 64;
pub const CU_LAUNCH_PARAM_END_AS_INT: u32 = 0;
pub const CU_LAUNCH_PARAM_BUFFER_POINTER_AS_INT: u32 = 1;
pub const CU_LAUNCH_PARAM_BUFFER_SIZE_AS_INT: u32 = 2;
pub const CU_PARAM_TR_DEFAULT: i32 = -1;
pub const CUDA_EGL_INFINITE_TIMEOUT: u32 = 4294967295;
pub type cuuint32_t = u32;
pub type cuuint64_t = u64;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUdeviceptr_v2(pub ::core::ffi::c_ulonglong);
pub type CUdeviceptr = CUdeviceptr_v2;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUdevice_v1(pub ::core::ffi::c_int);
pub type CUdevice = CUdevice_v1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUctx_st {
    _unused: [u8; 0],
}
pub type CUcontext = *mut CUctx_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmod_st {
    _unused: [u8; 0],
}
pub type CUmodule = *mut CUmod_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUfunc_st {
    _unused: [u8; 0],
}
pub type CUfunction = *mut CUfunc_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlib_st {
    _unused: [u8; 0],
}
pub type CUlibrary = *mut CUlib_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUkern_st {
    _unused: [u8; 0],
}
pub type CUkernel = *mut CUkern_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUarray_st {
    _unused: [u8; 0],
}
pub type CUarray = *mut CUarray_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmipmappedArray_st {
    _unused: [u8; 0],
}
pub type CUmipmappedArray = *mut CUmipmappedArray_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUtexref_st {
    _unused: [u8; 0],
}
pub type CUtexref = *mut CUtexref_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUsurfref_st {
    _unused: [u8; 0],
}
pub type CUsurfref = *mut CUsurfref_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUevent_st {
    _unused: [u8; 0],
}
pub type CUevent = *mut CUevent_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUstream_st {
    _unused: [u8; 0],
}
pub type CUstream = *mut CUstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphicsResource_st {
    _unused: [u8; 0],
}
pub type CUgraphicsResource = *mut CUgraphicsResource_st;
pub type CUtexObject_v1 = ::core::ffi::c_ulonglong;
pub type CUtexObject = CUtexObject_v1;
pub type CUsurfObject_v1 = ::core::ffi::c_ulonglong;
pub type CUsurfObject = CUsurfObject_v1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUextMemory_st {
    _unused: [u8; 0],
}
pub type CUexternalMemory = *mut CUextMemory_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUextSemaphore_st {
    _unused: [u8; 0],
}
pub type CUexternalSemaphore = *mut CUextSemaphore_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraph_st {
    _unused: [u8; 0],
}
pub type CUgraph = *mut CUgraph_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphNode_st {
    _unused: [u8; 0],
}
pub type CUgraphNode = *mut CUgraphNode_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphExec_st {
    _unused: [u8; 0],
}
pub type CUgraphExec = *mut CUgraphExec_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUmemPoolHandle_st {
    _unused: [u8; 0],
}
pub type CUmemoryPool = *mut CUmemPoolHandle_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUuserObject_st {
    _unused: [u8; 0],
}
pub type CUuserObject = *mut CUuserObject_st;
pub type CUgraphConditionalHandle = cuuint64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgraphDeviceUpdatableNode_st {
    _unused: [u8; 0],
}
pub type CUgraphDeviceNode = *mut CUgraphDeviceUpdatableNode_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUasyncCallbackEntry_st {
    _unused: [u8; 0],
}
pub type CUasyncCallbackHandle = *mut CUasyncCallbackEntry_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUuuid_st {
    pub bytes: [::core::ffi::c_char; 16usize],
}
pub type CUuuid = CUuuid_st;
/** Fabric handle - An opaque handle representing a memory allocation
 that can be exported to processes in same or different nodes. For IPC
 between processes on different nodes they must be connected via the
 NVSwitch fabric.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemFabricHandle_st {
    pub data: [::core::ffi::c_uchar; 64usize],
}
/** Fabric handle - An opaque handle representing a memory allocation
 that can be exported to processes in same or different nodes. For IPC
 between processes on different nodes they must be connected via the
 NVSwitch fabric.*/
pub type CUmemFabricHandle_v1 = CUmemFabricHandle_st;
/** Fabric handle - An opaque handle representing a memory allocation
 that can be exported to processes in same or different nodes. For IPC
 between processes on different nodes they must be connected via the
 NVSwitch fabric.*/
pub type CUmemFabricHandle = CUmemFabricHandle_v1;
/// CUDA IPC event handle
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUipcEventHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
/// CUDA IPC event handle
pub type CUipcEventHandle_v1 = CUipcEventHandle_st;
/// CUDA IPC event handle
pub type CUipcEventHandle = CUipcEventHandle_v1;
/// CUDA IPC mem handle
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUipcMemHandle_st {
    pub reserved: [::core::ffi::c_char; 64usize],
}
/// CUDA IPC mem handle
pub type CUipcMemHandle_v1 = CUipcMemHandle_st;
/// CUDA IPC mem handle
pub type CUipcMemHandle = CUipcMemHandle_v1;
impl CUipcMem_flags_enum {
    ///< Automatically enable peer access between remote devices as needed
    pub const CU_IPC_MEM_LAZY_ENABLE_PEER_ACCESS: CUipcMem_flags_enum = CUipcMem_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// CUDA Ipc Mem Flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUipcMem_flags_enum(pub ::core::ffi::c_uint);
/// CUDA Ipc Mem Flags
pub use self::CUipcMem_flags_enum as CUipcMem_flags;
impl CUmemAttach_flags_enum {
    ///< Memory can be accessed by any stream on any device
    pub const CU_MEM_ATTACH_GLOBAL: CUmemAttach_flags_enum = CUmemAttach_flags_enum(1);
}
impl CUmemAttach_flags_enum {
    ///< Memory cannot be accessed by any stream on any device
    pub const CU_MEM_ATTACH_HOST: CUmemAttach_flags_enum = CUmemAttach_flags_enum(2);
}
impl CUmemAttach_flags_enum {
    ///< Memory can only be accessed by a single stream on the associated device
    pub const CU_MEM_ATTACH_SINGLE: CUmemAttach_flags_enum = CUmemAttach_flags_enum(4);
}
#[repr(transparent)]
/// CUDA Mem Attach Flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAttach_flags_enum(pub ::core::ffi::c_uint);
/// CUDA Mem Attach Flags
pub use self::CUmemAttach_flags_enum as CUmemAttach_flags;
impl CUctx_flags_enum {
    ///< Automatic scheduling
    pub const CU_CTX_SCHED_AUTO: CUctx_flags_enum = CUctx_flags_enum(0);
}
impl CUctx_flags_enum {
    ///< Set spin as default scheduling
    pub const CU_CTX_SCHED_SPIN: CUctx_flags_enum = CUctx_flags_enum(1);
}
impl CUctx_flags_enum {
    ///< Set yield as default scheduling
    pub const CU_CTX_SCHED_YIELD: CUctx_flags_enum = CUctx_flags_enum(2);
}
impl CUctx_flags_enum {
    ///< Set blocking synchronization as default scheduling
    pub const CU_CTX_SCHED_BLOCKING_SYNC: CUctx_flags_enum = CUctx_flags_enum(4);
}
impl CUctx_flags_enum {
    /**< Set blocking synchronization as default scheduling
  \deprecated This flag was deprecated as of CUDA 4.0
  and was replaced with ::CU_CTX_SCHED_BLOCKING_SYNC.*/
    pub const CU_CTX_BLOCKING_SYNC: CUctx_flags_enum = CUctx_flags_enum(4);
}
impl CUctx_flags_enum {
    pub const CU_CTX_SCHED_MASK: CUctx_flags_enum = CUctx_flags_enum(7);
}
impl CUctx_flags_enum {
    /**< \deprecated This flag was deprecated as of CUDA 11.0
  and it no longer has any effect. All contexts
  as of CUDA 3.2 behave as though the flag is enabled.*/
    pub const CU_CTX_MAP_HOST: CUctx_flags_enum = CUctx_flags_enum(8);
}
impl CUctx_flags_enum {
    ///< Keep local memory allocation after launch
    pub const CU_CTX_LMEM_RESIZE_TO_MAX: CUctx_flags_enum = CUctx_flags_enum(16);
}
impl CUctx_flags_enum {
    ///< Trigger coredumps from exceptions in this context
    pub const CU_CTX_COREDUMP_ENABLE: CUctx_flags_enum = CUctx_flags_enum(32);
}
impl CUctx_flags_enum {
    ///< Enable user pipe to trigger coredumps in this context
    pub const CU_CTX_USER_COREDUMP_ENABLE: CUctx_flags_enum = CUctx_flags_enum(64);
}
impl CUctx_flags_enum {
    ///< Ensure synchronous memory operations on this context will synchronize
    pub const CU_CTX_SYNC_MEMOPS: CUctx_flags_enum = CUctx_flags_enum(128);
}
impl CUctx_flags_enum {
    pub const CU_CTX_FLAGS_MASK: CUctx_flags_enum = CUctx_flags_enum(255);
}
#[repr(transparent)]
/// Context creation flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUctx_flags_enum(pub ::core::ffi::c_uint);
/// Context creation flags
pub use self::CUctx_flags_enum as CUctx_flags;
impl CUevent_sched_flags_enum {
    ///< Automatic scheduling
    pub const CU_EVENT_SCHED_AUTO: CUevent_sched_flags_enum = CUevent_sched_flags_enum(
        0,
    );
}
impl CUevent_sched_flags_enum {
    ///< Set spin as default scheduling
    pub const CU_EVENT_SCHED_SPIN: CUevent_sched_flags_enum = CUevent_sched_flags_enum(
        1,
    );
}
impl CUevent_sched_flags_enum {
    ///< Set yield as default scheduling
    pub const CU_EVENT_SCHED_YIELD: CUevent_sched_flags_enum = CUevent_sched_flags_enum(
        2,
    );
}
impl CUevent_sched_flags_enum {
    ///< Set blocking synchronization as default scheduling
    pub const CU_EVENT_SCHED_BLOCKING_SYNC: CUevent_sched_flags_enum = CUevent_sched_flags_enum(
        4,
    );
}
#[repr(transparent)]
/// Event sched flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUevent_sched_flags_enum(pub ::core::ffi::c_uint);
/// Event sched flags
pub use self::CUevent_sched_flags_enum as CUevent_sched_flags;
impl CUstream_flags_enum {
    ///< Default stream flag
    pub const CU_STREAM_DEFAULT: CUstream_flags_enum = CUstream_flags_enum(0);
}
impl CUstream_flags_enum {
    ///< Stream does not synchronize with stream 0 (the NULL stream)
    pub const CU_STREAM_NON_BLOCKING: CUstream_flags_enum = CUstream_flags_enum(1);
}
#[repr(transparent)]
/// Stream creation flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstream_flags_enum(pub ::core::ffi::c_uint);
/// Stream creation flags
pub use self::CUstream_flags_enum as CUstream_flags;
impl CUevent_flags_enum {
    ///< Default event flag
    pub const CU_EVENT_DEFAULT: CUevent_flags_enum = CUevent_flags_enum(0);
}
impl CUevent_flags_enum {
    ///< Event uses blocking synchronization
    pub const CU_EVENT_BLOCKING_SYNC: CUevent_flags_enum = CUevent_flags_enum(1);
}
impl CUevent_flags_enum {
    ///< Event will not record timing data
    pub const CU_EVENT_DISABLE_TIMING: CUevent_flags_enum = CUevent_flags_enum(2);
}
impl CUevent_flags_enum {
    ///< Event is suitable for interprocess use. CU_EVENT_DISABLE_TIMING must be set
    pub const CU_EVENT_INTERPROCESS: CUevent_flags_enum = CUevent_flags_enum(4);
}
#[repr(transparent)]
/// Event creation flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUevent_flags_enum(pub ::core::ffi::c_uint);
/// Event creation flags
pub use self::CUevent_flags_enum as CUevent_flags;
impl CUevent_record_flags_enum {
    ///< Default event record flag
    pub const CU_EVENT_RECORD_DEFAULT: CUevent_record_flags_enum = CUevent_record_flags_enum(
        0,
    );
}
impl CUevent_record_flags_enum {
    /**< When using stream capture, create an event record node
  instead of the default behavior.  This flag is invalid
  when used outside of capture.*/
    pub const CU_EVENT_RECORD_EXTERNAL: CUevent_record_flags_enum = CUevent_record_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Event record flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUevent_record_flags_enum(pub ::core::ffi::c_uint);
/// Event record flags
pub use self::CUevent_record_flags_enum as CUevent_record_flags;
impl CUevent_wait_flags_enum {
    ///< Default event wait flag
    pub const CU_EVENT_WAIT_DEFAULT: CUevent_wait_flags_enum = CUevent_wait_flags_enum(
        0,
    );
}
impl CUevent_wait_flags_enum {
    /**< When using stream capture, create an event wait node
  instead of the default behavior.  This flag is invalid
  when used outside of capture.*/
    pub const CU_EVENT_WAIT_EXTERNAL: CUevent_wait_flags_enum = CUevent_wait_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Event wait flags
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUevent_wait_flags_enum(pub ::core::ffi::c_uint);
/// Event wait flags
pub use self::CUevent_wait_flags_enum as CUevent_wait_flags;
impl CUstreamWaitValue_flags_enum {
    /**< Wait until (int32_t)(*addr - value) >= 0 (or int64_t for 64 bit
values). Note this is a cyclic comparison which ignores wraparound.
(Default behavior.)*/
    pub const CU_STREAM_WAIT_VALUE_GEQ: CUstreamWaitValue_flags_enum = CUstreamWaitValue_flags_enum(
        0,
    );
}
impl CUstreamWaitValue_flags_enum {
    ///< Wait until *addr == value.
    pub const CU_STREAM_WAIT_VALUE_EQ: CUstreamWaitValue_flags_enum = CUstreamWaitValue_flags_enum(
        1,
    );
}
impl CUstreamWaitValue_flags_enum {
    ///< Wait until (*addr & value) != 0.
    pub const CU_STREAM_WAIT_VALUE_AND: CUstreamWaitValue_flags_enum = CUstreamWaitValue_flags_enum(
        2,
    );
}
impl CUstreamWaitValue_flags_enum {
    /**< Wait until ~(*addr | value) != 0. Support for this operation can be
queried with ::cuDeviceGetAttribute() and
::CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR.*/
    pub const CU_STREAM_WAIT_VALUE_NOR: CUstreamWaitValue_flags_enum = CUstreamWaitValue_flags_enum(
        3,
    );
}
impl CUstreamWaitValue_flags_enum {
    /**< Follow the wait operation with a flush of outstanding remote writes. This
means that, if a remote write operation is guaranteed to have reached the
device before the wait can be satisfied, that write is guaranteed to be
visible to downstream device work. The device is permitted to reorder
remote writes internally. For example, this flag would be required if
two remote writes arrive in a defined order, the wait is satisfied by the
second write, and downstream work needs to observe the first write.
Support for this operation is restricted to selected platforms and can be
queried with ::CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES.*/
    pub const CU_STREAM_WAIT_VALUE_FLUSH: CUstreamWaitValue_flags_enum = CUstreamWaitValue_flags_enum(
        1073741824,
    );
}
#[repr(transparent)]
/// Flags for ::cuStreamWaitValue32 and ::cuStreamWaitValue64
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamWaitValue_flags_enum(pub ::core::ffi::c_uint);
/// Flags for ::cuStreamWaitValue32 and ::cuStreamWaitValue64
pub use self::CUstreamWaitValue_flags_enum as CUstreamWaitValue_flags;
impl CUstreamWriteValue_flags_enum {
    ///< Default behavior
    pub const CU_STREAM_WRITE_VALUE_DEFAULT: CUstreamWriteValue_flags_enum = CUstreamWriteValue_flags_enum(
        0,
    );
}
impl CUstreamWriteValue_flags_enum {
    /**< Permits the write to be reordered with writes which were issued
before it, as a performance optimization. Normally,
::cuStreamWriteValue32 will provide a memory fence before the
write, which has similar semantics to
__threadfence_system() but is scoped to the stream
rather than a CUDA thread.
This flag is not supported in the v2 API.*/
    pub const CU_STREAM_WRITE_VALUE_NO_MEMORY_BARRIER: CUstreamWriteValue_flags_enum = CUstreamWriteValue_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for ::cuStreamWriteValue32
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamWriteValue_flags_enum(pub ::core::ffi::c_uint);
/// Flags for ::cuStreamWriteValue32
pub use self::CUstreamWriteValue_flags_enum as CUstreamWriteValue_flags;
impl CUstreamBatchMemOpType_enum {
    ///< Represents a ::cuStreamWaitValue32 operation
    pub const CU_STREAM_MEM_OP_WAIT_VALUE_32: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        1,
    );
}
impl CUstreamBatchMemOpType_enum {
    ///< Represents a ::cuStreamWriteValue32 operation
    pub const CU_STREAM_MEM_OP_WRITE_VALUE_32: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        2,
    );
}
impl CUstreamBatchMemOpType_enum {
    ///< Represents a ::cuStreamWaitValue64 operation
    pub const CU_STREAM_MEM_OP_WAIT_VALUE_64: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        4,
    );
}
impl CUstreamBatchMemOpType_enum {
    ///< Represents a ::cuStreamWriteValue64 operation
    pub const CU_STREAM_MEM_OP_WRITE_VALUE_64: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        5,
    );
}
impl CUstreamBatchMemOpType_enum {
    ///< Insert a memory barrier of the specified type
    pub const CU_STREAM_MEM_OP_BARRIER: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        6,
    );
}
impl CUstreamBatchMemOpType_enum {
    /**< This has the same effect as ::CU_STREAM_WAIT_VALUE_FLUSH, but as a
standalone operation.*/
    pub const CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES: CUstreamBatchMemOpType_enum = CUstreamBatchMemOpType_enum(
        3,
    );
}
#[repr(transparent)]
/// Operations for ::cuStreamBatchMemOp
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamBatchMemOpType_enum(pub ::core::ffi::c_uint);
/// Operations for ::cuStreamBatchMemOp
pub use self::CUstreamBatchMemOpType_enum as CUstreamBatchMemOpType;
impl CUstreamMemoryBarrier_flags_enum {
    ///< System-wide memory barrier.
    pub const CU_STREAM_MEMORY_BARRIER_TYPE_SYS: CUstreamMemoryBarrier_flags_enum = CUstreamMemoryBarrier_flags_enum(
        0,
    );
}
impl CUstreamMemoryBarrier_flags_enum {
    ///< Limit memory barrier scope to the GPU.
    pub const CU_STREAM_MEMORY_BARRIER_TYPE_GPU: CUstreamMemoryBarrier_flags_enum = CUstreamMemoryBarrier_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for ::cuStreamMemoryBarrier
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamMemoryBarrier_flags_enum(pub ::core::ffi::c_uint);
/// Flags for ::cuStreamMemoryBarrier
pub use self::CUstreamMemoryBarrier_flags_enum as CUstreamMemoryBarrier_flags;
/// Per-operation parameters for ::cuStreamBatchMemOp
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union {
    pub operation: CUstreamBatchMemOpType,
    pub waitValue: CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st,
    pub writeValue: CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st,
    pub flushRemoteWrites: CUstreamBatchMemOpParams_union_CUstreamMemOpFlushRemoteWritesParams_st,
    pub memoryBarrier: CUstreamBatchMemOpParams_union_CUstreamMemOpMemoryBarrierParams_st,
    pub pad: [cuuint64_t; 6usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub address: CUdeviceptr,
    pub __bindgen_anon_1: CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    ///< For driver internal use. Initial value is unimportant.
    pub alias: CUdeviceptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union_CUstreamMemOpWaitValueParams_st__bindgen_ty_1 {
    pub value: cuuint32_t,
    pub value64: cuuint64_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub address: CUdeviceptr,
    pub __bindgen_anon_1: CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st__bindgen_ty_1,
    pub flags: ::core::ffi::c_uint,
    ///< For driver internal use. Initial value is unimportant.
    pub alias: CUdeviceptr,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUstreamBatchMemOpParams_union_CUstreamMemOpWriteValueParams_st__bindgen_ty_1 {
    pub value: cuuint32_t,
    pub value64: cuuint64_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpFlushRemoteWritesParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUstreamBatchMemOpParams_union_CUstreamMemOpMemoryBarrierParams_st {
    pub operation: CUstreamBatchMemOpType,
    pub flags: ::core::ffi::c_uint,
}
/// Per-operation parameters for ::cuStreamBatchMemOp
pub type CUstreamBatchMemOpParams_v1 = CUstreamBatchMemOpParams_union;
/// Per-operation parameters for ::cuStreamBatchMemOp
pub type CUstreamBatchMemOpParams = CUstreamBatchMemOpParams_v1;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_BATCH_MEM_OP_NODE_PARAMS_v1_st {
    pub ctx: CUcontext,
    pub count: ::core::ffi::c_uint,
    pub paramArray: *mut CUstreamBatchMemOpParams,
    pub flags: ::core::ffi::c_uint,
}
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS_v1 = CUDA_BATCH_MEM_OP_NODE_PARAMS_v1_st;
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS = CUDA_BATCH_MEM_OP_NODE_PARAMS_v1;
/// Batch memory operation node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_BATCH_MEM_OP_NODE_PARAMS_v2_st {
    ///< Context to use for the operations.
    pub ctx: CUcontext,
    ///< Number of operations in paramArray.
    pub count: ::core::ffi::c_uint,
    ///< Array of batch memory operations.
    pub paramArray: *mut CUstreamBatchMemOpParams,
    ///< Flags to control the node.
    pub flags: ::core::ffi::c_uint,
}
/// Batch memory operation node parameters
pub type CUDA_BATCH_MEM_OP_NODE_PARAMS_v2 = CUDA_BATCH_MEM_OP_NODE_PARAMS_v2_st;
impl CUoccupancy_flags_enum {
    ///< Default behavior
    pub const CU_OCCUPANCY_DEFAULT: CUoccupancy_flags_enum = CUoccupancy_flags_enum(0);
}
impl CUoccupancy_flags_enum {
    ///< Assume global caching is enabled and cannot be automatically turned off
    pub const CU_OCCUPANCY_DISABLE_CACHING_OVERRIDE: CUoccupancy_flags_enum = CUoccupancy_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Occupancy calculator flag
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUoccupancy_flags_enum(pub ::core::ffi::c_uint);
/// Occupancy calculator flag
pub use self::CUoccupancy_flags_enum as CUoccupancy_flags;
impl CUstreamUpdateCaptureDependencies_flags_enum {
    ///< Add new nodes to the dependency set
    pub const CU_STREAM_ADD_CAPTURE_DEPENDENCIES: CUstreamUpdateCaptureDependencies_flags_enum = CUstreamUpdateCaptureDependencies_flags_enum(
        0,
    );
}
impl CUstreamUpdateCaptureDependencies_flags_enum {
    ///< Replace the dependency set with the new nodes
    pub const CU_STREAM_SET_CAPTURE_DEPENDENCIES: CUstreamUpdateCaptureDependencies_flags_enum = CUstreamUpdateCaptureDependencies_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for ::cuStreamUpdateCaptureDependencies
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamUpdateCaptureDependencies_flags_enum(pub ::core::ffi::c_uint);
/// Flags for ::cuStreamUpdateCaptureDependencies
pub use self::CUstreamUpdateCaptureDependencies_flags_enum as CUstreamUpdateCaptureDependencies_flags;
impl CUasyncNotificationType_enum {
    pub const CU_ASYNC_NOTIFICATION_TYPE_OVER_BUDGET: CUasyncNotificationType_enum = CUasyncNotificationType_enum(
        1,
    );
}
#[repr(transparent)]
/// Types of async notification that can be sent
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUasyncNotificationType_enum(pub ::core::ffi::c_uint);
/// Types of async notification that can be sent
pub use self::CUasyncNotificationType_enum as CUasyncNotificationType;
/// Information passed to the user via the async notification callback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUasyncNotificationInfo_st {
    pub type_: CUasyncNotificationType,
    pub info: CUasyncNotificationInfo_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUasyncNotificationInfo_st__bindgen_ty_1 {
    pub overBudget: CUasyncNotificationInfo_st__bindgen_ty_1__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUasyncNotificationInfo_st__bindgen_ty_1__bindgen_ty_1 {
    pub bytesOverBudget: ::core::ffi::c_ulonglong,
}
/// Information passed to the user via the async notification callback
pub type CUasyncNotificationInfo = CUasyncNotificationInfo_st;
/** CUDA async notification callback
 \param info Information describing what actions to take as a result of this trim notification.
 \param userData Pointer to user defined data provided at registration.
 \param callback The callback handle associated with this specific callback.*/
pub type CUasyncCallback = ::core::option::Option<
    unsafe extern "system" fn(
        info: *mut CUasyncNotificationInfo,
        userData: *mut ::core::ffi::c_void,
        callback: CUasyncCallbackHandle,
    ),
>;
impl CUarray_format_enum {
    ///< Unsigned 8-bit integers
    pub const CU_AD_FORMAT_UNSIGNED_INT8: CUarray_format_enum = CUarray_format_enum(1);
}
impl CUarray_format_enum {
    ///< Unsigned 16-bit integers
    pub const CU_AD_FORMAT_UNSIGNED_INT16: CUarray_format_enum = CUarray_format_enum(2);
}
impl CUarray_format_enum {
    ///< Unsigned 32-bit integers
    pub const CU_AD_FORMAT_UNSIGNED_INT32: CUarray_format_enum = CUarray_format_enum(3);
}
impl CUarray_format_enum {
    ///< Signed 8-bit integers
    pub const CU_AD_FORMAT_SIGNED_INT8: CUarray_format_enum = CUarray_format_enum(8);
}
impl CUarray_format_enum {
    ///< Signed 16-bit integers
    pub const CU_AD_FORMAT_SIGNED_INT16: CUarray_format_enum = CUarray_format_enum(9);
}
impl CUarray_format_enum {
    ///< Signed 32-bit integers
    pub const CU_AD_FORMAT_SIGNED_INT32: CUarray_format_enum = CUarray_format_enum(10);
}
impl CUarray_format_enum {
    ///< 16-bit floating point
    pub const CU_AD_FORMAT_HALF: CUarray_format_enum = CUarray_format_enum(16);
}
impl CUarray_format_enum {
    ///< 32-bit floating point
    pub const CU_AD_FORMAT_FLOAT: CUarray_format_enum = CUarray_format_enum(32);
}
impl CUarray_format_enum {
    ///< 8-bit YUV planar format, with 4:2:0 sampling
    pub const CU_AD_FORMAT_NV12: CUarray_format_enum = CUarray_format_enum(176);
}
impl CUarray_format_enum {
    ///< 1 channel unsigned 8-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT8X1: CUarray_format_enum = CUarray_format_enum(192);
}
impl CUarray_format_enum {
    ///< 2 channel unsigned 8-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT8X2: CUarray_format_enum = CUarray_format_enum(193);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned 8-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT8X4: CUarray_format_enum = CUarray_format_enum(194);
}
impl CUarray_format_enum {
    ///< 1 channel unsigned 16-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT16X1: CUarray_format_enum = CUarray_format_enum(195);
}
impl CUarray_format_enum {
    ///< 2 channel unsigned 16-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT16X2: CUarray_format_enum = CUarray_format_enum(196);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned 16-bit normalized integer
    pub const CU_AD_FORMAT_UNORM_INT16X4: CUarray_format_enum = CUarray_format_enum(197);
}
impl CUarray_format_enum {
    ///< 1 channel signed 8-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT8X1: CUarray_format_enum = CUarray_format_enum(198);
}
impl CUarray_format_enum {
    ///< 2 channel signed 8-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT8X2: CUarray_format_enum = CUarray_format_enum(199);
}
impl CUarray_format_enum {
    ///< 4 channel signed 8-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT8X4: CUarray_format_enum = CUarray_format_enum(200);
}
impl CUarray_format_enum {
    ///< 1 channel signed 16-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT16X1: CUarray_format_enum = CUarray_format_enum(201);
}
impl CUarray_format_enum {
    ///< 2 channel signed 16-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT16X2: CUarray_format_enum = CUarray_format_enum(202);
}
impl CUarray_format_enum {
    ///< 4 channel signed 16-bit normalized integer
    pub const CU_AD_FORMAT_SNORM_INT16X4: CUarray_format_enum = CUarray_format_enum(203);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC1 compression) format
    pub const CU_AD_FORMAT_BC1_UNORM: CUarray_format_enum = CUarray_format_enum(145);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC1 compression) format with sRGB encoding
    pub const CU_AD_FORMAT_BC1_UNORM_SRGB: CUarray_format_enum = CUarray_format_enum(
        146,
    );
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC2 compression) format
    pub const CU_AD_FORMAT_BC2_UNORM: CUarray_format_enum = CUarray_format_enum(147);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC2 compression) format with sRGB encoding
    pub const CU_AD_FORMAT_BC2_UNORM_SRGB: CUarray_format_enum = CUarray_format_enum(
        148,
    );
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC3 compression) format
    pub const CU_AD_FORMAT_BC3_UNORM: CUarray_format_enum = CUarray_format_enum(149);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC3 compression) format with sRGB encoding
    pub const CU_AD_FORMAT_BC3_UNORM_SRGB: CUarray_format_enum = CUarray_format_enum(
        150,
    );
}
impl CUarray_format_enum {
    ///< 1 channel unsigned normalized block-compressed (BC4 compression) format
    pub const CU_AD_FORMAT_BC4_UNORM: CUarray_format_enum = CUarray_format_enum(151);
}
impl CUarray_format_enum {
    ///< 1 channel signed normalized block-compressed (BC4 compression) format
    pub const CU_AD_FORMAT_BC4_SNORM: CUarray_format_enum = CUarray_format_enum(152);
}
impl CUarray_format_enum {
    ///< 2 channel unsigned normalized block-compressed (BC5 compression) format
    pub const CU_AD_FORMAT_BC5_UNORM: CUarray_format_enum = CUarray_format_enum(153);
}
impl CUarray_format_enum {
    ///< 2 channel signed normalized block-compressed (BC5 compression) format
    pub const CU_AD_FORMAT_BC5_SNORM: CUarray_format_enum = CUarray_format_enum(154);
}
impl CUarray_format_enum {
    ///< 3 channel unsigned half-float block-compressed (BC6H compression) format
    pub const CU_AD_FORMAT_BC6H_UF16: CUarray_format_enum = CUarray_format_enum(155);
}
impl CUarray_format_enum {
    ///< 3 channel signed half-float block-compressed (BC6H compression) format
    pub const CU_AD_FORMAT_BC6H_SF16: CUarray_format_enum = CUarray_format_enum(156);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC7 compression) format
    pub const CU_AD_FORMAT_BC7_UNORM: CUarray_format_enum = CUarray_format_enum(157);
}
impl CUarray_format_enum {
    ///< 4 channel unsigned normalized block-compressed (BC7 compression) format with sRGB encoding
    pub const CU_AD_FORMAT_BC7_UNORM_SRGB: CUarray_format_enum = CUarray_format_enum(
        158,
    );
}
#[repr(transparent)]
/// Array formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUarray_format_enum(pub ::core::ffi::c_uint);
/// Array formats
pub use self::CUarray_format_enum as CUarray_format;
impl CUaddress_mode_enum {
    ///< Wrapping address mode
    pub const CU_TR_ADDRESS_MODE_WRAP: CUaddress_mode_enum = CUaddress_mode_enum(0);
}
impl CUaddress_mode_enum {
    ///< Clamp to edge address mode
    pub const CU_TR_ADDRESS_MODE_CLAMP: CUaddress_mode_enum = CUaddress_mode_enum(1);
}
impl CUaddress_mode_enum {
    ///< Mirror address mode
    pub const CU_TR_ADDRESS_MODE_MIRROR: CUaddress_mode_enum = CUaddress_mode_enum(2);
}
impl CUaddress_mode_enum {
    ///< Border address mode
    pub const CU_TR_ADDRESS_MODE_BORDER: CUaddress_mode_enum = CUaddress_mode_enum(3);
}
#[repr(transparent)]
/// Texture reference addressing modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUaddress_mode_enum(pub ::core::ffi::c_uint);
/// Texture reference addressing modes
pub use self::CUaddress_mode_enum as CUaddress_mode;
impl CUfilter_mode_enum {
    ///< Point filter mode
    pub const CU_TR_FILTER_MODE_POINT: CUfilter_mode_enum = CUfilter_mode_enum(0);
}
impl CUfilter_mode_enum {
    ///< Linear filter mode
    pub const CU_TR_FILTER_MODE_LINEAR: CUfilter_mode_enum = CUfilter_mode_enum(1);
}
#[repr(transparent)]
/// Texture reference filtering modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUfilter_mode_enum(pub ::core::ffi::c_uint);
/// Texture reference filtering modes
pub use self::CUfilter_mode_enum as CUfilter_mode;
impl CUdevice_attribute_enum {
    ///< Maximum number of threads per block
    pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        1,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum block dimension X
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X: CUdevice_attribute_enum = CUdevice_attribute_enum(
        2,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum block dimension Y
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y: CUdevice_attribute_enum = CUdevice_attribute_enum(
        3,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum block dimension Z
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z: CUdevice_attribute_enum = CUdevice_attribute_enum(
        4,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum grid dimension X
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X: CUdevice_attribute_enum = CUdevice_attribute_enum(
        5,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum grid dimension Y
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y: CUdevice_attribute_enum = CUdevice_attribute_enum(
        6,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum grid dimension Z
    pub const CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z: CUdevice_attribute_enum = CUdevice_attribute_enum(
        7,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum shared memory available per block in bytes
    pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        8,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, use CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK
    pub const CU_DEVICE_ATTRIBUTE_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        8,
    );
}
impl CUdevice_attribute_enum {
    ///< Memory available on device for __constant__ variables in a CUDA C kernel in bytes
    pub const CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY: CUdevice_attribute_enum = CUdevice_attribute_enum(
        9,
    );
}
impl CUdevice_attribute_enum {
    ///< Warp size in threads
    pub const CU_DEVICE_ATTRIBUTE_WARP_SIZE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        10,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum pitch in bytes allowed by memory copies
    pub const CU_DEVICE_ATTRIBUTE_MAX_PITCH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        11,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum number of 32-bit registers available per block
    pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        12,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, use CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK
    pub const CU_DEVICE_ATTRIBUTE_REGISTERS_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        12,
    );
}
impl CUdevice_attribute_enum {
    ///< Typical clock frequency in kilohertz
    pub const CU_DEVICE_ATTRIBUTE_CLOCK_RATE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        13,
    );
}
impl CUdevice_attribute_enum {
    ///< Alignment requirement for textures
    pub const CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        14,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can possibly copy memory and execute a kernel concurrently. Deprecated. Use instead CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT.
    pub const CU_DEVICE_ATTRIBUTE_GPU_OVERLAP: CUdevice_attribute_enum = CUdevice_attribute_enum(
        15,
    );
}
impl CUdevice_attribute_enum {
    ///< Number of multiprocessors on device
    pub const CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        16,
    );
}
impl CUdevice_attribute_enum {
    ///< Specifies whether there is a run time limit on kernels
    pub const CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        17,
    );
}
impl CUdevice_attribute_enum {
    ///< Device is integrated with host memory
    pub const CU_DEVICE_ATTRIBUTE_INTEGRATED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        18,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can map host memory into CUDA address space
    pub const CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY: CUdevice_attribute_enum = CUdevice_attribute_enum(
        19,
    );
}
impl CUdevice_attribute_enum {
    ///< Compute mode (See ::CUcomputemode for details)
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_MODE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        20,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 1D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        21,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        22,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        23,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        24,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        25,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D texture depth
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        26,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D layered texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        27,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D layered texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        28,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a 2D layered texture
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        29,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, use CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_WIDTH
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        27,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, use CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_HEIGHT
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        28,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, use CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LAYERED_LAYERS
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_ARRAY_NUMSLICES: CUdevice_attribute_enum = CUdevice_attribute_enum(
        29,
    );
}
impl CUdevice_attribute_enum {
    ///< Alignment requirement for surfaces
    pub const CU_DEVICE_ATTRIBUTE_SURFACE_ALIGNMENT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        30,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can possibly execute multiple kernels concurrently
    pub const CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        31,
    );
}
impl CUdevice_attribute_enum {
    ///< Device has ECC support enabled
    pub const CU_DEVICE_ATTRIBUTE_ECC_ENABLED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        32,
    );
}
impl CUdevice_attribute_enum {
    ///< PCI bus ID of the device
    pub const CU_DEVICE_ATTRIBUTE_PCI_BUS_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        33,
    );
}
impl CUdevice_attribute_enum {
    ///< PCI device ID of the device
    pub const CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        34,
    );
}
impl CUdevice_attribute_enum {
    ///< Device is using TCC driver model
    pub const CU_DEVICE_ATTRIBUTE_TCC_DRIVER: CUdevice_attribute_enum = CUdevice_attribute_enum(
        35,
    );
}
impl CUdevice_attribute_enum {
    ///< Peak memory clock frequency in kilohertz
    pub const CU_DEVICE_ATTRIBUTE_MEMORY_CLOCK_RATE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        36,
    );
}
impl CUdevice_attribute_enum {
    ///< Global memory bus width in bits
    pub const CU_DEVICE_ATTRIBUTE_GLOBAL_MEMORY_BUS_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        37,
    );
}
impl CUdevice_attribute_enum {
    ///< Size of L2 cache in bytes
    pub const CU_DEVICE_ATTRIBUTE_L2_CACHE_SIZE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        38,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum resident threads per multiprocessor
    pub const CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        39,
    );
}
impl CUdevice_attribute_enum {
    ///< Number of asynchronous engines
    pub const CU_DEVICE_ATTRIBUTE_ASYNC_ENGINE_COUNT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        40,
    );
}
impl CUdevice_attribute_enum {
    ///< Device shares a unified address space with the host
    pub const CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING: CUdevice_attribute_enum = CUdevice_attribute_enum(
        41,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 1D layered texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        42,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a 1D layered texture
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        43,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, do not use.
    pub const CU_DEVICE_ATTRIBUTE_CAN_TEX2D_GATHER: CUdevice_attribute_enum = CUdevice_attribute_enum(
        44,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D texture width if CUDA_ARRAY3D_TEXTURE_GATHER is set
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        45,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D texture height if CUDA_ARRAY3D_TEXTURE_GATHER is set
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_GATHER_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        46,
    );
}
impl CUdevice_attribute_enum {
    ///< Alternate maximum 3D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_WIDTH_ALTERNATE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        47,
    );
}
impl CUdevice_attribute_enum {
    ///< Alternate maximum 3D texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_HEIGHT_ALTERNATE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        48,
    );
}
impl CUdevice_attribute_enum {
    ///< Alternate maximum 3D texture depth
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE3D_DEPTH_ALTERNATE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        49,
    );
}
impl CUdevice_attribute_enum {
    ///< PCI domain ID of the device
    pub const CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        50,
    );
}
impl CUdevice_attribute_enum {
    ///< Pitch alignment requirement for textures
    pub const CU_DEVICE_ATTRIBUTE_TEXTURE_PITCH_ALIGNMENT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        51,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum cubemap texture width/height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        52,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum cubemap layered texture width/height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        53,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a cubemap layered texture
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        54,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 1D surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        55,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        56,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D surface height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        57,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        58,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D surface height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        59,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 3D surface depth
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE3D_DEPTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        60,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 1D layered surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        61,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a 1D layered surface
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE1D_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        62,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D layered surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        63,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D layered surface height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        64,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a 2D layered surface
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACE2D_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        65,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum cubemap surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        66,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum cubemap layered surface width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        67,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum layers in a cubemap layered surface
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_SURFACECUBEMAP_LAYERED_LAYERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        68,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, do not use. Use cudaDeviceGetTexture1DLinearMaxWidth() or cuDeviceGetTexture1DLinearMaxWidth() instead.
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_LINEAR_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        69,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D linear texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        70,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D linear texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        71,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum 2D linear texture pitch in bytes
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_LINEAR_PITCH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        72,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum mipmapped 2D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        73,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum mipmapped 2D texture height
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE2D_MIPMAPPED_HEIGHT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        74,
    );
}
impl CUdevice_attribute_enum {
    ///< Major compute capability version number
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        75,
    );
}
impl CUdevice_attribute_enum {
    ///< Minor compute capability version number
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        76,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum mipmapped 1D texture width
    pub const CU_DEVICE_ATTRIBUTE_MAXIMUM_TEXTURE1D_MIPMAPPED_WIDTH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        77,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports stream priorities
    pub const CU_DEVICE_ATTRIBUTE_STREAM_PRIORITIES_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        78,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports caching globals in L1
    pub const CU_DEVICE_ATTRIBUTE_GLOBAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        79,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports caching locals in L1
    pub const CU_DEVICE_ATTRIBUTE_LOCAL_L1_CACHE_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        80,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum shared memory available per multiprocessor in bytes
    pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        81,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum number of 32-bit registers available per multiprocessor
    pub const CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        82,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can allocate managed memory on this system
    pub const CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY: CUdevice_attribute_enum = CUdevice_attribute_enum(
        83,
    );
}
impl CUdevice_attribute_enum {
    ///< Device is on a multi-GPU board
    pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD: CUdevice_attribute_enum = CUdevice_attribute_enum(
        84,
    );
}
impl CUdevice_attribute_enum {
    ///< Unique id for a group of devices on the same multi-GPU board
    pub const CU_DEVICE_ATTRIBUTE_MULTI_GPU_BOARD_GROUP_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        85,
    );
}
impl CUdevice_attribute_enum {
    ///< Link between the device and the host supports native atomic operations (this is a placeholder attribute, and is not supported on any current hardware)
    pub const CU_DEVICE_ATTRIBUTE_HOST_NATIVE_ATOMIC_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        86,
    );
}
impl CUdevice_attribute_enum {
    ///< Ratio of single precision performance (in floating-point operations per second) to double precision performance
    pub const CU_DEVICE_ATTRIBUTE_SINGLE_TO_DOUBLE_PRECISION_PERF_RATIO: CUdevice_attribute_enum = CUdevice_attribute_enum(
        87,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports coherently accessing pageable memory without calling cudaHostRegister on it
    pub const CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        88,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can coherently access managed memory concurrently with the CPU
    pub const CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        89,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports compute preemption.
    pub const CU_DEVICE_ATTRIBUTE_COMPUTE_PREEMPTION_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        90,
    );
}
impl CUdevice_attribute_enum {
    ///< Device can access host registered memory at the same virtual address as the CPU
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_HOST_POINTER_FOR_REGISTERED_MEM: CUdevice_attribute_enum = CUdevice_attribute_enum(
        91,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, along with v1 MemOps API, ::cuStreamBatchMemOp and related APIs are supported.
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_MEM_OPS_V1: CUdevice_attribute_enum = CUdevice_attribute_enum(
        92,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, along with v1 MemOps API, 64-bit operations are supported in ::cuStreamBatchMemOp and related APIs.
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS_V1: CUdevice_attribute_enum = CUdevice_attribute_enum(
        93,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, along with v1 MemOps API, ::CU_STREAM_WAIT_VALUE_NOR is supported.
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR_V1: CUdevice_attribute_enum = CUdevice_attribute_enum(
        94,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports launching cooperative kernels via ::cuLaunchCooperativeKernel
    pub const CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        95,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, ::cuLaunchCooperativeKernelMultiDevice is deprecated.
    pub const CU_DEVICE_ATTRIBUTE_COOPERATIVE_MULTI_DEVICE_LAUNCH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        96,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum optin shared memory per block
    pub const CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN: CUdevice_attribute_enum = CUdevice_attribute_enum(
        97,
    );
}
impl CUdevice_attribute_enum {
    ///< The ::CU_STREAM_WAIT_VALUE_FLUSH flag and the ::CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES MemOp are supported on the device. See \ref CUDA_MEMOP for additional details.
    pub const CU_DEVICE_ATTRIBUTE_CAN_FLUSH_REMOTE_WRITES: CUdevice_attribute_enum = CUdevice_attribute_enum(
        98,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports host memory registration via ::cudaHostRegister.
    pub const CU_DEVICE_ATTRIBUTE_HOST_REGISTER_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        99,
    );
}
impl CUdevice_attribute_enum {
    ///< Device accesses pageable memory via the host's page tables.
    pub const CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS_USES_HOST_PAGE_TABLES: CUdevice_attribute_enum = CUdevice_attribute_enum(
        100,
    );
}
impl CUdevice_attribute_enum {
    ///< The host can directly access managed memory on the device without migration.
    pub const CU_DEVICE_ATTRIBUTE_DIRECT_MANAGED_MEM_ACCESS_FROM_HOST: CUdevice_attribute_enum = CUdevice_attribute_enum(
        101,
    );
}
impl CUdevice_attribute_enum {
    ///< Deprecated, Use CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED
    pub const CU_DEVICE_ATTRIBUTE_VIRTUAL_ADDRESS_MANAGEMENT_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        102,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports virtual memory management APIs like ::cuMemAddressReserve, ::cuMemCreate, ::cuMemMap and related APIs
    pub const CU_DEVICE_ATTRIBUTE_VIRTUAL_MEMORY_MANAGEMENT_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        102,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports exporting memory to a posix file descriptor with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
    pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        103,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports exporting memory to a Win32 NT handle with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
    pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_HANDLE_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        104,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports exporting memory to a Win32 KMT handle with ::cuMemExportToShareableHandle, if requested via ::cuMemCreate
    pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_WIN32_KMT_HANDLE_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        105,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum number of blocks per multiprocessor
    pub const CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        106,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports compression of memory
    pub const CU_DEVICE_ATTRIBUTE_GENERIC_COMPRESSION_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        107,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum L2 persisting lines capacity setting in bytes.
    pub const CU_DEVICE_ATTRIBUTE_MAX_PERSISTING_L2_CACHE_SIZE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        108,
    );
}
impl CUdevice_attribute_enum {
    ///< Maximum value of CUaccessPolicyWindow::num_bytes.
    pub const CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE: CUdevice_attribute_enum = CUdevice_attribute_enum(
        109,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports specifying the GPUDirect RDMA flag with ::cuMemCreate
    pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WITH_CUDA_VMM_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        110,
    );
}
impl CUdevice_attribute_enum {
    ///< Shared memory reserved by CUDA driver per block in bytes
    pub const CU_DEVICE_ATTRIBUTE_RESERVED_SHARED_MEMORY_PER_BLOCK: CUdevice_attribute_enum = CUdevice_attribute_enum(
        111,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports sparse CUDA arrays and sparse CUDA mipmapped arrays
    pub const CU_DEVICE_ATTRIBUTE_SPARSE_CUDA_ARRAY_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        112,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports using the ::cuMemHostRegister flag ::CU_MEMHOSTERGISTER_READ_ONLY to register memory that must be mapped as read-only to the GPU
    pub const CU_DEVICE_ATTRIBUTE_READ_ONLY_HOST_REGISTER_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        113,
    );
}
impl CUdevice_attribute_enum {
    ///< External timeline semaphore interop is supported on the device
    pub const CU_DEVICE_ATTRIBUTE_TIMELINE_SEMAPHORE_INTEROP_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        114,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports using the ::cuMemAllocAsync and ::cuMemPool family of APIs
    pub const CU_DEVICE_ATTRIBUTE_MEMORY_POOLS_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        115,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports GPUDirect RDMA APIs, like nvidia_p2p_get_pages (see https://docs.nvidia.com/cuda/gpudirect-rdma for more information)
    pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        116,
    );
}
impl CUdevice_attribute_enum {
    ///< The returned attribute shall be interpreted as a bitmask, where the individual bits are described by the ::CUflushGPUDirectRDMAWritesOptions enum
    pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        117,
    );
}
impl CUdevice_attribute_enum {
    ///< GPUDirect RDMA writes to the device do not need to be flushed for consumers within the scope indicated by the returned attribute. See ::CUGPUDirectRDMAWritesOrdering for the numerical values returned here.
    pub const CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_WRITES_ORDERING: CUdevice_attribute_enum = CUdevice_attribute_enum(
        118,
    );
}
impl CUdevice_attribute_enum {
    ///< Handle types supported with mempool based IPC
    pub const CU_DEVICE_ATTRIBUTE_MEMPOOL_SUPPORTED_HANDLE_TYPES: CUdevice_attribute_enum = CUdevice_attribute_enum(
        119,
    );
}
impl CUdevice_attribute_enum {
    ///< Indicates device supports cluster launch
    pub const CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH: CUdevice_attribute_enum = CUdevice_attribute_enum(
        120,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports deferred mapping CUDA arrays and CUDA mipmapped arrays
    pub const CU_DEVICE_ATTRIBUTE_DEFERRED_MAPPING_CUDA_ARRAY_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        121,
    );
}
impl CUdevice_attribute_enum {
    ///< 64-bit operations are supported in ::cuStreamBatchMemOp and related MemOp APIs.
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_64_BIT_STREAM_MEM_OPS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        122,
    );
}
impl CUdevice_attribute_enum {
    ///< ::CU_STREAM_WAIT_VALUE_NOR is supported by MemOp APIs.
    pub const CU_DEVICE_ATTRIBUTE_CAN_USE_STREAM_WAIT_VALUE_NOR: CUdevice_attribute_enum = CUdevice_attribute_enum(
        123,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports buffer sharing with dma_buf mechanism.
    pub const CU_DEVICE_ATTRIBUTE_DMA_BUF_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        124,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports IPC Events.
    pub const CU_DEVICE_ATTRIBUTE_IPC_EVENT_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        125,
    );
}
impl CUdevice_attribute_enum {
    ///< Number of memory domains the device supports.
    pub const CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT: CUdevice_attribute_enum = CUdevice_attribute_enum(
        126,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports accessing memory using Tensor Map.
    pub const CU_DEVICE_ATTRIBUTE_TENSOR_MAP_ACCESS_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        127,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports exporting memory to a fabric handle with cuMemExportToShareableHandle() or requested with cuMemCreate()
    pub const CU_DEVICE_ATTRIBUTE_HANDLE_TYPE_FABRIC_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        128,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports unified function pointers.
    pub const CU_DEVICE_ATTRIBUTE_UNIFIED_FUNCTION_POINTERS: CUdevice_attribute_enum = CUdevice_attribute_enum(
        129,
    );
}
impl CUdevice_attribute_enum {
    pub const CU_DEVICE_ATTRIBUTE_NUMA_CONFIG: CUdevice_attribute_enum = CUdevice_attribute_enum(
        130,
    );
}
impl CUdevice_attribute_enum {
    pub const CU_DEVICE_ATTRIBUTE_NUMA_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        131,
    );
}
impl CUdevice_attribute_enum {
    ///< Device supports switch multicast and reduction operations.
    pub const CU_DEVICE_ATTRIBUTE_MULTICAST_SUPPORTED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        132,
    );
}
impl CUdevice_attribute_enum {
    ///< Indicates if contexts created on this device will be shared via MPS
    pub const CU_DEVICE_ATTRIBUTE_MPS_ENABLED: CUdevice_attribute_enum = CUdevice_attribute_enum(
        133,
    );
}
impl CUdevice_attribute_enum {
    ///< NUMA ID of the host node closest to the device. Returns -1 when system does not support NUMA.
    pub const CU_DEVICE_ATTRIBUTE_HOST_NUMA_ID: CUdevice_attribute_enum = CUdevice_attribute_enum(
        134,
    );
}
impl CUdevice_attribute_enum {
    pub const CU_DEVICE_ATTRIBUTE_MAX: CUdevice_attribute_enum = CUdevice_attribute_enum(
        135,
    );
}
#[repr(transparent)]
/// Device properties
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdevice_attribute_enum(pub ::core::ffi::c_uint);
/// Device properties
pub use self::CUdevice_attribute_enum as CUdevice_attribute;
/// Legacy device properties
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUdevprop_st {
    ///< Maximum number of threads per block
    pub maxThreadsPerBlock: ::core::ffi::c_int,
    ///< Maximum size of each dimension of a block
    pub maxThreadsDim: [::core::ffi::c_int; 3usize],
    ///< Maximum size of each dimension of a grid
    pub maxGridSize: [::core::ffi::c_int; 3usize],
    ///< Shared memory available per block in bytes
    pub sharedMemPerBlock: ::core::ffi::c_int,
    ///< Constant memory available on device in bytes
    pub totalConstantMemory: ::core::ffi::c_int,
    ///< Warp size in threads
    pub SIMDWidth: ::core::ffi::c_int,
    ///< Maximum pitch in bytes allowed by memory copies
    pub memPitch: ::core::ffi::c_int,
    ///< 32-bit registers available per block
    pub regsPerBlock: ::core::ffi::c_int,
    ///< Clock frequency in kilohertz
    pub clockRate: ::core::ffi::c_int,
    ///< Alignment requirement for textures
    pub textureAlign: ::core::ffi::c_int,
}
/// Legacy device properties
pub type CUdevprop_v1 = CUdevprop_st;
/// Legacy device properties
pub type CUdevprop = CUdevprop_v1;
impl CUpointer_attribute_enum {
    ///< The ::CUcontext on which a pointer was allocated or registered
    pub const CU_POINTER_ATTRIBUTE_CONTEXT: CUpointer_attribute_enum = CUpointer_attribute_enum(
        1,
    );
}
impl CUpointer_attribute_enum {
    ///< The ::CUmemorytype describing the physical location of a pointer
    pub const CU_POINTER_ATTRIBUTE_MEMORY_TYPE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        2,
    );
}
impl CUpointer_attribute_enum {
    ///< The address at which a pointer's memory may be accessed on the device
    pub const CU_POINTER_ATTRIBUTE_DEVICE_POINTER: CUpointer_attribute_enum = CUpointer_attribute_enum(
        3,
    );
}
impl CUpointer_attribute_enum {
    ///< The address at which a pointer's memory may be accessed on the host
    pub const CU_POINTER_ATTRIBUTE_HOST_POINTER: CUpointer_attribute_enum = CUpointer_attribute_enum(
        4,
    );
}
impl CUpointer_attribute_enum {
    ///< A pair of tokens for use with the nv-p2p.h Linux kernel interface
    pub const CU_POINTER_ATTRIBUTE_P2P_TOKENS: CUpointer_attribute_enum = CUpointer_attribute_enum(
        5,
    );
}
impl CUpointer_attribute_enum {
    ///< Synchronize every synchronous memory operation initiated on this region
    pub const CU_POINTER_ATTRIBUTE_SYNC_MEMOPS: CUpointer_attribute_enum = CUpointer_attribute_enum(
        6,
    );
}
impl CUpointer_attribute_enum {
    ///< A process-wide unique ID for an allocated memory region
    pub const CU_POINTER_ATTRIBUTE_BUFFER_ID: CUpointer_attribute_enum = CUpointer_attribute_enum(
        7,
    );
}
impl CUpointer_attribute_enum {
    ///< Indicates if the pointer points to managed memory
    pub const CU_POINTER_ATTRIBUTE_IS_MANAGED: CUpointer_attribute_enum = CUpointer_attribute_enum(
        8,
    );
}
impl CUpointer_attribute_enum {
    ///< A device ordinal of a device on which a pointer was allocated or registered
    pub const CU_POINTER_ATTRIBUTE_DEVICE_ORDINAL: CUpointer_attribute_enum = CUpointer_attribute_enum(
        9,
    );
}
impl CUpointer_attribute_enum {
    ///< 1 if this pointer maps to an allocation that is suitable for ::cudaIpcGetMemHandle, 0 otherwise
    pub const CU_POINTER_ATTRIBUTE_IS_LEGACY_CUDA_IPC_CAPABLE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        10,
    );
}
impl CUpointer_attribute_enum {
    ///< Starting address for this requested pointer
    pub const CU_POINTER_ATTRIBUTE_RANGE_START_ADDR: CUpointer_attribute_enum = CUpointer_attribute_enum(
        11,
    );
}
impl CUpointer_attribute_enum {
    ///< Size of the address range for this requested pointer
    pub const CU_POINTER_ATTRIBUTE_RANGE_SIZE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        12,
    );
}
impl CUpointer_attribute_enum {
    ///< 1 if this pointer is in a valid address range that is mapped to a backing allocation, 0 otherwise
    pub const CU_POINTER_ATTRIBUTE_MAPPED: CUpointer_attribute_enum = CUpointer_attribute_enum(
        13,
    );
}
impl CUpointer_attribute_enum {
    ///< Bitmask of allowed ::CUmemAllocationHandleType for this allocation
    pub const CU_POINTER_ATTRIBUTE_ALLOWED_HANDLE_TYPES: CUpointer_attribute_enum = CUpointer_attribute_enum(
        14,
    );
}
impl CUpointer_attribute_enum {
    ///< 1 if the memory this pointer is referencing can be used with the GPUDirect RDMA API
    pub const CU_POINTER_ATTRIBUTE_IS_GPU_DIRECT_RDMA_CAPABLE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        15,
    );
}
impl CUpointer_attribute_enum {
    ///< Returns the access flags the device associated with the current context has on the corresponding memory referenced by the pointer given
    pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAGS: CUpointer_attribute_enum = CUpointer_attribute_enum(
        16,
    );
}
impl CUpointer_attribute_enum {
    ///< Returns the mempool handle for the allocation if it was allocated from a mempool. Otherwise returns NULL.
    pub const CU_POINTER_ATTRIBUTE_MEMPOOL_HANDLE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        17,
    );
}
impl CUpointer_attribute_enum {
    ///< Size of the actual underlying mapping that the pointer belongs to
    pub const CU_POINTER_ATTRIBUTE_MAPPING_SIZE: CUpointer_attribute_enum = CUpointer_attribute_enum(
        18,
    );
}
impl CUpointer_attribute_enum {
    ///< The start address of the mapping that the pointer belongs to
    pub const CU_POINTER_ATTRIBUTE_MAPPING_BASE_ADDR: CUpointer_attribute_enum = CUpointer_attribute_enum(
        19,
    );
}
impl CUpointer_attribute_enum {
    ///< A process-wide unique id corresponding to the physical allocation the pointer belongs to
    pub const CU_POINTER_ATTRIBUTE_MEMORY_BLOCK_ID: CUpointer_attribute_enum = CUpointer_attribute_enum(
        20,
    );
}
#[repr(transparent)]
/// Pointer information
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUpointer_attribute_enum(pub ::core::ffi::c_uint);
/// Pointer information
pub use self::CUpointer_attribute_enum as CUpointer_attribute;
impl CUfunction_attribute_enum {
    /** The maximum number of threads per block, beyond which a launch of the
 function would fail. This number depends on both the function and the
 device on which the function is currently loaded.*/
    pub const CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK: CUfunction_attribute_enum = CUfunction_attribute_enum(
        0,
    );
}
impl CUfunction_attribute_enum {
    /** The size in bytes of statically-allocated shared memory required by
 this function. This does not include dynamically-allocated shared
 memory requested by the user at runtime.*/
    pub const CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES: CUfunction_attribute_enum = CUfunction_attribute_enum(
        1,
    );
}
impl CUfunction_attribute_enum {
    /** The size in bytes of user-allocated constant memory required by this
 function.*/
    pub const CU_FUNC_ATTRIBUTE_CONST_SIZE_BYTES: CUfunction_attribute_enum = CUfunction_attribute_enum(
        2,
    );
}
impl CUfunction_attribute_enum {
    /// The size in bytes of local memory used by each thread of this function.
    pub const CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES: CUfunction_attribute_enum = CUfunction_attribute_enum(
        3,
    );
}
impl CUfunction_attribute_enum {
    /// The number of registers used by each thread of this function.
    pub const CU_FUNC_ATTRIBUTE_NUM_REGS: CUfunction_attribute_enum = CUfunction_attribute_enum(
        4,
    );
}
impl CUfunction_attribute_enum {
    /** The PTX virtual architecture version for which the function was
 compiled. This value is the major PTX version * 10 + the minor PTX
 version, so a PTX version 1.3 function would return the value 13.
 Note that this may return the undefined value of 0 for cubins
 compiled prior to CUDA 3.0.*/
    pub const CU_FUNC_ATTRIBUTE_PTX_VERSION: CUfunction_attribute_enum = CUfunction_attribute_enum(
        5,
    );
}
impl CUfunction_attribute_enum {
    /** The binary architecture version for which the function was compiled.
 This value is the major binary version * 10 + the minor binary version,
 so a binary version 1.3 function would return the value 13. Note that
 this will return a value of 10 for legacy cubins that do not have a
 properly-encoded binary architecture version.*/
    pub const CU_FUNC_ATTRIBUTE_BINARY_VERSION: CUfunction_attribute_enum = CUfunction_attribute_enum(
        6,
    );
}
impl CUfunction_attribute_enum {
    /** The attribute to indicate whether the function has been compiled with
 user specified option "-Xptxas --dlcm=ca" set .*/
    pub const CU_FUNC_ATTRIBUTE_CACHE_MODE_CA: CUfunction_attribute_enum = CUfunction_attribute_enum(
        7,
    );
}
impl CUfunction_attribute_enum {
    /** The maximum size in bytes of dynamically-allocated shared memory that can be used by
 this function. If the user-specified dynamic shared memory size is larger than this
 value, the launch will fail.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES: CUfunction_attribute_enum = CUfunction_attribute_enum(
        8,
    );
}
impl CUfunction_attribute_enum {
    /** On devices where the L1 cache and shared memory use the same hardware resources,
 this sets the shared memory carveout preference, in percent of the total shared memory.
 Refer to ::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR.
 This is only a hint, and the driver can choose a different ratio if required to execute the function.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_PREFERRED_SHARED_MEMORY_CARVEOUT: CUfunction_attribute_enum = CUfunction_attribute_enum(
        9,
    );
}
impl CUfunction_attribute_enum {
    /** If this attribute is set, the kernel must launch with a valid cluster
 size specified.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_CLUSTER_SIZE_MUST_BE_SET: CUfunction_attribute_enum = CUfunction_attribute_enum(
        10,
    );
}
impl CUfunction_attribute_enum {
    /** The required cluster width in blocks. The values must either all be 0 or
 all be positive. The validity of the cluster dimensions is otherwise
 checked at launch time.

 If the value is set during compile time, it cannot be set at runtime.
 Setting it at runtime will return CUDA_ERROR_NOT_PERMITTED.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_WIDTH: CUfunction_attribute_enum = CUfunction_attribute_enum(
        11,
    );
}
impl CUfunction_attribute_enum {
    /** The required cluster height in blocks. The values must either all be 0 or
 all be positive. The validity of the cluster dimensions is otherwise
 checked at launch time.

 If the value is set during compile time, it cannot be set at runtime.
 Setting it at runtime should return CUDA_ERROR_NOT_PERMITTED.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_HEIGHT: CUfunction_attribute_enum = CUfunction_attribute_enum(
        12,
    );
}
impl CUfunction_attribute_enum {
    /** The required cluster depth in blocks. The values must either all be 0 or
 all be positive. The validity of the cluster dimensions is otherwise
 checked at launch time.

 If the value is set during compile time, it cannot be set at runtime.
 Setting it at runtime should return CUDA_ERROR_NOT_PERMITTED.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_REQUIRED_CLUSTER_DEPTH: CUfunction_attribute_enum = CUfunction_attribute_enum(
        13,
    );
}
impl CUfunction_attribute_enum {
    /** Whether the function can be launched with non-portable cluster size. 1 is
 allowed, 0 is disallowed. A non-portable cluster size may only function
 on the specific SKUs the program is tested on. The launch might fail if
 the program is run on a different hardware platform.

 CUDA API provides cudaOccupancyMaxActiveClusters to assist with checking
 whether the desired size can be launched on the current device.

 Portable Cluster Size

 A portable cluster size is guaranteed to be functional on all compute
 capabilities higher than the target compute capability. The portable
 cluster size for sm_90 is 8 blocks per cluster. This value may increase
 for future compute capabilities.

 The specific hardware unit may support higher cluster sizes that’s not
 guaranteed to be portable.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_NON_PORTABLE_CLUSTER_SIZE_ALLOWED: CUfunction_attribute_enum = CUfunction_attribute_enum(
        14,
    );
}
impl CUfunction_attribute_enum {
    /** The block scheduling policy of a function. The value type is
 CUclusterSchedulingPolicy / cudaClusterSchedulingPolicy.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: CUfunction_attribute_enum = CUfunction_attribute_enum(
        15,
    );
}
impl CUfunction_attribute_enum {
    /** The block scheduling policy of a function. The value type is
 CUclusterSchedulingPolicy / cudaClusterSchedulingPolicy.
 See ::cuFuncSetAttribute, ::cuKernelSetAttribute*/
    pub const CU_FUNC_ATTRIBUTE_MAX: CUfunction_attribute_enum = CUfunction_attribute_enum(
        16,
    );
}
#[repr(transparent)]
/// Function properties
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUfunction_attribute_enum(pub ::core::ffi::c_uint);
/// Function properties
pub use self::CUfunction_attribute_enum as CUfunction_attribute;
impl CUfunc_cache_enum {
    ///< no preference for shared memory or L1 (default)
    pub const CU_FUNC_CACHE_PREFER_NONE: CUfunc_cache_enum = CUfunc_cache_enum(0);
}
impl CUfunc_cache_enum {
    ///< prefer larger shared memory and smaller L1 cache
    pub const CU_FUNC_CACHE_PREFER_SHARED: CUfunc_cache_enum = CUfunc_cache_enum(1);
}
impl CUfunc_cache_enum {
    ///< prefer larger L1 cache and smaller shared memory
    pub const CU_FUNC_CACHE_PREFER_L1: CUfunc_cache_enum = CUfunc_cache_enum(2);
}
impl CUfunc_cache_enum {
    ///< prefer equal sized L1 cache and shared memory
    pub const CU_FUNC_CACHE_PREFER_EQUAL: CUfunc_cache_enum = CUfunc_cache_enum(3);
}
#[repr(transparent)]
/// Function cache configurations
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUfunc_cache_enum(pub ::core::ffi::c_uint);
/// Function cache configurations
pub use self::CUfunc_cache_enum as CUfunc_cache;
impl CUsharedconfig_enum {
    ///< set default shared memory bank size
    pub const CU_SHARED_MEM_CONFIG_DEFAULT_BANK_SIZE: CUsharedconfig_enum = CUsharedconfig_enum(
        0,
    );
}
impl CUsharedconfig_enum {
    ///< set shared memory bank width to four bytes
    pub const CU_SHARED_MEM_CONFIG_FOUR_BYTE_BANK_SIZE: CUsharedconfig_enum = CUsharedconfig_enum(
        1,
    );
}
impl CUsharedconfig_enum {
    ///< set shared memory bank width to eight bytes
    pub const CU_SHARED_MEM_CONFIG_EIGHT_BYTE_BANK_SIZE: CUsharedconfig_enum = CUsharedconfig_enum(
        2,
    );
}
#[repr(transparent)]
/** \deprecated

 Shared memory configurations*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUsharedconfig_enum(pub ::core::ffi::c_uint);
/** \deprecated

 Shared memory configurations*/
pub use self::CUsharedconfig_enum as CUsharedconfig;
impl CUshared_carveout_enum {
    ///< No preference for shared memory or L1 (default)
    pub const CU_SHAREDMEM_CARVEOUT_DEFAULT: CUshared_carveout_enum = CUshared_carveout_enum(
        -1,
    );
}
impl CUshared_carveout_enum {
    ///< Prefer maximum available shared memory, minimum L1 cache
    pub const CU_SHAREDMEM_CARVEOUT_MAX_SHARED: CUshared_carveout_enum = CUshared_carveout_enum(
        100,
    );
}
impl CUshared_carveout_enum {
    ///< Prefer maximum available L1 cache, minimum shared memory
    pub const CU_SHAREDMEM_CARVEOUT_MAX_L1: CUshared_carveout_enum = CUshared_carveout_enum(
        0,
    );
}
#[repr(transparent)]
/// Shared memory carveout configurations. These may be passed to ::cuFuncSetAttribute or ::cuKernelSetAttribute
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUshared_carveout_enum(pub ::core::ffi::c_int);
/// Shared memory carveout configurations. These may be passed to ::cuFuncSetAttribute or ::cuKernelSetAttribute
pub use self::CUshared_carveout_enum as CUshared_carveout;
impl CUmemorytype_enum {
    ///< Host memory
    pub const CU_MEMORYTYPE_HOST: CUmemorytype_enum = CUmemorytype_enum(1);
}
impl CUmemorytype_enum {
    ///< Device memory
    pub const CU_MEMORYTYPE_DEVICE: CUmemorytype_enum = CUmemorytype_enum(2);
}
impl CUmemorytype_enum {
    ///< Array memory
    pub const CU_MEMORYTYPE_ARRAY: CUmemorytype_enum = CUmemorytype_enum(3);
}
impl CUmemorytype_enum {
    ///< Unified device or host memory
    pub const CU_MEMORYTYPE_UNIFIED: CUmemorytype_enum = CUmemorytype_enum(4);
}
#[repr(transparent)]
/// Memory types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemorytype_enum(pub ::core::ffi::c_uint);
/// Memory types
pub use self::CUmemorytype_enum as CUmemorytype;
impl CUcomputemode_enum {
    ///< Default compute mode (Multiple contexts allowed per device)
    pub const CU_COMPUTEMODE_DEFAULT: CUcomputemode_enum = CUcomputemode_enum(0);
}
impl CUcomputemode_enum {
    ///< Compute-prohibited mode (No contexts can be created on this device at this time)
    pub const CU_COMPUTEMODE_PROHIBITED: CUcomputemode_enum = CUcomputemode_enum(2);
}
impl CUcomputemode_enum {
    ///< Compute-exclusive-process mode (Only one context used by a single process can be present on this device at a time)
    pub const CU_COMPUTEMODE_EXCLUSIVE_PROCESS: CUcomputemode_enum = CUcomputemode_enum(
        3,
    );
}
#[repr(transparent)]
/// Compute Modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUcomputemode_enum(pub ::core::ffi::c_uint);
/// Compute Modes
pub use self::CUcomputemode_enum as CUcomputemode;
impl CUmem_advise_enum {
    ///< Data will mostly be read and only occasionally be written to
    pub const CU_MEM_ADVISE_SET_READ_MOSTLY: CUmem_advise_enum = CUmem_advise_enum(1);
}
impl CUmem_advise_enum {
    ///< Undo the effect of ::CU_MEM_ADVISE_SET_READ_MOSTLY
    pub const CU_MEM_ADVISE_UNSET_READ_MOSTLY: CUmem_advise_enum = CUmem_advise_enum(2);
}
impl CUmem_advise_enum {
    ///< Set the preferred location for the data as the specified device
    pub const CU_MEM_ADVISE_SET_PREFERRED_LOCATION: CUmem_advise_enum = CUmem_advise_enum(
        3,
    );
}
impl CUmem_advise_enum {
    ///< Clear the preferred location for the data
    pub const CU_MEM_ADVISE_UNSET_PREFERRED_LOCATION: CUmem_advise_enum = CUmem_advise_enum(
        4,
    );
}
impl CUmem_advise_enum {
    ///< Data will be accessed by the specified device, so prevent page faults as much as possible
    pub const CU_MEM_ADVISE_SET_ACCESSED_BY: CUmem_advise_enum = CUmem_advise_enum(5);
}
impl CUmem_advise_enum {
    ///< Let the Unified Memory subsystem decide on the page faulting policy for the specified device
    pub const CU_MEM_ADVISE_UNSET_ACCESSED_BY: CUmem_advise_enum = CUmem_advise_enum(6);
}
#[repr(transparent)]
/// Memory advise values
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmem_advise_enum(pub ::core::ffi::c_uint);
/// Memory advise values
pub use self::CUmem_advise_enum as CUmem_advise;
impl CUmem_range_attribute_enum {
    ///< Whether the range will mostly be read and only occasionally be written to
    pub const CU_MEM_RANGE_ATTRIBUTE_READ_MOSTLY: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        1,
    );
}
impl CUmem_range_attribute_enum {
    ///< The preferred location of the range
    pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        2,
    );
}
impl CUmem_range_attribute_enum {
    ///< Memory range has ::CU_MEM_ADVISE_SET_ACCESSED_BY set for specified device
    pub const CU_MEM_RANGE_ATTRIBUTE_ACCESSED_BY: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        3,
    );
}
impl CUmem_range_attribute_enum {
    ///< The last location to which the range was prefetched
    pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        4,
    );
}
impl CUmem_range_attribute_enum {
    ///< The preferred location type of the range
    pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_TYPE: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        5,
    );
}
impl CUmem_range_attribute_enum {
    ///< The preferred location id of the range
    pub const CU_MEM_RANGE_ATTRIBUTE_PREFERRED_LOCATION_ID: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        6,
    );
}
impl CUmem_range_attribute_enum {
    ///< The last location type to which the range was prefetched
    pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_TYPE: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        7,
    );
}
impl CUmem_range_attribute_enum {
    ///< The last location id to which the range was prefetched
    pub const CU_MEM_RANGE_ATTRIBUTE_LAST_PREFETCH_LOCATION_ID: CUmem_range_attribute_enum = CUmem_range_attribute_enum(
        8,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmem_range_attribute_enum(pub ::core::ffi::c_uint);
pub use self::CUmem_range_attribute_enum as CUmem_range_attribute;
impl CUjit_option_enum {
    /** Max number of registers that a thread may use.\n
 Option type: unsigned int\n
 Applies to: compiler only*/
    pub const CU_JIT_MAX_REGISTERS: CUjit_option_enum = CUjit_option_enum(0);
}
impl CUjit_option_enum {
    /** IN: Specifies minimum number of threads per block to target compilation
 for\n
 OUT: Returns the number of threads the compiler actually targeted.
 This restricts the resource utilization of the compiler (e.g. max
 registers) such that a block with the given number of threads should be
 able to launch based on register limitations. Note, this option does not
 currently take into account any other resource limitations, such as
 shared memory utilization.\n
 Cannot be combined with ::CU_JIT_TARGET.\n
 Option type: unsigned int\n
 Applies to: compiler only*/
    pub const CU_JIT_THREADS_PER_BLOCK: CUjit_option_enum = CUjit_option_enum(1);
}
impl CUjit_option_enum {
    /** Overwrites the option value with the total wall clock time, in
 milliseconds, spent in the compiler and linker\n
 Option type: float\n
 Applies to: compiler and linker*/
    pub const CU_JIT_WALL_TIME: CUjit_option_enum = CUjit_option_enum(2);
}
impl CUjit_option_enum {
    /** Pointer to a buffer in which to print any log messages
 that are informational in nature (the buffer size is specified via
 option ::CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES)\n
 Option type: char *\n
 Applies to: compiler and linker*/
    pub const CU_JIT_INFO_LOG_BUFFER: CUjit_option_enum = CUjit_option_enum(3);
}
impl CUjit_option_enum {
    /** IN: Log buffer size in bytes.  Log messages will be capped at this size
 (including null terminator)\n
 OUT: Amount of log buffer filled with messages\n
 Option type: unsigned int\n
 Applies to: compiler and linker*/
    pub const CU_JIT_INFO_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = CUjit_option_enum(
        4,
    );
}
impl CUjit_option_enum {
    /** Pointer to a buffer in which to print any log messages that
 reflect errors (the buffer size is specified via option
 ::CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES)\n
 Option type: char *\n
 Applies to: compiler and linker*/
    pub const CU_JIT_ERROR_LOG_BUFFER: CUjit_option_enum = CUjit_option_enum(5);
}
impl CUjit_option_enum {
    /** IN: Log buffer size in bytes.  Log messages will be capped at this size
 (including null terminator)\n
 OUT: Amount of log buffer filled with messages\n
 Option type: unsigned int\n
 Applies to: compiler and linker*/
    pub const CU_JIT_ERROR_LOG_BUFFER_SIZE_BYTES: CUjit_option_enum = CUjit_option_enum(
        6,
    );
}
impl CUjit_option_enum {
    /** Level of optimizations to apply to generated code (0 - 4), with 4
 being the default and highest level of optimizations.\n
 Option type: unsigned int\n
 Applies to: compiler only*/
    pub const CU_JIT_OPTIMIZATION_LEVEL: CUjit_option_enum = CUjit_option_enum(7);
}
impl CUjit_option_enum {
    /** No option value required. Determines the target based on the current
 attached context (default)\n
 Option type: No option value needed\n
 Applies to: compiler and linker*/
    pub const CU_JIT_TARGET_FROM_CUCONTEXT: CUjit_option_enum = CUjit_option_enum(8);
}
impl CUjit_option_enum {
    /** Target is chosen based on supplied ::CUjit_target.  Cannot be
 combined with ::CU_JIT_THREADS_PER_BLOCK.\n
 Option type: unsigned int for enumerated type ::CUjit_target\n
 Applies to: compiler and linker*/
    pub const CU_JIT_TARGET: CUjit_option_enum = CUjit_option_enum(9);
}
impl CUjit_option_enum {
    /** Specifies choice of fallback strategy if matching cubin is not found.
 Choice is based on supplied ::CUjit_fallback.  This option cannot be
 used with cuLink* APIs as the linker requires exact matches.\n
 Option type: unsigned int for enumerated type ::CUjit_fallback\n
 Applies to: compiler only*/
    pub const CU_JIT_FALLBACK_STRATEGY: CUjit_option_enum = CUjit_option_enum(10);
}
impl CUjit_option_enum {
    /** Specifies whether to create debug information in output (-g)
 (0: false, default)\n
 Option type: int\n
 Applies to: compiler and linker*/
    pub const CU_JIT_GENERATE_DEBUG_INFO: CUjit_option_enum = CUjit_option_enum(11);
}
impl CUjit_option_enum {
    /** Generate verbose log messages (0: false, default)\n
 Option type: int\n
 Applies to: compiler and linker*/
    pub const CU_JIT_LOG_VERBOSE: CUjit_option_enum = CUjit_option_enum(12);
}
impl CUjit_option_enum {
    /** Generate line number information (-lineinfo) (0: false, default)\n
 Option type: int\n
 Applies to: compiler only*/
    pub const CU_JIT_GENERATE_LINE_INFO: CUjit_option_enum = CUjit_option_enum(13);
}
impl CUjit_option_enum {
    /** Specifies whether to enable caching explicitly (-dlcm) \n
 Choice is based on supplied ::CUjit_cacheMode_enum.\n
 Option type: unsigned int for enumerated type ::CUjit_cacheMode_enum\n
 Applies to: compiler only*/
    pub const CU_JIT_CACHE_MODE: CUjit_option_enum = CUjit_option_enum(14);
}
impl CUjit_option_enum {
    /** \deprecated
 This jit option is deprecated and should not be used.*/
    pub const CU_JIT_NEW_SM3X_OPT: CUjit_option_enum = CUjit_option_enum(15);
}
impl CUjit_option_enum {
    /// This jit option is used for internal purpose only.
    pub const CU_JIT_FAST_COMPILE: CUjit_option_enum = CUjit_option_enum(16);
}
impl CUjit_option_enum {
    /** Array of device symbol names that will be relocated to the corresponding
 host addresses stored in ::CU_JIT_GLOBAL_SYMBOL_ADDRESSES.\n
 Must contain ::CU_JIT_GLOBAL_SYMBOL_COUNT entries.\n
 When loading a device module, driver will relocate all encountered
 unresolved symbols to the host addresses.\n
 It is only allowed to register symbols that correspond to unresolved
 global variables.\n
 It is illegal to register the same device symbol at multiple addresses.\n
 Option type: const char **\n
 Applies to: dynamic linker only*/
    pub const CU_JIT_GLOBAL_SYMBOL_NAMES: CUjit_option_enum = CUjit_option_enum(17);
}
impl CUjit_option_enum {
    /** Array of host addresses that will be used to relocate corresponding
 device symbols stored in ::CU_JIT_GLOBAL_SYMBOL_NAMES.\n
 Must contain ::CU_JIT_GLOBAL_SYMBOL_COUNT entries.\n
 Option type: void **\n
 Applies to: dynamic linker only*/
    pub const CU_JIT_GLOBAL_SYMBOL_ADDRESSES: CUjit_option_enum = CUjit_option_enum(18);
}
impl CUjit_option_enum {
    /** Number of entries in ::CU_JIT_GLOBAL_SYMBOL_NAMES and
 ::CU_JIT_GLOBAL_SYMBOL_ADDRESSES arrays.\n
 Option type: unsigned int\n
 Applies to: dynamic linker only*/
    pub const CU_JIT_GLOBAL_SYMBOL_COUNT: CUjit_option_enum = CUjit_option_enum(19);
}
impl CUjit_option_enum {
    /** \deprecated
 Enable link-time optimization (-dlto) for device code (Disabled by default).\n
 This option is not supported on 32-bit platforms.\n
 Option type: int\n
 Applies to: compiler and linker

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_LTO: CUjit_option_enum = CUjit_option_enum(20);
}
impl CUjit_option_enum {
    /** \deprecated
 Control single-precision denormals (-ftz) support (0: false, default).
 1 : flushes denormal values to zero
 0 : preserves denormal values
 Option type: int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_FTZ: CUjit_option_enum = CUjit_option_enum(21);
}
impl CUjit_option_enum {
    /** \deprecated
 Control single-precision floating-point division and reciprocals
 (-prec-div) support (1: true, default).
 1 : Enables the IEEE round-to-nearest mode
 0 : Enables the fast approximation mode
 Option type: int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_PREC_DIV: CUjit_option_enum = CUjit_option_enum(22);
}
impl CUjit_option_enum {
    /** \deprecated
 Control single-precision floating-point square root
 (-prec-sqrt) support (1: true, default).
 1 : Enables the IEEE round-to-nearest mode
 0 : Enables the fast approximation mode
 Option type: int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_PREC_SQRT: CUjit_option_enum = CUjit_option_enum(23);
}
impl CUjit_option_enum {
    /** \deprecated
 Enable/Disable the contraction of floating-point multiplies
 and adds/subtracts into floating-point multiply-add (-fma)
 operations (1: Enable, default; 0: Disable).
 Option type: int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_FMA: CUjit_option_enum = CUjit_option_enum(24);
}
impl CUjit_option_enum {
    /** \deprecated
 Array of kernel names that should be preserved at link time while others
 can be removed.\n
 Must contain ::CU_JIT_REFERENCED_KERNEL_COUNT entries.\n
 Note that kernel names can be mangled by the compiler in which case the
 mangled name needs to be specified.\n
 Wildcard "*" can be used to represent zero or more characters instead of
 specifying the full or mangled name.\n
 It is important to note that the wildcard "*" is also added implicitly.
 For example, specifying "foo" will match "foobaz", "barfoo", "barfoobaz" and
 thus preserve all kernels with those names. This can be avoided by providing
 a more specific name like "barfoobaz".\n
 Option type: const char **\n
 Applies to: dynamic linker only

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_REFERENCED_KERNEL_NAMES: CUjit_option_enum = CUjit_option_enum(25);
}
impl CUjit_option_enum {
    /** \deprecated
 Number of entries in ::CU_JIT_REFERENCED_KERNEL_NAMES array.\n
 Option type: unsigned int\n
 Applies to: dynamic linker only

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_REFERENCED_KERNEL_COUNT: CUjit_option_enum = CUjit_option_enum(26);
}
impl CUjit_option_enum {
    /** \deprecated
 Array of variable names (__device__ and/or __constant__) that should be
 preserved at link time while others can be removed.\n
 Must contain ::CU_JIT_REFERENCED_VARIABLE_COUNT entries.\n
 Note that variable names can be mangled by the compiler in which case the
 mangled name needs to be specified.\n
 Wildcard "*" can be used to represent zero or more characters instead of
 specifying the full or mangled name.\n
 It is important to note that the wildcard "*" is also added implicitly.
 For example, specifying "foo" will match "foobaz", "barfoo", "barfoobaz" and
 thus preserve all variables with those names. This can be avoided by providing
 a more specific name like "barfoobaz".\n
 Option type: const char **\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_REFERENCED_VARIABLE_NAMES: CUjit_option_enum = CUjit_option_enum(
        27,
    );
}
impl CUjit_option_enum {
    /** \deprecated
 Number of entries in ::CU_JIT_REFERENCED_VARIABLE_NAMES array.\n
 Option type: unsigned int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_REFERENCED_VARIABLE_COUNT: CUjit_option_enum = CUjit_option_enum(
        28,
    );
}
impl CUjit_option_enum {
    /** \deprecated
 This option serves as a hint to enable the JIT compiler/linker
 to remove constant (__constant__) and device (__device__) variables
 unreferenced in device code (Disabled by default).\n
 Note that host references to constant and device variables using APIs like
 ::cuModuleGetGlobal() with this option specified may result in undefined behavior unless
 the variables are explicitly specified using ::CU_JIT_REFERENCED_VARIABLE_NAMES.\n
 Option type: int\n
 Applies to: link-time optimization specified with CU_JIT_LTO

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_OPTIMIZE_UNUSED_DEVICE_VARIABLES: CUjit_option_enum = CUjit_option_enum(
        29,
    );
}
impl CUjit_option_enum {
    /** Generate position independent code (0: false)\n
 Option type: int\n
 Applies to: compiler only*/
    pub const CU_JIT_POSITION_INDEPENDENT_CODE: CUjit_option_enum = CUjit_option_enum(
        30,
    );
}
impl CUjit_option_enum {
    /** This option hints to the JIT compiler the minimum number of CTAs from the
 kernel’s grid to be mapped to a SM. This option is ignored when used together
 with ::CU_JIT_MAX_REGISTERS or ::CU_JIT_THREADS_PER_BLOCK.
 Optimizations based on this option need ::CU_JIT_MAX_THREADS_PER_BLOCK to
 be specified as well. For kernels already using PTX directive .minnctapersm,
 this option will be ignored by default. Use ::CU_JIT_OVERRIDE_DIRECTIVE_VALUES
 to let this option take precedence over the PTX directive.
 Option type: unsigned int\n
 Applies to: compiler only*/
    pub const CU_JIT_MIN_CTA_PER_SM: CUjit_option_enum = CUjit_option_enum(31);
}
impl CUjit_option_enum {
    /** Maximum number threads in a thread block, computed as the product of
 the maximum extent specifed for each dimension of the block. This limit
 is guaranteed not to be exeeded in any invocation of the kernel. Exceeding
 the the maximum number of threads results in runtime error or kernel launch
 failure. For kernels already using PTX directive .maxntid, this option will
 be ignored by default. Use ::CU_JIT_OVERRIDE_DIRECTIVE_VALUES to let this
 option take precedence over the PTX directive.
 Option type: int\n
 Applies to: compiler only*/
    pub const CU_JIT_MAX_THREADS_PER_BLOCK: CUjit_option_enum = CUjit_option_enum(32);
}
impl CUjit_option_enum {
    /** This option lets the values specified using ::CU_JIT_MAX_REGISTERS,
 ::CU_JIT_THREADS_PER_BLOCK, ::CU_JIT_MAX_THREADS_PER_BLOCK and
 ::CU_JIT_MIN_CTA_PER_SM take precedence over any PTX directives.
 (0: Disable, default; 1: Enable)
 Option type: int\n
 Applies to: compiler only*/
    pub const CU_JIT_OVERRIDE_DIRECTIVE_VALUES: CUjit_option_enum = CUjit_option_enum(
        33,
    );
}
impl CUjit_option_enum {
    /** This option lets the values specified using ::CU_JIT_MAX_REGISTERS,
 ::CU_JIT_THREADS_PER_BLOCK, ::CU_JIT_MAX_THREADS_PER_BLOCK and
 ::CU_JIT_MIN_CTA_PER_SM take precedence over any PTX directives.
 (0: Disable, default; 1: Enable)
 Option type: int\n
 Applies to: compiler only*/
    pub const CU_JIT_NUM_OPTIONS: CUjit_option_enum = CUjit_option_enum(34);
}
#[repr(transparent)]
/// Online compiler and linker options
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUjit_option_enum(pub ::core::ffi::c_uint);
/// Online compiler and linker options
pub use self::CUjit_option_enum as CUjit_option;
impl CUjit_target_enum {
    ///< Compute device class 3.0
    pub const CU_TARGET_COMPUTE_30: CUjit_target_enum = CUjit_target_enum(30);
}
impl CUjit_target_enum {
    ///< Compute device class 3.2
    pub const CU_TARGET_COMPUTE_32: CUjit_target_enum = CUjit_target_enum(32);
}
impl CUjit_target_enum {
    ///< Compute device class 3.5
    pub const CU_TARGET_COMPUTE_35: CUjit_target_enum = CUjit_target_enum(35);
}
impl CUjit_target_enum {
    ///< Compute device class 3.7
    pub const CU_TARGET_COMPUTE_37: CUjit_target_enum = CUjit_target_enum(37);
}
impl CUjit_target_enum {
    ///< Compute device class 5.0
    pub const CU_TARGET_COMPUTE_50: CUjit_target_enum = CUjit_target_enum(50);
}
impl CUjit_target_enum {
    ///< Compute device class 5.2
    pub const CU_TARGET_COMPUTE_52: CUjit_target_enum = CUjit_target_enum(52);
}
impl CUjit_target_enum {
    ///< Compute device class 5.3
    pub const CU_TARGET_COMPUTE_53: CUjit_target_enum = CUjit_target_enum(53);
}
impl CUjit_target_enum {
    ///< Compute device class 6.0.
    pub const CU_TARGET_COMPUTE_60: CUjit_target_enum = CUjit_target_enum(60);
}
impl CUjit_target_enum {
    ///< Compute device class 6.1.
    pub const CU_TARGET_COMPUTE_61: CUjit_target_enum = CUjit_target_enum(61);
}
impl CUjit_target_enum {
    ///< Compute device class 6.2.
    pub const CU_TARGET_COMPUTE_62: CUjit_target_enum = CUjit_target_enum(62);
}
impl CUjit_target_enum {
    ///< Compute device class 7.0.
    pub const CU_TARGET_COMPUTE_70: CUjit_target_enum = CUjit_target_enum(70);
}
impl CUjit_target_enum {
    ///< Compute device class 7.2.
    pub const CU_TARGET_COMPUTE_72: CUjit_target_enum = CUjit_target_enum(72);
}
impl CUjit_target_enum {
    ///< Compute device class 7.5.
    pub const CU_TARGET_COMPUTE_75: CUjit_target_enum = CUjit_target_enum(75);
}
impl CUjit_target_enum {
    ///< Compute device class 8.0.
    pub const CU_TARGET_COMPUTE_80: CUjit_target_enum = CUjit_target_enum(80);
}
impl CUjit_target_enum {
    ///< Compute device class 8.6.
    pub const CU_TARGET_COMPUTE_86: CUjit_target_enum = CUjit_target_enum(86);
}
impl CUjit_target_enum {
    ///< Compute device class 8.7.
    pub const CU_TARGET_COMPUTE_87: CUjit_target_enum = CUjit_target_enum(87);
}
impl CUjit_target_enum {
    ///< Compute device class 8.9.
    pub const CU_TARGET_COMPUTE_89: CUjit_target_enum = CUjit_target_enum(89);
}
impl CUjit_target_enum {
    ///< Compute device class 9.0.
    pub const CU_TARGET_COMPUTE_90: CUjit_target_enum = CUjit_target_enum(90);
}
impl CUjit_target_enum {
    pub const CU_TARGET_COMPUTE_90A: CUjit_target_enum = CUjit_target_enum(65626);
}
#[repr(transparent)]
/// Online compilation targets
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUjit_target_enum(pub ::core::ffi::c_uint);
/// Online compilation targets
pub use self::CUjit_target_enum as CUjit_target;
impl CUjit_fallback_enum {
    ///< Prefer to compile ptx if exact binary match not found
    pub const CU_PREFER_PTX: CUjit_fallback_enum = CUjit_fallback_enum(0);
}
impl CUjit_fallback_enum {
    ///< Prefer to fall back to compatible binary code if exact match not found
    pub const CU_PREFER_BINARY: CUjit_fallback_enum = CUjit_fallback_enum(1);
}
#[repr(transparent)]
/// Cubin matching fallback strategies
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUjit_fallback_enum(pub ::core::ffi::c_uint);
/// Cubin matching fallback strategies
pub use self::CUjit_fallback_enum as CUjit_fallback;
impl CUjit_cacheMode_enum {
    ///< Compile with no -dlcm flag specified
    pub const CU_JIT_CACHE_OPTION_NONE: CUjit_cacheMode_enum = CUjit_cacheMode_enum(0);
}
impl CUjit_cacheMode_enum {
    ///< Compile with L1 cache disabled
    pub const CU_JIT_CACHE_OPTION_CG: CUjit_cacheMode_enum = CUjit_cacheMode_enum(1);
}
impl CUjit_cacheMode_enum {
    ///< Compile with L1 cache enabled
    pub const CU_JIT_CACHE_OPTION_CA: CUjit_cacheMode_enum = CUjit_cacheMode_enum(2);
}
#[repr(transparent)]
/// Caching modes for dlcm
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUjit_cacheMode_enum(pub ::core::ffi::c_uint);
/// Caching modes for dlcm
pub use self::CUjit_cacheMode_enum as CUjit_cacheMode;
impl CUjitInputType_enum {
    /** Compiled device-class-specific device code\n
 Applicable options: none*/
    pub const CU_JIT_INPUT_CUBIN: CUjitInputType_enum = CUjitInputType_enum(0);
}
impl CUjitInputType_enum {
    /** PTX source code\n
 Applicable options: PTX compiler options*/
    pub const CU_JIT_INPUT_PTX: CUjitInputType_enum = CUjitInputType_enum(1);
}
impl CUjitInputType_enum {
    /** Bundle of multiple cubins and/or PTX of some device code\n
 Applicable options: PTX compiler options, ::CU_JIT_FALLBACK_STRATEGY*/
    pub const CU_JIT_INPUT_FATBINARY: CUjitInputType_enum = CUjitInputType_enum(2);
}
impl CUjitInputType_enum {
    /** Host object with embedded device code\n
 Applicable options: PTX compiler options, ::CU_JIT_FALLBACK_STRATEGY*/
    pub const CU_JIT_INPUT_OBJECT: CUjitInputType_enum = CUjitInputType_enum(3);
}
impl CUjitInputType_enum {
    /** Archive of host objects with embedded device code\n
 Applicable options: PTX compiler options, ::CU_JIT_FALLBACK_STRATEGY*/
    pub const CU_JIT_INPUT_LIBRARY: CUjitInputType_enum = CUjitInputType_enum(4);
}
impl CUjitInputType_enum {
    /** \deprecated
 High-level intermediate code for link-time optimization\n
 Applicable options: NVVM compiler options, PTX compiler options

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_INPUT_NVVM: CUjitInputType_enum = CUjitInputType_enum(5);
}
impl CUjitInputType_enum {
    /** \deprecated
 High-level intermediate code for link-time optimization\n
 Applicable options: NVVM compiler options, PTX compiler options

 Only valid with LTO-IR compiled with toolkits prior to CUDA 12.0*/
    pub const CU_JIT_NUM_INPUT_TYPES: CUjitInputType_enum = CUjitInputType_enum(6);
}
#[repr(transparent)]
/// Device code formats
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUjitInputType_enum(pub ::core::ffi::c_uint);
/// Device code formats
pub use self::CUjitInputType_enum as CUjitInputType;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUlinkState_st {
    _unused: [u8; 0],
}
pub type CUlinkState = *mut CUlinkState_st;
impl CUgraphicsRegisterFlags_enum {
    pub const CU_GRAPHICS_REGISTER_FLAGS_NONE: CUgraphicsRegisterFlags_enum = CUgraphicsRegisterFlags_enum(
        0,
    );
}
impl CUgraphicsRegisterFlags_enum {
    pub const CU_GRAPHICS_REGISTER_FLAGS_READ_ONLY: CUgraphicsRegisterFlags_enum = CUgraphicsRegisterFlags_enum(
        1,
    );
}
impl CUgraphicsRegisterFlags_enum {
    pub const CU_GRAPHICS_REGISTER_FLAGS_WRITE_DISCARD: CUgraphicsRegisterFlags_enum = CUgraphicsRegisterFlags_enum(
        2,
    );
}
impl CUgraphicsRegisterFlags_enum {
    pub const CU_GRAPHICS_REGISTER_FLAGS_SURFACE_LDST: CUgraphicsRegisterFlags_enum = CUgraphicsRegisterFlags_enum(
        4,
    );
}
impl CUgraphicsRegisterFlags_enum {
    pub const CU_GRAPHICS_REGISTER_FLAGS_TEXTURE_GATHER: CUgraphicsRegisterFlags_enum = CUgraphicsRegisterFlags_enum(
        8,
    );
}
#[repr(transparent)]
/// Flags to register a graphics resource
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphicsRegisterFlags_enum(pub ::core::ffi::c_uint);
/// Flags to register a graphics resource
pub use self::CUgraphicsRegisterFlags_enum as CUgraphicsRegisterFlags;
impl CUgraphicsMapResourceFlags_enum {
    pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_NONE: CUgraphicsMapResourceFlags_enum = CUgraphicsMapResourceFlags_enum(
        0,
    );
}
impl CUgraphicsMapResourceFlags_enum {
    pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_READ_ONLY: CUgraphicsMapResourceFlags_enum = CUgraphicsMapResourceFlags_enum(
        1,
    );
}
impl CUgraphicsMapResourceFlags_enum {
    pub const CU_GRAPHICS_MAP_RESOURCE_FLAGS_WRITE_DISCARD: CUgraphicsMapResourceFlags_enum = CUgraphicsMapResourceFlags_enum(
        2,
    );
}
#[repr(transparent)]
/// Flags for mapping and unmapping interop resources
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphicsMapResourceFlags_enum(pub ::core::ffi::c_uint);
/// Flags for mapping and unmapping interop resources
pub use self::CUgraphicsMapResourceFlags_enum as CUgraphicsMapResourceFlags;
impl CUarray_cubemap_face_enum {
    ///< Positive X face of cubemap
    pub const CU_CUBEMAP_FACE_POSITIVE_X: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        0,
    );
}
impl CUarray_cubemap_face_enum {
    ///< Negative X face of cubemap
    pub const CU_CUBEMAP_FACE_NEGATIVE_X: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        1,
    );
}
impl CUarray_cubemap_face_enum {
    ///< Positive Y face of cubemap
    pub const CU_CUBEMAP_FACE_POSITIVE_Y: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        2,
    );
}
impl CUarray_cubemap_face_enum {
    ///< Negative Y face of cubemap
    pub const CU_CUBEMAP_FACE_NEGATIVE_Y: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        3,
    );
}
impl CUarray_cubemap_face_enum {
    ///< Positive Z face of cubemap
    pub const CU_CUBEMAP_FACE_POSITIVE_Z: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        4,
    );
}
impl CUarray_cubemap_face_enum {
    ///< Negative Z face of cubemap
    pub const CU_CUBEMAP_FACE_NEGATIVE_Z: CUarray_cubemap_face_enum = CUarray_cubemap_face_enum(
        5,
    );
}
#[repr(transparent)]
/// Array indices for cube faces
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUarray_cubemap_face_enum(pub ::core::ffi::c_uint);
/// Array indices for cube faces
pub use self::CUarray_cubemap_face_enum as CUarray_cubemap_face;
impl CUlimit_enum {
    ///< GPU thread stack size
    pub const CU_LIMIT_STACK_SIZE: CUlimit_enum = CUlimit_enum(0);
}
impl CUlimit_enum {
    ///< GPU printf FIFO size
    pub const CU_LIMIT_PRINTF_FIFO_SIZE: CUlimit_enum = CUlimit_enum(1);
}
impl CUlimit_enum {
    ///< GPU malloc heap size
    pub const CU_LIMIT_MALLOC_HEAP_SIZE: CUlimit_enum = CUlimit_enum(2);
}
impl CUlimit_enum {
    ///< GPU device runtime launch synchronize depth
    pub const CU_LIMIT_DEV_RUNTIME_SYNC_DEPTH: CUlimit_enum = CUlimit_enum(3);
}
impl CUlimit_enum {
    ///< GPU device runtime pending launch count
    pub const CU_LIMIT_DEV_RUNTIME_PENDING_LAUNCH_COUNT: CUlimit_enum = CUlimit_enum(4);
}
impl CUlimit_enum {
    ///< A value between 0 and 128 that indicates the maximum fetch granularity of L2 (in Bytes). This is a hint
    pub const CU_LIMIT_MAX_L2_FETCH_GRANULARITY: CUlimit_enum = CUlimit_enum(5);
}
impl CUlimit_enum {
    ///< A size in bytes for L2 persisting lines cache size
    pub const CU_LIMIT_PERSISTING_L2_CACHE_SIZE: CUlimit_enum = CUlimit_enum(6);
}
impl CUlimit_enum {
    pub const CU_LIMIT_MAX: CUlimit_enum = CUlimit_enum(7);
}
#[repr(transparent)]
/// Limits
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUlimit_enum(pub ::core::ffi::c_uint);
/// Limits
pub use self::CUlimit_enum as CUlimit;
impl CUresourcetype_enum {
    ///< Array resource
    pub const CU_RESOURCE_TYPE_ARRAY: CUresourcetype_enum = CUresourcetype_enum(0);
}
impl CUresourcetype_enum {
    ///< Mipmapped array resource
    pub const CU_RESOURCE_TYPE_MIPMAPPED_ARRAY: CUresourcetype_enum = CUresourcetype_enum(
        1,
    );
}
impl CUresourcetype_enum {
    ///< Linear resource
    pub const CU_RESOURCE_TYPE_LINEAR: CUresourcetype_enum = CUresourcetype_enum(2);
}
impl CUresourcetype_enum {
    ///< Pitch 2D resource
    pub const CU_RESOURCE_TYPE_PITCH2D: CUresourcetype_enum = CUresourcetype_enum(3);
}
#[repr(transparent)]
/// Resource types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUresourcetype_enum(pub ::core::ffi::c_uint);
/// Resource types
pub use self::CUresourcetype_enum as CUresourcetype;
/** CUDA host function
 \param userData Argument value passed to the function*/
pub type CUhostFn = ::core::option::Option<
    unsafe extern "system" fn(userData: *mut ::core::ffi::c_void),
>;
impl CUaccessProperty_enum {
    ///< Normal cache persistence.
    pub const CU_ACCESS_PROPERTY_NORMAL: CUaccessProperty_enum = CUaccessProperty_enum(
        0,
    );
}
impl CUaccessProperty_enum {
    ///< Streaming access is less likely to persit from cache.
    pub const CU_ACCESS_PROPERTY_STREAMING: CUaccessProperty_enum = CUaccessProperty_enum(
        1,
    );
}
impl CUaccessProperty_enum {
    ///< Persisting access is more likely to persist in cache.
    pub const CU_ACCESS_PROPERTY_PERSISTING: CUaccessProperty_enum = CUaccessProperty_enum(
        2,
    );
}
#[repr(transparent)]
/// Specifies performance hint with ::CUaccessPolicyWindow for hitProp and missProp members.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUaccessProperty_enum(pub ::core::ffi::c_uint);
/// Specifies performance hint with ::CUaccessPolicyWindow for hitProp and missProp members.
pub use self::CUaccessProperty_enum as CUaccessProperty;
/** Specifies an access policy for a window, a contiguous extent of memory
 beginning at base_ptr and ending at base_ptr + num_bytes.
 num_bytes is limited by CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE.
 Partition into many segments and assign segments such that:
 sum of "hit segments" / window == approx. ratio.
 sum of "miss segments" / window == approx 1-ratio.
 Segments and ratio specifications are fitted to the capabilities of
 the architecture.
 Accesses in a hit segment apply the hitProp access policy.
 Accesses in a miss segment apply the missProp access policy.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CUaccessPolicyWindow_st {
    ///< Starting address of the access policy window. CUDA driver may align it.
    pub base_ptr: *mut ::core::ffi::c_void,
    ///< Size in bytes of the window policy. CUDA driver may restrict the maximum size and alignment.
    pub num_bytes: usize,
    ///< hitRatio specifies percentage of lines assigned hitProp, rest are assigned missProp.
    pub hitRatio: f32,
    ///< ::CUaccessProperty set for hit.
    pub hitProp: CUaccessProperty,
    ///< ::CUaccessProperty set for miss. Must be either NORMAL or STREAMING
    pub missProp: CUaccessProperty,
}
/** Specifies an access policy for a window, a contiguous extent of memory
 beginning at base_ptr and ending at base_ptr + num_bytes.
 num_bytes is limited by CU_DEVICE_ATTRIBUTE_MAX_ACCESS_POLICY_WINDOW_SIZE.
 Partition into many segments and assign segments such that:
 sum of "hit segments" / window == approx. ratio.
 sum of "miss segments" / window == approx 1-ratio.
 Segments and ratio specifications are fitted to the capabilities of
 the architecture.
 Accesses in a hit segment apply the hitProp access policy.
 Accesses in a miss segment apply the missProp access policy.*/
pub type CUaccessPolicyWindow_v1 = CUaccessPolicyWindow_st;
/// Access policy window
pub type CUaccessPolicyWindow = CUaccessPolicyWindow_v1;
/// GPU kernel node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_KERNEL_NODE_PARAMS_st {
    ///< Kernel to launch
    pub func: CUfunction,
    ///< Width of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Dynamic shared-memory size per thread block in bytes
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Array of pointers to kernel parameters
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    ///< Extra options
    pub extra: *mut *mut ::core::ffi::c_void,
}
/// GPU kernel node parameters
pub type CUDA_KERNEL_NODE_PARAMS_v1 = CUDA_KERNEL_NODE_PARAMS_st;
/// GPU kernel node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_KERNEL_NODE_PARAMS_v2_st {
    ///< Kernel to launch
    pub func: CUfunction,
    ///< Width of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Dynamic shared-memory size per thread block in bytes
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Array of pointers to kernel parameters
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    ///< Extra options
    pub extra: *mut *mut ::core::ffi::c_void,
    ///< Kernel to launch, will only be referenced if func is NULL
    pub kern: CUkernel,
    ///< Context for the kernel task to run in. The value NULL will indicate the current context should be used by the api. This field is ignored if func is set.
    pub ctx: CUcontext,
}
/// GPU kernel node parameters
pub type CUDA_KERNEL_NODE_PARAMS_v2 = CUDA_KERNEL_NODE_PARAMS_v2_st;
/// GPU kernel node parameters
pub type CUDA_KERNEL_NODE_PARAMS = CUDA_KERNEL_NODE_PARAMS_v2;
/// GPU kernel node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_KERNEL_NODE_PARAMS_v3_st {
    ///< Kernel to launch
    pub func: CUfunction,
    ///< Width of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Dynamic shared-memory size per thread block in bytes
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Array of pointers to kernel parameters
    pub kernelParams: *mut *mut ::core::ffi::c_void,
    ///< Extra options
    pub extra: *mut *mut ::core::ffi::c_void,
    ///< Kernel to launch, will only be referenced if func is NULL
    pub kern: CUkernel,
    ///< Context for the kernel task to run in. The value NULL will indicate the current context should be used by the api. This field is ignored if func is set.
    pub ctx: CUcontext,
}
/// GPU kernel node parameters
pub type CUDA_KERNEL_NODE_PARAMS_v3 = CUDA_KERNEL_NODE_PARAMS_v3_st;
/// Memset node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMSET_NODE_PARAMS_st {
    ///< Destination device pointer
    pub dst: CUdeviceptr,
    ///< Pitch of destination device pointer. Unused if height is 1
    pub pitch: usize,
    ///< Value to be set
    pub value: ::core::ffi::c_uint,
    ///< Size of each element in bytes. Must be 1, 2, or 4.
    pub elementSize: ::core::ffi::c_uint,
    ///< Width of the row in elements
    pub width: usize,
    ///< Number of rows
    pub height: usize,
}
/// Memset node parameters
pub type CUDA_MEMSET_NODE_PARAMS_v1 = CUDA_MEMSET_NODE_PARAMS_st;
/// Memset node parameters
pub type CUDA_MEMSET_NODE_PARAMS = CUDA_MEMSET_NODE_PARAMS_v1;
/// Memset node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMSET_NODE_PARAMS_v2_st {
    ///< Destination device pointer
    pub dst: CUdeviceptr,
    ///< Pitch of destination device pointer. Unused if height is 1
    pub pitch: usize,
    ///< Value to be set
    pub value: ::core::ffi::c_uint,
    ///< Size of each element in bytes. Must be 1, 2, or 4.
    pub elementSize: ::core::ffi::c_uint,
    ///< Width of the row in elements
    pub width: usize,
    ///< Number of rows
    pub height: usize,
    ///< Context on which to run the node
    pub ctx: CUcontext,
}
/// Memset node parameters
pub type CUDA_MEMSET_NODE_PARAMS_v2 = CUDA_MEMSET_NODE_PARAMS_v2_st;
/// Host node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUDA_HOST_NODE_PARAMS_st {
    ///< The function to call when the node executes
    pub fn_: CUhostFn,
    ///< Argument to pass to the function
    pub userData: *mut ::core::ffi::c_void,
}
/// Host node parameters
pub type CUDA_HOST_NODE_PARAMS_v1 = CUDA_HOST_NODE_PARAMS_st;
/// Host node parameters
pub type CUDA_HOST_NODE_PARAMS = CUDA_HOST_NODE_PARAMS_v1;
/// Host node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_HOST_NODE_PARAMS_v2_st {
    ///< The function to call when the node executes
    pub fn_: CUhostFn,
    ///< Argument to pass to the function
    pub userData: *mut ::core::ffi::c_void,
}
/// Host node parameters
pub type CUDA_HOST_NODE_PARAMS_v2 = CUDA_HOST_NODE_PARAMS_v2_st;
impl CUgraphConditionalNodeType_enum {
    ///< Conditional 'if' Node. Body executed once if condition value is non-zero.
    pub const CU_GRAPH_COND_TYPE_IF: CUgraphConditionalNodeType_enum = CUgraphConditionalNodeType_enum(
        0,
    );
}
impl CUgraphConditionalNodeType_enum {
    ///< Conditional 'while' Node. Body executed repeatedly while condition value is non-zero.
    pub const CU_GRAPH_COND_TYPE_WHILE: CUgraphConditionalNodeType_enum = CUgraphConditionalNodeType_enum(
        1,
    );
}
#[repr(transparent)]
/// Conditional node types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphConditionalNodeType_enum(pub ::core::ffi::c_uint);
/// Conditional node types
pub use self::CUgraphConditionalNodeType_enum as CUgraphConditionalNodeType;
/// Conditional node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_CONDITIONAL_NODE_PARAMS {
    /**< Conditional node handle.
Handles must be created in advance of creating the node
using ::cuGraphConditionalHandleCreate.*/
    pub handle: CUgraphConditionalHandle,
    ///< Type of conditional node.
    pub type_: CUgraphConditionalNodeType,
    ///< Size of graph output array.  Must be 1.
    pub size: ::core::ffi::c_uint,
    /**< CUDA-owned array populated with conditional node child graphs during creation of the node.
Valid for the lifetime of the conditional node.
The contents of the graph(s) are subject to the following constraints:

- Allowed node types are kernel nodes, empty nodes, child graphs, memsets,
memcopies, and conditionals. This applies recursively to child graphs and conditional bodies.
- All kernels, including kernels in nested conditionals or child graphs at any level,
must belong to the same CUDA context.

These graphs may be populated using graph node creation APIs or ::cuStreamBeginCaptureToGraph.*/
    pub phGraph_out: *mut CUgraph,
    ///< Context on which to run the node.  Must match context used to create the handle and all body nodes.
    pub ctx: CUcontext,
}
impl CUgraphNodeType_enum {
    ///< GPU kernel node
    pub const CU_GRAPH_NODE_TYPE_KERNEL: CUgraphNodeType_enum = CUgraphNodeType_enum(0);
}
impl CUgraphNodeType_enum {
    ///< Memcpy node
    pub const CU_GRAPH_NODE_TYPE_MEMCPY: CUgraphNodeType_enum = CUgraphNodeType_enum(1);
}
impl CUgraphNodeType_enum {
    ///< Memset node
    pub const CU_GRAPH_NODE_TYPE_MEMSET: CUgraphNodeType_enum = CUgraphNodeType_enum(2);
}
impl CUgraphNodeType_enum {
    ///< Host (executable) node
    pub const CU_GRAPH_NODE_TYPE_HOST: CUgraphNodeType_enum = CUgraphNodeType_enum(3);
}
impl CUgraphNodeType_enum {
    ///< Node which executes an embedded graph
    pub const CU_GRAPH_NODE_TYPE_GRAPH: CUgraphNodeType_enum = CUgraphNodeType_enum(4);
}
impl CUgraphNodeType_enum {
    ///< Empty (no-op) node
    pub const CU_GRAPH_NODE_TYPE_EMPTY: CUgraphNodeType_enum = CUgraphNodeType_enum(5);
}
impl CUgraphNodeType_enum {
    ///< External event wait node
    pub const CU_GRAPH_NODE_TYPE_WAIT_EVENT: CUgraphNodeType_enum = CUgraphNodeType_enum(
        6,
    );
}
impl CUgraphNodeType_enum {
    ///< External event record node
    pub const CU_GRAPH_NODE_TYPE_EVENT_RECORD: CUgraphNodeType_enum = CUgraphNodeType_enum(
        7,
    );
}
impl CUgraphNodeType_enum {
    ///< External semaphore signal node
    pub const CU_GRAPH_NODE_TYPE_EXT_SEMAS_SIGNAL: CUgraphNodeType_enum = CUgraphNodeType_enum(
        8,
    );
}
impl CUgraphNodeType_enum {
    ///< External semaphore wait node
    pub const CU_GRAPH_NODE_TYPE_EXT_SEMAS_WAIT: CUgraphNodeType_enum = CUgraphNodeType_enum(
        9,
    );
}
impl CUgraphNodeType_enum {
    ///< Memory Allocation Node
    pub const CU_GRAPH_NODE_TYPE_MEM_ALLOC: CUgraphNodeType_enum = CUgraphNodeType_enum(
        10,
    );
}
impl CUgraphNodeType_enum {
    ///< Memory Free Node
    pub const CU_GRAPH_NODE_TYPE_MEM_FREE: CUgraphNodeType_enum = CUgraphNodeType_enum(
        11,
    );
}
impl CUgraphNodeType_enum {
    ///< Batch MemOp Node
    pub const CU_GRAPH_NODE_TYPE_BATCH_MEM_OP: CUgraphNodeType_enum = CUgraphNodeType_enum(
        12,
    );
}
impl CUgraphNodeType_enum {
    /**< Conditional Node

May be used to implement a conditional execution path or loop
inside of a graph. The graph(s) contained within the body of the conditional node
can be selectively executed or iterated upon based on the value of a conditional
variable.

Handles must be created in advance of creating the node
using ::cuGraphConditionalHandleCreate.

The following restrictions apply to graphs which contain conditional nodes:
The graph cannot be used in a child node.
Only one instantiation of the graph may exist at any point in time.
The graph cannot be cloned.

To set the control value, supply a default value when creating the handle and/or
call ::cudaGraphSetConditional from device code.*/
    pub const CU_GRAPH_NODE_TYPE_CONDITIONAL: CUgraphNodeType_enum = CUgraphNodeType_enum(
        13,
    );
}
#[repr(transparent)]
/// Graph node types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphNodeType_enum(pub ::core::ffi::c_uint);
/// Graph node types
pub use self::CUgraphNodeType_enum as CUgraphNodeType;
impl CUgraphDependencyType_enum {
    ///< This is an ordinary dependency.
    pub const CU_GRAPH_DEPENDENCY_TYPE_DEFAULT: CUgraphDependencyType_enum = CUgraphDependencyType_enum(
        0,
    );
}
impl CUgraphDependencyType_enum {
    /**< This dependency type allows the downstream node to
use \c cudaGridDependencySynchronize(). It may only be used
between kernel nodes, and must be used with either the
::CU_GRAPH_KERNEL_NODE_PORT_PROGRAMMATIC or
::CU_GRAPH_KERNEL_NODE_PORT_LAUNCH_ORDER outgoing port.*/
    pub const CU_GRAPH_DEPENDENCY_TYPE_PROGRAMMATIC: CUgraphDependencyType_enum = CUgraphDependencyType_enum(
        1,
    );
}
#[repr(transparent)]
/// Type annotations that can be applied to graph edges as part of ::CUgraphEdgeData.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphDependencyType_enum(pub ::core::ffi::c_uint);
/// Type annotations that can be applied to graph edges as part of ::CUgraphEdgeData.
pub use self::CUgraphDependencyType_enum as CUgraphDependencyType;
/** Optional annotation for edges in a CUDA graph. Note, all edges implicitly have annotations and
 default to a zero-initialized value if not specified. A zero-initialized struct indicates a
 standard full serialization of two nodes with memory visibility.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUgraphEdgeData_st {
    /**< This indicates when the dependency is triggered from the upstream
node on the edge. The meaning is specfic to the node type. A value
of 0 in all cases means full completion of the upstream node, with
memory visibility to the downstream node or portion thereof
(indicated by \c to_port).
<br>
Only kernel nodes define non-zero ports. A kernel node
can use the following output port types:
::CU_GRAPH_KERNEL_NODE_PORT_DEFAULT, ::CU_GRAPH_KERNEL_NODE_PORT_PROGRAMMATIC,
or ::CU_GRAPH_KERNEL_NODE_PORT_LAUNCH_ORDER.*/
    pub from_port: ::core::ffi::c_uchar,
    /**< This indicates what portion of the downstream node is dependent on
the upstream node or portion thereof (indicated by \c from_port). The
meaning is specific to the node type. A value of 0 in all cases means
the entirety of the downstream node is dependent on the upstream work.
<br>
Currently no node types define non-zero ports. Accordingly, this field
must be set to zero.*/
    pub to_port: ::core::ffi::c_uchar,
    /**< This should be populated with a value from ::CUgraphDependencyType. (It
is typed as char due to compiler-specific layout of bitfields.) See
::CUgraphDependencyType.*/
    pub type_: ::core::ffi::c_uchar,
    /**< These bytes are unused and must be zeroed. This ensures
compatibility if additional fields are added in the future.*/
    pub reserved: [::core::ffi::c_uchar; 5usize],
}
/** Optional annotation for edges in a CUDA graph. Note, all edges implicitly have annotations and
 default to a zero-initialized value if not specified. A zero-initialized struct indicates a
 standard full serialization of two nodes with memory visibility.*/
pub type CUgraphEdgeData = CUgraphEdgeData_st;
impl CUgraphInstantiateResult_enum {
    ///< Instantiation succeeded
    pub const CUDA_GRAPH_INSTANTIATE_SUCCESS: CUgraphInstantiateResult_enum = CUgraphInstantiateResult_enum(
        0,
    );
}
impl CUgraphInstantiateResult_enum {
    ///< Instantiation failed for an unexpected reason which is described in the return value of the function
    pub const CUDA_GRAPH_INSTANTIATE_ERROR: CUgraphInstantiateResult_enum = CUgraphInstantiateResult_enum(
        1,
    );
}
impl CUgraphInstantiateResult_enum {
    ///< Instantiation failed due to invalid structure, such as cycles
    pub const CUDA_GRAPH_INSTANTIATE_INVALID_STRUCTURE: CUgraphInstantiateResult_enum = CUgraphInstantiateResult_enum(
        2,
    );
}
impl CUgraphInstantiateResult_enum {
    ///< Instantiation for device launch failed because the graph contained an unsupported operation
    pub const CUDA_GRAPH_INSTANTIATE_NODE_OPERATION_NOT_SUPPORTED: CUgraphInstantiateResult_enum = CUgraphInstantiateResult_enum(
        3,
    );
}
impl CUgraphInstantiateResult_enum {
    ///< Instantiation for device launch failed due to the nodes belonging to different contexts
    pub const CUDA_GRAPH_INSTANTIATE_MULTIPLE_CTXS_NOT_SUPPORTED: CUgraphInstantiateResult_enum = CUgraphInstantiateResult_enum(
        4,
    );
}
#[repr(transparent)]
/// Graph instantiation results
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphInstantiateResult_enum(pub ::core::ffi::c_uint);
/// Graph instantiation results
pub use self::CUgraphInstantiateResult_enum as CUgraphInstantiateResult;
/// Graph instantiation parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_GRAPH_INSTANTIATE_PARAMS_st {
    ///< Instantiation flags
    pub flags: cuuint64_t,
    ///< Upload stream
    pub hUploadStream: CUstream,
    ///< The node which caused instantiation to fail, if any
    pub hErrNode_out: CUgraphNode,
    ///< Whether instantiation was successful.  If it failed, the reason why
    pub result_out: CUgraphInstantiateResult,
}
/// Graph instantiation parameters
pub type CUDA_GRAPH_INSTANTIATE_PARAMS = CUDA_GRAPH_INSTANTIATE_PARAMS_st;
impl CUsynchronizationPolicy_enum {
    pub const CU_SYNC_POLICY_AUTO: CUsynchronizationPolicy_enum = CUsynchronizationPolicy_enum(
        1,
    );
}
impl CUsynchronizationPolicy_enum {
    pub const CU_SYNC_POLICY_SPIN: CUsynchronizationPolicy_enum = CUsynchronizationPolicy_enum(
        2,
    );
}
impl CUsynchronizationPolicy_enum {
    pub const CU_SYNC_POLICY_YIELD: CUsynchronizationPolicy_enum = CUsynchronizationPolicy_enum(
        3,
    );
}
impl CUsynchronizationPolicy_enum {
    pub const CU_SYNC_POLICY_BLOCKING_SYNC: CUsynchronizationPolicy_enum = CUsynchronizationPolicy_enum(
        4,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUsynchronizationPolicy_enum(pub ::core::ffi::c_uint);
pub use self::CUsynchronizationPolicy_enum as CUsynchronizationPolicy;
impl CUclusterSchedulingPolicy_enum {
    ///< the default policy
    pub const CU_CLUSTER_SCHEDULING_POLICY_DEFAULT: CUclusterSchedulingPolicy_enum = CUclusterSchedulingPolicy_enum(
        0,
    );
}
impl CUclusterSchedulingPolicy_enum {
    ///< spread the blocks within a cluster to the SMs
    pub const CU_CLUSTER_SCHEDULING_POLICY_SPREAD: CUclusterSchedulingPolicy_enum = CUclusterSchedulingPolicy_enum(
        1,
    );
}
impl CUclusterSchedulingPolicy_enum {
    ///< allow the hardware to load-balance the blocks in a cluster to the SMs
    pub const CU_CLUSTER_SCHEDULING_POLICY_LOAD_BALANCING: CUclusterSchedulingPolicy_enum = CUclusterSchedulingPolicy_enum(
        2,
    );
}
#[repr(transparent)]
/// Cluster scheduling policies. These may be passed to ::cuFuncSetAttribute or ::cuKernelSetAttribute
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUclusterSchedulingPolicy_enum(pub ::core::ffi::c_uint);
/// Cluster scheduling policies. These may be passed to ::cuFuncSetAttribute or ::cuKernelSetAttribute
pub use self::CUclusterSchedulingPolicy_enum as CUclusterSchedulingPolicy;
impl CUlaunchMemSyncDomain_enum {
    ///< Launch kernels in the default domain
    pub const CU_LAUNCH_MEM_SYNC_DOMAIN_DEFAULT: CUlaunchMemSyncDomain_enum = CUlaunchMemSyncDomain_enum(
        0,
    );
}
impl CUlaunchMemSyncDomain_enum {
    ///< Launch kernels in the remote domain
    pub const CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE: CUlaunchMemSyncDomain_enum = CUlaunchMemSyncDomain_enum(
        1,
    );
}
#[repr(transparent)]
/** Memory Synchronization Domain

 A kernel can be launched in a specified memory synchronization domain that affects all memory operations issued by
 that kernel. A memory barrier issued in one domain will only order memory operations in that domain, thus eliminating
 latency increase from memory barriers ordering unrelated traffic.

 By default, kernels are launched in domain 0. Kernel launched with ::CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE will have a
 different domain ID. User may also alter the domain ID with ::CUlaunchMemSyncDomainMap for a specific stream /
 graph node / kernel launch. See ::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN, ::cuStreamSetAttribute, ::cuLaunchKernelEx,
 ::cuGraphKernelNodeSetAttribute.

 Memory operations done in kernels launched in different domains are considered system-scope distanced. In other
 words, a GPU scoped memory synchronization is not sufficient for memory order to be observed by kernels in another
 memory synchronization domain even if they are on the same GPU.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUlaunchMemSyncDomain_enum(pub ::core::ffi::c_uint);
/** Memory Synchronization Domain

 A kernel can be launched in a specified memory synchronization domain that affects all memory operations issued by
 that kernel. A memory barrier issued in one domain will only order memory operations in that domain, thus eliminating
 latency increase from memory barriers ordering unrelated traffic.

 By default, kernels are launched in domain 0. Kernel launched with ::CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE will have a
 different domain ID. User may also alter the domain ID with ::CUlaunchMemSyncDomainMap for a specific stream /
 graph node / kernel launch. See ::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN, ::cuStreamSetAttribute, ::cuLaunchKernelEx,
 ::cuGraphKernelNodeSetAttribute.

 Memory operations done in kernels launched in different domains are considered system-scope distanced. In other
 words, a GPU scoped memory synchronization is not sufficient for memory order to be observed by kernels in another
 memory synchronization domain even if they are on the same GPU.*/
pub use self::CUlaunchMemSyncDomain_enum as CUlaunchMemSyncDomain;
/** Memory Synchronization Domain map

 See ::cudaLaunchMemSyncDomain.

 By default, kernels are launched in domain 0. Kernel launched with ::CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE will have a
 different domain ID. User may also alter the domain ID with ::CUlaunchMemSyncDomainMap for a specific stream /
 graph node / kernel launch. See ::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP.

 Domain ID range is available through ::CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchMemSyncDomainMap_st {
    ///< The default domain ID to use for designated kernels
    pub default_: ::core::ffi::c_uchar,
    ///< The remote domain ID to use for designated kernels
    pub remote: ::core::ffi::c_uchar,
}
/** Memory Synchronization Domain map

 See ::cudaLaunchMemSyncDomain.

 By default, kernels are launched in domain 0. Kernel launched with ::CU_LAUNCH_MEM_SYNC_DOMAIN_REMOTE will have a
 different domain ID. User may also alter the domain ID with ::CUlaunchMemSyncDomainMap for a specific stream /
 graph node / kernel launch. See ::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP.

 Domain ID range is available through ::CU_DEVICE_ATTRIBUTE_MEM_SYNC_DOMAIN_COUNT.*/
pub type CUlaunchMemSyncDomainMap = CUlaunchMemSyncDomainMap_st;
impl CUlaunchAttributeID_enum {
    ///< Ignored entry, for convenient composition
    pub const CU_LAUNCH_ATTRIBUTE_IGNORE: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        0,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for streams, graph nodes, launches. See
::CUlaunchAttributeValue::accessPolicyWindow.*/
    pub const CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        1,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for graph nodes, launches. See
::CUlaunchAttributeValue::cooperative.*/
    pub const CU_LAUNCH_ATTRIBUTE_COOPERATIVE: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        2,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for streams. See
::CUlaunchAttributeValue::syncPolicy.*/
    pub const CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        3,
    );
}
impl CUlaunchAttributeID_enum {
    ///< Valid for graph nodes, launches. See ::CUlaunchAttributeValue::clusterDim.
    pub const CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        4,
    );
}
impl CUlaunchAttributeID_enum {
    ///< Valid for graph nodes, launches. See ::CUlaunchAttributeValue::clusterSchedulingPolicyPreference.
    pub const CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        5,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for launches. Setting
::CUlaunchAttributeValue::programmaticStreamSerializationAllowed
to non-0 signals that the kernel will use programmatic
means to resolve its stream dependency, so that the
CUDA runtime should opportunistically allow the grid's
execution to overlap with the previous kernel in the
stream, if that kernel requests the overlap. The
dependent launches can choose to wait on the
dependency using the programmatic sync
(cudaGridDependencySynchronize() or equivalent PTX
instructions).*/
    pub const CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        6,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for launches. Set
::CUlaunchAttributeValue::programmaticEvent to
record the event. Event recorded through this
launch attribute is guaranteed to only trigger
after all block in the associated kernel trigger
the event. A block can trigger the event through
PTX launchdep.release or CUDA builtin function
cudaTriggerProgrammaticLaunchCompletion(). A
trigger can also be inserted at the beginning of
each block's execution if triggerAtBlockStart is
set to non-0. The dependent launches can choose to
wait on the dependency using the programmatic sync
(cudaGridDependencySynchronize() or equivalent PTX
instructions). Note that dependents (including the
CPU thread calling cuEventSynchronize()) are not
guaranteed to observe the release precisely when
it is released.  For example, cuEventSynchronize()
may only observe the event trigger long after the
associated kernel has completed. This recording
type is primarily meant for establishing
programmatic dependency between device tasks. Note
also this type of dependency allows, but does not
guarantee, concurrent execution of tasks.
<br>
The event supplied must not be an interprocess or
interop event. The event must disable timing (i.e.
must be created with the ::CU_EVENT_DISABLE_TIMING
flag set).*/
    pub const CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        7,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for streams, graph nodes, launches. See
::CUlaunchAttributeValue::priority.*/
    pub const CU_LAUNCH_ATTRIBUTE_PRIORITY: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        8,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for streams, graph nodes, launches. See
::CUlaunchAttributeValue::memSyncDomainMap.*/
    pub const CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        9,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for streams, graph nodes, launches. See
::CUlaunchAttributeValue::memSyncDomain.*/
    pub const CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        10,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for launches. Set
::CUlaunchAttributeValue::launchCompletionEvent to record the
event.
<br>
Nominally, the event is triggered once all blocks of the kernel
have begun execution. Currently this is a best effort. If a kernel
B has a launch completion dependency on a kernel A, B may wait
until A is complete. Alternatively, blocks of B may begin before
all blocks of A have begun, for example if B can claim execution
resources unavailable to A (e.g. they run on different GPUs) or
if B is a higher priority than A.
Exercise caution if such an ordering inversion could lead
to deadlock.
<br>
A launch completion event is nominally similar to a programmatic
event with \c triggerAtBlockStart set except that it is not
visible to \c cudaGridDependencySynchronize() and can be used with
compute capability less than 9.0.
<br>
The event supplied must not be an interprocess or interop
event. The event must disable timing (i.e. must be created
with the ::CU_EVENT_DISABLE_TIMING flag set).*/
    pub const CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        12,
    );
}
impl CUlaunchAttributeID_enum {
    /**< Valid for graph nodes, launches. This attribute is graphs-only,
and passing it to a launch in a non-capturing stream will result
in an error.
<br>
::CUlaunchAttributeValue::deviceUpdatableKernelNode::deviceUpdatable can
only be set to 0 or 1. Setting the field to 1 indicates that the
corresponding kernel node should be device-updatable. On success, a handle
will be returned via
::CUlaunchAttributeValue::deviceUpdatableKernelNode::devNode which can be
passed to the various device-side update functions to update the node's
kernel parameters from within another kernel. For more information on the
types of device updates that can be made, as well as the relevant limitations
thereof, see ::cudaGraphKernelNodeUpdatesApply.
<br>
Nodes which are device-updatable have additional restrictions compared to
regular kernel nodes. Firstly, device-updatable nodes cannot be removed
from their graph via ::cuGraphDestroyNode. Additionally, once opted-in
to this functionality, a node cannot opt out, and any attempt to set the
deviceUpdatable attribute to 0 will result in an error. Device-updatable
kernel nodes also cannot have their attributes copied to/from another kernel
node via ::cuGraphKernelNodeCopyAttributes. Graphs containing one or more
device-updatable nodes also do not allow multiple instantiation, and neither
the graph nor its instantiated version can be passed to ::cuGraphExecUpdate.
<br>
If a graph contains device-updatable nodes and updates those nodes from the device
from within the graph, the graph must be uploaded with ::cuGraphUpload before it
is launched. For such a graph, if host-side executable graph updates are made to the
device-updatable nodes, the graph must be uploaded before it is launched again.*/
    pub const CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        13,
    );
}
impl CUlaunchAttributeID_enum {
    pub const CU_LAUNCH_ATTRIBUTE_MAX: CUlaunchAttributeID_enum = CUlaunchAttributeID_enum(
        14,
    );
}
#[repr(transparent)]
/// Launch attributes enum; used as id field of ::CUlaunchAttribute
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUlaunchAttributeID_enum(pub ::core::ffi::c_uint);
/// Launch attributes enum; used as id field of ::CUlaunchAttribute
pub use self::CUlaunchAttributeID_enum as CUlaunchAttributeID;
/// Launch attributes union; used as value field of ::CUlaunchAttribute
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUlaunchAttributeValue_union {
    pub pad: [::core::ffi::c_char; 64usize],
    ///< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_ACCESS_POLICY_WINDOW.
    pub accessPolicyWindow: CUaccessPolicyWindow,
    /**< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_COOPERATIVE. Nonzero indicates a cooperative
kernel (see ::cuLaunchCooperativeKernel).*/
    pub cooperative: ::core::ffi::c_int,
    /**< Value of launch attribute
::CU_LAUNCH_ATTRIBUTE_SYNCHRONIZATION_POLICY. ::CUsynchronizationPolicy for
work queued up in this stream*/
    pub syncPolicy: CUsynchronizationPolicy,
    pub clusterDim: CUlaunchAttributeValue_union__bindgen_ty_1,
    /**< Value of launch attribute
::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE. Cluster
scheduling policy preference for the kernel.*/
    pub clusterSchedulingPolicyPreference: CUclusterSchedulingPolicy,
    /**< Value of launch attribute
::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION.*/
    pub programmaticStreamSerializationAllowed: ::core::ffi::c_int,
    ///< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_EVENT.
    pub programmaticEvent: CUlaunchAttributeValue_union__bindgen_ty_2,
    ///< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_LAUNCH_COMPLETION_EVENT.
    pub launchCompletionEvent: CUlaunchAttributeValue_union__bindgen_ty_3,
    ///< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_PRIORITY. Execution priority of the kernel.
    pub priority: ::core::ffi::c_int,
    /**< Value of launch attribute
::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN_MAP. See
::CUlaunchMemSyncDomainMap.*/
    pub memSyncDomainMap: CUlaunchMemSyncDomainMap,
    /**< Value of launch attribute
::CU_LAUNCH_ATTRIBUTE_MEM_SYNC_DOMAIN. See::CUlaunchMemSyncDomain*/
    pub memSyncDomain: CUlaunchMemSyncDomain,
    ///< Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_DEVICE_UPDATABLE_KERNEL_NODE.
    pub deviceUpdatableKernelNode: CUlaunchAttributeValue_union__bindgen_ty_4,
}
/**  Value of launch attribute ::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION that
  represents the desired cluster dimensions for the kernel. Opaque type
  with the following fields:
      - \p x - The X dimension of the cluster, in blocks. Must be a divisor
               of the grid X dimension.
      - \p y - The Y dimension of the cluster, in blocks. Must be a divisor
               of the grid Y dimension.
      - \p z - The Z dimension of the cluster, in blocks. Must be a divisor
               of the grid Z dimension.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_1 {
    pub x: ::core::ffi::c_uint,
    pub y: ::core::ffi::c_uint,
    pub z: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_2 {
    ///< Event to fire when all blocks trigger it
    pub event: CUevent,
    /**< Event record flags, see ::cuEventRecordWithFlags. Does not accept
::CU_EVENT_RECORD_EXTERNAL.*/
    pub flags: ::core::ffi::c_int,
    ///< If this is set to non-0, each block launch will automatically trigger the event
    pub triggerAtBlockStart: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_3 {
    ///< Event to fire when the last block launches
    pub event: CUevent,
    ///< Event record flags, see ::cuEventRecordWithFlags. Does not accept ::CU_EVENT_RECORD_EXTERNAL.
    pub flags: ::core::ffi::c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchAttributeValue_union__bindgen_ty_4 {
    ///< Whether or not the resulting kernel node should be device-updatable.
    pub deviceUpdatable: ::core::ffi::c_int,
    ///< Returns a handle to pass to the various device-side update functions.
    pub devNode: CUgraphDeviceNode,
}
/// Launch attributes union; used as value field of ::CUlaunchAttribute
pub type CUlaunchAttributeValue = CUlaunchAttributeValue_union;
/// Launch attribute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUlaunchAttribute_st {
    ///< Attribute to set
    pub id: CUlaunchAttributeID,
    pub pad: [::core::ffi::c_char; 4usize],
    ///< Value of the attribute
    pub value: CUlaunchAttributeValue,
}
/// Launch attribute
pub type CUlaunchAttribute = CUlaunchAttribute_st;
/// CUDA extensible launch configuration
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlaunchConfig_st {
    ///< Width of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Dynamic shared-memory size per thread block in bytes
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Stream identifier
    pub hStream: CUstream,
    ///< List of attributes; nullable if ::CUlaunchConfig::numAttrs == 0
    pub attrs: *mut CUlaunchAttribute,
    ///< Number of attributes populated in ::CUlaunchConfig::attrs
    pub numAttrs: ::core::ffi::c_uint,
}
/// CUDA extensible launch configuration
pub type CUlaunchConfig = CUlaunchConfig_st;
/// Launch attributes enum; used as id field of ::CUlaunchAttribute
pub use self::CUlaunchAttributeID as CUkernelNodeAttrID;
/// Launch attributes union; used as value field of ::CUlaunchAttribute
pub type CUkernelNodeAttrValue_v1 = CUlaunchAttributeValue;
/// Launch attributes union; used as value field of ::CUlaunchAttribute
pub type CUkernelNodeAttrValue = CUkernelNodeAttrValue_v1;
impl CUstreamCaptureStatus_enum {
    ///< Stream is not capturing
    pub const CU_STREAM_CAPTURE_STATUS_NONE: CUstreamCaptureStatus_enum = CUstreamCaptureStatus_enum(
        0,
    );
}
impl CUstreamCaptureStatus_enum {
    ///< Stream is actively capturing
    pub const CU_STREAM_CAPTURE_STATUS_ACTIVE: CUstreamCaptureStatus_enum = CUstreamCaptureStatus_enum(
        1,
    );
}
impl CUstreamCaptureStatus_enum {
    /**< Stream is part of a capture sequence that
has been invalidated, but not terminated*/
    pub const CU_STREAM_CAPTURE_STATUS_INVALIDATED: CUstreamCaptureStatus_enum = CUstreamCaptureStatus_enum(
        2,
    );
}
#[repr(transparent)]
/// Possible stream capture statuses returned by ::cuStreamIsCapturing
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamCaptureStatus_enum(pub ::core::ffi::c_uint);
/// Possible stream capture statuses returned by ::cuStreamIsCapturing
pub use self::CUstreamCaptureStatus_enum as CUstreamCaptureStatus;
impl CUstreamCaptureMode_enum {
    pub const CU_STREAM_CAPTURE_MODE_GLOBAL: CUstreamCaptureMode_enum = CUstreamCaptureMode_enum(
        0,
    );
}
impl CUstreamCaptureMode_enum {
    pub const CU_STREAM_CAPTURE_MODE_THREAD_LOCAL: CUstreamCaptureMode_enum = CUstreamCaptureMode_enum(
        1,
    );
}
impl CUstreamCaptureMode_enum {
    pub const CU_STREAM_CAPTURE_MODE_RELAXED: CUstreamCaptureMode_enum = CUstreamCaptureMode_enum(
        2,
    );
}
#[repr(transparent)]
/** Possible modes for stream capture thread interactions. For more details see
 ::cuStreamBeginCapture and ::cuThreadExchangeStreamCaptureMode*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUstreamCaptureMode_enum(pub ::core::ffi::c_uint);
/// Launch attributes enum; used as id field of ::CUlaunchAttribute
pub use self::CUlaunchAttributeID as CUstreamAttrID;
/** Possible modes for stream capture thread interactions. For more details see
 ::cuStreamBeginCapture and ::cuThreadExchangeStreamCaptureMode*/
pub use self::CUstreamCaptureMode_enum as CUstreamCaptureMode;
/// Launch attributes union; used as value field of ::CUlaunchAttribute
pub type CUstreamAttrValue_v1 = CUlaunchAttributeValue;
/// Launch attributes union; used as value field of ::CUlaunchAttribute
pub type CUstreamAttrValue = CUstreamAttrValue_v1;
impl CUdriverProcAddress_flags_enum {
    ///< Default search mode for driver symbols.
    pub const CU_GET_PROC_ADDRESS_DEFAULT: CUdriverProcAddress_flags_enum = CUdriverProcAddress_flags_enum(
        0,
    );
}
impl CUdriverProcAddress_flags_enum {
    ///< Search for legacy versions of driver symbols.
    pub const CU_GET_PROC_ADDRESS_LEGACY_STREAM: CUdriverProcAddress_flags_enum = CUdriverProcAddress_flags_enum(
        1,
    );
}
impl CUdriverProcAddress_flags_enum {
    ///< Search for per-thread versions of driver symbols.
    pub const CU_GET_PROC_ADDRESS_PER_THREAD_DEFAULT_STREAM: CUdriverProcAddress_flags_enum = CUdriverProcAddress_flags_enum(
        2,
    );
}
#[repr(transparent)]
/// Flags to specify search options. For more details see ::cuGetProcAddress
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdriverProcAddress_flags_enum(pub ::core::ffi::c_uint);
/// Flags to specify search options. For more details see ::cuGetProcAddress
pub use self::CUdriverProcAddress_flags_enum as CUdriverProcAddress_flags;
impl CUdriverProcAddressQueryResult_enum {
    ///< Symbol was succesfully found
    pub const CU_GET_PROC_ADDRESS_SUCCESS: CUdriverProcAddressQueryResult_enum = CUdriverProcAddressQueryResult_enum(
        0,
    );
}
impl CUdriverProcAddressQueryResult_enum {
    ///< Symbol was not found in search
    pub const CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND: CUdriverProcAddressQueryResult_enum = CUdriverProcAddressQueryResult_enum(
        1,
    );
}
impl CUdriverProcAddressQueryResult_enum {
    ///< Symbol was found but version supplied was not sufficient
    pub const CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT: CUdriverProcAddressQueryResult_enum = CUdriverProcAddressQueryResult_enum(
        2,
    );
}
#[repr(transparent)]
/// Flags to indicate search status. For more details see ::cuGetProcAddress
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdriverProcAddressQueryResult_enum(pub ::core::ffi::c_uint);
/// Flags to indicate search status. For more details see ::cuGetProcAddress
pub use self::CUdriverProcAddressQueryResult_enum as CUdriverProcAddressQueryResult;
impl CUexecAffinityType_enum {
    ///< Create a context with limited SMs.
    pub const CU_EXEC_AFFINITY_TYPE_SM_COUNT: CUexecAffinityType_enum = CUexecAffinityType_enum(
        0,
    );
}
impl CUexecAffinityType_enum {
    pub const CU_EXEC_AFFINITY_TYPE_MAX: CUexecAffinityType_enum = CUexecAffinityType_enum(
        1,
    );
}
#[repr(transparent)]
/// Execution Affinity Types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUexecAffinityType_enum(pub ::core::ffi::c_uint);
/// Execution Affinity Types
pub use self::CUexecAffinityType_enum as CUexecAffinityType;
/// Value for ::CU_EXEC_AFFINITY_TYPE_SM_COUNT
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUexecAffinitySmCount_st {
    ///< The number of SMs the context is limited to use.
    pub val: ::core::ffi::c_uint,
}
/// Value for ::CU_EXEC_AFFINITY_TYPE_SM_COUNT
pub type CUexecAffinitySmCount_v1 = CUexecAffinitySmCount_st;
/// Value for ::CU_EXEC_AFFINITY_TYPE_SM_COUNT
pub type CUexecAffinitySmCount = CUexecAffinitySmCount_v1;
/// Execution Affinity Parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUexecAffinityParam_st {
    pub type_: CUexecAffinityType,
    pub param: CUexecAffinityParam_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUexecAffinityParam_st__bindgen_ty_1 {
    pub smCount: CUexecAffinitySmCount,
}
/// Execution Affinity Parameters
pub type CUexecAffinityParam_v1 = CUexecAffinityParam_st;
/// Execution Affinity Parameters
pub type CUexecAffinityParam = CUexecAffinityParam_v1;
impl CUlibraryOption_enum {
    pub const CU_LIBRARY_HOST_UNIVERSAL_FUNCTION_AND_DATA_TABLE: CUlibraryOption_enum = CUlibraryOption_enum(
        0,
    );
}
impl CUlibraryOption_enum {
    /** Specifes that the argument \p code passed to ::cuLibraryLoadData() will be preserved.
 Specifying this option will let the driver know that \p code can be accessed at any point
 until ::cuLibraryUnload(). The default behavior is for the driver to allocate and
 maintain its own copy of \p code. Note that this is only a memory usage optimization
 hint and the driver can choose to ignore it if required.
 Specifying this option with ::cuLibraryLoadFromFile() is invalid and
 will return ::CUDA_ERROR_INVALID_VALUE.*/
    pub const CU_LIBRARY_BINARY_IS_PRESERVED: CUlibraryOption_enum = CUlibraryOption_enum(
        1,
    );
}
impl CUlibraryOption_enum {
    /** Specifes that the argument \p code passed to ::cuLibraryLoadData() will be preserved.
 Specifying this option will let the driver know that \p code can be accessed at any point
 until ::cuLibraryUnload(). The default behavior is for the driver to allocate and
 maintain its own copy of \p code. Note that this is only a memory usage optimization
 hint and the driver can choose to ignore it if required.
 Specifying this option with ::cuLibraryLoadFromFile() is invalid and
 will return ::CUDA_ERROR_INVALID_VALUE.*/
    pub const CU_LIBRARY_NUM_OPTIONS: CUlibraryOption_enum = CUlibraryOption_enum(2);
}
#[repr(transparent)]
/// Library options to be specified with ::cuLibraryLoadData() or ::cuLibraryLoadFromFile()
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUlibraryOption_enum(pub ::core::ffi::c_uint);
/// Library options to be specified with ::cuLibraryLoadData() or ::cuLibraryLoadFromFile()
pub use self::CUlibraryOption_enum as CUlibraryOption;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUlibraryHostUniversalFunctionAndDataTable_st {
    pub functionTable: *mut ::core::ffi::c_void,
    pub functionWindowSize: usize,
    pub dataTable: *mut ::core::ffi::c_void,
    pub dataWindowSize: usize,
}
pub type CUlibraryHostUniversalFunctionAndDataTable = CUlibraryHostUniversalFunctionAndDataTable_st;
/// Error codes
#[must_use]
pub type cudaError_enum = ::core::ffi::c_uint;
impl CUdevice_P2PAttribute_enum {
    ///< A relative value indicating the performance of the link between two devices
    pub const CU_DEVICE_P2P_ATTRIBUTE_PERFORMANCE_RANK: CUdevice_P2PAttribute_enum = CUdevice_P2PAttribute_enum(
        1,
    );
}
impl CUdevice_P2PAttribute_enum {
    ///< P2P Access is enable
    pub const CU_DEVICE_P2P_ATTRIBUTE_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = CUdevice_P2PAttribute_enum(
        2,
    );
}
impl CUdevice_P2PAttribute_enum {
    ///< Atomic operation over the link supported
    pub const CU_DEVICE_P2P_ATTRIBUTE_NATIVE_ATOMIC_SUPPORTED: CUdevice_P2PAttribute_enum = CUdevice_P2PAttribute_enum(
        3,
    );
}
impl CUdevice_P2PAttribute_enum {
    ///< \deprecated use CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED instead
    pub const CU_DEVICE_P2P_ATTRIBUTE_ACCESS_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = CUdevice_P2PAttribute_enum(
        4,
    );
}
impl CUdevice_P2PAttribute_enum {
    ///< Accessing CUDA arrays over the link supported
    pub const CU_DEVICE_P2P_ATTRIBUTE_CUDA_ARRAY_ACCESS_SUPPORTED: CUdevice_P2PAttribute_enum = CUdevice_P2PAttribute_enum(
        4,
    );
}
#[repr(transparent)]
/// P2P Attributes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdevice_P2PAttribute_enum(pub ::core::ffi::c_uint);
/// P2P Attributes
pub use self::CUdevice_P2PAttribute_enum as CUdevice_P2PAttribute;
/** CUDA stream callback
 \param hStream The stream the callback was added to, as passed to ::cuStreamAddCallback.  May be NULL.
 \param status ::CUDA_SUCCESS or any persistent error on the stream.
 \param userData User parameter provided at registration.*/
pub type CUstreamCallback = ::core::option::Option<
    unsafe extern "system" fn(
        hStream: CUstream,
        status: CUresult,
        userData: *mut ::core::ffi::c_void,
    ),
>;
/** Block size to per-block dynamic shared memory mapping for a certain
 kernel \param blockSize Block size of the kernel.

 \return The dynamic shared memory needed by a block.*/
pub type CUoccupancyB2DSize = ::core::option::Option<
    unsafe extern "system" fn(blockSize: ::core::ffi::c_int) -> usize,
>;
/// 2D memory copy parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY2D_st {
    ///< Source X in bytes
    pub srcXInBytes: usize,
    ///< Source Y
    pub srcY: usize,
    ///< Source memory type (host, device, array)
    pub srcMemoryType: CUmemorytype,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device pointer
    pub srcDevice: CUdeviceptr,
    ///< Source array reference
    pub srcArray: CUarray,
    ///< Source pitch (ignored when src is array)
    pub srcPitch: usize,
    ///< Destination X in bytes
    pub dstXInBytes: usize,
    ///< Destination Y
    pub dstY: usize,
    ///< Destination memory type (host, device, array)
    pub dstMemoryType: CUmemorytype,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device pointer
    pub dstDevice: CUdeviceptr,
    ///< Destination array reference
    pub dstArray: CUarray,
    ///< Destination pitch (ignored when dst is array)
    pub dstPitch: usize,
    ///< Width of 2D memory copy in bytes
    pub WidthInBytes: usize,
    ///< Height of 2D memory copy
    pub Height: usize,
}
/// 2D memory copy parameters
pub type CUDA_MEMCPY2D_v2 = CUDA_MEMCPY2D_st;
/// 2D memory copy parameters
pub type CUDA_MEMCPY2D = CUDA_MEMCPY2D_v2;
/// 3D memory copy parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY3D_st {
    ///< Source X in bytes
    pub srcXInBytes: usize,
    ///< Source Y
    pub srcY: usize,
    ///< Source Z
    pub srcZ: usize,
    ///< Source LOD
    pub srcLOD: usize,
    ///< Source memory type (host, device, array)
    pub srcMemoryType: CUmemorytype,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device pointer
    pub srcDevice: CUdeviceptr,
    ///< Source array reference
    pub srcArray: CUarray,
    ///< Must be NULL
    pub reserved0: *mut ::core::ffi::c_void,
    ///< Source pitch (ignored when src is array)
    pub srcPitch: usize,
    ///< Source height (ignored when src is array; may be 0 if Depth==1)
    pub srcHeight: usize,
    ///< Destination X in bytes
    pub dstXInBytes: usize,
    ///< Destination Y
    pub dstY: usize,
    ///< Destination Z
    pub dstZ: usize,
    ///< Destination LOD
    pub dstLOD: usize,
    ///< Destination memory type (host, device, array)
    pub dstMemoryType: CUmemorytype,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device pointer
    pub dstDevice: CUdeviceptr,
    ///< Destination array reference
    pub dstArray: CUarray,
    ///< Must be NULL
    pub reserved1: *mut ::core::ffi::c_void,
    ///< Destination pitch (ignored when dst is array)
    pub dstPitch: usize,
    ///< Destination height (ignored when dst is array; may be 0 if Depth==1)
    pub dstHeight: usize,
    ///< Width of 3D memory copy in bytes
    pub WidthInBytes: usize,
    ///< Height of 3D memory copy
    pub Height: usize,
    ///< Depth of 3D memory copy
    pub Depth: usize,
}
/// 3D memory copy parameters
pub type CUDA_MEMCPY3D_v2 = CUDA_MEMCPY3D_st;
/// 3D memory copy parameters
pub type CUDA_MEMCPY3D = CUDA_MEMCPY3D_v2;
/// 3D memory cross-context copy parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY3D_PEER_st {
    ///< Source X in bytes
    pub srcXInBytes: usize,
    ///< Source Y
    pub srcY: usize,
    ///< Source Z
    pub srcZ: usize,
    ///< Source LOD
    pub srcLOD: usize,
    ///< Source memory type (host, device, array)
    pub srcMemoryType: CUmemorytype,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device pointer
    pub srcDevice: CUdeviceptr,
    ///< Source array reference
    pub srcArray: CUarray,
    ///< Source context (ignored with srcMemoryType is ::CU_MEMORYTYPE_ARRAY)
    pub srcContext: CUcontext,
    ///< Source pitch (ignored when src is array)
    pub srcPitch: usize,
    ///< Source height (ignored when src is array; may be 0 if Depth==1)
    pub srcHeight: usize,
    ///< Destination X in bytes
    pub dstXInBytes: usize,
    ///< Destination Y
    pub dstY: usize,
    ///< Destination Z
    pub dstZ: usize,
    ///< Destination LOD
    pub dstLOD: usize,
    ///< Destination memory type (host, device, array)
    pub dstMemoryType: CUmemorytype,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device pointer
    pub dstDevice: CUdeviceptr,
    ///< Destination array reference
    pub dstArray: CUarray,
    ///< Destination context (ignored with dstMemoryType is ::CU_MEMORYTYPE_ARRAY)
    pub dstContext: CUcontext,
    ///< Destination pitch (ignored when dst is array)
    pub dstPitch: usize,
    ///< Destination height (ignored when dst is array; may be 0 if Depth==1)
    pub dstHeight: usize,
    ///< Width of 3D memory copy in bytes
    pub WidthInBytes: usize,
    ///< Height of 3D memory copy
    pub Height: usize,
    ///< Depth of 3D memory copy
    pub Depth: usize,
}
/// 3D memory cross-context copy parameters
pub type CUDA_MEMCPY3D_PEER_v1 = CUDA_MEMCPY3D_PEER_st;
/// 3D memory cross-context copy parameters
pub type CUDA_MEMCPY3D_PEER = CUDA_MEMCPY3D_PEER_v1;
/// Memcpy node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY_NODE_PARAMS_st {
    ///< Must be zero
    pub flags: ::core::ffi::c_int,
    ///< Must be zero
    pub reserved: ::core::ffi::c_int,
    ///< Context on which to run the node
    pub copyCtx: CUcontext,
    ///< Parameters for the memory copy
    pub copyParams: CUDA_MEMCPY3D,
}
/// Memcpy node parameters
pub type CUDA_MEMCPY_NODE_PARAMS = CUDA_MEMCPY_NODE_PARAMS_st;
/// Array descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY_DESCRIPTOR_st {
    ///< Width of array
    pub Width: usize,
    ///< Height of array
    pub Height: usize,
    ///< Array format
    pub Format: CUarray_format,
    ///< Channels per array element
    pub NumChannels: ::core::ffi::c_uint,
}
/// Array descriptor
pub type CUDA_ARRAY_DESCRIPTOR_v2 = CUDA_ARRAY_DESCRIPTOR_st;
/// Array descriptor
pub type CUDA_ARRAY_DESCRIPTOR = CUDA_ARRAY_DESCRIPTOR_v2;
/// 3D array descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY3D_DESCRIPTOR_st {
    ///< Width of 3D array
    pub Width: usize,
    ///< Height of 3D array
    pub Height: usize,
    ///< Depth of 3D array
    pub Depth: usize,
    ///< Array format
    pub Format: CUarray_format,
    ///< Channels per array element
    pub NumChannels: ::core::ffi::c_uint,
    ///< Flags
    pub Flags: ::core::ffi::c_uint,
}
/// 3D array descriptor
pub type CUDA_ARRAY3D_DESCRIPTOR_v2 = CUDA_ARRAY3D_DESCRIPTOR_st;
/// 3D array descriptor
pub type CUDA_ARRAY3D_DESCRIPTOR = CUDA_ARRAY3D_DESCRIPTOR_v2;
/// CUDA array sparse properties
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY_SPARSE_PROPERTIES_st {
    pub tileExtent: CUDA_ARRAY_SPARSE_PROPERTIES_st__bindgen_ty_1,
    /// First mip level at which the mip tail begins.
    pub miptailFirstLevel: ::core::ffi::c_uint,
    /// Total size of the mip tail.
    pub miptailSize: ::core::ffi::c_ulonglong,
    /// Flags will either be zero or ::CU_ARRAY_SPARSE_PROPERTIES_SINGLE_MIPTAIL
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY_SPARSE_PROPERTIES_st__bindgen_ty_1 {
    ///< Width of sparse tile in elements
    pub width: ::core::ffi::c_uint,
    ///< Height of sparse tile in elements
    pub height: ::core::ffi::c_uint,
    ///< Depth of sparse tile in elements
    pub depth: ::core::ffi::c_uint,
}
/// CUDA array sparse properties
pub type CUDA_ARRAY_SPARSE_PROPERTIES_v1 = CUDA_ARRAY_SPARSE_PROPERTIES_st;
/// CUDA array sparse properties
pub type CUDA_ARRAY_SPARSE_PROPERTIES = CUDA_ARRAY_SPARSE_PROPERTIES_v1;
/// CUDA array memory requirements
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY_MEMORY_REQUIREMENTS_st {
    ///< Total required memory size
    pub size: usize,
    ///< alignment requirement
    pub alignment: usize,
    pub reserved: [::core::ffi::c_uint; 4usize],
}
/// CUDA array memory requirements
pub type CUDA_ARRAY_MEMORY_REQUIREMENTS_v1 = CUDA_ARRAY_MEMORY_REQUIREMENTS_st;
/// CUDA array memory requirements
pub type CUDA_ARRAY_MEMORY_REQUIREMENTS = CUDA_ARRAY_MEMORY_REQUIREMENTS_v1;
/// CUDA Resource descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_RESOURCE_DESC_st {
    ///< Resource type
    pub resType: CUresourcetype,
    pub res: CUDA_RESOURCE_DESC_st__bindgen_ty_1,
    ///< Flags (must be zero)
    pub flags: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_RESOURCE_DESC_st__bindgen_ty_1 {
    pub array: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    pub mipmap: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2,
    pub linear: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3,
    pub pitch2D: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4,
    pub reserved: CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    ///< CUDA array
    pub hArray: CUarray,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_2 {
    ///< CUDA mipmapped array
    pub hMipmappedArray: CUmipmappedArray,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_3 {
    ///< Device pointer
    pub devPtr: CUdeviceptr,
    ///< Array format
    pub format: CUarray_format,
    ///< Channels per array element
    pub numChannels: ::core::ffi::c_uint,
    ///< Size in bytes
    pub sizeInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_4 {
    ///< Device pointer
    pub devPtr: CUdeviceptr,
    ///< Array format
    pub format: CUarray_format,
    ///< Channels per array element
    pub numChannels: ::core::ffi::c_uint,
    ///< Width of the array in elements
    pub width: usize,
    ///< Height of the array in elements
    pub height: usize,
    ///< Pitch between two rows in bytes
    pub pitchInBytes: usize,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_DESC_st__bindgen_ty_1__bindgen_ty_5 {
    pub reserved: [::core::ffi::c_int; 32usize],
}
/// CUDA Resource descriptor
pub type CUDA_RESOURCE_DESC_v1 = CUDA_RESOURCE_DESC_st;
/// CUDA Resource descriptor
pub type CUDA_RESOURCE_DESC = CUDA_RESOURCE_DESC_v1;
/// Texture descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CUDA_TEXTURE_DESC_st {
    ///< Address modes
    pub addressMode: [CUaddress_mode; 3usize],
    ///< Filter mode
    pub filterMode: CUfilter_mode,
    ///< Flags
    pub flags: ::core::ffi::c_uint,
    ///< Maximum anisotropy ratio
    pub maxAnisotropy: ::core::ffi::c_uint,
    ///< Mipmap filter mode
    pub mipmapFilterMode: CUfilter_mode,
    ///< Mipmap level bias
    pub mipmapLevelBias: f32,
    ///< Mipmap minimum level clamp
    pub minMipmapLevelClamp: f32,
    ///< Mipmap maximum level clamp
    pub maxMipmapLevelClamp: f32,
    ///< Border Color
    pub borderColor: [f32; 4usize],
    pub reserved: [::core::ffi::c_int; 12usize],
}
/// Texture descriptor
pub type CUDA_TEXTURE_DESC_v1 = CUDA_TEXTURE_DESC_st;
/// Texture descriptor
pub type CUDA_TEXTURE_DESC = CUDA_TEXTURE_DESC_v1;
impl CUresourceViewFormat_enum {
    ///< No resource view format (use underlying resource format)
    pub const CU_RES_VIEW_FORMAT_NONE: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        0,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel unsigned 8-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_1X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        1,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel unsigned 8-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_2X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        2,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel unsigned 8-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_4X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        3,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel signed 8-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_1X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        4,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel signed 8-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_2X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        5,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel signed 8-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_4X8: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        6,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel unsigned 16-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_1X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        7,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel unsigned 16-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_2X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        8,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel unsigned 16-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_4X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        9,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel signed 16-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_1X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        10,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel signed 16-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_2X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        11,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel signed 16-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_4X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        12,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel unsigned 32-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_1X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        13,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel unsigned 32-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_2X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        14,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel unsigned 32-bit integers
    pub const CU_RES_VIEW_FORMAT_UINT_4X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        15,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel signed 32-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_1X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        16,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel signed 32-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_2X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        17,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel signed 32-bit integers
    pub const CU_RES_VIEW_FORMAT_SINT_4X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        18,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel 16-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_1X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        19,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel 16-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_2X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        20,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel 16-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_4X16: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        21,
    );
}
impl CUresourceViewFormat_enum {
    ///< 1 channel 32-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_1X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        22,
    );
}
impl CUresourceViewFormat_enum {
    ///< 2 channel 32-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_2X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        23,
    );
}
impl CUresourceViewFormat_enum {
    ///< 4 channel 32-bit floating point
    pub const CU_RES_VIEW_FORMAT_FLOAT_4X32: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        24,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 1
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC1: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        25,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 2
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC2: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        26,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 3
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC3: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        27,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 4 unsigned
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC4: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        28,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 4 signed
    pub const CU_RES_VIEW_FORMAT_SIGNED_BC4: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        29,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 5 unsigned
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC5: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        30,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 5 signed
    pub const CU_RES_VIEW_FORMAT_SIGNED_BC5: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        31,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 6 unsigned half-float
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC6H: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        32,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 6 signed half-float
    pub const CU_RES_VIEW_FORMAT_SIGNED_BC6H: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        33,
    );
}
impl CUresourceViewFormat_enum {
    ///< Block compressed 7
    pub const CU_RES_VIEW_FORMAT_UNSIGNED_BC7: CUresourceViewFormat_enum = CUresourceViewFormat_enum(
        34,
    );
}
#[repr(transparent)]
/// Resource view format
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUresourceViewFormat_enum(pub ::core::ffi::c_uint);
/// Resource view format
pub use self::CUresourceViewFormat_enum as CUresourceViewFormat;
/// Resource view descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_RESOURCE_VIEW_DESC_st {
    ///< Resource view format
    pub format: CUresourceViewFormat,
    ///< Width of the resource view
    pub width: usize,
    ///< Height of the resource view
    pub height: usize,
    ///< Depth of the resource view
    pub depth: usize,
    ///< First defined mipmap level
    pub firstMipmapLevel: ::core::ffi::c_uint,
    ///< Last defined mipmap level
    pub lastMipmapLevel: ::core::ffi::c_uint,
    ///< First layer index
    pub firstLayer: ::core::ffi::c_uint,
    ///< Last layer index
    pub lastLayer: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
/// Resource view descriptor
pub type CUDA_RESOURCE_VIEW_DESC_v1 = CUDA_RESOURCE_VIEW_DESC_st;
/// Resource view descriptor
pub type CUDA_RESOURCE_VIEW_DESC = CUDA_RESOURCE_VIEW_DESC_v1;
/// Tensor map descriptor. Requires compiler support for aligning to 64 bytes.
#[repr(C)]
#[repr(align(64))]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUtensorMap_st {
    pub opaque: [cuuint64_t; 16usize],
}
/// Tensor map descriptor. Requires compiler support for aligning to 64 bytes.
pub type CUtensorMap = CUtensorMap_st;
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_UINT8: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        0,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_UINT16: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        1,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_UINT32: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        2,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_INT32: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        3,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_UINT64: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        4,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_INT64: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        5,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT16: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        6,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT32: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        7,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT64: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        8,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_BFLOAT16: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        9,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_FLOAT32_FTZ: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        10,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_TFLOAT32: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        11,
    );
}
impl CUtensorMapDataType_enum {
    pub const CU_TENSOR_MAP_DATA_TYPE_TFLOAT32_FTZ: CUtensorMapDataType_enum = CUtensorMapDataType_enum(
        12,
    );
}
#[repr(transparent)]
/// Tensor map data type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUtensorMapDataType_enum(pub ::core::ffi::c_uint);
/// Tensor map data type
pub use self::CUtensorMapDataType_enum as CUtensorMapDataType;
impl CUtensorMapInterleave_enum {
    pub const CU_TENSOR_MAP_INTERLEAVE_NONE: CUtensorMapInterleave_enum = CUtensorMapInterleave_enum(
        0,
    );
}
impl CUtensorMapInterleave_enum {
    pub const CU_TENSOR_MAP_INTERLEAVE_16B: CUtensorMapInterleave_enum = CUtensorMapInterleave_enum(
        1,
    );
}
impl CUtensorMapInterleave_enum {
    pub const CU_TENSOR_MAP_INTERLEAVE_32B: CUtensorMapInterleave_enum = CUtensorMapInterleave_enum(
        2,
    );
}
#[repr(transparent)]
/// Tensor map interleave layout type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUtensorMapInterleave_enum(pub ::core::ffi::c_uint);
/// Tensor map interleave layout type
pub use self::CUtensorMapInterleave_enum as CUtensorMapInterleave;
impl CUtensorMapSwizzle_enum {
    pub const CU_TENSOR_MAP_SWIZZLE_NONE: CUtensorMapSwizzle_enum = CUtensorMapSwizzle_enum(
        0,
    );
}
impl CUtensorMapSwizzle_enum {
    pub const CU_TENSOR_MAP_SWIZZLE_32B: CUtensorMapSwizzle_enum = CUtensorMapSwizzle_enum(
        1,
    );
}
impl CUtensorMapSwizzle_enum {
    pub const CU_TENSOR_MAP_SWIZZLE_64B: CUtensorMapSwizzle_enum = CUtensorMapSwizzle_enum(
        2,
    );
}
impl CUtensorMapSwizzle_enum {
    pub const CU_TENSOR_MAP_SWIZZLE_128B: CUtensorMapSwizzle_enum = CUtensorMapSwizzle_enum(
        3,
    );
}
#[repr(transparent)]
/// Tensor map swizzling mode of shared memory banks
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUtensorMapSwizzle_enum(pub ::core::ffi::c_uint);
/// Tensor map swizzling mode of shared memory banks
pub use self::CUtensorMapSwizzle_enum as CUtensorMapSwizzle;
impl CUtensorMapL2promotion_enum {
    pub const CU_TENSOR_MAP_L2_PROMOTION_NONE: CUtensorMapL2promotion_enum = CUtensorMapL2promotion_enum(
        0,
    );
}
impl CUtensorMapL2promotion_enum {
    pub const CU_TENSOR_MAP_L2_PROMOTION_L2_64B: CUtensorMapL2promotion_enum = CUtensorMapL2promotion_enum(
        1,
    );
}
impl CUtensorMapL2promotion_enum {
    pub const CU_TENSOR_MAP_L2_PROMOTION_L2_128B: CUtensorMapL2promotion_enum = CUtensorMapL2promotion_enum(
        2,
    );
}
impl CUtensorMapL2promotion_enum {
    pub const CU_TENSOR_MAP_L2_PROMOTION_L2_256B: CUtensorMapL2promotion_enum = CUtensorMapL2promotion_enum(
        3,
    );
}
#[repr(transparent)]
/// Tensor map L2 promotion type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUtensorMapL2promotion_enum(pub ::core::ffi::c_uint);
/// Tensor map L2 promotion type
pub use self::CUtensorMapL2promotion_enum as CUtensorMapL2promotion;
impl CUtensorMapFloatOOBfill_enum {
    pub const CU_TENSOR_MAP_FLOAT_OOB_FILL_NONE: CUtensorMapFloatOOBfill_enum = CUtensorMapFloatOOBfill_enum(
        0,
    );
}
impl CUtensorMapFloatOOBfill_enum {
    pub const CU_TENSOR_MAP_FLOAT_OOB_FILL_NAN_REQUEST_ZERO_FMA: CUtensorMapFloatOOBfill_enum = CUtensorMapFloatOOBfill_enum(
        1,
    );
}
#[repr(transparent)]
/// Tensor map out-of-bounds fill type
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUtensorMapFloatOOBfill_enum(pub ::core::ffi::c_uint);
/// Tensor map out-of-bounds fill type
pub use self::CUtensorMapFloatOOBfill_enum as CUtensorMapFloatOOBfill;
/// GPU Direct v3 tokens
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_st {
    pub p2pToken: ::core::ffi::c_ulonglong,
    pub vaSpaceToken: ::core::ffi::c_uint,
}
/// GPU Direct v3 tokens
pub type CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_v1 = CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_st;
/// GPU Direct v3 tokens
pub type CUDA_POINTER_ATTRIBUTE_P2P_TOKENS = CUDA_POINTER_ATTRIBUTE_P2P_TOKENS_v1;
impl CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum {
    ///< No access, meaning the device cannot access this memory at all, thus must be staged through accessible memory in order to complete certain operations
    pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_NONE: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum(
        0,
    );
}
impl CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum {
    ///< Read-only access, meaning writes to this memory are considered invalid accesses and thus return error in that case.
    pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READ: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum(
        1,
    );
}
impl CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum {
    ///< Read-write access, the device has full read-write access to the memory
    pub const CU_POINTER_ATTRIBUTE_ACCESS_FLAG_READWRITE: CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum = CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum(
        3,
    );
}
#[repr(transparent)]
/** Access flags that specify the level of access the current context's device has
 on the memory referenced.*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum(pub ::core::ffi::c_uint);
/** Access flags that specify the level of access the current context's device has
 on the memory referenced.*/
pub use self::CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS_enum as CUDA_POINTER_ATTRIBUTE_ACCESS_FLAGS;
/// Kernel launch parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_LAUNCH_PARAMS_st {
    ///< Kernel to launch
    pub function: CUfunction,
    ///< Width of grid in blocks
    pub gridDimX: ::core::ffi::c_uint,
    ///< Height of grid in blocks
    pub gridDimY: ::core::ffi::c_uint,
    ///< Depth of grid in blocks
    pub gridDimZ: ::core::ffi::c_uint,
    ///< X dimension of each thread block
    pub blockDimX: ::core::ffi::c_uint,
    ///< Y dimension of each thread block
    pub blockDimY: ::core::ffi::c_uint,
    ///< Z dimension of each thread block
    pub blockDimZ: ::core::ffi::c_uint,
    ///< Dynamic shared-memory size per thread block in bytes
    pub sharedMemBytes: ::core::ffi::c_uint,
    ///< Stream identifier
    pub hStream: CUstream,
    ///< Array of pointers to kernel parameters
    pub kernelParams: *mut *mut ::core::ffi::c_void,
}
/// Kernel launch parameters
pub type CUDA_LAUNCH_PARAMS_v1 = CUDA_LAUNCH_PARAMS_st;
/// Kernel launch parameters
pub type CUDA_LAUNCH_PARAMS = CUDA_LAUNCH_PARAMS_v1;
impl CUexternalMemoryHandleType_enum {
    /// Handle is an opaque file descriptor
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        1,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is an opaque shared NT handle
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        2,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is an opaque, globally shared handle
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        3,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is a D3D12 heap object
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        4,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is a D3D12 committed resource
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        5,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is a shared NT handle to a D3D11 resource
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        6,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is a globally shared handle to a D3D11 resource
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        7,
    );
}
impl CUexternalMemoryHandleType_enum {
    /// Handle is an NvSciBuf object
    pub const CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF: CUexternalMemoryHandleType_enum = CUexternalMemoryHandleType_enum(
        8,
    );
}
#[repr(transparent)]
/// External memory handle types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUexternalMemoryHandleType_enum(pub ::core::ffi::c_uint);
/// External memory handle types
pub use self::CUexternalMemoryHandleType_enum as CUexternalMemoryHandleType;
/// External memory handle descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st {
    /// Type of the handle
    pub type_: CUexternalMemoryHandleType,
    pub handle: CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1,
    /// Size of the memory allocation
    pub size: ::core::ffi::c_ulonglong,
    /// Flags must either be zero or ::CUDA_EXTERNAL_MEMORY_DEDICATED
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1 {
    /** File descriptor referencing the memory object. Valid
 when type is
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD*/
    pub fd: ::core::ffi::c_int,
    pub win32: CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    /** A handle representing an NvSciBuf Object. Valid when type
 is ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF*/
    pub nvSciBufObject: *const ::core::ffi::c_void,
}
/** Win32 handle referencing the semaphore object. Valid when
 type is one of the following:
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE
 - ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT
 Exactly one of 'handle' and 'name' must be non-NULL. If
 type is one of the following:
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT
 ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_RESOURCE_KMT
 then 'name' must be NULL.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    /// Valid NT handle. Must be NULL if 'name' is non-NULL
    pub handle: *mut ::core::ffi::c_void,
    /** Name of a valid memory object.
 Must be NULL if 'handle' is non-NULL.*/
    pub name: *const ::core::ffi::c_void,
}
/// External memory handle descriptor
pub type CUDA_EXTERNAL_MEMORY_HANDLE_DESC_v1 = CUDA_EXTERNAL_MEMORY_HANDLE_DESC_st;
/// External memory handle descriptor
pub type CUDA_EXTERNAL_MEMORY_HANDLE_DESC = CUDA_EXTERNAL_MEMORY_HANDLE_DESC_v1;
/// External memory buffer descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st {
    /// Offset into the memory object where the buffer's base is
    pub offset: ::core::ffi::c_ulonglong,
    /// Size of the buffer
    pub size: ::core::ffi::c_ulonglong,
    /// Flags reserved for future use. Must be zero.
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
/// External memory buffer descriptor
pub type CUDA_EXTERNAL_MEMORY_BUFFER_DESC_v1 = CUDA_EXTERNAL_MEMORY_BUFFER_DESC_st;
/// External memory buffer descriptor
pub type CUDA_EXTERNAL_MEMORY_BUFFER_DESC = CUDA_EXTERNAL_MEMORY_BUFFER_DESC_v1;
/// External memory mipmap descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st {
    /** Offset into the memory object where the base level of the
 mipmap chain is.*/
    pub offset: ::core::ffi::c_ulonglong,
    /// Format, dimension and type of base level of the mipmap chain
    pub arrayDesc: CUDA_ARRAY3D_DESCRIPTOR,
    /// Total number of levels in the mipmap chain
    pub numLevels: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
/// External memory mipmap descriptor
pub type CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_v1 = CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_st;
/// External memory mipmap descriptor
pub type CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC = CUDA_EXTERNAL_MEMORY_MIPMAPPED_ARRAY_DESC_v1;
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is an opaque file descriptor
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        1,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is an opaque shared NT handle
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        2,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is an opaque, globally shared handle
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        3,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is a shared NT handle referencing a D3D12 fence object
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        4,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is a shared NT handle referencing a D3D11 fence object
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        5,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Opaque handle to NvSciSync Object
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        6,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is a shared NT handle referencing a D3D11 keyed mutex object
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        7,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is a globally shared handle referencing a D3D11 keyed mutex object
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        8,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is an opaque file descriptor referencing a timeline semaphore
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        9,
    );
}
impl CUexternalSemaphoreHandleType_enum {
    /// Handle is an opaque shared NT handle referencing a timeline semaphore
    pub const CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32: CUexternalSemaphoreHandleType_enum = CUexternalSemaphoreHandleType_enum(
        10,
    );
}
#[repr(transparent)]
/// External semaphore handle types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUexternalSemaphoreHandleType_enum(pub ::core::ffi::c_uint);
/// External semaphore handle types
pub use self::CUexternalSemaphoreHandleType_enum as CUexternalSemaphoreHandleType;
/// External semaphore handle descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st {
    /// Type of the handle
    pub type_: CUexternalSemaphoreHandleType,
    pub handle: CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1,
    /// Flags reserved for the future. Must be zero.
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1 {
    /** File descriptor referencing the semaphore object. Valid
 when type is one of the following:
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_FD*/
    pub fd: ::core::ffi::c_int,
    pub win32: CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1,
    /// Valid NvSciSyncObj. Must be non NULL
    pub nvSciSyncObj: *const ::core::ffi::c_void,
}
/** Win32 handle referencing the semaphore object. Valid when
 type is one of the following:
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_TIMELINE_SEMAPHORE_WIN32
 Exactly one of 'handle' and 'name' must be non-NULL. If
 type is one of the following:
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT
 - ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_KEYED_MUTEX_KMT
 then 'name' must be NULL.*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st__bindgen_ty_1__bindgen_ty_1 {
    /// Valid NT handle. Must be NULL if 'name' is non-NULL
    pub handle: *mut ::core::ffi::c_void,
    /** Name of a valid synchronization primitive.
 Must be NULL if 'handle' is non-NULL.*/
    pub name: *const ::core::ffi::c_void,
}
/// External semaphore handle descriptor
pub type CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_v1 = CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_st;
/// External semaphore handle descriptor
pub type CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC = CUDA_EXTERNAL_SEMAPHORE_HANDLE_DESC_v1;
/// External semaphore signal parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st {
    pub params: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1,
    /** Only when ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS is used to
 signal a ::CUexternalSemaphore of type
 ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC, the valid flag is
 ::CUDA_EXTERNAL_SEMAPHORE_SIGNAL_SKIP_NVSCIBUF_MEMSYNC which indicates
 that while signaling the ::CUexternalSemaphore, no memory synchronization
 operations should be performed for any external memory object imported
 as ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF.
 For all other types of ::CUexternalSemaphore, flags must be zero.*/
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1 {
    pub fence: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 12usize],
}
/// Parameters for fence objects
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    /// Value of fence to be signaled
    pub value: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    /** Pointer to NvSciSyncFence. Valid if ::CUexternalSemaphoreHandleType
 is of type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC.*/
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
/// Parameters for keyed mutex objects
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    /// Value of key to release the mutex with
    pub key: ::core::ffi::c_ulonglong,
}
/// External semaphore signal parameters
pub type CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_v1 = CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_st;
/// External semaphore signal parameters
pub type CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS = CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS_v1;
/// External semaphore wait parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st {
    pub params: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1,
    /** Only when ::CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS is used to wait on
 a ::CUexternalSemaphore of type ::CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC,
 the valid flag is ::CUDA_EXTERNAL_SEMAPHORE_WAIT_SKIP_NVSCIBUF_MEMSYNC
 which indicates that while waiting for the ::CUexternalSemaphore, no memory
 synchronization operations should be performed for any external memory
 object imported as ::CU_EXTERNAL_MEMORY_HANDLE_TYPE_NVSCIBUF.
 For all other types of ::CUexternalSemaphore, flags must be zero.*/
    pub flags: ::core::ffi::c_uint,
    pub reserved: [::core::ffi::c_uint; 16usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1 {
    pub fence: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_1,
    pub nvSciSync: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2,
    pub keyedMutex: CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_3,
    pub reserved: [::core::ffi::c_uint; 10usize],
}
/// Parameters for fence objects
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_1 {
    /// Value of fence to be waited on
    pub value: ::core::ffi::c_ulonglong,
}
/** Pointer to NvSciSyncFence. Valid if CUexternalSemaphoreHandleType
 is of type CU_EXTERNAL_SEMAPHORE_HANDLE_TYPE_NVSCISYNC.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_2 {
    pub fence: *mut ::core::ffi::c_void,
    pub reserved: ::core::ffi::c_ulonglong,
}
/// Parameters for keyed mutex objects
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st__bindgen_ty_1__bindgen_ty_3 {
    /// Value of key to acquire the mutex with
    pub key: ::core::ffi::c_ulonglong,
    /// Timeout in milliseconds to wait to acquire the mutex
    pub timeoutMs: ::core::ffi::c_uint,
}
/// External semaphore wait parameters
pub type CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_v1 = CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_st;
/// External semaphore wait parameters
pub type CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS = CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS_v1;
/// Semaphore signal node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_st {
    ///< Array of external semaphore handles.
    pub extSemArray: *mut CUexternalSemaphore,
    ///< Array of external semaphore signal parameters.
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    ///< Number of handles and parameters supplied in extSemArray and paramsArray.
    pub numExtSems: ::core::ffi::c_uint,
}
/// Semaphore signal node parameters
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v1 = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_st;
/// Semaphore signal node parameters
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v1;
/// Semaphore signal node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2_st {
    ///< Array of external semaphore handles.
    pub extSemArray: *mut CUexternalSemaphore,
    ///< Array of external semaphore signal parameters.
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_SIGNAL_PARAMS,
    ///< Number of handles and parameters supplied in extSemArray and paramsArray.
    pub numExtSems: ::core::ffi::c_uint,
}
/// Semaphore signal node parameters
pub type CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2 = CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2_st;
/// Semaphore wait node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXT_SEM_WAIT_NODE_PARAMS_st {
    ///< Array of external semaphore handles.
    pub extSemArray: *mut CUexternalSemaphore,
    ///< Array of external semaphore wait parameters.
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    ///< Number of handles and parameters supplied in extSemArray and paramsArray.
    pub numExtSems: ::core::ffi::c_uint,
}
/// Semaphore wait node parameters
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS_v1 = CUDA_EXT_SEM_WAIT_NODE_PARAMS_st;
/// Semaphore wait node parameters
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS = CUDA_EXT_SEM_WAIT_NODE_PARAMS_v1;
/// Semaphore wait node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2_st {
    ///< Array of external semaphore handles.
    pub extSemArray: *mut CUexternalSemaphore,
    ///< Array of external semaphore wait parameters.
    pub paramsArray: *const CUDA_EXTERNAL_SEMAPHORE_WAIT_PARAMS,
    ///< Number of handles and parameters supplied in extSemArray and paramsArray.
    pub numExtSems: ::core::ffi::c_uint,
}
/// Semaphore wait node parameters
pub type CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2 = CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2_st;
pub type CUmemGenericAllocationHandle_v1 = ::core::ffi::c_ulonglong;
pub type CUmemGenericAllocationHandle = CUmemGenericAllocationHandle_v1;
impl CUmemAllocationHandleType_enum {
    ///< Does not allow any export mechanism. >
    pub const CU_MEM_HANDLE_TYPE_NONE: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        0,
    );
}
impl CUmemAllocationHandleType_enum {
    ///< Allows a file descriptor to be used for exporting. Permitted only on POSIX systems. (int)
    pub const CU_MEM_HANDLE_TYPE_POSIX_FILE_DESCRIPTOR: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        1,
    );
}
impl CUmemAllocationHandleType_enum {
    ///< Allows a Win32 NT handle to be used for exporting. (HANDLE)
    pub const CU_MEM_HANDLE_TYPE_WIN32: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        2,
    );
}
impl CUmemAllocationHandleType_enum {
    ///< Allows a Win32 KMT handle to be used for exporting. (D3DKMT_HANDLE)
    pub const CU_MEM_HANDLE_TYPE_WIN32_KMT: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        4,
    );
}
impl CUmemAllocationHandleType_enum {
    ///< Allows a fabric handle to be used for exporting. (CUmemFabricHandle)
    pub const CU_MEM_HANDLE_TYPE_FABRIC: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        8,
    );
}
impl CUmemAllocationHandleType_enum {
    pub const CU_MEM_HANDLE_TYPE_MAX: CUmemAllocationHandleType_enum = CUmemAllocationHandleType_enum(
        2147483647,
    );
}
#[repr(transparent)]
/// Flags for specifying particular handle types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAllocationHandleType_enum(pub ::core::ffi::c_uint);
/// Flags for specifying particular handle types
pub use self::CUmemAllocationHandleType_enum as CUmemAllocationHandleType;
impl CUmemAccess_flags_enum {
    ///< Default, make the address range not accessible
    pub const CU_MEM_ACCESS_FLAGS_PROT_NONE: CUmemAccess_flags_enum = CUmemAccess_flags_enum(
        0,
    );
}
impl CUmemAccess_flags_enum {
    ///< Make the address range read accessible
    pub const CU_MEM_ACCESS_FLAGS_PROT_READ: CUmemAccess_flags_enum = CUmemAccess_flags_enum(
        1,
    );
}
impl CUmemAccess_flags_enum {
    ///< Make the address range read-write accessible
    pub const CU_MEM_ACCESS_FLAGS_PROT_READWRITE: CUmemAccess_flags_enum = CUmemAccess_flags_enum(
        3,
    );
}
impl CUmemAccess_flags_enum {
    pub const CU_MEM_ACCESS_FLAGS_PROT_MAX: CUmemAccess_flags_enum = CUmemAccess_flags_enum(
        2147483647,
    );
}
#[repr(transparent)]
/// Specifies the memory protection flags for mapping.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAccess_flags_enum(pub ::core::ffi::c_uint);
/// Specifies the memory protection flags for mapping.
pub use self::CUmemAccess_flags_enum as CUmemAccess_flags;
impl CUmemLocationType_enum {
    pub const CU_MEM_LOCATION_TYPE_INVALID: CUmemLocationType_enum = CUmemLocationType_enum(
        0,
    );
}
impl CUmemLocationType_enum {
    ///< Location is a device location, thus id is a device ordinal
    pub const CU_MEM_LOCATION_TYPE_DEVICE: CUmemLocationType_enum = CUmemLocationType_enum(
        1,
    );
}
impl CUmemLocationType_enum {
    ///< Location is host, id is ignored
    pub const CU_MEM_LOCATION_TYPE_HOST: CUmemLocationType_enum = CUmemLocationType_enum(
        2,
    );
}
impl CUmemLocationType_enum {
    ///< Location is a host NUMA node, thus id is a host NUMA node id
    pub const CU_MEM_LOCATION_TYPE_HOST_NUMA: CUmemLocationType_enum = CUmemLocationType_enum(
        3,
    );
}
impl CUmemLocationType_enum {
    ///< Location is a host NUMA node of the current thread, id is ignored
    pub const CU_MEM_LOCATION_TYPE_HOST_NUMA_CURRENT: CUmemLocationType_enum = CUmemLocationType_enum(
        4,
    );
}
impl CUmemLocationType_enum {
    pub const CU_MEM_LOCATION_TYPE_MAX: CUmemLocationType_enum = CUmemLocationType_enum(
        2147483647,
    );
}
#[repr(transparent)]
/// Specifies the type of location
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemLocationType_enum(pub ::core::ffi::c_uint);
/// Specifies the type of location
pub use self::CUmemLocationType_enum as CUmemLocationType;
impl CUmemAllocationType_enum {
    pub const CU_MEM_ALLOCATION_TYPE_INVALID: CUmemAllocationType_enum = CUmemAllocationType_enum(
        0,
    );
}
impl CUmemAllocationType_enum {
    /** This allocation type is 'pinned', i.e. cannot migrate from its current
 location while the application is actively using it*/
    pub const CU_MEM_ALLOCATION_TYPE_PINNED: CUmemAllocationType_enum = CUmemAllocationType_enum(
        1,
    );
}
impl CUmemAllocationType_enum {
    /** This allocation type is 'pinned', i.e. cannot migrate from its current
 location while the application is actively using it*/
    pub const CU_MEM_ALLOCATION_TYPE_MAX: CUmemAllocationType_enum = CUmemAllocationType_enum(
        2147483647,
    );
}
#[repr(transparent)]
/// Defines the allocation types available
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAllocationType_enum(pub ::core::ffi::c_uint);
/// Defines the allocation types available
pub use self::CUmemAllocationType_enum as CUmemAllocationType;
impl CUmemAllocationGranularity_flags_enum {
    ///< Minimum required granularity for allocation
    pub const CU_MEM_ALLOC_GRANULARITY_MINIMUM: CUmemAllocationGranularity_flags_enum = CUmemAllocationGranularity_flags_enum(
        0,
    );
}
impl CUmemAllocationGranularity_flags_enum {
    ///< Recommended granularity for allocation for best performance
    pub const CU_MEM_ALLOC_GRANULARITY_RECOMMENDED: CUmemAllocationGranularity_flags_enum = CUmemAllocationGranularity_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flag for requesting different optimal and required granularities for an allocation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAllocationGranularity_flags_enum(pub ::core::ffi::c_uint);
/// Flag for requesting different optimal and required granularities for an allocation.
pub use self::CUmemAllocationGranularity_flags_enum as CUmemAllocationGranularity_flags;
impl CUmemRangeHandleType_enum {
    pub const CU_MEM_RANGE_HANDLE_TYPE_DMA_BUF_FD: CUmemRangeHandleType_enum = CUmemRangeHandleType_enum(
        1,
    );
}
impl CUmemRangeHandleType_enum {
    pub const CU_MEM_RANGE_HANDLE_TYPE_MAX: CUmemRangeHandleType_enum = CUmemRangeHandleType_enum(
        2147483647,
    );
}
#[repr(transparent)]
/// Specifies the handle type for address range
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemRangeHandleType_enum(pub ::core::ffi::c_uint);
/// Specifies the handle type for address range
pub use self::CUmemRangeHandleType_enum as CUmemRangeHandleType;
impl CUarraySparseSubresourceType_enum {
    pub const CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_SPARSE_LEVEL: CUarraySparseSubresourceType_enum = CUarraySparseSubresourceType_enum(
        0,
    );
}
impl CUarraySparseSubresourceType_enum {
    pub const CU_ARRAY_SPARSE_SUBRESOURCE_TYPE_MIPTAIL: CUarraySparseSubresourceType_enum = CUarraySparseSubresourceType_enum(
        1,
    );
}
#[repr(transparent)]
/// Sparse subresource types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUarraySparseSubresourceType_enum(pub ::core::ffi::c_uint);
/// Sparse subresource types
pub use self::CUarraySparseSubresourceType_enum as CUarraySparseSubresourceType;
impl CUmemOperationType_enum {
    pub const CU_MEM_OPERATION_TYPE_MAP: CUmemOperationType_enum = CUmemOperationType_enum(
        1,
    );
}
impl CUmemOperationType_enum {
    pub const CU_MEM_OPERATION_TYPE_UNMAP: CUmemOperationType_enum = CUmemOperationType_enum(
        2,
    );
}
#[repr(transparent)]
/// Memory operation types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemOperationType_enum(pub ::core::ffi::c_uint);
/// Memory operation types
pub use self::CUmemOperationType_enum as CUmemOperationType;
impl CUmemHandleType_enum {
    pub const CU_MEM_HANDLE_TYPE_GENERIC: CUmemHandleType_enum = CUmemHandleType_enum(0);
}
#[repr(transparent)]
/// Memory handle types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemHandleType_enum(pub ::core::ffi::c_uint);
/// Memory handle types
pub use self::CUmemHandleType_enum as CUmemHandleType;
/// Specifies the CUDA array or CUDA mipmapped array memory mapping information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUarrayMapInfo_st {
    ///< Resource type
    pub resourceType: CUresourcetype,
    pub resource: CUarrayMapInfo_st__bindgen_ty_1,
    ///< Sparse subresource type
    pub subresourceType: CUarraySparseSubresourceType,
    pub subresource: CUarrayMapInfo_st__bindgen_ty_2,
    ///< Memory operation type
    pub memOperationType: CUmemOperationType,
    ///< Memory handle type
    pub memHandleType: CUmemHandleType,
    pub memHandle: CUarrayMapInfo_st__bindgen_ty_3,
    ///< Offset within the memory
    pub offset: ::core::ffi::c_ulonglong,
    ///< Device ordinal bit mask
    pub deviceBitMask: ::core::ffi::c_uint,
    ///< flags for future use, must be zero now.
    pub flags: ::core::ffi::c_uint,
    ///< Reserved for future use, must be zero now.
    pub reserved: [::core::ffi::c_uint; 2usize],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_1 {
    pub mipmap: CUmipmappedArray,
    pub array: CUarray,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_2 {
    pub sparseLevel: CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1,
    pub miptail: CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_1 {
    ///< For CUDA mipmapped arrays must a valid mipmap level. For CUDA arrays must be zero
    pub level: ::core::ffi::c_uint,
    ///< For CUDA layered arrays must be a valid layer index. Otherwise, must be zero
    pub layer: ::core::ffi::c_uint,
    ///< Starting X offset in elements
    pub offsetX: ::core::ffi::c_uint,
    ///< Starting Y offset in elements
    pub offsetY: ::core::ffi::c_uint,
    ///< Starting Z offset in elements
    pub offsetZ: ::core::ffi::c_uint,
    ///< Width in elements
    pub extentWidth: ::core::ffi::c_uint,
    ///< Height in elements
    pub extentHeight: ::core::ffi::c_uint,
    ///< Depth in elements
    pub extentDepth: ::core::ffi::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUarrayMapInfo_st__bindgen_ty_2__bindgen_ty_2 {
    ///< For CUDA layered arrays must be a valid layer index. Otherwise, must be zero
    pub layer: ::core::ffi::c_uint,
    ///< Offset within mip tail
    pub offset: ::core::ffi::c_ulonglong,
    ///< Extent in bytes
    pub size: ::core::ffi::c_ulonglong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUarrayMapInfo_st__bindgen_ty_3 {
    pub memHandle: CUmemGenericAllocationHandle,
}
/// Specifies the CUDA array or CUDA mipmapped array memory mapping information
pub type CUarrayMapInfo_v1 = CUarrayMapInfo_st;
/// Specifies the CUDA array or CUDA mipmapped array memory mapping information
pub type CUarrayMapInfo = CUarrayMapInfo_v1;
/// Specifies a memory location.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemLocation_st {
    ///< Specifies the location type, which modifies the meaning of id.
    pub type_: CUmemLocationType,
    ///< identifier for a given this location's ::CUmemLocationType.
    pub id: ::core::ffi::c_int,
}
/// Specifies a memory location.
pub type CUmemLocation_v1 = CUmemLocation_st;
/// Specifies a memory location.
pub type CUmemLocation = CUmemLocation_v1;
impl CUmemAllocationCompType_enum {
    ///< Allocating non-compressible memory
    pub const CU_MEM_ALLOCATION_COMP_NONE: CUmemAllocationCompType_enum = CUmemAllocationCompType_enum(
        0,
    );
}
impl CUmemAllocationCompType_enum {
    ///< Allocating  compressible memory
    pub const CU_MEM_ALLOCATION_COMP_GENERIC: CUmemAllocationCompType_enum = CUmemAllocationCompType_enum(
        1,
    );
}
#[repr(transparent)]
/// Specifies compression attribute for an allocation.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemAllocationCompType_enum(pub ::core::ffi::c_uint);
/// Specifies compression attribute for an allocation.
pub use self::CUmemAllocationCompType_enum as CUmemAllocationCompType;
/// Specifies the allocation properties for a allocation.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemAllocationProp_st {
    /// Allocation type
    pub type_: CUmemAllocationType,
    /// requested ::CUmemAllocationHandleType
    pub requestedHandleTypes: CUmemAllocationHandleType,
    /// Location of allocation
    pub location: CUmemLocation,
    /** Windows-specific POBJECT_ATTRIBUTES required when
 ::CU_MEM_HANDLE_TYPE_WIN32 is specified.  This object attributes structure
 includes security attributes that define
 the scope of which exported allocations may be transferred to other
 processes.  In all other cases, this field is required to be zero.*/
    pub win32HandleMetaData: *mut ::core::ffi::c_void,
    pub allocFlags: CUmemAllocationProp_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemAllocationProp_st__bindgen_ty_1 {
    /** Allocation hint for requesting compressible memory.
 On devices that support Compute Data Compression, compressible
 memory can be used to accelerate accesses to data with unstructured
 sparsity and other compressible data patterns. Applications are
 expected to query allocation property of the handle obtained with
 ::cuMemCreate using ::cuMemGetAllocationPropertiesFromHandle to
 validate if the obtained allocation is compressible or not. Note that
 compressed memory may not be mappable on all devices.*/
    pub compressionType: ::core::ffi::c_uchar,
    pub gpuDirectRDMACapable: ::core::ffi::c_uchar,
    /// Bitmask indicating intended usage for this allocation
    pub usage: ::core::ffi::c_ushort,
    pub reserved: [::core::ffi::c_uchar; 4usize],
}
/// Specifies the allocation properties for a allocation.
pub type CUmemAllocationProp_v1 = CUmemAllocationProp_st;
/// Specifies the allocation properties for a allocation.
pub type CUmemAllocationProp = CUmemAllocationProp_v1;
impl CUmulticastGranularity_flags_enum {
    ///< Minimum required granularity
    pub const CU_MULTICAST_GRANULARITY_MINIMUM: CUmulticastGranularity_flags_enum = CUmulticastGranularity_flags_enum(
        0,
    );
}
impl CUmulticastGranularity_flags_enum {
    ///< Recommended granularity for best performance
    pub const CU_MULTICAST_GRANULARITY_RECOMMENDED: CUmulticastGranularity_flags_enum = CUmulticastGranularity_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for querying different granularities for a multicast object
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmulticastGranularity_flags_enum(pub ::core::ffi::c_uint);
/// Flags for querying different granularities for a multicast object
pub use self::CUmulticastGranularity_flags_enum as CUmulticastGranularity_flags;
/// Specifies the properties for a multicast object.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmulticastObjectProp_st {
    /** The number of devices in the multicast team that will bind memory to this
 object*/
    pub numDevices: ::core::ffi::c_uint,
    /** The maximum amount of memory that can be bound to this multicast object
 per device*/
    pub size: usize,
    /** Bitmask of exportable handle types (see ::CUmemAllocationHandleType) for
 this object*/
    pub handleTypes: ::core::ffi::c_ulonglong,
    /// Flags for future use, must be zero now
    pub flags: ::core::ffi::c_ulonglong,
}
/// Specifies the properties for a multicast object.
pub type CUmulticastObjectProp_v1 = CUmulticastObjectProp_st;
/// Specifies the properties for a multicast object.
pub type CUmulticastObjectProp = CUmulticastObjectProp_v1;
/// Memory access descriptor
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemAccessDesc_st {
    ///< Location on which the request is to change it's accessibility
    pub location: CUmemLocation,
    ///< ::CUmemProt accessibility flags to set on the request
    pub flags: CUmemAccess_flags,
}
/// Memory access descriptor
pub type CUmemAccessDesc_v1 = CUmemAccessDesc_st;
/// Memory access descriptor
pub type CUmemAccessDesc = CUmemAccessDesc_v1;
impl CUgraphExecUpdateResult_enum {
    ///< The update succeeded
    pub const CU_GRAPH_EXEC_UPDATE_SUCCESS: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        0,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed for an unexpected reason which is described in the return value of the function
    pub const CU_GRAPH_EXEC_UPDATE_ERROR: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        1,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because the topology changed
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_TOPOLOGY_CHANGED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        2,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because a node type changed
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_NODE_TYPE_CHANGED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        3,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because the function of a kernel node changed (CUDA driver < 11.2)
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_FUNCTION_CHANGED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        4,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because the parameters changed in a way that is not supported
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_PARAMETERS_CHANGED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        5,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because something about the node is not supported
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_NOT_SUPPORTED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        6,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because the function of a kernel node changed in an unsupported way
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_UNSUPPORTED_FUNCTION_CHANGE: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        7,
    );
}
impl CUgraphExecUpdateResult_enum {
    ///< The update failed because the node attributes changed in a way that is not supported
    pub const CU_GRAPH_EXEC_UPDATE_ERROR_ATTRIBUTES_CHANGED: CUgraphExecUpdateResult_enum = CUgraphExecUpdateResult_enum(
        8,
    );
}
#[repr(transparent)]
/// CUDA Graph Update error types
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphExecUpdateResult_enum(pub ::core::ffi::c_uint);
/// CUDA Graph Update error types
pub use self::CUgraphExecUpdateResult_enum as CUgraphExecUpdateResult;
/// Result information returned by cuGraphExecUpdate
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUgraphExecUpdateResultInfo_st {
    /// Gives more specific detail when a cuda graph update fails.
    pub result: CUgraphExecUpdateResult,
    /** The "to node" of the error edge when the topologies do not match.
 The error node when the error is associated with a specific node.
 NULL when the error is generic.*/
    pub errorNode: CUgraphNode,
    /// The from node of error edge when the topologies do not match. Otherwise NULL.
    pub errorFromNode: CUgraphNode,
}
/// Result information returned by cuGraphExecUpdate
pub type CUgraphExecUpdateResultInfo_v1 = CUgraphExecUpdateResultInfo_st;
/// Result information returned by cuGraphExecUpdate
pub type CUgraphExecUpdateResultInfo = CUgraphExecUpdateResultInfo_v1;
impl CUmemPool_attribute_enum {
    /** (value type = int)
 Allow cuMemAllocAsync to use memory asynchronously freed
 in another streams as long as a stream ordering dependency
 of the allocating stream on the free action exists.
 Cuda events and null stream interactions can create the required
 stream ordered dependencies. (default enabled)*/
    pub const CU_MEMPOOL_ATTR_REUSE_FOLLOW_EVENT_DEPENDENCIES: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        1,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = int)
 Allow reuse of already completed frees when there is no dependency
 between the free and allocation. (default enabled)*/
    pub const CU_MEMPOOL_ATTR_REUSE_ALLOW_OPPORTUNISTIC: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        2,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = int)
 Allow cuMemAllocAsync to insert new stream dependencies
 in order to establish the stream ordering required to reuse
 a piece of memory released by cuFreeAsync (default enabled).*/
    pub const CU_MEMPOOL_ATTR_REUSE_ALLOW_INTERNAL_DEPENDENCIES: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        3,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = cuuint64_t)
 Amount of reserved memory in bytes to hold onto before trying
 to release memory back to the OS. When more than the release
 threshold bytes of memory are held by the memory pool, the
 allocator will try to release memory back to the OS on the
 next call to stream, event or context synchronize. (default 0)*/
    pub const CU_MEMPOOL_ATTR_RELEASE_THRESHOLD: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        4,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = cuuint64_t)
 Amount of backing memory currently allocated for the mempool.*/
    pub const CU_MEMPOOL_ATTR_RESERVED_MEM_CURRENT: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        5,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = cuuint64_t)
 High watermark of backing memory allocated for the mempool since the
 last time it was reset. High watermark can only be reset to zero.*/
    pub const CU_MEMPOOL_ATTR_RESERVED_MEM_HIGH: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        6,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = cuuint64_t)
 Amount of memory from the pool that is currently in use by the application.*/
    pub const CU_MEMPOOL_ATTR_USED_MEM_CURRENT: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        7,
    );
}
impl CUmemPool_attribute_enum {
    /** (value type = cuuint64_t)
 High watermark of the amount of memory from the pool that was in use by the application since
 the last time it was reset. High watermark can only be reset to zero.*/
    pub const CU_MEMPOOL_ATTR_USED_MEM_HIGH: CUmemPool_attribute_enum = CUmemPool_attribute_enum(
        8,
    );
}
#[repr(transparent)]
/// CUDA memory pool attributes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmemPool_attribute_enum(pub ::core::ffi::c_uint);
/// CUDA memory pool attributes
pub use self::CUmemPool_attribute_enum as CUmemPool_attribute;
/// Specifies the properties of allocations made from the pool.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemPoolProps_st {
    ///< Allocation type. Currently must be specified as CU_MEM_ALLOCATION_TYPE_PINNED
    pub allocType: CUmemAllocationType,
    ///< Handle types that will be supported by allocations from the pool.
    pub handleTypes: CUmemAllocationHandleType,
    ///< Location where allocations should reside.
    pub location: CUmemLocation,
    /** Windows-specific LPSECURITYATTRIBUTES required when
 ::CU_MEM_HANDLE_TYPE_WIN32 is specified.  This security attribute defines
 the scope of which exported allocations may be transferred to other
 processes.  In all other cases, this field is required to be zero.*/
    pub win32SecurityAttributes: *mut ::core::ffi::c_void,
    ///< Maximum pool size. When set to 0, defaults to a system dependent value.
    pub maxSize: usize,
    ///< reserved for future use, must be 0
    pub reserved: [::core::ffi::c_uchar; 56usize],
}
/// Specifies the properties of allocations made from the pool.
pub type CUmemPoolProps_v1 = CUmemPoolProps_st;
/// Specifies the properties of allocations made from the pool.
pub type CUmemPoolProps = CUmemPoolProps_v1;
/// Opaque data for exporting a pool allocation
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUmemPoolPtrExportData_st {
    pub reserved: [::core::ffi::c_uchar; 64usize],
}
/// Opaque data for exporting a pool allocation
pub type CUmemPoolPtrExportData_v1 = CUmemPoolPtrExportData_st;
/// Opaque data for exporting a pool allocation
pub type CUmemPoolPtrExportData = CUmemPoolPtrExportData_v1;
/// Memory allocation node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEM_ALLOC_NODE_PARAMS_v1_st {
    /** in: location where the allocation should reside (specified in ::location).
 ::handleTypes must be ::CU_MEM_HANDLE_TYPE_NONE. IPC is not supported.*/
    pub poolProps: CUmemPoolProps,
    ///< in: array of memory access descriptors. Used to describe peer GPU access
    pub accessDescs: *const CUmemAccessDesc,
    ///< in: number of memory access descriptors.  Must not exceed the number of GPUs.
    pub accessDescCount: usize,
    ///< in: size in bytes of the requested allocation
    pub bytesize: usize,
    ///< out: address of the allocation returned by CUDA
    pub dptr: CUdeviceptr,
}
/// Memory allocation node parameters
pub type CUDA_MEM_ALLOC_NODE_PARAMS_v1 = CUDA_MEM_ALLOC_NODE_PARAMS_v1_st;
/// Memory allocation node parameters
pub type CUDA_MEM_ALLOC_NODE_PARAMS = CUDA_MEM_ALLOC_NODE_PARAMS_v1;
/// Memory allocation node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEM_ALLOC_NODE_PARAMS_v2_st {
    /** in: location where the allocation should reside (specified in ::location).
 ::handleTypes must be ::CU_MEM_HANDLE_TYPE_NONE. IPC is not supported.*/
    pub poolProps: CUmemPoolProps,
    ///< in: array of memory access descriptors. Used to describe peer GPU access
    pub accessDescs: *const CUmemAccessDesc,
    ///< in: number of memory access descriptors.  Must not exceed the number of GPUs.
    pub accessDescCount: usize,
    ///< in: size in bytes of the requested allocation
    pub bytesize: usize,
    ///< out: address of the allocation returned by CUDA
    pub dptr: CUdeviceptr,
}
/// Memory allocation node parameters
pub type CUDA_MEM_ALLOC_NODE_PARAMS_v2 = CUDA_MEM_ALLOC_NODE_PARAMS_v2_st;
/// Memory free node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEM_FREE_NODE_PARAMS_st {
    ///< in: the pointer to free
    pub dptr: CUdeviceptr,
}
/// Memory free node parameters
pub type CUDA_MEM_FREE_NODE_PARAMS = CUDA_MEM_FREE_NODE_PARAMS_st;
impl CUgraphMem_attribute_enum {
    /** (value type = cuuint64_t)
 Amount of memory, in bytes, currently associated with graphs*/
    pub const CU_GRAPH_MEM_ATTR_USED_MEM_CURRENT: CUgraphMem_attribute_enum = CUgraphMem_attribute_enum(
        0,
    );
}
impl CUgraphMem_attribute_enum {
    /** (value type = cuuint64_t)
 High watermark of memory, in bytes, associated with graphs since the
 last time it was reset.  High watermark can only be reset to zero.*/
    pub const CU_GRAPH_MEM_ATTR_USED_MEM_HIGH: CUgraphMem_attribute_enum = CUgraphMem_attribute_enum(
        1,
    );
}
impl CUgraphMem_attribute_enum {
    /** (value type = cuuint64_t)
 Amount of memory, in bytes, currently allocated for use by
 the CUDA graphs asynchronous allocator.*/
    pub const CU_GRAPH_MEM_ATTR_RESERVED_MEM_CURRENT: CUgraphMem_attribute_enum = CUgraphMem_attribute_enum(
        2,
    );
}
impl CUgraphMem_attribute_enum {
    /** (value type = cuuint64_t)
 High watermark of memory, in bytes, currently allocated for use by
 the CUDA graphs asynchronous allocator.*/
    pub const CU_GRAPH_MEM_ATTR_RESERVED_MEM_HIGH: CUgraphMem_attribute_enum = CUgraphMem_attribute_enum(
        3,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphMem_attribute_enum(pub ::core::ffi::c_uint);
pub use self::CUgraphMem_attribute_enum as CUgraphMem_attribute;
/// Child graph node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_CHILD_GRAPH_NODE_PARAMS_st {
    /**< The child graph to clone into the node for node creation, or
a handle to the graph owned by the node for node query*/
    pub graph: CUgraph,
}
/// Child graph node parameters
pub type CUDA_CHILD_GRAPH_NODE_PARAMS = CUDA_CHILD_GRAPH_NODE_PARAMS_st;
/// Event record node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EVENT_RECORD_NODE_PARAMS_st {
    ///< The event to record when the node executes
    pub event: CUevent,
}
/// Event record node parameters
pub type CUDA_EVENT_RECORD_NODE_PARAMS = CUDA_EVENT_RECORD_NODE_PARAMS_st;
/// Event wait node parameters
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_EVENT_WAIT_NODE_PARAMS_st {
    ///< The event to wait on from the node
    pub event: CUevent,
}
/// Event wait node parameters
pub type CUDA_EVENT_WAIT_NODE_PARAMS = CUDA_EVENT_WAIT_NODE_PARAMS_st;
/// Graph node parameters.  See ::cuGraphAddNode.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUgraphNodeParams_st {
    ///< Type of the node
    pub type_: CUgraphNodeType,
    ///< Reserved. Must be zero.
    pub reserved0: [::core::ffi::c_int; 3usize],
    pub __bindgen_anon_1: CUgraphNodeParams_st__bindgen_ty_1,
    ///< Reserved bytes. Must be zero.
    pub reserved2: ::core::ffi::c_longlong,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUgraphNodeParams_st__bindgen_ty_1 {
    ///< Padding. Unused bytes must be zero.
    pub reserved1: [::core::ffi::c_longlong; 29usize],
    ///< Kernel node parameters.
    pub kernel: CUDA_KERNEL_NODE_PARAMS_v3,
    ///< Memcpy node parameters.
    pub memcpy: CUDA_MEMCPY_NODE_PARAMS,
    ///< Memset node parameters.
    pub memset: CUDA_MEMSET_NODE_PARAMS_v2,
    ///< Host node parameters.
    pub host: CUDA_HOST_NODE_PARAMS_v2,
    ///< Child graph node parameters.
    pub graph: CUDA_CHILD_GRAPH_NODE_PARAMS,
    ///< Event wait node parameters.
    pub eventWait: CUDA_EVENT_WAIT_NODE_PARAMS,
    ///< Event record node parameters.
    pub eventRecord: CUDA_EVENT_RECORD_NODE_PARAMS,
    ///< External semaphore signal node parameters.
    pub extSemSignal: CUDA_EXT_SEM_SIGNAL_NODE_PARAMS_v2,
    ///< External semaphore wait node parameters.
    pub extSemWait: CUDA_EXT_SEM_WAIT_NODE_PARAMS_v2,
    ///< Memory allocation node parameters.
    pub alloc: CUDA_MEM_ALLOC_NODE_PARAMS_v2,
    ///< Memory free node parameters.
    pub free: CUDA_MEM_FREE_NODE_PARAMS,
    ///< MemOp node parameters.
    pub memOp: CUDA_BATCH_MEM_OP_NODE_PARAMS_v2,
    ///< Conditional node parameters.
    pub conditional: CUDA_CONDITIONAL_NODE_PARAMS,
}
/// Graph node parameters.  See ::cuGraphAddNode.
pub type CUgraphNodeParams = CUgraphNodeParams_st;
impl CUflushGPUDirectRDMAWritesOptions_enum {
    ///< ::cuFlushGPUDirectRDMAWrites() and its CUDA Runtime API counterpart are supported on the device.
    pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_HOST: CUflushGPUDirectRDMAWritesOptions_enum = CUflushGPUDirectRDMAWritesOptions_enum(
        1,
    );
}
impl CUflushGPUDirectRDMAWritesOptions_enum {
    ///< The ::CU_STREAM_WAIT_VALUE_FLUSH flag and the ::CU_STREAM_MEM_OP_FLUSH_REMOTE_WRITES MemOp are supported on the device.
    pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_OPTION_MEMOPS: CUflushGPUDirectRDMAWritesOptions_enum = CUflushGPUDirectRDMAWritesOptions_enum(
        2,
    );
}
#[repr(transparent)]
/// Bitmasks for ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUflushGPUDirectRDMAWritesOptions_enum(pub ::core::ffi::c_uint);
/// Bitmasks for ::CU_DEVICE_ATTRIBUTE_GPU_DIRECT_RDMA_FLUSH_WRITES_OPTIONS
pub use self::CUflushGPUDirectRDMAWritesOptions_enum as CUflushGPUDirectRDMAWritesOptions;
impl CUGPUDirectRDMAWritesOrdering_enum {
    ///< The device does not natively support ordering of remote writes. ::cuFlushGPUDirectRDMAWrites() can be leveraged if supported.
    pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_NONE: CUGPUDirectRDMAWritesOrdering_enum = CUGPUDirectRDMAWritesOrdering_enum(
        0,
    );
}
impl CUGPUDirectRDMAWritesOrdering_enum {
    ///< Natively, the device can consistently consume remote writes, although other CUDA devices may not.
    pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_OWNER: CUGPUDirectRDMAWritesOrdering_enum = CUGPUDirectRDMAWritesOrdering_enum(
        100,
    );
}
impl CUGPUDirectRDMAWritesOrdering_enum {
    ///< Any CUDA device in the system can consistently consume remote writes to this device.
    pub const CU_GPU_DIRECT_RDMA_WRITES_ORDERING_ALL_DEVICES: CUGPUDirectRDMAWritesOrdering_enum = CUGPUDirectRDMAWritesOrdering_enum(
        200,
    );
}
#[repr(transparent)]
/// Platform native ordering for GPUDirect RDMA writes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUGPUDirectRDMAWritesOrdering_enum(pub ::core::ffi::c_uint);
/// Platform native ordering for GPUDirect RDMA writes
pub use self::CUGPUDirectRDMAWritesOrdering_enum as CUGPUDirectRDMAWritesOrdering;
impl CUflushGPUDirectRDMAWritesScope_enum {
    ///< Blocks until remote writes are visible to the CUDA device context owning the data.
    pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_OWNER: CUflushGPUDirectRDMAWritesScope_enum = CUflushGPUDirectRDMAWritesScope_enum(
        100,
    );
}
impl CUflushGPUDirectRDMAWritesScope_enum {
    ///< Blocks until remote writes are visible to all CUDA device contexts.
    pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TO_ALL_DEVICES: CUflushGPUDirectRDMAWritesScope_enum = CUflushGPUDirectRDMAWritesScope_enum(
        200,
    );
}
#[repr(transparent)]
/// The scopes for ::cuFlushGPUDirectRDMAWrites
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUflushGPUDirectRDMAWritesScope_enum(pub ::core::ffi::c_uint);
/// The scopes for ::cuFlushGPUDirectRDMAWrites
pub use self::CUflushGPUDirectRDMAWritesScope_enum as CUflushGPUDirectRDMAWritesScope;
impl CUflushGPUDirectRDMAWritesTarget_enum {
    ///< Sets the target for ::cuFlushGPUDirectRDMAWrites() to the currently active CUDA device context.
    pub const CU_FLUSH_GPU_DIRECT_RDMA_WRITES_TARGET_CURRENT_CTX: CUflushGPUDirectRDMAWritesTarget_enum = CUflushGPUDirectRDMAWritesTarget_enum(
        0,
    );
}
#[repr(transparent)]
/// The targets for ::cuFlushGPUDirectRDMAWrites
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUflushGPUDirectRDMAWritesTarget_enum(pub ::core::ffi::c_uint);
/// The targets for ::cuFlushGPUDirectRDMAWrites
pub use self::CUflushGPUDirectRDMAWritesTarget_enum as CUflushGPUDirectRDMAWritesTarget;
impl CUgraphDebugDot_flags_enum {
    ///< Output all debug data as if every debug flag is enabled
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_VERBOSE: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        1,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Use CUDA Runtime structures for output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_RUNTIME_TYPES: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        2,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_KERNEL_NODE_PARAMS values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        4,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_MEMCPY3D values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEMCPY_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        8,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_MEMSET_NODE_PARAMS values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEMSET_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        16,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_HOST_NODE_PARAMS values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_HOST_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        32,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUevent handle from record and wait nodes to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_EVENT_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        64,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_EXT_SEM_SIGNAL_NODE_PARAMS values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_SIGNAL_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        128,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUDA_EXT_SEM_WAIT_NODE_PARAMS values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXT_SEMAS_WAIT_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        256,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds CUkernelNodeAttrValue values to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_KERNEL_NODE_ATTRIBUTES: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        512,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds node handles and every kernel function handle to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_HANDLES: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        1024,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds memory alloc node parameters to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEM_ALLOC_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        2048,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds memory free node parameters to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_MEM_FREE_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        4096,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds batch mem op node parameters to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_BATCH_MEM_OP_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        8192,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds edge numbering information
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_EXTRA_TOPO_INFO: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        16384,
    );
}
impl CUgraphDebugDot_flags_enum {
    ///< Adds conditional node parameters to output
    pub const CU_GRAPH_DEBUG_DOT_FLAGS_CONDITIONAL_NODE_PARAMS: CUgraphDebugDot_flags_enum = CUgraphDebugDot_flags_enum(
        32768,
    );
}
#[repr(transparent)]
/// The additional write options for ::cuGraphDebugDotPrint
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphDebugDot_flags_enum(pub ::core::ffi::c_uint);
/// The additional write options for ::cuGraphDebugDotPrint
pub use self::CUgraphDebugDot_flags_enum as CUgraphDebugDot_flags;
impl CUuserObject_flags_enum {
    ///< Indicates the destructor execution is not synchronized by any CUDA handle.
    pub const CU_USER_OBJECT_NO_DESTRUCTOR_SYNC: CUuserObject_flags_enum = CUuserObject_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for user objects for graphs
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUuserObject_flags_enum(pub ::core::ffi::c_uint);
/// Flags for user objects for graphs
pub use self::CUuserObject_flags_enum as CUuserObject_flags;
impl CUuserObjectRetain_flags_enum {
    ///< Transfer references from the caller rather than creating new references.
    pub const CU_GRAPH_USER_OBJECT_MOVE: CUuserObjectRetain_flags_enum = CUuserObjectRetain_flags_enum(
        1,
    );
}
#[repr(transparent)]
/// Flags for retaining user object references for graphs
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUuserObjectRetain_flags_enum(pub ::core::ffi::c_uint);
/// Flags for retaining user object references for graphs
pub use self::CUuserObjectRetain_flags_enum as CUuserObjectRetain_flags;
impl CUgraphInstantiate_flags_enum {
    ///< Automatically free memory allocated in a graph before relaunching.
    pub const CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH: CUgraphInstantiate_flags_enum = CUgraphInstantiate_flags_enum(
        1,
    );
}
impl CUgraphInstantiate_flags_enum {
    /**< Automatically upload the graph after instantiation. Only supported by
::cuGraphInstantiateWithParams.  The upload will be performed using the
stream provided in \p instantiateParams.*/
    pub const CUDA_GRAPH_INSTANTIATE_FLAG_UPLOAD: CUgraphInstantiate_flags_enum = CUgraphInstantiate_flags_enum(
        2,
    );
}
impl CUgraphInstantiate_flags_enum {
    /**< Instantiate the graph to be launchable from the device. This flag can only
be used on platforms which support unified addressing. This flag cannot be
used in conjunction with CUDA_GRAPH_INSTANTIATE_FLAG_AUTO_FREE_ON_LAUNCH.*/
    pub const CUDA_GRAPH_INSTANTIATE_FLAG_DEVICE_LAUNCH: CUgraphInstantiate_flags_enum = CUgraphInstantiate_flags_enum(
        4,
    );
}
impl CUgraphInstantiate_flags_enum {
    /**< Run the graph using the per-node priority attributes rather than the
priority of the stream it is launched into.*/
    pub const CUDA_GRAPH_INSTANTIATE_FLAG_USE_NODE_PRIORITY: CUgraphInstantiate_flags_enum = CUgraphInstantiate_flags_enum(
        8,
    );
}
#[repr(transparent)]
/// Flags for instantiating a graph
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgraphInstantiate_flags_enum(pub ::core::ffi::c_uint);
/// Flags for instantiating a graph
pub use self::CUgraphInstantiate_flags_enum as CUgraphInstantiate_flags;
impl CUdeviceNumaConfig_enum {
    ///< The GPU is not a NUMA node
    pub const CU_DEVICE_NUMA_CONFIG_NONE: CUdeviceNumaConfig_enum = CUdeviceNumaConfig_enum(
        0,
    );
}
impl CUdeviceNumaConfig_enum {
    ///< The GPU is a NUMA node, CU_DEVICE_ATTRIBUTE_NUMA_ID contains its NUMA ID
    pub const CU_DEVICE_NUMA_CONFIG_NUMA_NODE: CUdeviceNumaConfig_enum = CUdeviceNumaConfig_enum(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdeviceNumaConfig_enum(pub ::core::ffi::c_uint);
pub use self::CUdeviceNumaConfig_enum as CUdeviceNumaConfig;
impl CUmoduleLoadingMode_enum {
    ///< Lazy Kernel Loading is not enabled
    pub const CU_MODULE_EAGER_LOADING: CUmoduleLoadingMode_enum = CUmoduleLoadingMode_enum(
        1,
    );
}
impl CUmoduleLoadingMode_enum {
    ///< Lazy Kernel Loading is enabled
    pub const CU_MODULE_LAZY_LOADING: CUmoduleLoadingMode_enum = CUmoduleLoadingMode_enum(
        2,
    );
}
#[repr(transparent)]
/// CUDA Lazy Loading status
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUmoduleLoadingMode_enum(pub ::core::ffi::c_uint);
/// CUDA Lazy Loading status
pub use self::CUmoduleLoadingMode_enum as CUmoduleLoadingMode;
impl CUfunctionLoadingState_enum {
    pub const CU_FUNCTION_LOADING_STATE_UNLOADED: CUfunctionLoadingState_enum = CUfunctionLoadingState_enum(
        0,
    );
}
impl CUfunctionLoadingState_enum {
    pub const CU_FUNCTION_LOADING_STATE_LOADED: CUfunctionLoadingState_enum = CUfunctionLoadingState_enum(
        1,
    );
}
impl CUfunctionLoadingState_enum {
    pub const CU_FUNCTION_LOADING_STATE_MAX: CUfunctionLoadingState_enum = CUfunctionLoadingState_enum(
        2,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUfunctionLoadingState_enum(pub ::core::ffi::c_uint);
pub use self::CUfunctionLoadingState_enum as CUfunctionLoadingState;
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_ENABLE_ON_EXCEPTION: CUcoredumpSettings_enum = CUcoredumpSettings_enum(
        1,
    );
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_TRIGGER_HOST: CUcoredumpSettings_enum = CUcoredumpSettings_enum(
        2,
    );
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_LIGHTWEIGHT: CUcoredumpSettings_enum = CUcoredumpSettings_enum(
        3,
    );
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_ENABLE_USER_TRIGGER: CUcoredumpSettings_enum = CUcoredumpSettings_enum(
        4,
    );
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_FILE: CUcoredumpSettings_enum = CUcoredumpSettings_enum(5);
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_PIPE: CUcoredumpSettings_enum = CUcoredumpSettings_enum(6);
}
impl CUcoredumpSettings_enum {
    pub const CU_COREDUMP_MAX: CUcoredumpSettings_enum = CUcoredumpSettings_enum(7);
}
#[repr(transparent)]
/// Flags for choosing a coredump attribute to get/set
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUcoredumpSettings_enum(pub ::core::ffi::c_uint);
/// Flags for choosing a coredump attribute to get/set
pub use self::CUcoredumpSettings_enum as CUcoredumpSettings;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUgreenCtx_st {
    _unused: [u8; 0],
}
/** \typedef typedef struct CUgreenCtx_st* CUgreenCtx
 A green context handle. This handle can be used safely from only one CPU thread at a time.
 Created via ::cuGreenCtxCreate*/
pub type CUgreenCtx = *mut CUgreenCtx_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUdevResourceDesc_st {
    _unused: [u8; 0],
}
/** \typedef struct CUdevResourceDesc_st* CUdevResourceDesc;
 An opaque descriptor handle. The descriptor encapsulates multiple created and configured resources.
 Created via ::cuDevResourceGenerateDesc*/
pub type CUdevResourceDesc = *mut CUdevResourceDesc_st;
impl CUgreenCtxCreate_flags {
    ///< Required. Creates a default stream to use inside the green context
    pub const CU_GREEN_CTX_DEFAULT_STREAM: CUgreenCtxCreate_flags = CUgreenCtxCreate_flags(
        1,
    );
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUgreenCtxCreate_flags(pub ::core::ffi::c_uint);
impl CUdevResourceType {
    pub const CU_DEV_RESOURCE_TYPE_INVALID: CUdevResourceType = CUdevResourceType(0);
}
impl CUdevResourceType {
    ///< Streaming multiprocessors related information
    pub const CU_DEV_RESOURCE_TYPE_SM: CUdevResourceType = CUdevResourceType(1);
}
impl CUdevResourceType {
    pub const CU_DEV_RESOURCE_TYPE_MAX: CUdevResourceType = CUdevResourceType(2);
}
#[repr(transparent)]
/** \typedef enum CUdevResourceType
 Type of resource*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUdevResourceType(pub ::core::ffi::c_uint);
/** \struct CUdevSmResource
 Data for SM-related resources*/
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUdevSmResource_st {
    ///< The amount of streaming multiprocessors available in this resource. This is an output parameter only, do not write to this field.
    pub smCount: ::core::ffi::c_uint,
}
/** \struct CUdevSmResource
 Data for SM-related resources*/
pub type CUdevSmResource = CUdevSmResource_st;
/** \struct CUdevResource
 A tagged union describing different resources identified by the type field. This structure should not be directly modified outside of the API that created it.
 \code
 struct {
     CUdevResourceType type;
     union {
         CUdevSmResource sm;
     };
 };
 \endcode
 - If \p type is \p CU_DEV_RESOURCE_TYPE_INVALID, this resoure is not valid and cannot be further accessed.
 - If \p type is \p CU_DEV_RESOURCE_TYPE_SM, the ::CUdevSmResource structure \p sm is filled in. For example,
 \p sm.smCount will reflect the amount of streaming multiprocessors available in this resource.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUdevResource_st {
    ///< Type of resource, dictates which union field was last set
    pub type_: CUdevResourceType,
    pub _internal_padding: [::core::ffi::c_uchar; 92usize],
    pub __bindgen_anon_1: CUdevResource_st__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUdevResource_st__bindgen_ty_1 {
    ///< Resource corresponding to CU_DEV_RESOURCE_TYPE_SM \p. type.
    pub sm: CUdevSmResource,
    pub _oversize: [::core::ffi::c_uchar; 48usize],
}
/** \struct CUdevResource
 A tagged union describing different resources identified by the type field. This structure should not be directly modified outside of the API that created it.
 \code
 struct {
     CUdevResourceType type;
     union {
         CUdevSmResource sm;
     };
 };
 \endcode
 - If \p type is \p CU_DEV_RESOURCE_TYPE_INVALID, this resoure is not valid and cannot be further accessed.
 - If \p type is \p CU_DEV_RESOURCE_TYPE_SM, the ::CUdevSmResource structure \p sm is filled in. For example,
 \p sm.smCount will reflect the amount of streaming multiprocessors available in this resource.*/
pub type CUdevResource_v1 = CUdevResource_st;
/** \struct CUdevResource
 A tagged union describing different resources identified by the type field. This structure should not be directly modified outside of the API that created it.
 \code
 struct {
     CUdevResourceType type;
     union {
         CUdevSmResource sm;
     };
 };
 \endcode
 - If \p type is \p CU_DEV_RESOURCE_TYPE_INVALID, this resoure is not valid and cannot be further accessed.
 - If \p type is \p CU_DEV_RESOURCE_TYPE_SM, the ::CUdevSmResource structure \p sm is filled in. For example,
 \p sm.smCount will reflect the amount of streaming multiprocessors available in this resource.*/
pub type CUdevResource = CUdevResource_v1;
#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUdeviceptr_v1(pub ::core::ffi::c_uint);
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY2D_v1_st {
    ///< Source X in bytes
    pub srcXInBytes: ::core::ffi::c_uint,
    ///< Source Y
    pub srcY: ::core::ffi::c_uint,
    ///< Source memory type (host, device, array)
    pub srcMemoryType: CUmemorytype,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device pointer
    pub srcDevice: CUdeviceptr_v1,
    ///< Source array reference
    pub srcArray: CUarray,
    ///< Source pitch (ignored when src is array)
    pub srcPitch: ::core::ffi::c_uint,
    ///< Destination X in bytes
    pub dstXInBytes: ::core::ffi::c_uint,
    ///< Destination Y
    pub dstY: ::core::ffi::c_uint,
    ///< Destination memory type (host, device, array)
    pub dstMemoryType: CUmemorytype,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device pointer
    pub dstDevice: CUdeviceptr_v1,
    ///< Destination array reference
    pub dstArray: CUarray,
    ///< Destination pitch (ignored when dst is array)
    pub dstPitch: ::core::ffi::c_uint,
    ///< Width of 2D memory copy in bytes
    pub WidthInBytes: ::core::ffi::c_uint,
    ///< Height of 2D memory copy
    pub Height: ::core::ffi::c_uint,
}
pub type CUDA_MEMCPY2D_v1 = CUDA_MEMCPY2D_v1_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_MEMCPY3D_v1_st {
    ///< Source X in bytes
    pub srcXInBytes: ::core::ffi::c_uint,
    ///< Source Y
    pub srcY: ::core::ffi::c_uint,
    ///< Source Z
    pub srcZ: ::core::ffi::c_uint,
    ///< Source LOD
    pub srcLOD: ::core::ffi::c_uint,
    ///< Source memory type (host, device, array)
    pub srcMemoryType: CUmemorytype,
    ///< Source host pointer
    pub srcHost: *const ::core::ffi::c_void,
    ///< Source device pointer
    pub srcDevice: CUdeviceptr_v1,
    ///< Source array reference
    pub srcArray: CUarray,
    ///< Must be NULL
    pub reserved0: *mut ::core::ffi::c_void,
    ///< Source pitch (ignored when src is array)
    pub srcPitch: ::core::ffi::c_uint,
    ///< Source height (ignored when src is array; may be 0 if Depth==1)
    pub srcHeight: ::core::ffi::c_uint,
    ///< Destination X in bytes
    pub dstXInBytes: ::core::ffi::c_uint,
    ///< Destination Y
    pub dstY: ::core::ffi::c_uint,
    ///< Destination Z
    pub dstZ: ::core::ffi::c_uint,
    ///< Destination LOD
    pub dstLOD: ::core::ffi::c_uint,
    ///< Destination memory type (host, device, array)
    pub dstMemoryType: CUmemorytype,
    ///< Destination host pointer
    pub dstHost: *mut ::core::ffi::c_void,
    ///< Destination device pointer
    pub dstDevice: CUdeviceptr_v1,
    ///< Destination array reference
    pub dstArray: CUarray,
    ///< Must be NULL
    pub reserved1: *mut ::core::ffi::c_void,
    ///< Destination pitch (ignored when dst is array)
    pub dstPitch: ::core::ffi::c_uint,
    ///< Destination height (ignored when dst is array; may be 0 if Depth==1)
    pub dstHeight: ::core::ffi::c_uint,
    ///< Width of 3D memory copy in bytes
    pub WidthInBytes: ::core::ffi::c_uint,
    ///< Height of 3D memory copy
    pub Height: ::core::ffi::c_uint,
    ///< Depth of 3D memory copy
    pub Depth: ::core::ffi::c_uint,
}
pub type CUDA_MEMCPY3D_v1 = CUDA_MEMCPY3D_v1_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY_DESCRIPTOR_v1_st {
    ///< Width of array
    pub Width: ::core::ffi::c_uint,
    ///< Height of array
    pub Height: ::core::ffi::c_uint,
    ///< Array format
    pub Format: CUarray_format,
    ///< Channels per array element
    pub NumChannels: ::core::ffi::c_uint,
}
pub type CUDA_ARRAY_DESCRIPTOR_v1 = CUDA_ARRAY_DESCRIPTOR_v1_st;
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CUDA_ARRAY3D_DESCRIPTOR_v1_st {
    ///< Width of 3D array
    pub Width: ::core::ffi::c_uint,
    ///< Height of 3D array
    pub Height: ::core::ffi::c_uint,
    ///< Depth of 3D array
    pub Depth: ::core::ffi::c_uint,
    ///< Array format
    pub Format: CUarray_format,
    ///< Channels per array element
    pub NumChannels: ::core::ffi::c_uint,
    ///< Flags
    pub Flags: ::core::ffi::c_uint,
}
pub type CUDA_ARRAY3D_DESCRIPTOR_v1 = CUDA_ARRAY3D_DESCRIPTOR_v1_st;
impl CUoutput_mode_enum {
    ///< Output mode Key-Value pair format.
    pub const CU_OUT_KEY_VALUE_PAIR: CUoutput_mode_enum = CUoutput_mode_enum(0);
}
impl CUoutput_mode_enum {
    ///< Output mode Comma separated values format.
    pub const CU_OUT_CSV: CUoutput_mode_enum = CUoutput_mode_enum(1);
}
#[repr(transparent)]
/// Profiler Output Modes
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUoutput_mode_enum(pub ::core::ffi::c_uint);
/// Profiler Output Modes
pub use self::CUoutput_mode_enum as CUoutput_mode;
pub type GLenum = ::core::ffi::c_uint;
pub type GLuint = ::core::ffi::c_uint;
pub type khronos_int32_t = i32;
impl CUGLDeviceList_enum {
    ///< The CUDA devices for all GPUs used by the current OpenGL context
    pub const CU_GL_DEVICE_LIST_ALL: CUGLDeviceList_enum = CUGLDeviceList_enum(1);
}
impl CUGLDeviceList_enum {
    ///< The CUDA devices for the GPUs used by the current OpenGL context in its currently rendering frame
    pub const CU_GL_DEVICE_LIST_CURRENT_FRAME: CUGLDeviceList_enum = CUGLDeviceList_enum(
        2,
    );
}
impl CUGLDeviceList_enum {
    ///< The CUDA devices for the GPUs to be used by the current OpenGL context in the next frame
    pub const CU_GL_DEVICE_LIST_NEXT_FRAME: CUGLDeviceList_enum = CUGLDeviceList_enum(3);
}
#[repr(transparent)]
/// CUDA devices corresponding to an OpenGL device
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUGLDeviceList_enum(pub ::core::ffi::c_uint);
/// CUDA devices corresponding to an OpenGL device
pub use self::CUGLDeviceList_enum as CUGLDeviceList;
impl CUGLmap_flags_enum {
    pub const CU_GL_MAP_RESOURCE_FLAGS_NONE: CUGLmap_flags_enum = CUGLmap_flags_enum(0);
}
impl CUGLmap_flags_enum {
    pub const CU_GL_MAP_RESOURCE_FLAGS_READ_ONLY: CUGLmap_flags_enum = CUGLmap_flags_enum(
        1,
    );
}
impl CUGLmap_flags_enum {
    pub const CU_GL_MAP_RESOURCE_FLAGS_WRITE_DISCARD: CUGLmap_flags_enum = CUGLmap_flags_enum(
        2,
    );
}
#[repr(transparent)]
/// Flags to map or unmap a resource
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUGLmap_flags_enum(pub ::core::ffi::c_uint);
/// Flags to map or unmap a resource
pub use self::CUGLmap_flags_enum as CUGLmap_flags;
pub type EGLint = khronos_int32_t;
pub type EGLSyncKHR = *mut ::core::ffi::c_void;
pub type EGLImageKHR = *mut ::core::ffi::c_void;
pub type EGLStreamKHR = *mut ::core::ffi::c_void;
impl CUeglFrameType_enum {
    ///< Frame type CUDA array
    pub const CU_EGL_FRAME_TYPE_ARRAY: CUeglFrameType_enum = CUeglFrameType_enum(0);
}
impl CUeglFrameType_enum {
    ///< Frame type pointer
    pub const CU_EGL_FRAME_TYPE_PITCH: CUeglFrameType_enum = CUeglFrameType_enum(1);
}
#[repr(transparent)]
/// CUDA EglFrame type - array or pointer
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUeglFrameType_enum(pub ::core::ffi::c_uint);
/// CUDA EglFrame type - array or pointer
pub use self::CUeglFrameType_enum as CUeglFrameType;
impl CUeglResourceLocationFlags_enum {
    ///< Resource location sysmem
    pub const CU_EGL_RESOURCE_LOCATION_SYSMEM: CUeglResourceLocationFlags_enum = CUeglResourceLocationFlags_enum(
        0,
    );
}
impl CUeglResourceLocationFlags_enum {
    ///< Resource location vidmem
    pub const CU_EGL_RESOURCE_LOCATION_VIDMEM: CUeglResourceLocationFlags_enum = CUeglResourceLocationFlags_enum(
        1,
    );
}
#[repr(transparent)]
/** Resource location flags- sysmem or vidmem

 For CUDA context on iGPU, since video and system memory are equivalent -
 these flags will not have an effect on the execution.

 For CUDA context on dGPU, applications can use the flag ::CUeglResourceLocationFlags
 to give a hint about the desired location.

 ::CU_EGL_RESOURCE_LOCATION_SYSMEM - the frame data is made resident on the system memory
 to be accessed by CUDA.

 ::CU_EGL_RESOURCE_LOCATION_VIDMEM - the frame data is made resident on the dedicated
 video memory to be accessed by CUDA.

 There may be an additional latency due to new allocation and data migration,
 if the frame is produced on a different memory.
*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUeglResourceLocationFlags_enum(pub ::core::ffi::c_uint);
/** Resource location flags- sysmem or vidmem

 For CUDA context on iGPU, since video and system memory are equivalent -
 these flags will not have an effect on the execution.

 For CUDA context on dGPU, applications can use the flag ::CUeglResourceLocationFlags
 to give a hint about the desired location.

 ::CU_EGL_RESOURCE_LOCATION_SYSMEM - the frame data is made resident on the system memory
 to be accessed by CUDA.

 ::CU_EGL_RESOURCE_LOCATION_VIDMEM - the frame data is made resident on the dedicated
 video memory to be accessed by CUDA.

 There may be an additional latency due to new allocation and data migration,
 if the frame is produced on a different memory.
*/
pub use self::CUeglResourceLocationFlags_enum as CUeglResourceLocationFlags;
impl CUeglColorFormat_enum {
    ///< Y, U, V in three surfaces, each in a separate surface, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        0,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, UV in two surfaces (UV as one surface) with VU byte ordering, width, height ratio same as YUV420Planar.
    pub const CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        1,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V  each in a separate  surface, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV422_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        2,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, UV in two surfaces with VU byte ordering, width, height ratio same as YUV422Planar.
    pub const CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        3,
    );
}
impl CUeglColorFormat_enum {
    ///< R/G/B three channels in one surface with BGR byte ordering. Only pitch linear format supported.
    pub const CU_EGL_COLOR_FORMAT_RGB: CUeglColorFormat_enum = CUeglColorFormat_enum(4);
}
impl CUeglColorFormat_enum {
    ///< R/G/B three channels in one surface with RGB byte ordering. Only pitch linear format supported.
    pub const CU_EGL_COLOR_FORMAT_BGR: CUeglColorFormat_enum = CUeglColorFormat_enum(5);
}
impl CUeglColorFormat_enum {
    ///< R/G/B/A four channels in one surface with BGRA byte ordering.
    pub const CU_EGL_COLOR_FORMAT_ARGB: CUeglColorFormat_enum = CUeglColorFormat_enum(6);
}
impl CUeglColorFormat_enum {
    ///< R/G/B/A four channels in one surface with ABGR byte ordering.
    pub const CU_EGL_COLOR_FORMAT_RGBA: CUeglColorFormat_enum = CUeglColorFormat_enum(7);
}
impl CUeglColorFormat_enum {
    ///< single luminance channel in one surface.
    pub const CU_EGL_COLOR_FORMAT_L: CUeglColorFormat_enum = CUeglColorFormat_enum(8);
}
impl CUeglColorFormat_enum {
    ///< single color channel in one surface.
    pub const CU_EGL_COLOR_FORMAT_R: CUeglColorFormat_enum = CUeglColorFormat_enum(9);
}
impl CUeglColorFormat_enum {
    ///< Y, U, V in three surfaces, each in a separate surface, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV444_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        10,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, UV in two surfaces (UV as one surface) with VU byte ordering, width, height ratio same as YUV444Planar.
    pub const CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        11,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V in one surface, interleaved as UYVY in one channel.
    pub const CU_EGL_COLOR_FORMAT_YUYV_422: CUeglColorFormat_enum = CUeglColorFormat_enum(
        12,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V in one surface, interleaved as YUYV in one channel.
    pub const CU_EGL_COLOR_FORMAT_UYVY_422: CUeglColorFormat_enum = CUeglColorFormat_enum(
        13,
    );
}
impl CUeglColorFormat_enum {
    ///< R/G/B/A four channels in one surface with RGBA byte ordering.
    pub const CU_EGL_COLOR_FORMAT_ABGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        14,
    );
}
impl CUeglColorFormat_enum {
    ///< R/G/B/A four channels in one surface with ARGB byte ordering.
    pub const CU_EGL_COLOR_FORMAT_BGRA: CUeglColorFormat_enum = CUeglColorFormat_enum(
        15,
    );
}
impl CUeglColorFormat_enum {
    ///< Alpha color format - one channel in one surface.
    pub const CU_EGL_COLOR_FORMAT_A: CUeglColorFormat_enum = CUeglColorFormat_enum(16);
}
impl CUeglColorFormat_enum {
    ///< R/G color format - two channels in one surface with GR byte ordering
    pub const CU_EGL_COLOR_FORMAT_RG: CUeglColorFormat_enum = CUeglColorFormat_enum(17);
}
impl CUeglColorFormat_enum {
    ///< Y, U, V, A four channels in one surface, interleaved as VUYA.
    pub const CU_EGL_COLOR_FORMAT_AYUV: CUeglColorFormat_enum = CUeglColorFormat_enum(
        18,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        19,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        20,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        21,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces (VU as one surface) with UV byte ordering, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        22,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        23,
    );
}
impl CUeglColorFormat_enum {
    ///< Y12, V12U12 in two surfaces (VU as one surface) with UV byte ordering, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        24,
    );
}
impl CUeglColorFormat_enum {
    ///< Y12, V12U12 in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        25,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in one surface, interleaved as YVYU in one channel.
    pub const CU_EGL_COLOR_FORMAT_VYUY_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        26,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in one surface, interleaved as YUYV in one channel.
    pub const CU_EGL_COLOR_FORMAT_UYVY_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        27,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in one surface, interleaved as UYVY in one channel.
    pub const CU_EGL_COLOR_FORMAT_YUYV_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        28,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in one surface, interleaved as VYUY in one channel.
    pub const CU_EGL_COLOR_FORMAT_YVYU_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        29,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V three channels in one surface, interleaved as VUY. Only pitch linear format supported.
    pub const CU_EGL_COLOR_FORMAT_YUV_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        30,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V, A four channels in one surface, interleaved as AVUY.
    pub const CU_EGL_COLOR_FORMAT_YUVA_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        31,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V, A four channels in one surface, interleaved as VUYA.
    pub const CU_EGL_COLOR_FORMAT_AYUV_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        32,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in three surfaces, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV444_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        33,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in three surfaces, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV422_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        34,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, U, V in three surfaces, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        35,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, UV in two surfaces (UV as one surface) with VU byte ordering, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV444_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        36,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, UV in two surfaces (UV as one surface) with VU byte ordering, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV422_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        37,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, UV in two surfaces (UV as one surface) with VU byte ordering, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        38,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, V, U in three surfaces, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU444_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        39,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, V, U in three surfaces, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU422_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        40,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, V, U in three surfaces, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_PLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        41,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU444_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        42,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU422_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        43,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y, VU in two surfaces (VU as one surface) with UV byte ordering, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        44,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved RGGB ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        45,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved BGGR ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        46,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved GRBG ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        47,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved GBRG ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        48,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer10 format - one channel in one surface with interleaved RGGB ordering. Out of 16 bits, 10 bits used 6 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER10_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        49,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer10 format - one channel in one surface with interleaved BGGR ordering. Out of 16 bits, 10 bits used 6 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER10_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        50,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer10 format - one channel in one surface with interleaved GRBG ordering. Out of 16 bits, 10 bits used 6 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER10_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        51,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer10 format - one channel in one surface with interleaved GBRG ordering. Out of 16 bits, 10 bits used 6 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER10_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        52,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved RGGB ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        53,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved BGGR ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        54,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved GRBG ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        55,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved GBRG ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        56,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer14 format - one channel in one surface with interleaved RGGB ordering. Out of 16 bits, 14 bits used 2 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER14_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        57,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer14 format - one channel in one surface with interleaved BGGR ordering. Out of 16 bits, 14 bits used 2 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER14_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        58,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer14 format - one channel in one surface with interleaved GRBG ordering. Out of 16 bits, 14 bits used 2 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER14_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        59,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer14 format - one channel in one surface with interleaved GBRG ordering. Out of 16 bits, 14 bits used 2 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER14_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        60,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer20 format - one channel in one surface with interleaved RGGB ordering. Out of 32 bits, 20 bits used 12 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER20_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        61,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer20 format - one channel in one surface with interleaved BGGR ordering. Out of 32 bits, 20 bits used 12 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER20_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        62,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer20 format - one channel in one surface with interleaved GRBG ordering. Out of 32 bits, 20 bits used 12 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER20_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        63,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer20 format - one channel in one surface with interleaved GBRG ordering. Out of 32 bits, 20 bits used 12 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER20_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        64,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, V, U in three surfaces, each in a separate surface, U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU444_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        65,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, V, U in three surfaces, each in a separate surface, U/V width = 1/2 Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU422_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        66,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, V, U in three surfaces, each in a separate surface, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_PLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        67,
    );
}
impl CUeglColorFormat_enum {
    ///< Nvidia proprietary Bayer ISP format - one channel in one surface with interleaved RGGB ordering and mapped to opaque integer datatype.
    pub const CU_EGL_COLOR_FORMAT_BAYER_ISP_RGGB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        68,
    );
}
impl CUeglColorFormat_enum {
    ///< Nvidia proprietary Bayer ISP format - one channel in one surface with interleaved BGGR ordering and mapped to opaque integer datatype.
    pub const CU_EGL_COLOR_FORMAT_BAYER_ISP_BGGR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        69,
    );
}
impl CUeglColorFormat_enum {
    ///< Nvidia proprietary Bayer ISP format - one channel in one surface with interleaved GRBG ordering and mapped to opaque integer datatype.
    pub const CU_EGL_COLOR_FORMAT_BAYER_ISP_GRBG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        70,
    );
}
impl CUeglColorFormat_enum {
    ///< Nvidia proprietary Bayer ISP format - one channel in one surface with interleaved GBRG ordering and mapped to opaque integer datatype.
    pub const CU_EGL_COLOR_FORMAT_BAYER_ISP_GBRG: CUeglColorFormat_enum = CUeglColorFormat_enum(
        71,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved BCCR ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_BCCR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        72,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved RCCB ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_RCCB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        73,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved CRBC ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_CRBC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        74,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer format - one channel in one surface with interleaved CBRC ordering.
    pub const CU_EGL_COLOR_FORMAT_BAYER_CBRC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        75,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer10 format - one channel in one surface with interleaved CCCC ordering. Out of 16 bits, 10 bits used 6 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER10_CCCC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        76,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved BCCR ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_BCCR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        77,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved RCCB ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_RCCB: CUeglColorFormat_enum = CUeglColorFormat_enum(
        78,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved CRBC ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_CRBC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        79,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved CBRC ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_CBRC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        80,
    );
}
impl CUeglColorFormat_enum {
    ///< Bayer12 format - one channel in one surface with interleaved CCCC ordering. Out of 16 bits, 12 bits used 4 bits No-op.
    pub const CU_EGL_COLOR_FORMAT_BAYER12_CCCC: CUeglColorFormat_enum = CUeglColorFormat_enum(
        81,
    );
}
impl CUeglColorFormat_enum {
    ///< Color format for single Y plane.
    pub const CU_EGL_COLOR_FORMAT_Y: CUeglColorFormat_enum = CUeglColorFormat_enum(82);
}
impl CUeglColorFormat_enum {
    ///< Y, UV in two surfaces (UV as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        83,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, VU in two surfaces (VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        84,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V  each in a separate  surface, U/V width = 1/2 Y width, U/V height= 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_PLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        85,
    );
}
impl CUeglColorFormat_enum {
    /**< Y, V, U each in a separate surface, U/V width = 1/2 Y width, U/V height
= 1/2 Y height.*/
    pub const CU_EGL_COLOR_FORMAT_YVU420_PLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        86,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, UV in two surfaces (UV as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YUV420_SEMIPLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        87,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, VU in two surfaces (VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_SEMIPLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        88,
    );
}
impl CUeglColorFormat_enum {
    /**< Y, U, V  each in a separate  surface, U/V width = 1/2 Y width, U/V height
= 1/2 Y height.*/
    pub const CU_EGL_COLOR_FORMAT_YUV420_PLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        89,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, V, U each in a separate surface, U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_YVU420_PLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        90,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces (VU as one surface), U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        91,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces (VU as one surface), U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        92,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces(VU as one surface) U/V width = 1/2 Y width, U/V height  = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_2020: CUeglColorFormat_enum = CUeglColorFormat_enum(
        93,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces(VU as one surface) U/V width = 1/2 Y width, U/V height  = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR: CUeglColorFormat_enum = CUeglColorFormat_enum(
        94,
    );
}
impl CUeglColorFormat_enum {
    ///< Y10, V10U10 in two surfaces(VU as one surface) U/V width = 1/2 Y width, U/V height  = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_422_SEMIPLANAR_709: CUeglColorFormat_enum = CUeglColorFormat_enum(
        95,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y plane.
    pub const CU_EGL_COLOR_FORMAT_Y_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        96,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y plane.
    pub const CU_EGL_COLOR_FORMAT_Y_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        97,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y10 plane.
    pub const CU_EGL_COLOR_FORMAT_Y10_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        98,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y10 plane.
    pub const CU_EGL_COLOR_FORMAT_Y10_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        99,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y12 plane.
    pub const CU_EGL_COLOR_FORMAT_Y12_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        100,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Color format for single Y12 plane.
    pub const CU_EGL_COLOR_FORMAT_Y12_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        101,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V, A four channels in one surface, interleaved as AVUY.
    pub const CU_EGL_COLOR_FORMAT_YUVA: CUeglColorFormat_enum = CUeglColorFormat_enum(
        102,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V three channels in one surface, interleaved as VUY. Only pitch linear format supported.
    pub const CU_EGL_COLOR_FORMAT_YUV: CUeglColorFormat_enum = CUeglColorFormat_enum(
        103,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V in one surface, interleaved as YVYU in one channel.
    pub const CU_EGL_COLOR_FORMAT_YVYU: CUeglColorFormat_enum = CUeglColorFormat_enum(
        104,
    );
}
impl CUeglColorFormat_enum {
    ///< Y, U, V in one surface, interleaved as VYUY in one channel.
    pub const CU_EGL_COLOR_FORMAT_VYUY: CUeglColorFormat_enum = CUeglColorFormat_enum(
        105,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y10, V10U10 in two surfaces(VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        106,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y10, V10U10 in two surfaces(VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_420_SEMIPLANAR_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        107,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y10, V10U10 in two surfaces (VU as one surface) U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        108,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y10, V10U10 in two surfaces (VU as one surface)  U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y10V10U10_444_SEMIPLANAR_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        109,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y12, V12U12 in two surfaces (VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        110,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y12, V12U12 in two surfaces (VU as one surface) U/V width = 1/2 Y width, U/V height = 1/2 Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_420_SEMIPLANAR_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        111,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y12, V12U12 in two surfaces (VU as one surface) U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        112,
    );
}
impl CUeglColorFormat_enum {
    ///< Extended Range Y12, V12U12 in two surfaces (VU as one surface) U/V width = Y width, U/V height = Y height.
    pub const CU_EGL_COLOR_FORMAT_Y12V12U12_444_SEMIPLANAR_709_ER: CUeglColorFormat_enum = CUeglColorFormat_enum(
        113,
    );
}
impl CUeglColorFormat_enum {
    pub const CU_EGL_COLOR_FORMAT_MAX: CUeglColorFormat_enum = CUeglColorFormat_enum(
        114,
    );
}
#[repr(transparent)]
/** CUDA EGL Color Format - The different planar and multiplanar formats currently supported for CUDA_EGL interops.
 Three channel formats are currently not supported for ::CU_EGL_FRAME_TYPE_ARRAY*/
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct CUeglColorFormat_enum(pub ::core::ffi::c_uint);
/** CUDA EGL Color Format - The different planar and multiplanar formats currently supported for CUDA_EGL interops.
 Three channel formats are currently not supported for ::CU_EGL_FRAME_TYPE_ARRAY*/
pub use self::CUeglColorFormat_enum as CUeglColorFormat;
/** CUDA EGLFrame structure Descriptor - structure defining one frame of EGL.

 Each frame may contain one or more planes depending on whether the surface  * is Multiplanar or not.*/
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CUeglFrame_st {
    pub frame: CUeglFrame_st__bindgen_ty_1,
    ///< Width of first plane
    pub width: ::core::ffi::c_uint,
    ///< Height of first plane
    pub height: ::core::ffi::c_uint,
    ///< Depth of first plane
    pub depth: ::core::ffi::c_uint,
    ///< Pitch of first plane
    pub pitch: ::core::ffi::c_uint,
    ///< Number of planes
    pub planeCount: ::core::ffi::c_uint,
    ///< Number of channels for the plane
    pub numChannels: ::core::ffi::c_uint,
    ///< Array or Pitch
    pub frameType: CUeglFrameType,
    ///< CUDA EGL Color Format
    pub eglColorFormat: CUeglColorFormat,
    ///< CUDA Array Format
    pub cuFormat: CUarray_format,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union CUeglFrame_st__bindgen_ty_1 {
    ///< Array of CUarray corresponding to each plane
    pub pArray: [CUarray; 3usize],
    ///< Array of Pointers corresponding to each plane
    pub pPitch: [*mut ::core::ffi::c_void; 3usize],
}
/** CUDA EGLFrame structure Descriptor - structure defining one frame of EGL.

 Each frame may contain one or more planes depending on whether the surface  * is Multiplanar or not.*/
pub type CUeglFrame_v1 = CUeglFrame_st;
/** CUDA EGLFrame structure Descriptor - structure defining one frame of EGL.

 Each frame may contain one or more planes depending on whether the surface  * is Multiplanar or not.*/
pub type CUeglFrame = CUeglFrame_v1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CUeglStreamConnection_st {
    _unused: [u8; 0],
}
/// CUDA EGLSream Connection
pub type CUeglStreamConnection = *mut CUeglStreamConnection_st;
impl VdpStatus {
    pub const VDP_STATUS_OK: VdpStatus = VdpStatus(0);
}
impl VdpStatus {
    pub const VDP_STATUS_NO_IMPLEMENTATION: VdpStatus = VdpStatus(1);
}
impl VdpStatus {
    pub const VDP_STATUS_DISPLAY_PREEMPTED: VdpStatus = VdpStatus(2);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_HANDLE: VdpStatus = VdpStatus(3);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_POINTER: VdpStatus = VdpStatus(4);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_CHROMA_TYPE: VdpStatus = VdpStatus(5);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_Y_CB_CR_FORMAT: VdpStatus = VdpStatus(6);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_RGBA_FORMAT: VdpStatus = VdpStatus(7);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_INDEXED_FORMAT: VdpStatus = VdpStatus(8);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_COLOR_STANDARD: VdpStatus = VdpStatus(9);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_COLOR_TABLE_FORMAT: VdpStatus = VdpStatus(10);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_BLEND_FACTOR: VdpStatus = VdpStatus(11);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_BLEND_EQUATION: VdpStatus = VdpStatus(12);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_FLAG: VdpStatus = VdpStatus(13);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_DECODER_PROFILE: VdpStatus = VdpStatus(14);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_VIDEO_MIXER_FEATURE: VdpStatus = VdpStatus(15);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_VIDEO_MIXER_PARAMETER: VdpStatus = VdpStatus(16);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_VIDEO_MIXER_ATTRIBUTE: VdpStatus = VdpStatus(17);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_VIDEO_MIXER_PICTURE_STRUCTURE: VdpStatus = VdpStatus(
        18,
    );
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_FUNC_ID: VdpStatus = VdpStatus(19);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_SIZE: VdpStatus = VdpStatus(20);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_VALUE: VdpStatus = VdpStatus(21);
}
impl VdpStatus {
    pub const VDP_STATUS_INVALID_STRUCT_VERSION: VdpStatus = VdpStatus(22);
}
impl VdpStatus {
    pub const VDP_STATUS_RESOURCES: VdpStatus = VdpStatus(23);
}
impl VdpStatus {
    pub const VDP_STATUS_HANDLE_DEVICE_MISMATCH: VdpStatus = VdpStatus(24);
}
impl VdpStatus {
    pub const VDP_STATUS_ERROR: VdpStatus = VdpStatus(25);
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct VdpStatus(pub ::core::ffi::c_uint);
pub type VdpDevice = u32;
pub type VdpVideoSurface = u32;
pub type VdpOutputSurface = u32;
pub type VdpFuncId = u32;
pub type VdpGetProcAddress = ::core::option::Option<
    unsafe extern "system" fn(
        device: VdpDevice,
        function_id: VdpFuncId,
        function_pointer: *mut *mut ::core::ffi::c_void,
    ) -> VdpStatus,
>;
impl CUerror {
    pub const INVALID_VALUE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(1)
    });
    pub const OUT_OF_MEMORY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(2)
    });
    pub const NOT_INITIALIZED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(3)
    });
    pub const DEINITIALIZED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(4)
    });
    pub const PROFILER_DISABLED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(5)
    });
    pub const PROFILER_NOT_INITIALIZED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(6)
    });
    pub const PROFILER_ALREADY_STARTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(7)
    });
    pub const PROFILER_ALREADY_STOPPED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(8)
    });
    pub const STUB_LIBRARY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(34)
    });
    pub const DEVICE_UNAVAILABLE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(46)
    });
    pub const NO_DEVICE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(100)
    });
    pub const INVALID_DEVICE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(101)
    });
    pub const DEVICE_NOT_LICENSED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(102)
    });
    pub const INVALID_IMAGE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(200)
    });
    pub const INVALID_CONTEXT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(201)
    });
    pub const CONTEXT_ALREADY_CURRENT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(202)
    });
    pub const MAP_FAILED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(205)
    });
    pub const UNMAP_FAILED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(206)
    });
    pub const ARRAY_IS_MAPPED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(207)
    });
    pub const ALREADY_MAPPED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(208)
    });
    pub const NO_BINARY_FOR_GPU: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(209)
    });
    pub const ALREADY_ACQUIRED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(210)
    });
    pub const NOT_MAPPED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(211)
    });
    pub const NOT_MAPPED_AS_ARRAY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(212)
    });
    pub const NOT_MAPPED_AS_POINTER: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(213)
    });
    pub const ECC_UNCORRECTABLE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(214)
    });
    pub const UNSUPPORTED_LIMIT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(215)
    });
    pub const CONTEXT_ALREADY_IN_USE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(216)
    });
    pub const PEER_ACCESS_UNSUPPORTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(217)
    });
    pub const INVALID_PTX: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(218)
    });
    pub const INVALID_GRAPHICS_CONTEXT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(219)
    });
    pub const NVLINK_UNCORRECTABLE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(220)
    });
    pub const JIT_COMPILER_NOT_FOUND: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(221)
    });
    pub const UNSUPPORTED_PTX_VERSION: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(222)
    });
    pub const JIT_COMPILATION_DISABLED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(223)
    });
    pub const UNSUPPORTED_EXEC_AFFINITY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(224)
    });
    pub const UNSUPPORTED_DEVSIDE_SYNC: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(225)
    });
    pub const INVALID_SOURCE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(300)
    });
    pub const FILE_NOT_FOUND: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(301)
    });
    pub const SHARED_OBJECT_SYMBOL_NOT_FOUND: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(302)
    });
    pub const SHARED_OBJECT_INIT_FAILED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(303)
    });
    pub const OPERATING_SYSTEM: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(304)
    });
    pub const INVALID_HANDLE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(400)
    });
    pub const ILLEGAL_STATE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(401)
    });
    pub const LOSSY_QUERY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(402)
    });
    pub const NOT_FOUND: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(500)
    });
    pub const NOT_READY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(600)
    });
    pub const ILLEGAL_ADDRESS: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(700)
    });
    pub const LAUNCH_OUT_OF_RESOURCES: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(701)
    });
    pub const LAUNCH_TIMEOUT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(702)
    });
    pub const LAUNCH_INCOMPATIBLE_TEXTURING: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(703)
    });
    pub const PEER_ACCESS_ALREADY_ENABLED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(704)
    });
    pub const PEER_ACCESS_NOT_ENABLED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(705)
    });
    pub const PRIMARY_CONTEXT_ACTIVE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(708)
    });
    pub const CONTEXT_IS_DESTROYED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(709)
    });
    pub const ASSERT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(710)
    });
    pub const TOO_MANY_PEERS: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(711)
    });
    pub const HOST_MEMORY_ALREADY_REGISTERED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(712)
    });
    pub const HOST_MEMORY_NOT_REGISTERED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(713)
    });
    pub const HARDWARE_STACK_ERROR: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(714)
    });
    pub const ILLEGAL_INSTRUCTION: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(715)
    });
    pub const MISALIGNED_ADDRESS: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(716)
    });
    pub const INVALID_ADDRESS_SPACE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(717)
    });
    pub const INVALID_PC: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(718)
    });
    pub const LAUNCH_FAILED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(719)
    });
    pub const COOPERATIVE_LAUNCH_TOO_LARGE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(720)
    });
    pub const NOT_PERMITTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(800)
    });
    pub const NOT_SUPPORTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(801)
    });
    pub const SYSTEM_NOT_READY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(802)
    });
    pub const SYSTEM_DRIVER_MISMATCH: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(803)
    });
    pub const COMPAT_NOT_SUPPORTED_ON_DEVICE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(804)
    });
    pub const MPS_CONNECTION_FAILED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(805)
    });
    pub const MPS_RPC_FAILURE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(806)
    });
    pub const MPS_SERVER_NOT_READY: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(807)
    });
    pub const MPS_MAX_CLIENTS_REACHED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(808)
    });
    pub const MPS_MAX_CONNECTIONS_REACHED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(809)
    });
    pub const MPS_CLIENT_TERMINATED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(810)
    });
    pub const CDP_NOT_SUPPORTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(811)
    });
    pub const CDP_VERSION_MISMATCH: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(812)
    });
    pub const STREAM_CAPTURE_UNSUPPORTED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(900)
    });
    pub const STREAM_CAPTURE_INVALIDATED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(901)
    });
    pub const STREAM_CAPTURE_MERGE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(902)
    });
    pub const STREAM_CAPTURE_UNMATCHED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(903)
    });
    pub const STREAM_CAPTURE_UNJOINED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(904)
    });
    pub const STREAM_CAPTURE_ISOLATION: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(905)
    });
    pub const STREAM_CAPTURE_IMPLICIT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(906)
    });
    pub const CAPTURED_EVENT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(907)
    });
    pub const STREAM_CAPTURE_WRONG_THREAD: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(908)
    });
    pub const TIMEOUT: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(909)
    });
    pub const GRAPH_EXEC_UPDATE_FAILURE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(910)
    });
    pub const EXTERNAL_DEVICE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(911)
    });
    pub const INVALID_CLUSTER_SIZE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(912)
    });
    pub const FUNCTION_NOT_LOADED: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(913)
    });
    pub const INVALID_RESOURCE_TYPE: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(914)
    });
    pub const INVALID_RESOURCE_CONFIGURATION: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(915)
    });
    pub const UNKNOWN: CUerror = CUerror(unsafe {
        ::core::num::NonZeroU32::new_unchecked(999)
    });
}
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CUerror(pub ::core::num::NonZeroU32);
pub trait CUresultConsts {
    const SUCCESS: CUresult = CUresult::Ok(());
    const ERROR_INVALID_VALUE: CUresult = CUresult::Err(CUerror::INVALID_VALUE);
    const ERROR_OUT_OF_MEMORY: CUresult = CUresult::Err(CUerror::OUT_OF_MEMORY);
    const ERROR_NOT_INITIALIZED: CUresult = CUresult::Err(CUerror::NOT_INITIALIZED);
    const ERROR_DEINITIALIZED: CUresult = CUresult::Err(CUerror::DEINITIALIZED);
    const ERROR_PROFILER_DISABLED: CUresult = CUresult::Err(CUerror::PROFILER_DISABLED);
    const ERROR_PROFILER_NOT_INITIALIZED: CUresult = CUresult::Err(
        CUerror::PROFILER_NOT_INITIALIZED,
    );
    const ERROR_PROFILER_ALREADY_STARTED: CUresult = CUresult::Err(
        CUerror::PROFILER_ALREADY_STARTED,
    );
    const ERROR_PROFILER_ALREADY_STOPPED: CUresult = CUresult::Err(
        CUerror::PROFILER_ALREADY_STOPPED,
    );
    const ERROR_STUB_LIBRARY: CUresult = CUresult::Err(CUerror::STUB_LIBRARY);
    const ERROR_DEVICE_UNAVAILABLE: CUresult = CUresult::Err(
        CUerror::DEVICE_UNAVAILABLE,
    );
    const ERROR_NO_DEVICE: CUresult = CUresult::Err(CUerror::NO_DEVICE);
    const ERROR_INVALID_DEVICE: CUresult = CUresult::Err(CUerror::INVALID_DEVICE);
    const ERROR_DEVICE_NOT_LICENSED: CUresult = CUresult::Err(
        CUerror::DEVICE_NOT_LICENSED,
    );
    const ERROR_INVALID_IMAGE: CUresult = CUresult::Err(CUerror::INVALID_IMAGE);
    const ERROR_INVALID_CONTEXT: CUresult = CUresult::Err(CUerror::INVALID_CONTEXT);
    const ERROR_CONTEXT_ALREADY_CURRENT: CUresult = CUresult::Err(
        CUerror::CONTEXT_ALREADY_CURRENT,
    );
    const ERROR_MAP_FAILED: CUresult = CUresult::Err(CUerror::MAP_FAILED);
    const ERROR_UNMAP_FAILED: CUresult = CUresult::Err(CUerror::UNMAP_FAILED);
    const ERROR_ARRAY_IS_MAPPED: CUresult = CUresult::Err(CUerror::ARRAY_IS_MAPPED);
    const ERROR_ALREADY_MAPPED: CUresult = CUresult::Err(CUerror::ALREADY_MAPPED);
    const ERROR_NO_BINARY_FOR_GPU: CUresult = CUresult::Err(CUerror::NO_BINARY_FOR_GPU);
    const ERROR_ALREADY_ACQUIRED: CUresult = CUresult::Err(CUerror::ALREADY_ACQUIRED);
    const ERROR_NOT_MAPPED: CUresult = CUresult::Err(CUerror::NOT_MAPPED);
    const ERROR_NOT_MAPPED_AS_ARRAY: CUresult = CUresult::Err(
        CUerror::NOT_MAPPED_AS_ARRAY,
    );
    const ERROR_NOT_MAPPED_AS_POINTER: CUresult = CUresult::Err(
        CUerror::NOT_MAPPED_AS_POINTER,
    );
    const ERROR_ECC_UNCORRECTABLE: CUresult = CUresult::Err(CUerror::ECC_UNCORRECTABLE);
    const ERROR_UNSUPPORTED_LIMIT: CUresult = CUresult::Err(CUerror::UNSUPPORTED_LIMIT);
    const ERROR_CONTEXT_ALREADY_IN_USE: CUresult = CUresult::Err(
        CUerror::CONTEXT_ALREADY_IN_USE,
    );
    const ERROR_PEER_ACCESS_UNSUPPORTED: CUresult = CUresult::Err(
        CUerror::PEER_ACCESS_UNSUPPORTED,
    );
    const ERROR_INVALID_PTX: CUresult = CUresult::Err(CUerror::INVALID_PTX);
    const ERROR_INVALID_GRAPHICS_CONTEXT: CUresult = CUresult::Err(
        CUerror::INVALID_GRAPHICS_CONTEXT,
    );
    const ERROR_NVLINK_UNCORRECTABLE: CUresult = CUresult::Err(
        CUerror::NVLINK_UNCORRECTABLE,
    );
    const ERROR_JIT_COMPILER_NOT_FOUND: CUresult = CUresult::Err(
        CUerror::JIT_COMPILER_NOT_FOUND,
    );
    const ERROR_UNSUPPORTED_PTX_VERSION: CUresult = CUresult::Err(
        CUerror::UNSUPPORTED_PTX_VERSION,
    );
    const ERROR_JIT_COMPILATION_DISABLED: CUresult = CUresult::Err(
        CUerror::JIT_COMPILATION_DISABLED,
    );
    const ERROR_UNSUPPORTED_EXEC_AFFINITY: CUresult = CUresult::Err(
        CUerror::UNSUPPORTED_EXEC_AFFINITY,
    );
    const ERROR_UNSUPPORTED_DEVSIDE_SYNC: CUresult = CUresult::Err(
        CUerror::UNSUPPORTED_DEVSIDE_SYNC,
    );
    const ERROR_INVALID_SOURCE: CUresult = CUresult::Err(CUerror::INVALID_SOURCE);
    const ERROR_FILE_NOT_FOUND: CUresult = CUresult::Err(CUerror::FILE_NOT_FOUND);
    const ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND: CUresult = CUresult::Err(
        CUerror::SHARED_OBJECT_SYMBOL_NOT_FOUND,
    );
    const ERROR_SHARED_OBJECT_INIT_FAILED: CUresult = CUresult::Err(
        CUerror::SHARED_OBJECT_INIT_FAILED,
    );
    const ERROR_OPERATING_SYSTEM: CUresult = CUresult::Err(CUerror::OPERATING_SYSTEM);
    const ERROR_INVALID_HANDLE: CUresult = CUresult::Err(CUerror::INVALID_HANDLE);
    const ERROR_ILLEGAL_STATE: CUresult = CUresult::Err(CUerror::ILLEGAL_STATE);
    const ERROR_LOSSY_QUERY: CUresult = CUresult::Err(CUerror::LOSSY_QUERY);
    const ERROR_NOT_FOUND: CUresult = CUresult::Err(CUerror::NOT_FOUND);
    const ERROR_NOT_READY: CUresult = CUresult::Err(CUerror::NOT_READY);
    const ERROR_ILLEGAL_ADDRESS: CUresult = CUresult::Err(CUerror::ILLEGAL_ADDRESS);
    const ERROR_LAUNCH_OUT_OF_RESOURCES: CUresult = CUresult::Err(
        CUerror::LAUNCH_OUT_OF_RESOURCES,
    );
    const ERROR_LAUNCH_TIMEOUT: CUresult = CUresult::Err(CUerror::LAUNCH_TIMEOUT);
    const ERROR_LAUNCH_INCOMPATIBLE_TEXTURING: CUresult = CUresult::Err(
        CUerror::LAUNCH_INCOMPATIBLE_TEXTURING,
    );
    const ERROR_PEER_ACCESS_ALREADY_ENABLED: CUresult = CUresult::Err(
        CUerror::PEER_ACCESS_ALREADY_ENABLED,
    );
    const ERROR_PEER_ACCESS_NOT_ENABLED: CUresult = CUresult::Err(
        CUerror::PEER_ACCESS_NOT_ENABLED,
    );
    const ERROR_PRIMARY_CONTEXT_ACTIVE: CUresult = CUresult::Err(
        CUerror::PRIMARY_CONTEXT_ACTIVE,
    );
    const ERROR_CONTEXT_IS_DESTROYED: CUresult = CUresult::Err(
        CUerror::CONTEXT_IS_DESTROYED,
    );
    const ERROR_ASSERT: CUresult = CUresult::Err(CUerror::ASSERT);
    const ERROR_TOO_MANY_PEERS: CUresult = CUresult::Err(CUerror::TOO_MANY_PEERS);
    const ERROR_HOST_MEMORY_ALREADY_REGISTERED: CUresult = CUresult::Err(
        CUerror::HOST_MEMORY_ALREADY_REGISTERED,
    );
    const ERROR_HOST_MEMORY_NOT_REGISTERED: CUresult = CUresult::Err(
        CUerror::HOST_MEMORY_NOT_REGISTERED,
    );
    const ERROR_HARDWARE_STACK_ERROR: CUresult = CUresult::Err(
        CUerror::HARDWARE_STACK_ERROR,
    );
    const ERROR_ILLEGAL_INSTRUCTION: CUresult = CUresult::Err(
        CUerror::ILLEGAL_INSTRUCTION,
    );
    const ERROR_MISALIGNED_ADDRESS: CUresult = CUresult::Err(
        CUerror::MISALIGNED_ADDRESS,
    );
    const ERROR_INVALID_ADDRESS_SPACE: CUresult = CUresult::Err(
        CUerror::INVALID_ADDRESS_SPACE,
    );
    const ERROR_INVALID_PC: CUresult = CUresult::Err(CUerror::INVALID_PC);
    const ERROR_LAUNCH_FAILED: CUresult = CUresult::Err(CUerror::LAUNCH_FAILED);
    const ERROR_COOPERATIVE_LAUNCH_TOO_LARGE: CUresult = CUresult::Err(
        CUerror::COOPERATIVE_LAUNCH_TOO_LARGE,
    );
    const ERROR_NOT_PERMITTED: CUresult = CUresult::Err(CUerror::NOT_PERMITTED);
    const ERROR_NOT_SUPPORTED: CUresult = CUresult::Err(CUerror::NOT_SUPPORTED);
    const ERROR_SYSTEM_NOT_READY: CUresult = CUresult::Err(CUerror::SYSTEM_NOT_READY);
    const ERROR_SYSTEM_DRIVER_MISMATCH: CUresult = CUresult::Err(
        CUerror::SYSTEM_DRIVER_MISMATCH,
    );
    const ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE: CUresult = CUresult::Err(
        CUerror::COMPAT_NOT_SUPPORTED_ON_DEVICE,
    );
    const ERROR_MPS_CONNECTION_FAILED: CUresult = CUresult::Err(
        CUerror::MPS_CONNECTION_FAILED,
    );
    const ERROR_MPS_RPC_FAILURE: CUresult = CUresult::Err(CUerror::MPS_RPC_FAILURE);
    const ERROR_MPS_SERVER_NOT_READY: CUresult = CUresult::Err(
        CUerror::MPS_SERVER_NOT_READY,
    );
    const ERROR_MPS_MAX_CLIENTS_REACHED: CUresult = CUresult::Err(
        CUerror::MPS_MAX_CLIENTS_REACHED,
    );
    const ERROR_MPS_MAX_CONNECTIONS_REACHED: CUresult = CUresult::Err(
        CUerror::MPS_MAX_CONNECTIONS_REACHED,
    );
    const ERROR_MPS_CLIENT_TERMINATED: CUresult = CUresult::Err(
        CUerror::MPS_CLIENT_TERMINATED,
    );
    const ERROR_CDP_NOT_SUPPORTED: CUresult = CUresult::Err(CUerror::CDP_NOT_SUPPORTED);
    const ERROR_CDP_VERSION_MISMATCH: CUresult = CUresult::Err(
        CUerror::CDP_VERSION_MISMATCH,
    );
    const ERROR_STREAM_CAPTURE_UNSUPPORTED: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_UNSUPPORTED,
    );
    const ERROR_STREAM_CAPTURE_INVALIDATED: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_INVALIDATED,
    );
    const ERROR_STREAM_CAPTURE_MERGE: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_MERGE,
    );
    const ERROR_STREAM_CAPTURE_UNMATCHED: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_UNMATCHED,
    );
    const ERROR_STREAM_CAPTURE_UNJOINED: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_UNJOINED,
    );
    const ERROR_STREAM_CAPTURE_ISOLATION: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_ISOLATION,
    );
    const ERROR_STREAM_CAPTURE_IMPLICIT: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_IMPLICIT,
    );
    const ERROR_CAPTURED_EVENT: CUresult = CUresult::Err(CUerror::CAPTURED_EVENT);
    const ERROR_STREAM_CAPTURE_WRONG_THREAD: CUresult = CUresult::Err(
        CUerror::STREAM_CAPTURE_WRONG_THREAD,
    );
    const ERROR_TIMEOUT: CUresult = CUresult::Err(CUerror::TIMEOUT);
    const ERROR_GRAPH_EXEC_UPDATE_FAILURE: CUresult = CUresult::Err(
        CUerror::GRAPH_EXEC_UPDATE_FAILURE,
    );
    const ERROR_EXTERNAL_DEVICE: CUresult = CUresult::Err(CUerror::EXTERNAL_DEVICE);
    const ERROR_INVALID_CLUSTER_SIZE: CUresult = CUresult::Err(
        CUerror::INVALID_CLUSTER_SIZE,
    );
    const ERROR_FUNCTION_NOT_LOADED: CUresult = CUresult::Err(
        CUerror::FUNCTION_NOT_LOADED,
    );
    const ERROR_INVALID_RESOURCE_TYPE: CUresult = CUresult::Err(
        CUerror::INVALID_RESOURCE_TYPE,
    );
    const ERROR_INVALID_RESOURCE_CONFIGURATION: CUresult = CUresult::Err(
        CUerror::INVALID_RESOURCE_CONFIGURATION,
    );
    const ERROR_UNKNOWN: CUresult = CUresult::Err(CUerror::UNKNOWN);
}
impl CUresultConsts for CUresult {}
#[must_use]
pub type CUresult = ::core::result::Result<(), CUerror>;
const _: fn() = || {
    let _ = std::mem::transmute::<CUresult, u32>;
};
