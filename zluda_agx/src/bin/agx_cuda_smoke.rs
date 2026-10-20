//! `agx_cuda_smoke [kernel.ptx]`: runs vectorAdd through the CUDA driver API of zluda_agx on the GPU, the way a
//! CUDA application would (cuInit ... cuModuleLoadData, cuLaunchKernel, cuMemcpyDtoH, events), and checks the
//! result. Needs zoc (`ZLUDA_AGX_ZOC`, next to this binary, or on PATH). Prints one JSON line.

use cuda_types::cuda::*;
use nvcuda_agx::*;
use std::ffi::{c_void, CStr};
use std::ptr::null_mut;

const PTX: &str = concat!(include_str!("../../../ptx/src/test/_Z9vectorAddPKfS0_Pfi.ptx"), "\0");

fn check(what: &str, r: CUresult) {
    if let Err(e) = r {
        let mut s = std::ptr::null();
        unsafe { cuGetErrorName(Err(e), &mut s) }.ok();
        let name = if s.is_null() { "?".into() } else { unsafe { CStr::from_ptr(s) }.to_string_lossy() };
        println!("{{\"error\":\"{what}: {name} ({})\"}}", e.0);
        std::process::exit(1);
    }
}

fn main() {
    unsafe {
        check("cuInit", cuInit(0));
        let mut dev = 0;
        check("cuDeviceGet", cuDeviceGet(&mut dev, 0));
        let mut name = [0i8; 128];
        check("cuDeviceGetName", cuDeviceGetName(name.as_mut_ptr(), 128, dev));
        let mut ctx = CUcontext(null_mut());
        check("cuCtxCreate", cuCtxCreate_v2(&mut ctx, 0, dev));
        let mut module = CUmodule(null_mut());
        check("cuModuleLoadData", cuModuleLoadData(&mut module, PTX.as_ptr() as *const c_void));
        let mut f = CUfunction(null_mut());
        check("cuModuleGetFunction", cuModuleGetFunction(&mut f, module, c"_Z9vectorAddPKfS0_Pfi".as_ptr()));

        let n: i32 = 50_000;
        let bytes = n as usize * 4;
        let a: Vec<f32> = (0..n).map(|i| i as f32).collect();
        let b: Vec<f32> = (0..n).map(|i| 0.5 * i as f32 + 1.0).collect();
        let (mut da, mut db, mut dc) = (CUdeviceptr_v2(null_mut()), CUdeviceptr_v2(null_mut()), CUdeviceptr_v2(null_mut()));
        check("cuMemAlloc", cuMemAlloc_v2(&mut da, bytes));
        check("cuMemAlloc", cuMemAlloc_v2(&mut db, bytes));
        check("cuMemAlloc", cuMemAlloc_v2(&mut dc, bytes));
        check("cuMemcpyHtoD", cuMemcpyHtoD_v2(da, a.as_ptr() as *const c_void, bytes));
        check("cuMemcpyHtoD", cuMemcpyHtoD_v2(db, b.as_ptr() as *const c_void, bytes));

        let (mut start, mut stop) = (null_mut(), null_mut());
        check("cuEventCreate", cuEventCreate(&mut start, 0));
        check("cuEventCreate", cuEventCreate(&mut stop, 0));
        let stream = CUstream(null_mut());
        check("cuEventRecord", cuEventRecord(start, stream));
        let mut params: [*mut c_void; 4] =
            [&mut da as *mut _ as *mut c_void, &mut db as *mut _ as *mut c_void, &mut dc as *mut _ as *mut c_void, &n as *const i32 as *mut c_void];
        let block = 256u32;
        let grid = (n as u32).div_ceil(block);
        check("cuLaunchKernel", cuLaunchKernel(f, grid, 1, 1, block, 1, 1, 0, stream, params.as_mut_ptr(), null_mut()));
        check("cuEventRecord", cuEventRecord(stop, stream));
        check("cuEventSynchronize", cuEventSynchronize(stop));
        let mut ms = 0f32;
        check("cuEventElapsedTime", cuEventElapsedTime(&mut ms, start, stop));

        let mut c = vec![0f32; n as usize];
        check("cuMemcpyDtoH", cuMemcpyDtoH_v2(c.as_mut_ptr() as *mut c_void, dc, bytes));
        let wrong = (0..n as usize).filter(|&i| c[i] != a[i] + b[i]).count();
        check("cuCtxSynchronize", cuCtxSynchronize());
        let name = CStr::from_ptr(name.as_ptr()).to_string_lossy();
        println!("{{\"device\":{name:?},\"n\":{n},\"wrong\":{wrong},\"kernel_ms\":{ms}}}");
        std::process::exit(if wrong == 0 { 0 } else { 2 });
    }
}
