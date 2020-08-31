#![allow(non_snake_case)]

use crate::r#impl as notcuda;
use crate::r#impl::CUresult;
use crate::{cuda::CUuuid, r#impl::Encuda};
use ::std::{
    ffi::c_void,
    os::raw::{c_int, c_uint},
};
use cuda_driver_sys as cuda;

#[macro_export]
macro_rules! cuda_driver_test {
    ($func:ident) => {
        paste! {
            #[test]
            fn [<$func _notcuda>]() {
                $func::<crate::r#impl::test::NotCuda>()
            }

            #[test]
            fn [<$func _cuda>]() {
                $func::<crate::r#impl::test::Cuda>()
            }
        }
    };
}

pub trait CudaDriverFns {
    fn cuInit(flags: c_uint) -> CUresult;
    fn cuCtxCreate_v2(pctx: *mut *mut c_void, flags: c_uint, dev: c_int) -> CUresult;
    fn cuCtxDestroy_v2(ctx: *mut c_void) -> CUresult;
    fn cuCtxPopCurrent_v2(pctx: *mut *mut c_void) -> CUresult;
    fn cuCtxGetApiVersion(ctx: *mut c_void, version: *mut c_uint) -> CUresult;
    fn cuCtxGetCurrent(pctx: *mut *mut c_void) -> CUresult;
    fn cuMemAlloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> CUresult;
    fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: c_int) -> CUresult;
    fn cuDevicePrimaryCtxGetState(dev: c_int, flags: *mut c_uint, active: *mut c_int) -> CUresult;
}

pub struct NotCuda();

impl CudaDriverFns for NotCuda {
    fn cuInit(_flags: c_uint) -> CUresult {
        assert!(notcuda::context::is_context_stack_empty());
        notcuda::init().encuda()
    }

    fn cuCtxCreate_v2(pctx: *mut *mut c_void, flags: c_uint, dev: c_int) -> CUresult {
        notcuda::context::create_v2(pctx as *mut _, flags, notcuda::device::Index(dev)).encuda()
    }

    fn cuCtxDestroy_v2(ctx: *mut c_void) -> CUresult {
        notcuda::context::destroy_v2(ctx as *mut _)
    }

    fn cuCtxPopCurrent_v2(pctx: *mut *mut c_void) -> CUresult {
        notcuda::context::pop_current_v2(pctx as *mut _)
    }

    fn cuCtxGetApiVersion(ctx: *mut c_void, version: *mut c_uint) -> CUresult {
        notcuda::context::get_api_version(ctx as *mut _, version)
    }

    fn cuCtxGetCurrent(pctx: *mut *mut c_void) -> CUresult {
        notcuda::context::get_current(pctx as *mut _).encuda()
    }
    fn cuMemAlloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> CUresult {
        notcuda::memory::alloc_v2(dptr as *mut _, bytesize)
    }

    fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: c_int) -> CUresult {
        notcuda::device::get_uuid(uuid, notcuda::device::Index(dev)).encuda()
    }

    fn cuDevicePrimaryCtxGetState(dev: c_int, flags: *mut c_uint, active: *mut c_int) -> CUresult {
        notcuda::device::primary_ctx_get_state(notcuda::device::Index(dev), flags, active).encuda()
    }
}

pub struct Cuda();

impl CudaDriverFns for Cuda {
    fn cuInit(flags: c_uint) -> CUresult {
        unsafe { CUresult(cuda::cuInit(flags) as c_uint) }
    }

    fn cuCtxCreate_v2(pctx: *mut *mut c_void, flags: c_uint, dev: c_int) -> CUresult {
        unsafe { CUresult(cuda::cuCtxCreate_v2(pctx as *mut _, flags, dev) as c_uint) }
    }

    fn cuCtxDestroy_v2(ctx: *mut c_void) -> CUresult {
        unsafe { CUresult(cuda::cuCtxDestroy_v2(ctx as *mut _) as c_uint) }
    }

    fn cuCtxPopCurrent_v2(pctx: *mut *mut c_void) -> CUresult {
        unsafe { CUresult(cuda::cuCtxPopCurrent_v2(pctx as *mut _) as c_uint) }
    }

    fn cuCtxGetApiVersion(ctx: *mut c_void, version: *mut c_uint) -> CUresult {
        unsafe { CUresult(cuda::cuCtxGetApiVersion(ctx as *mut _, version) as c_uint) }
    }

    fn cuCtxGetCurrent(pctx: *mut *mut c_void) -> CUresult {
        unsafe { CUresult(cuda::cuCtxGetCurrent(pctx as *mut _) as c_uint) }
    }
    fn cuMemAlloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> CUresult {
        unsafe { CUresult(cuda::cuMemAlloc_v2(dptr as *mut _, bytesize) as c_uint) }
    }

    fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: c_int) -> CUresult {
        unsafe { CUresult(cuda::cuDeviceGetUuid(uuid as *mut _, dev) as c_uint) }
    }

    fn cuDevicePrimaryCtxGetState(dev: c_int, flags: *mut c_uint, active: *mut c_int) -> CUresult {
        unsafe { CUresult(cuda::cuDevicePrimaryCtxGetState(dev, flags, active) as c_uint) }
    }
}
