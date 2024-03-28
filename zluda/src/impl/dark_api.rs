use super::module;
use super::{
    context::{self, LocalStorageValue},
    device, FromCuda, IntoCuda, LiveCheck,
};
use crate::r#impl::{dark_api, stream};
use cuda_types::*;
use hip_common::zluda_ext::CudaResult;
use std::{
    ffi::c_void,
    mem,
    os::raw::{c_int, c_uchar, c_uint},
    ptr,
};
use zluda_dark_api::{
    AntiZludaHashInput, CUmoduleContent, CudaDarkApi, CudaDarkApiTable, CudaFatbin,
};

pub(crate) unsafe fn get_table(
    pp_export_table: *mut *const ::std::os::raw::c_void,
    p_export_table_id: *const CUuuid,
) -> CUresult {
    if pp_export_table == ptr::null_mut() || p_export_table_id == ptr::null() {
        return CUresult::CUDA_ERROR_INVALID_VALUE;
    }
    if let Some(table_ptr) = CUDA_DARK_API_TABLE.get(&(*p_export_table_id).bytes) {
        *pp_export_table = table_ptr.as_ptr() as _;
        CUresult::CUDA_SUCCESS
    } else {
        CUresult::CUDA_ERROR_UNKNOWN
    }
}

static CUDA_DARK_API_TABLE: CudaDarkApiTable = zluda_dark_api::init_dark_api::<CudaDarkApiZluda>();

struct CudaDarkApiZluda;

static mut TOOLS_RUNTIME_CALLBACK_HOOKS_FN2_SPACE: [usize; 1024] = [0; 1024];
static mut TOOLS_RUNTIME_CALLBACK_HOOKS_FN6_SPACE: [u8; 14] = [0; 14];

impl CudaDarkApi for CudaDarkApiZluda {
    unsafe extern "system" fn get_module_from_cubin(
        module: *mut cuda_types::CUmodule,
        fatbinc_wrapper: *const zluda_dark_api::FatbincWrapper,
    ) -> CUresult {
        if module == ptr::null_mut() || fatbinc_wrapper == ptr::null_mut() {
            return CUresult::CUDA_ERROR_INVALID_VALUE;
        }
        let fatbin = match CudaFatbin::from_wrapper(fatbinc_wrapper) {
            Ok(fatbin) => fatbin,
            Err(_) => return CUresult::CUDA_ERROR_NOT_SUPPORTED,
        };
        module::load_impl(module.cast(), CUmoduleContent::Fatbin(fatbin)).into_cuda()
    }

    unsafe extern "system" fn get_primary_context(
        pctx: *mut cuda_types::CUcontext,
        dev: cuda_types::CUdevice,
    ) -> CUresult {
        let pctx: *mut *mut context::Context = FromCuda::from_cuda(pctx);
        let hip_dev = FromCuda::from_cuda(dev);
        device::primary_ctx_get(pctx, hip_dev).into_cuda()
    }

    unsafe extern "system" fn get_module_from_cubin_ex1(
        module: *mut cuda_types::CUmodule,
        fatbinc_wrapper: *const zluda_dark_api::FatbincWrapper,
        arg3: *mut c_void,
        arg4: *mut c_void,
        _arg5: usize,
    ) -> CUresult {
        if arg3 != ptr::null_mut() || arg4 != ptr::null_mut() {
            return CUresult::CUDA_ERROR_NOT_SUPPORTED;
        }
        if module == ptr::null_mut() || fatbinc_wrapper == ptr::null_mut() {
            return CUresult::CUDA_ERROR_INVALID_VALUE;
        }
        let fatbin = match CudaFatbin::from_wrapper(fatbinc_wrapper) {
            Ok(fatbin) => fatbin,
            Err(_) => return CUresult::CUDA_ERROR_NOT_SUPPORTED,
        };
        module::load_impl(module.cast(), CUmoduleContent::Fatbin(fatbin)).into_cuda()
    }

    unsafe extern "system" fn cudart_interface_fn7(_arg1: usize) -> () {}

    unsafe extern "system" fn get_module_from_cubin_ex2(
        fatbin_header: *const zluda_dark_api::FatbinHeader,
        module: *mut cuda_types::CUmodule,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: c_uint,
    ) -> CUresult {
        if arg3 != ptr::null_mut() || arg4 != ptr::null_mut() || arg5 != 0 {
            CUresult::CUDA_ERROR_NOT_SUPPORTED
        } else {
            let fatbin = CudaFatbin::from_header(fatbin_header);
            module::load_impl(module.cast(), CUmoduleContent::Fatbin(fatbin)).into_cuda()
        }
    }

