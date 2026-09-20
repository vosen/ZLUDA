use cuda_types::cufft::{cufftError_t, cufftHandle};
use hipfft_sys::hipfftHandle;
use rustc_hash::FxHashMap;

pub struct Registry {
    next_handle: u32,
    handles: FxHashMap<cuda_types::cufft::cufftHandle, hipfftHandle>,
}

impl Registry {
    pub(crate) fn new() -> Self {
        Self {
            next_handle: 1,
            handles: FxHashMap::default(),
        }
    }

    pub(crate) fn get(
        &mut self,
        cu_handle: cufftHandle,
    ) -> Result<hipfftHandle, cufftError_t> {
        self.handles
            .get(&cu_handle)
            .copied()
            .ok_or(cufftError_t::INVALID_VALUE)
    }

    pub(crate) fn insert(&mut self, handle: hipfftHandle) -> cufftHandle {
        let raw_handle = self.next_handle as i32;
        self.next_handle += 1;
        let cu_handle = cufftHandle(raw_handle);
        self.handles.insert(cu_handle, handle);
        cu_handle
    }

    pub(crate) fn remove(&mut self, cu_handle: cufftHandle) -> Result<hipfftHandle, cufftError_t> {
        self.handles
            .remove(&cu_handle)
            .ok_or(cufftError_t::INVALID_VALUE)
    }
}
