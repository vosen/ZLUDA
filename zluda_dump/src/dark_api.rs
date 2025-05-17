use crate::{log, os, trace::StateTracker};
use crate::{log::UInt, GlobalDelayedState};
use crate::{CudaFunctionName, ErrorEntry, FnCallLog, GlobalState2, GLOBAL_STATE};
use cuda_types::cuda::*;
use rustc_hash::FxHashMap;
use std::borrow::Cow;
use std::cell::RefMut;
use std::hash::Hash;
use std::{
    collections::{hash_map, HashMap},
    ffi::c_void,
    mem,
    os::raw::{c_int, c_uint, c_ulong, c_ushort},
    ptr, slice,
};

pub(crate) struct DarkApiState2 {
    // Key is Box<CUuuid, because thunk reporting unknown export table needs a
    // stable memory location for the guid
    overrides: FxHashMap<Box<CUuuidWrapper>, (*const *const c_void, Vec<*const c_void>)>,
}

unsafe impl Send for DarkApiState2 {}
unsafe impl Sync for DarkApiState2 {}

impl DarkApiState2 {
    pub(crate) fn new() -> Self {
        DarkApiState2 {
            overrides: FxHashMap::default(),
        }
    }

    pub(crate) fn override_export_table(
        &mut self,
        known_exports: &::dark_api::cuda::CudaDarkApiGlobalTable,
        export_table: *const *const c_void,
        guid: &CUuuid_st,
    ) -> (*const *const c_void, Option<ErrorEntry>) {
        let entry = match self.overrides.entry(Box::new(CUuuidWrapper(*guid))) {
            hash_map::Entry::Occupied(entry) => {
                let (_, override_table) = entry.get();
                return (override_table.as_ptr(), None);
            }
            hash_map::Entry::Vacant(entry) => entry,
        };
        let mut error = None;
        let byte_size: usize = unsafe { *(export_table.cast::<usize>()) };
        // Some export tables don't start with a byte count, but directly with a
        // pointer, and are instead terminated by 0 or MAX
        let export_functions_start_idx;
        let export_functions_size;
        if byte_size > 0x10000 {
            export_functions_start_idx = 0;
            let mut i = 0;
            loop {
                let current_ptr = unsafe { export_table.add(i) };
                let current_ptr_numeric = unsafe { *current_ptr } as usize;
                if current_ptr_numeric == 0usize || current_ptr_numeric == usize::MAX {
                    export_functions_size = i;
                    break;
                }
                i += 1;
            }
        } else {
            export_functions_start_idx = 1;
            export_functions_size = byte_size / mem::size_of::<usize>();
        }
        let our_functions = known_exports.get(guid);
        if let Some(our_functions) = our_functions {
            if our_functions.len() != export_functions_size {
                error.insert(ErrorEntry::UnexpectedExportTableSize {
                    guid: *guid,
                    expected: our_functions.len(),
                    computed: export_functions_size,
                });
            }
        }
        let mut override_table =
            unsafe { std::slice::from_raw_parts(export_table, export_functions_size) }.to_vec();
        for i in export_functions_start_idx..export_functions_size {
            override_table[i] = os::get_thunk(
                override_table[i],
                Self::report_unknown_export_table_call,
                std::ptr::from_ref(entry.key().as_ref()).cast(),
                i,
            );
        }
        (
            entry.insert((export_table, override_table)).1.as_ptr(),
            error,
        )
    }

    unsafe extern "system" fn report_unknown_export_table_call(
        guid: &CUuuid,
        index: usize,
    ) {
        let global_state = crate::GLOBAL_STATE2.lock();
        let global_state_ref_cell = &*global_state;
        let mut global_state_ref_mut = global_state_ref_cell.borrow_mut();
        let global_state = &mut *global_state_ref_mut;
        let drop_guard = crate::OuterCallGuard {
            writer: &mut global_state.log_writer,
            log_root: &global_state.log_stack,
        };
        let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
            log_stack.enter()
        });
        logger.name = CudaFunctionName::Dark { guid: *guid, index };
        drop(drop_guard);
    }
}

pub(crate) struct DarkApiState {
    // Key is Box<CUuuid, because thunk reporting unknown export table needs a
    // stable memory location for the guid
    overrides: HashMap<Box<CUuuidWrapper>, Vec<*const c_void>>,
    original: OriginalExports,
}

unsafe impl Send for DarkApiState {}
unsafe impl Sync for DarkApiState {}

