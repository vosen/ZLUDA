use ::dark_api::fatbin::FatbinFileIterator;
use ::dark_api::FnFfi;
use cuda_types::cuda::*;
use cuda_types::dark_api::FatbinHeader;
use dark_api::DarkApiState2;
use log::{CudaFunctionName, ErrorEntry};
use parking_lot::ReentrantMutex;
use paste::paste;
use regex::Regex;
use std::cell::{RefCell, RefMut};
use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::{env, error::Error, fs, path::PathBuf, sync::Mutex};
use std::{io, mem, ptr, usize};

extern crate cuda_types;

struct DynamicFn<T> {
    pointer: usize,
    _marker: PhantomData<T>,
}

impl<T> Default for DynamicFn<T> {
    fn default() -> Self {
        DynamicFn {
            pointer: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> DynamicFn<T> {
    unsafe fn get(&mut self, lib: *mut c_void, name: &[u8]) -> Option<T> {
        match self.pointer {
            0 => {
                let addr = os::get_proc_address(lib, CStr::from_bytes_with_nul_unchecked(name));
                if addr == ptr::null_mut() {
                    self.pointer = 1;
                    return None;
                } else {
                    self.pointer = addr as _;
                }
            }
            1 => return None,
            _ => {}
        }
        Some(mem::transmute_copy(&self.pointer))
    }
}

pub(crate) struct CudaDynamicFns {
    lib_handle: NonNull<::std::ffi::c_void>,
    fn_table: CudaFnTable,
}

impl CudaDynamicFns {
    pub(crate) unsafe fn load_library(path: &str) -> Option<Self> {
        let lib_handle = os::dlopen_local_noredirect(path).ok()?;
        Some(CudaDynamicFns {
            lib_handle,
            fn_table: CudaFnTable::default(),
        })
    }
}

unsafe impl Send for CudaDynamicFns {}
unsafe impl Sync for CudaDynamicFns {}

macro_rules! emit_cuda_fn_table {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        #[derive(Default)]
        #[allow(improper_ctypes)]
        #[allow(improper_ctypes_definitions)]
        struct CudaFnTable {
            $($fn_name: DynamicFn<extern $abi fn ( $($arg_id : $arg_type),* ) -> $ret_type>),*
        }

        impl CudaDynamicFns {
            $(
                #[allow(dead_code)]
                pub(crate) fn $fn_name(&mut self, $($arg_id : $arg_type),*) -> Option<$ret_type> {
                    let func = unsafe { self.fn_table.$fn_name.get(self.lib_handle.as_ptr(), concat!(stringify!($fn_name), "\0").as_bytes()) };
                    func.map(|f| f($($arg_id),*) )
                }

                paste::paste! {
                    #[allow(dead_code)]
                    #[allow(non_snake_case)]
                    pub(crate) fn  [<get_ $fn_name>] (&mut self) -> Option<extern $abi fn ( $($arg_type),* ) -> $ret_type> {
                        let func = unsafe { self.fn_table.$fn_name.get(self.lib_handle.as_ptr(), concat!(stringify!($fn_name), "\0").as_bytes()) };
                        func
                    }
                }
            )*
        }
    };
}

macro_rules! override_fn_core {
    ($($abi:literal fn $fn_name: ident ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let format_args = || {
                    let mut formatted_args = Vec::new();
                    (paste! { format :: [<write_ $fn_name>] }) (
                            &mut formatted_args
                            $(,$arg_id)*
                    ).ok();
                    formatted_args
                };
                let extract_fn_ptr = |_: &mut GlobalDelayedState, _: &mut FnCallLog| Some(());
                let cuda_call = |_| {
                    paste!{ [<$fn_name _impl >] ( $($arg_id),* ) }
                };
                GlobalState2::under_lock(
                    CudaFunctionName::Normal(stringify!($fn_name)),
                    Some(format_args),
                    CUresult::INTERNAL_ERROR,
                    format_curesult,
                    extract_fn_ptr,
                    cuda_call,
                    move |_, _, _, _| {}
                )
            }
        )*
    }
}

macro_rules! override_fn_full {
    ($($abi:literal fn $fn_name: ident ( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(non_snake_case)]
            pub unsafe extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                paste!{ [<$fn_name _impl >] ( $($arg_id),* ) }
            }
        )*
    }
}

static INTERNAL_TABLE: ::dark_api::zluda_trace::CudaDarkApiGlobalTable =
    ::dark_api::zluda_trace::CudaDarkApiGlobalTable::new::<InternalTableImpl>();
struct InternalTableImpl;

impl ::dark_api::zluda_trace::CudaDarkApi for InternalTableImpl {
    unsafe extern "system" fn logged_call(
        fn_name: cglue::slice::CSliceRef<'static, u8>,
        args: ::dark_api::FnFfiRef<::dark_api::ByteVecFfi>,
        fn_: ::dark_api::FnFfiRef<usize>,
        internal_error: usize,
        format_status: extern "C" fn(usize) -> ::dark_api::ByteVecFfi,
    ) -> usize {
        GlobalState2::under_lock(
            CudaFunctionName::Normal(unsafe { fn_name.into_str() }),
            Some(|| args.call().to_vec()),
            internal_error,
            |status| format_status(status).to_vec(),
            |_, _| Some(()),
            |_| fn_.call(),
            move |_, _, _, _| {},
        )
    }
}

static EXPORT_TABLE: ::dark_api::cuda::CudaDarkApiGlobalTable =
    ::dark_api::cuda::CudaDarkApiGlobalTable::new::<DarkApiTrace>();

