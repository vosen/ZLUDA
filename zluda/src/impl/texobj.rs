use super::hipfix;
use crate::hip_call_cuda;
use cuda_types::*;
use hip_runtime_sys::*;
use std::ptr;

pub(crate) unsafe fn create(
    p_tex_object: *mut hipTextureObject_t,
    p_res_desc: *const CUDA_RESOURCE_DESC,
    p_tex_desc: *const HIP_TEXTURE_DESC,
    p_res_view_desc: *const HIP_RESOURCE_VIEW_DESC,
) -> Result<(), CUresult> {
    if p_res_desc == ptr::null() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    hipfix::array::with_resource_desc(
        p_res_desc,
        p_res_view_desc,
        |p_res_desc, p_res_view_desc| {
            hip_call_cuda!(hipTexObjectCreate(
                p_tex_object,
                p_res_desc,
                p_tex_desc,
                p_res_view_desc
            ));
            Ok(())
        },
    )?
}
