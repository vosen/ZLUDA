//! `agx_cuda_smoke [vectoradd.o]`: runs vectorAdd through the CUDA driver API of zluda_agx on the GPU, the way a
//! CUDA application would (cuInit ... cuModuleLoadData, cuLaunchKernel, cuMemcpyDtoH, events), and checks the
//! result; then replays it as a CUDA graph, runs it on managed and mapped host memory, and does a 2D copy. Without
//! an argument the PTX is compiled by zoc (`ZLUDA_AGX_ZOC`, next to this binary, or on PATH); with one, that AGX
//! object (zoc's output for the same PTX) is loaded instead, so no LLVM is needed. Without an argument it also links
//! two PTX modules with cuLink* and runs the result (`link_ok`). Prints one JSON line.

use cuda_types::cuda::*;
use nvcuda_agx::*;
use std::ffi::{c_void, CStr};
use std::ptr::null_mut;

const PTX: &str = concat!(include_str!("../../../ptx/src/test/_Z9vectorAddPKfS0_Pfi.ptx"), "\0");
/// cuLink: a kernel calling an `.extern` function, and a module defining it (x + 1).
const LINK_MAIN: &str = concat!(include_str!("../../../ptx/src/test/spirv_run/extern_func.ptx"), "\0");
const LINK_LIB: &str = ".version 6.5
.target sm_30
.address_size 64
.visible .func (.param .align 16 .b8 r[16]) foobar(.param .b64 x)
{
    .reg .u64 a;
    ld.param.u64 a, [x];
    add.u64 a, a, 1;
    st.param.u64 [r+0], a;
    ret;
}
\0";

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
        let mut name = [0 as std::ffi::c_char; 128];
        check("cuDeviceGetName", cuDeviceGetName(name.as_mut_ptr(), 128, dev));
        let mut ctx = CUcontext(null_mut());
        check("cuCtxCreate", cuCtxCreate_v2(&mut ctx, 0, dev));
        let mut module = CUmodule(null_mut());
        let object = std::env::args().nth(1).map(|p| std::fs::read(&p).unwrap_or_else(|e| panic!("{p}: {e}")));
        let image = object.as_deref().map_or(PTX.as_ptr(), |o| o.as_ptr());
        check("cuModuleLoadData", cuModuleLoadData(&mut module, image as *const c_void));
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

        // The same work as a CUDA graph: capture (clear C, launch), then replay twice.
        let mut s1 = CUstream(null_mut());
        check("cuStreamCreate", cuStreamCreate(&mut s1, 0));
        check("cuStreamBeginCapture", cuStreamBeginCapture_v2(s1, CUstreamCaptureMode_enum::CU_STREAM_CAPTURE_MODE_GLOBAL));
        check("cuMemsetD32Async", cuMemsetD32Async(dc, 0, n as usize, s1));
        check("cuLaunchKernel", cuLaunchKernel(f, grid, 1, 1, block, 1, 1, 0, s1, params.as_mut_ptr(), null_mut()));
        let mut graph = null_mut();
        check("cuStreamEndCapture", cuStreamEndCapture(s1, &mut graph));
        let mut exec = null_mut();
        check("cuGraphInstantiate", cuGraphInstantiateWithFlags(&mut exec, graph, 0));
        let mut graph_wrong = 0;
        for _ in 0..2 {
            check("cuGraphLaunch", cuGraphLaunch(exec, s1));
            check("cuStreamSynchronize", cuStreamSynchronize(s1));
            c.fill(0.0);
            check("cuMemcpyDtoH", cuMemcpyDtoH_v2(c.as_mut_ptr() as *mut c_void, dc, bytes));
            graph_wrong += (0..n as usize).filter(|&i| c[i] != a[i] + b[i]).count();
        }

        // Unified memory: managed A and B written by the CPU, C in mapped host memory read by the CPU directly.
        let (mut ma, mut mb) = (CUdeviceptr_v2(null_mut()), CUdeviceptr_v2(null_mut()));
        let managed = cuMemAllocManaged(&mut ma, bytes, 1).is_ok() && cuMemAllocManaged(&mut mb, bytes, 1).is_ok();
        let mut hc: *mut c_void = null_mut();
        check("cuMemHostAlloc", cuMemHostAlloc(&mut hc, bytes, 2));
        let mut dhc = CUdeviceptr_v2(null_mut());
        let mapped = cuMemHostGetDevicePointer_v2(&mut dhc, hc, 0).is_ok();
        let mut unified_wrong = -1i64;
        if managed && mapped {
            std::ptr::copy_nonoverlapping(a.as_ptr(), ma.0 as *mut f32, n as usize);
            std::ptr::copy_nonoverlapping(b.as_ptr(), mb.0 as *mut f32, n as usize);
            let mut up: [*mut c_void; 4] =
                [&mut ma as *mut _ as *mut c_void, &mut mb as *mut _ as *mut c_void, &mut dhc as *mut _ as *mut c_void, &n as *const i32 as *mut c_void];
            check("cuLaunchKernel", cuLaunchKernel(f, grid, 1, 1, block, 1, 1, 0, stream, up.as_mut_ptr(), null_mut()));
            check("cuCtxSynchronize", cuCtxSynchronize());
            let hcv = std::slice::from_raw_parts(hc as *const f32, n as usize);
            unified_wrong = (0..n as usize).filter(|&i| hcv[i] != a[i] + b[i]).count() as i64;
        }

        // A 2D copy of a 7 x 5 window of C (as u32) into a pitched allocation and back.
        let (w, h) = (7usize, 5usize);
        let (mut pd, mut pitch) = (CUdeviceptr_v2(null_mut()), 0usize);
        check("cuMemAllocPitch", cuMemAllocPitch_v2(&mut pd, &mut pitch, w * 4, h, 4));
        let src: Vec<u32> = (0..64 * 8).collect();
        let mut cp: CUDA_MEMCPY2D = std::mem::zeroed();
        cp.srcMemoryType = CUmemorytype_enum::CU_MEMORYTYPE_HOST;
        cp.srcHost = src.as_ptr() as *const c_void;
        cp.srcPitch = 64 * 4;
        cp.srcXInBytes = 3 * 4;
        cp.srcY = 2;
        cp.dstMemoryType = CUmemorytype_enum::CU_MEMORYTYPE_DEVICE;
        cp.dstDevice = pd;
        cp.dstPitch = pitch;
        cp.WidthInBytes = w * 4;
        cp.Height = h;
        check("cuMemcpy2D", cuMemcpy2D_v2(&cp));
        let mut back = vec![0u32; w * h];
        let mut cb: CUDA_MEMCPY2D = std::mem::zeroed();
        cb.srcMemoryType = CUmemorytype_enum::CU_MEMORYTYPE_DEVICE;
        cb.srcDevice = pd;
        cb.srcPitch = pitch;
        cb.dstMemoryType = CUmemorytype_enum::CU_MEMORYTYPE_HOST;
        cb.dstHost = back.as_mut_ptr() as *mut c_void;
        cb.dstPitch = w * 4;
        cb.WidthInBytes = w * 4;
        cb.Height = h;
        check("cuMemcpy2D", cuMemcpy2D_v2(&cb));
        let copy2d_wrong = (0..w * h).filter(|&i| back[i] != src[(2 + i / w) * 64 + 3 + i % w]).count();

        // cuLink (needs zoc, so only without an object argument): extern_func(in = 7) -> foobar(7) = 8
        let link_ok = if object.is_some() {
            "null".to_string()
        } else {
            let mut state = null_mut();
            check("cuLinkCreate", cuLinkCreate_v2(0, null_mut(), null_mut(), &mut state));
            for (src, name) in [(LINK_MAIN, c"main.ptx"), (LINK_LIB, c"lib.ptx")] {
                check(
                    "cuLinkAddData",
                    cuLinkAddData_v2(state, CUjitInputType::CU_JIT_INPUT_PTX, src.as_ptr() as *mut c_void, src.len(), name.as_ptr(), 0, null_mut(), null_mut()),
                );
            }
            let (mut cubin, mut size) = (null_mut(), 0usize);
            check("cuLinkComplete", cuLinkComplete(state, &mut cubin, &mut size));
            let mut linked = CUmodule(null_mut());
            check("cuModuleLoadData(linked)", cuModuleLoadData(&mut linked, cubin));
            check("cuLinkDestroy", cuLinkDestroy(state));
            let mut lf = CUfunction(null_mut());
            check("cuModuleGetFunction(extern_func)", cuModuleGetFunction(&mut lf, linked, c"extern_func".as_ptr()));
            let (mut din, mut dout) = (CUdeviceptr_v2(null_mut()), CUdeviceptr_v2(null_mut()));
            check("cuMemAlloc", cuMemAlloc_v2(&mut din, 8));
            check("cuMemAlloc", cuMemAlloc_v2(&mut dout, 8));
            let seven = 7u64;
            check("cuMemcpyHtoD", cuMemcpyHtoD_v2(din, &seven as *const u64 as *const c_void, 8));
            let mut lp: [*mut c_void; 2] = [&mut din as *mut _ as *mut c_void, &mut dout as *mut _ as *mut c_void];
            check("cuLaunchKernel(extern_func)", cuLaunchKernel(lf, 1, 1, 1, 1, 1, 1, 0, stream, lp.as_mut_ptr(), null_mut()));
            let mut got = 0u64;
            check("cuMemcpyDtoH", cuMemcpyDtoH_v2(&mut got as *mut u64 as *mut c_void, dout, 8));
            (got == 8).to_string()
        };

        let name = CStr::from_ptr(name.as_ptr()).to_string_lossy();
        println!(
            "{{\"device\":{name:?},\"link_ok\":{link_ok},\"n\":{n},\"wrong\":{wrong},\"graph_wrong\":{graph_wrong},\"unified\":{},\"unified_wrong\":{unified_wrong},\"copy2d_wrong\":{copy2d_wrong},\"kernel_ms\":{ms}}}",
            managed && mapped
        );
        // unified memory needs the CPU range of the GPU addresses to be free; its absence is reported, not a failure
        let ok = wrong == 0 && graph_wrong == 0 && unified_wrong <= 0 && copy2d_wrong == 0 && link_ok != "false";
        std::process::exit(if ok { 0 } else { 2 });
    }
}
