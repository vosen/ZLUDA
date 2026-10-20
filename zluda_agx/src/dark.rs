//! CUDA's private driver interfaces ("export tables", `cuGetExportTable`), which the CUDA runtime (libcudart)
//! needs to register its fat binaries and to start. Their layout is defined in `dark_api`; this is the AGX
//! implementation, following the HIP-based `zluda` crate's.

#![allow(non_snake_case)]

use crate::driver;
use cuda_types::cuda::*;
use std::collections::HashMap;
use std::ffi::c_void;
use std::sync::Mutex;

struct UnknownBuffer<const S: usize>(std::cell::UnsafeCell<[u32; S]>);
unsafe impl<const S: usize> Sync for UnknownBuffer<S> {}
static UNKNOWN_BUFFER1: UnknownBuffer<1024> = UnknownBuffer(std::cell::UnsafeCell::new([0; 1024]));
static UNKNOWN_BUFFER2: UnknownBuffer<14> = UnknownBuffer(std::cell::UnsafeCell::new([0; 14]));

/// Context-local storage of the one context: key -> value.
static STORAGE: Mutex<Option<HashMap<usize, usize>>> = Mutex::new(None);

pub(crate) static EXPORT_TABLE: ::dark_api::cuda::CudaDarkApiGlobalTable =
    ::dark_api::cuda::CudaDarkApiGlobalTable::new::<DarkApi>();

#[link(name = "pthread")]
extern "C" {
    fn pthread_self() -> std::os::unix::thread::RawPthread;
}

struct DarkApi {}

impl ::dark_api::cuda::CudaDarkApi for DarkApi {
    unsafe extern "system" fn get_module_from_cubin(result: *mut CUmodule, fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper) -> CUresult {
        driver::cuModuleLoadData(result, fatbinc_wrapper as *const c_void)
    }

    unsafe extern "system" fn cudart_interface_fn2(pctx: *mut CUcontext, dev: CUdevice) -> CUresult {
        driver::cuDevicePrimaryCtxRetain(pctx, dev)
    }

    unsafe extern "system" fn get_module_from_cubin_ext1(
        result: *mut CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: u32,
    ) -> CUresult {
        if !arg3.is_null() || !arg4.is_null() || arg5 != 0 {
            return Err(CUerror::NOT_SUPPORTED);
        }
        driver::cuModuleLoadData(result, fatbinc_wrapper as *const c_void)
    }

    unsafe extern "system" fn cudart_interface_fn7(_arg1: usize) {}

    unsafe extern "system" fn get_module_from_cubin_ext2(
        fatbin_header: *const cuda_types::dark_api::FatbinHeader,
        result: *mut CUmodule,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: u32,
    ) -> CUresult {
        if !arg3.is_null() || !arg4.is_null() || arg5 != 0 {
            return Err(CUerror::NOT_SUPPORTED);
        }
        driver::cuModuleLoadData(result, fatbin_header as *const c_void)
    }

    unsafe extern "system" fn load_compilers() -> CUresult {
        Ok(())
    }

    unsafe extern "system" fn get_unknown_buffer1(ptr: *mut *mut c_void, size: *mut usize) {
        *ptr = UNKNOWN_BUFFER1.0.get() as *mut c_void;
        *size = 1024;
    }

    unsafe extern "system" fn get_unknown_buffer2(ptr: *mut *mut c_void, size: *mut usize) {
        *ptr = UNKNOWN_BUFFER2.0.get() as *mut c_void;
        *size = 14;
    }

    unsafe extern "system" fn context_local_storage_put(
        _ctx: CUcontext,
        key: *mut c_void,
        value: *mut c_void,
        _dtor_cb: Option<extern "system" fn(CUcontext, *mut c_void, *mut c_void)>,
    ) -> CUresult {
        STORAGE.lock().unwrap().get_or_insert_with(HashMap::new).insert(key as usize, value as usize);
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_delete(_ctx: CUcontext, key: *mut c_void) -> CUresult {
        if let Some(s) = STORAGE.lock().unwrap().as_mut() {
            s.remove(&(key as usize));
        }
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_get(value: *mut *mut c_void, _ctx: CUcontext, key: *mut c_void) -> CUresult {
        match STORAGE.lock().unwrap().as_ref().and_then(|s| s.get(&(key as usize))) {
            Some(&v) => {
                *value = v as *mut c_void;
                Ok(())
            }
            None => Err(CUerror::INVALID_HANDLE),
        }
    }

    unsafe extern "system" fn ctx_create_v2_bypass(pctx: *mut CUcontext, flags: std::os::raw::c_uint, dev: CUdevice) -> CUresult {
        driver::cuCtxCreate_v2(pctx, flags, dev)
    }

    unsafe extern "system" fn heap_alloc(_record: *mut *const c_void, _arg2: usize, _arg3: usize) -> CUresult {
        Err(CUerror::NOT_SUPPORTED)
    }

    unsafe extern "system" fn heap_free(_record: *const c_void, _arg2: *mut usize) -> CUresult {
        Err(CUerror::NOT_SUPPORTED)
    }

    unsafe extern "system" fn device_get_attribute_ext(_dev: CUdevice, _attribute: std::ffi::c_uint, _unknown: std::ffi::c_int, _result: *mut [usize; 2]) -> CUresult {
        Err(CUerror::NOT_SUPPORTED)
    }

    unsafe extern "system" fn device_get_something(_result: *mut std::ffi::c_uchar, _dev: CUdevice) -> CUresult {
        Err(CUerror::NOT_SUPPORTED)
    }

    unsafe extern "system" fn integrity_check(version: u32, unix_seconds: u64, result: *mut [u64; 2]) -> CUresult {
        let mut guid = std::mem::zeroed();
        driver::cuDeviceGetUuid_v2(&mut guid, 0)?;
        let device = ::dark_api::DeviceHashinfo { guid, pci_domain: 0, pci_bus: 0, pci_device: 0 };
        *result = ::dark_api::integrity_check(
            version,
            unix_seconds,
            cuda_types::cuda::CUDA_VERSION,
            std::process::id(),
            pthread_self() as u32,
            EXPORT_TABLE.INTEGRITY_CHECK.as_ptr().cast(),
            EXPORT_TABLE.CUDART_INTERFACE.as_ptr().cast(),
            EXPORT_TABLE.INTEGRITY_CHECK[1],
            1,
            |_| device,
        );
        Ok(())
    }

    unsafe extern "system" fn context_check(_ctx: CUcontext, result1: *mut u32, _result2: *mut *const c_void) -> CUresult {
        *result1 = 0;
        Ok(())
    }

    unsafe extern "system" fn check_fn3() -> u32 {
        0
    }

    // Only the Windows runtime loads a hybrid runtime.
    unsafe extern "system" fn hybrid_runtime_load_get_proc_address(_name: *const std::ffi::c_char, _fn_ptr: *mut *const c_void, _token: *mut usize) -> CUresult {
        Err(CUerror::INVALID_VALUE)
    }

    unsafe extern "system" fn hybrid_runtime_free(_token: usize) -> CUresult {
        Err(CUerror::INVALID_HANDLE)
    }
}

pub(crate) unsafe fn cuGetExportTable(table: *mut *const c_void, id: *const CUuuid) -> CUresult {
    let (Some(table), Some(id)) = (table.as_mut(), id.as_ref()) else { return Err(CUerror::INVALID_VALUE) };
    match EXPORT_TABLE.get(id) {
        Some(t) => {
            *table = t.start();
            Ok(())
        }
        None => Err(CUerror::INVALID_VALUE),
    }
}
