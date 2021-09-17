use std::{
    ffi::{c_void, CStr},
    mem, ptr, slice,
};

use hip_runtime_sys::{hipCtxGetDevice, hipError_t, hipGetDeviceProperties};

use crate::{
    cuda::{CUjitInputType, CUjit_option, CUlinkState, CUresult},
    hip_call,
};

use super::module::{self, SpirvModule};

struct LinkState {
    modules: Vec<SpirvModule>,
    result: Option<Vec<u8>>,
}

pub(crate) unsafe fn create(
    num_options: u32,
    options: *mut CUjit_option,
    option_values: *mut *mut c_void,
    state_out: *mut CUlinkState,
) -> CUresult {
    if state_out == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let state = Box::new(LinkState {
        modules: Vec::new(),
        result: None,
    });
    *state_out = mem::transmute(state);
    CUresult::CUDA_SUCCESS
}

pub(crate) unsafe fn add_data(
    state: CUlinkState,
    type_: CUjitInputType,
    data: *mut c_void,
    size: usize,
    name: *const i8,
    num_options: u32,
    options: *mut CUjit_option,
    option_values: *mut *mut c_void,
) -> Result<(), hipError_t> {
    if state == ptr::null_mut() {
        return Err(hipError_t::hipErrorInvalidValue);
    }
    let state: *mut LinkState = mem::transmute(state);
    let state = &mut *state;
    // V-RAY specific hack
    if state.modules.len() == 2 {
        return Err(hipError_t::hipSuccess);
    }
    let spirv_data = SpirvModule::new_raw(data as *const _)?;
    state.modules.push(spirv_data);
    Ok(())
}

pub(crate) unsafe fn complete(
    state: CUlinkState,
    cubin_out: *mut *mut c_void,
    size_out: *mut usize,
) -> Result<(), hipError_t> {
    let mut dev = 0;
    hip_call! { hipCtxGetDevice(&mut dev) };
    let mut props = unsafe { mem::zeroed() };
    hip_call! { hipGetDeviceProperties(&mut props, dev) };
    let state: &mut LinkState = mem::transmute(state);
    let spirv_bins = state.modules.iter().map(|m| &m.binaries[..]);
    let should_link_ptx_impl = state.modules.iter().find_map(|m| m.should_link_ptx_impl);
    let mut arch_binary = module::compile_amd(&props, spirv_bins, should_link_ptx_impl)
        .map_err(|_| hipError_t::hipErrorUnknown)?;
    let ptr = arch_binary.as_mut_ptr();
    let size = arch_binary.len();
    state.result = Some(arch_binary);
    *cubin_out = ptr as _;
    *size_out = size;
    Ok(())
}

pub(crate) unsafe fn destroy(state: CUlinkState) -> CUresult {
    let state: Box<LinkState> = mem::transmute(state);
    CUresult::CUDA_SUCCESS
}