macro_rules! dark_api_fn_redirect_log {
    (
        $iface:ident {
            $([$index:expr] = $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty ),+
        }
    ) => {
            $(
                unsafe extern "system" fn $fn_(
                    $($arg_id: $arg_type),*
                ) -> $ret_type {
                    use zluda_trace_common::ReprUsize;
                    let original_fn = {
                        let dark_api = DARK_API_STATE.lock().unwrap();
                        let (original_table, _) = dark_api
                            .overrides
                            .get(&crate::dark_api::CUuuidWrapper(
                                paste::paste! { ::dark_api::cuda:: [< $iface:camel >] ::GUID  },
                            ))
                            .unwrap();
                        mem::transmute::<
                            _,
                            unsafe extern "system" fn(
                                $($arg_id: $arg_type),*
                            ) -> $ret_type,
                        >(*((*original_table).add($index)))
                    };
                    let format_args = || {
                        let mut formatted_args = Vec::new();
                        ::dark_api::cuda::format::$fn_ (
                                &mut formatted_args
                                $(,$arg_id)*
                        ).ok();
                        formatted_args
                    };
                    let extract_fn_ptr = |_: &mut GlobalDelayedState, _: &mut FnCallLog| { Some(()) };
                    let cuda_call = |_: () | {
                       ReprUsize::to_usize(original_fn( $( $arg_id ),* ))
                    };
                    ReprUsize::from_usize(GlobalState2::under_lock(
                        CudaFunctionName::Dark {
                            guid: paste::paste! { ::dark_api::cuda:: [< $iface:camel >] ::GUID  },
                            index: $index,
                        },
                        Some(format_args),
                        <$ret_type as ReprUsize>::INTERNAL_ERROR,
                        |status| <$ret_type as ReprUsize>::format_status(status).to_vec(),
                        extract_fn_ptr,
                        cuda_call,
                        move |_, _, _, _| {}
                    ))
                }
            )+
    };
}

macro_rules! dark_api_fn_redirect_log_post {
    (
        $iface:ident {
            $([$index:expr] = $fn_:ident ( $($arg_id:ident: $arg_type:ty),* ) -> $ret_type:ty ),+
        }
    ) => {
            $(
                unsafe extern "system" fn $fn_(
                    $($arg_id: $arg_type),*
                ) -> $ret_type {
                    use zluda_trace_common::ReprUsize;
                    let original_fn = {
                        let dark_api = DARK_API_STATE.lock().unwrap();
                        let (original_table, _) = dark_api
                            .overrides
                            .get(&crate::dark_api::CUuuidWrapper(
                                paste::paste! { ::dark_api::cuda:: [< $iface:camel >] ::GUID  },
                            ))
                            .unwrap();
                        mem::transmute::<
                            _,
                            unsafe extern "system" fn(
                                $($arg_id: $arg_type),*
                            ) -> $ret_type,
                        >(*((*original_table).add($index)))
                    };
                    let format_args = || {
                        let mut formatted_args = Vec::new();
                        ::dark_api::cuda::format::$fn_ (
                                &mut formatted_args
                                $(,$arg_id)*
                        ).ok();
                        formatted_args
                    };
                    let extract_fn_ptr = |_: &mut GlobalDelayedState, _: &mut FnCallLog| { Some(()) };
                    let cuda_call = |_: () | {
                       ReprUsize::to_usize(original_fn( $( $arg_id ),* ))
                    };
                    ReprUsize::from_usize(GlobalState2::under_lock(
                        CudaFunctionName::Dark {
                            guid: paste::paste! { ::dark_api::cuda:: [< $iface:camel >] ::GUID  },
                            index: $index,
                        },
                        Some(format_args),
                        <$ret_type as ReprUsize>::INTERNAL_ERROR,
                        |status| <$ret_type as ReprUsize>::format_status(status).to_vec(),
                        extract_fn_ptr,
                        cuda_call,
                        move |state, logger, _, cuda_result| paste! { Self:: [<$fn_ _post>] } ( $( $arg_id ),* , &mut state.cuda_state, logger, <$ret_type as ReprUsize>::from_usize(cuda_result))
                    ))
                }
            )+
    };
}

struct DarkApiTrace;

impl DarkApiTrace {
    fn get_module_from_cubin_post(
        module: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        state: &mut trace::StateTracker,
        fn_logger: &mut FnCallLog,
        _result: CUresult,
    ) {
        fn_logger.try_(|fn_logger| unsafe {
            trace::record_submodules_from_wrapped_fatbin(*module, fatbinc_wrapper, fn_logger, state)
        });
    }

    fn get_module_from_cubin_ext1_post(
        module: *mut cuda_types::cuda::CUmodule,
        fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
        state: &mut trace::StateTracker,
        fn_logger: &mut FnCallLog,
        _result: CUresult,
    ) {
        if arg3 != ptr::null_mut() {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg3",
                expected: vec![UInt::USize(0)],
                observed: UInt::USize(arg3 as usize),
            });
        }
        if arg4 != ptr::null_mut() {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg4",
                expected: vec![UInt::USize(0)],
                observed: UInt::USize(arg4 as usize),
            });
        }
        if arg5 != 0 {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg5",
                expected: vec![UInt::U32(0)],
                observed: UInt::U32(arg5),
            });
        }
        fn_logger.try_(|fn_logger| unsafe {
            trace::record_submodules_from_wrapped_fatbin(*module, fatbinc_wrapper, fn_logger, state)
        });
    }

    fn get_module_from_cubin_ext2_post(
        fatbin_header: *const cuda_types::dark_api::FatbinHeader,
        module: *mut cuda_types::cuda::CUmodule,
        arg3: *mut std::ffi::c_void,
        arg4: *mut std::ffi::c_void,
        arg5: u32,
        state: &mut trace::StateTracker,
        fn_logger: &mut FnCallLog,
        _result: CUresult,
    ) {
        if arg3 != ptr::null_mut() {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg3",
                expected: vec![UInt::USize(0)],
                observed: UInt::USize(arg3 as usize),
            });
        }
        if arg4 != ptr::null_mut() {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg4",
                expected: vec![UInt::USize(0)],
                observed: UInt::USize(arg4 as usize),
            });
        }
        if arg5 != 0 {
            fn_logger.log(log::ErrorEntry::UnexpectedArgument {
                arg_name: "arg5",
                expected: vec![UInt::U32(0)],
                observed: UInt::U32(arg5),
            });
        }
        fn_logger.try_(|fn_logger| unsafe {
            trace::record_submodules(
                *module,
                fn_logger,
                state,
                FatbinFileIterator::new(
                    fatbin_header
                        .as_ref()
                        .ok_or(ErrorEntry::NullPointer("FatbinHeader"))?,
                ),
            )
        });
    }
}

