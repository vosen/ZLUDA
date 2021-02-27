use crate::format;
extern crate zluda_dark_api;
use crate::log::LogEntry;
use crate::{log, os};
use cuda_types::*;
use std::hash::Hash;
use std::{
    collections::{hash_map, HashMap},
    ffi::c_void,
    mem,
    os::raw::c_uint,
    slice,
};
use std::{iter, ptr};
use zluda_dark_api::CUmoduleContent;
use zluda_dark_api::{AntiZludaHashInput, CudaDarkApiDump, CudaDarkApiKnownExports};

const MINIMUM_FUNCTION_ADDRESS: usize = 0x1000;

pub(crate) struct DarkApiState {
    // Key is Box<CUuuid>, because thunk fn neeads a stable memory location for
    // the guid
    overrides: HashMap<Box<[u8; 16]>, (Vec<*const c_void>, *const *const c_void)>,
    known_exports: CudaDarkApiKnownExports,
}

#[derive(Eq, PartialEq)]
pub(crate) struct CUuuidWrapper(pub CUuuid);

impl Hash for CUuuidWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.bytes.hash(state);
    }
}

impl DarkApiState {
    pub(crate) fn new() -> Self {
        DarkApiState {
            overrides: HashMap::new(),
            known_exports: CudaDarkApiKnownExports::new(),
        }
    }
}
pub(crate) fn override_export_table(
    fn_logger: &mut log::FunctionLogger,
    pp_export_table: *mut *const c_void,
    p_export_table_id: *const CUuuid,
    state: &mut crate::trace::StateTracker,
) {
    let state = &mut state.dark_api;
    let guid = Box::new(unsafe { *p_export_table_id }.bytes);
    let export_table_mut = unsafe { &mut *pp_export_table };
    *export_table_mut = match state.overrides.entry(guid) {
        hash_map::Entry::Occupied(entry) => entry.get().0.as_ptr() as *const _,
        hash_map::Entry::Vacant(entry) => {
            let known_fns_bitmap = state.known_exports.known(entry.key());
            let byte_length = unsafe { *{ (*export_table_mut) as *const usize } };
            let length;
            if byte_length >= MINIMUM_FUNCTION_ADDRESS {
                // not length prefixed
                if known_fns_bitmap.len() > 0 {
                    // take "our" length
                    length = known_fns_bitmap.len();
                } else {
                    // guess
                    length = (0..usize::max_value())
                        .position(|offset| unsafe {
                            *((*export_table_mut) as *mut usize).add(offset)
                                < MINIMUM_FUNCTION_ADDRESS
                        })
                        .unwrap()
                        + 1; // include terminator just in case
                    fn_logger.log(LogEntry::ExportTableLengthGuess(length));
                }
            } else {
                length = byte_length / mem::size_of::<*mut c_void>();
                if known_fns_bitmap.len() > 0 && known_fns_bitmap.len() != length {
                    fn_logger.log(LogEntry::ExportTableLengthMismatch {
                        expected: known_fns_bitmap.len(),
                        observed: length,
                    });
                }
            }
            let overriden_table = unsafe {
                get_overriden_export_table(
                    entry.key(),
                    // padding with false in case the export table got longer in a newer version
                    known_fns_bitmap.chain(iter::repeat(false)),
                    slice::from_raw_parts(*pp_export_table as _, length)
                        .iter()
                        .copied(),
                )
            };
            entry
                .insert((overriden_table, unsafe { *pp_export_table } as _))
                .0
                .as_ptr() as _
        }
    }
}

unsafe fn get_overriden_export_table(
    guid: &Box<[u8; 16]>,
    known_exports: impl Iterator<Item = bool>,
    original_exports: impl Iterator<Item = *const c_void>,
) -> Vec<*const c_void> {
    let mut export_table = Vec::new();
    for (idx, (original_fn, is_known)) in original_exports.zip(known_exports).enumerate() {
        let override_fn = if !is_known {
            os::get_thunk(original_fn, report_unknown_export_table_call, &**guid, idx)
        } else {
            match zluda_dark_api::get_dark_api_fn::<CudaDarkApiDumpFns>(&guid, idx) {
                Some(fn_ptr) => fn_ptr,
                None => {
                    if (original_fn as usize) < MINIMUM_FUNCTION_ADDRESS {
                        original_fn
                    } else {
                        os::get_thunk(original_fn, report_known_export_table_call, &**guid, idx)
                    }
                }
            }
        };
        export_table.push(override_fn);
    }
    export_table
}

