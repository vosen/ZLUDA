use cuda_types::cufft::{cufftError_t, cufftHandle};
use rocfft_sys::rocfft_plan;
use rustc_hash::FxHashMap;

pub(crate) struct Plan {
    pub(crate) nx: i32,
    pub(crate) type_: i32,
    pub(crate) batch: i32,
    pub(crate) work_size: usize,
    pub(crate) plans: [rocfft_plan; 4],
}

pub struct Registry {
    next_handle: u32,
    plans: FxHashMap<cuda_types::cufft::cufftHandle, Option<Plan>>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            next_handle: 1,
            plans: FxHashMap::default(),
        }
    }

    pub(crate) fn get(
        &mut self,
        cu_handle: cufftHandle,
    ) -> Result<&mut Option<Plan>, cufftError_t> {
        self.plans
            .get_mut(&cu_handle)
            .ok_or(cufftError_t::INVALID_PLAN)
    }

    pub(crate) fn insert(&mut self, plan: Option<Plan>) -> cufftHandle {
        let raw_handle = self.next_handle as i32;
        self.next_handle += 1;
        let cu_handle = cufftHandle(raw_handle);
        self.plans.insert(cu_handle, plan);
        cu_handle
    }

    /*
    pub(crate) fn remove(&mut self, cu_handle: cufftHandle) -> Result<rocfft_plan, cufftError_t> {
        self.plans
            .remove(&cu_handle)
            .ok_or(cufftError_t::INVALID_PLAN)
    }
    */
}
