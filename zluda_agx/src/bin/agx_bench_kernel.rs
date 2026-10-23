//! `agx_bench_kernel MODULE KERNEL --grid X[,Y,Z] --block x[,y,z] [--shared BYTES] [--iters N] [--warmup N]
//!                   [--graph-len N] ARG...`: times one kernel through the CUDA driver API of zluda_agx the way an application
//! keeps the GPU busy: `--warmup` launches, then `--iters` launches queued back to back on one stream between two
//! events. `--graph-len` captures that many launches and times repeated graph replays. MODULE is PTX (compiled
//! by zoc, `ZLUDA_AGX_ZOC`) or zoc's AGX object for it. Each ARG is a kernel
//! parameter: `u32:V`, `s32:V`, `f32:V`, `u64:V`, `buf:BYTES` (a zero-filled allocation of that size, passed as
//! its address) or `rnd:BYTES` (one filled with floats in [-1, 1) from a fixed seed). Prints one JSON line: the mean
//! milliseconds per launch over the timed launches, and a hash of each allocation's final contents (to check that
//! two builds of a kernel compute the same).

use cuda_types::cuda::*;
use nvcuda_agx::*;
use std::ffi::{c_void, CString};
use std::ptr::null_mut;

fn check(what: &str, r: CUresult) {
    if let Err(e) = r {
        println!("{{\"error\":\"{what} failed: {e:?}\"}}");
        std::process::exit(1);
    }
}

fn dims(v: &str) -> [u32; 3] {
    let v: Vec<u32> = v.split(',').map(|x| x.parse().expect("dimension")).collect();
    [v[0], *v.get(1).unwrap_or(&1), *v.get(2).unwrap_or(&1)]
}

