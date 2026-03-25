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
    texref: *mut textureReference,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    hipTexRefSetFlags(texref, flags)
}

pub(crate) unsafe fn ref_set_filter_mode(
    texref: *mut textureReference,
    filter_mode: hipTextureFilterMode,
) -> hipError_t {
    hipTexRefSetFilterMode(texref, filter_mode)
}

pub(crate) unsafe fn ref_set_address_mode(
    texref: *mut textureReference,
    dim: i32,
    address_mode: hipTextureAddressMode,
) -> hipError_t {
    hipTexRefSetAddressMode(texref, dim, address_mode)
}

pub(crate) unsafe fn ref_set_format(
    texref: *mut textureReference,
    format: hipArray_Format,
    num_components: ::core::ffi::c_int,
) -> hipError_t {
    hipTexRefSetFormat(texref, format, num_components)
}
