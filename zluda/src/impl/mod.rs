use cuda_types::*;
use hip_runtime_sys::*;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> CUresult {
    CUresult::ERROR_NOT_SUPPORTED
}

pub(crate) trait FromCuda<T>: Sized {
    fn from_cuda(t: T) -> Result<Self, CUerror>;
}

impl FromCuda<u32> for u32 {
    fn from_cuda(x: u32) -> Result<Self, CUerror> {
        Ok(x)
    }
}

pub(crate) fn init(flags: ::core::ffi::c_uint) -> hipError_t {
    unsafe { hipInit(flags) }
}
