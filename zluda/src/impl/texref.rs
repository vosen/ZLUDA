use super::hipfix;
use crate::hip_call_cuda;
use cuda_types::*;
use hip_runtime_sys::*;
use std::{mem, ptr};

// TODO: remove this when HIP starts handling NULL here gracefully
pub(crate) unsafe fn set_address(
    byte_offset: *mut usize,
    tex_ref: *mut textureReference,
    dptr: hipDeviceptr_t,
    bytes: usize,
) -> hipError_t {
    if dptr.0 == ptr::null_mut() {
        return hipUnbindTexture(tex_ref);
    }
    let mut unused = 0;
    hipTexRefSetAddress(
        if byte_offset == ptr::null_mut() {
            &mut unused
        } else {
            byte_offset
        },
        tex_ref,
        dptr,
        bytes,
    )
}

// TODO replace with HIP call once HIP fixes it
pub(crate) unsafe fn get_max_anisotropy(
    pmax_aniso: *mut i32,
    tex_ref: *mut textureReference,
) -> hipError_t {
    if pmax_aniso == ptr::null_mut() || tex_ref == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    *pmax_aniso = (*tex_ref).maxAnisotropy as i32;
    hipError_t::hipSuccess
}

// TODO replace with HIP call once HIP fixes it
pub(crate) unsafe fn get_mipmap_filter_mode(
    pfm: *mut hipTextureFilterMode,
    tex_ref: *mut textureReference,
) -> hipError_t {
    if pfm == ptr::null_mut() || tex_ref == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    *pfm = (*tex_ref).mipmapFilterMode;
    hipError_t::hipSuccess
}

// TODO replace with HIP call once HIP fixes it
pub(crate) unsafe fn get_mipmap_level_bias(
    pbias: *mut f32,
    tex_ref: *mut textureReference,
) -> hipError_t {
    if pbias == ptr::null_mut() || tex_ref == ptr::null_mut() {
        return hipError_t::hipErrorInvalidValue;
    }
    *pbias = (*tex_ref).mipmapLevelBias;
    hipError_t::hipSuccess
}

// TODO replace with HIP call once HIP fixes it
pub(crate) unsafe fn get_mipmap_level_clamp(
    min_mipmap_level_clamp: *mut f32,
    max_mipmap_level_clamp: *mut f32,
    tex_ref: *mut textureReference,
) -> hipError_t {
    if min_mipmap_level_clamp == ptr::null_mut()
        || max_mipmap_level_clamp == ptr::null_mut()
        || tex_ref == ptr::null_mut()
    {
        return hipError_t::hipErrorInvalidValue;
    }
    *min_mipmap_level_clamp = (*tex_ref).minMipmapLevelClamp;
    *max_mipmap_level_clamp = (*tex_ref).maxMipmapLevelClamp;
    hipError_t::hipSuccess
}

// HIP_TRSA_OVERRIDE_FORMAT is required but does nothing
// HIP team refuses to fix it
pub(crate) unsafe fn set_array(
    texref: *mut textureReference,
    array: CUarray,
    flags: u32,
) -> Result<(), CUresult> {
    if (flags & !1u32) != 0 {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    let array = hipfix::array::get(array);
    if let Some(array) = array.as_ref() {
        hip_call_cuda!(hipTexRefSetFormat(
            texref,
            hipfix::get_broken_format(array.textureType, array.Format),
            array.NumChannels as i32,
        ));
        hip_call_cuda!(hipTexRefSetArray(texref, array, HIP_TRSA_OVERRIDE_FORMAT));
        Ok(())
    } else {
        Err(CUresult::CUDA_ERROR_INVALID_VALUE)
    }
}

unsafe fn reset(tex_ref: *mut textureReference) -> Result<(), CUresult> {
    if tex_ref == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let mut res_desc = mem::zeroed();
    hip_call_cuda!(hipGetTextureObjectResourceDesc(
        &mut res_desc,
        (*tex_ref).textureObject
    ));
    match res_desc.resType {
        hipResourceType::hipResourceTypeArray => {
            let array = res_desc.res.array.array;
            if array != ptr::null_mut() {
                hip_call_cuda!(hipTexRefSetArray(tex_ref, array, HIP_TRSA_OVERRIDE_FORMAT));
            }
        }
        hipResourceType::hipResourceTypeLinear => {
            let linear = res_desc.res.linear;
            if linear.devPtr != ptr::null_mut() && linear.sizeInBytes != 0 {
                let mut unused = 0usize;
                hip_call_cuda!(hipTexRefSetAddress(
                    &mut unused,
                    tex_ref,
                    hipDeviceptr_t(linear.devPtr),
                    linear.sizeInBytes
                ))
            }
        }
        hipResourceType::hipResourceTypePitch2D => {
            let pitch_2d: hipResourceDesc__bindgen_ty_1__bindgen_ty_4 = res_desc.res.pitch2D;
            let (format, channels) = from_channel_format_desc(pitch_2d.desc)?;
            let desc = HIP_ARRAY_DESCRIPTOR {
                Width: pitch_2d.width,
                Height: pitch_2d.height,
                Format: format,
                NumChannels: channels,
            };
            hip_call_cuda!(hipTexRefSetAddress2D(
                tex_ref,
                &desc,
                hipDeviceptr_t(pitch_2d.devPtr),
                pitch_2d.pitchInBytes
            ));
        }
        hipResourceType::hipResourceTypeMipmappedArray => {
            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED)
        }
        _ => return Err(CUresult::CUDA_ERROR_INVALID_VALUE),
    }
    Ok(())
}

