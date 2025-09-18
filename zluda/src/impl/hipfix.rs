// There's a bug in hipDrvPointerGetAttributes where it returns
// HIP_ERROR_INVALID_VALUE if the pointer is null. It works correctly for any
// other invalid pointer
pub(crate) fn get_attributes(
    ptr: hip_runtime_sys::hipDeviceptr_t,
) -> hip_runtime_sys::hipDeviceptr_t {
    if ptr.0.is_null() {
        hip_runtime_sys::hipDeviceptr_t(usize::MAX as _)
    } else {
        ptr
    }
}
