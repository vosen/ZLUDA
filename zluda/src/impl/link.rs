use super::{context, module, LiveCheck, ZludaObject, GLOBAL_STATE};
use cuda_types::*;
use std::{borrow::Cow, ptr, sync::Mutex};

pub(crate) type LinkState = LiveCheck<LinkStateData>;

impl ZludaObject for LinkStateData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0x0f8acfce25ea71da;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0x5f92e7dc;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn drop_with_result(&mut self, _by_owner: bool) -> Result<(), CUresult> {
        Ok(())
    }
}

pub(crate) struct LinkStateData {
    ptx_modules: Mutex<Vec<Cow<'static, str>>>,
}

pub(crate) unsafe fn add_data(
    state: *mut LinkState,
    type_: CUjitInputType,
    data: *mut ::std::os::raw::c_void,
    mut size: usize,
    _name: *const ::std::os::raw::c_char,
    _num_options: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _option_values: *mut *mut ::std::os::raw::c_void,
) -> Result<(), CUresult> {
    let state = LiveCheck::as_result(state)?;
    match type_ {
        CUjitInputType::CU_JIT_INPUT_PTX => {
            let data = data.cast::<u8>();
            loop {
                if *data.add(size - 1) == 0 {
                    size -= 1;
                } else {
                    break;
                }
            }
            let buffer = std::slice::from_raw_parts(data.cast::<u8>(), size);
            let buffer =
                std::str::from_utf8(buffer).map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?;
            let ptx = buffer.to_string();
            let mut modules = state
                .ptx_modules
                .lock()
                .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
            modules.push(Cow::Owned(ptx));
            Ok(())
        }
        // Right now only user of this data type is
        // V-Ray, which passes CUDA Runtime archive
        // that is not used anyway
        CUjitInputType::CU_JIT_INPUT_LIBRARY => Ok(()),
        _ => Err(CUresult::CUDA_ERROR_NOT_SUPPORTED),
    }
}

pub(crate) unsafe fn complete(
    state: *mut LinkState,
    cubin_out: *mut *mut ::std::os::raw::c_void,
    size_out: *mut usize,
) -> Result<(), CUresult> {
    if cubin_out == std::ptr::null_mut() || size_out == std::ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    let state = LiveCheck::as_result(state)?;
    let modules = state
        .ptx_modules
        .lock()
        .map_err(|_| CUresult::CUDA_ERROR_UNKNOWN)?;
    let device = context::with_current(|ctx| ctx.device)?;
    let global_state = GLOBAL_STATE.get()?;
    let device_object = global_state.device(device)?;
    let module = module::link_build_zluda_module(
        global_state,
        device_object.compilation_mode,
        &device_object.comgr_isa,
        &modules,
    )?;
    let module = module.into_boxed_slice();
    let size = module.len();
    let ptr = Box::into_raw(module);
    *size_out = size;
    *cubin_out = ptr.cast();
    Ok(())
}

pub(crate) unsafe fn create(
    _num_options: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _option_values: *mut *mut ::std::os::raw::c_void,
    state_out: *mut *mut LinkState,
) -> Result<(), CUresult> {
    let link_state = LinkState::new(LinkStateData {
        ptx_modules: Mutex::new(Vec::new()),
    });
    let link_state = Box::into_raw(Box::new(link_state));
    *state_out = link_state;
    Ok(())
}

pub(crate) unsafe fn destroy(state: *mut LinkState) -> Result<(), CUresult> {
    if state == ptr::null_mut() {
        return Err(CUresult::CUDA_ERROR_INVALID_VALUE);
    }
    LiveCheck::drop_box_with_result(state, false)
}