fn from_channel_format_desc(
    desc: hipChannelFormatDesc,
) -> Result<(hipArray_Format, u32), CUresult> {
    if desc.x != desc.y || desc.x != desc.z || desc.x != desc.w {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    let num_channels =
        (desc.x != 0) as u32 + (desc.y != 0) as u32 + (desc.z != 0) as u32 + (desc.w != 0) as u32;
    let format = match (desc.f, desc.x) {
        (hipChannelFormatKind::hipChannelFormatKindUnsigned, 8) => {
            hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        }
        (hipChannelFormatKind::hipChannelFormatKindUnsigned, 16) => {
            hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
        }
        (hipChannelFormatKind::hipChannelFormatKindUnsigned, 32) => {
            hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32
        }
        (hipChannelFormatKind::hipChannelFormatKindSigned, 8) => {
            hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8
        }
        (hipChannelFormatKind::hipChannelFormatKindSigned, 16) => {
            hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        }
        (hipChannelFormatKind::hipChannelFormatKindSigned, 32) => {
            hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32
        }
        (hipChannelFormatKind::hipChannelFormatKindFloat, 16) => {
            hipArray_Format::HIP_AD_FORMAT_HALF
        }
        (hipChannelFormatKind::hipChannelFormatKindFloat, 32) => {
            hipArray_Format::HIP_AD_FORMAT_FLOAT
        }
        _ => return Err(CUresult::CUDA_ERROR_INVALID_VALUE),
    };
    Ok((format, num_channels))
}

pub(crate) unsafe fn set_address_mode(
    tex_ref: *mut textureReference,
    dim: i32,
    am: hipTextureAddressMode,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetAddressMode(tex_ref, dim, am));
    reset(tex_ref)
}

pub(crate) unsafe fn set_filter_mode(
    tex_ref: *mut textureReference,
    fm: hipTextureFilterMode,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetFilterMode(tex_ref, fm));
    reset(tex_ref)
}

pub(crate) unsafe fn set_flags(tex_ref: *mut textureReference, flags: u32) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetFlags(tex_ref, flags));
    reset(tex_ref)
}

pub(crate) unsafe fn set_format(
    tex_ref: *mut textureReference,
    fmt: hipArray_Format,
    num_packed_components: i32,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetFormat(tex_ref, fmt, num_packed_components));
    reset(tex_ref)
}

pub(crate) unsafe fn set_max_anisotropy(
    tex_ref: *mut textureReference,
    max_aniso: u32,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetMaxAnisotropy(tex_ref, max_aniso));
    reset(tex_ref)
}

pub(crate) unsafe fn set_mipmap_filter_mode(
    tex_ref: *mut textureReference,
    fm: hipTextureFilterMode,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetMipmapFilterMode(tex_ref, fm));
    reset(tex_ref)
}

pub(crate) unsafe fn set_mipmap_level_bias(
    tex_ref: *mut textureReference,
    bias: f32,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetMipmapLevelBias(tex_ref, bias));
    reset(tex_ref)
}

pub(crate) unsafe fn set_mipmap_level_clamp(
    tex_ref: *mut textureReference,
    min_mipmap_level_clamp: f32,
    max_mipmap_level_clamp: f32,
) -> Result<(), CUresult> {
    hip_call_cuda!(hipTexRefSetMipmapLevelClamp(
        tex_ref,
        min_mipmap_level_clamp,
        max_mipmap_level_clamp
    ));
    reset(tex_ref)
}