#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub(crate) struct CUuuidWrapper(pub CUuuid);

impl Hash for CUuuidWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.bytes.hash(state);
    }
}

#[allow(improper_ctypes_definitions)]
pub(crate) struct OriginalExports {
    original_get_module_from_cubin: Option<
        unsafe extern "system" fn(
            result: *mut CUmodule,
            fatbinc_wrapper: *const FatbincWrapper,
        ) -> CUresult,
    >,
    original_get_module_from_cubin_ext1: Option<
        unsafe extern "system" fn(
            result: *mut CUmodule,
            fatbinc_wrapper: *const FatbincWrapper,
            ptr1: *mut c_void,
            ptr2: *mut c_void,
            _unknown: usize,
        ) -> CUresult,
    >,
    original_get_module_from_cubin_ext2: Option<
        unsafe extern "system" fn(
            fatbinc_wrapper: *const FatbinHeader,
            result: *mut CUmodule,
            ptr1: *mut c_void,
            ptr2: *mut c_void,
            _unknown: usize,
        ) -> CUresult,
    >,
}

impl DarkApiState {
    pub(crate) fn new() -> Self {
        let original = OriginalExports {
            original_get_module_from_cubin: None,
            original_get_module_from_cubin_ext1: None,
            original_get_module_from_cubin_ext2: None,
        };
        DarkApiState {
            overrides: HashMap::new(),
            original,
        }
    }
}

pub(crate) fn override_export_table(
    pp_export_table: *mut *const c_void,
    p_export_table_id: *const CUuuid,
    state: &mut crate::trace::StateTracker,
) {
    let state = &mut state.dark_api;
    let export_table_mut = unsafe { &mut *pp_export_table };
    let export_id = Box::new(CUuuidWrapper(unsafe { *p_export_table_id }));
    *export_table_mut = match state.overrides.entry(export_id) {
        hash_map::Entry::Occupied(entry) => entry.get().as_ptr() as *const _,
        hash_map::Entry::Vacant(entry) => {
            let guid_ptr = unsafe {
                mem::transmute::<*const CUuuidWrapper, *const CUuuid>(&**entry.key() as *const _)
            };
            entry
                .insert(unsafe {
                    create_new_override(*pp_export_table as *const _, guid_ptr, &mut state.original)
                })
                .as_ptr() as *const _
        }
    };
}

unsafe fn create_new_override(
    export_table: *const *const c_void,
    export_id: *const CUuuid,
    state: &mut OriginalExports,
) -> Vec<*const c_void> {
    let byte_size: usize = *(export_table as *const usize);
    // Some export tables don't start with a byte count, but directly with a
    // pointer, and are instead terminated by 0 or MAX
    let export_functions_start_idx;
    let export_functions_size;
    if byte_size > 0x10000 {
        export_functions_start_idx = 0;
        let mut i = 0;
        loop {
            let current_ptr = export_table.add(i);
            let current_ptr_numeric = *current_ptr as usize;
            if current_ptr_numeric == 0usize || current_ptr_numeric == usize::MAX {
                export_functions_size = i;
                break;
            }
            i += 1;
        }
    } else {
        export_functions_start_idx = 1;
        export_functions_size = byte_size / mem::size_of::<usize>();
    }
    let mut override_table =
        std::slice::from_raw_parts(export_table, export_functions_size).to_vec();
    for i in export_functions_start_idx..export_functions_size {
        let current_fn = export_table.add(i);
        override_table.push(get_export_override_fn(state, *current_fn, export_id, i));
    }
    override_table
}

unsafe extern "system" fn report_unknown_export_table_call(
    export_table: *const CUuuid,
    idx: usize,
) {
    if let Ok(mut global_state) = crate::GLOBAL_STATE.lock() {
        let mut logger = global_state
            .log_factory
            .get_logger_dark_api(*export_table, idx, None);
        logger.log(log::ErrorEntry::UnknownExportTableFn)
    }
}

const CUDART_INTERFACE_GUID: CUuuid = CUuuid {
    bytes: [
        0x6b, 0xd5, 0xfb, 0x6c, 0x5b, 0xf4, 0xe7, 0x4a, 0x89, 0x87, 0xd9, 0x39, 0x12, 0xfd, 0x9d,
        0xf9,
    ],
};