impl ::dark_api::cuda::CudaDarkApi for DarkApiTrace {
    dark_api_fn_redirect_log! {
        CUDART_INTERFACE {
            [2] = cudart_interface_fn2(
                pctx: *mut cuda_types::cuda::CUcontext,
                dev: cuda_types::cuda::CUdevice
            ) -> cuda_types::cuda::CUresult,
            [7] = cudart_interface_fn7(arg1: usize) -> cuda_types::cuda::CUresult
        }
    }

    dark_api_fn_redirect_log_post! {
        CUDART_INTERFACE {
            [1] = get_module_from_cubin(
                module: *mut cuda_types::cuda::CUmodule,
                fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper // FatbincWrapper
            ) -> cuda_types::cuda::CUresult,
            [6] = get_module_from_cubin_ext1(
                result: *mut cuda_types::cuda::CUmodule,
                fatbinc_wrapper: *const cuda_types::dark_api::FatbincWrapper, // FatbincWrapper
                arg3: *mut std::ffi::c_void,
                arg4: *mut std::ffi::c_void,
                arg5: u32
            ) -> cuda_types::cuda::CUresult,
            [8] = get_module_from_cubin_ext2(
                fatbinc_wrapper: *const cuda_types::dark_api::FatbinHeader, // FatbinHeader
                result: *mut cuda_types::cuda::CUmodule,
                arg3: *mut std::ffi::c_void,
                arg4: *mut std::ffi::c_void,
                arg5: u32
            ) -> cuda_types::cuda::CUresult
        }
    }

    dark_api_fn_redirect_log! {
        TOOLS_RUNTIME_CALLBACK_HOOKS {
            [2] = get_unknown_buffer1(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> (),
            [6] = get_unknown_buffer2(ptr: *mut *mut std::ffi::c_void, size: *mut usize) -> ()
        }
    }

    dark_api_fn_redirect_log! {
        CONTEXT_LOCAL_STORAGE_INTERFACE_V0301 {
            [0] = context_local_storage_put(
                context: cuda_types::cuda::CUcontext,
                key: *mut std::ffi::c_void,
                value: *mut std::ffi::c_void,
                // clsContextDestroyCallback, have to be called on cuDevicePrimaryCtxReset
                dtor_cb: Option<extern "system" fn(
                    cuda_types::cuda::CUcontext,
                    *mut std::ffi::c_void,
                    *mut std::ffi::c_void,
                )>
            ) -> cuda_types::cuda::CUresult,
            [1] = context_local_storage_delete(
                context: cuda_types::cuda::CUcontext,
                key: *mut std::ffi::c_void
            ) -> cuda_types::cuda::CUresult,
            [2] = context_local_storage_get(
                value: *mut *mut std::ffi::c_void,
                cu_ctx: cuda_types::cuda::CUcontext,
                key: *mut std::ffi::c_void
            ) -> cuda_types::cuda::CUresult
        }
    }

    dark_api_fn_redirect_log! {
        CTX_CREATE_BYPASS {
            [1] = ctx_create_v2_bypass(
                pctx: *mut cuda_types::cuda::CUcontext,
                flags: ::std::os::raw::c_uint,
                dev: cuda_types::cuda::CUdevice
            ) -> cuda_types::cuda::CUresult
        }
    }

    dark_api_fn_redirect_log! {
        HEAP_ACCESS {
            [1] = heap_alloc(
                heap_alloc_record_ptr: *mut *const std::ffi::c_void, // HeapAllocRecord
                arg2: usize,
                arg3: usize
            ) -> cuda_types::cuda::CUresult,
            [2] = heap_free(
                heap_alloc_record_ptr: *const std::ffi::c_void, // HeapAllocRecord
                arg2: *mut usize
            ) -> cuda_types::cuda::CUresult
        }
    }

    dark_api_fn_redirect_log! {
        DEVICE_EXTENDED_RT {
            [5] = device_get_attribute_ext(
                dev: cuda_types::cuda::CUdevice,
                attribute: std::ffi::c_uint,
                unknown: std::ffi::c_int,
                result: *mut [usize; 2]
            ) -> cuda_types::cuda::CUresult,
            // I don't know is this function return, but on my GTX 1060 it returns 0
            [13] = device_get_something(
                result: *mut std::ffi::c_uchar,
                dev: cuda_types::cuda::CUdevice
            ) -> cuda_types::cuda::CUresult
        }
    }

    unsafe extern "system" fn integrity_check(
        version: u32,
        unix_seconds: u64,
        result: *mut [u64; 2],
    ) -> cuda_types::cuda::CUresult {
        let global_state = crate::GLOBAL_STATE2.lock();
        let global_state_ref_cell = &*global_state;
        let mut global_state_ref_mut = global_state_ref_cell.borrow_mut();
        let global_state = &mut *global_state_ref_mut;
        let log_guard = crate::OuterCallGuard {
            writer: &mut global_state.log_writer,
            log_root: &global_state.log_stack,
        };
        let original_result = {
            let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
                log_stack.enter()
            });
            logger.name = CudaFunctionName::Dark {
                guid: ::dark_api::cuda::IntegrityCheck::GUID,
                index: 1,
            };
            let dark_api = DARK_API_STATE.lock().unwrap();
            let (original_table, override_table) = dark_api
                .overrides
                .get(&crate::dark_api::CUuuidWrapper(
                    ::dark_api::cuda::IntegrityCheck::GUID,
                ))
                .unwrap();
            let original_fn = mem::transmute::<
                _,
                unsafe extern "system" fn(u32, u64, *mut [u64; 2]) -> cuda_types::cuda::CUresult,
            >(*((*original_table).add(1)));
            let original_result = original_fn(version, unix_seconds, result);
            if original_result.is_ok() && version % 10 >= 2 {
                (|| {
                    let (driver_version, devices) =
                        get_cuda_hash_input(&mut global_state.delayed_state)?;
                    let current_process = std::process::id();
                    let current_thread = os::current_thread();
                    let integrity_check_table = override_table.as_ptr().cast();
                    let cudart_table = dark_api
                        .overrides
                        .get(&crate::dark_api::CUuuidWrapper(
                            ::dark_api::cuda::CudartInterface::GUID,
                        ))?
                        .1
                        .as_ptr()
                        .cast();
                    let fn_address = unsafe { *override_table.as_ptr().add(1) };
                    let devices_count = devices.len() as u32;
                    let get_device = |dev| devices[dev as usize];
                    let new_hash = ::dark_api::integrity_check(
                        version,
                        unix_seconds,
                        driver_version,
                        current_process,
                        current_thread,
                        integrity_check_table,
                        cudart_table,
                        fn_address,
                        devices_count,
                        get_device,
                    );
                    logger.log(ErrorEntry::IntegrityCheck {
                        original: *result,
                        overriden: new_hash,
                    });
                    *result = new_hash;
                    Some(())
                })();
            }
            let mut args = Vec::new();
            ::dark_api::cuda::format::integrity_check(&mut args, version, unix_seconds, result)
                .ok();
            logger.args = Some(args);
            let mut output = Vec::new();
            ::format::CudaDisplay::write(&original_result, "", 0, &mut output).ok();
            logger.output = Some(output);
            original_result
        };
        drop(log_guard);
        original_result
    }

    dark_api_fn_redirect_log! {
        UNKNOWN_CHECKS {
            [2] = context_check(
                ctx_in: cuda_types::cuda::CUcontext,
                result1: *mut u32,
                result2: *mut *const std::ffi::c_void
            ) -> cuda_types::cuda::CUresult,
            [3] = check_fn3() -> u32
        }
    }
}

