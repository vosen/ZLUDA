use hip_runtime_sys::*;

pub(crate) fn alloc_v2(dptr: *mut hipDeviceptr_t, bytesize: usize) -> hipError_t {
    unsafe { hipMalloc(dptr.cast(), bytesize) }?;
    // TODO: parametrize for non-Geekbench
    unsafe { hipMemsetD8(*dptr, 0, bytesize) }
}

pub(crate) fn free_v2(dptr: hipDeviceptr_t) -> hipError_t {
    unsafe { hipFree(dptr.0) }
}

pub(crate) fn copy_dto_h_v2(
    dst_host: *mut ::core::ffi::c_void,
    src_device: hipDeviceptr_t,
    byte_count: usize,
) -> hipError_t {
    unsafe { hipMemcpyDtoH(dst_host, src_device, byte_count) }
}

pub(crate) fn copy_hto_d_v2(
    dst_device: hipDeviceptr_t,
    src_host: *const ::core::ffi::c_void,
    byte_count: usize,
) -> hipError_t {
    unsafe { hipMemcpyHtoD(dst_device, src_host.cast_mut(), byte_count) }
}

pub(crate) fn get_address_range_v2(
    pbase: *mut hipDeviceptr_t,
    psize: *mut usize,
    dptr: hipDeviceptr_t,
) -> hipError_t {
    unsafe { hipMemGetAddressRange(pbase, psize, dptr) }
}

pub(crate) fn set_d32_v2(dst: hipDeviceptr_t, ui: ::core::ffi::c_uint, n: usize) -> hipError_t {
    unsafe { hipMemsetD32(dst, ui as std::ffi::c_int, n) }
}

pub(crate) fn set_d8_v2(dst: hipDeviceptr_t, value: ::core::ffi::c_uchar, n: usize) -> hipError_t {
    unsafe { hipMemsetD8(dst, value, n) }
}
pub(crate) fn get_info_v2(free: *mut usize, total: *mut usize) -> hipError_t {
    unsafe { hipMemGetInfo(free, total) }
}

pub(crate) unsafe fn free_host(ptr: *mut ::core::ffi::c_void) -> hipError_t {
    hipFreeHost(ptr)
}

pub(crate) unsafe fn host_alloc(
    pp: *mut *mut ::core::ffi::c_void,
    bytesize: usize,
    flags: ::std::os::raw::c_uint,
) -> hipError_t {
    hipHostMalloc(pp, bytesize, flags)
}
