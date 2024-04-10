use cuda_types::*;
use hip_runtime_sys::*;
use std::{mem, ptr};

use crate::hip_call_cuda;

use super::{hipfix, FromCuda};

pub(crate) unsafe fn create(
    result: *mut hipSurfaceObject_t,
    p_res_desc: *const CUDA_RESOURCE_DESC,
) -> Result<(), CUresult> {
    if p_res_desc == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let desc = to_surface_desc(*p_res_desc)?;
    let mut surf_obj = mem::zeroed();
    hip_call_cuda!(hipCreateSurfaceObject(&mut surf_obj, &desc));
    if desc.resType != hipResourceType::hipResourceTypeArray {
        panic!()
    }
    let format_size = format_size((&*desc.res.array.array).Format);
    let channels = (&*desc.res.array.array).NumChannels;
    let pixel_size = format_size * channels as usize;
    let shift_amount = (pixel_size.trailing_zeros() as usize) << (64 - 3);
    surf_obj = (surf_obj as usize | shift_amount) as _;
    dbg!(surf_obj);
    *result = surf_obj;
    Ok(())
}

pub(crate) fn format_size(f: hipArray_Format) -> usize {
    match f {
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8 => 1,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16
        | hipArray_Format::HIP_AD_FORMAT_HALF => 2,
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32
        | hipArray_Format::HIP_AD_FORMAT_FLOAT => 4,
        _ => panic!(),
    }
}

unsafe fn to_surface_desc(res_desc: CUDA_RESOURCE_DESC) -> Result<hipResourceDesc, CUresult> {
    let res_type = mem::transmute(res_desc.resType);
    let res: hipResourceDesc__bindgen_ty_1 = match res_desc.resType {
        CUresourcetype::CU_RESOURCE_TYPE_ARRAY => hipResourceDesc__bindgen_ty_1 {
            array: hipResourceDesc__bindgen_ty_1__bindgen_ty_1 {
                array: hipfix::array::get(res_desc.res.array.hArray),
            },
        },
        CUresourcetype::CU_RESOURCE_TYPE_MIPMAPPED_ARRAY => hipResourceDesc__bindgen_ty_1 {
            mipmap: hipResourceDesc__bindgen_ty_1__bindgen_ty_2 {
                mipmap: mem::transmute(res_desc.res.mipmap.hMipmappedArray),
            },
        },
        CUresourcetype::CU_RESOURCE_TYPE_LINEAR => hipResourceDesc__bindgen_ty_1 {
            linear: hipResourceDesc__bindgen_ty_1__bindgen_ty_3 {
                devPtr: res_desc.res.linear.devPtr.0,
                desc: channel_format_desc(
                    FromCuda::from_cuda(res_desc.res.linear.format),
                    res_desc.res.linear.numChannels,
                )?,
                sizeInBytes: res_desc.res.linear.sizeInBytes,
            },
        },
        CUresourcetype::CU_RESOURCE_TYPE_PITCH2D => hipResourceDesc__bindgen_ty_1 {
            pitch2D: hipResourceDesc__bindgen_ty_1__bindgen_ty_4 {
                devPtr: res_desc.res.pitch2D.devPtr.0,
                desc: channel_format_desc(
                    FromCuda::from_cuda(res_desc.res.pitch2D.format),
                    res_desc.res.pitch2D.numChannels,
                )?,
                width: res_desc.res.pitch2D.width,
                height: res_desc.res.pitch2D.height,
                pitchInBytes: res_desc.res.pitch2D.pitchInBytes,
            },
        },
        _ => todo!(),
    };
    Ok(hipResourceDesc {
        resType: res_type,
        res,
    })
}

fn channel_format_desc(
    format: hipArray_Format,
    num_channels: u32,
) -> Result<hipChannelFormatDesc, CUresult> {
    let mut bits = match num_channels {
        1 => (1, 0, 0, 0),
        2 => (1, 1, 0, 0),
        3 => (1, 1, 1, 0),
        4 => (1, 1, 1, 1),
        _ => return Err(CUresult::CUDA_ERROR_INVALID_VALUE),
    };
    let (kind, bit_width) = match format {
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT8 => {
            (hipChannelFormatKind::hipChannelFormatKindUnsigned, u8::BITS)
        }
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT16 => (
            hipChannelFormatKind::hipChannelFormatKindUnsigned,
            u16::BITS,
        ),
        hipArray_Format::HIP_AD_FORMAT_UNSIGNED_INT32 => (
            hipChannelFormatKind::hipChannelFormatKindUnsigned,
            u32::BITS,
        ),
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT8 => {
            (hipChannelFormatKind::hipChannelFormatKindSigned, i8::BITS)
        }
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT16 => {
            (hipChannelFormatKind::hipChannelFormatKindSigned, i16::BITS)
        }
        hipArray_Format::HIP_AD_FORMAT_SIGNED_INT32 => {
            (hipChannelFormatKind::hipChannelFormatKindSigned, i32::BITS)
        }
        hipArray_Format::HIP_AD_FORMAT_HALF => (
            hipChannelFormatKind::hipChannelFormatKindFloat,
            mem::size_of::<u16>() as u32 * u8::BITS,
        ),
        hipArray_Format::HIP_AD_FORMAT_FLOAT => (
            hipChannelFormatKind::hipChannelFormatKindFloat,
            mem::size_of::<f32>() as u32 * u8::BITS,
        ),
        _ => return Err(CUresult::CUDA_ERROR_INVALID_VALUE),
    };
    bits.0 *= bit_width;
    bits.1 *= bit_width;
    bits.2 *= bit_width;
    bits.3 *= bit_width;
    Ok(hipChannelFormatDesc {
        x: bits.0 as i32,
        y: bits.0 as i32,
        z: bits.0 as i32,
        w: bits.0 as i32,
        f: kind,
    })
}