fn get_cuda_hash_input(
    delayed_state: &mut LateInit<GlobalDelayedState>,
) -> Option<(u32, Vec<::dark_api::DeviceHashinfo>)> {
    let delayed_state = delayed_state.as_mut()?;
    let mut driver_version = 0;
    delayed_state
        .libcuda
        .cuDriverGetVersion(&mut driver_version)?
        .ok()?;
    let mut devices = 0;
    delayed_state.libcuda.cuDeviceGetCount(&mut devices)?.ok()?;
    let devices = (0..devices)
        .map(|dev| {
            let mut guid = unsafe { mem::zeroed() };
            delayed_state
                .libcuda
                .cuDeviceGetUuid(&mut guid, dev)?
                .ok()?;
            let mut pci_domain = 0;
            delayed_state
                .libcuda
                .cuDeviceGetAttribute(
                    &mut pci_domain,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID,
                    dev,
                )?
                .ok()?;
            let mut pci_bus = 0;
            delayed_state
                .libcuda
                .cuDeviceGetAttribute(
                    &mut pci_bus,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID,
                    dev,
                )?
                .ok()?;
            let mut pci_device = 0;
            delayed_state
                .libcuda
                .cuDeviceGetAttribute(
                    &mut pci_device,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID,
                    dev,
                )?
                .ok()?;
            Some(::dark_api::DeviceHashinfo {
                guid,
                pci_domain,
                pci_bus,
                pci_device,
            })
        })
        .collect::<Option<Vec<_>>>()?;
    Some((driver_version as u32, devices))
}

#[allow(non_snake_case)]
unsafe fn cuGetExportTable_impl(
    ppExportTable: *mut *const ::core::ffi::c_void,
    pExportTableId: *const cuda_types::cuda::CUuuid,
) -> cuda_types::cuda::CUresult {
    let (result, guid) =
        if let (Some(result), Some(guid)) = (ppExportTable.as_mut(), pExportTableId.as_ref()) {
            if let Some(table) = INTERNAL_TABLE.get(guid) {
                *result = table.start();
                return cuda_types::cuda::CUresult::SUCCESS;
            } else {
                (result, guid)
            }
        } else {
            return cuda_types::cuda::CUresult::ERROR_INVALID_VALUE;
        };
    // I'd rather use GlobalState::under_lock, but that function has a bunch of
    // requirements about types that are very difficult to fulfill here.
    // This particular function does not call any public CUDA functions so it
    // should be all fine
    let global_state = GLOBAL_STATE2.lock();
    let global_state = &mut *global_state.borrow_mut();
    let panic_guard = OuterCallGuard {
        writer: &mut global_state.log_writer,
        log_root: &global_state.log_stack,
    };
    let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
        log_stack.enter()
    });
    logger.name = CudaFunctionName::Normal("cuGetExportTable");
    let delayed_state = match global_state.delayed_state {
        LateInit::Success(ref mut delayed_state) => delayed_state,
        // There's no libcuda to load, so we might as well panic
        LateInit::Error => panic!(),
        LateInit::Unitialized => {
            global_state.delayed_state = GlobalDelayedState::new(panic_guard.writer, &mut logger);
            // `global_state.delayed_state` could be LateInit::Error,
            // we can crash in this case since there's no libcuda
            global_state.delayed_state.as_mut().unwrap()
        }
    };
    let original_fn = match delayed_state.libcuda.get_cuGetExportTable() {
        None => {
            logger.log(ErrorEntry::FunctionNotFound(CudaFunctionName::Normal(
                "cuGetExportTable",
            )));
            return cuda_types::cuda::CUresult::ERROR_UNKNOWN;
        }
        Some(original_fn) => original_fn,
    };
    let original_result = original_fn(ppExportTable, pExportTableId);
    let mut args = Vec::new();
    format::write_cuGetExportTable(&mut args, ppExportTable, pExportTableId).ok();
    logger.args = Some(args);
    logger.output = Some(format_curesult(original_result));
    original_result?;
    let maybe_error = cu_get_export_table_override(result, guid)?;
    if let Some(error) = maybe_error {
        logger.log(error);
    }
    CUresult::SUCCESS
}

static DARK_API_STATE: LazyLock<Mutex<DarkApiState2>> =
    LazyLock::new(|| Mutex::new(DarkApiState2::new()));