fn num(v: &str) -> u64 {
    match v.strip_prefix("0x") {
        Some(h) => u64::from_str_radix(h, 16).expect("number"),
        None => v.parse().expect("number"),
    }
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() < 2 {
        println!("{{\"error\":\"usage: agx_bench_kernel MODULE KERNEL --grid X,Y,Z --block x,y,z [--shared B] [--iters N] [--warmup N] [--graph-len N] ARG...\"}}");
        std::process::exit(1);
    }
    let (mut grid, mut block, mut shared, mut iters, mut warmup, mut graph_len) = ([1u32; 3], [1u32; 3], 0u32, 100u32, 20u32, 0u32);
    let mut args: Vec<String> = Vec::new();
    let mut i = 2;
    while i < a.len() {
        match a[i].as_str() {
            "--grid" => grid = dims(&a[i + 1]),
            "--block" => block = dims(&a[i + 1]),
            "--shared" => shared = num(&a[i + 1]) as u32,
            "--iters" => iters = num(&a[i + 1]) as u32,
            "--warmup" => warmup = num(&a[i + 1]) as u32,
            "--graph-len" => graph_len = num(&a[i + 1]) as u32,
            _ => {
                args.push(a[i].clone());
                i += 1;
                continue;
            }
        }
        i += 2;
    }
    unsafe {
        check("cuInit", cuInit(0));
        let mut dev = 0;
        check("cuDeviceGet", cuDeviceGet(&mut dev, 0));
        let mut ctx = CUcontext(null_mut());
        check("cuCtxCreate", cuCtxCreate_v2(&mut ctx, 0, dev));
        let mut image = std::fs::read(&a[0]).unwrap_or_else(|e| panic!("{}: {e}", a[0]));
        image.push(0);
        let mut module = CUmodule(null_mut());
        check("cuModuleLoadData", cuModuleLoadData(&mut module, image.as_ptr() as *const c_void));
        let mut f = CUfunction(null_mut());
        let name = CString::new(a[1].clone()).unwrap();
        check("cuModuleGetFunction", cuModuleGetFunction(&mut f, module, name.as_ptr()));

        // every parameter as its bytes, kept alive for the launches
        let mut values: Vec<Vec<u8>> = Vec::new();
        let mut bufs: Vec<(CUdeviceptr, usize)> = Vec::new();
        for arg in &args {
            let (kind, v) = arg.split_once(':').expect("ARG is KIND:VALUE");
            values.push(match kind {
                "u32" => (num(v) as u32).to_le_bytes().to_vec(),
                "s32" => v.parse::<i32>().expect("s32").to_le_bytes().to_vec(),
                "f32" => v.parse::<f32>().expect("f32").to_le_bytes().to_vec(),
                "u64" => num(v).to_le_bytes().to_vec(),
                "buf" | "rnd" => {
                    let bytes = num(v) as usize;
                    let mut p = CUdeviceptr_v2(null_mut());
                    check("cuMemAlloc", cuMemAlloc_v2(&mut p, bytes));
                    check("cuMemsetD8", cuMemsetD8_v2(p, 0, bytes));
                    if kind == "rnd" {
                        let mut x = 0x9e3779b97f4a7c15u64 ^ bufs.len() as u64;
                        let data: Vec<f32> = (0..bytes / 4)
                            .map(|_| {
                                x ^= x << 13;
                                x ^= x >> 7;
                                x ^= x << 17;
                                (x >> 40) as f32 / (1u64 << 23) as f32 - 1.0
                            })
                            .collect();
                        check("cuMemcpyHtoD", cuMemcpyHtoD_v2(p, data.as_ptr() as *const c_void, data.len() * 4));
                    }
                    bufs.push((p, bytes));
                    (p.0 as u64).to_le_bytes().to_vec()
                }
                _ => panic!("unknown argument kind {kind}"),
            });
        }
        let mut params: Vec<*mut c_void> = values.iter_mut().map(|v| v.as_mut_ptr() as *mut c_void).collect();
        let stream = CUstream(null_mut());
        let launch = |params: &mut Vec<*mut c_void>| {
            check(
                "cuLaunchKernel",
                cuLaunchKernel(f, grid[0], grid[1], grid[2], block[0], block[1], block[2], shared, stream, params.as_mut_ptr(), null_mut()),
            )
        };
        for _ in 0..warmup {
            launch(&mut params);
        }
        let mut graph = null_mut();
        let mut exec = null_mut();
        if graph_len > 0 {
            check("cuStreamBeginCapture", cuStreamBeginCapture_v2(stream, CUstreamCaptureMode_enum::CU_STREAM_CAPTURE_MODE_GLOBAL));
            for _ in 0..graph_len {
                launch(&mut params);
            }
            check("cuStreamEndCapture", cuStreamEndCapture(stream, &mut graph));
            check("cuGraphInstantiate", cuGraphInstantiateWithFlags(&mut exec, graph, 0));
        }
        let (mut start, mut stop) = (null_mut(), null_mut());
        check("cuEventCreate", cuEventCreate(&mut start, 0));
        check("cuEventCreate", cuEventCreate(&mut stop, 0));
        check("cuEventRecord", cuEventRecord(start, stream));
        let host_start = std::time::Instant::now();
        for _ in 0..iters {
            if graph_len > 0 {
                check("cuGraphLaunch", cuGraphLaunch(exec, stream));
            } else {
                launch(&mut params);
            }
        }
        let launches = iters as u64 * graph_len.max(1) as u64;
        let host_ms_per_launch = host_start.elapsed().as_secs_f64() * 1000.0 / launches as f64;
        check("cuEventRecord", cuEventRecord(stop, stream));
        check("cuEventSynchronize", cuEventSynchronize(stop));
        let mut ms = 0f32;
        check("cuEventElapsedTime", cuEventElapsedTime(&mut ms, start, stop));
        let hashes: Vec<String> = bufs
            .iter()
            .map(|&(p, n)| {
                let mut b = vec![0u8; n];
                check("cuMemcpyDtoH", cuMemcpyDtoH_v2(b.as_mut_ptr() as *mut c_void, p, n));
                format!("\"{:016x}\"", b.iter().fold(0xcbf29ce484222325u64, |h, &c| (h ^ c as u64).wrapping_mul(0x100000001b3)))
            })
            .collect();
        // The queue has 1024 timestamp slots. When the start fence's slot has wrapped,
        // cuEventElapsedTime falls back to the host clock; expose that in the result.
        // A graph may need anywhere from one command per 64 launches to one per launch (for
        // kernels that cannot be batched). Below this bound, the start slot cannot have wrapped.
        let timing_source = if launches < agx_runtime::TIMESTAMP_SLOTS { "gpu" } else { "gpu_or_host" };
        println!("{{\"kernel\":{:?},\"iters\":{iters},\"graph_len\":{graph_len},\"ms_per_launch\":{},\"timing_source\":{:?},\"host_ms_per_launch\":{},\"hashes\":[{}]}}", a[1], ms / launches as f32, timing_source, host_ms_per_launch, hashes.join(","));
    }
}
