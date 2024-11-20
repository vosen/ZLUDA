use cuda_types::*;
use hip_runtime_sys::*;

pub(super) mod context;
pub(super) mod device;

#[cfg(debug_assertions)]
pub(crate) fn unimplemented() -> CUresult {
    unimplemented!()
}

#[cfg(not(debug_assertions))]
pub(crate) fn unimplemented() -> CUresult {
    CUresult::ERROR_NOT_SUPPORTED
}

pub(crate) trait FromCuda<'a, T>: Sized {
    fn from_cuda(t: &'a T) -> Result<Self, CUerror>;
}

macro_rules! from_cuda_nop {
    ($($type_:ty),*) => {
        $(
            impl<'a> FromCuda<'a, $type_> for $type_ {
                fn from_cuda(x: &'a $type_) -> Result<Self, CUerror> {
                    Ok(*x)
                }
            }

            impl<'a> FromCuda<'a, *mut $type_> for &'a mut $type_ {
                fn from_cuda(x: &'a *mut $type_) -> Result<Self, CUerror> {
                    match unsafe { x.as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(CUerror::INVALID_VALUE),
                    }
                }
            }
        )*
    };
}

macro_rules! from_cuda_transmute {
    ($($from:ty => $to:ty),*) => {
        $(
            impl<'a> FromCuda<'a, $from> for $to {
                fn from_cuda(x: &'a $from) -> Result<Self, CUerror> {
                    Ok(unsafe { std::mem::transmute(*x) })
                }
            }

            impl<'a> FromCuda<'a, *mut $from> for &'a mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, CUerror> {
                    match unsafe { x.cast::<$to>().as_mut() } {
                        Some(x) => Ok(x),
                        None => Err(CUerror::INVALID_VALUE),
                    }
                }
            }

            impl<'a> FromCuda<'a, *mut $from> for * mut $to {
                fn from_cuda(x: &'a *mut $from) -> Result<Self, CUerror> {
                    Ok(x.cast::<$to>())
                }
            }
        )*
    };
}

from_cuda_nop!(
    *mut i8,
    *mut usize,
    i32,
    u32,
    usize,
    cuda_types::CUdevprop,
    CUdevice_attribute
);
from_cuda_transmute!(
    CUdevice => hipDevice_t,
    CUuuid => hipUUID
);

impl<'a> FromCuda<'a, CUlimit> for hipLimit_t {
    fn from_cuda(limit: &'a CUlimit) -> Result<Self, CUerror> {
        Ok(match *limit {
            CUlimit::CU_LIMIT_STACK_SIZE => hipLimit_t::hipLimitStackSize,
            CUlimit::CU_LIMIT_PRINTF_FIFO_SIZE => hipLimit_t::hipLimitPrintfFifoSize,
            CUlimit::CU_LIMIT_MALLOC_HEAP_SIZE => hipLimit_t::hipLimitMallocHeapSize,
            _ => return Err(CUerror::NOT_SUPPORTED),
        })
    }
}

pub(crate) fn init(flags: ::core::ffi::c_uint) -> hipError_t {
    unsafe { hipInit(flags) }
}