const TOOLS_RUNTIME_CALLBACK_HOOKS_GUID: CUuuid = CUuuid {
    bytes: [
        0xa0, 0x94, 0x79, 0x8c, 0x2e, 0x74, 0x2e, 0x74, 0x93, 0xf2, 0x08, 0x00, 0x20, 0x0c, 0x0a,
        0x66,
    ],
};

const CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID: CUuuid = CUuuid {
    bytes: [
        0xc6, 0x93, 0x33, 0x6e, 0x11, 0x21, 0xdf, 0x11, 0xa8, 0xc3, 0x68, 0xf3, 0x55, 0xd8, 0x95,
        0x93,
    ],
};

const CTX_CREATE_BYPASS_GUID: CUuuid = CUuuid {
    bytes: [
        0x0C, 0xA5, 0x0B, 0x8C, 0x10, 0x04, 0x92, 0x9A, 0x89, 0xA7, 0xD0, 0xDF, 0x10, 0xE7, 0x72,
        0x86,
    ],
};

const HEAP_ACCESS_GUID: CUuuid = CUuuid {
    bytes: [
        0x19, 0x5B, 0xCB, 0xF4, 0xD6, 0x7D, 0x02, 0x4A, 0xAC, 0xC5, 0x1D, 0x29, 0xCE, 0xA6, 0x31,
        0xAE,
    ],
};

const DEVICE_EXTENDED_RT_GUID: CUuuid = CUuuid {
    bytes: [
        0xB1u8, 0x05, 0x41, 0xE1, 0xF7, 0xC7, 0xC7, 0x4A, 0x9F, 0x64, 0xF2, 0x23, 0xBE, 0x99, 0xF1,
        0xE2,
    ],
};

unsafe fn get_export_override_fn(
    state: &mut OriginalExports,
    original_fn: *const c_void,
    guid: *const CUuuid,
    idx: usize,
) -> *const c_void {
    match (*guid, idx) {
        (TOOLS_RUNTIME_CALLBACK_HOOKS_GUID, 2)
        | (TOOLS_RUNTIME_CALLBACK_HOOKS_GUID, 6)
        | (CUDART_INTERFACE_GUID, 2)
        | (CUDART_INTERFACE_GUID, 7)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 0)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 1)
        | (CONTEXT_LOCAL_STORAGE_INTERFACE_V0301_GUID, 2)
        | (CTX_CREATE_BYPASS_GUID, 1)
        | (HEAP_ACCESS_GUID, 1)
        | (HEAP_ACCESS_GUID, 2)
        | (DEVICE_EXTENDED_RT_GUID, 5)
        | (DEVICE_EXTENDED_RT_GUID, 13) => original_fn,
        (CUDART_INTERFACE_GUID, 1) => {
            state.original_get_module_from_cubin = mem::transmute(original_fn);
            get_module_from_cubin as *const _
        }
        (CUDART_INTERFACE_GUID, 6) => {
            state.original_get_module_from_cubin_ext1 = mem::transmute(original_fn);
            get_module_from_cubin_ext1 as *const _
        }
        (CUDART_INTERFACE_GUID, 8) => {
            state.original_get_module_from_cubin_ext2 = mem::transmute(original_fn);
            get_module_from_cubin_ext2 as *const _
        }
        _ => {
            // terminator if it's an export table that is not size-prefixed
            if original_fn == ptr::null() || (original_fn as usize) == usize::MAX {
                ptr::null()
            } else {
                os::get_thunk(original_fn, report_unknown_export_table_call, guid, idx)
            }
        }
    }
}

const FATBINC_MAGIC: c_uint = 0x466243B1;
const FATBINC_VERSION_V1: c_uint = 0x1;
const FATBINC_VERSION_V2: c_uint = 0x2;

#[repr(C)]
struct FatbincWrapper {
    magic: c_uint,
    version: c_uint,
    data: *const FatbinHeader,
    filename_or_fatbins: *const c_void,
}

const FATBIN_MAGIC: c_uint = 0xBA55ED50;
const FATBIN_VERSION: c_ushort = 0x01;

#[repr(C, align(8))]
struct FatbinHeader {
    magic: c_uint,
    version: c_ushort,
    header_size: c_ushort,
    files_size: c_ulong, // excluding frame header, size of all blocks framed by this frame
}

const FATBIN_FILE_HEADER_KIND_PTX: c_ushort = 0x01;
const FATBIN_FILE_HEADER_KIND_ELF: c_ushort = 0x02;
const FATBIN_FILE_HEADER_VERSION_CURRENT: c_ushort = 0x101;

