use hip_runtime_sys::*;

pub(crate) unsafe fn create_v2(
    handle: *mut hipArray_t,
    desc: *const HIP_ARRAY_DESCRIPTOR,
) -> hipError_t {
    hipArrayCreate(handle, desc)
}

pub(crate) unsafe fn destroy(array: hipArray_t) -> hipError_t {
    hipArrayDestroy(array)
}

pub(crate) unsafe fn _3d_create_v2(
    handle: *mut hipArray_t,
    desc: *const HIP_ARRAY3D_DESCRIPTOR,
) -> hipError_t {
    hipArray3DCreate(handle, desc)
}
