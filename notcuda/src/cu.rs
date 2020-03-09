use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
use std::os::raw::c_int;
use std::ptr;

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum Result {
    SUCCESS = 0,
    ERROR_INVALID_VALUE = 1,
    ERROR_OUT_OF_MEMORY = 2,
    ERROR_NOT_INITIALIZED = 3,
    ERROR_DEINITIALIZED = 4,
    ERROR_PROFILER_DISABLED = 5,
    ERROR_PROFILER_NOT_INITIALIZED = 6,
    ERROR_PROFILER_ALREADY_STARTED = 7,
    ERROR_PROFILER_ALREADY_STOPPED = 8,
    ERROR_NO_DEVICE = 100,
    ERROR_INVALID_DEVICE = 101,
    ERROR_INVALID_IMAGE = 200,
    ERROR_INVALID_CONTEXT = 201,
    ERROR_CONTEXT_ALREADY_CURRENT = 202,
    ERROR_MAP_FAILED = 205,
    ERROR_UNMAP_FAILED = 206,
    ERROR_ARRAY_IS_MAPPED = 207,
    ERROR_ALREADY_MAPPED = 208,
    ERROR_NO_BINARY_FOR_GPU = 209,
    ERROR_ALREADY_ACQUIRED = 210,
    ERROR_NOT_MAPPED = 211,
    ERROR_NOT_MAPPED_AS_ARRAY = 212,
    ERROR_NOT_MAPPED_AS_POINTER = 213,
    ERROR_ECC_UNCORRECTABLE = 214,
    ERROR_UNSUPPORTED_LIMIT = 215,
    ERROR_CONTEXT_ALREADY_IN_USE = 216,
    ERROR_PEER_ACCESS_UNSUPPORTED = 217,
    ERROR_INVALID_PTX = 218,
    ERROR_INVALID_GRAPHICS_CONTEXT = 219,
    ERROR_NVLINK_UNCORRECTABLE = 220,
    ERROR_JIT_COMPILER_NOT_FOUND = 221,
    ERROR_INVALID_SOURCE = 300,
    ERROR_FILE_NOT_FOUND = 301,
    ERROR_SHARED_OBJECT_SYMBOL_NOT_FOUND = 302,
    ERROR_SHARED_OBJECT_INIT_FAILED = 303,
    ERROR_OPERATING_SYSTEM = 304,
    ERROR_INVALID_HANDLE = 400,
    ERROR_ILLEGAL_STATE = 401,
    ERROR_NOT_FOUND = 500,
    ERROR_NOT_READY = 600,
    ERROR_ILLEGAL_ADDRESS = 700,
    ERROR_LAUNCH_OUT_OF_RESOURCES = 701,
    ERROR_LAUNCH_TIMEOUT = 702,
    ERROR_LAUNCH_INCOMPATIBLE_TEXTURING = 703,
    ERROR_PEER_ACCESS_ALREADY_ENABLED = 704,
    ERROR_PEER_ACCESS_NOT_ENABLED = 705,
    ERROR_PRIMARY_CONTEXT_ACTIVE = 708,
    ERROR_CONTEXT_IS_DESTROYED = 709,
    ERROR_ASSERT = 710,
    ERROR_TOO_MANY_PEERS = 711,
    ERROR_HOST_MEMORY_ALREADY_REGISTERED = 712,
    ERROR_HOST_MEMORY_NOT_REGISTERED = 713,
    ERROR_HARDWARE_STACK_ERROR = 714,
    ERROR_ILLEGAL_INSTRUCTION = 715,
    ERROR_MISALIGNED_ADDRESS = 716,
    ERROR_INVALID_ADDRESS_SPACE = 717,
    ERROR_INVALID_PC = 718,
    ERROR_LAUNCH_FAILED = 719,
    ERROR_COOPERATIVE_LAUNCH_TOO_LARGE = 720,
    ERROR_NOT_PERMITTED = 800,
    ERROR_NOT_SUPPORTED = 801,
    ERROR_SYSTEM_NOT_READY = 802,
    ERROR_SYSTEM_DRIVER_MISMATCH = 803,
    ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE = 804,
    ERROR_STREAM_CAPTURE_UNSUPPORTED = 900,
    ERROR_STREAM_CAPTURE_INVALIDATED = 901,
    ERROR_STREAM_CAPTURE_MERGE = 902,
    ERROR_STREAM_CAPTURE_UNMATCHED = 903,
    ERROR_STREAM_CAPTURE_UNJOINED = 904,
    ERROR_STREAM_CAPTURE_ISOLATION = 905,
    ERROR_STREAM_CAPTURE_IMPLICIT = 906,
    ERROR_CAPTURED_EVENT = 907,
    ERROR_STREAM_CAPTURE_WRONG_THREAD = 908,
    ERROR_TIMEOUT = 909,
    ERROR_GRAPH_EXEC_UPDATE_FAILURE = 910,
    ERROR_UNKNOWN = 999,
}