// assembly file header is a bit different, but we don't care
#[repr(C)]
#[derive(Debug)]
struct FatbinFileHeader {
    kind: c_ushort,
    version: c_ushort,
    header_size: c_uint,
    padded_payload_size: c_uint,
    unknown0: c_uint, // check if it's written into separately
    payload_size: c_uint,
    unknown1: c_uint,
    unknown2: c_uint,
    sm_version: c_uint,
    bit_width: c_uint,
    unknown3: c_uint,
    unknown4: c_ulong,
    unknown5: c_ulong,
    uncompressed_payload: c_ulong,
}

unsafe fn record_submodules_from_wrapped_fatbin(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    fn_logger: &mut FnCallLog,
    delayed_state: &mut GlobalDelayedState,
    original_fn: impl FnOnce(&OriginalExports) -> CUresult,
) -> CUresult {
    todo!()
}

/*
unsafe fn record_submodules_from_wrapped_fatbin(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    fn_logger: &mut FnCallLog,
    delayed_state: &mut GlobalDelayedState,
    original_fn: impl FnOnce(&OriginalExports) -> CUresult,
) -> CUresult {
    let result = original_fn(&delayed_state.cuda_state.dark_api.original);
    fn_logger.result = Some(result);
    let magic = (*fatbinc_wrapper).magic;
    if magic != FATBINC_MAGIC {
        fn_logger.log(log::ErrorEntry::UnexpectedBinaryField {
            field_name: "FATBINC_MAGIC",
            expected: vec![UInt::U32(FATBINC_MAGIC)],
            observed: UInt::U32(magic),
        });
    }
    if (*fatbinc_wrapper).version != FATBINC_VERSION_V1
        && (*fatbinc_wrapper).version != FATBINC_VERSION_V2
    {
        fn_logger.log(log::ErrorEntry::UnexpectedBinaryField {
            field_name: "FATBINC_VERSION",
            expected: vec![UInt::U32(FATBINC_VERSION_V1), UInt::U32(FATBINC_VERSION_V2)],
            observed: UInt::U32(magic),
        });
    }
    let is_version_2 = (*fatbinc_wrapper).version == FATBINC_VERSION_V2;
    record_submodules_from_fatbin(
        *module,
        (*fatbinc_wrapper).data,
        if is_version_2 { Some(1) } else { None },
        fn_logger,
        &mut delayed_state.cuda_state,
    );
    if is_version_2 {
        let mut current = (*fatbinc_wrapper).filename_or_fatbins as *const *const c_void;
        while *current != ptr::null() {
            record_submodules_from_fatbin(
                *module,
                *current as *const _,
                Some(2),
                fn_logger,
                &mut delayed_state.cuda_state,
            );
            current = current.add(1);
        }
    }
    result
}
     */

unsafe fn record_submodules_from_fatbin(
    module: CUmodule,
    fatbin_header: *const FatbinHeader,
    fatbin_version: Option<usize>,
    logger: &mut FnCallLog,
    state: &mut StateTracker,
) {
    let magic = (*fatbin_header).magic;
    if magic != FATBIN_MAGIC {
        logger.log(log::ErrorEntry::UnexpectedBinaryField {
            field_name: "FATBIN_MAGIC",
            expected: vec![UInt::U32(FATBIN_MAGIC)],
            observed: UInt::U32(magic),
        });
        return;
    }
    let version = (*fatbin_header).version;
    if version != FATBIN_VERSION {
        logger.log(log::ErrorEntry::UnexpectedBinaryField {
            field_name: "FATBIN_VERSION",
            expected: vec![UInt::U16(FATBIN_VERSION)],
            observed: UInt::U16(version),
        });
        return;
    }
    let file = (fatbin_header as *const u8).add((*fatbin_header).header_size as usize);
    let end = file.add((*fatbin_header).files_size as usize);
    record_submodules(
        fatbin_version == Some(2),
        module,
        fatbin_version,
        logger,
        state,
        file,
        end,
    );
}

#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
) -> CUresult {
    todo!()
}

