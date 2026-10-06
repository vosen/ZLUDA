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

pub(crate) unsafe fn object_create(
    p_tex_object: *mut hipSurfaceObject_t,
    p_res_desc: &HIP_RESOURCE_DESC,
) -> hipError_t {
    let converted = convert_resource_desc(p_res_desc)?;
    hipCreateSurfaceObject(p_tex_object, &converted)
}

pub(crate) unsafe fn object_destroy(tex_object: hipSurfaceObject_t) -> hipError_t {
    hipDestroySurfaceObject(tex_object)
}