fn cu_get_export_table_override(
    result: &mut *const c_void,
    guid: &CUuuid_st,
) -> Result<Option<ErrorEntry>, CUerror> {
    let (new_ptr, error) = {
        let mut state = DARK_API_STATE.lock().unwrap();
        state.override_export_table(&EXPORT_TABLE, (*result).cast(), guid)
    };
    *result = new_ptr.cast();
    Ok(error)
}

#[allow(non_snake_case)]
fn cuGetProcAddress_impl(
    symbol: *const ::core::ffi::c_char,
    pfn: *mut *mut ::core::ffi::c_void,
    cudaVersion: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
) -> cuda_types::cuda::CUresult {
    unsafe { cuGetProcAddress_v2_impl(symbol, pfn, cudaVersion, flags, ptr::null_mut()) }
}

#[allow(non_snake_case)]
unsafe fn cuGetProcAddress_v2_impl(
    symbol: *const ::core::ffi::c_char,
    pfn: *mut *mut ::core::ffi::c_void,
    cudaVersion: ::core::ffi::c_int,
    flags: cuda_types::cuda::cuuint64_t,
    symbolStatus: *mut cuda_types::cuda::CUdriverProcAddressQueryResult,
) -> cuda_types::cuda::CUresult {
    fn raw_match(name: &[u8], flag: u64, version: i32) -> *mut ::core::ffi::c_void {
        include!("../../zluda_bindgen/src/process_table.rs")
    }
    if symbol == ptr::null() {
        return CUresult::ERROR_INVALID_VALUE;
    }
    let symbolStatus = symbolStatus.as_mut();
    let pfn = if let Some(pfn) = pfn.as_mut() {
        pfn
    } else {
        return CUresult::ERROR_INVALID_VALUE;
    };
    let fn_ptr = raw_match(CStr::from_ptr(symbol).to_bytes(), flags, cudaVersion);
    match fn_ptr as usize {
        0 => {
            if let Some(symbolStatus) = symbolStatus {
                *symbolStatus = cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND;
            }
            *pfn = ptr::null_mut();
            CUresult::ERROR_NOT_FOUND
        }
        usize::MAX => {
            if let Some(symbolStatus) = symbolStatus {
                *symbolStatus = cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT;
            }
            *pfn = ptr::null_mut();
            CUresult::ERROR_NOT_FOUND
        }
        _ => {
            if let Some(symbolStatus) = symbolStatus {
                *symbolStatus =
                    cuda_types::cuda::CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS;
            }
            *pfn = fn_ptr;
            Ok(())
        }
    }
}

cuda_function_declarations!(emit_cuda_fn_table);

macro_rules! extern_redirect {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let format_args = || {
                    let mut formatted_args = Vec::new();
                    (paste! { format :: [<write_ $fn_name>] }) (
                            &mut formatted_args
                            $(,$arg_id)*
                    ).ok();
                    formatted_args
                };
                let extract_fn_ptr = |state: &mut GlobalDelayedState, _: &mut FnCallLog| {
                    paste::paste! {
                        state.libcuda. [<get_ $fn_name>]()
                    }
                };
                let cuda_call = |fn_ptr: extern $abi fn ( $($arg_type),* ) -> $ret_type | {
                    fn_ptr( $( $arg_id ),* )
                };
                GlobalState2::under_lock(
                    CudaFunctionName::Normal(stringify!($fn_name)),
                    Some(format_args),
                    CUresult::INTERNAL_ERROR,
                    format_curesult,
                    extract_fn_ptr,
                    cuda_call,
                    move |_, _, _, _| {}
                )
            }
        )*
    };
}

macro_rules! extern_redirect_with_post {
    ($($abi:literal fn $fn_name:ident( $($arg_id:ident : $arg_type:ty),* ) -> $ret_type:ty;)*) => {
        $(
            #[no_mangle]
            #[allow(improper_ctypes_definitions)]
            pub extern $abi fn $fn_name ( $( $arg_id : $arg_type),* ) -> $ret_type {
                let format_args = || {
                    let mut formatted_args = Vec::new();
                    (paste! { format :: [<write_ $fn_name>] }) (
                            &mut formatted_args
                            $(,$arg_id)*
                    ).ok();
                    formatted_args
                };
                let extract_fn_ptr = |state: &mut GlobalDelayedState, _: &mut FnCallLog| {
                    paste::paste! {
                        state.libcuda. [<get_ $fn_name>]()
                    }
                };
                let cuda_call = |fn_ptr: extern $abi fn ( $($arg_type),* ) -> $ret_type | {
                    fn_ptr( $( $arg_id ),* )
                };
                GlobalState2::under_lock(
                    CudaFunctionName::Normal(stringify!($fn_name)),
                    Some(format_args),
                    CUresult::INTERNAL_ERROR,
                    format_curesult,
                    extract_fn_ptr,
                    cuda_call,
                    move |state, logger, _, cuda_result| paste! { [<$fn_name _Post>] } ( $( $arg_id ),* , &mut state.cuda_state, logger, cuda_result )
                )
            }
        )*
    };
}

fn format_curesult(curesult: CUresult) -> Vec<u8> {
    use format::CudaDisplay;
    let mut output_string = Vec::new();
    curesult.write("", usize::MAX, &mut output_string).ok();
    output_string
}

use cuda_macros::cuda_function_declarations;

use crate::log::UInt;
cuda_function_declarations!(
    extern_redirect,
    extern_redirect_with_post
        <= [
            cuModuleLoad,
            cuModuleLoadData,
            cuModuleLoadDataEx,
            cuModuleGetFunction,
            cuDeviceGetAttribute,
            cuDeviceComputeCapability,
            cuModuleLoadFatBinary,
            cuLibraryGetModule,
            cuLibraryLoadData
        ],
    override_fn_core <= [cuGetProcAddress, cuGetProcAddress_v2],
    override_fn_full <= [cuGetExportTable],
);

mod dark_api;
mod log;
#[cfg_attr(windows, path = "os_win.rs")]
#[cfg_attr(not(windows), path = "os_unix.rs")]
mod os;
mod trace;

