use cuda_types::cuda::CUsurfObject;
use hip_runtime_sys::*;

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

const RESERVED_SURFACE_TOP_BITS: u32 = 3;

fn to_cuda(hip: hipSurfaceObject_t, pixel_size: u32) -> Result<CUsurfObject, hipErrorCode_t> {
    let hip = hip as u64;
    let top_bits = get_top_bits::<RESERVED_SURFACE_TOP_BITS>(hip);
    if top_bits != 0 {
        return Err(hipErrorCode_t::Unknown);
    }
    let shift_size = pixel_size.ilog2() as u64;
    Ok(set_top_bits::<RESERVED_SURFACE_TOP_BITS>(hip, shift_size))
}

fn to_hip(cuda: CUsurfObject) -> hipSurfaceObject_t {
    ((cuda << RESERVED_SURFACE_TOP_BITS) >> RESERVED_SURFACE_TOP_BITS) as _
}

fn get_top_bits<const N: u32>(x: u64) -> u64 {
    x >> (u64::BITS - N)
}

fn set_top_bits<const N: u32>(x: u64, bits: u64) -> u64 {
    let shift = u64::BITS - N;
    (bits << shift) | x
}
