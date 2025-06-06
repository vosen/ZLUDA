use crate::{log, os, trace::StateTracker};
use crate::{log::UInt, GlobalDelayedState};
use crate::{CudaFunctionName, ErrorEntry, FnCallLog, GlobalState2};
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
    pub(crate) overrides: FxHashMap<Box<CUuuidWrapper>, (*const *const c_void, Vec<*const c_void>)>,
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
        original_export_table: *const *const c_void,
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
        let byte_size: usize = unsafe { *(original_export_table.cast::<usize>()) };
        // Some export tables don't start with a byte count, but directly with a
        // pointer, and are instead terminated by 0 or MAX
        let export_functions_start_idx;
        let export_functions_size;
        if byte_size > 0x10000 {
            export_functions_start_idx = 0;
            let mut i = 0;
            loop {
                let current_ptr = unsafe { original_export_table.add(i) };
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
        if let Some(ref our_functions) = our_functions {
            if our_functions.len() != export_functions_size {
                error = Some(ErrorEntry::UnexpectedExportTableSize {
                    expected: our_functions.len(),
                    computed: export_functions_size,
                });
            }
        }
        let mut override_table =
            unsafe { std::slice::from_raw_parts(original_export_table, export_functions_size) }
                .to_vec();
        for i in export_functions_start_idx..export_functions_size {
            let current_fn = (|| {
                if let Some(ref our_functions) = our_functions {
                    if let Some(fn_) = our_functions.get_fn(i) {
                        return fn_;
                    }
                }
                os::get_thunk(
                    override_table[i],
                    Self::report_unknown_export_table_call,
                    std::ptr::from_ref(entry.key().as_ref()).cast(),
                    i,
                )
            })();
            override_table[i] = current_fn;
        }
        (
            entry
                .insert((original_export_table, override_table))
                .1
                .as_ptr(),
            error,
        )
    }

    unsafe extern "system" fn report_unknown_export_table_call(guid: &CUuuid, index: usize) {
        let global_state = crate::GLOBAL_STATE2.lock();
        let global_state_ref_cell = &*global_state;
        let mut global_state_ref_mut = global_state_ref_cell.borrow_mut();
        let global_state = &mut *global_state_ref_mut;
        let log_guard = crate::OuterCallGuard {
            writer: &mut global_state.log_writer,
            log_root: &global_state.log_stack,
        };
        {
            let mut logger = RefMut::map(global_state.log_stack.borrow_mut(), |log_stack| {
                log_stack.enter()
            });
            logger.name = CudaFunctionName::Dark { guid: *guid, index };
        };
        drop(log_guard);
    }
}

#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub(crate) struct CUuuidWrapper(pub CUuuid);

impl Hash for CUuuidWrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.bytes.hash(state);
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
