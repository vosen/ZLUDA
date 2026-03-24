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
