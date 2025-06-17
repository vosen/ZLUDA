use super::LiveCheck;
use crate::r#impl::{context, device};
use comgr::Comgr;
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{
    ffi::{CStr, CString},
    mem, ptr, slice,
    sync::OnceLock,
    usize,
};

pub(crate) struct GlobalState {
    pub devices: Vec<Device>,
    pub comgr: Comgr,
}

pub(crate) struct Device {
    pub(crate) _comgr_isa: CString,
    primary_context: LiveCheck<context::Context>,
}

impl Device {
    pub(crate) fn primary_context<'a>(&'a self) -> (&'a context::Context, CUcontext) {
        unsafe {
            (
                self.primary_context.data.assume_init_ref(),
                self.primary_context.as_handle(),
            )
        }
    }
}

pub(crate) fn device(dev: i32) -> Result<&'static Device, CUerror> {
    global_state()?
        .devices
        .get(dev as usize)
        .ok_or(CUerror::INVALID_DEVICE)
}

pub(crate) fn global_state() -> Result<&'static GlobalState, CUerror> {
    static GLOBAL_STATE: OnceLock<Result<GlobalState, CUerror>> = OnceLock::new();
    fn cast_slice<'a>(bytes: &'a [i8]) -> &'a [u8] {
        unsafe { slice::from_raw_parts(bytes.as_ptr().cast(), bytes.len()) }
    }
    GLOBAL_STATE
        .get_or_init(|| {
            let mut device_count = 0;
            unsafe { hipGetDeviceCount(&mut device_count) }?;
            let comgr = Comgr::new().map_err(|_| CUerror::UNKNOWN)?;
            Ok(GlobalState {
                comgr,
                devices: (0..device_count)
                    .map(|i| {
                        let mut props = unsafe { mem::zeroed() };
                        unsafe { hipGetDevicePropertiesR0600(&mut props, i) }?;
                        Ok::<_, CUerror>(Device {
                            _comgr_isa: CStr::from_bytes_until_nul(cast_slice(
                                &props.gcnArchName[..],
                            ))
                            .map_err(|_| CUerror::UNKNOWN)?
                            .to_owned(),
                            primary_context: LiveCheck::new(context::new(i)),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            })
        })
        .as_ref()
        .map_err(|e| *e)
}

pub(crate) fn init(flags: ::core::ffi::c_uint) -> CUresult {
    unsafe { hipInit(flags) }?;
    global_state()?;
    Ok(())
}

struct UnknownBuffer<const S: usize> {
    buffer: std::cell::UnsafeCell<[u32; S]>,
}

impl<const S: usize> UnknownBuffer<S> {
    const fn new() -> Self {
        UnknownBuffer {
            buffer: std::cell::UnsafeCell::new([0; S]),
        }
    }
    const fn len(&self) -> usize {
        S
    }
}

unsafe impl<const S: usize> Sync for UnknownBuffer<S> {}

static UNKNOWN_BUFFER1: UnknownBuffer<1024> = UnknownBuffer::new();
static UNKNOWN_BUFFER2: UnknownBuffer<14> = UnknownBuffer::new();

struct DarkApi {}

impl ::dark_api::cuda::CudaDarkApi for DarkApi {
    unsafe extern "system" fn get_module_from_cubin(
        module: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
    ) -> () {
        todo!()
    }

    unsafe extern "system" fn cudart_interface_fn2(
        pctx: *mut cuda_types::cuda::CUcontext,
        hip_dev: hipDevice_t,
    ) -> cuda_types::cuda::CUresult {
        let pctx = match pctx.as_mut() {
            Some(p) => p,
            None => return CUresult::ERROR_INVALID_VALUE,
        };

        device::primary_context_retain(pctx, hip_dev)
    }

    unsafe extern "system" fn get_module_from_cubin_ext1(
        result: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn cudart_interface_fn7(arg1: usize) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn get_module_from_cubin_ext2(
        fatbin_header: *const cuda_types::dark_api::FatbinHeader,
        result: *mut cuda_types::cuda::CUmodule,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn get_unknown_buffer1(
        ptr: *mut *mut std::ffi::c_void,
        size: *mut usize,
    ) -> () {
        *ptr = UNKNOWN_BUFFER1.buffer.get() as *mut std::ffi::c_void;
        *size = UNKNOWN_BUFFER1.len();
    }

    unsafe extern "system" fn get_unknown_buffer2(
        ptr: *mut *mut std::ffi::c_void,
        size: *mut usize,
    ) -> () {
        *ptr = UNKNOWN_BUFFER2.buffer.get() as *mut std::ffi::c_void;
        *size = UNKNOWN_BUFFER2.len();
    }

    unsafe extern "system" fn context_local_storage_ctor(
        context: cuda_types::cuda::CUcontext,
        manager: *mut std::ffi::c_void,
        ctx_state: *mut std::ffi::c_void,
        dtor_cb: Option<
            extern "system" fn(
                cuda_types::cuda::CUcontext,
                *mut std::ffi::c_void,
                *mut std::ffi::c_void,
            ),
        >,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn context_local_storage_dtor(
        arg1: *mut std::ffi::c_void,
        arg2: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn context_local_storage_get_state(
        ctx_state: *mut std::ffi::c_void,
        cu_ctx: cuda_types::cuda::CUcontext,
        manager: *mut std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn ctx_create_v2_bypass(
        pctx: *mut cuda_types::cuda::CUcontext,
        flags: ::std::os::raw::c_uint,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn heap_alloc(
        heap_alloc_record_ptr: *mut *const std::ffi::c_void,
        arg2: usize,
        arg3: usize,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn heap_free(
        heap_alloc_record_ptr: *const std::ffi::c_void,
        arg2: *mut usize,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn device_get_attribute_ext(
        dev: cuda_types::cuda::CUdevice,
        attribute: std::ffi::c_uint,
        unknown: std::ffi::c_int,
        result: *mut [usize; 2],
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn device_get_something(
        result: *mut std::ffi::c_uchar,
        dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn integrity_check(
        version: u32,
        unix_seconds: u64,
        result: *mut [u64; 2],
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn context_check(
        ctx_in: cuda_types::cuda::CUcontext,
        result1: *mut u32,
        result2: *mut *const std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        todo!()
    }

    unsafe extern "system" fn check_fn3() -> u32 {
        todo!()
    }
}

static EXPORT_TABLE: ::dark_api::cuda::CudaDarkApiGlobalTable =
    ::dark_api::cuda::CudaDarkApiGlobalTable::new::<DarkApi>();

pub(crate) fn get_export_table(
    pp_export_table: &mut *const ::core::ffi::c_void,
    p_export_table_id: &CUuuid,
) -> CUresult {
    if let Some(table) = EXPORT_TABLE.get(p_export_table_id) {
        *pp_export_table = table.start();
        cuda_types::cuda::CUresult::SUCCESS
    } else {
        cuda_types::cuda::CUresult::ERROR_INVALID_VALUE
    }
}

pub(crate) fn get_version(version: &mut ::core::ffi::c_int) -> CUresult {
    *version = cuda_types::cuda::CUDA_VERSION as i32;
    Ok(())
}

pub(crate) unsafe fn get_proc_address(
    symbol: &CStr,
    pfn: &mut *mut ::core::ffi::c_void,
    cuda_version: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
) -> CUresult {
    get_proc_address_v2(symbol, pfn, cuda_version, flags, None)
}

pub(crate) unsafe fn get_proc_address_v2(
    symbol: &CStr,
    pfn: &mut *mut ::core::ffi::c_void,
    cuda_version: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
    symbol_status: Option<&mut cuda_types::cuda::CUdriverProcAddressQueryResult>,
) -> CUresult {
    // This implementation is mostly the same as cuGetProcAddress_v2 in zluda_dump. We may want to factor out the duplication at some point.
    fn raw_match(name: &[u8], flag: u64, version: i32) -> *mut ::core::ffi::c_void {
        use crate::*;
        include!("../../../zluda_bindgen/src/process_table.rs")
    }
    let fn_ptr = raw_match(symbol.to_bytes(), flags, cuda_version);
    match fn_ptr as usize {
        0 => {
            if let Some(symbol_status) = symbol_status {
                *symbol_status = cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND;
            }
            *pfn = ptr::null_mut();
            CUresult::ERROR_NOT_FOUND
        }
        usize::MAX => {
            if let Some(symbol_status) = symbol_status {
                *symbol_status = cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT;
            }
            *pfn = ptr::null_mut();
            CUresult::ERROR_NOT_FOUND
        }
        _ => {
            if let Some(symbol_status) = symbol_status {
                *symbol_status =
                    cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS;
            }
            *pfn = fn_ptr;
            Ok(())
        }
    }
}
