use ::std::os::raw::{c_uint, c_void};
use std::ptr;

use super::{context, device, stream::Stream, CUresult};

pub struct Function {
    pub base: l0::Kernel<'static>,
    pub arg_size: Vec<usize>,
}

pub fn launch_kernel(
    f: *mut Function,
    grid_dim_x: c_uint,
    grid_dim_y: c_uint,
    grid_dim_z: c_uint,
    block_dim_x: c_uint,
    block_dim_y: c_uint,
    block_dim_z: c_uint,
    shared_mem_bytes: c_uint,
    strean: *mut Stream,
    kernel_params: *mut *mut c_void,
    extra: *mut *mut c_void,
) -> Result<(), CUresult> {
    if f == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    if shared_mem_bytes != 0 || strean != ptr::null_mut() || extra != ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    let func = unsafe { &*f };
    for (i, arg_size) in func.arg_size.iter().copied().enumerate() {
        unsafe {
            func.base
                .set_arg_raw(i as u32, arg_size, *kernel_params.add(i))?
        };
    }
    unsafe { &*f }
        .base
        .set_group_size(block_dim_x, block_dim_y, block_dim_z)?;
    device::with_current_exclusive(|dev| {
        let mut cmd_list = l0::CommandList::new(&mut dev.l0_context, &dev.base)?;
        cmd_list.append_launch_kernel(
            &unsafe { &*f }.base,
            &[grid_dim_x, grid_dim_y, grid_dim_z],
            None,
            &mut [],
        )?;
        dev.default_queue.execute(cmd_list)?;
        l0::Result::Ok(())
    })??;
    Ok(())
}
