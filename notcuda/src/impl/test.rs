#![allow(non_snake_case)]

use crate::cuda as notcuda;
use crate::cuda::CUstream;
use crate::cuda::CUuuid;
use crate::{
    cuda::{CUdevice, CUdeviceptr},
    r#impl::CUresult,
};
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
    fn cuStreamGetCtx(hStream: CUstream, pctx: *mut *mut c_void) -> CUresult;
    fn cuStreamCreate(stream: *mut CUstream, flags: c_uint) -> CUresult;
    fn cuMemFree_v2(mem: *mut c_void) -> CUresult;
    fn cuStreamDestroy_v2(stream: CUstream) -> CUresult;
}

pub struct NotCuda();

impl CudaDriverFns for NotCuda {
    fn cuInit(_flags: c_uint) -> CUresult {
        notcuda::cuInit(_flags as _)
    }

    fn cuCtxCreate_v2(pctx: *mut *mut c_void, flags: c_uint, dev: c_int) -> CUresult {
        notcuda::cuCtxCreate_v2(pctx as *mut _, flags, CUdevice(dev))
    }

    fn cuCtxDestroy_v2(ctx: *mut c_void) -> CUresult {
        notcuda::cuCtxDestroy_v2(ctx as *mut _)
    }

    fn cuCtxPopCurrent_v2(pctx: *mut *mut c_void) -> CUresult {
        notcuda::cuCtxPopCurrent_v2(pctx as *mut _)
    }

    fn cuCtxGetApiVersion(ctx: *mut c_void, version: *mut c_uint) -> CUresult {
        notcuda::cuCtxGetApiVersion(ctx as *mut _, version)
    }

    fn cuCtxGetCurrent(pctx: *mut *mut c_void) -> CUresult {
        notcuda::cuCtxGetCurrent(pctx as *mut _)
    }
    fn cuMemAlloc_v2(dptr: *mut *mut c_void, bytesize: usize) -> CUresult {
        notcuda::cuMemAlloc_v2(dptr as *mut _, bytesize)
    }

    fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: c_int) -> CUresult {
        notcuda::cuDeviceGetUuid(uuid, CUdevice(dev))
    }

    fn cuDevicePrimaryCtxGetState(dev: c_int, flags: *mut c_uint, active: *mut c_int) -> CUresult {
        notcuda::cuDevicePrimaryCtxGetState(CUdevice(dev), flags, active)
    }

    fn cuStreamGetCtx(hStream: CUstream, pctx: *mut *mut c_void) -> CUresult {
        notcuda::cuStreamGetCtx(hStream, pctx as _)
    }

    fn cuStreamCreate(stream: *mut CUstream, flags: c_uint) -> CUresult {
        notcuda::cuStreamCreate(stream, flags)
    }

    fn cuMemFree_v2(dptr: *mut c_void) -> CUresult {
        notcuda::cuMemFree_v2(CUdeviceptr(dptr as _))
    }

    fn cuStreamDestroy_v2(stream: CUstream) -> CUresult {
        notcuda::cuStreamDestroy_v2(stream)
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

    fn cuStreamGetCtx(hStream: CUstream, pctx: *mut *mut c_void) -> CUresult {
        unsafe { CUresult(cuda::cuStreamGetCtx(hStream as _, pctx as _) as c_uint) }
    }

    fn cuStreamCreate(stream: *mut CUstream, flags: c_uint) -> CUresult {
        unsafe { CUresult(cuda::cuStreamCreate(stream as _, flags as _) as c_uint) }
    }

    fn cuMemFree_v2(mem: *mut c_void) -> CUresult {
        unsafe { CUresult(cuda::cuMemFree_v2(mem as _) as c_uint) }
    }

    fn cuStreamDestroy_v2(stream: CUstream) -> CUresult {
        unsafe { CUresult(cuda::cuStreamDestroy_v2(stream as _) as c_uint) }
    }
}