struct GlobalState2 {
    log_writer: log::Writer,
    log_stack: RefCell<FnCallLogStack>,
    // We split off fields that require a mutable reference to log factory to be
    // created, additionally creation of some fields in this struct can fail
    // initalization (e.g. we passed a non-existant path to libcuda)
    delayed_state: LateInit<GlobalDelayedState>,
}

static GLOBAL_STATE2: LazyLock<ReentrantMutex<RefCell<GlobalState2>>> =
    LazyLock::new(|| ReentrantMutex::new(RefCell::new(GlobalState2::new())));

impl GlobalState2 {
    fn new() -> Self {
        Self {
            log_writer: log::Writer::new(),
            delayed_state: LateInit::Unitialized,
            log_stack: RefCell::new(FnCallLogStack::new()),
        }
    }

    // This function is at the core of the logging mechanism.
    // How it works:
    // When user calls a CUDA function, we want to log the call and its arguments. So in a trace
    // library every public CUDA function will call this function like so:
    //     cuMemAlloc_v2(args) -> under_lock("cuMemAlloc_v2", Some(args), ...)
    // That sounds simple enough, but there are some exotic requirements we have to fulfill:
    // * Reentrancy: CUDA library functions can call other CUDA libary functions and CUDA driver
    //               functions. We need to be able to log all of these calls hierarchically
    // * Thread-safety: CUDA functions can be called from multiple threads
    // * Error-handling: If we fail internally for whatever reason (e.g. we can't load the CUDA
    //                   library, the trace directory is not writable, etc.), we need to log this
    //                   error no matter what
    // Because of that the function is split into three phases:
    // * Pre-call:
    //   We need to load the settings (location of the CUDA libary, trace directory, etc.), write the
    //   function name and its arguments to logging buffer. This whole phase is covered by a drop
    //   guard which will flush the log buffer in case of panic
    // * Call:
    //   We call the actual CUDA function from the actual library. This phase is covered by
    //   a (different) drop guard. We also make sure to drop any &mut and RefMuts we created in the
    //   pre-call phase - this is done to ensure reentrancy. Our CUDA function could call another
    //   CUDA function which would try to acquire the same RefCell
    // * Post-call:
    //   We log the output of the CUDA function and any errors that may have occurred. This phase
    //   is also covered by a drop guard which will flush the log buffer in case of panic
    fn under_lock<'a, FnPtr: Copy, InnerResult: Copy>(
        name: CudaFunctionName,
        args: Option<impl FnOnce() -> Vec<u8>>,
        internal_error: InnerResult,
        format_status: impl FnOnce(InnerResult) -> Vec<u8>,
        pre_call: impl FnOnce(&mut GlobalDelayedState, &mut FnCallLog) -> Option<FnPtr>,
        inner_call: impl FnOnce(FnPtr) -> InnerResult,
        post_call: impl FnOnce(&mut GlobalDelayedState, &mut FnCallLog, FnPtr, InnerResult),
    ) -> InnerResult {
        fn under_lock_impl<'a, FnPtr: Copy, InnerResult: Copy>(
            name: CudaFunctionName,
            args: Option<impl FnOnce() -> Vec<u8>>,
            internal_error: InnerResult,
            format_status: impl FnOnce(InnerResult) -> Vec<u8>,
            pre_call: impl FnOnce(&mut GlobalDelayedState, &mut FnCallLog) -> Option<FnPtr>,
            inner_call: impl FnOnce(FnPtr) -> InnerResult,
            post_call: impl FnOnce(&mut GlobalDelayedState, &mut FnCallLog, FnPtr, InnerResult),
        ) -> InnerResult {
            let global_state = GLOBAL_STATE2.lock();
            let global_state_ref_cell = &*global_state;
            let pre_value = {
                let mut global_state_ref_mut = global_state_ref_cell.borrow_mut();
                let global_state = &mut *global_state_ref_mut;
                let panic_guard = OuterCallGuard {
                    writer: &mut global_state.log_writer,
                    log_root: &global_state.log_stack,
                };
                let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
                    log_stack.enter()
                });
                logger.name = name.clone();
                let delayed_state = match global_state.delayed_state {
                    LateInit::Success(ref mut delayed_state) => delayed_state,
                    // There's no libcuda to load, so we might as well panic
                    LateInit::Error => panic!(),
                    LateInit::Unitialized => {
                        global_state.delayed_state =
                            GlobalDelayedState::new(panic_guard.writer, &mut logger);
                        // `global_state.delayed_state` could be LateInit::Error,
                        // we can crash in this case since there's no libcuda
                        global_state.delayed_state.as_mut().unwrap()
                    }
                };
                let fn_ptr = pre_call(delayed_state, &mut logger);
                match fn_ptr {
                    Some(fn_ptr) => {
                        mem::forget(panic_guard);
                        fn_ptr
                    }
                    None => {
                        logger.log(ErrorEntry::FunctionNotFound(name));
                        return internal_error;
                    }
                }
            };
            let panic_guard = InnerCallGuard(global_state_ref_cell);
            let inner_result = inner_call(pre_value);
            let global_state = &mut *global_state_ref_cell.borrow_mut();
            mem::forget(panic_guard);
            let _drop_guard = OuterCallGuard {
                writer: &mut global_state.log_writer,
                log_root: &global_state.log_stack,
            };
            let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
                log_stack.resume()
            });
            logger.args = args.map(|args| args());
            logger.output = Some(format_status(inner_result));
            post_call(
                global_state.delayed_state.as_mut().unwrap(),
                &mut logger,
                pre_value,
                inner_result,
            );
            inner_result
        }
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            under_lock_impl(
                name,
                args,
                internal_error,
                format_status,
                pre_call,
                inner_call,
                post_call,
            )
        }))
        .unwrap_or(internal_error)
    }
}

trait CudaResult: Copy + format::CudaDisplay {
    const INTERNAL_ERROR: Self;
}

impl CudaResult for CUresult {
    const INTERNAL_ERROR: Self = CUresult::ERROR_UNKNOWN;
}

struct FnCallLogStack {
    depth: usize,
    log_root: FnCallLog,
}