unsafe extern "system" fn report_unknown_export_table_call(
    export_table: *const [u8; 16],
    idx: usize,
) {
    if let Ok(mut global_state) = crate::GLOBAL_STATE.lock() {
        let mut logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: *export_table,
            },
            idx,
            None,
        );
        logger.log(log::LogEntry::UnknownExportTableFn)
    }
}

unsafe extern "system" fn report_known_export_table_call(
    export_table: *const [u8; 16],
    idx: usize,
) {
    if let Ok(mut global_state) = crate::GLOBAL_STATE.lock() {
        let _logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: *export_table,
            },
            idx,
            None,
        );
    }
}

struct CudaDarkApiDumpFns;

impl CudaDarkApiDump for CudaDarkApiDumpFns {
    unsafe fn get_module_from_cubin_impl(
        guid: &[u8; 16],
        idx: usize,
        module: *mut CUmodule,
        fatbinc_wrapper: *const zluda_dark_api::FatbincWrapper,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(result).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&module, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(fatbinc_wrapper).as_bytes())?;
            write!(writer, ": {:p})", fatbinc_wrapper)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                *mut CUmodule,
                *const zluda_dark_api::FatbincWrapper,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(module, fatbinc_wrapper);
        fn_logger.result = Some(original_result);
        if !matches!(
            original_result,
            CUresult::CUDA_SUCCESS
                | CUresult::CUDA_ERROR_INVALID_PTX
                | CUresult::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
                | CUresult::CUDA_ERROR_NOT_SUPPORTED,
        ) {
            return original_result;
        }
        let maybe_fatbin = fn_logger.log_unwrap(
            zluda_dark_api::CudaFatbin::from_wrapper(fatbinc_wrapper).map_err(Into::into),
        );
        if let Some(fatbin) = maybe_fatbin {
            cuda_state.record_module(
                &mut fn_logger,
                Some(*module),
                CUmoduleContent::Fatbin(fatbin),
            );
        }
        original_result
    }

    unsafe fn get_module_from_cubin_ex1_impl(
        guid: &[u8; 16],
        idx: usize,
        module: *mut CUmodule,
        fatbinc_wrapper: *const zluda_dark_api::FatbincWrapper,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: usize,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(module).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&module, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(fatbinc_wrapper).as_bytes())?;
            write!(writer, ": {:p}, ", fatbinc_wrapper)?;
            writer.write_all(stringify!(arg3).as_bytes())?;
            write!(writer, ": {:p}, ", arg3)?;
            writer.write_all(stringify!(arg4).as_bytes())?;
            write!(writer, ": {:p}, ", arg4)?;
            writer.write_all(stringify!(arg5).as_bytes())?;
            write!(writer, ": {})", arg5)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                *mut CUmodule,
                *const zluda_dark_api::FatbincWrapper,
                *mut c_void,
                *mut c_void,
                usize,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(module, fatbinc_wrapper, arg3, arg4, arg5);
        fn_logger.result = Some(original_result);
        if !matches!(
            original_result,
            CUresult::CUDA_SUCCESS
                | CUresult::CUDA_ERROR_INVALID_PTX
                | CUresult::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
                | CUresult::CUDA_ERROR_NOT_SUPPORTED,
        ) {
            return original_result;
        }
        let maybe_fatbin = fn_logger.log_unwrap(
            zluda_dark_api::CudaFatbin::from_wrapper(fatbinc_wrapper).map_err(Into::into),
        );
        if let Some(fatbin) = maybe_fatbin {
            cuda_state.record_module(
                &mut fn_logger,
                Some(*module),
                CUmoduleContent::Fatbin(fatbin),
            );
        }
        original_result
    }

    unsafe fn get_module_from_cubin_ex2_impl(
        guid: &[u8; 16],
        idx: usize,
        fatbin_header: *const zluda_dark_api::FatbinHeader,
        module: *mut CUmodule,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: c_uint,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(fatbin_header).as_bytes())?;
            write!(writer, ": {:p}, ", fatbin_header)?;
            writer.write_all(stringify!(module).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&module, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(arg3).as_bytes())?;
            write!(writer, ": {:p}, ", arg3)?;
            writer.write_all(stringify!(arg4).as_bytes())?;
            write!(writer, ": {:p}, ", arg4)?;
            writer.write_all(stringify!(arg5).as_bytes())?;
            write!(writer, ": {})", arg5)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                *const zluda_dark_api::FatbinHeader,
                *mut CUmodule,
                *mut c_void,
                *mut c_void,
                c_uint,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(fatbin_header, module, arg3, arg4, arg5);
        fn_logger.result = Some(original_result);
        if !matches!(
            original_result,
            CUresult::CUDA_SUCCESS
                | CUresult::CUDA_ERROR_INVALID_PTX
                | CUresult::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
                | CUresult::CUDA_ERROR_NOT_SUPPORTED,
        ) {
            return original_result;
        }
        cuda_state.record_module(
            &mut fn_logger,
            Some(*module),
            CUmoduleContent::Fatbin(zluda_dark_api::CudaFatbin::from_header(fatbin_header)),
        );
        original_result
    }

    #[allow(non_snake_case)]
    unsafe fn launch_kernel_impl(
        guid: &[u8; 16],
        idx: usize,
        f: CUfunction,
        gridDimX: ::std::os::raw::c_uint,
        gridDimY: ::std::os::raw::c_uint,
        gridDimZ: ::std::os::raw::c_uint,
        blockDimX: ::std::os::raw::c_uint,
        blockDimY: ::std::os::raw::c_uint,
        blockDimZ: ::std::os::raw::c_uint,
        sharedMemBytes: ::std::os::raw::c_uint,
        hStream: CUstream,
        extra: *mut *mut ::std::os::raw::c_void,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(f).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&f, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(gridDimX).as_bytes())?;
            write!(writer, ": {}, ", gridDimX)?;
            writer.write_all(stringify!(gridDimY).as_bytes())?;
            write!(writer, ": {}, ", gridDimY)?;
            writer.write_all(stringify!(gridDimZ).as_bytes())?;
            write!(writer, ": {}, ", gridDimZ)?;
            writer.write_all(stringify!(blockDimX).as_bytes())?;
            write!(writer, ": {}, ", blockDimX)?;
            writer.write_all(stringify!(blockDimY).as_bytes())?;
            write!(writer, ": {}, ", blockDimY)?;
            writer.write_all(stringify!(blockDimZ).as_bytes())?;
            write!(writer, ": {}, ", blockDimZ)?;
            writer.write_all(stringify!(sharedMemBytes).as_bytes())?;
            write!(writer, ": {}, ", sharedMemBytes)?;
            writer.write_all(stringify!(hStream).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&hStream, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(extra).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&extra, "cuLaunchKernel", 9, writer)?;
            writer.write_all(b")")
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                CUfunction,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                ::std::os::raw::c_uint,
                CUstream,
                *mut *mut ::std::os::raw::c_void,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(
            f,
            gridDimX,
            gridDimY,
            gridDimZ,
            blockDimX,
            blockDimY,
            blockDimZ,
            sharedMemBytes,
            hStream,
            extra,
        );
        fn_logger.result = Some(original_result);
        original_result
    }

    unsafe fn ctx_create_v2_bypass_impl(
        _guid: &[u8; 16],
        _idx: usize,
        _pctx: *mut cuda_types::CUcontext,
        _flags: c_uint,
        _dev: cuda_types::CUdevice,
    ) -> CUresult {
        todo!()
    }

    unsafe fn dlss_feature_evaluate_init_impl(
        guid: &[u8; 16],
        idx: usize,
        retval1: *mut *mut c_void,
        handle: *mut c_void,
        retval2: *mut *mut c_void,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(retval1).as_bytes())?;
            write!(writer, ": {:p}, ", deref_not_null(retval1))?;
            writer.write_all(stringify!(handle).as_bytes())?;
            write!(writer, ": {:p}, ", handle)?;
            writer.write_all(stringify!(retval2).as_bytes())?;
            write!(writer, ": {:p})", deref_not_null(retval2))
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(*mut *mut c_void, *mut c_void, *mut *mut c_void) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(retval1, handle, retval2);
        fn_logger.result = Some(original_result);
        original_result
    }

    unsafe fn dlss_cuInit_impl(guid: &[u8; 16], idx: usize) -> CUresult {
        let arguments_writer =
            Box::new(move |writer: &mut dyn std::io::Write| writer.write_all(b"()"));
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn =
            mem::transmute::<_, unsafe extern "system" fn() -> CUresult>(*original_ptr);
        let original_result = original_fn();
        fn_logger.result = Some(original_result);
        original_result
    }

    unsafe fn dlss_start1_impl(
        guid: &[u8; 16],
        idx: usize,
        retval1: *mut *mut c_void,
        arg2: *mut c_void,
        arg3: *mut c_void,
        arg4: *mut c_void,
        arg5: *mut c_void,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(retval1).as_bytes())?;
            write!(writer, ": {:p}, ", deref_not_null(retval1))?;
            writer.write_all(stringify!(arg2).as_bytes())?;
            write!(writer, ": {:p}, ", arg2)?;
            writer.write_all(stringify!(arg3).as_bytes())?;
            write!(writer, ": {:p}, ", arg3)?;
            writer.write_all(stringify!(arg4).as_bytes())?;
            write!(writer, ": {:p}, ", arg4)?;
            writer.write_all(stringify!(arg5).as_bytes())?;
            write!(writer, ": {:p})", arg5)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                *mut *mut c_void,
                *mut c_void,
                *mut c_void,
                *mut c_void,
                *mut c_void,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(retval1, arg2, arg3, arg4, arg5);
        fn_logger.result = Some(original_result);
        original_result
    }

    #[allow(non_snake_case)]
    unsafe fn dlss_start2_impl(
        guid: &[u8; 16],
        idx: usize,
        handle: *mut c_void,
        retval: *mut u32,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(handle).as_bytes())?;
            write!(writer, ": {:p}, ", handle)?;
            writer.write_all(stringify!(retval).as_bytes())?;
            if retval == ptr::null_mut() {
                write!(writer, ": {:p})", retval)
            } else {
                write!(writer, ": {})", *retval)
            }
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(*mut c_void, *mut u32) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(handle, retval);
        fn_logger.result = Some(original_result);
        original_result
    }

    #[allow(non_snake_case)]
    unsafe fn dlss_feature_evaluate1_impl(
        guid: &[u8; 16],
        idx: usize,
        retval1: *mut u32,
        retval2: *mut u32,
        retval3: *mut u32,
        handle: *mut c_void,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(retval1).as_bytes())?;
            writer.write_all(b": ")?;
            write_deref(writer, retval1)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(retval2).as_bytes())?;
            writer.write_all(b": ")?;
            write_deref(writer, retval2)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(retval3).as_bytes())?;
            writer.write_all(b": ")?;
            write_deref(writer, retval3)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(handle).as_bytes())?;
            write!(writer, ": {:p})", handle)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(*mut u32, *mut u32, *mut u32, *mut c_void) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(retval1, retval2, retval3, handle);
        fn_logger.result = Some(original_result);
        original_result
    }

    #[allow(non_snake_case)]
    unsafe fn dlss_feature_evaluate2_impl(
        guid: &[u8; 16],
        idx: usize,
        handle1: *mut c_void,
        handle2: *mut c_void,
        handle3: *mut c_void,
        arg4: u8,
        handle5: *mut c_void,
        arg6: u32,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(handle1).as_bytes())?;
            write!(writer, ": {:p}, ", handle1)?;
            writer.write_all(stringify!(handle2).as_bytes())?;
            write!(writer, ": {:p}, ", handle2)?;
            writer.write_all(stringify!(handle3).as_bytes())?;
            write!(writer, ": {:p}, ", handle3)?;
            writer.write_all(stringify!(arg4).as_bytes())?;
            write!(writer, ": {}, ", arg4)?;
            writer.write_all(stringify!(handle5).as_bytes())?;
            write!(writer, ": {:p}, ", handle5)?;
            writer.write_all(stringify!(arg6).as_bytes())?;
            write!(writer, ": {})", arg6)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                *mut c_void,
                *mut c_void,
                *mut c_void,
                u8,
                *mut c_void,
                u32,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(handle1, handle2, handle3, arg4, handle5, arg6);
        fn_logger.result = Some(original_result);
        original_result
    }

    unsafe fn dlss_module_load_impl(
        guid: &[u8; 16],
        idx: usize,
        context: CUcontext,
        result: *mut CUmodule,
        fatbin: *mut c_void,
        arg4: u32,
        arg5: *mut c_void,
        arg6: *mut c_void,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(context).as_bytes())?;
            write!(writer, ": {:p}, ", context)?;
            writer.write_all(stringify!(result).as_bytes())?;
            write!(writer, ": {:p}, ", deref_not_null(result))?;
            writer.write_all(stringify!(fatbin).as_bytes())?;
            write!(writer, ": {:p}, ", fatbin)?;
            writer.write_all(stringify!(arg4).as_bytes())?;
            write!(writer, ": {}, ", arg4)?;
            writer.write_all(stringify!(arg5).as_bytes())?;
            write!(writer, ": {:p}, ", arg5)?;
            writer.write_all(stringify!(arg6).as_bytes())?;
            write!(writer, ": {:p})", arg6)
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(
                CUcontext,
                *mut CUmodule,
                *mut c_void,
                u32,
                *mut c_void,
                *mut c_void,
            ) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(context, result, fatbin, arg4, arg5, arg6);
        fn_logger.result = Some(original_result);
        if !matches!(
            original_result,
            CUresult::CUDA_SUCCESS
                | CUresult::CUDA_ERROR_INVALID_PTX
                | CUresult::CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE
                | CUresult::CUDA_ERROR_NOT_SUPPORTED,
        ) {
            return original_result;
        }
        cuda_state.record_module(
            &mut fn_logger,
            Some(*result),
            CUmoduleContent::Fatbin(zluda_dark_api::CudaFatbin::from_header(fatbin.cast())),
        );
        original_result
    }

    unsafe fn dlss_module_get_function_impl(
        guid: &[u8; 16],
        idx: usize,
        result: *mut CUfunction,
        module: CUmodule,
        name: *const i8,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(result).as_bytes())?;
            write!(writer, ": {:p}, ", deref_not_null(result))?;
            writer.write_all(stringify!(module).as_bytes())?;
            write!(writer, ": {:p}, ", module)?;
            writer.write_all(stringify!(name).as_bytes())?;
            write!(writer, ": ")?;
            format::CudaDisplay::write(&name, "", 0, writer)?;
            write!(writer, ")")
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(*mut CUfunction, CUmodule, *const i8) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(result, module, name);
        fn_logger.result = Some(original_result);
        original_result
    }

    #[allow(non_snake_case)]
    unsafe fn zluda_check_impl(
        guid: &[u8; 16],
        idx: usize,
        rt_version: u32,
        timestamp: u64,
        result: *mut u128,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(")?;
            writer.write_all(stringify!(rt_version).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&rt_version, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(timestamp).as_bytes())?;
            writer.write_all(b": ")?;
            format::CudaDisplay::write(&timestamp, "", 0, writer)?;
            writer.write_all(b", ")?;
            writer.write_all(stringify!(result).as_bytes())?;
            writer.write_all(b": ")?;
            write_deref(writer, result)?;
            write!(writer, ")")
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let delayed_state = global_state.delayed_state.unwrap_mut();
        let libcuda = &mut delayed_state.libcuda;
        let cuda_state = &mut delayed_state.cuda_state;
        let (overriding_table, original_table) = &cuda_state.dark_api.overrides[guid];
        let original_ptr = original_table.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(u32, u64, *mut u128) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(rt_version, timestamp, result);
        let mut device_count = 0i32;
        libcuda.cuDeviceGetCount(&mut device_count as _);
        let device_attributes = (0..device_count)
            .map(|dev| {
                let dev = CUdevice_v1(dev);
                let mut device_attributes =
                    mem::zeroed::<zluda_dark_api::AntiZludaHashInputDevice>();
                libcuda.cuDeviceGetUuid(&mut device_attributes.guid, dev);
                libcuda.cuDeviceGetAttribute(
                    &mut device_attributes.pci_bus as *mut u32 as _,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_BUS_ID,
                    dev,
                );
                libcuda.cuDeviceGetAttribute(
                    &mut device_attributes.pci_domain as *mut u32 as _,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DOMAIN_ID,
                    dev,
                );
                libcuda.cuDeviceGetAttribute(
                    &mut device_attributes.pci_device as *mut u32 as _,
                    CUdevice_attribute::CU_DEVICE_ATTRIBUTE_PCI_DEVICE_ID,
                    dev,
                );
                device_attributes
            })
            .collect::<Vec<_>>();
        let mut driver_version = 0u32;
        libcuda.cuDriverGetVersion(&mut driver_version as *mut _ as _);
        let zludart_guid = [
            0x6bu8, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd,
            0x9d, 0xf9,
        ];
        let cudart_export_table = &cuda_state.dark_api.overrides[&zludart_guid].0;
        let hash_input = AntiZludaHashInput {
            cudart_export_table: cudart_export_table.as_ptr() as _,
            anti_zluda_export_table: overriding_table.as_ptr() as _,
            fn_ptr: overriding_table[idx] as _,
            device_count: device_count as u32,
            driver_version,
            rt_version,
            timestamp,
        };
        let dev_getter = |dev| device_attributes[dev as usize].clone();
        let hash = zluda_dark_api::anti_zluda_hash(false, hash_input, dev_getter);
        if hash != *result {
            fn_logger.log(LogEntry::IntegrityHashOverride {
                before: *result,
                after: hash,
            });
            *result = hash;
        }
        fn_logger.result = Some(original_result);
        original_result
    }

    unsafe fn unwrap_context_impl(
        guid: &[u8; 16],
        idx: usize,
        ctx: CUcontext,
        is_wrapped: *mut u32,
        unwrapped_ctx: *mut CUcontext,
    ) -> CUresult {
        let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
            writer.write_all(b"(ctx: ")?;
            format::CudaDisplay::write(&ctx, "", 0, writer)?;
            writer.write_all(b", is_wrapped: ")?;
            format::CudaDisplay::write(&is_wrapped, "", 0, writer)?;
            writer.write_all(b", unwrapped_ctx: ")?;
            format::CudaDisplay::write(&unwrapped_ctx, "", 0, writer)?;
            write!(writer, ")")
        });
        let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
        let mut fn_logger = global_state.log_factory.get_logger_dark_api(
            CUuuid {
                bytes: guid.clone(),
            },
            idx,
            Some(arguments_writer),
        );
        let cuda_state = &mut global_state.delayed_state.unwrap_mut().cuda_state;
        let original_ptr = cuda_state.dark_api.overrides[guid].1.add(idx);
        let original_fn = mem::transmute::<
            _,
            unsafe extern "system" fn(CUcontext, *mut u32, *mut CUcontext) -> CUresult,
        >(*original_ptr);
        let original_result = original_fn(ctx, is_wrapped, unwrapped_ctx);
        if *is_wrapped != 0 {
            fn_logger.log(LogEntry::WrappedContext);
        }
        fn_logger.result = Some(original_result);
        original_result
    }
}

unsafe fn deref_not_null<T>(ptr: *mut *mut T) -> *mut T {
    if ptr == ptr::null_mut() {
        ptr::null_mut()
    } else {
        *ptr
    }
}

unsafe fn write_deref<T: std::fmt::Display>(
    writer: &mut dyn std::io::Write,
    t: *mut T,
) -> Result<(), std::io::Error> {
    if t == ptr::null_mut() {
        write!(writer, "{:p}", t)
    } else {
        write!(writer, "{}", *t)
    }
}