pub enum DeviceAttribute {
    Static(DeviceStaticAttribute),
    Dynamic(DeviceDynamicAttribute)
}

impl DeviceAttribute {
    pub fn try_new(e: u8) -> Option<DeviceAttribute> {
        DeviceStaticAttribute::try_from(e).map(DeviceAttribute::Static)
            .or_else(|_| DeviceGeneralAttribute::try_from(e).map(DeviceDynamicAttribute::General).map(DeviceAttribute::Dynamic))
            .or_else(|_| DeviceTextureAttribute::try_from(e).map(DeviceDynamicAttribute::Texture).map(DeviceAttribute::Dynamic))
            .ok()
    }
}

#[repr(u8)]
#[derive(TryFromPrimitive)]
#[allow(non_camel_case_types)]
pub enum DeviceStaticAttribute {
    GPU_OVERLAP = 15,
    KERNEL_EXEC_TIMEOUT = 17,
    INTEGRATED = 18,
    COMPUTE_CAPABILITY_MAJOR = 75,
    COMPUTE_CAPABILITY_MINOR = 76,
}

pub enum DeviceDynamicAttribute {
    General(DeviceGeneralAttribute),
    Texture(DeviceTextureAttribute)
}


#[repr(u8)]
#[derive(TryFromPrimitive)]
#[allow(non_camel_case_types)]
pub enum DeviceGeneralAttribute {
    MULTIPROCESSOR_COUNT = 16,
    CAN_MAP_HOST_MEMORY = 19,
    ASYNC_ENGINE_COUNT = 40,
}


#[repr(u8)]
#[derive(TryFromPrimitive)]
#[allow(non_camel_case_types)]
pub enum DeviceTextureAttribute {
    MAXIMUM_TEXTURE1D_WIDTH = 21
}


impl Result {
    pub fn from_l0(result: l0::ze_result_t) -> Result {
        match result {
            l0::ze_result_t::ZE_RESULT_SUCCESS => Result::SUCCESS,
            l0::ze_result_t::ZE_RESULT_ERROR_UNINITIALIZED => Result::ERROR_NOT_INITIALIZED,
            l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ENUMERATION => Result::ERROR_INVALID_VALUE,
            l0::ze_result_t::ZE_RESULT_ERROR_INVALID_ARGUMENT => Result::ERROR_INVALID_VALUE,
            l0::ze_result_t::ZE_RESULT_ERROR_OUT_OF_HOST_MEMORY => Result::ERROR_OUT_OF_MEMORY,
            l0::ze_result_t::ZE_RESULT_ERROR_UNSUPPORTED_FEATURE => Result::ERROR_NOT_SUPPORTED,
            _ => Result::ERROR_UNKNOWN
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Eq)]
pub struct Uuid {
    pub x: [std::os::raw::c_uchar; 16]
}

#[repr(transparent)]
pub struct Device(pub c_int);

#[repr(transparent)]
pub struct DevicePtr(usize);

#[repr(transparent)]
#[derive(Clone, PartialEq)]
pub struct Context(*mut ());
impl Context {
    pub fn null() -> Context {
        Context(ptr::null_mut())
    }
}

#[repr(transparent)]
pub struct Module(*mut ());

#[repr(transparent)]
pub struct Function(*mut ());

#[repr(transparent)]
pub struct Stream(*mut ());

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum JitOption {
    MAX_REGISTERS = 0,
    THREADS_PER_BLOCK = 1,
    WALL_TIME = 2,
    INFO_LOG_BUFFER = 3,
    INFO_LOG_BUFFER_SIZE_BYTES = 4,
    ERROR_LOG_BUFFER = 5,
    ERROR_LOG_BUFFER_SIZE_BYTES = 6,
    OPTIMIZATION_LEVEL = 7,
    TARGET_FROM_CUCONTEXT = 8,
    TARGET = 9,
    FALLBACK_STRATEGY = 10,
    GENERATE_DEBUG_INFO = 11,
    LOG_VERBOSE = 12,
    GENERATE_LINE_INFO = 13,
    CACHE_MODE = 14,
    NEW_SM3X_OPT = 15,
    FAST_COMPILE = 16,
    GLOBAL_SYMBOL_NAMES = 17,
    GLOBAL_SYMBOL_ADDRESSES = 18,
    GLOBAL_SYMBOL_COUNT = 19,
    NUM_OPTIONS = 20,
}