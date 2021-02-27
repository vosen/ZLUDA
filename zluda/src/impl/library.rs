// Library is a module that is not context-bound, see here:
// https://developer.nvidia.com/blog/cuda-context-independent-module-loading/
// It's supposed to be lazy-loaded for each device (depending on cuModuleGetLoadingMode(...)),
// but we do eager loading right now for simplicity
// TODO: make libraries lazy-loadable
use super::{
    context, fold_cuda_errors,
    module::{self, ModuleData},
    LiveCheck, ZludaObject, GLOBAL_STATE,
};
use cuda_types::{CUjit_option, CUlibraryOption, CUresult};

pub(crate) type Library = LiveCheck<LibraryData>;

impl ZludaObject for LibraryData {
    #[cfg(target_pointer_width = "64")]
    const LIVENESS_COOKIE: usize = 0x9769b2dd3d1764df;
    #[cfg(target_pointer_width = "32")]
    const LIVENESS_COOKIE: usize = 0xdbbdd7c7;
    const LIVENESS_FAIL: CUresult = CUresult::CUDA_ERROR_INVALID_HANDLE;

    fn drop_with_result(&mut self, _by_owner: bool) -> Result<(), CUresult> {
        fold_cuda_errors(
            self.modules
                .iter_mut()
                .map(|module| unsafe { LiveCheck::drop_box_with_result(*module, true) }),
        )
    }
}

pub(crate) struct LibraryData {
    modules: Vec<*mut module::Module>,
}

pub(crate) unsafe fn load_data(
    library: *mut *mut Library,
    code: *const ::std::os::raw::c_void,
    // TODO: start handling JIT options
    _jit_options: *mut CUjit_option,
    _jit_options_values: *mut *mut ::std::os::raw::c_void,
    _num_jit_options: ::std::os::raw::c_uint,
    library_options: *mut CUlibraryOption,
    _library_option_values: *mut *mut ::std::os::raw::c_void,
    num_library_options: ::std::os::raw::c_uint,
) -> Result<(), CUresult> {
    for option in std::slice::from_raw_parts(library_options, num_library_options as usize) {
        if !matches!(*option, CUlibraryOption::CU_LIBRARY_BINARY_IS_PRESERVED) {
            return Err(CUresult::CUDA_ERROR_NOT_SUPPORTED);
        }
    }
    let global_state = GLOBAL_STATE.get()?;
    let modules = global_state
        .devices
        .iter()
        .map(|device| {
            let module_data = module::load_data_any(
                None,
                device.compilation_mode,
                &device.comgr_isa,
                zluda_dark_api::CUmoduleContent::from_ptr(code.cast())
                    .map_err(|_| CUresult::CUDA_ERROR_INVALID_VALUE)?,
            )?;
            Ok(ModuleData::alloc(module_data))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let library_data = LibraryData { modules };
    *library = Box::into_raw(Box::new(LiveCheck::new(library_data)));
    Ok(())
}

pub(crate) unsafe fn get_module(
    output: *mut *mut module::Module,
    library: *mut Library,
) -> Result<(), CUresult> {
    let library = LiveCheck::as_result(library)?;
    context::with_current(|ctx| {
        let device = ctx.device as usize;
        let module = library
            .modules
            .get(device)
            .copied()
            .ok_or(CUresult::CUDA_ERROR_UNKNOWN)?;
        *output = module;
        Ok(())
    })?
}

pub(crate) unsafe fn unload(library: *mut Library) -> Result<(), CUresult> {
    LiveCheck::drop_box_with_result(library, false)
}
