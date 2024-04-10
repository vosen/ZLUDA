use crate::{
    hip_call_cuda,
    r#impl::{hipfix, surface},
};
use cuda_types::{CUarray, CUresult};
use hip_runtime_sys::*;
use std::{mem, ptr};

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
    //assert_eq!(hipError_t::hipSuccess, hipHostGetDevicePointer(&mut dev_ptr, surfref.cast(), 0));
    dbg!(surfref);
    // TODO: clear bits on the old textureobject
    hip_call_cuda!(hipTexRefSetArray(surfref, array, HIP_TRSA_OVERRIDE_FORMAT));
    let format_size = surface::format_size(array.Format);
    let pixel_size = format_size * array.NumChannels as usize;
    let shift_amount = (pixel_size.trailing_zeros() as usize) << (64 - 3);
    let mut surfref = &mut *surfref;
    surfref.textureObject = (surfref.textureObject as usize | shift_amount) as _;
    Ok(())
}
