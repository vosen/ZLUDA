use crate::{hip_call_cuda, r#impl::hipfix};
use cuda_types::{CUarray, CUresult};
use hip_runtime_sys::*;
use std::ptr;

pub(crate) unsafe fn set_array(
    surfref: *mut textureReference,
    array: CUarray,
    _flags: u32,
) -> Result<(), CUresult> {
    if array == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let array = hipfix::array::get(array);
    let array = array.as_mut().unwrap();
    hip_call_cuda!(hipTexRefSetFormat(
        surfref,
        array.Format,
        array.NumChannels as i32,
    ));
    hip_call_cuda!(hipTexRefSetArray(surfref, array, HIP_TRSA_OVERRIDE_FORMAT));
    Ok(())
}