/*
#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
) -> CUresult {
    let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
        writer.write_all(b"(")?;
        writer.write_all(stringify!(module).as_bytes())?;
        writer.write_all(b": ")?;
        format::CudaDisplay::write(&module, "", 0, writer)?;
        writer.write_all(b", ")?;
        writer.write_all(stringify!(fatbinc_wrapper).as_bytes())?;
        write!(writer, ": {:p})", fatbinc_wrapper)
    });
    let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
    let mut fn_logger = global_state.log_factory.get_logger_dark_api(
        CUDART_INTERFACE_GUID,
        1,
        Some(arguments_writer),
    );
    let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
    let delayed_state = global_state.delayed_state.unwrap_mut();
    record_submodules_from_wrapped_fatbin(
        module,
        fatbinc_wrapper,
        &mut fn_logger,
        delayed_state,
        |original_exports| {
            original_exports.original_get_module_from_cubin.unwrap()(module, fatbinc_wrapper)
        },
    )
}
    */

#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin_ext1(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    todo!()
}

/*
#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin_ext1(
    module: *mut CUmodule,
    fatbinc_wrapper: *const FatbincWrapper,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
        writer.write_all(b"(")?;
        writer.write_all(stringify!(module).as_bytes())?;
        writer.write_all(b": ")?;
        format::CudaDisplay::write(&module, "", 0, writer)?;
        writer.write_all(b", ")?;
        writer.write_all(stringify!(fatbinc_wrapper).as_bytes())?;
        write!(writer, ": {:p}, ", fatbinc_wrapper)?;
        writer.write_all(stringify!(ptr1).as_bytes())?;
        write!(writer, ": {:p}, ", ptr1)?;
        writer.write_all(stringify!(ptr2).as_bytes())?;
        write!(writer, ": {:p}, ", ptr2)?;
        writer.write_all(stringify!(_unknown).as_bytes())?;
        write!(writer, ": {})", _unknown)
    });
    let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
    let mut fn_logger = global_state.log_factory.get_logger_dark_api(
        CUDART_INTERFACE_GUID,
        6,
        Some(arguments_writer),
    );
    if ptr1 != ptr::null_mut() {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(ptr1),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(ptr1 as usize),
        });
    }
    if ptr2 != ptr::null_mut() {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(ptr2),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(ptr2 as usize),
        });
    }
    if _unknown != 0 {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(_unknown),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(_unknown),
        });
    }
    let delayed_state = global_state.delayed_state.unwrap_mut();
    record_submodules_from_wrapped_fatbin(
        module,
        fatbinc_wrapper,
        &mut fn_logger,
        delayed_state,
        |original_exports| {
            original_exports
                .original_get_module_from_cubin_ext1
                .unwrap()(module, fatbinc_wrapper, ptr1, ptr2, _unknown)
        },
    )
}
    */

#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin_ext2(
    fatbin_header: *const FatbinHeader,
    module: *mut CUmodule,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    todo!()
}

/*
#[allow(improper_ctypes_definitions)]
unsafe extern "system" fn get_module_from_cubin_ext2(
    fatbin_header: *const FatbinHeader,
    module: *mut CUmodule,
    ptr1: *mut c_void,
    ptr2: *mut c_void,
    _unknown: usize,
) -> CUresult {
    let arguments_writer = Box::new(move |writer: &mut dyn std::io::Write| {
        writer.write_all(b"(")?;
        writer.write_all(stringify!(fatbin_header).as_bytes())?;
        write!(writer, ": {:p}, ", fatbin_header)?;
        writer.write_all(stringify!(module).as_bytes())?;
        writer.write_all(b": ")?;
        format::CudaDisplay::write(&module, "", 0, writer)?;
        writer.write_all(b", ")?;
        writer.write_all(stringify!(ptr1).as_bytes())?;
        write!(writer, ": {:p}, ", ptr1)?;
        writer.write_all(stringify!(ptr2).as_bytes())?;
        write!(writer, ": {:p}, ", ptr2)?;
        writer.write_all(stringify!(_unknown).as_bytes())?;
        write!(writer, ": {})", _unknown)
    });
    let global_state = &mut *super::GLOBAL_STATE.lock().unwrap();
    let mut fn_logger = global_state.log_factory.get_logger_dark_api(
        CUDART_INTERFACE_GUID,
        8,
        Some(arguments_writer),
    );
    if ptr1 != ptr::null_mut() {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(ptr1),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(ptr1 as usize),
        });
    }
    if ptr2 != ptr::null_mut() {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(ptr2),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(ptr2 as usize),
        });
    }
    if _unknown != 0 {
        fn_logger.log(log::ErrorEntry::UnexpectedArgument {
            arg_name: stringify!(_unknown),
            expected: vec![UInt::USize(0)],
            observed: UInt::USize(_unknown),
        });
    }
    let delayed_state = global_state.delayed_state.unwrap_mut();
    let result = delayed_state
        .cuda_state
        .dark_api
        .original
        .original_get_module_from_cubin_ext2
        .unwrap()(fatbin_header, module, ptr1, ptr2, _unknown);
    fn_logger.result = Some(result);
    if result.is_err() {
        return result;
    }
    record_submodules_from_fatbin(
        *module,
        fatbin_header,
        None,
        &mut fn_logger,
        &mut delayed_state.cuda_state,
    );
    result
}
     */

