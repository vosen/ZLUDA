use std::{
    ffi::{c_void, CStr},
    mem, ptr, slice,
};

use crate::cuda::{CUjitInputType, CUjit_option, CUlinkState, CUresult};

struct LinkState {
    modules: Vec<String>,
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
) -> CUresult {
    if state == ptr::null_mut() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    let state: *mut LinkState = mem::transmute(state);
    let state = &mut *state;
    // V-RAY specific hack
    if state.modules.len() == 2 {
        return CUresult::CUDA_SUCCESS;
    }
    let ptx = slice::from_raw_parts(data as *mut u8, size);
    state.modules.push(
        CStr::from_bytes_with_nul_unchecked(ptx)
            .to_string_lossy()
            .to_string(),
    );
    CUresult::CUDA_SUCCESS
}

pub(crate) fn complete(
    state: CUlinkState,
    cubin_out: *mut *mut c_void,
    size_out: *mut usize,
) -> CUresult {
    CUresult::CUDA_SUCCESS
}

pub(crate) unsafe fn destroy(state: CUlinkState) -> CUresult {
    let state: Box<LinkState> = mem::transmute(state);
    CUresult::CUDA_SUCCESS
}