    unsafe extern "system" fn tools_runtime_callback_hooks_fn2(
        ptr: *mut *mut usize,
        size: *mut usize,
    ) -> () {
        *ptr = TOOLS_RUNTIME_CALLBACK_HOOKS_FN2_SPACE.as_mut_ptr();
        *size = TOOLS_RUNTIME_CALLBACK_HOOKS_FN2_SPACE.len();
    }

    unsafe extern "system" fn tools_runtime_callback_hooks_fn6(
        ptr: *mut *mut u8,
        size: *mut usize,
    ) -> () {
        *ptr = TOOLS_RUNTIME_CALLBACK_HOOKS_FN6_SPACE.as_mut_ptr();
        *size = TOOLS_RUNTIME_CALLBACK_HOOKS_FN6_SPACE.len();
    }

    unsafe extern "system" fn context_local_storage_insert(
        cu_ctx: cuda_types::CUcontext,
        key: *mut c_void,
        value: *mut c_void,
        dtor_callback: Option<extern "system" fn(cuda_types::CUcontext, *mut c_void, *mut c_void)>,
    ) -> CUresult {
        unsafe fn context_local_storage_insert_impl(
            cu_ctx: cuda_types::CUcontext,
            key: *mut c_void,
            value: *mut c_void,
            dtor_callback: Option<
                extern "system" fn(cuda_types::CUcontext, *mut c_void, *mut c_void),
            >,
        ) -> Result<(), CUresult> {
            with_context_or_current(cu_ctx, |ctx| {
                ctx.with_inner_mut(|ctx_mutable| {
                    ctx_mutable.local_storage.insert(
                        key,
                        LocalStorageValue {
                            value,
                            _dtor_callback: dtor_callback,
                        },
                    );
                })
            })?
        }
        context_local_storage_insert_impl(cu_ctx, key, value, dtor_callback).into_cuda()
    }

    // TODO
    unsafe extern "system" fn context_local_storage_remove(_arg1: usize, _arg2: usize) -> CUresult {
        CUresult::CUDA_SUCCESS
    }

    unsafe extern "system" fn context_local_storage_get(
        cu_result: *mut *mut c_void,
        cu_ctx: cuda_types::CUcontext,
        key: *mut c_void,
    ) -> CUresult {
        unsafe fn context_local_storage_get_impl(
            cu_ctx: cuda_types::CUcontext,
            key: *mut c_void,
        ) -> Result<*mut c_void, CUresult> {
            with_context_or_current(cu_ctx, |ctx| {
                ctx.with_inner_mut(|ctx_mutable| {
                    ctx_mutable
                        .local_storage
                        .get(&key)
                        .map(|v| v.value)
                        .ok_or(CUresult::CUDA_ERROR_INVALID_VALUE)
                })?
            })?
        }
        match context_local_storage_get_impl(cu_ctx, key) {
            Ok(result) => {
                *cu_result = result;
                CUresult::CUDA_SUCCESS
            }
            Err(err) => err,
        }
    }

    unsafe extern "system" fn ctx_create_v2_bypass(
        pctx: *mut cuda_types::CUcontext,
        flags: c_uint,
        dev: cuda_types::CUdevice,
    ) -> CUresult {
        let pctx = FromCuda::from_cuda(pctx);
        let dev = FromCuda::from_cuda(dev);
        context::create(pctx, flags, dev).into_cuda()
    }

    unsafe extern "system" fn heap_alloc(
        _halloc_ptr: *mut *mut zluda_dark_api::HeapAllocRecord,
        _param1: usize,
        _param2: usize,
    ) -> CUresult {
        super::unimplemented()
    }

    unsafe extern "system" fn heap_free(
        _halloc: *mut zluda_dark_api::HeapAllocRecord,
        _param2: *mut usize,
    ) -> CUresult {
        super::unimplemented()
    }

    unsafe extern "system" fn device_get_attribute_ex(
        _dev: cuda_types::CUdevice,
        _attribute: c_uint,
        _unknown: c_int,
        _result: *mut [usize; 2],
    ) -> CUresult {
        super::unimplemented()
    }

    unsafe extern "system" fn device_get_something(
        _result: *mut c_uchar,
        _dev: cuda_types::CUdevice,
    ) -> CUresult {
        super::unimplemented()
    }

