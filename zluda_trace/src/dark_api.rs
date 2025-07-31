use crate::os;
use crate::{CudaFunctionName, ErrorEntry};
use cuda_types::cuda::*;
use rustc_hash::FxHashMap;
use std::cell::RefMut;
use std::hash::Hash;
use std::{collections::hash_map, ffi::c_void, mem};

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
