extern crate lazy_static;
#[cfg(test)]
extern crate paste;
extern crate ptx;

#[allow(warnings)]
pub mod cuda;
pub(crate) mod r#impl;

use crate::r#impl::LiveCheck;
use cuda_types::CUresult;
use hip_common::zluda_ext::{CudaObjectKind, CudaResult};
use r#impl::{context, stream};

const DRIVER_VERSION: i32 = 12020;

#[no_mangle]
pub unsafe extern "C" fn zluda_get_hip_object(
    cuda_object: *mut std::os::raw::c_void,
    kind: CudaObjectKind,
) -> CudaResult<*const std::os::raw::c_void> {
    unsafe fn zluda_get_hip_object_impl(
        cuda_object: *const std::os::raw::c_void,
        kind: CudaObjectKind,
    ) -> Result<*const std::os::raw::c_void, CUresult> {
        match kind {
            CudaObjectKind::Context => {
                let cuda_object = cuda_object as *mut context::Context;
                let ctx = LiveCheck::as_result(cuda_object)?;
                Ok(ctx.device as usize as _)
            }
            CudaObjectKind::Stream => {
                let cuda_object = cuda_object as *mut stream::Stream;
                let stream = stream::as_hip_stream(cuda_object)?;
                Ok(stream as _)
            }
        }
    }
    zluda_get_hip_object_impl(cuda_object, kind).into()
}
