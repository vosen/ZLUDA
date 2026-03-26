use crate::r#impl::hipfix;
use hip_runtime_sys::*;

pub(crate) unsafe fn object_create(
    p_tex_object: *mut hipTextureObject_t,
    p_res_desc: *const HIP_RESOURCE_DESC,
    p_tex_desc: *const HIP_TEXTURE_DESC,
    p_res_view_desc: *const HIP_RESOURCE_VIEW_DESC,
) -> hipError_t {
    hipTexObjectCreate(p_tex_object, p_res_desc, p_tex_desc, p_res_view_desc)
}

pub(crate) unsafe fn object_destroy(tex_object: hipTextureObject_t) -> hipError_t {
    hipDestroyTextureObject(tex_object)
}

pub(crate) unsafe fn ref_set_array(
    texref: *mut textureReference,
    array: hipArray_t,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    hipTexRefSetArray(texref, array, flags)
}

pub(crate) unsafe fn ref_set_flags(
    raw_texref: *mut textureReference,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> (u32, i32, i32) {
        (texref.readMode.0, texref.normalized, texref.sRGB)
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFlags(raw_texref, flags)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_filter_mode(
    raw_texref: *mut textureReference,
    filter_mode: hipTextureFilterMode,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> u32 {
        texref.filterMode.0
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFilterMode(raw_texref, filter_mode)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_address_mode(
    raw_texref: *mut textureReference,
    dim: i32,
    address_mode: hipTextureAddressMode,
) -> hipError_t {
    fn get_flags(texref: &textureReference, dim: i32) -> u32 {
        texref.addressMode[dim as usize].0
    }
    if dim < 0 || dim > 2 {
        return Err(hipErrorCode_t::InvalidValue);
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref, dim);
    hipTexRefSetAddressMode(raw_texref, dim, address_mode)?;
    let post_flags = get_flags(texref, dim);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}

pub(crate) unsafe fn ref_set_format(
    raw_texref: *mut textureReference,
    format: hipArray_Format,
    num_components: ::core::ffi::c_int,
) -> hipError_t {
    fn get_flags(texref: &textureReference) -> (u32, u32) {
        (texref.format.0, texref.numChannels as u32)
    }
    let texref = raw_texref.as_ref().ok_or(hipErrorCode_t::InvalidValue)?;
    let pre_flags = get_flags(texref);
    hipTexRefSetFormat(raw_texref, format, num_components)?;
    let post_flags = get_flags(texref);
    if pre_flags != post_flags {
        hipfix::refresh_texref(raw_texref)?;
    }
    Ok(())
}
