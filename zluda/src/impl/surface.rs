use super::hipfix;
use crate::hip_call_cuda;
use cuda_types::*;
use hip_runtime_sys::*;
use std::{mem, ptr};

// Same as in zluda_ptx_impl.cpp
const IMAGE_RESERVED_TOP_BITS: u32 = 3;

pub(crate) unsafe fn create(
    result: *mut hipSurfaceObject_t,
    p_res_desc: *const CUDA_RESOURCE_DESC,
) -> Result<(), CUresult> {
    if p_res_desc == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let desc = to_surface_desc(*p_res_desc)?;
    // We need to check array format and channel count to set top bits of the surface object.
    // HIP does not support non-Array sources anyway
    if desc.resType != hipResourceType::hipResourceTypeArray {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    let mut surf_obj = mem::zeroed();
    hip_call_cuda!(hipCreateSurfaceObject(&mut surf_obj, &desc));
    let format_size = format_size((&*desc.res.array.array).Format)?;
    let channels = (&*desc.res.array.array).NumChannels;
    let pixel_size = format_size * channels as usize;
    let shift_amount = (pixel_size.trailing_zeros() as usize) << (64 - IMAGE_RESERVED_TOP_BITS);
    surf_obj = (surf_obj as usize | shift_amount) as _;
    *result = surf_obj;
    Ok(())
}

pub(crate) unsafe fn destroy(surf_object: hipSurfaceObject_t) -> hipError_t {
    hipDestroySurfaceObject(
        (((surf_object as usize) << IMAGE_RESERVED_TOP_BITS) >> IMAGE_RESERVED_TOP_BITS) as _,
    )
}

pub(crate) fn format_size(f: hipArray_Format) -> Result<usize, CUresult> {
    Ok(match f {
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8 => 1,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_HALF => 2,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_FLOAT => 4,
        _ => return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    })
}

unsafe fn to_surface_desc(res_desc: CUDA_RESOURCE_DESC) -> Result<hipResourceDesc, CUresult> {
    let res_type = mem::transmute(res_desc.resType);
    let res: hipResourceDesc__bindgen_ty_1 = match res_desc.resType {
        CUresourcetype::CU_RESOURCE_TYPE_ARRAY => hipResourceDesc__bindgen_ty_1 {
            array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1 {
                array: hipfix::array::get(res_desc.res.array.hArray),
            },
        },
        _ => return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    };
    Ok(hipResourceDesc {
        resType: res_type,
        res,
    })
}
