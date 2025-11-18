use hip_runtime_sys::*;

pub(crate) unsafe fn create(event: *mut hipEvent_t, flags: ::core::ffi::c_uint) -> hipError_t {
    // Flag values are compatible between CUDA and HIP for 0,1,2,4
    hipEventCreateWithFlags(event, flags)
}

pub(crate) unsafe fn elapsed_time(
    milliseconds: *mut ::core::ffi::c_float,
    start: hipEvent_t,
    end: hipEvent_t,
) -> hipError_t {
    // Flag values are compatible between CUDA and HIP for 0,1,2,4
    hipEventElapsedTime(milliseconds, start, end)
}

pub(crate) unsafe fn query(event: hipEvent_t) -> hipError_t {
    hipEventQuery(event)
}

pub(crate) unsafe fn destroy_v2(event: hipEvent_t) -> hipError_t {
    hipEventDestroy(event)
}

pub(crate) unsafe fn record(event: hipEvent_t, stream: hipStream_t) -> hipError_t {
    hipEventRecord(event, stream)
}

pub(crate) unsafe fn synchronize(event: hipEvent_t) -> hipError_t {
    hipEventSynchronize(event)
}
