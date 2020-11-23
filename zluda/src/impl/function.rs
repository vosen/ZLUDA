use ::std::os::raw::{c_uint, c_void};
use std::{hint, ptr};

use crate::cuda::CUfunction_attribute;

use super::{stream::Stream, CUresult, GlobalState, HasLivenessCookie, LiveCheck};

pub type Function = LiveCheck<FunctionData>;

impl HasLivenessCookie for FunctionData {
    #[cfg(target_pointer_width = "64")]
    const COOKIE: usize = 0x5e2ab14d5840678e;

    #[cfg(target_pointer_width = "32")]
    const COOKIE: usize = 0x33e6a1e6;

    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn try_drop(&mut self) -> Result<(), CUresult> {
        Ok(())
    }
}

pub struct FunctionData {
    pub base: l0::Kernel<'static>,
    pub arg_size: Vec<usize>,
    pub use_shared_mem: bool,
    pub properties: Option<Box<l0::sys::ze_kernel_properties_t>>,
    pub do_nothing_hack: bool,
}

impl FunctionData {
    fn get_properties(&mut self) -> Result<&l0::sys::ze_kernel_properties_t, l0::sys::ze_result_t> {
        if let None = self.properties {
            self.properties = Some(self.base.get_properties()?)
        }
        match self.properties {
            Some(ref props) => Ok(props.as_ref()),
            None => unsafe { hint::unreachable_unchecked() },
        }
    }
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
    hstream: *mut Stream,
    kernel_params: *mut *mut c_void,
    extra: *mut *mut c_void,
) -> Result<(), CUresult> {
    if f == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    if extra != ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
    }
    GlobalState::lock_stream(hstream, |stream| {
        let func: &mut FunctionData = unsafe { &mut *f }.as_result_mut()?;
        if func.do_nothing_hack {
            return Ok(());
        }
        for (i, arg_size) in func.arg_size.iter().enumerate() {
            unsafe {
                func.base
                    .set_arg_raw(i as u32, *arg_size, *kernel_params.add(i))?
            };
        }
        if func.use_shared_mem {
            unsafe {
                func.base.set_arg_raw(
                    func.arg_size.len() as u32,
                    shared_mem_bytes as usize,
                    ptr::null(),
                )?
            };
        }
        func.base
            .set_group_size(block_dim_x, block_dim_y, block_dim_z)?;
        let mut cmd_list = stream.command_list()?;
        cmd_list.append_launch_kernel(
            &mut func.base,
            &[grid_dim_x, grid_dim_y, grid_dim_z],
            None,
            &mut [],
        )?;
        stream.queue.execute(cmd_list)?;
        Ok(())
    })?
}

pub(crate) fn get_attribute(
    pi: *mut i32,
    attrib: CUfunction_attribute,
    func: *mut Function,
) -> Result<(), CUresult> {
    if pi == ptr::null_mut() || func == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    match attrib {
        CUfunction_attribute::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => {
            let max_threads = GlobalState::lock_function(func, |func| {
                let props = func.get_properties()?;
                Ok::<_, CUresult>(props.maxSubgroupSize * props.maxNumSubgroups)
            })??;
            unsafe { *pi = max_threads as i32 };
            Ok(())
        }
        _ => Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    }
}