impl FnCallLogStack {
    fn new() -> Self {
        Self {
            depth: 0,
            log_root: FnCallLog::new(),
        }
    }

    fn enter<'a>(&'a mut self) -> &'a mut FnCallLog {
        let depth = self.depth;
        self.depth += 1;
        let mut current = &mut self.log_root;
        match depth {
            0 => {}
            depth => {
                for _ in 0..(depth - 1) {
                    current = current
                        .subcalls
                        .iter_mut()
                        .rev()
                        .find_map(|entry| match entry {
                            LogEntry::FnCall(fn_call_log) => Some(fn_call_log),
                            LogEntry::Error(_) => None,
                        })
                        .unwrap();
                }
                current.subcalls.push(LogEntry::FnCall(FnCallLog::new()));
                match current.subcalls.last_mut().unwrap() {
                    LogEntry::FnCall(call) => {
                        current = call;
                    }
                    &mut LogEntry::Error(_) => unreachable!(),
                }
            }
        };
        current
    }

    fn resume<'a>(&'a mut self) -> &'a mut FnCallLog {
        let mut current = &mut self.log_root;
        match self.depth {
            0 | 1 => {}
            depth => {
                for _ in 0..(depth - 1) {
                    current = current
                        .subcalls
                        .iter_mut()
                        .rev()
                        .find_map(|entry| match entry {
                            LogEntry::FnCall(fn_call_log) => Some(fn_call_log),
                            LogEntry::Error(_) => None,
                        })
                        .unwrap();
                }
            }
        };
        current
    }
}

struct FnCallLog {
    name: CudaFunctionName,
    args: Option<Vec<u8>>,
    output: Option<Vec<u8>>,
    subcalls: Vec<LogEntry>,
}

impl FnCallLog {
    fn new() -> Self {
        Self {
            name: CudaFunctionName::Normal(""),
            args: None,
            output: None,
            subcalls: Vec::new(),
        }
    }

    fn try_return<T>(&mut self, fn_: impl FnOnce() -> Result<T, ErrorEntry>) -> Option<T> {
        match fn_() {
            Err(err) => {
                self.subcalls.push(LogEntry::Error(err));
                None
            }
            Ok(x) => Some(x),
        }
    }

    fn try_<T>(&mut self, f: impl FnOnce(&mut Self) -> Result<T, ErrorEntry>) -> Option<T> {
        match f(self) {
            Err(e) => {
                self.log(e);
                None
            }
            Ok(x) => Some(x),
        }
    }

    fn log(&mut self, err: ErrorEntry) {
        self.subcalls.push(LogEntry::Error(err));
    }

    pub(crate) fn log_io_error(&mut self, error: io::Result<()>) {
        if let Err(e) = error {
            self.subcalls.push(LogEntry::Error(ErrorEntry::IoError(e)));
        }
    }

    fn reset(&mut self) {
        self.name = CudaFunctionName::Normal("");
        self.args = None;
        self.output = None;
        self.subcalls.clear();
    }
}

struct OuterCallGuard<'a> {
    writer: &'a mut log::Writer,
    log_root: &'a RefCell<FnCallLogStack>,
}

impl<'a> Drop for OuterCallGuard<'a> {
    fn drop(&mut self) {
        let mut log_root = self.log_root.borrow_mut();
        log_root.depth -= 1;
        if log_root.depth == 0 {
            self.writer.write_and_flush(&mut log_root.log_root);
        }
    }
}

struct InnerCallGuard<'a>(&'a RefCell<GlobalState2>);

impl<'a> Drop for InnerCallGuard<'a> {
    fn drop(&mut self) {
        let mut global_state = self.0.borrow_mut();
        let global_state = &mut *global_state;
        let mut log_root = global_state.log_stack.borrow_mut();
        log_root.depth -= 1;
        if log_root.depth == 0 {
            global_state
                .log_writer
                .write_and_flush(&mut log_root.log_root);
        }
    }
}

enum LogEntry {
    FnCall(FnCallLog),
    Error(ErrorEntry),
}

enum LateInit<T> {
    Success(T),
    Unitialized,
    Error,
}

impl<T> LateInit<T> {
    fn as_mut(&mut self) -> Option<&mut T> {
        match self {
            Self::Success(t) => Some(t),
            Self::Unitialized => None,
            Self::Error => None,
        }
    }
}

struct GlobalDelayedState {
    libcuda: CudaDynamicFns,
    cuda_state: trace::StateTracker,
}

impl GlobalDelayedState {
    fn new<'a>(log_writer: &mut log::Writer, logger: &mut FnCallLog) -> LateInit<Self> {
        let settings = Settings::read_and_init(logger);
        logger.try_return(|| log_writer.late_init(&settings));
        let libcuda = match unsafe { CudaDynamicFns::load_library(&settings.libcuda_path) } {
            Some(libcuda) => libcuda,
            None => {
                logger.log(log::ErrorEntry::ErrorBox(
                    format!("Invalid CUDA library at path {}", &settings.libcuda_path).into(),
                ));
                return LateInit::Error;
            }
        };
        let cuda_state = trace::StateTracker::new(&settings);
        let delayed_state = GlobalDelayedState {
            libcuda,
            cuda_state,
        };
        LateInit::Success(delayed_state)
    }
}

struct Settings {
    dump_dir: Option<PathBuf>,
    libcuda_path: String,
    override_cc: Option<(u32, u32)>,
}

