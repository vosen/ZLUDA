use cuda_types::*;
use hip_runtime_sys::*;
use std::ptr;

use super::hipfix;

pub(crate) unsafe fn create(
    p_tex_object: *mut hipTextureObject_t,
    p_res_desc: *const CUDA_RESOURCE_DESC,
    p_tex_desc: *const HIP_TEXTURE_DESC,
    p_res_view_desc: *const HIP_RESOURCE_VIEW_DESC,
) -> hipError_t {
    if p_res_desc == ptr::null() {
        return hipError_t::hipErrorInvalidValue;
    }
    hipfix::array::with_resource_desc(p_res_desc, |p_res_desc| {
        hipTexObjectCreate(p_tex_object, p_res_desc, p_tex_desc, p_res_view_desc)
    })
}