unsafe fn record_submodules(
    should_decompress_elf: bool,
    module: CUmodule,
    version: Option<usize>,
    fn_logger: &mut FnCallLog,
    state: &mut StateTracker,
    start: *const u8,
    end: *const u8,
) {
    let mut index = start;
    while index < end {
        let fatbin_file = index as *const FatbinFileHeader;
        let fatbin_file_version = (*fatbin_file).version;
        if fatbin_file_version != FATBIN_FILE_HEADER_VERSION_CURRENT {
            fn_logger.log(log::ErrorEntry::UnexpectedBinaryField {
                field_name: stringify!(fatbin_file_version),
                expected: vec![UInt::U16(FATBIN_FILE_HEADER_VERSION_CURRENT)],
                observed: UInt::U16(fatbin_file_version),
            });
        }
        let fatbin_file_kind = (*fatbin_file).kind;
        if fatbin_file_kind == FATBIN_FILE_HEADER_KIND_PTX {
            let decompressed = decompress_kernel_module(fatbin_file);
            match decompressed {
                Some(mut decompressed) => {
                    decompressed.pop(); // remove trailing zero
                    state.record_new_submodule(module, version, &*decompressed, fn_logger, "ptx")
                }
                None => fn_logger.log(log::ErrorEntry::Lz4DecompressionFailure),
            }
        } else if fatbin_file_kind == FATBIN_FILE_HEADER_KIND_ELF {
            let source_buffer = if should_decompress_elf {
                let decompressed = decompress_kernel_module(fatbin_file);
                match decompressed {
                    Some(decompressed) => Cow::Owned(decompressed),
                    None => {
                        fn_logger.log(log::ErrorEntry::Lz4DecompressionFailure);
                        continue;
                    }
                }
            } else {
                Cow::Borrowed(slice::from_raw_parts(
                    (fatbin_file as *const u8).add((*fatbin_file).header_size as usize),
                    (*fatbin_file).padded_payload_size as usize,
                ))
            };
            state.record_new_submodule(module, version, &*source_buffer, fn_logger, "elf")
        } else {
            fn_logger.log(log::ErrorEntry::UnexpectedBinaryField {
                field_name: stringify!(fatbin_file_kind),
                expected: vec![
                    UInt::U16(FATBIN_FILE_HEADER_KIND_PTX),
                    UInt::U16(FATBIN_FILE_HEADER_KIND_ELF),
                ],
                observed: UInt::U16(fatbin_file_kind),
            });
        }
        index = index
            .add((*fatbin_file).header_size as usize + (*fatbin_file).padded_payload_size as usize);
    }
}

const MAX_MODULE_DECOMPRESSION_BOUND: usize = 64 * 1024 * 1024;

unsafe fn decompress_kernel_module(file: *const FatbinFileHeader) -> Option<Vec<u8>> {
    let decompressed_size = usize::max(1024, (*file).uncompressed_payload as usize);
    let mut decompressed_vec = vec![0u8; decompressed_size];
    loop {
        match lz4_sys::LZ4_decompress_safe(
            (file as *const u8).add((*file).header_size as usize) as *const _,
            decompressed_vec.as_mut_ptr() as *mut _,
            (*file).payload_size as c_int,
            decompressed_vec.len() as c_int,
        ) {
            error if error < 0 => {
                let new_size = decompressed_vec.len() * 2;
                if new_size > MAX_MODULE_DECOMPRESSION_BOUND {
                    return None;
                }
                decompressed_vec.resize(decompressed_vec.len() * 2, 0);
            }
            real_decompressed_size => {
                decompressed_vec.truncate(real_decompressed_size as usize);
                return Some(decompressed_vec);
            }
        }
    }
}
