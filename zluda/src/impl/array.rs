use std::{mem, ptr};

use crate::hip_call_cuda;

use super::hipfix;
use cuda_types::*;
use hip_runtime_sys::*;

pub(crate) unsafe fn create_3d(
    array: *mut CUarray,
    allocate_array: *const HIP_ARRAY3D_DESCRIPTOR,
) -> Result<(), CUresult> {
    if let (Some(array_ptr), Some(desc)) = (
        array.as_mut(),
        (allocate_array as *const HIP_ARRAY3D_DESCRIPTOR).as_ref(),
    ) {
        let mut desc = *desc;
        let (hack_flag, format) = hipfix::get_non_broken_format(desc.Format);
        desc.Format = format;
        hipfix::array_3d_create(&mut desc);
        let mut hip_array = mem::zeroed();
        hip_call_cuda!(hipArray3DCreate(&mut hip_array, &mut desc as _));
        (&mut *hip_array).textureType = hack_flag;
        let layered_dimensions = if desc.Flags & hipArrayLayered != 0 {
            if desc.Height == 0 {
                1usize
            } else {
                2
            }
        } else {
            0
        };
        *array_ptr = hipfix::array::to_cuda(hip_array, layered_dimensions);
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn get_descriptor_3d(
    array_descriptor: *mut CUDA_ARRAY3D_DESCRIPTOR,
    array: CUarray,
) -> hipError_t {
    let layered = hipfix::array::get_layered_dimensions(array);
    let mut flags = if layered > 0 { CUDA_ARRAY3D_LAYERED } else { 0 };
    // HIP surfaces are always ld/st capable you want it or not
    flags |= CUDA_ARRAY3D_SURFACE_LDST;
    let array = hipfix::array::get(array);
    if let (Some(array), Some(array_descriptor)) = (array.as_ref(), array_descriptor.as_mut()) {
        *array_descriptor = CUDA_ARRAY3D_DESCRIPTOR {
            Width: array.width as usize,
            Height: array.height as usize,
            Depth: array.depth as usize,
            NumChannels: array.NumChannels,
            Format: mem::transmute(array.Format), // compatible
            Flags: flags,
        };
        hipError_t::hipSuccess
    } else {
        hipError_t::hipErrorInvalidValue
    }
}

pub(crate) unsafe fn create(
    array: *mut *mut CUarray_st,
    desc: *const HIP_ARRAY_DESCRIPTOR,
) -> Result<(), CUresult> {
    if array == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    if let Some(desc) = (desc as *const HIP_ARRAY_DESCRIPTOR).as_ref() {
        let mut desc = *desc;
        let (hack_flag, format) = hipfix::get_non_broken_format(desc.Format);
        desc.Format = format;
        let mut hip_array = ptr::null_mut();
        hip_call_cuda!(hipArrayCreate(&mut hip_array, &desc));
        (&mut *hip_array).textureType = hack_flag;
        *array = hip_array.cast();
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn mipmapped_create(
    p_handle: *mut hipMipmappedArray_t,
    p_mipmapped_array_desc: *const HIP_ARRAY3D_DESCRIPTOR,
    num_mipmap_levels: u32,
) -> hipError_t {
    hipMipmappedArrayCreate(
        p_handle,
        p_mipmapped_array_desc.cast_mut(),
        num_mipmap_levels,
    )
}
