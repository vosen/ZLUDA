use crate::r#impl::{self, context, device, function};
use comgr::Comgr;
use cuda_types::cuda::*;
use hip_runtime_sys::*;
use std::{
    collections::BTreeMap,
    ffi::{c_void, CStr, CString},
    mem, ptr, slice,
    sync::{Mutex, OnceLock},
    usize,
};
use zluda_common::{FromCuda, LiveCheck};

#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;

pub(crate) struct GlobalState {
    pub devices: Vec<Device>,
    pub comgr: Comgr,
    pub comgr_clang_version: String,
    pub cache_path: Option<String>,
    pub allocations: Mutex<Allocations>,
}

pub(crate) struct Allocations {
    pub pointers: BTreeMap<usize, AllocationInfo>,
}

impl Allocations {
    pub fn new() -> Self {
        Allocations {
            pointers: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, ptr: usize, size: usize, context: CUcontext) {
        self.pointers.insert(ptr, AllocationInfo { size, context });
    }

    pub fn get_offset_and_info(&self, ptr: usize) -> Option<(usize, AllocationInfo)> {
        // Find last pair where `start <= ptr`
        let (start, alloc) = self.pointers.range(..=ptr).rev().next()?;
        // Check if allocation contains the pointer
        if start + alloc.size > ptr {
            Some((ptr - start, *alloc))
        } else {
            None
        }
    }

    pub fn remove(&mut self, ptr: usize) {
        self.pointers.remove(&ptr);
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) struct AllocationInfo {
    pub size: usize,
    pub context: CUcontext,
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
            let comgr_clang_version =
                comgr::get_clang_version(&comgr).map_err(|_| CUerror::UNKNOWN)?;
            let allocations = Mutex::new(Allocations::new());
            Ok(GlobalState {
                comgr,
                comgr_clang_version,
                allocations,
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
                            primary_context: LiveCheck::new(context::Context::new(i)),
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?,
                cache_path: zluda_cache::ModuleCache::create_cache_dir_and_get_path(),
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
        _module: *mut cuda_types::cuda::CUmodule,
        _fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
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
        _result: *mut cuda_types::cuda::CUmodule,
        _fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        _arg3: *mut std::ffi::c_void,
        _arg4: *mut std::ffi::c_void,
        _arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn cudart_interface_fn7(_arg1: usize) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn get_module_from_cubin_ext2(
        _fatbin_header: *const cuda_types::dark_api::FatbinHeader,
        _result: *mut cuda_types::cuda::CUmodule,
        _arg3: *mut std::ffi::c_void,
        _arg4: *mut std::ffi::c_void,
        _arg5: u32,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
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

    unsafe extern "system" fn context_local_storage_put(
        cu_ctx: CUcontext,
        key: *mut c_void,
        value: *mut c_void,
        dtor_cb: Option<extern "system" fn(CUcontext, *mut c_void, *mut c_void)>,
    ) -> CUresult {
        let _ctx = if cu_ctx.0 != ptr::null_mut() {
            cu_ctx
        } else {
            let mut current_ctx: CUcontext = CUcontext(ptr::null_mut());
            context::get_current(&mut current_ctx)?;
            current_ctx
        };
        let ctx_obj: &context::Context = FromCuda::<_, CUerror>::from_cuda(&_ctx)?;
        ctx_obj.with_state_mut(|state: &mut context::ContextState| {
            state.storage.insert(
                key as usize,
                context::StorageData {
                    value: value as usize,
                    reset_cb: dtor_cb,
                    handle: _ctx,
                },
            );
            Ok(())
        })?;
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_delete(
        cu_ctx: CUcontext,
        key: *mut c_void,
    ) -> CUresult {
        let ctx_obj: &context::Context = FromCuda::<_, CUerror>::from_cuda(&cu_ctx)?;
        ctx_obj.with_state_mut(|state: &mut context::ContextState| {
            state.storage.remove(&(key as usize));
            Ok(())
        })?;
        Ok(())
    }

    unsafe extern "system" fn context_local_storage_get(
        value: *mut *mut c_void,
        cu_ctx: CUcontext,
        key: *mut c_void,
    ) -> CUresult {
        let mut _ctx: CUcontext;
        if cu_ctx.0 == ptr::null_mut() {
            _ctx = context::get_current_context()?;
        } else {
            _ctx = cu_ctx
        };
        let ctx_obj: &context::Context = FromCuda::<_, CUerror>::from_cuda(&_ctx)?;
        ctx_obj.with_state(|state: &context::ContextState| {
            match state.storage.get(&(key as usize)) {
                Some(data) => *value = data.value as *mut c_void,
                None => return CUresult::ERROR_INVALID_HANDLE,
            }
            Ok(())
        })?;
        Ok(())
    }

    unsafe extern "system" fn ctx_create_v2_bypass(
        _pctx: *mut cuda_types::cuda::CUcontext,
        _flags: ::std::os::raw::c_uint,
        _dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn heap_alloc(
        _heap_alloc_record_ptr: *mut *const std::ffi::c_void,
        _arg2: usize,
        _arg3: usize,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn heap_free(
        _heap_alloc_record_ptr: *const std::ffi::c_void,
        _arg2: *mut usize,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn device_get_attribute_ext(
        _dev: cuda_types::cuda::CUdevice,
        _attribute: std::ffi::c_uint,
        _unknown: std::ffi::c_int,
        _result: *mut [usize; 2],
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn device_get_something(
        _result: *mut std::ffi::c_uchar,
        _dev: cuda_types::cuda::CUdevice,
    ) -> cuda_types::cuda::CUresult {
        r#impl::unimplemented()
    }

    unsafe extern "system" fn integrity_check(
        version: u32,
        unix_seconds: u64,
        result: *mut [u64; 2],
    ) -> cuda_types::cuda::CUresult {
        let current_process = std::process::id();
        let current_thread = os::current_thread();

        let integrity_check_table = EXPORT_TABLE.INTEGRITY_CHECK.as_ptr().cast();
        let cudart_table = EXPORT_TABLE.CUDART_INTERFACE.as_ptr().cast();
        let fn_address = EXPORT_TABLE.INTEGRITY_CHECK[1];

        let devices = get_device_hash_info()?;
        let device_count = devices.len() as u32;
        let get_device = |dev| devices[dev as usize];

        let hash = ::dark_api::integrity_check(
            version,
            unix_seconds,
            cuda_types::cuda::CUDA_VERSION,
            current_process,
            current_thread,
            integrity_check_table,
            cudart_table,
            fn_address,
            device_count,
            get_device,
        );
        *result = hash;
        Ok(())
    }

    unsafe extern "system" fn context_check(
        _ctx_in: cuda_types::cuda::CUcontext,
        result1: *mut u32,
        _result2: *mut *const std::ffi::c_void,
    ) -> cuda_types::cuda::CUresult {
        *result1 = 0;
        CUresult::SUCCESS
    }

    unsafe extern "system" fn check_fn3() -> u32 {
        0
    }
}

fn get_device_hash_info() -> Result<Vec<::dark_api::DeviceHashinfo>, CUerror> {
    let mut device_count = 0;
    device::get_count(&mut device_count)?;

    (0..device_count)
        .map(|dev| {
            let mut guid = CUuuid_st { bytes: [0; 16] };
            unsafe { crate::cuDeviceGetUuid(&mut guid, dev)? };

            let mut pci_domain = 0;
            device::get_attribute(
                &mut pci_domain,
                CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID,
                dev,
            )?;

            let mut pci_bus = 0;
            device::get_attribute(
                &mut pci_bus,
                CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID,
                dev,
            )?;

            let mut pci_device = 0;
            device::get_attribute(
                &mut pci_device,
                CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID,
                dev,
            )?;

            Ok(::dark_api::DeviceHashinfo {
                guid,
                pci_domain,
                pci_bus,
                pci_device,
            })
        })
        .collect()
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
    // This implementation is mostly the same as cuGetProcAddress_v2 in zluda_trace. We may want to factor out the duplication at some point.
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

pub(crate) fn profiler_start() -> CUresult {
    Ok(())
}

pub(crate) fn profiler_stop() -> CUresult {
    Ok(())
}

pub(crate) unsafe fn thread_exchange_stream_capture_mode(
    mode: *mut hipStreamCaptureMode,
) -> hipError_t {
    hipThreadExchangeStreamCaptureMode(mode)
}

pub(crate) unsafe fn occupancy_max_active_blocks_per_multiprocessor_with_flags(
    num_blocks: &mut ::core::ffi::c_int,
    func: hipFunction_t,
    block_size: ::core::ffi::c_int,
    dynamic_smem_size: usize,
    flags: ::core::ffi::c_uint,
) -> hipError_t {
    hipOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
        num_blocks,
        func.0.cast(),
        block_size,
        dynamic_smem_size,
        flags,
    )?;
    *num_blocks = (*num_blocks).max(1);
    Ok(())
}

pub(crate) unsafe fn launch_kernel_ex(
    config: &cuda_types::cuda::CUlaunchConfig,
    f: hipFunction_t,
    kernel_params: *mut *mut ::core::ffi::c_void,
    extra: *mut *mut ::core::ffi::c_void,
) -> CUresult {
    let attrs = std::slice::from_raw_parts(config.attrs, config.numAttrs as usize);
    if attrs.iter().any(|&attr| {
        !(attr.id == CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_PROGRAMMATIC_STREAM_SERIALIZATION
            && attr.value.programmaticStreamSerializationAllowed == 0)
    }) {
        return CUresult::ERROR_NOT_SUPPORTED;
    }
    function::launch_kernel(
        f,
        config.gridDimX,
        config.gridDimY,
        config.gridDimZ,
        config.blockDimX,
        config.blockDimY,
        config.blockDimZ,
        config.sharedMemBytes,
        FromCuda::<_, CUerror>::from_cuda(&config.hStream)?,
        kernel_params,
        extra,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::r#impl::driver::AllocationInfo;
    use crate::tests::CudaApi;
    use cuda_macros::test_cuda;
    use cuda_types::cuda::CUcontext;

    #[test_cuda]
    fn init(api: impl CudaApi) {
        api.cuInit(0);
    }

    #[test]
    fn get_allocation() {
        let ctx1 = CUcontext(0x1234 as _);
        let ctx2 = CUcontext(0x5678 as _);
        let mut alloc_info = super::Allocations::new();
        alloc_info.insert(0x1000, 4, ctx1);
        alloc_info.insert(0x2000, 8, ctx2);
        for i in 0..4 {
            assert_eq!(
                alloc_info.get_offset_and_info(0x1000 + i),
                Some((
                    i,
                    AllocationInfo {
                        size: 4,
                        context: ctx1
                    }
                ))
            );
        }
        assert_eq!(alloc_info.get_offset_and_info(0x1000 + 4), None);
        for i in 0..8 {
            assert_eq!(
                alloc_info.get_offset_and_info(0x2000 + i),
                Some((
                    i,
                    AllocationInfo {
                        size: 8,
                        context: ctx2
                    }
                ))
            );
        }
        assert_eq!(alloc_info.get_offset_and_info(0x2000 + 8), None);
    }
}
