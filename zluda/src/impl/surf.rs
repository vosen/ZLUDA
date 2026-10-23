use crate::r#impl::{context, hipfix};
use cuda_types::cuda::{CUerror, CUsurfObject, CUsurfref};
use hip_runtime_sys::*;
use std::mem::{self, offset_of};
use zluda_common::FromCuda;

#[repr(C)]
pub(crate) struct SurfRefData {
    pub(crate) base: *mut textureReference,
    pub(crate) devptr: hipDeviceptr_t,
}

unsafe impl Send for SurfRefData {}
unsafe impl Sync for SurfRefData {}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, CUsurfref, E>
    for &'a SurfRefData
{
    fn from_cuda(surf_ref: &'a CUsurfref) -> Result<Self, E> {
        if surf_ref.is_null() {
            return Err(E::INVALID_VALUE);
        }
        Ok(unsafe { mem::transmute(*surf_ref) })
    }
}

impl<'a, E: zluda_common::CudaErrorType> zluda_common::FromCuda<'a, *mut CUsurfref, E>
    for &'a mut &'a SurfRefData
{
    fn from_cuda(surf_ref: &'a *mut CUsurfref) -> Result<Self, E> {
        let surf_ref = unsafe { surf_ref.as_mut() }.ok_or_else(|| E::INVALID_VALUE)?;
        Ok(unsafe { mem::transmute(surf_ref) })
    }
}

fn convert_resource_desc(desc: &HIP_RESOURCE_DESC) -> Result<hipResourceDesc, hipErrorCode_t> {
    let (type_, desc) = match desc.resType {
        hipResourcetype::HIP_RESOURCE_TYPE_ARRAY => (
            hipResourceType::hipResourceTypeArray,
            hipResourceDesc__bindgen_ty_1 {
                array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1 {
                    array: unsafe { desc.res.array.hArray },
                },
            },
        ),
        hipResourcetype::HIP_RESOURCE_TYPE_MIPMAPPED_ARRAY => (
            hipResourceType::hipResourceTypeMipmappedArray,
            hipResourceDesc__bindgen_ty_1 {
                mipmap: hipResourceDesc__bindgen_ty_1__bindgen_ty_2 {
                    mipmap: unsafe { desc.res.mipmap.hMipmappedArray },
                },
            },
        ),
        hipResourcetype::HIP_RESOURCE_TYPE_LINEAR => {
            let linear = unsafe { desc.res.linear };
            let format_kind = format_kind(linear.format)?;
            let channel_bits = channel_bits(linear.format)?;
            let desc = hipChannelFormatDesc {
                x: channel_bits,
                y: if linear.numChannels > 1 {
                    channel_bits
                } else {
                    0
                },
                z: if linear.numChannels > 2 {
                    channel_bits
                } else {
                    0
                },
                w: if linear.numChannels > 3 {
                    channel_bits
                } else {
                    0
                },
                f: format_kind,
            };
            (
                hipResourceType::hipResourceTypeLinear,
                hipResourceDesc__bindgen_ty_1 {
                    linear: hipResourceDesc__bindgen_ty_1__bindgen_ty_3 {
                        devPtr: linear.devPtr.0,
                        desc,
                        sizeInBytes: linear.sizeInBytes,
                    },
                },
            )
        }
        hipResourcetype::HIP_RESOURCE_TYPE_PITCH2D => {
            let pitch2d = unsafe { desc.res.pitch2D };
            let format_kind = format_kind(pitch2d.format)?;
            let channel_bits = channel_bits(pitch2d.format)?;
            let desc = hipChannelFormatDesc {
                x: channel_bits,
                y: if pitch2d.numChannels > 1 {
                    channel_bits
                } else {
                    0
                },
                z: if pitch2d.numChannels > 2 {
                    channel_bits
                } else {
                    0
                },
                w: if pitch2d.numChannels > 3 {
                    channel_bits
                } else {
                    0
                },
                f: format_kind,
            };
            (
                hipResourceType::hipResourceTypePitch2D,
                hipResourceDesc__bindgen_ty_1 {
                    pitch2D: hipResourceDesc__bindgen_ty_1__bindgen_ty_4 {
                        devPtr: pitch2d.devPtr.0,
                        desc,
                        width: pitch2d.width,
                        height: pitch2d.height,
                        pitchInBytes: pitch2d.pitchInBytes,
                    },
                },
            )
        }
        _ => return Err(hipErrorCode_t::InvalidValue),
    };

    Ok(hipResourceDesc {
        resType: type_,
        res: desc,
    })
}

fn format_kind(format: hipArray_Format) -> Result<hipChannelFormatKind, hipErrorCode_t> {
    Ok(match format {
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        | hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32 => {
            hipChannelFormatKind::hipChannelFormatKindUnsigned
        }
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32 => {
            hipChannelFormatKind::hipChannelFormatKindSigned
        }
        hipArray_Format::HIP_AD_FORMAT_HALF | hipArray_Format::HIP_AD_FORMAT_FLOAT => {
            hipChannelFormatKind::hipChannelFormatKindFloat
        }
        _ => return Err(hipErrorCode_t::InvalidValue),
    })
}

fn channel_bits(format: hipArray_Format) -> Result<i32, hipErrorCode_t> {
    Ok(match format {
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8 => 8,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_HALF => 16,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_FLOAT => 32,
        _ => return Err(hipErrorCode_t::InvalidValue),
    })
}

