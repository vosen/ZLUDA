use super::LiveCheck;
use crate::r#impl::context;
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
