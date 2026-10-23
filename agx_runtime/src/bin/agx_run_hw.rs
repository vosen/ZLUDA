//! `agx_run_hw MAIN.bin --grid X[,Y,Z] --wg x[,y,z] [--regs HALVES] [--shared BYTES] [--stack BYTES] [--uniform I=V ...]
//!            [--map VA=FILE ...] [--alloc VA=BYTES ...] [--dump VA=BYTES:FILE ...] [--node PATH] [--timeout-ms N]
//!            [--repeat N] [--warm-ms MS] [--keep-memory]`
//!
//! Runs a whole AGX compute kernel on the GPU with the command line of the AGX Performance Simulator's `agx_run`, so
//! that any simulator test (the LLVM AGX backend's kernels, ZLUDA's PTX tests, the Sail-derived difftests) can also
//! run on hardware. The options have the same meaning as in `agx_run`:
//! - `--grid` is in threads; when it is not a multiple of `--wg`, the last workgroups are partial (the hardware runs
//!   them exactly, ptx-re-blockers B2a);
//! - `--stack` is the stack (scratch) per thread of a kernel that uses `stack_*` instructions, set up through the
//!   helper program as zluda_agx does (the kernel record's stack bytes; 0, the default, gives none);
//! - `--uniform I=V` sets the 32-bit uniform register pair `u(2I)_u(2I+1)`, i.e. 32-bit uniform I;
//! - `--map` / `--alloc` place memory at fixed GPU addresses (rounded out to 16 KiB pages, which must not overlap);
//! - `--dump` writes a region back after the run.
//!
//! `--regs` is the register allocation in 16-bit halves (default 256, the maximum, which is always safe).
//! Run every kernel through `agx_run` first: a kernel that does not terminate there must not reach the GPU.
//! `--repeat N` runs the launch N times from the same initial memory (restored before each run; the dumps are the
//! last run's), and `--warm-ms MS` keeps repeating (up to 5000 runs) until the runs add up to MS of GPU time: the GPU
//! clock ramps up over the first runs, so `min_fw_ns` / `median_fw_ns` are the warm times. `--keep-memory` skips the
//! restore between runs (for kernels whose time does not depend on their earlier results): restoring large regions
//! from the CPU leaves the GPU idle long enough that its clock stays low.
//! Prints one JSON line with the firmware command time.

use agx_runtime::*;
use std::collections::BTreeMap;
use std::time::Duration;

fn die(m: &str) -> ! {
    println!("{{\"error\":{:?}}}", m);
    std::process::exit(1)
}

fn num(s: &str) -> u64 {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x") { u64::from_str_radix(h, 16) } else { s.parse() }
        .unwrap_or_else(|_| die(&format!("bad number {s:?}")))
}

