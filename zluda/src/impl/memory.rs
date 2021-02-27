use super::stream::Stream;
use super::{hipfix, stream};
use crate::hip_call_cuda;
use crate::r#impl::{memcpy2d_from_cuda, GLOBAL_STATE};
use cuda_types::*;
use hip_runtime_sys::*;
use std::{mem, ptr};

pub(crate) unsafe fn alloc(dptr: *mut hipDeviceptr_t, mut bytesize: usize) -> Result<(), CUresult> {
    if dptr == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let zero_buffers = GLOBAL_STATE.get()?.zero_buffers;
    bytesize = hipfix::alloc_round_up(bytesize);
    let mut ptr = mem::zeroed();
    hip_call_cuda!(hipMalloc(&mut ptr, bytesize));
    if zero_buffers {
        hip_call_cuda!(hipMemsetD32(hipDeviceptr_t(ptr), 0, bytesize / 4));
    }
    *dptr = hipDeviceptr_t(ptr);
    Ok(())
}

pub(crate) unsafe fn copy_h_to_d_async(
    dst_device: hipDeviceptr_t,
    src_host: *const std::ffi::c_void,
    byte_count: usize,
    stream: *mut Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda!(hipMemcpyHtoDAsync(
        dst_device,
        src_host as _,
        byte_count,
        hip_stream
    ));
    Ok(())
}

pub(crate) unsafe fn copy_d_to_h_async(
    dst_host: *mut ::std::os::raw::c_void,
    src_device: hipDeviceptr_t,
    byte_count: usize,
    stream: *mut Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda!(hipMemcpyDtoHAsync(
        dst_host, src_device, byte_count, hip_stream
    ));
    Ok(())
}

// TODO: just call hipMemGetAddressRange when HIP fixes handling of NULL args
pub(crate) unsafe fn get_address_range(
    pbase: *mut hipDeviceptr_t,
    psize: *mut usize,
    dptr: hipDeviceptr_t,
) -> hipError_t {
    let mut base = hipDeviceptr_t(ptr::null_mut());
    let mut size = 0;
    let result = hipMemGetAddressRange(&mut base, &mut size, dptr);
    if pbase != ptr::null_mut() {
        *pbase = base;
    }
    if psize != ptr::null_mut() {
        *psize = size;
    }
    result
}

pub(crate) unsafe fn copy3d(copy: *const CUDA_MEMCPY3D) -> Result<(), CUresult> {
    if let Some(copy_desc) = copy.as_ref() {
        hipfix::array::copy3d(copy_desc)
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn copy2d_async(
    copy: *const CUDA_MEMCPY2D,
    stream: *mut Stream,
) -> Result<(), CUresult> {
    if let Some(copy) = copy.as_ref() {
        let hip_stream = stream::as_hip_stream(stream)?;
        let copy = memcpy2d_from_cuda(copy);
        hip_call_cuda!(hipMemcpyParam2DAsync(&copy, hip_stream));
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn copy3d_async(
    copy: *const CUDA_MEMCPY3D,
    stream: *mut Stream,
) -> Result<(), CUresult> {
    if let Some(copy) = copy.as_ref() {
        let hip_stream = stream::as_hip_stream(stream)?;
        hipfix::array::copy3d_async(hip_stream, copy)?;
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn copy2d(copy: *const CUDA_MEMCPY2D) -> hipError_t {
    if let Some(copy) = copy.as_ref() {
        let copy = memcpy2d_from_cuda(copy);
        hipMemcpyParam2D(&copy)
    } else {
        hipError_t::hipErrorInvalidValue
    }
}

pub(crate) unsafe fn copy2d_unaligned(copy: *const CUDA_MEMCPY2D) -> hipError_t {
    if let Some(copy) = copy.as_ref() {
        let copy = memcpy2d_from_cuda(copy);
        hipDrvMemcpy2DUnaligned(&copy)
    } else {
        hipError_t::hipErrorInvalidValue
    }
}

pub(crate) unsafe fn set_d8_async(
    dst_device: hipDeviceptr_t,
    uc: ::std::os::raw::c_uchar,
    n: usize,
    stream: *mut stream::Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda!(hipMemsetD8Async(dst_device, uc, n, hip_stream));
    Ok(())
}

pub(crate) unsafe fn set_d32_async(
    dst_device: hipDeviceptr_t,
    uc: ::std::os::raw::c_uint,
    n: usize,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let hip_stream = stream::as_hip_stream(stream)?;
    hip_call_cuda!(hipMemsetD32Async(dst_device, uc as i32, n, hip_stream));
    Ok(())
}

pub(crate) unsafe fn host_get_device_pointer(
    pdptr: *mut hipDeviceptr_t,
    p: *mut ::std::os::raw::c_void,
    flags: ::std::os::raw::c_uint,
) -> hipError_t {
    hipHostGetDevicePointer(pdptr as _, p, flags)
}

pub(crate) unsafe fn copy_dtd_async(
    dst_device: hipDeviceptr_t,
    src_device: hipDeviceptr_t,
    byte_count: usize,
    stream: *mut stream::Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(stream, default_stream_per_thread)?;
    hip_call_cuda!(hipMemcpyDtoDAsync(
        dst_device, src_device, byte_count, hip_stream
    ));
    Ok(())
}

pub(crate) unsafe fn copy_async(
    dst: hipDeviceptr_t,
    src: hipDeviceptr_t,
    byte_count: usize,
    h_stream: *mut stream::Stream,
    default_stream_per_thread: bool,
) -> Result<(), CUresult> {
    let hip_stream = hipfix::as_hip_stream_per_thread(h_stream, default_stream_per_thread)?;
    hip_call_cuda!(hipMemcpyAsync(
        dst.0,
        src.0,
        byte_count,
        hipMemcpyKind::hipMemcpyDefault,
        hip_stream
    ));
    Ok(())
}

pub(crate) unsafe fn free_async(
    dptr: hipDeviceptr_t,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let hip_stream = stream::as_hip_stream(stream)?;
    hip_call_cuda! { hipFreeAsync(dptr.0, hip_stream) };
    Ok(())
}

pub(crate) unsafe fn prefetch_async(
    dev_ptr: hipDeviceptr_t,
    count: usize,
    dst_device: hipDevice_t,
    stream: *mut stream::Stream,
) -> Result<(), CUresult> {
    let hip_stream = stream::as_hip_stream(stream)?;
    hip_call_cuda! { hipMemPrefetchAsync(dev_ptr.0, count, dst_device, hip_stream) };
    Ok(())
}

pub(crate) unsafe fn set_d8_ptds(
    dst_device: hipDeviceptr_t,
    uc: ::std::os::raw::c_uchar,
    byte_size: usize,
) -> hipError_t {
    let byte_size = hipfix::alloc_round_up(byte_size);
    let int_size = byte_size / 4;
    let value = i32::from_ne_bytes([uc, uc, uc, uc]);
    hipMemset_spt(dst_device.0, value, int_size)
}
