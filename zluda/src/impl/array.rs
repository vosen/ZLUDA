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
    mipmapped_array: *mut CUmipmappedArray,
    mipmapped_array_desc: *const HIP_ARRAY3D_DESCRIPTOR,
    num_mipmap_levels: u32,
) -> Result<(), CUresult> {
    if let Some(mipmapped_array_desc) = (mipmapped_array_desc).as_ref() {
        let mut mipmapped_array_desc = *mipmapped_array_desc;
        let (hack_flag, format) = hipfix::get_non_broken_format(mipmapped_array_desc.Format);
        mipmapped_array_desc.Format = format;
        let mut hip_array = ptr::null_mut();
        hip_call_cuda!(hipMipmappedArrayCreate(
            &mut hip_array,
            &mut mipmapped_array_desc,
            num_mipmap_levels
        ));
        if (hip_array as usize & 0b11) != 0 {
            hip_call_cuda!(hipMipmappedArrayDestroy(hip_array));
            return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
        }
        hip_array = (hip_array as usize | hack_flag as usize) as _;
        *mipmapped_array = hip_array.cast();
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

pub(crate) unsafe fn mipmapped_destroy(mipmapped_array: CUmipmappedArray) -> hipError_t {
    let mipmapped_array = hipfix::array::get_mipmapped(mipmapped_array).0;
    hipMipmappedArrayDestroy(mipmapped_array)
}

pub(crate) unsafe fn mipmapped_get_level(
    level_array: *mut CUarray,
    mipmapped_array: CUmipmappedArray,
    level: u32,
) -> Result<(), CUresult> {
    let (mipmapped_array, hack_flag) = hipfix::array::get_mipmapped(mipmapped_array);
    if let Some(mipmapped_array) = mipmapped_array.as_mut() {
        let mut hip_array = mem::zeroed();
        hip_call_cuda!(hipMipmappedArrayGetLevel(
            &mut hip_array,
            mipmapped_array as *mut _,
            level
        ));
        let hip_array_mut = hip_array.as_mut().ok_or(CUresult::CUDA_ERROR_UNKNOWN)?;
        hip_array_mut.textureType = hack_flag;
        *level_array = mem::transmute(hip_array);
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}