fn dims(s: &str) -> [u32; 3] {
    let v: Vec<u32> = s.split(',').map(|x| num(x) as u32).collect();
    [v[0], *v.get(1).unwrap_or(&1), *v.get(2).unwrap_or(&1)]
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        die("usage: agx_run_hw MAIN.bin --grid X,Y,Z --wg x,y,z [--regs HALVES] [--shared BYTES] [--uniform I=V] [--map VA=FILE] [--alloc VA=BYTES] [--dump VA=BYTES:FILE]");
    }
    let code = std::fs::read(&a[0]).unwrap_or_else(|e| die(&format!("{}: {e}", a[0])));
    let (mut grid, mut wg) = ([1u32; 3], [1u32; 3]);
    let (mut regs, mut shared, mut node, mut timeout) = (256u32, 0u32, "/dev/dri/renderD128".to_string(), 3000u64);
    let mut stack = 0u32;
    let (mut repeat, mut warm_ms, mut keep_memory) = (1u64, 0u64, false);
    let mut uniforms: BTreeMap<usize, u32> = BTreeMap::new();
    let mut regions: Vec<(u64, Vec<u8>)> = Vec::new();
    let mut dumps: Vec<(u64, usize, String)> = Vec::new();
    let mut i = 1;
    while i < a.len() {
        if a[i] == "--keep-memory" {
            keep_memory = true;
            i += 1;
            continue;
        }
        let v = a.get(i + 1).cloned().unwrap_or_else(|| die(&format!("{} needs a value", a[i])));
        let kv = |sep: char| -> (String, String) {
            let (k, x) = v.split_once(sep).unwrap_or_else(|| die(&format!("{} expects K{sep}V", a[i])));
            (k.to_string(), x.to_string())
        };
        match a[i].as_str() {
            "--grid" => grid = dims(&v),
            "--wg" => wg = dims(&v),
            "--regs" => regs = num(&v) as u32,
            "--shared" => shared = num(&v) as u32,
            "--stack" => stack = num(&v) as u32,
            "--node" => node = v.clone(),
            "--timeout-ms" => timeout = num(&v),
            "--repeat" => repeat = num(&v).max(1),
            "--warm-ms" => warm_ms = num(&v),
            "--uniform" => {
                let (k, x) = kv('=');
                uniforms.insert(num(&k) as usize, num(&x) as u32);
            }
            "--map" => {
                let (va, f) = kv('=');
                regions.push((num(&va), std::fs::read(&f).unwrap_or_else(|e| die(&format!("{f}: {e}")))));
            }
            "--alloc" => {
                let (va, n) = kv('=');
                regions.push((num(&va), vec![0u8; num(&n) as usize]));
            }
            "--dump" => {
                let (va, rest) = kv('=');
                let (n, f) = rest.split_once(':').unwrap_or_else(|| die("--dump expects VA=BYTES:FILE"));
                dumps.push((num(&va), num(n) as usize, f.to_string()));
            }
            o => die(&format!("unknown option {o}")),
        }
        i += 2;
    }
    if !(8..=256).contains(&regs) || shared > 65536 {
        die("--regs must be 8..=256 halves and --shared at most 65536 bytes");
    }
    for d in 0..3 {
        if wg[d] == 0 || grid[d] == 0 {
            die("--grid and --wg must be nonzero in every dimension");
        }
    }
    if wg.iter().map(|&x| x as u64).product::<u64>() > 1024 {
        die("at most 1024 threads per workgroup");
    }

    let dev = Device::open(&node).unwrap_or_else(|e| die(&e.to_string()));
    let mut vm = Vm::new(&dev).unwrap_or_else(|e| die(&e.to_string()));

    // Fixed-address regions, one object per region; page-rounded ranges must not overlap.
    regions.sort_by_key(|r| r.0);
    let mut bos = Vec::new();
    let mut last_end = 0u64;
    for (va, bytes) in &regions {
        let start = va & !(PAGE - 1);
        let end = (va + bytes.len() as u64).div_ceil(PAGE) * PAGE;
        if start < last_end {
            die(&format!("region at {va:#x} overlaps the previous one after page rounding"));
        }
        let mut bo = vm.alloc_at(start, end - start).unwrap_or_else(|e| die(&e.to_string()));
        let off = (va - start) as usize;
        bo.as_mut_slice()[off..off + bytes.len()].copy_from_slice(bytes);
        bos.push(bo);
        last_end = end;
    }

    // Uniform values, preloaded from a data object: 32-bit uniform I is halves 2I and 2I+1.
    let n32 = uniforms.keys().max().map_or(0, |&k| k + 1);
    if n32 > 128 {
        die("uniforms above u255 do not exist");
    }
    let mut unif = vm.alloc_data(PAGE).unwrap_or_else(|e| die(&e.to_string()));
    for (&k, &v) in &uniforms {
        unif.as_mut_slice()[4 * k..4 * k + 4].copy_from_slice(&v.to_le_bytes());
    }

    // the stack, from the helper program
    let stack_dwords = stack.div_ceil(4);
    let helper = if stack_dwords > 0 { Some(vm.scratch(stack_dwords).unwrap_or_else(|e| die(&e.to_string()))) } else { None };

    // USC object: pipeline words at 0, code at 0x400 (64-byte aligned, with 64 bytes of slack after it).
    let code_off = 0x400u64;
    let mut usc = vm.alloc_usc(code_off + code.len() as u64 + 64).unwrap_or_else(|e| die(&e.to_string()));
    let usc_off = (usc.va - vm.usc_base) as u32;
    let pipeline = Pipeline {
        code: usc_off + code_off as u32,
        gpr_halves: regs,
        uniforms: if n32 > 0 { vec![(0, 2 * n32 as u32, unif.va)] } else { vec![] },
        shared_bytes: shared,
        spill_bucket: if stack_dwords > 0 { scratch::Layout::new(stack_dwords).bucket } else { 0 },
    };
    let words = pipeline.usc_words();
    if words.len() > code_off as usize {
        die("too many pipeline words");
    }
    usc.as_mut_slice()[..words.len()].copy_from_slice(&words);
    usc.as_mut_slice()[code_off as usize..code_off as usize + code.len()].copy_from_slice(&code);

    let stream = cdm_dispatch(usc_off, pipeline.uniform_halves(), dev.g14x(), grid, wg);
    let mut cdm = vm.alloc_data(PAGE).unwrap_or_else(|e| die(&e.to_string()));
    cdm.as_mut_slice()[..stream.len()].copy_from_slice(&stream);

    let mut queue = Queue::new(&mut vm).unwrap_or_else(|e| die(&e.to_string()));
    queue.timeout = Duration::from_millis(timeout);
    let hz = dev.params.command_timestamp_frequency_hz.max(1) as u128;
    let initial: Vec<Vec<u8>> = bos.iter().map(|b| b.as_slice().to_vec()).collect();
    let (mut times, mut busy_ns, mut t) = (Vec::new(), 0u128, None);
    while (times.len() as u64) < repeat || (busy_ns < warm_ms as u128 * 1_000_000 && times.len() < 5000) {
        if !times.is_empty() && !keep_memory {
            for (bo, init) in bos.iter_mut().zip(&initial) {
                bo.as_mut_slice().copy_from_slice(init);
            }
        }
        let tr = queue
            .run_compute_with_helper(cdm.va, cdm.va + stream.len() as u64, helper)
            .unwrap_or_else(|e| die(&e.to_string()));
        let ns = tr.end.wrapping_sub(tr.start) as u128 * 1_000_000_000 / hz;
        times.push(ns);
        busy_ns += ns;
        t = Some(tr);
    }
    let t = t.unwrap();
    let first_ns = times[0];
    times.sort();

    for (va, n, f) in &dumps {
        let bo = bos
            .iter()
            .find(|b| *va >= b.va && va + *n as u64 <= b.va + b.size)
            .unwrap_or_else(|| die(&format!("--dump {va:#x} is not inside one mapped region")));
        let off = (va - bo.va) as usize;
        std::fs::write(f, &bo.as_slice()[off..off + n]).unwrap_or_else(|e| die(&format!("{f}: {e}")));
    }
    println!(
        "{{\"completed\":true,\"cpu\":\"{}\",\"cores\":{},\"fw_ticks\":{},\"fw_ns\":{},\"host_us\":{},\"repeat\":{},\"first_fw_ns\":{},\"min_fw_ns\":{},\"median_fw_ns\":{}}}",
        dev.llvm_cpu(),
        dev.core_count(),
        t.end.wrapping_sub(t.start),
        (t.end.wrapping_sub(t.start) as u128 * 1_000_000_000 / hz),
        t.host.as_micros(),
        times.len(),
        first_ns,
        times[0],
        times[times.len() / 2]
    );
}