fn num_channels_for_array(array: hipArray_t) -> Result<(hipArray_Format, u32), hipErrorCode_t> {
    unsafe {
        let mut desc = std::mem::zeroed();
        hipArrayGetDescriptor(&mut desc, array)?;
        Ok((desc.Format, desc.NumChannels))
    }
}

fn num_channels_for_mipmapped_array(
    array: hipMipmappedArray_t,
) -> Result<(hipArray_Format, u32), hipErrorCode_t> {
    unsafe {
        let mut level_array = std::ptr::null_mut();
        hipGetMipmappedArrayLevel(&mut level_array, array, 0)?;
        num_channels_for_array(level_array)
    }
}

fn num_channels_for_resource_desc(
    desc: &HIP_RESOURCE_DESC,
) -> Result<(hipArray_Format, u32), hipErrorCode_t> {
    match desc.resType {
        hipResourcetype::HIP_RESOURCE_TYPE_ARRAY => unsafe {
            num_channels_for_array(desc.res.array.hArray)
        },
        hipResourcetype::HIP_RESOURCE_TYPE_MIPMAPPED_ARRAY => unsafe {
            num_channels_for_mipmapped_array(desc.res.mipmap.hMipmappedArray)
        },
        hipResourcetype::HIP_RESOURCE_TYPE_LINEAR => {
            Ok(unsafe { (desc.res.linear.format, desc.res.linear.numChannels) })
        }
        hipResourcetype::HIP_RESOURCE_TYPE_PITCH2D => {
            Ok(unsafe { (desc.res.pitch2D.format, desc.res.pitch2D.numChannels) })
        }
        _ => Err(hipErrorCode_t::InvalidValue),
    }
}

fn pixel_size_from_format_channels(
    format: hipArray_Format,
    num_channels: u32,
) -> Result<u32, hipErrorCode_t> {
    let bits_per_channel = channel_bits(format)? as u32;
    Ok((bits_per_channel * num_channels) / 8)
}

pub(crate) unsafe fn object_create(
    p_tex_object: &mut CUsurfObject,
    p_res_desc: &HIP_RESOURCE_DESC,
) -> hipError_t {
    let resource_desc = convert_resource_desc(p_res_desc)?;
    let (format, num_channels) = num_channels_for_resource_desc(p_res_desc)?;
    let pixel_size = pixel_size_from_format_channels(format, num_channels)?;
    let mut hip_surfobj = std::mem::zeroed();
    hipCreateSurfaceObject(&mut hip_surfobj, &resource_desc)?;
    *p_tex_object = to_cuda(hip_surfobj, pixel_size)?;
    Ok(())
}

pub(crate) unsafe fn object_destroy(tex_object: CUsurfObject) -> hipError_t {
    hipDestroySurfaceObject(to_hip(tex_object))
}

const RESERVED_SURFACE_BOTTOM_BITS: u32 = 3;

fn to_cuda(hip: hipSurfaceObject_t, pixel_size: u32) -> Result<CUsurfObject, hipErrorCode_t> {
    let hip = hip as u64;
    let bottom_bits = extract_bottom_bits::<RESERVED_SURFACE_BOTTOM_BITS>(hip);
    if bottom_bits != 0 {
        return Err(hipErrorCode_t::Unknown);
    }
    let shift_size = pixel_size.ilog2() as u64;
    Ok(set_bottom_bits::<RESERVED_SURFACE_BOTTOM_BITS>(
        hip, shift_size,
    ))
}

fn to_hip(cuda: CUsurfObject) -> hipSurfaceObject_t {
    ((cuda >> RESERVED_SURFACE_BOTTOM_BITS) << RESERVED_SURFACE_BOTTOM_BITS) as _
}

fn extract_bottom_bits<const N: u32>(x: u64) -> u64 {
    x & ((1 << N) - 1)
}

fn set_bottom_bits<const N: u32>(x: u64, bits: u64) -> u64 {
    x | bits
}

pub(crate) unsafe fn ref_set_array(
    surf_ref: &SurfRefData,
    array: hipArray_t,
    mut flags: ::core::ffi::c_uint,
) -> Result<(), CUerror> {
    flags |= hipfix::HIP_TRSA_OVERRIDE_FORMAT;
    super::tex::ref_set_array(surf_ref.base, array, flags)?;
    if let Some(texref) = surf_ref.base.as_ref() {
        if !texref.textureObject.is_null() {
            let pixel_size =
                pixel_size_from_format_channels(texref.format, texref.numChannels as u32)?;
            let mut cuda_texobj = to_cuda(texref.textureObject as _, pixel_size)?;
            let cu_context = context::get_current_context()?;
            let context: &context::Context = FromCuda::<_, CUerror>::from_cuda(&cu_context)?;
            hipMemcpyHtoDAsync(
                hipDeviceptr_t(
                    surf_ref
                        .devptr
                        .0
                        .add(offset_of!(textureReference, textureObject)),
                ),
                std::ptr::from_mut(&mut cuda_texobj).cast(),
                std::mem::size_of_val(&cuda_texobj),
                context.alloc_stream,
            )?;
            hipStreamSynchronize(context.alloc_stream)?;
        }
    }
    Ok(())
}
