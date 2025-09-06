use crate::r#impl::{context, driver, module};
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{ffi::c_void, sync::OnceLock};
use zluda_common::{CodeLibraryRef, ZludaObject};

pub(crate) struct Library {
    data: LibraryData,
    modules: Vec<OnceLock<Result<hipModule_t, CUerror>>>,
}

impl Library {
    pub(crate) fn get_module_for_device(&self, device: usize) -> Result<hipModule_t, CUerror> {
        let module_lock = self.modules.get(device).ok_or(CUerror::INVALID_DEVICE)?;
        *module_lock.get_or_init(|| match self.data {
            LibraryData::Lazy(lib) => module::load_hip_module(lib),
            LibraryData::Eager(()) => Err(CUerror::NOT_SUPPORTED),
        })
    }
}

enum LibraryData {
    Lazy(CodeLibraryRef<'static>),
    Eager(()),
}

impl LibraryData {
    unsafe fn new(ptr: *mut c_void, static_lifetime: bool) -> Result<Self, CUerror> {
        if static_lifetime {
            let lib = CodeLibraryRef::try_load(ptr).map_err(|_| CUerror::INVALID_VALUE)?;
            Ok(LibraryData::Lazy(lib))
        } else {
            Err(CUerror::NOT_SUPPORTED)
        }
    }
}

impl ZludaObject for Library {
    const COOKIE: usize = 0xb328a916cc234d7c;

    type Error = CUerror;
    type CudaHandle = CUlibrary;

    fn drop_checked(&mut self) -> CUresult {
        // TODO: implement unloading
        // TODO: we will want to test that we handle `cuModuleUnload` on a module that came from a library correctly, without calling `hipModuleUnload` twice
        Ok(())
    }
}

pub(crate) unsafe fn load_data(
    result: &mut CUlibrary,
    code: *const ::core::ffi::c_void,
    _jit_options: Option<&mut CUjit_option>,
    _jit_options_values: Option<&mut *mut ::core::ffi::c_void>,
    _num_jit_options: ::core::ffi::c_uint,
    library_options: Option<&mut CUlibraryOption>,
    library_option_values: Option<&mut *mut ::core::ffi::c_void>,
    num_library_options: ::core::ffi::c_uint,
) -> CUresult {
    let global_state = driver::global_state()?;
    let options =
        LibraryOptions::load(library_options, library_option_values, num_library_options)?;
    let library = Library {
        data: LibraryData::new(code as *mut c_void, options.preserve_binary)?,
        modules: vec![OnceLock::new(); global_state.devices.len()],
    };
    *result = library.wrap();
    Ok(())
}

struct LibraryOptions {
    preserve_binary: bool,
}

impl LibraryOptions {
    unsafe fn load(
        library_options: Option<&mut CUlibraryOption>,
        library_option_values: Option<&mut *mut ::core::ffi::c_void>,
        num_library_options: ::core::ffi::c_uint,
    ) -> Result<Self, CUerror> {
        if num_library_options == 0 {
            return Ok(LibraryOptions {
                preserve_binary: false,
            });
        }
        let (library_options, library_option_values) =
            match (library_options, library_option_values) {
                (Some(library_options), Some(library_option_values)) => {
                    let library_options =
                        std::slice::from_raw_parts(library_options, num_library_options as usize);
                    let library_option_values = std::slice::from_raw_parts(
                        library_option_values,
                        num_library_options as usize,
                    );
                    (library_options, library_option_values)
                }
                _ => return Err(CUerror::INVALID_VALUE),
            };
        let mut preserve_binary = false;
        for (option, value) in library_options
            .iter()
            .copied()
            .zip(library_option_values.iter())
        {
            match option {
                CUlibraryOption::CU_LIBRARY_BINARY_IS_PRESERVED => {
                    preserve_binary = *(value.cast::<bool>());
                }
                _ => return Err(CUerror::NOT_SUPPORTED),
            }
        }
        Ok(LibraryOptions { preserve_binary })
    }
}

pub(crate) unsafe fn unload(library: CUlibrary) -> CUresult {
    zluda_common::drop_checked::<Library>(library)
}

pub(crate) unsafe fn get_module(out: &mut CUmodule, library: &Library) -> CUresult {
    let device = context::get_current_device()?;
    // TODO: lifetime is very wrong here
    let library = module::Module {
        base: library.get_module_for_device(device as usize)?,
    };
    *out = library.wrap();
    Ok(())
}

pub(crate) unsafe fn get_kernel(
    kernel: *mut hipFunction_t,
    library: &Library,
    name: *const ::core::ffi::c_char,
) -> CUresult {
    let device = context::get_current_device()?;
    let module = library.get_module_for_device(device as usize)?;
    hipModuleGetFunction(kernel, module, name)?;
    Ok(())
}

pub(crate) unsafe fn get_global(
    dptr: *mut hipDeviceptr_t,
    bytes: *mut usize,
    library: &Library,
    name: *const ::core::ffi::c_char,
) -> CUresult {
    let device = context::get_current_device()?;
    let module = library.get_module_for_device(device as usize)?;
    hipModuleGetGlobal(dptr, bytes, module, name)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::{CUlibraryOption, CUresult, CUresultConsts};
    use std::{
        ffi::{c_void, CStr},
        mem, ptr,
    };

    #[test_cuda]
    unsafe fn library_loads_without_context(api: impl CudaApi) {
        static PTX: &'static CStr = c"
            .version 7.0
            .target sm_70
            .address_size 64

            .visible .entry foobar() {
              ret;
            }
        ";
        api.cuInit(0);
        let mut device = mem::zeroed();
        assert_eq!(
            CUresult::ERROR_INVALID_CONTEXT,
            api.cuCtxGetDevice_unchecked(&mut device)
        );
        let mut module = mem::zeroed();
        assert_eq!(
            CUresult::ERROR_INVALID_CONTEXT,
            api.cuModuleLoadData_unchecked(&mut module, PTX.as_ptr().cast())
        );
        let mut library = mem::zeroed();
        api.cuLibraryLoadData(
            &mut library,
            PTX.as_ptr().cast(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            [CUlibraryOption::CU_LIBRARY_BINARY_IS_PRESERVED].as_mut_ptr(),
            [(&true as *const bool) as *mut c_void].as_mut_ptr(),
            1,
        );
        assert_eq!(
            CUresult::ERROR_INVALID_CONTEXT,
            api.cuLibraryGetModule_unchecked(&mut mem::zeroed(), library)
        );
    }
}
