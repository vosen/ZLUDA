use cuda_types::cufft::cufftHandle;
use rocfft_sys::rocfft_plan;
use rustc_hash::FxHashMap;
use std::ptr;

pub(crate) struct Plan {
    pub(crate) plan: rocfft_plan,
}

pub struct Registry {
    next_handle: u32,
    plans: FxHashMap<cuda_types::cufft::cufftHandle, Plan>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            next_handle: 1,
            plans: FxHashMap::default(),
        }
    }

    pub(crate) fn new_empty(&mut self) -> cufftHandle {
        let raw_handle = self.next_handle as i32;
        self.next_handle += 1;
        self.plans.insert(
            cufftHandle(raw_handle),
            Plan {
                plan: rocfft_plan(ptr::null_mut()),
            },
        );
        cufftHandle(raw_handle)
    }

    /*
    pub(crate) fn get(&self, cu_handle: cufftHandle) -> Result<rocfft_plan, cufftError_t> {
        self.plans
            .get(&cu_handle)
            .cloned()
            .ok_or(cufftError_t::INVALID_PLAN)
    }

    pub(crate) fn remove(&mut self, cu_handle: cufftHandle) -> Result<rocfft_plan, cufftError_t> {
        self.plans
            .remove(&cu_handle)
            .ok_or(cufftError_t::INVALID_PLAN)
    }
    */
}