    unsafe extern "system" fn launch_kernel(
        _f: CUfunction,
        _grid_dim_x: std::os::raw::c_uint,
        _grid_dim_y: std::os::raw::c_uint,
        _grid_dim_z: std::os::raw::c_uint,
        _block_dim_x: std::os::raw::c_uint,
        _block_dim_y: std::os::raw::c_uint,
        _block_dim_z: std::os::raw::c_uint,
        _shared_mem_bytes: std::os::raw::c_uint,
        _stream: CUstream,
        _extra: *mut *mut std::os::raw::c_void,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_cuInit() -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_start1(
        _retval1: *mut *mut c_void,
        _arg2: *mut c_void,
        _arg3: *mut c_void,
        _arg4: *mut c_void,
        _arg5: *mut c_void,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_start2(_handle: *mut c_void, _arg2: *mut u32) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_module_load(
        _context: CUcontext,
        _result: *mut CUmodule,
        _fatbin: *mut c_void,
        _arg4: u32,
        _arg5: *mut c_void,
        _arg6: *mut c_void,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_module_get_function(
        _result: *mut CUfunction,
        _module: CUmodule,
        _name: *const i8,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_feature_evaluate2(
        _handle1: *mut c_void,
        _handle2: *mut c_void,
        _handle3: *mut c_void,
        _arg4: u8,
        _handle5: *mut c_void,
        _arg6: u32,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_feature_evaluate1(
        _retval1: *mut u32,
        _retval2: *mut u32,
        _retval3: *mut u32,
        _handle: *mut c_void,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn dlss_feature_evaluate_init(
        _retval1: *mut *mut c_void,
        _handle: *mut c_void,
        _retval2: *mut *mut c_void,
    ) -> CUresult {
        super::unimplemented()
    }

    #[allow(non_snake_case)]
    unsafe extern "system" fn zluda_check(
        rt_version: u32,
        timestamp: u64,
        result: *mut u128,
    ) -> CUresult {
        use crate::hip_call_cuda;
        use hip_common::cuda;
        use hip_runtime_sys::*;
        unsafe fn zluda_check_impl(rt_version: u32, timestamp: u64) -> Result<u128, CUresult> {
            let mut device_count = 0i32;
            hip_call_cuda! { hipGetDeviceCount(&mut device_count as _) };
            let driver_version = crate::DRIVER_VERSION as u32;
            let device_attributes = (0..device_count)
                .map(|dev| {
                    let mut device_attributes =
                        mem::zeroed::<zluda_dark_api::AntiZludaHashInputDevice>();
                    cuda! { device::get_uuid(&mut device_attributes.guid, dev)};
                    device::get_attribute(
                        &mut device_attributes.pci_bus as *mut u32 as _,
                        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID,
                        dev,
                    )?;
                    device::get_attribute(
                        &mut device_attributes.pci_domain as *mut u32 as _,
                        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID,
                        dev,
                    )?;
                    device::get_attribute(
                        &mut device_attributes.pci_device as *mut u32 as _,
                        CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID,
                        dev,
                    )?;
                    Ok(device_attributes)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let mut cudart_export_table = ptr::null();
            cuda! { dark_api::get_table(
                &mut cudart_export_table,
                &zluda_dark_api::CudartInterface::GUID as _,
            ) };
            let mut anti_zluda_export_table = ptr::null();
            cuda! { dark_api::get_table(
                &mut anti_zluda_export_table,
                &zluda_dark_api::AntiZluda::GUID as _,
            ) };
            let hash_input = AntiZludaHashInput {
                cudart_export_table: cudart_export_table as _,
                anti_zluda_export_table: anti_zluda_export_table as _,
                fn_ptr: CudaDarkApiZluda::zluda_check as _,
                device_count: device_count as u32,
                driver_version,
                rt_version,
                timestamp,
            };
            let dev_getter = |dev| device_attributes[dev as usize].clone();
            Ok(zluda_dark_api::anti_zluda_hash(
                false, hash_input, dev_getter,
            ))
        }
        match zluda_check_impl(rt_version, timestamp) {
            Ok(hash) => {
                *result = hash;
                CUresult::CUDA_SUCCESS
            }
            Err(e) => e,
        }
    }

    unsafe extern "system" fn get_hip_stream(
        stream: CUstream,
    ) -> CudaResult<*const std::os::raw::c_void> {
        let cuda_object: *mut LiveCheck<stream::StreamData> = stream as *mut stream::Stream;
        stream::as_hip_stream(cuda_object)
            .map(|ptr| ptr as *const _)
            .into()
    }

    unsafe extern "system" fn unwrap_context(
        _ctx: CUcontext,
        is_wrapped: *mut u32,
        _unwrapped_ctx: *mut CUcontext,
    ) -> CUresult {
        *is_wrapped = 0;
        CUresult::CUDA_SUCCESS
    }
}

unsafe fn with_context_or_current<T>(
    ctx: CUcontext,
    fn_: impl FnOnce(&context::ContextData) -> T,
) -> Result<T, CUresult> {
    if ctx == ptr::null_mut() {
        context::with_current(|c| fn_(c))
    } else {
        let ctx = FromCuda::from_cuda(ctx);
        Ok(fn_(LiveCheck::as_result(ctx)?))
    }
}
