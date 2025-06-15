use hip_runtime_sys::*;

pub(crate) fn synchronize(stream: hipStream_t) -> hipError_t {
    unsafe { hipStreamSynchronize(stream) }
}