impl Settings {
    fn read_and_init(logger: &mut FnCallLog) -> Self {
        fn parse_compute_capability(env_string: &str) -> Option<(u32, u32)> {
            let regex = Regex::new(r"(\d+)\.(\d+)").unwrap();
            let captures = regex.captures(&env_string)?;
            let major = captures.get(1)?;
            let major = str::parse::<u32>(major.as_str()).ok()?;
            let minor = captures.get(2)?;
            let minor = str::parse::<u32>(minor.as_str()).ok()?;
            Some((major, minor))
        }

        let maybe_dump_dir = Self::read_and_init_dump_dir();
        let dump_dir = match maybe_dump_dir {
            Ok(Some(dir)) => {
                logger.log(log::ErrorEntry::CreatedDumpDirectory(dir.clone()));
                Some(dir)
            }
            Ok(None) => None,
            Err(err) => {
                logger.log(log::ErrorEntry::ErrorBox(err));
                None
            }
        };
        let libcuda_path = match env::var("ZLUDA_CUDA_LIB") {
            Err(env::VarError::NotPresent) => os::LIBCUDA_DEFAULT_PATH.to_string(),
            Err(e) => {
                logger.log(log::ErrorEntry::ErrorBox(Box::new(e) as _));
                os::LIBCUDA_DEFAULT_PATH.to_string()
            }
            Ok(env_string) => env_string,
        };
        let override_cc = match env::var("ZLUDA_OVERRIDE_COMPUTE_CAPABILITY") {
            Err(env::VarError::NotPresent) => None,
            Err(e) => {
                logger.log(log::ErrorEntry::ErrorBox(Box::new(e) as _));
                None
            }
            Ok(env_string) => logger.try_return(|| {
                parse_compute_capability(&env_string).ok_or_else(|| ErrorEntry::InvalidEnvVar {
                    var: "ZLUDA_OVERRIDE_COMPUTE_CAPABILITY",
                    pattern: "MAJOR.MINOR",
                    value: env_string,
                })
            }),
        };
        Settings {
            dump_dir,
            libcuda_path,
            override_cc,
        }
    }

    fn read_and_init_dump_dir() -> Result<Option<PathBuf>, Box<dyn Error>> {
        let dir = match env::var("ZLUDA_LOG_DIR") {
            Ok(dir) => dir,
            Err(env::VarError::NotPresent) => return Ok(None),
            Err(err) => return Err(Box::new(err) as Box<_>),
        };
        Ok(Some(Self::create_trace_directory(dir)?))
    }

    fn create_trace_directory(dir: String) -> io::Result<PathBuf> {
        let mut main_dir = PathBuf::from(dir);
        let current_exe = env::current_exe()?;
        let file_name_base = current_exe.file_name().unwrap().to_string_lossy();
        main_dir.push(&*file_name_base);
        let mut suffix = 1;
        while main_dir.try_exists()? {
            main_dir.set_file_name(format!("{}_{}", file_name_base, suffix));
            suffix += 1;
        }
        fs::create_dir_all(&*main_dir)?;
        Ok(main_dir)
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoad_Post(
    module: *mut CUmodule,
    fname: *const ::std::os::raw::c_char,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    state.record_new_module_file(unsafe { *module }, fname, fn_logger)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadData_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    state.record_new_module(unsafe { *module }, raw_image, fn_logger)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadDataEx_Post(
    module: *mut CUmodule,
    raw_image: *const ::std::os::raw::c_void,
    _numOptions: ::std::os::raw::c_uint,
    _options: *mut CUjit_option,
    _optionValues: *mut *mut ::std::os::raw::c_void,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    result: CUresult,
) {
    cuModuleLoadData_Post(module, raw_image, state, fn_logger, result)
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleGetFunction_Post(
    _hfunc: *mut CUfunction,
    _hmod: CUmodule,
    _name: *const ::std::os::raw::c_char,
    _state: &mut trace::StateTracker,
    _fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceGetAttribute_Post(
    pi: *mut ::std::os::raw::c_int,
    attrib: CUdevice_attribute,
    _dev: CUdevice,
    state: &mut trace::StateTracker,
    _fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    if attrib == CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR {
        if let Some((major_override, _)) = state.override_cc {
            unsafe {
                *pi = major_override as i32;
            };
        }
    }
    if attrib == CUdevice_attribute::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR {
        if let Some((_, minor_override)) = state.override_cc {
            unsafe {
                *pi = minor_override as i32;
            };
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuDeviceComputeCapability_Post(
    major: *mut ::std::os::raw::c_int,
    minor: *mut ::std::os::raw::c_int,
    _dev: CUdevice,
    state: &mut trace::StateTracker,
    _fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    if let Some((major_override, minor_override)) = state.override_cc {
        unsafe {
            *major = major_override as i32;
            *minor = minor_override as i32;
        };
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuModuleLoadFatBinary_Post(
    module: *mut CUmodule,
    fatbin_header: *const ::std::os::raw::c_void,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    fn_logger.try_(|fn_logger| unsafe {
        trace::record_submodules(
            *module,
            fn_logger,
            state,
            FatbinFileIterator::new(
                fatbin_header
                    .cast::<FatbinHeader>()
                    .as_ref()
                    .ok_or(ErrorEntry::NullPointer("FatbinHeader"))?,
            ),
        )
    });
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryGetModule_Post(
    module: *mut cuda_types::cuda::CUmodule,
    library: cuda_types::cuda::CUlibrary,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    match state.libraries.get(&library).copied() {
        None => fn_logger.log(log::ErrorEntry::UnknownLibrary(library)),
        Some(code) => {
            fn_logger.try_(|fn_logger| unsafe {
                trace::record_submodules_from_wrapped_fatbin(
                    *module,
                    code.0.cast(),
                    fn_logger,
                    state,
                )
            });
        }
    }
}

#[allow(non_snake_case)]
pub(crate) fn cuLibraryLoadData_Post(
    library: *mut cuda_types::cuda::CUlibrary,
    code: *const ::core::ffi::c_void,
    _jit_options: *mut cuda_types::cuda::CUjit_option,
    _jit_options_values: *mut *mut ::core::ffi::c_void,
    _num_jit_options: ::core::ffi::c_uint,
    _library_options: *mut cuda_types::cuda::CUlibraryOption,
    _library_option_values: *mut *mut ::core::ffi::c_void,
    _num_library_options: ::core::ffi::c_uint,
    state: &mut trace::StateTracker,
    fn_logger: &mut FnCallLog,
    _result: CUresult,
) {
    state
        .libraries
        .insert(unsafe { *library }, trace::CodePointer(code));
    // TODO: this is not correct, but it's enough for now, we just want to
    // save the binary to disk
    state.record_new_module(unsafe { CUmodule((*library).0.cast()) }, code, fn_logger); 
}
