use hip_runtime_sys::*;

// There's a bug in hipDrvPointerGetAttributes where it returns
// HIP_ERROR_INVALID_VALUE if the pointer is null. It works correctly for any
// other invalid pointer
pub(crate) fn get_attributes(
    ptr: hip_runtime_sys::hipDeviceptr_t,
) -> hip_runtime_sys::hipDeviceptr_t {
    if ptr.0.is_null() {
        hip_runtime_sys::hipDeviceptr_t(usize::MAX as _)
    } else {
        ptr
    }
}

const HIP_TRSA_OVERRIDE_FORMAT: u32 = 1;

pub(crate) unsafe fn refresh_texref(raw_texref: *mut textureReference) -> hipError_t {
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let mut res_desc = std::mem::zeroed();
    hipTexObjectGetResourceDesc(&mut res_desc, texref.textureObject)?;
    match res_desc.resType {
        HIPresourcetype::HIP_RESOURCE_TYPE_ARRAY => hipTexRefSetArray(
            raw_texref,
            res_desc.res.array.hArray,
            HIP_TRSA_OVERRIDE_FORMAT,
        ),
        HIPresourcetype::HIP_RESOURCE_TYPE_MIPMAPPED_ARRAY => hipTexRefSetMipmappedArray(
            raw_texref,
            res_desc.res.mipmap.hMipmappedArray,
            HIP_TRSA_OVERRIDE_FORMAT,
        ),
        HIPresourcetype::HIP_RESOURCE_TYPE_LINEAR => hipTexRefSetAddress(
            &mut 0,
            raw_texref,
            res_desc.res.linear.devPtr,
            res_desc.res.linear.sizeInBytes,
        ),
        HIPresourcetype::HIP_RESOURCE_TYPE_PITCH2D => hipTexRefSetAddress2D(
            raw_texref,
            &HIP_ARRAY_DESCRIPTOR {
                Width: res_desc.res.pitch2D.width,
                Height: res_desc.res.pitch2D.height,
                Format: res_desc.res.pitch2D.format,
                NumChannels: res_desc.res.pitch2D.numChannels,
            },
            res_desc.res.pitch2D.devPtr,
            res_desc.res.pitch2D.pitchInBytes,
        ),
        _ => Err(hipErrorCode_t::InvalidValue),
    }
}
