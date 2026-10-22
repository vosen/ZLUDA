//! The implemented CUDA driver functions. All state lives behind one mutex: `agx_runtime` objects are not
//! thread-safe, and CUDA calls from several threads are serialised here.

#![allow(non_snake_case)]

use crate::contexts;
use agx_runtime::cache::{Cache, Key};
use agx_runtime::context::Context;
use agx_runtime::{error, module, Fence, LoadedModule, Queue};
use cuda_types::cuda::*;
use std::collections::HashMap;
use std::ffi::{c_char, c_int, c_uint, c_void, CStr};
use std::io;
use std::num::NonZeroU32;
use std::sync::Mutex;
use std::time::Instant;

pub(crate) use crate::texture::*;

struct Stream {
    queue: Queue,
    /// Fences of other streams the next command must wait for (cuStreamWaitEvent).
    waits: Vec<Fence>,
}

#[derive(Clone, Copy)]
struct Event {
    /// The stream (0 for the default one) and its fence when recorded, if it had work.
    fence: Option<(usize, Fence)>,
    recorded: Option<Instant>,
}

/// A captured stream operation (CUDA graphs are built by stream capture).
#[derive(Clone)]
enum Op {
    Launch { f: usize, grid: [u32; 3], block: [u32; 3], shared: u32, params: Vec<Vec<u8>> },
    Memset { dst: u64, pattern: Vec<u8>, count: u64 },
    /// Device or host addresses, resolved when the graph runs (unified addressing).
    Copy { dst: u64, src: u64, n: usize },
}

pub(crate) struct Driver {
    default_waits: Vec<Fence>,
    modules: HashMap<usize, LoadedModule>,
    functions: HashMap<usize, (usize, String)>,
    streams: HashMap<usize, Stream>,
    events: HashMap<usize, Event>,
    /// cuCtxSetLimit values of this context.
    limits: HashMap<u32, usize>,
    /// Streams being captured (0 is the default stream) and their operations so far.
    capturing: HashMap<usize, Vec<Op>>,
    /// Graphs and their instantiations: both are operation lists.
    graphs: HashMap<usize, Vec<Op>>,
    execs: HashMap<usize, Vec<Op>>,
    /// Host allocations the GPU can address (their pointer is their GPU address), with their cuMemHostAlloc flags.
    host_allocs: HashMap<u64, c_uint>,
    /// cuMemAllocManaged allocations.
    managed: std::collections::HashSet<u64>,
    /// An error that makes the context unusable until it is destroyed, as in CUDA: a failed device assert
    /// (CUDA_ERROR_ASSERT). Every call returns it meanwhile.
    sticky: Option<CUerror>,
    /// The launches whose commands have not been seen to finish, oldest first: what a failed command was.
    launches: std::collections::VecDeque<LaunchRecord>,
    /// Per (module, kernel) built in checked mode, its first launch since the last synchronisation and how many
    /// followed: the launches a reported bad access may come from.
    checked_launches: HashMap<(usize, String), (String, u32)>,
    /// The device heap of `malloc` / `free`, made when the first module that uses it is loaded
    /// (CU_LIMIT_MALLOC_HEAP_SIZE bytes).
    heap: Option<agx_runtime::Bo>,
    /// The timer block of the clocks (`%globaltimer`, `%clock64`, `nanosleep`), made when the first module that
    /// reads them is loaded, with the CPU thread that keeps it current
    timer: Option<Timer>,
    /// Last: fields drop in declaration order, and the modules' and streams' objects must be unbound before the
    /// context's VM is destroyed (B42).
    pub(crate) ctx: Context,
}

/// A launch: its kernel, geometry and argument bytes (task description), kept until its command has finished.
#[derive(Clone, Debug)]
pub(crate) struct LaunchRecord {
    pub fence: Fence,
    pub stream: usize,
    pub module: usize,
    pub kernel: String,
    pub grid: [u32; 3],
    pub block: [u32; 3],
    pub shared: u32,
    pub params: Vec<Vec<u8>>,
}

impl LaunchRecord {
    pub fn describe(&self) -> String {
        let args: Vec<String> = self
            .params
            .iter()
            .map(|p| match p.len() {
                8 => format!("{:#x}", u64::from_le_bytes(p[..].try_into().unwrap())),
                4 => format!("{:#x}", u32::from_le_bytes(p[..].try_into().unwrap())),
                _ => p.iter().map(|b| format!("{b:02x}")).collect(),
            })
            .collect();
        format!(
            "kernel {} grid ({}, {}, {}) block ({}, {}, {}) shared {} bytes, stream {}, command {}, arguments [{}]",
            self.kernel,
            self.grid[0],
            self.grid[1],
            self.grid[2],
            self.block[0],
            self.block[1],
            self.block[2],
            self.shared,
            self.stream,
            self.fence.point,
            args.join(", ")
        )
    }
}

fn drop_finished(launches: &mut std::collections::VecDeque<LaunchRecord>, syncobj: u32, point: u64) {
    launches.retain(|r| !(r.fence.syncobj == syncobj && r.fence.point <= point));
}

/// Nanoseconds the GPU spent on this process's finished launches, by their firmware timestamps.
pub(crate) static GPU_BUSY_NS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// `GPU_BUSY_NS` for ZLUDA's NVML (`zluda_ml_agx`), which finds it in the process by name: the Asahi driver exposes
/// no device-wide utilisation to userspace (ptx-re-blockers B25).
#[no_mangle]
pub extern "C" fn zluda_agx_gpu_busy_ns() -> u64 {
    GPU_BUSY_NS.load(std::sync::atomic::Ordering::Relaxed)
}

/// At most this many launch records are kept; older ones are dropped.
const MAX_LAUNCH_RECORDS: usize = 1024;

fn stream_key(s: CUstream) -> usize {
    if (s.0 as usize) <= 2 {
        0
    } else {
        s.0 as usize
    }
}

impl Driver {
    /// Records `op` if `stream` is being captured.
    fn captured(&mut self, stream: CUstream, op: impl FnOnce() -> Op) -> bool {
        match self.capturing.get_mut(&stream_key(stream)) {
            Some(ops) => {
                ops.push(op());
                true
            }
            None => false,
        }
    }

    fn launch(&mut self, f: usize, grid: [u32; 3], block: [u32; 3], shared: u32, sh: usize, args: &[&[u8]]) -> CUresult {
        let (m, name) = self.functions.get(&f).cloned().ok_or(CUerror::INVALID_HANDLE)?;
        let threads = block.iter().map(|&b| b as u64).product::<u64>();
        let p = &self.ctx.props;
        // the launch words hold a 32-bit thread count per dimension
        let too_large = (0..3).any(|i| {
            block[i] > p.max_block_dim[i] || grid[i] > p.max_grid_dim[i] || grid[i] as u64 * block[i] as u64 > u32::MAX as u64
        });
        if threads == 0 || grid.contains(&0) || threads > p.max_threads_per_block as u64 || too_large {
            return Err(CUerror::INVALID_VALUE);
        }
        let k = &self.modules[&m].module.kernels[&name];
        // the kernel's registers do not leave room for this many threads on a core
        if threads > k.max_threads_per_threadgroup() as u64 {
            return Err(CUerror::LAUNCH_OUT_OF_RESOURCES);
        }
        // a cluster kernel's grid is a multiple of its cluster
        if (0..3).any(|i| grid[i] % k.cluster[i] != 0) {
            return Err(CUerror::INVALID_CLUSTER_SIZE);
        }
        let bounds = match k.checked {
            Some(_) => {
                let pointers = module::checked_pointers(k, args).map_err(|_| CUerror::INVALID_VALUE)?;
                pointers.iter().map(|&p| self.allocation_bounds(p)).collect()
            }
            None => Vec::new(),
        };
        let allocations = if k.checked.as_ref().is_some_and(|c| c.table.is_some()) { self.live_allocations() } else { Vec::new() };
        let g14x = self.ctx.device.g14x();
        let Driver { ctx, default_waits, modules, streams, .. } = self;
        let (queue, waits) = if sh <= 2 {
            (&mut ctx.stream, default_waits)
        } else {
            let s = streams.get_mut(&sh).ok_or(CUerror::INVALID_HANDLE)?;
            (&mut s.queue, &mut s.waits)
        };
        let w = std::mem::take(waits);
        let fence = modules[&m]
            .launch_params_async(&mut ctx.vm, queue, g14x, &name, args, grid, block, shared, &bounds, &allocations, &w)
            .map_err(from_io)?;
        if self.launches.len() == MAX_LAUNCH_RECORDS {
            self.launches.pop_front();
        }
        let record = LaunchRecord {
            fence,
            stream: sh,
            module: m,
            kernel: name,
            grid,
            block,
            shared,
            params: args.iter().map(|a| a.to_vec()).collect(),
        };
        if self.modules[&m].module.kernels[&record.kernel].checked.is_some() {
            self.checked_launches.entry((m, record.kernel.clone())).or_insert_with(|| (record.describe(), 0)).1 += 1;
        }
        self.launches.push_back(record);
        Ok(())
    }

    /// Every allocation a pointer can point into, for a checked kernel's allocation table: device memory and the
    /// modules' global data buffers.
    /// The size of the device heap: CU_LIMIT_MALLOC_HEAP_SIZE (8 MiB by default, as in CUDA), at most 4 GiB less
    /// 16 bytes (the heap's offsets are 32-bit).
    fn heap_size(&self) -> u32 {
        let limit = self.limits.get(&CUlimit_enum::CU_LIMIT_MALLOC_HEAP_SIZE.0).copied().unwrap_or(DEFAULT_HEAP_SIZE);
        (limit.min(u32::MAX as usize & !15) as u32).max(4096)
    }

    fn live_allocations(&self) -> Vec<(u64, u64)> {
        let mut all = self.ctx.allocation_ranges();
        all.extend(self.modules.values().filter_map(|m| m.globals_va().map(|va| (va, va + m.globals().len() as u64))));
        // the blocks of malloc are not told apart: the heap is one allocation
        all.extend(self.heap.as_ref().map(|h| (h.va, h.va + self.heap_size() as u64)));
        all
    }

    /// `[lo, hi)` of the allocation `p` points into, for a checked kernel's bounds: device memory, or a module's
    /// global data buffer (`cuModuleGetGlobal`); `lo = hi = p` (every access fails) if it points into neither.
    fn allocation_bounds(&self, p: u64) -> (u64, u64) {
        if let Some((start, size)) = self.ctx.address_range(p) {
            return (start, start + size);
        }
        for m in self.modules.values() {
            if let Some(va) = m.globals_va().filter(|&va| p >= va && p < va + m.globals().len() as u64) {
                return (va, va + m.globals().len() as u64);
            }
        }
        if let Some(h) = self.heap.as_ref().filter(|h| p >= h.va && p < h.va + self.heap_size() as u64) {
            return (h.va, h.va + self.heap_size() as u64);
        }
        (p, p)
    }

    /// The CUDA error of a failed wait on the queue whose timeline is `syncobj`. A failed command is reported with
    /// its launch record, and makes the context unusable (sticky), as in CUDA.
    fn failed(&mut self, syncobj: u32, e: io::Error) -> CUerror {
        let code = err(error::cuda_error(&e));
        if let Some(point) = e.get_ref().and_then(|i| i.downcast_ref::<error::GpuError>()).and_then(|g| g.point) {
            match self.launches.iter().find(|r| r.fence.syncobj == syncobj && r.fence.point == point) {
                Some(r) => {
                    eprintln!("zluda_agx: {e}: {}", r.describe());
                    if let Some(dir) = std::env::var_os("ZLUDA_AGX_DUMP").filter(|d| !d.is_empty()) {
                        match self.dump(r, &format!("{e}"), std::path::Path::new(&dir)) {
                            Ok(at) => eprintln!("zluda_agx: launch saved to {} (replay.sh runs it in agx_run)", at.display()),
                            Err(d) => eprintln!("zluda_agx: could not save the launch: {d}"),
                        }
                    }
                }
                None => eprintln!("zluda_agx: {e}: command {point}"),
            }
            self.sticky.get_or_insert(code);
        }
        code
    }

    /// Saves a failed launch under `dir/<pid>-<command>` for replaying it in the AGX Performance Simulator's
    /// `agx_run`: the kernel's code, the launch's uniforms, and the current contents of the memory it can reach (the
    /// allocations its pointer arguments point into, the module's global data, parameter block and error record),
    /// with `replay.sh`. Memory is as the fault left it, not as the launch found it.
    fn dump(&self, r: &LaunchRecord, error: &str, dir: &std::path::Path) -> io::Result<std::path::PathBuf> {
        let m = &self.modules[&r.module];
        let k = &m.module.kernels[&r.kernel];
        let params: Vec<&[u8]> = r.params.iter().map(|p| &p[..]).collect();
        let pointers = module::checked_pointers(k, &params).unwrap_or_default();
        let bounds: Vec<(u64, u64)> = pointers.iter().map(|&p| self.allocation_bounds(p)).collect();
        // the parameter block goes where no allocation is
        const PARAM_BLOCK: u64 = 0x7f_0000_0000;
        const TABLE: u64 = 0x7e_0000_0000;
        let allocations = self.live_allocations();
        let replay = m.replay(&r.kernel, &params, r.grid, r.block, r.shared, &bounds, &allocations, TABLE, PARAM_BLOCK)?;
        let mut regions = replay.regions.clone();
        // every allocation an 8-byte word of the arguments points into
        let words = r.params.iter().flat_map(|p| p.chunks_exact(8)).chain(replay.regions.first().filter(|(va, _)| *va == PARAM_BLOCK).into_iter().flat_map(|(_, b)| b.chunks_exact(8)));
        for w in words {
            if let Some((start, bytes)) = self.ctx.allocation(u64::from_le_bytes(w.try_into().unwrap())) {
                if !regions.iter().any(|(va, _)| *va == start) {
                    regions.push((start, bytes.to_vec()));
                }
            }
        }
        let at = dir.join(format!("{}-{}", std::process::id(), r.fence.point));
        write_dump(&at, &replay, &regions, &format!("{error}\n{}", r.describe()))?;
        Ok(at)
    }

    /// Drops the launch records of commands that have finished on the queue of `syncobj`, up to `point`.
    fn finished(&mut self, syncobj: u32, point: u64) {
        // the GPU time of the launches that finished, from their firmware timestamps (NVML utilisation)
        let freq = self.ctx.device.params.command_timestamp_frequency_hz.max(1) as u128;
        let queue = if self.ctx.stream.last().syncobj == syncobj {
            Some(&self.ctx.stream)
        } else {
            self.streams.values().map(|s| &s.queue).find(|q| q.last().syncobj == syncobj)
        };
        if let Some(q) = queue {
            let busy: u64 = self
                .launches
                .iter()
                .filter(|r| r.fence.syncobj == syncobj && r.fence.point <= point)
                .filter_map(|r| q.timing(r.fence))
                .map(|(start, end)| end.saturating_sub(start))
                .sum();
            GPU_BUSY_NS.fetch_add((busy as u128 * 1_000_000_000 / freq) as u64, std::sync::atomic::Ordering::Relaxed);
        }
        drop_finished(&mut self.launches, syncobj, point);
    }

    fn run(&mut self, op: &Op, sh: usize) -> CUresult {
        match op {
            Op::Launch { f, grid, block, shared, params } => {
                let args: Vec<&[u8]> = params.iter().map(|p| &p[..]).collect();
                self.launch(*f, *grid, *block, *shared, sh, &args)
            }
            Op::Memset { dst, pattern, count } => {
                self.sync_all()?;
                self.ctx.memset(*dst, pattern, *count).map_err(from_io)
            }
            Op::Copy { dst, src, n } => {
                self.sync_all()?;
                let (dd, sd) = (self.ctx.address_range(*dst).is_some(), self.ctx.address_range(*src).is_some());
                match (dd, sd) {
                    (true, true) => self.ctx.memcpy_dtod(*dst, *src, *n as u64),
                    (true, false) => self.ctx.memcpy_htod(*dst, unsafe { std::slice::from_raw_parts(*src as *const u8, *n) }),
                    (false, true) => self.ctx.memcpy_dtoh(unsafe { std::slice::from_raw_parts_mut(*dst as *mut u8, *n) }, *src),
                    (false, false) => {
                        unsafe { std::ptr::copy(*src as *const u8, *dst as *mut u8, *n) };
                        Ok(())
                    }
                }
                .map_err(from_io)
            }
        }
    }
}

// The runtime objects use Rc internally; they are only reached through DRIVER's mutex.
unsafe impl Send for Driver {}

/// The live contexts; None before cuInit.
static DRIVER: Mutex<Option<contexts::Contexts<Driver>>> = Mutex::new(None);

/// Handles of contexts, modules, functions, streams, events and graphs, unique across contexts.
fn new_handle() -> usize {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(16);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

/// A context's state on a newly opened device and VM.
fn new_driver() -> Result<Driver, CUerror> {
    let ctx = open_context().map_err(|_| CUerror::NO_DEVICE)?;
    Ok(Driver {
        ctx,
        default_waits: Vec::new(),
        modules: HashMap::new(),
        functions: HashMap::new(),
        streams: HashMap::new(),
        events: HashMap::new(),
        limits: HashMap::new(),
        capturing: HashMap::new(),
        graphs: HashMap::new(),
        execs: HashMap::new(),
        host_allocs: HashMap::new(),
        managed: Default::default(),
        sticky: None,
        launches: Default::default(),
        checked_launches: Default::default(),
        heap: None,
        timer: None,
    })
}

/// A timer block and the CPU thread that writes the CPU's timer into it until the context goes (a shader sees time
/// only through such a CPU-written word, ptx-re-blockers B35). The thread keeps a CPU core busy;
/// ZLUDA_AGX_TIMER=0 leaves the clocks counting reads instead.
struct Timer {
    bo: agx_runtime::Bo,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Timer {
    fn new(vm: &mut agx_runtime::Vm, clock_khz: u32) -> io::Result<Self> {
        let mut bo = vm.alloc_data(agx_runtime::TIMER_BLOCK_BYTES as u64)?;
        bo.as_mut_slice()[..agx_runtime::TIMER_BLOCK_BYTES].copy_from_slice(&agx_runtime::timer_block(clock_khz));
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let word = bo.as_mut_slice().as_mut_ptr() as usize;
        let flag = stop.clone();
        let thread = std::thread::Builder::new().name("zluda-agx-timer".into()).spawn(move || {
            while !flag.load(std::sync::atomic::Ordering::Relaxed) {
                // the mapping outlives the thread: Drop joins it before the buffer goes
                unsafe { std::ptr::write_volatile(word as *mut u64, agx_runtime::cpu_timer().0) };
                std::hint::spin_loop();
            }
        })?;
        Ok(Timer { bo, stop, thread: Some(thread) })
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.stop.store(true, std::sync::atomic::Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// Runs `f` on the registry of contexts.
fn with_contexts<T>(f: impl FnOnce(&mut contexts::Contexts<Driver>) -> Result<T, CUerror>) -> Result<T, CUerror> {
    let mut g = DRIVER.lock().unwrap_or_else(|p| p.into_inner());
    f(g.as_mut().ok_or(CUerror::NOT_INITIALIZED)?)
}

fn err(code: u32) -> CUerror {
    CUerror(NonZeroU32::new(code).unwrap())
}

pub(crate) fn from_io(e: io::Error) -> CUerror {
    err(error::cuda_error(&e))
}

pub(crate) fn with<T>(f: impl FnOnce(&mut Driver) -> Result<T, CUerror>) -> Result<T, CUerror> {
    with_sticky(|d| match d.sticky {
        Some(e) => Err(e),
        None => f(d),
    })
}

/// `with`, also after a sticky error (cuCtxDestroy). Runs on the thread's current context.
pub(crate) fn with_sticky<T>(f: impl FnOnce(&mut Driver) -> Result<T, CUerror>) -> Result<T, CUerror> {
    with_contexts(|c| {
        let h = c.current();
        f(c.get_mut(h).ok_or(CUerror::INVALID_CONTEXT)?)
    })
}

/// The Asahi render node: `$ZLUDA_AGX_NODE`, or the first render node that answers the Asahi parameter query.
fn open_context() -> io::Result<Context> {
    if let Ok(node) = std::env::var("ZLUDA_AGX_NODE") {
        return Context::new(&node);
    }
    let mut last = io::Error::new(io::ErrorKind::NotFound, "no Asahi render node");
    for n in 128..192 {
        match Context::new(&format!("/dev/dri/renderD{n}")) {
            Ok(c) => return Ok(c),
            Err(e) => last = e,
        }
    }
    Err(last)
}

impl Driver {
    fn handle(&mut self) -> usize {
        new_handle()
    }

    /// The queue of a stream handle; 0 and the legacy / per-thread handles (1, 2) are the default stream.
    fn queue(&mut self, stream: usize) -> Result<(&mut Queue, &mut Vec<Fence>), CUerror> {
        if stream <= 2 {
            return Ok((&mut self.ctx.stream, &mut self.default_waits));
        }
        let s = self.streams.get_mut(&stream).ok_or(CUerror::INVALID_HANDLE)?;
        Ok((&mut s.queue, &mut s.waits))
    }

    /// Prints the device printf output of every module and reports a failed device assert.
    fn after_sync(&mut self) -> Result<(), CUerror> {
        let mut failed = false;
        for (h, m) in self.modules.iter_mut() {
            if let Some(out) = m.drain_printf(|a| self.ctx.read_c_string(a)) {
                use std::io::Write;
                let mut so = std::io::stdout().lock();
                let _ = so.write_all(&out.text);
                if out.dropped_bytes > 0 {
                    let _ = writeln!(so, "[zluda_agx: {} bytes of printf output did not fit the buffer]", out.dropped_bytes);
                }
                let _ = so.flush();
            }
            failed |= m.take_assert_failure();
            if m.take_stack_overflow() && self.sticky.is_none() {
                eprintln!("zluda_agx: a thread ran out of its stack (recursion deeper than the backend's stack)");
                self.sticky = Some(CUerror::LAUNCH_FAILED);
            }
            if let Some(e) = m.checked_error().filter(|_| self.sticky.is_none()) {
                eprintln!("zluda_agx: {e}");
                if let Some((first, n)) = self.checked_launches.get(&(*h, site_kernel(&e.site_text).to_string())) {
                    match n {
                        1 => eprintln!("  in {first}"),
                        _ => eprintln!("  in one of {n} launches since the last synchronisation, the first {first}"),
                    }
                }
                self.sticky = Some(if e.misaligned() { CUerror::MISALIGNED_ADDRESS } else { CUerror::ILLEGAL_ADDRESS });
            }
        }
        self.checked_launches.clear();
        sticky_after_sync(&mut self.sticky, failed)
    }

    pub(crate) fn sync_all(&mut self) -> Result<(), CUerror> {
        let last = self.ctx.stream.last();
        if let Err(e) = self.ctx.stream.synchronize() {
            return Err(self.failed(last.syncobj, e));
        }
        self.finished(last.syncobj, last.point);
        let keys: Vec<usize> = self.streams.keys().copied().collect();
        for k in keys {
            let q = &mut self.streams.get_mut(&k).unwrap().queue;
            let last = q.last();
            if let Err(e) = q.synchronize() {
                return Err(self.failed(last.syncobj, e));
            }
            self.finished(last.syncobj, last.point);
        }
        self.after_sync()
    }
}

/// Writes a launch for `agx_run` to the directory `at`: `code.bin`, region `i` of `regions` as `m<i>.bin` (listed in
/// `regions.txt`), and `replay.sh`, which runs it (`$AGX_RUN` or `agx_run` on the PATH) and leaves each region's
/// final contents in `m<i>.out`. `header` goes into the script as comments.
fn write_dump(at: &std::path::Path, replay: &agx_runtime::Replay, regions: &[(u64, Vec<u8>)], header: &str) -> io::Result<()> {
    std::fs::create_dir_all(at)?;
    std::fs::write(at.join("code.bin"), &replay.code)?;
    if let Some(object) = &replay.debug_object {
        std::fs::write(at.join("module.o"), object)?;
    }
    let mut list = String::new();
    for (i, (va, bytes)) in regions.iter().enumerate() {
        std::fs::write(at.join(format!("m{i}.bin")), bytes)?;
        list += &format!("m{i}.bin {va:#x} {}\n", bytes.len());
    }
    std::fs::write(at.join("regions.txt"), list)?;
    let mut sh = String::from("#!/bin/sh\n");
    for line in header.lines() {
        sh += &format!("# {line}\n");
    }
    if replay.stack_bytes > 0 {
        sh += &format!("# the kernel uses {} bytes of stack per thread\n", replay.stack_bytes);
    }
    let args = replay.agx_run_args(regions, "m");
    sh += &format!("cd \"$(dirname \"$0\")\" && exec \"${{AGX_RUN:-agx_run}}\" {} \"$@\"\n", args.join(" "));
    std::fs::write(at.join("replay.sh"), sh)?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(at.join("replay.sh"), std::fs::Permissions::from_mode(0o755))?;
    }
    Ok(())
}

/// The kernel of a check site: the function of its outermost frame (`<access> <size>\t<function> ...`).
fn site_kernel(site_text: &str) -> &str {
    site_text.split('\t').skip(1).last().and_then(|f| f.split(' ').next()).unwrap_or_default()
}

/// The result of a synchronisation: a failed device assert makes CUDA_ERROR_ASSERT sticky.
fn sticky_after_sync(sticky: &mut Option<CUerror>, assert_failed: bool) -> Result<(), CUerror> {
    if assert_failed && sticky.is_none() {
        *sticky = Some(CUerror::ASSERT);
    }
    match *sticky {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// `zoc`, ZLUDA's PTX compiler built with the `agx` feature: `$ZLUDA_AGX_ZOC`, else `zoc` next to this library's
/// executable, else on `PATH`.
fn zoc_path() -> std::path::PathBuf {
    if let Some(p) = std::env::var_os("ZLUDA_AGX_ZOC") {
        return p.into();
    }
    if let Some(dir) = std::env::current_exe().ok().and_then(|e| e.parent().map(|p| p.to_owned())) {
        let p = dir.join("zoc");
        if p.exists() {
            return p;
        }
    }
    "zoc".into()
}

/// `ZLUDA_AGX_STRICT=1`: compile only to the instructions Mesa's asahi compiler emits (zoc --agx-strict).
pub(crate) fn agx_strict() -> bool {
    std::env::var("ZLUDA_AGX_STRICT").is_ok_and(|v| !v.is_empty() && v != "0")
}

/// `ZLUDA_AGX_CHECKED=1`: check every memory access of the kernels and report the first bad one with its source
/// location (zoc --agx-checked), as compute-sanitizer's memcheck does.
pub(crate) fn agx_checked() -> bool {
    std::env::var("ZLUDA_AGX_CHECKED").is_ok_and(|v| !v.is_empty() && v != "0")
}

/// `ZLUDA_AGX_LINEINFO=1`: keep the PTX source positions as a line table in the objects (zoc --lineinfo), so that
/// fault dumps replay with source lines (checked mode always has them).
fn agx_lineinfo() -> bool {
    std::env::var("ZLUDA_AGX_LINEINFO").is_ok_and(|v| !v.is_empty() && v != "0")
}

/// Compiles PTX for `cpu` with zoc, through the compilation cache.
pub(crate) fn compile_ptx(ptx: &[u8], cpu: &str) -> Result<(Vec<u8>, String), String> {
    compile_ptx_linked(&[ptx], cpu)
}

/// Compiles PTX modules into one AGX object: one module, or several linked together (`cuLink*`: functions declared
/// `.extern` in one are defined `.visible` in another).
pub(crate) fn compile_ptx_linked(sources: &[&[u8]], cpu: &str) -> Result<(Vec<u8>, String), String> {
    let zoc = zoc_path();
    let stamp = std::fs::metadata(&zoc)
        .and_then(|m| m.modified())
        .map(|t| format!("{t:?}"))
        .unwrap_or_default();
    let compiler = format!("zoc {} {stamp}", zoc.display());
    // the cache key of a link is every source, each prefixed with its length
    let joined;
    let source: &[u8] = if let [one] = sources {
        one
    } else {
        joined = sources.iter().flat_map(|s| (s.len() as u64).to_le_bytes().into_iter().chain(s.iter().copied())).collect::<Vec<u8>>();
        &joined
    };
    let mesa = agx_strict();
    let mut options: Vec<&str> = Vec::new();
    if sources.len() > 1 {
        options.push("link");
    }
    if mesa {
        options.push("agx-strict");
    }
    let checked = agx_checked();
    if checked {
        options.push("agx-checked");
    }
    let lineinfo = agx_lineinfo() && !checked;
    if lineinfo {
        options.push("lineinfo");
    }
    let stack_size = STACK_SIZE.load(std::sync::atomic::Ordering::Relaxed);
    let stack_option = format!("stack-size={stack_size}");
    if stack_size != DEFAULT_STACK_SIZE {
        options.push(&stack_option);
    }
    let key = Key { source, chip: cpu, compiler: &compiler, options: &options };
    let compile = || -> Result<(Vec<u8>, String), String> {
        let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
        let mut paths = Vec::new();
        for (i, ptx) in sources.iter().enumerate() {
            let src = dir.path().join(if i == 0 { "module.ptx".to_string() } else { format!("module_{i}.ptx") });
            std::fs::write(&src, ptx).map_err(|e| e.to_string())?;
            paths.push(src);
        }
        let out = std::process::Command::new(&zoc)
            .args(["--target", "agx", "--arch", cpu])
            .args(mesa.then_some("--agx-strict"))
            .args(checked.then_some("--agx-checked"))
            .args(lineinfo.then_some("--lineinfo"))
            .args((stack_size != DEFAULT_STACK_SIZE).then(|| format!("--stack-size={stack_size}")))
            .arg("--output-dir")
            .arg(dir.path())
            .args(&paths)
            .output()
            .map_err(|e| format!("{}: {e}", zoc.display()))?;
        let object = std::fs::read(dir.path().join("module.o"))
            .map_err(|_| format!("zoc failed: {}", String::from_utf8_lossy(&out.stderr)))?;
        let asm = std::fs::read_to_string(dir.path().join("module.asm")).unwrap_or_default();
        Ok((object, asm))
    };
    match Cache::from_env() {
        Some(c) => c.get_or_compile(&key, compile),
        None => compile(),
    }
}

/// The module in an image: an AGX object, or PTX text (NUL-terminated).
pub(crate) fn load_image(image: &[u8], cpu: &str) -> Result<module::Module, CUerror> {
    if image.starts_with(b"\x7fELF") {
        return module::Module::load_object(image).map_err(|_| CUerror::INVALID_IMAGE);
    }
    let (object, asm) = compile_ptx(image, cpu).map_err(|e| {
        eprintln!("zluda_agx: {e}");
        CUerror::INVALID_PTX
    })?;
    module::Module::load(&object, &asm).map_err(|_| CUerror::INVALID_IMAGE)
}

/// The PTX of a CUDA fat binary (a `__fatBinC_Wrapper_t` or a bare fatbin header): of the PTX files it holds, the
/// one for the newest architecture. NVIDIA machine code (SASS) cannot run here.
unsafe fn ptx_in_fatbin(image: *const u8) -> Result<Vec<u8>, CUerror> {
    use dark_api::fatbin::{Fatbin, FatbinSubmodule};
    let mut best: Option<(u32, Vec<u8>)> = None;
    let mut take = |sub: FatbinSubmodule| {
        let mut files = sub.get_files();
        while let Some(Ok(f)) = files.next() {
            if f.header.kind != cuda_types::dark_api::FatbinFileHeader::HEADER_KIND_PTX {
                continue;
            }
            let sm = f.header.sm_version;
            if best.as_ref().is_some_and(|(b, _)| *b >= sm) {
                continue;
            }
            if let Ok(text) = f.get_or_decompress_content(true) {
                best = Some((sm, text.into_owned()));
            }
        }
    };
    let magic = std::ptr::read_unaligned(image as *const u32);
    if magic == u32::from_le_bytes(cuda_types::dark_api::FatbincWrapper::MAGIC) {
        let ptr = image as *const c_void;
        let fatbin = Fatbin::new(&ptr).map_err(|_| CUerror::INVALID_IMAGE)?;
        let mut subs = fatbin.get_submodules().map_err(|_| CUerror::INVALID_IMAGE)?;
        while let Some(s) = subs.next() {
            take(s.map_err(|_| CUerror::INVALID_IMAGE)?);
        }
    } else {
        take(FatbinSubmodule::new(&*(image as *const cuda_types::dark_api::FatbinHeader)));
    }
    let (_, mut ptx) = best.ok_or(CUerror::NO_BINARY_FOR_GPU)?;
    ptx.push(0);
    Ok(ptx)
}

pub(crate) unsafe fn out<T>(p: *mut T, v: T) -> CUresult {
    if p.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    *p = v;
    Ok(())
}

pub(crate) unsafe fn cuInit(_flags: c_uint) -> CUresult {
    let mut g = DRIVER.lock().unwrap_or_else(|p| p.into_inner());
    if g.is_none() {
        *g = Some(contexts::Contexts::new(new_handle(), new_driver()?));
    }
    Ok(())
}

pub(crate) unsafe fn cuDriverGetVersion(v: *mut c_int) -> CUresult {
    out(v, 12040)
}

pub(crate) unsafe fn cuDeviceGetCount(count: *mut c_int) -> CUresult {
    let n = with(|_| Ok(1)).unwrap_or(0);
    out(count, n)
}

pub(crate) unsafe fn cuDeviceGet(device: *mut CUdevice, ordinal: c_int) -> CUresult {
    with(|_| if ordinal == 0 { Ok(()) } else { Err(CUerror::INVALID_DEVICE) })?;
    out(device, 0)
}

fn check_device(dev: CUdevice) -> Result<(), CUerror> {
    if dev == 0 {
        Ok(())
    } else {
        Err(CUerror::INVALID_DEVICE)
    }
}

pub(crate) unsafe fn cuDeviceGetName(name: *mut c_char, len: c_int, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let n = with(|d| Ok(d.ctx.props.name.clone()))?;
    if name.is_null() || len <= 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let bytes = n.as_bytes();
    let k = bytes.len().min(len as usize - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), name as *mut u8, k);
    *name.add(k) = 0;
    Ok(())
}

pub(crate) unsafe fn cuDeviceTotalMem_v2(bytes: *mut usize, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let t = with(|d| Ok(d.ctx.props.total_memory as usize))?;
    out(bytes, t)
}

/// CUDA compute capability reported for AGX: what ZLUDA's PTX front end accepts.
const CC: (c_int, c_int) = (8, 6);

pub(crate) unsafe fn cuDeviceComputeCapability(major: *mut c_int, minor: *mut c_int, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    out(major, CC.0)?;
    out(minor, CC.1)
}

pub(crate) unsafe fn cuDeviceGetAttribute(pi: *mut c_int, attrib: CUdevice_attribute, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    use CUdevice_attribute_enum as A;
    let v = with(|d| {
        let p = &d.ctx.props;
        Ok(match attrib {
            A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_BLOCK => p.max_threads_per_block as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_X => p.max_block_dim[0] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Y => p.max_block_dim[1] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCK_DIM_Z => p.max_block_dim[2] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_X => p.max_grid_dim[0].min(i32::MAX as u32) as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Y => p.max_grid_dim[1] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_GRID_DIM_Z => p.max_grid_dim[2] as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK
            | A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_BLOCK_OPTIN
            | A::CU_DEVICE_ATTRIBUTE_MAX_SHARED_MEMORY_PER_MULTIPROCESSOR => p.shared_memory_per_block as c_int,
            A::CU_DEVICE_ATTRIBUTE_WARP_SIZE => p.warp_size as c_int,
            A::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_BLOCK | A::CU_DEVICE_ATTRIBUTE_MAX_REGISTERS_PER_MULTIPROCESSOR => 65536,
            A::CU_DEVICE_ATTRIBUTE_CLOCK_RATE => p.clock_khz as c_int,
            A::CU_DEVICE_ATTRIBUTE_MULTIPROCESSOR_COUNT => p.cores as c_int,
            // a core runs up to 96 SIMD-groups (G14 occupancy at the smallest register allocation)
            A::CU_DEVICE_ATTRIBUTE_MAX_THREADS_PER_MULTIPROCESSOR => 96 * 32,
            A::CU_DEVICE_ATTRIBUTE_MAX_BLOCKS_PER_MULTIPROCESSOR => 32,
            A::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR => CC.0,
            A::CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR => CC.1,
            A::CU_DEVICE_ATTRIBUTE_INTEGRATED | A::CU_DEVICE_ATTRIBUTE_UNIFIED_ADDRESSING => 1,
            // managed and mapped host memory are GPU allocations mapped at their GPU address
            A::CU_DEVICE_ATTRIBUTE_MANAGED_MEMORY | A::CU_DEVICE_ATTRIBUTE_CAN_MAP_HOST_MEMORY => 1,
            A::CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS | A::CU_DEVICE_ATTRIBUTE_PAGEABLE_MEMORY_ACCESS => 0,
            A::CU_DEVICE_ATTRIBUTE_CONCURRENT_KERNELS => 1,
            // grids up to the co-resident limit (B5); thread block clusters as compiled (B28)
            A::CU_DEVICE_ATTRIBUTE_COOPERATIVE_LAUNCH | A::CU_DEVICE_ATTRIBUTE_CLUSTER_LAUNCH => 1,
            // no run time limit on kernels (ptx-re-blockers B6)
            A::CU_DEVICE_ATTRIBUTE_KERNEL_EXEC_TIMEOUT => 0,
            A::CU_DEVICE_ATTRIBUTE_TOTAL_CONSTANT_MEMORY => 65536,
            A::CU_DEVICE_ATTRIBUTE_TEXTURE_ALIGNMENT => 256,
            _ => 0,
        })
    })?;
    out(pi, v)
}

pub(crate) unsafe fn cuDevicePrimaryCtxRetain(pctx: *mut CUcontext, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let h = with_contexts(|c| Ok(c.retain_primary()))?;
    out(pctx, CUcontext(h as *mut _))
}

/// The primary context stays alive at no retains (the CUDA runtime retains it again before using it); its
/// resources are freed by cuDevicePrimaryCtxReset.
pub(crate) unsafe fn cuDevicePrimaryCtxRelease_v2(dev: CUdevice) -> CUresult {
    check_device(dev)?;
    with_contexts(|c| c.release_primary().map_err(|_| CUerror::INVALID_CONTEXT))
}

/// cudaDeviceReset: waits for the primary context's work, frees all its memory, modules, streams, events and
/// graphs, and leaves a fresh context under the same handle (a sticky error ends here).
pub(crate) unsafe fn cuDevicePrimaryCtxReset_v2(dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let fresh = new_driver()?;
    let old = with_contexts(|c| Ok(c.reset_primary(fresh)))?;
    if let Some(mut d) = old {
        let _ = d.sync_all();
    }
    Ok(())
}

/// A new context with its own VM: its memory, modules and streams are its own and are freed with it.
pub(crate) unsafe fn cuCtxCreate_v2(pctx: *mut CUcontext, _flags: c_uint, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let d = new_driver()?;
    let h = new_handle();
    with_contexts(|c| {
        c.insert(h, d);
        Ok(())
    })?;
    contexts::push(h);
    out(pctx, CUcontext(h as *mut _))
}

pub(crate) unsafe fn cuCtxDestroy_v2(ctx: CUcontext) -> CUresult {
    let h = ctx.0 as usize;
    let removed = with_contexts(|c| {
        if !c.contains(h) {
            return Err(CUerror::INVALID_CONTEXT);
        }
        Ok(c.remove(h))
    })?;
    contexts::forget(h);
    match removed {
        // a created context: wait for its work, then drop everything it owns
        Some(mut d) => {
            let r = d.sync_all();
            drop(d);
            r.or_else(|e| if e == CUerror::ASSERT { Ok(()) } else { Err(e) })
        }
        // the primary context: as before, its sticky error ends (the next use starts clean)
        None => with_contexts(|c| {
            let d = c.get_mut(h).ok_or(CUerror::INVALID_CONTEXT)?;
            let r = d.sync_all();
            d.sticky = None;
            d.modules.values_mut().for_each(|m| m.reset_checked());
            r.or_else(|e| if e == CUerror::ASSERT { Ok(()) } else { Err(e) })
        }),
    }
}

fn check_context(ctx: CUcontext) -> Result<Option<usize>, CUerror> {
    if ctx.0.is_null() {
        return Ok(None);
    }
    let h = ctx.0 as usize;
    with_contexts(|c| if c.contains(h) { Ok(Some(h)) } else { Err(CUerror::INVALID_CONTEXT) })
}

pub(crate) unsafe fn cuCtxSetCurrent(ctx: CUcontext) -> CUresult {
    contexts::set(check_context(ctx)?);
    Ok(())
}

pub(crate) unsafe fn cuCtxGetCurrent(pctx: *mut CUcontext) -> CUresult {
    with_contexts(|_| Ok(()))?;
    out(pctx, CUcontext(contexts::top().unwrap_or(0) as *mut _))
}

pub(crate) unsafe fn cuCtxGetDevice(device: *mut CUdevice) -> CUresult {
    out(device, 0)
}

pub(crate) unsafe fn cuCtxSynchronize() -> CUresult {
    with(|d| d.sync_all())
}

pub(crate) unsafe fn cuMemAlloc_v2(dptr: *mut CUdeviceptr, bytesize: usize) -> CUresult {
    let va = with(|d| d.ctx.mem_alloc(bytesize as u64).map_err(from_io))?;
    out(dptr, CUdeviceptr_v2(va as *mut c_void))
}

pub(crate) unsafe fn cuMemFree_v2(dptr: CUdeviceptr) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.mem_free(dptr.0 as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemGetInfo_v2(free: *mut usize, total: *mut usize) -> CUresult {
    let (f, t) = with(|d| Ok(d.ctx.mem_info()))?;
    out(free, f as usize)?;
    out(total, t as usize)
}

pub(crate) unsafe fn cuMemcpyHtoD_v2(dst: CUdeviceptr, src: *const c_void, n: usize) -> CUresult {
    if n == 0 {
        return Ok(());
    }
    let s = std::slice::from_raw_parts(src as *const u8, n);
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_htod(dst.0 as u64, s).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemcpyDtoH_v2(dst: *mut c_void, src: CUdeviceptr, n: usize) -> CUresult {
    if n == 0 {
        return Ok(());
    }
    let s = std::slice::from_raw_parts_mut(dst as *mut u8, n);
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_dtoh(s, src.0 as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemcpyDtoD_v2(dst: CUdeviceptr, src: CUdeviceptr, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memcpy_dtod(dst.0 as u64, src.0 as u64, n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemsetD8_v2(dst: CUdeviceptr, uc: u8, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memset(dst.0 as u64, &[uc], n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemsetD32_v2(dst: CUdeviceptr, ui: c_uint, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memset(dst.0 as u64, &ui.to_le_bytes(), n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuModuleLoadData(module: *mut CUmodule, image: *const c_void) -> CUresult {
    if image.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let bytes = image as *const u8;
    let h = with(|d| {
        let cpu = d.ctx.props.llvm_cpu.clone();
        // An ELF object's size is in its header; PTX is NUL-terminated.
        let magic = std::ptr::read_unaligned(bytes as *const u32);
        let unpacked;
        let image: &[u8] = if magic == 0x466243b1 || magic == 0xba55ed50 {
            unpacked = ptx_in_fatbin(bytes)?;
            &unpacked[..unpacked.len() - 1]
        } else if std::slice::from_raw_parts(bytes, 8) == module::FAT_MAGIC {
            // a fat AGX object (library kernels): the object for this device's chip, in strict mode its
            // "+agx-strict" build, in checked mode its "+agx-checked" one if it has one
            let fat = std::slice::from_raw_parts(bytes, module::fat_object_len(bytes));
            let entry = format!("{cpu}{}", if agx_strict() { "+agx-strict" } else { "" });
            // library kernels without a checked build run unchecked
            let checked = agx_checked().then(|| module::fat_object_entry(fat, &format!("{entry}+agx-checked")).ok()).flatten();
            checked.map_or_else(|| module::fat_object_entry(fat, &entry), Ok).map_err(|e| {
                if crate::log_enabled() {
                    eprintln!("[zluda_agx] {}", e.0);
                }
                CUerror::NO_BINARY_FOR_GPU
            })?
        } else if std::slice::from_raw_parts(bytes, 4) == b"\x7fELF" {
            let h = std::slice::from_raw_parts(bytes, 64);
            let shoff = u64::from_le_bytes(h[40..48].try_into().unwrap());
            let (shentsize, shnum) = (u16::from_le_bytes([h[58], h[59]]) as u64, u16::from_le_bytes([h[60], h[61]]) as u64);
            std::slice::from_raw_parts(bytes, (shoff + shentsize * shnum) as usize)
        } else {
            CStr::from_ptr(bytes as *const c_char).to_bytes()
        };
        let m = load_image(image, &cpu)?;
        let mut loaded = LoadedModule::new(&mut d.ctx.vm, m).map_err(from_io)?;
        if loaded.uses_heap() {
            if d.heap.is_none() {
                let size = d.heap_size();
                let mut heap = d.ctx.vm.alloc_data(size as u64).map_err(from_io)?;
                heap.as_mut_slice()[..8].copy_from_slice(&agx_runtime::heap_header(size));
                d.heap = Some(heap);
            }
            loaded.set_heap(d.heap.as_ref().unwrap().va);
        }
        if loaded.uses_timer() && std::env::var("ZLUDA_AGX_TIMER").map_or(true, |v| v != "0") {
            if d.timer.is_none() {
                let khz = d.ctx.props.clock_khz;
                d.timer = Some(Timer::new(&mut d.ctx.vm, khz).map_err(from_io)?);
            }
            loaded.set_timer(d.timer.as_ref().unwrap().bo.va);
        }
        let h = d.handle();
        d.modules.insert(h, loaded);
        Ok(h)
    })?;
    out(module, CUmodule(h as *mut _))
}

pub(crate) unsafe fn cuModuleLoadDataEx(
    module: *mut CUmodule,
    image: *const c_void,
    _num_options: c_uint,
    _options: *mut CUjit_option,
    _values: *mut *mut c_void,
) -> CUresult {
    // the JIT options (log buffers, optimization level, ...) change nothing here
    cuModuleLoadData(module, image)
}

pub(crate) unsafe fn cuModuleLoadFatBinary(module: *mut CUmodule, image: *const c_void) -> CUresult {
    cuModuleLoadData(module, image)
}

pub(crate) unsafe fn cuModuleLoad(module: *mut CUmodule, fname: *const c_char) -> CUresult {
    if fname.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let path = std::path::PathBuf::from(CStr::from_ptr(fname).to_string_lossy().into_owned());
    let mut data = std::fs::read(&path).map_err(|_| CUerror::FILE_NOT_FOUND)?;
    data.push(0);
    cuModuleLoadData(module, data.as_ptr() as *const c_void)
}

/// A `cuLink*` session: the PTX of each input, then the linked AGX object.
#[derive(Default)]
struct Link {
    ptx: Vec<Vec<u8>>,
    /// An AGX object given as the only input passes through unchanged.
    object: Option<Vec<u8>>,
    linked: Option<Vec<u8>>,
}

pub(crate) unsafe fn cuLinkCreate_v2(
    _num_options: c_uint,
    _options: *mut CUjit_option,
    _values: *mut *mut c_void,
    state: *mut CUlinkState,
) -> CUresult {
    out(state, Box::into_raw(Box::<Link>::default()) as CUlinkState)
}

unsafe fn link_add(state: CUlinkState, ty: CUjitInputType, data: &[u8]) -> CUresult {
    let link = (state as *mut Link).as_mut().ok_or(CUerror::INVALID_HANDLE)?;
    match ty {
        CUjitInputType::CU_JIT_INPUT_PTX => {
            let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
            link.ptx.push(data[..end].to_vec());
        }
        CUjitInputType::CU_JIT_INPUT_FATBINARY => {
            let mut ptx = ptx_in_fatbin(data.as_ptr())?;
            ptx.pop();
            link.ptx.push(ptx);
        }
        CUjitInputType::CU_JIT_INPUT_CUBIN | CUjitInputType::CU_JIT_INPUT_OBJECT if data.starts_with(b"\x7fELF") => {
            // AGX objects are final: one can only be "linked" alone
            if link.object.is_some() {
                return Err(CUerror::NOT_SUPPORTED);
            }
            link.object = Some(data.to_vec());
        }
        CUjitInputType::CU_JIT_INPUT_CUBIN | CUjitInputType::CU_JIT_INPUT_OBJECT | CUjitInputType::CU_JIT_INPUT_LIBRARY => {
            // NVIDIA machine code, or an archive of it
            return Err(CUerror::NO_BINARY_FOR_GPU);
        }
        _ => return Err(CUerror::NOT_SUPPORTED),
    }
    Ok(())
}

pub(crate) unsafe fn cuLinkAddData_v2(
    state: CUlinkState,
    ty: CUjitInputType,
    data: *mut c_void,
    size: usize,
    _name: *const c_char,
    _num_options: c_uint,
    _options: *mut CUjit_option,
    _values: *mut *mut c_void,
) -> CUresult {
    if data.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    // a fat binary's size comes from its header
    let size = if size == 0 && ty == CUjitInputType::CU_JIT_INPUT_PTX {
        CStr::from_ptr(data as *const c_char).to_bytes().len()
    } else {
        size
    };
    if ty == CUjitInputType::CU_JIT_INPUT_FATBINARY {
        let link = (state as *mut Link).as_mut().ok_or(CUerror::INVALID_HANDLE)?;
        let mut ptx = ptx_in_fatbin(data as *const u8)?;
        ptx.pop();
        link.ptx.push(ptx);
        return Ok(());
    }
    link_add(state, ty, std::slice::from_raw_parts(data as *const u8, size))
}

pub(crate) unsafe fn cuLinkAddFile_v2(
    state: CUlinkState,
    ty: CUjitInputType,
    path: *const c_char,
    _num_options: c_uint,
    _options: *mut CUjit_option,
    _values: *mut *mut c_void,
) -> CUresult {
    if path.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let data = std::fs::read(CStr::from_ptr(path).to_string_lossy().as_ref()).map_err(|_| CUerror::FILE_NOT_FOUND)?;
    link_add(state, ty, &data)
}

pub(crate) unsafe fn cuLinkComplete(state: CUlinkState, cubin: *mut *mut c_void, size: *mut usize) -> CUresult {
    let link = (state as *mut Link).as_mut().ok_or(CUerror::INVALID_HANDLE)?;
    let object = match (&link.object, link.ptx.len()) {
        (Some(o), 0) => o.clone(),
        (Some(_), _) => return Err(CUerror::NOT_SUPPORTED),
        (None, 0) => return Err(CUerror::INVALID_VALUE),
        (None, _) => {
            let cpu = with(|d| Ok(d.ctx.props.llvm_cpu.clone()))?;
            let sources: Vec<&[u8]> = link.ptx.iter().map(|p| &p[..]).collect();
            compile_ptx_linked(&sources, &cpu)
                .map_err(|e| {
                    eprintln!("zluda_agx: {e}");
                    CUerror::INVALID_PTX
                })?
                .0
        }
    };
    let linked = link.linked.insert(object);
    out(cubin, linked.as_mut_ptr() as *mut c_void)?;
    out(size, linked.len())
}

pub(crate) unsafe fn cuLinkDestroy(state: CUlinkState) -> CUresult {
    if state.is_null() {
        return Err(CUerror::INVALID_HANDLE);
    }
    drop(Box::from_raw(state as *mut Link));
    Ok(())
}

pub(crate) unsafe fn cuModuleUnload(hmod: CUmodule) -> CUresult {
    with(|d| {
        d.sync_all()?;
        let h = hmod.0 as usize;
        d.modules.remove(&h).ok_or(CUerror::INVALID_HANDLE)?;
        d.functions.retain(|_, (m, _)| *m != h);
        Ok(())
    })
}

pub(crate) unsafe fn cuModuleGetFunction(hfunc: *mut CUfunction, hmod: CUmodule, name: *const c_char) -> CUresult {
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let h = with(|d| {
        let m = d.modules.get(&(hmod.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if !m.module.kernels.contains_key(&name) {
            return Err(CUerror::NOT_FOUND);
        }
        let h = d.handle();
        d.functions.insert(h, (hmod.0 as usize, name));
        Ok(h)
    })?;
    out(hfunc, CUfunction(h as *mut _))
}

pub(crate) unsafe fn cuModuleGetGlobal_v2(dptr: *mut CUdeviceptr, bytes: *mut usize, hmod: CUmodule, name: *const c_char) -> CUresult {
    let name = CStr::from_ptr(name).to_string_lossy().into_owned();
    let (va, size) = with(|d| {
        let m = d.modules.get(&(hmod.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let &(off, size) = m.module.global_symbols.get(&name).ok_or(CUerror::NOT_FOUND)?;
        Ok((m.globals_va().ok_or(CUerror::NOT_FOUND)? + off, size))
    })?;
    if !dptr.is_null() {
        *dptr = CUdeviceptr_v2(va as *mut c_void);
    }
    if !bytes.is_null() {
        *bytes = size as usize;
    }
    Ok(())
}

pub(crate) unsafe fn cuFuncGetAttribute(pi: *mut c_int, attrib: CUfunction_attribute, hfunc: CUfunction) -> CUresult {
    use CUfunction_attribute_enum as F;
    let v = with(|d| {
        let (m, name) = d.functions.get(&(hfunc.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let k = &d.modules[m].module.kernels[name];
        Ok(match attrib {
            F::CU_FUNC_ATTRIBUTE_MAX_THREADS_PER_BLOCK => k.max_threads_per_threadgroup() as c_int,
            F::CU_FUNC_ATTRIBUTE_NUM_REGS => k.gprs as c_int,
            F::CU_FUNC_ATTRIBUTE_SHARED_SIZE_BYTES => k.threadgroup_bytes as c_int,
            F::CU_FUNC_ATTRIBUTE_LOCAL_SIZE_BYTES => k.stack_bytes as c_int,
            _ => 0,
        })
    })?;
    out(pi, v)
}

pub(crate) unsafe fn cuLaunchKernel(
    f: CUfunction,
    gx: c_uint,
    gy: c_uint,
    gz: c_uint,
    bx: c_uint,
    by: c_uint,
    bz: c_uint,
    shared: c_uint,
    stream: CUstream,
    params: *mut *mut c_void,
    extra: *mut *mut c_void,
) -> CUresult {
    if !extra.is_null() {
        return Err(CUerror::NOT_SUPPORTED);
    }
    with(|d| {
        let (m, name) = d.functions.get(&(f.0 as usize)).cloned().ok_or(CUerror::INVALID_HANDLE)?;
        let slots = d.modules[&m].module.kernels[&name].args.clone();
        let args: Vec<&[u8]> = slots
            .iter()
            .enumerate()
            .map(|(i, s)| std::slice::from_raw_parts(*params.add(i) as *const u8, s.bytes as usize))
            .collect();
        let (grid, block) = ([gx, gy, gz], [bx, by, bz]);
        let op = || Op::Launch { f: f.0 as usize, grid, block, shared, params: args.iter().map(|a| a.to_vec()).collect() };
        if d.captured(stream, op) {
            return Ok(());
        }
        d.launch(f.0 as usize, grid, block, shared, stream.0 as usize, &args)
    })
}

pub(crate) unsafe fn cuStreamCreate(ph: *mut CUstream, _flags: c_uint) -> CUresult {
    let h = with(|d| {
        let queue = Queue::new(&mut d.ctx.vm).map_err(from_io)?;
        let h = d.handle();
        d.streams.insert(h, Stream { queue, waits: Vec::new() });
        Ok(h)
    })?;
    out(ph, CUstream(h as *mut _))
}

pub(crate) unsafe fn cuStreamDestroy_v2(s: CUstream) -> CUresult {
    with(|d| {
        let mut st = d.streams.remove(&(s.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let last = st.queue.last();
        match st.queue.synchronize() {
            Ok(()) => {
                d.finished(last.syncobj, last.point);
                Ok(())
            }
            Err(e) => Err(d.failed(last.syncobj, e)),
        }
    })
}

pub(crate) unsafe fn cuStreamSynchronize(s: CUstream) -> CUresult {
    with(|d| {
        let q = d.queue(s.0 as usize)?.0;
        let last = q.last();
        if let Err(e) = q.synchronize() {
            return Err(d.failed(last.syncobj, e));
        }
        d.finished(last.syncobj, last.point);
        d.after_sync()
    })
}

pub(crate) unsafe fn cuStreamWaitEvent(s: CUstream, e: CUevent, _flags: c_uint) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if let Some((_, fence)) = ev.fence {
            d.queue(s.0 as usize)?.1.push(fence);
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuEventCreate(ph: *mut CUevent, _flags: c_uint) -> CUresult {
    let h = with(|d| {
        let h = d.handle();
        d.events.insert(h, Event { fence: None, recorded: None });
        Ok(h)
    })?;
    out(ph, h as CUevent)
}

pub(crate) unsafe fn cuEventDestroy_v2(e: CUevent) -> CUresult {
    with(|d| d.events.remove(&(e as usize)).map(|_| ()).ok_or(CUerror::INVALID_HANDLE))
}

pub(crate) unsafe fn cuEventRecord(e: CUevent, s: CUstream) -> CUresult {
    with(|d| {
        let sh = if (s.0 as usize) <= 2 { 0 } else { s.0 as usize };
        let fence = d.queue(sh)?.0.last();
        let ev = d.events.get_mut(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        ev.fence = (fence.point != 0).then_some((sh, fence));
        ev.recorded = Some(Instant::now());
        Ok(())
    })
}

pub(crate) unsafe fn cuEventQuery(e: CUevent) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let Some((s, fence)) = ev.fence else { return Ok(()) };
        if d.queue(s)?.0.is_done(fence).map_err(from_io)? {
            Ok(())
        } else {
            Err(CUerror::NOT_READY)
        }
    })
}

pub(crate) unsafe fn cuEventSynchronize(e: CUevent) -> CUresult {
    with(|d| {
        let ev = *d.events.get(&(e as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        if let Some((s, fence)) = ev.fence {
            let q = d.queue(s)?.0;
            let timeout = q.timeout;
            if let Err(e) = q.wait(fence, timeout) {
                return Err(d.failed(fence.syncobj, e));
            }
            d.finished(fence.syncobj, fence.point);
        }
        d.after_sync()
    })
}

/// Milliseconds between two recorded events: from the firmware timestamps of the commands they follow when both
/// have one, else from the host clock at recording.
pub(crate) unsafe fn cuEventElapsedTime(ms: *mut f32, start: CUevent, end: CUevent) -> CUresult {
    let t = with(|d| {
        let a = *d.events.get(&(start as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let b = *d.events.get(&(end as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let freq = d.ctx.device.params.command_timestamp_frequency_hz.max(1) as f64;
        if let (Some((sa, fa)), Some((sb, fb))) = (a.fence, b.fence) {
            let ta = d.queue(sa)?.0.timing(fa);
            let tb = d.queue(sb)?.0.timing(fb);
            if let (Some((_, ea)), Some((_, eb))) = (ta, tb) {
                if ea != 0 && eb != 0 {
                    return Ok(((eb as f64 - ea as f64) / freq * 1e3) as f32);
                }
            }
        }
        match (a.recorded, b.recorded) {
            (Some(x), Some(y)) => Ok(y.saturating_duration_since(x).as_secs_f32() * 1e3),
            _ => Err(CUerror::INVALID_HANDLE),
        }
    })?;
    out(ms, t)
}

// ---- entry points the CUDA runtime (libcudart) uses at start-up and for every launch

fn raw_match(name: &[u8], flag: u64, version: i32) -> *mut c_void {
    #[allow(unused_imports)]
    use crate::*;
    include!("../../zluda_bindgen/src/process_table.rs")
}

pub(crate) unsafe fn cuGetProcAddress_v2(
    symbol: *const c_char,
    pfn: *mut *mut c_void,
    version: c_int,
    flags: cuuint64_t,
    status: *mut CUdriverProcAddressQueryResult,
) -> CUresult {
    if symbol.is_null() || pfn.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    let f = raw_match(CStr::from_ptr(symbol).to_bytes(), flags, version);
    let (r, st) = match f as usize {
        0 => (Err(CUerror::NOT_FOUND), CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND),
        usize::MAX => (Err(CUerror::NOT_FOUND), CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_VERSION_NOT_SUFFICIENT),
        _ => (Ok(()), CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS),
    };
    *pfn = if r.is_ok() { f } else { std::ptr::null_mut() };
    if !status.is_null() {
        *status = st;
    }
    r
}

pub(crate) use crate::dark::cuGetExportTable;

pub(crate) unsafe fn cuGetProcAddress(symbol: *const c_char, pfn: *mut *mut c_void, version: c_int, flags: cuuint64_t) -> CUresult {
    cuGetProcAddress_v2(symbol, pfn, version, flags, std::ptr::null_mut())
}

// Libraries are modules, and kernels are functions.
pub(crate) unsafe fn cuLibraryLoadData(
    library: *mut CUlibrary,
    code: *const c_void,
    _jit_options: *mut CUjit_option,
    _jit_values: *mut *mut c_void,
    _num_jit: c_uint,
    _lib_options: *mut CUlibraryOption,
    _lib_values: *mut *mut c_void,
    _num_lib: c_uint,
) -> CUresult {
    let mut m = CUmodule(std::ptr::null_mut());
    cuModuleLoadData(&mut m, code)?;
    out(library, CUlibrary(m.0 as *mut _))
}

pub(crate) unsafe fn cuLibraryUnload(library: CUlibrary) -> CUresult {
    cuModuleUnload(CUmodule(library.0 as *mut _))
}

pub(crate) unsafe fn cuLibraryGetModule(pmod: *mut CUmodule, library: CUlibrary) -> CUresult {
    out(pmod, CUmodule(library.0 as *mut _))
}

pub(crate) unsafe fn cuLibraryGetKernel(pkernel: *mut CUkernel, library: CUlibrary, name: *const c_char) -> CUresult {
    let mut f = CUfunction(std::ptr::null_mut());
    cuModuleGetFunction(&mut f, CUmodule(library.0 as *mut _), name)?;
    out(pkernel, f.0 as CUkernel)
}

pub(crate) unsafe fn cuKernelGetFunction(pfunc: *mut CUfunction, kernel: CUkernel) -> CUresult {
    out(pfunc, CUfunction(kernel as *mut _))
}

pub(crate) unsafe fn cuKernelGetAttribute(pi: *mut c_int, attrib: CUfunction_attribute, kernel: CUkernel, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    cuFuncGetAttribute(pi, attrib, CUfunction(kernel as *mut _))
}

pub(crate) unsafe fn cuFuncSetAttribute(hfunc: CUfunction, attrib: CUfunction_attribute, value: c_int) -> CUresult {
    with(|d| {
        d.functions.get(&(hfunc.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        // Dynamic threadgroup memory is up to the device's 64 KiB without opting in.
        if attrib == CUfunction_attribute_enum::CU_FUNC_ATTRIBUTE_MAX_DYNAMIC_SHARED_SIZE_BYTES && value > 65536 {
            return Err(CUerror::INVALID_VALUE);
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuFuncSetCacheConfig(hfunc: CUfunction, _config: CUfunc_cache) -> CUresult {
    with(|d| d.functions.get(&(hfunc.0 as usize)).map(|_| ()).ok_or(CUerror::INVALID_HANDLE))
}

pub(crate) unsafe fn cuModuleGetLoadingMode(mode: *mut CUmoduleLoadingMode) -> CUresult {
    out(mode, CUmoduleLoadingMode_enum::CU_MODULE_EAGER_LOADING)
}

pub(crate) unsafe fn cuCtxGetApiVersion(_ctx: CUcontext, version: *mut c_uint) -> CUresult {
    out(version, 12040)
}

pub(crate) unsafe fn cuCtxGetFlags(flags: *mut c_uint) -> CUresult {
    out(flags, 0)
}

pub(crate) unsafe fn cuCtxPushCurrent_v2(ctx: CUcontext) -> CUresult {
    let h = check_context(ctx)?.ok_or(CUerror::INVALID_VALUE)?;
    contexts::push(h);
    Ok(())
}

pub(crate) unsafe fn cuCtxPopCurrent_v2(pctx: *mut CUcontext) -> CUresult {
    with_contexts(|_| Ok(()))?;
    let h = contexts::pop().ok_or(CUerror::INVALID_CONTEXT)?;
    if pctx.is_null() {
        return Ok(());
    }
    out(pctx, CUcontext(h as *mut _))
}

pub(crate) unsafe fn cuDevicePrimaryCtxGetState(dev: CUdevice, flags: *mut c_uint, active: *mut c_int) -> CUresult {
    check_device(dev)?;
    out(flags, 0)?;
    out(active, with(|_| Ok(1)).unwrap_or(0))
}

pub(crate) unsafe fn cuDevicePrimaryCtxSetFlags_v2(dev: CUdevice, _flags: c_uint) -> CUresult {
    check_device(dev)
}

pub(crate) unsafe fn cuCtxGetStreamPriorityRange(least: *mut c_int, greatest: *mut c_int) -> CUresult {
    if !least.is_null() {
        *least = 0;
    }
    if !greatest.is_null() {
        *greatest = 0;
    }
    Ok(())
}

/// The stack per thread for `alloca` of a dynamic size, which zoc reserves in each kernel (`--stack-size`; recursive
/// functions get four times as much): CU_LIMIT_STACK_SIZE, for the modules compiled after it is set.
static STACK_SIZE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(DEFAULT_STACK_SIZE);
const DEFAULT_STACK_SIZE: usize = 1024;
const DEFAULT_HEAP_SIZE: usize = 8 << 20;

/// Stack size per thread for dynamic allocations (see STACK_SIZE), the printf buffer of the runtime library
/// (64 KiB per module) and the device heap of `malloc` (8 MiB).
/// The limits a context keeps: what cuCtxSetLimit set, else the default. The stack size applies to the modules
/// loaded later, the heap size until the first module that uses `malloc` is loaded; the printf buffer is the
/// module's.
pub(crate) unsafe fn cuCtxGetLimit(pvalue: *mut usize, limit: CUlimit) -> CUresult {
    let default = match limit {
        CUlimit_enum::CU_LIMIT_STACK_SIZE => DEFAULT_STACK_SIZE,
        CUlimit_enum::CU_LIMIT_PRINTF_FIFO_SIZE => 65536,
        CUlimit_enum::CU_LIMIT_MALLOC_HEAP_SIZE => DEFAULT_HEAP_SIZE,
        _ => return Err(CUerror::UNSUPPORTED_LIMIT),
    };
    let v = with(|d| Ok(d.limits.get(&limit.0).copied().unwrap_or(default)))?;
    out(pvalue, v)
}

pub(crate) unsafe fn cuCtxSetLimit(limit: CUlimit, value: usize) -> CUresult {
    match limit {
        CUlimit_enum::CU_LIMIT_STACK_SIZE | CUlimit_enum::CU_LIMIT_PRINTF_FIFO_SIZE | CUlimit_enum::CU_LIMIT_MALLOC_HEAP_SIZE => {
            if limit == CUlimit_enum::CU_LIMIT_MALLOC_HEAP_SIZE {
                // as in CUDA, the heap's size is fixed once a module that uses it is loaded
                let fixed = with(|d| Ok(d.heap.is_some()))?;
                if fixed {
                    return Err(CUerror::INVALID_VALUE);
                }
            }
            if limit == CUlimit_enum::CU_LIMIT_STACK_SIZE {
                // zoc takes a 32-bit size; private memory is far smaller
                if value > u32::MAX as usize / 4 {
                    return Err(CUerror::INVALID_VALUE);
                }
                STACK_SIZE.store(value.max(1), std::sync::atomic::Ordering::Relaxed);
            }
            with(|d| {
                d.limits.insert(limit.0, value);
                Ok(())
            })
        }
        _ => Err(CUerror::UNSUPPORTED_LIMIT),
    }
}

pub(crate) unsafe fn cuDeviceGetUuid_v2(uuid: *mut CUuuid, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    let chip = with(|d| Ok(d.ctx.device.params.chip_id))?;
    let mut bytes = *b"AGX-ZLUDA\0\0\0\0\0\0\0";
    bytes[12..16].copy_from_slice(&chip.to_le_bytes());
    out(uuid, CUuuid { bytes })
}

pub(crate) unsafe fn cuDeviceGetUuid(uuid: *mut CUuuid, dev: CUdevice) -> CUresult {
    cuDeviceGetUuid_v2(uuid, dev)
}

pub(crate) unsafe fn cuDeviceGetPCIBusId(bus: *mut c_char, len: c_int, dev: CUdevice) -> CUresult {
    check_device(dev)?;
    // An integrated GPU has no PCI address; report a fixed one so tools that print it work.
    let id = b"0000:00:00.0\0";
    if bus.is_null() || (len as usize) < id.len() {
        return Err(CUerror::INVALID_VALUE);
    }
    std::ptr::copy_nonoverlapping(id.as_ptr() as *const c_char, bus, id.len());
    Ok(())
}

pub(crate) unsafe fn cuStreamCreateWithPriority(ph: *mut CUstream, flags: c_uint, _priority: c_int) -> CUresult {
    cuStreamCreate(ph, flags)
}

pub(crate) unsafe fn cuStreamGetFlags(_s: CUstream, flags: *mut c_uint) -> CUresult {
    out(flags, 0)
}

pub(crate) unsafe fn cuStreamDestroy(s: CUstream) -> CUresult {
    cuStreamDestroy_v2(s)
}

pub(crate) unsafe fn cuStreamQuery(s: CUstream) -> CUresult {
    with(|d| {
        let q = d.queue(s.0 as usize)?.0;
        let last = q.last();
        if q.is_done(last).map_err(from_io)? {
            Ok(())
        } else {
            Err(CUerror::NOT_READY)
        }
    })
}

pub(crate) unsafe fn cuEventRecordWithFlags(e: CUevent, s: CUstream, _flags: c_uint) -> CUresult {
    cuEventRecord(e, s)
}

pub(crate) unsafe fn cuCtxSynchronize_v2(_ctx: CUcontext) -> CUresult {
    cuCtxSynchronize()
}

/// Launch attributes `cuLaunchKernelEx` takes: a cluster size equal to the kernel's compiled one (`.reqnctapercluster`;
/// a cluster runs as one threadgroup, which is safe because the threadgroup barrier does not wait for ended
/// SIMD-groups: ptx-re-blockers B28), the cluster scheduling preference (a hint), and cooperative launches.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct LaunchAttrs {
    cluster: Option<[u32; 3]>,
    cooperative: bool,
}

unsafe fn launch_attrs(c: &CUlaunchConfig) -> Result<LaunchAttrs, CUerror> {
    let mut out = LaunchAttrs::default();
    if c.numAttrs == 0 {
        return Ok(out);
    }
    let attrs = std::slice::from_raw_parts(c.attrs.as_ref().ok_or(CUerror::INVALID_VALUE)?, c.numAttrs as usize);
    for a in attrs {
        match a.id {
            CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_DIMENSION => {
                let d = a.value.clusterDim;
                out.cluster = Some([d.x, d.y, d.z]);
            }
            CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_CLUSTER_SCHEDULING_POLICY_PREFERENCE => {}
            CUlaunchAttributeID::CU_LAUNCH_ATTRIBUTE_COOPERATIVE => out.cooperative = a.value.cooperative != 0,
            _ => return Err(CUerror::NOT_SUPPORTED),
        }
    }
    Ok(out)
}

/// Whether `attrs` can be honoured for kernel `k`, launched as `grid` of `block` with `shared` dynamic bytes on a
/// device of `cores` cores.
fn check_launch_attrs(k: &module::KernelInfo, attrs: LaunchAttrs, grid: [u32; 3], block: [u32; 3], shared: u32, cores: u32) -> Result<(), CUerror> {
    if let Some(c) = attrs.cluster {
        if c.contains(&0) {
            return Err(CUerror::INVALID_VALUE);
        }
        // The cluster size is part of the code (the CTA rank within the cluster is computed from it).
        if c != k.cluster && c != [1, 1, 1] {
            return Err(if k.cluster == [1, 1, 1] { CUerror::NOT_SUPPORTED } else { CUerror::INVALID_CLUSTER_SIZE });
        }
    }
    if attrs.cooperative {
        let blocks: u64 = grid.iter().map(|&g| g as u64).product();
        let threads = block.iter().product::<u32>();
        if blocks > cooperative_limit(k, threads, shared as usize, cores) as u64 {
            return Err(CUerror::COOPERATIVE_LAUNCH_TOO_LARGE);
        }
    }
    Ok(())
}

/// Threadgroups of `k` that are resident together on the whole device (B5), which a cooperative launch may use.
fn cooperative_limit(k: &module::KernelInfo, threads: u32, dynamic_shared: usize, cores: u32) -> u32 {
    let per_core = co_resident_threadgroups(k.simd_groups_per_core() / threads.div_ceil(32).max(1));
    let shared = k.threadgroup_bytes as usize + dynamic_shared;
    let per_core = if shared > 0 { per_core.min((65536 / shared) as u32) } else { per_core };
    per_core * cores
}

pub(crate) unsafe fn cuLaunchKernelEx(config: *const CUlaunchConfig, f: CUfunction, params: *mut *mut c_void, extra: *mut *mut c_void) -> CUresult {
    let c = config.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let attrs = launch_attrs(c)?;
    let (grid, block) = ([c.gridDimX, c.gridDimY, c.gridDimZ], [c.blockDimX, c.blockDimY, c.blockDimZ]);
    with(|d| {
        let (m, name) = d.functions.get(&(f.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        check_launch_attrs(&d.modules[m].module.kernels[name], attrs, grid, block, c.sharedMemBytes, d.ctx.props.cores)
    })?;
    cuLaunchKernel(f, c.gridDimX, c.gridDimY, c.gridDimZ, c.blockDimX, c.blockDimY, c.blockDimZ, c.sharedMemBytes, c.hStream, params, extra)
}

pub(crate) unsafe fn cuLaunchCooperativeKernel(
    f: CUfunction,
    gx: c_uint,
    gy: c_uint,
    gz: c_uint,
    bx: c_uint,
    by: c_uint,
    bz: c_uint,
    shared: c_uint,
    stream: CUstream,
    params: *mut *mut c_void,
) -> CUresult {
    let attrs = LaunchAttrs { cooperative: true, ..Default::default() };
    with(|d| {
        let (m, name) = d.functions.get(&(f.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        check_launch_attrs(&d.modules[m].module.kernels[name], attrs, [gx, gy, gz], [bx, by, bz], shared, d.ctx.props.cores)
    })?;
    cuLaunchKernel(f, gx, gy, gz, bx, by, bz, shared, stream, params, std::ptr::null_mut())
}

/// Occupancy from the G14 register budget: up to 96 SIMD-groups per core, fewer when each thread needs more
/// registers (2 * floor(3328 / (2 * register halves)) SIMD-groups), and 64 KiB of threadgroup memory per core.
///
/// The count is the number of CTAs per core that are resident together, which persistent kernels and software grid
/// barriers rely on. Measured on the M2 Max (AGX Performance Simulator tasks/ptx-re-blockers.md, B5): a whole grid is
/// resident only up to min(15, floor(S / g) - 1) threadgroups per core, for S SIMD-groups per core and g per
/// threadgroup; above that, later threadgroups start only when earlier ones exit.
pub(crate) unsafe fn cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(
    n: *mut c_int,
    f: CUfunction,
    block: c_int,
    dynamic_shared: usize,
    _flags: c_uint,
) -> CUresult {
    let v = with(|d| {
        let (m, name) = d.functions.get(&(f.0 as usize)).ok_or(CUerror::INVALID_HANDLE)?;
        let k = &d.modules[m].module.kernels[name];
        if block <= 0 {
            return Err(CUerror::INVALID_VALUE);
        }
        let groups_per_block = (block as u32).div_ceil(32);
        let mut blocks = co_resident_threadgroups(k.simd_groups_per_core() / groups_per_block);
        let shared = k.threadgroup_bytes as usize + dynamic_shared;
        if shared > 0 {
            blocks = blocks.min((65536 / shared) as u32);
        }
        Ok(blocks as c_int)
    })?;
    out(n, v)
}

/// Threadgroups per core that are resident together, of `fit` that fit in the register budget (B5): one less, at most
/// 15. One threadgroup that fits always runs (B26).
fn co_resident_threadgroups(fit: u32) -> u32 {
    if fit <= 1 {
        fit
    } else {
        (fit - 1).min(15)
    }
}

pub(crate) unsafe fn cuOccupancyMaxActiveBlocksPerMultiprocessor(n: *mut c_int, f: CUfunction, block: c_int, dynamic_shared: usize) -> CUresult {
    cuOccupancyMaxActiveBlocksPerMultiprocessorWithFlags(n, f, block, dynamic_shared, 0)
}

// Copies are CPU copies of shared memory after the stream's earlier work, so the asynchronous forms are synchronous.
pub(crate) unsafe fn cuMemcpy(dst: CUdeviceptr, src: CUdeviceptr, n: usize) -> CUresult {
    let (dd, sd) = with(|d| Ok((d.ctx.address_range(dst.0 as u64).is_some(), d.ctx.address_range(src.0 as u64).is_some())))?;
    match (dd, sd) {
        (true, true) => cuMemcpyDtoD_v2(dst, src, n),
        (true, false) => cuMemcpyHtoD_v2(dst, src.0, n),
        (false, true) => cuMemcpyDtoH_v2(dst.0, src, n),
        (false, false) => {
            with(|d| d.sync_all())?;
            std::ptr::copy(src.0 as *const u8, dst.0 as *mut u8, n);
            Ok(())
        }
    }
}

pub(crate) unsafe fn cuMemcpyAsync(dst: CUdeviceptr, src: CUdeviceptr, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Copy { dst: dst.0 as u64, src: src.0 as u64, n })))? {
        return Ok(());
    }
    cuMemcpy(dst, src, n)
}

pub(crate) unsafe fn cuMemcpyHtoDAsync_v2(dst: CUdeviceptr, src: *const c_void, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Copy { dst: dst.0 as u64, src: src as u64, n })))? {
        return Ok(());
    }
    cuMemcpyHtoD_v2(dst, src, n)
}

pub(crate) unsafe fn cuMemcpyDtoHAsync_v2(dst: *mut c_void, src: CUdeviceptr, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Copy { dst: dst as u64, src: src.0 as u64, n })))? {
        return Ok(());
    }
    cuMemcpyDtoH_v2(dst, src, n)
}

pub(crate) unsafe fn cuMemcpyDtoDAsync_v2(dst: CUdeviceptr, src: CUdeviceptr, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Copy { dst: dst.0 as u64, src: src.0 as u64, n })))? {
        return Ok(());
    }
    cuMemcpyDtoD_v2(dst, src, n)
}

pub(crate) unsafe fn cuMemsetD8Async(dst: CUdeviceptr, uc: u8, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Memset { dst: dst.0 as u64, pattern: vec![uc], count: n as u64 })))? {
        return Ok(());
    }
    cuMemsetD8_v2(dst, uc, n)
}

pub(crate) unsafe fn cuMemsetD16_v2(dst: CUdeviceptr, us: u16, n: usize) -> CUresult {
    with(|d| {
        d.sync_all()?;
        d.ctx.memset(dst.0 as u64, &us.to_le_bytes(), n as u64).map_err(from_io)
    })
}

pub(crate) unsafe fn cuMemsetD16Async(dst: CUdeviceptr, us: u16, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Memset { dst: dst.0 as u64, pattern: us.to_le_bytes().to_vec(), count: n as u64 })))? {
        return Ok(());
    }
    cuMemsetD16_v2(dst, us, n)
}

pub(crate) unsafe fn cuMemsetD32Async(dst: CUdeviceptr, ui: c_uint, n: usize, s: CUstream) -> CUresult {
    if with(|d| Ok(d.captured(s, || Op::Memset { dst: dst.0 as u64, pattern: ui.to_le_bytes().to_vec(), count: n as u64 })))? {
        return Ok(());
    }
    cuMemsetD32_v2(dst, ui, n)
}

/// Page-locked host memory is a GPU allocation mapped at its GPU address, so the host pointer is also the device
/// pointer (the CPU and GPU share memory). If that CPU range is taken, it is ordinary host memory that the GPU cannot
/// address (cuMemHostGetDevicePointer fails); copies work either way.
pub(crate) unsafe fn cuMemAllocHost_v2(pp: *mut *mut c_void, bytes: usize) -> CUresult {
    cuMemHostAlloc(pp, bytes, 0)
}

pub(crate) unsafe fn cuMemHostAlloc(pp: *mut *mut c_void, bytes: usize, flags: c_uint) -> CUresult {
    let unified = with(|d| {
        Ok(d.ctx.mem_alloc_unified(bytes as u64).ok().inspect(|&va| {
            d.host_allocs.insert(va, flags);
        }))
    })?;
    if let Some(va) = unified {
        return out(pp, va as *mut c_void);
    }
    let mut p = std::ptr::null_mut();
    if libc_posix_memalign(&mut p, 4096, bytes.max(1)) != 0 {
        return Err(CUerror::OUT_OF_MEMORY);
    }
    out(pp, p)
}

pub(crate) unsafe fn cuMemFreeHost(p: *mut c_void) -> CUresult {
    let gpu = with(|d| {
        if d.host_allocs.remove(&(p as u64)).is_none() {
            return Ok(false);
        }
        d.sync_all()?;
        d.ctx.mem_free(p as u64).map_err(from_io)?;
        Ok(true)
    })?;
    if !gpu {
        libc_free(p);
    }
    Ok(())
}

pub(crate) unsafe fn cuMemHostGetDevicePointer_v2(pdptr: *mut CUdeviceptr, p: *mut c_void, flags: c_uint) -> CUresult {
    if flags != 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let ok = with(|d| Ok(d.ctx.is_unified(p as u64)))?;
    if !ok {
        return Err(CUerror::INVALID_VALUE);
    }
    out(pdptr, CUdeviceptr_v2(p))
}

pub(crate) unsafe fn cuMemHostGetFlags(pflags: *mut c_uint, p: *mut c_void) -> CUresult {
    let f = with(|d| d.host_allocs.get(&(p as u64)).copied().ok_or(CUerror::INVALID_VALUE))?;
    out(pflags, f)
}

/// Managed memory is a GPU allocation mapped at its GPU address. The CPU must not touch it while kernels that use
/// it run (CU_DEVICE_ATTRIBUTE_CONCURRENT_MANAGED_ACCESS is 0, as on GPUs before Pascal); synchronize first.
pub(crate) unsafe fn cuMemAllocManaged(dptr: *mut CUdeviceptr, bytesize: usize, _flags: c_uint) -> CUresult {
    let va = with(|d| {
        let va = d.ctx.mem_alloc_unified(bytesize as u64).map_err(|e| {
            if e.kind() == io::ErrorKind::Unsupported { CUerror::NOT_SUPPORTED } else { from_io(e) }
        })?;
        d.managed.insert(va);
        Ok(va)
    })?;
    out(dptr, CUdeviceptr_v2(va as *mut c_void))
}

pub(crate) unsafe fn cuMemGetAddressRange_v2(pbase: *mut CUdeviceptr, psize: *mut usize, dptr: CUdeviceptr) -> CUresult {
    let (base, size) = with(|d| d.ctx.address_range(dptr.0 as u64).ok_or(CUerror::NOT_FOUND))?;
    if !pbase.is_null() {
        out(pbase, CUdeviceptr_v2(base as *mut c_void))?;
    }
    if !psize.is_null() {
        out(psize, size as usize)?;
    }
    Ok(())
}

/// Rows are aligned to 128 bytes (the texture row alignment G13/G14 drivers use).
pub(crate) unsafe fn cuMemAllocPitch_v2(
    dptr: *mut CUdeviceptr,
    pitch: *mut usize,
    width: usize,
    height: usize,
    element: c_uint,
) -> CUresult {
    if !matches!(element, 4 | 8 | 16) || width == 0 || height == 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    let p = width.next_multiple_of(128);
    cuMemAlloc_v2(dptr, p * height)?;
    out(pitch, p)
}

/// The CPU address of one side of a 2D copy, after checking that its whole extent is in one allocation.
unsafe fn copy_side(
    d: &mut Driver,
    kind: CUmemorytype,
    host: *const c_void,
    dev: CUdeviceptr,
    x: usize,
    y: usize,
    pitch: usize,
    width: usize,
    height: usize,
) -> Result<*mut u8, CUerror> {
    use CUmemorytype_enum as T;
    let device = match kind {
        T::CU_MEMORYTYPE_HOST => false,
        T::CU_MEMORYTYPE_DEVICE => true,
        // a unified pointer is in an allocation or is host memory
        T::CU_MEMORYTYPE_UNIFIED => d.ctx.address_range(dev.0 as u64).is_some(),
        _ => return Err(CUerror::NOT_SUPPORTED),
    };
    let start = y * pitch + x;
    let extent = if height == 0 { 0 } else { (height - 1) * pitch + width };
    if !device {
        let base = if kind == T::CU_MEMORYTYPE_UNIFIED { dev.0 as *const u8 } else { host as *const u8 };
        return Ok(base.add(start) as *mut u8);
    }
    d.ctx.cpu_ptr(dev.0 as u64 + start as u64, extent as u64).map_err(|_| CUerror::INVALID_VALUE)
}

unsafe fn memcpy_2d(c: *const CUDA_MEMCPY2D) -> CUresult {
    let c = c.as_ref().ok_or(CUerror::INVALID_VALUE)?;
    let (w, h) = (c.WidthInBytes, c.Height);
    let array = CUmemorytype::CU_MEMORYTYPE_ARRAY;
    if h > 1 && ((c.srcMemoryType != array && c.srcPitch < w) || (c.dstMemoryType != array && c.dstPitch < w)) {
        return Err(CUerror::INVALID_VALUE);
    }
    with(|d| {
        d.sync_all()?;
        // an array's rows are dense, at its own pitch
        let (src, src_pitch) = if c.srcMemoryType == array {
            array_side(d, c.srcArray, c.srcXInBytes, c.srcY, 0)?
        } else {
            (copy_side(d, c.srcMemoryType, c.srcHost, c.srcDevice, c.srcXInBytes, c.srcY, c.srcPitch, w, h)?, c.srcPitch)
        };
        let (dst, dst_pitch) = if c.dstMemoryType == array {
            array_side(d, c.dstArray, c.dstXInBytes, c.dstY, 0)?
        } else {
            (copy_side(d, c.dstMemoryType, c.dstHost, c.dstDevice, c.dstXInBytes, c.dstY, c.dstPitch, w, h)?, c.dstPitch)
        };
        for row in 0..h {
            std::ptr::copy(src.add(row * src_pitch), dst.add(row * dst_pitch), w);
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuMemcpy2D_v2(c: *const CUDA_MEMCPY2D) -> CUresult {
    memcpy_2d(c)
}

pub(crate) unsafe fn cuMemcpy2DUnaligned_v2(c: *const CUDA_MEMCPY2D) -> CUresult {
    memcpy_2d(c)
}

// Copies are CPU copies after the earlier work, so the asynchronous forms are synchronous.
pub(crate) unsafe fn cuMemcpy2DAsync_v2(c: *const CUDA_MEMCPY2D, _s: CUstream) -> CUresult {
    memcpy_2d(c)
}

unsafe fn memset_2d(dst: CUdeviceptr, pitch: usize, pattern: &[u8], width: usize, height: usize) -> CUresult {
    let row = width * pattern.len();
    if height > 1 && pitch < row {
        return Err(CUerror::INVALID_VALUE);
    }
    with(|d| {
        for y in 0..height {
            d.ctx.memset(dst.0 as u64 + (y * pitch) as u64, pattern, width as u64).map_err(from_io)?;
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuMemsetD2D8_v2(dst: CUdeviceptr, pitch: usize, v: u8, w: usize, h: usize) -> CUresult {
    memset_2d(dst, pitch, &[v], w, h)
}

pub(crate) unsafe fn cuMemsetD2D16_v2(dst: CUdeviceptr, pitch: usize, v: u16, w: usize, h: usize) -> CUresult {
    memset_2d(dst, pitch, &v.to_le_bytes(), w, h)
}

pub(crate) unsafe fn cuMemsetD2D32_v2(dst: CUdeviceptr, pitch: usize, v: c_uint, w: usize, h: usize) -> CUresult {
    memset_2d(dst, pitch, &v.to_le_bytes(), w, h)
}

pub(crate) unsafe fn cuMemsetD2D8Async(dst: CUdeviceptr, pitch: usize, v: u8, w: usize, h: usize, _s: CUstream) -> CUresult {
    memset_2d(dst, pitch, &[v], w, h)
}

pub(crate) unsafe fn cuMemsetD2D16Async(dst: CUdeviceptr, pitch: usize, v: u16, w: usize, h: usize, _s: CUstream) -> CUresult {
    memset_2d(dst, pitch, &v.to_le_bytes(), w, h)
}

pub(crate) unsafe fn cuMemsetD2D32Async(dst: CUdeviceptr, pitch: usize, v: c_uint, w: usize, h: usize, _s: CUstream) -> CUresult {
    memset_2d(dst, pitch, &v.to_le_bytes(), w, h)
}

/// Stream-ordered allocation: the memory is allocated at once; a free waits for all work first.
pub(crate) unsafe fn cuMemAllocAsync(dptr: *mut CUdeviceptr, bytesize: usize, _s: CUstream) -> CUresult {
    cuMemAlloc_v2(dptr, bytesize)
}

pub(crate) unsafe fn cuMemFreeAsync(dptr: CUdeviceptr, _s: CUstream) -> CUresult {
    cuMemFree_v2(dptr)
}

/// The CPU and GPU share memory: there is nothing to migrate.
pub(crate) unsafe fn cuMemPrefetchAsync(p: CUdeviceptr, count: usize, _dev: CUdevice, _s: CUstream) -> CUresult {
    with(|d| if d.managed.contains(&(p.0 as u64)) || d.ctx.address_range(p.0 as u64).is_some() || count == 0 { Ok(()) } else { Err(CUerror::INVALID_VALUE) })
}

pub(crate) unsafe fn cuMemAdvise(_p: CUdeviceptr, _count: usize, _advice: CUmem_advise, _dev: CUdevice) -> CUresult {
    Ok(())
}

// One device: peer copies are device copies.
pub(crate) unsafe fn cuMemcpyPeer(dst: CUdeviceptr, _dc: CUcontext, src: CUdeviceptr, _sc: CUcontext, n: usize) -> CUresult {
    cuMemcpyDtoD_v2(dst, src, n)
}

pub(crate) unsafe fn cuMemcpyPeerAsync(
    dst: CUdeviceptr,
    _dc: CUcontext,
    src: CUdeviceptr,
    _sc: CUcontext,
    n: usize,
    s: CUstream,
) -> CUresult {
    cuMemcpyDtoDAsync_v2(dst, src, n, s)
}

extern "C" {
    #[link_name = "posix_memalign"]
    fn libc_posix_memalign(p: *mut *mut c_void, align: usize, size: usize) -> c_int;
    #[link_name = "free"]
    fn libc_free(p: *mut c_void);
}

pub(crate) unsafe fn cuPointerGetAttribute(data: *mut c_void, attribute: CUpointer_attribute, ptr: CUdeviceptr) -> CUresult {
    let (device, host, managed, unified) = with(|d| {
        let base = d.ctx.address_range(ptr.0 as u64).map(|r| r.0);
        Ok((
            base.is_some(),
            base.is_some_and(|b| d.host_allocs.contains_key(&b)),
            base.is_some_and(|b| d.managed.contains(&b)),
            d.ctx.is_unified(ptr.0 as u64),
        ))
    })?;
    use CUpointer_attribute_enum as P;
    match attribute {
        P::CU_POINTER_ATTRIBUTE_MEMORY_TYPE => {
            if !device {
                return Err(CUerror::INVALID_VALUE);
            }
            let t = if host { CUmemorytype_enum::CU_MEMORYTYPE_HOST } else { CUmemorytype_enum::CU_MEMORYTYPE_DEVICE };
            out(data as *mut CUmemorytype, t)
        }
        P::CU_POINTER_ATTRIBUTE_IS_MANAGED if device => out(data as *mut c_uint, managed as c_uint),
        P::CU_POINTER_ATTRIBUTE_HOST_POINTER if unified => out(data as *mut *mut c_void, ptr.0),
        P::CU_POINTER_ATTRIBUTE_DEVICE_POINTER if device => out(data as *mut CUdeviceptr, ptr),
        P::CU_POINTER_ATTRIBUTE_CONTEXT if device => {
            // the current context, whose allocation it is (each context has its own address space)
            let h = with_contexts(|c| Ok(c.current()))?;
            out(data as *mut CUcontext, CUcontext(h as *mut _))
        }
        _ => Err(CUerror::INVALID_VALUE),
    }
}

// ---- CUDA graphs, built by stream capture and replayed in order on the launch stream

pub(crate) unsafe fn cuStreamBeginCapture_v2(s: CUstream, _mode: CUstreamCaptureMode) -> CUresult {
    with(|d| {
        let k = stream_key(s);
        if d.capturing.contains_key(&k) {
            return Err(CUerror::ILLEGAL_STATE);
        }
        d.capturing.insert(k, Vec::new());
        Ok(())
    })
}

pub(crate) unsafe fn cuStreamEndCapture(s: CUstream, graph: *mut CUgraph) -> CUresult {
    let h = with(|d| {
        let ops = d.capturing.remove(&stream_key(s)).ok_or(CUerror::ILLEGAL_STATE)?;
        let h = d.handle();
        d.graphs.insert(h, ops);
        Ok(h)
    })?;
    out(graph, h as CUgraph)
}

pub(crate) unsafe fn cuStreamIsCapturing(s: CUstream, status: *mut CUstreamCaptureStatus) -> CUresult {
    let on = with(|d| Ok(d.capturing.contains_key(&stream_key(s))))?;
    out(
        status,
        if on { CUstreamCaptureStatus_enum::CU_STREAM_CAPTURE_STATUS_ACTIVE } else { CUstreamCaptureStatus_enum::CU_STREAM_CAPTURE_STATUS_NONE },
    )
}

pub(crate) unsafe fn cuThreadExchangeStreamCaptureMode(_mode: *mut CUstreamCaptureMode) -> CUresult {
    Ok(())
}

pub(crate) unsafe fn cuGraphInstantiateWithFlags(exec: *mut CUgraphExec, graph: CUgraph, _flags: std::ffi::c_ulonglong) -> CUresult {
    let h = with(|d| {
        let ops = d.graphs.get(&(graph as usize)).cloned().ok_or(CUerror::INVALID_HANDLE)?;
        let h = d.handle();
        d.execs.insert(h, ops);
        Ok(h)
    })?;
    out(exec, h as CUgraphExec)
}

pub(crate) unsafe fn cuGraphInstantiate_v2(
    exec: *mut CUgraphExec,
    graph: CUgraph,
    _error_node: *mut CUgraphNode,
    _log: *mut c_char,
    _log_size: usize,
) -> CUresult {
    cuGraphInstantiateWithFlags(exec, graph, 0)
}

pub(crate) unsafe fn cuGraphLaunch(exec: CUgraphExec, s: CUstream) -> CUresult {
    with(|d| {
        let ops = d.execs.get(&(exec as usize)).cloned().ok_or(CUerror::INVALID_HANDLE)?;
        for op in &ops {
            d.run(op, s.0 as usize)?;
        }
        Ok(())
    })
}

pub(crate) unsafe fn cuGraphExecDestroy(exec: CUgraphExec) -> CUresult {
    with(|d| d.execs.remove(&(exec as usize)).map(|_| ()).ok_or(CUerror::INVALID_HANDLE))
}

pub(crate) unsafe fn cuGraphDestroy(graph: CUgraph) -> CUresult {
    with(|d| d.graphs.remove(&(graph as usize)).map(|_| ()).ok_or(CUerror::INVALID_HANDLE))
}

fn error_name(code: u32) -> &'static CStr {
    match code {
        0 => c"CUDA_SUCCESS",
        1 => c"CUDA_ERROR_INVALID_VALUE",
        2 => c"CUDA_ERROR_OUT_OF_MEMORY",
        3 => c"CUDA_ERROR_NOT_INITIALIZED",
        100 => c"CUDA_ERROR_NO_DEVICE",
        101 => c"CUDA_ERROR_INVALID_DEVICE",
        200 => c"CUDA_ERROR_INVALID_IMAGE",
        201 => c"CUDA_ERROR_INVALID_CONTEXT",
        218 => c"CUDA_ERROR_INVALID_PTX",
        400 => c"CUDA_ERROR_INVALID_HANDLE",
        500 => c"CUDA_ERROR_NOT_FOUND",
        600 => c"CUDA_ERROR_NOT_READY",
        700 => c"CUDA_ERROR_ILLEGAL_ADDRESS",
        702 => c"CUDA_ERROR_LAUNCH_TIMEOUT",
        710 => c"CUDA_ERROR_ASSERT",
        719 => c"CUDA_ERROR_LAUNCH_FAILED",
        801 => c"CUDA_ERROR_NOT_SUPPORTED",
        _ => c"CUDA_ERROR_UNKNOWN",
    }
}

fn code(e: CUresult) -> u32 {
    match e {
        Ok(()) => 0,
        Err(e) => e.0.get(),
    }
}

pub(crate) unsafe fn cuGetErrorName(e: CUresult, p: *mut *const c_char) -> CUresult {
    out(p, error_name(code(e)).as_ptr())
}

pub(crate) unsafe fn cuGetErrorString(e: CUresult, p: *mut *const c_char) -> CUresult {
    out(p, error_name(code(e)).as_ptr())
}

// ---------------------------------------------------------------- tensor maps

/// The 16 64-bit words of a tensor map, in the encoding the AGX emulation of
/// cp.async.bulk.tensor reads (PTX-formalization's, see the TMA helpers of
/// zluda_ptx_impl_agx.ll): 0 magic | 1 element type (tensormap.replace
/// numbering; im2col lower corners at bit 8) | 2 rank (im2col upper corners at
/// bit 8) | 3 global address | 4..8 dims | 9..12 byte strides of dims 1..4 |
/// 13 box dims 0..3 | 14 box dim 4, swizzle_mode << 16, fill << 24,
/// interleave << 32, im2col << 40, swizzle_atomicity << 48 | 15 element strides.
struct TensorMapDesc<'a> {
    data_type: CUtensorMapDataType,
    rank: u32,
    address: *mut c_void,
    dims: &'a [u64],
    strides: &'a [u64],
    boxes: &'a [u32],
    element_strides: &'a [u32],
    interleave: CUtensorMapInterleave,
    swizzle: CUtensorMapSwizzle,
    fill: CUtensorMapFloatOOBfill,
    // lower and upper corners of the spatial dims
    im2col: Option<(&'a [c_int], &'a [c_int])>,
}

const TENSOR_MAP_MAGIC: u64 = 0x5448_4150;

fn encode_tensor_map(d: &TensorMapDesc) -> Result<[u64; 16], CUerror> {
    // CUtensorMapDataType -> tensormap.replace.elemtype
    let element_type = match d.data_type.0 {
        t @ 0..=7 => t as u64,
        8 => 9,  // FLOAT64
        9 => 10, // BFLOAT16
        10 => 8, // FLOAT32_FTZ
        t @ 11..=15 => t as u64,
        _ => return Err(CUerror::INVALID_VALUE),
    };
    let rank = d.rank as usize;
    let address = d.address as u64;
    if !(1..=5).contains(&rank) || address % 16 != 0 || d.swizzle.0 > 6 || d.interleave.0 > 2 || d.fill.0 > 1 {
        return Err(CUerror::INVALID_VALUE);
    }
    if d.dims.iter().any(|&n| n == 0 || n > 1 << 32)
        || d.strides.iter().any(|&s| s % 16 != 0 || s >= 1 << 40)
        || d.boxes.iter().any(|&b| b == 0 || b > 256)
        || d.element_strides.iter().any(|&e| e == 0 || e > 8)
    {
        return Err(CUerror::INVALID_VALUE);
    }
    let z = d.swizzle.0 as u64;
    let (mode, atomicity) = if (4..=6).contains(&z) { (3, z - 3) } else { (z, 0) };
    let mut w = [0u64; 16];
    w[0] = TENSOR_MAP_MAGIC;
    w[1] = element_type;
    w[2] = rank as u64;
    w[3] = address;
    w[4..4 + rank].copy_from_slice(d.dims);
    w[9..9 + rank - 1].copy_from_slice(d.strides);
    for (i, &b) in d.boxes.iter().enumerate() {
        if i < 4 {
            w[13] |= (b as u64) << (16 * i);
        } else {
            w[14] |= b as u64;
        }
    }
    w[14] |= mode << 16 | (d.fill.0 as u64) << 24 | (d.interleave.0 as u64) << 32 | atomicity << 48;
    for (i, &e) in d.element_strides.iter().enumerate() {
        w[15] |= (e as u64) << (8 * i);
    }
    if let Some((lower, upper)) = d.im2col {
        let pack = |corners: &[c_int]| -> u64 {
            corners
                .iter()
                .enumerate()
                .map(|(i, &c)| (((c + 32768) as u64) & 0xffff) << (16 * i))
                .sum()
        };
        w[1] |= pack(lower) << 8;
        w[2] |= pack(upper) << 8;
        w[14] |= 1 << 40;
    }
    Ok(w)
}

unsafe fn slice<'a, T>(p: *const T, n: usize) -> Result<&'a [T], CUerror> {
    if p.is_null() {
        return Err(CUerror::INVALID_VALUE);
    }
    Ok(std::slice::from_raw_parts(p, n))
}

pub(crate) unsafe fn cuTensorMapEncodeTiled(
    tensor_map: *mut CUtensorMap,
    data_type: CUtensorMapDataType,
    rank: cuuint32_t,
    address: *mut c_void,
    dims: *const cuuint64_t,
    strides: *const cuuint64_t,
    boxes: *const cuuint32_t,
    element_strides: *const cuuint32_t,
    interleave: CUtensorMapInterleave,
    swizzle: CUtensorMapSwizzle,
    _l2_promotion: CUtensorMapL2promotion,
    fill: CUtensorMapFloatOOBfill,
) -> CUresult {
    if tensor_map.is_null() || !(1..=5).contains(&rank) {
        return Err(CUerror::INVALID_VALUE);
    }
    let n = rank as usize;
    let words = encode_tensor_map(&TensorMapDesc {
        data_type,
        rank,
        address,
        dims: slice(dims, n)?,
        strides: if n > 1 { slice(strides, n - 1)? } else { &[] },
        boxes: slice(boxes, n)?,
        element_strides: slice(element_strides, n)?,
        interleave,
        swizzle,
        fill,
        im2col: None,
    })?;
    (*tensor_map).opaque = words;
    Ok(())
}

pub(crate) unsafe fn cuTensorMapEncodeIm2col(
    tensor_map: *mut CUtensorMap,
    data_type: CUtensorMapDataType,
    rank: cuuint32_t,
    address: *mut c_void,
    dims: *const cuuint64_t,
    strides: *const cuuint64_t,
    lower: *const c_int,
    upper: *const c_int,
    channels_per_pixel: cuuint32_t,
    pixels_per_column: cuuint32_t,
    element_strides: *const cuuint32_t,
    interleave: CUtensorMapInterleave,
    swizzle: CUtensorMapSwizzle,
    _l2_promotion: CUtensorMapL2promotion,
    fill: CUtensorMapFloatOOBfill,
) -> CUresult {
    if tensor_map.is_null() || !(3..=5).contains(&rank) {
        return Err(CUerror::INVALID_VALUE);
    }
    let n = rank as usize;
    let words = encode_tensor_map(&TensorMapDesc {
        data_type,
        rank,
        address,
        dims: slice(dims, n)?,
        strides: slice(strides, n - 1)?,
        boxes: &[channels_per_pixel, pixels_per_column],
        element_strides: slice(element_strides, n)?,
        interleave,
        swizzle,
        fill,
        im2col: Some((slice(lower, n - 2)?, slice(upper, n - 2)?)),
    })?;
    (*tensor_map).opaque = words;
    Ok(())
}

pub(crate) unsafe fn cuTensorMapReplaceAddress(tensor_map: *mut CUtensorMap, address: *mut c_void) -> CUresult {
    if tensor_map.is_null() || (*tensor_map).opaque[0] != TENSOR_MAP_MAGIC || address as u64 % 16 != 0 {
        return Err(CUerror::INVALID_VALUE);
    }
    (*tensor_map).opaque[3] = address as u64;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(syncobj: u32, point: u64) -> LaunchRecord {
        LaunchRecord {
            fence: Fence { syncobj, point },
            stream: 0,
            module: 1,
            kernel: "saxpy".into(),
            grid: [4, 1, 1],
            block: [256, 1, 1],
            shared: 0,
            params: vec![0x1100_0000_0040u64.to_le_bytes().to_vec(), 7u32.to_le_bytes().to_vec(), vec![1, 2, 3]],
        }
    }

    #[test]
    fn launch_records_describe_the_task() {
        assert_eq!(
            record(5, 9).describe(),
            "kernel saxpy grid (4, 1, 1) block (256, 1, 1) shared 0 bytes, stream 0, command 9, arguments \
             [0x110000000040, 0x7, 010203]"
        );
    }

    #[test]
    fn finished_launches_are_dropped_per_queue() {
        let mut l: std::collections::VecDeque<_> = [record(5, 1), record(5, 2), record(6, 1), record(5, 3)].into();
        drop_finished(&mut l, 5, 2);
        let left: Vec<_> = l.iter().map(|r| (r.fence.syncobj, r.fence.point)).collect();
        assert_eq!(left, vec![(6, 1), (5, 3)]);
    }

    // A dump of the checked kernel of agx_runtime/tests/data/checked.ll, replayed in agx_run when $AGX_RUN names it:
    // out[5] = n is out of bounds of a 16-byte buffer.
    #[test]
    fn a_dumped_launch_replays_in_agx_run() {
        let m = module::Module::load_object(include_bytes!("../../agx_runtime/tests/data/checked.o")).unwrap();
        let (out, q) = (0x100_0000u64, 0x200_0000u64);
        let mut pair = [0u8; 16];
        pair[..4].copy_from_slice(&9u32.to_le_bytes());
        pair[8..].copy_from_slice(&q.to_le_bytes());
        let (o, i) = (out.to_le_bytes(), 5u32.to_le_bytes());
        let params: [&[u8]; 3] = [&o, &i, &pair];
        let record = [0u8; 64];
        let r = agx_runtime::replay(
            &m,
            Some((0x800_0000, &m.globals)),
            Some((0x700_0000, &record)),
            "put",
            &params,
            [1, 1, 1],
            [8, 1, 1],
            0,
            &[(out, out + 16), (q, q + 64)],
            &[],
            0x7e_0000_0000,
            0x7f_0000_0000,
        )
        .unwrap();
        let mut regions = r.regions.clone();
        regions.extend([(out, vec![0u8; 16]), (q, vec![0u8; 64])]);
        let dir = tempfile::tempdir().unwrap();
        write_dump(dir.path(), &r, &regions, "a test\nsecond line").unwrap();
        let sh = std::fs::read_to_string(dir.path().join("replay.sh")).unwrap();
        assert!(sh.contains("# a test\n# second line\n") && sh.contains("--grid 8,1,1 --wg 8,1,1"), "{sh}");
        // The object has a line table: a fault in the replay names its source line.
        let lines = format!("--lines module.o --lines-offset {}", m.kernels["put"].offset);
        assert!(sh.contains(&lines) && dir.path().join("module.o").exists(), "{sh}");
        let list = std::fs::read_to_string(dir.path().join("regions.txt")).unwrap();
        let rec = list.lines().position(|l| l.contains(" 0x7000000 64")).expect(&list);
        let Some(agx_run) = std::env::var_os("AGX_RUN") else { return };
        let o = std::process::Command::new(dir.path().join("replay.sh")).env("AGX_RUN", agx_run).output().unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        let rec = std::fs::read(dir.path().join(format!("m{rec}.out"))).unwrap();
        let (off, size) = m.global_symbols[module::CHECKED_SITES_SYMBOL];
        let t = &m.globals[off as usize..(off + size) as usize];
        let e = module::CheckedError::decode(&rec, std::str::from_utf8(&t[..t.len() - 1]).unwrap()).unwrap();
        assert_eq!((e.kind, e.store, e.address, e.site_text.as_str()), (1, true, out + 20, "store 4\tput put.cu:4:3"));
    }

    #[test]
    fn a_pointer_loaded_from_memory_is_checked_against_the_allocations() {
        let m = module::Module::load_object(include_bytes!("../../agx_runtime/tests/data/chase.o")).unwrap();
        let (data, ptrs, out) = (0x100_0000u64, 0x200_0000u64, 0x300_0000u64);
        // threads 0..6 read data[t]; thread 7 reads just past the end of data
        let table: Vec<u8> = (0..8u64).flat_map(|t| (data + 4 * t + if t == 7 { 4 } else { 0 }).to_le_bytes()).collect();
        let record = [0u8; 64];
        let (p, o) = (ptrs.to_le_bytes(), out.to_le_bytes());
        let allocations = [(data, data + 32), (ptrs, ptrs + 64), (out, out + 32)];
        let r = agx_runtime::replay(
            &m,
            Some((0x800_0000, &m.globals)),
            Some((0x700_0000, &record)),
            "chase",
            &[&p, &o],
            [1, 1, 1],
            [8, 1, 1],
            0,
            &[(ptrs, ptrs + 64), (out, out + 32)],
            &allocations,
            0x7e_0000_0000,
            0x7f_0000_0000,
        )
        .unwrap();
        assert!(r.regions.iter().any(|(va, b)| *va == 0x7e_0000_0000 && b.len() == 48), "the table is a region");
        let mut regions = r.regions.clone();
        regions.extend([(data, (100..108u32).flat_map(u32::to_le_bytes).collect()), (ptrs, table), (out, vec![0u8; 32])]);
        let dir = tempfile::tempdir().unwrap();
        write_dump(dir.path(), &r, &regions, "chase").unwrap();
        let Some(agx_run) = std::env::var_os("AGX_RUN") else { return };
        let o = std::process::Command::new(dir.path().join("replay.sh")).env("AGX_RUN", agx_run).output().unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stdout));
        let list = std::fs::read_to_string(dir.path().join("regions.txt")).unwrap();
        let at = |va: &str| list.lines().position(|l| l.contains(va)).expect(&list);
        let rec = std::fs::read(dir.path().join(format!("m{}.out", at(" 0x7000000 64")))).unwrap();
        let (off, size) = m.global_symbols[module::CHECKED_SITES_SYMBOL];
        let t = &m.globals[off as usize..(off + size) as usize];
        let e = module::CheckedError::decode(&rec, std::str::from_utf8(&t[..t.len() - 1]).unwrap()).unwrap();
        assert_eq!((e.kind, e.store, e.address, e.thread), (1, false, data + 32, [7, 0, 0]));
        let out_bytes = std::fs::read(dir.path().join(format!("m{}.out", at(" 0x3000000 32")))).unwrap();
        assert_eq!(out_bytes[..28], (100..107u32).flat_map(u32::to_le_bytes).collect::<Vec<u8>>()[..]);
    }

    #[test]
    fn launch_attributes_are_checked_against_the_kernel() {
        let m = module::Module::load_object(include_bytes!("../../agx_runtime/tests/data/checked.o")).unwrap();
        let plain = m.kernels["put"].clone();
        let clustered = module::KernelInfo { cluster: [2, 1, 1], ..plain.clone() };
        let none = LaunchAttrs::default();
        let cluster = |c| LaunchAttrs { cluster: Some(c), ..none };
        let coop = LaunchAttrs { cooperative: true, ..none };
        let (g, b) = ([4, 1, 1], [32, 1, 1]);
        assert_eq!(check_launch_attrs(&plain, none, g, b, 0, 38), Ok(()));
        // a cluster is the compiled one, or 1
        assert_eq!(check_launch_attrs(&clustered, cluster([2, 1, 1]), g, b, 0, 38), Ok(()));
        assert_eq!(check_launch_attrs(&plain, cluster([1, 1, 1]), g, b, 0, 38), Ok(()));
        assert_eq!(check_launch_attrs(&clustered, cluster([4, 1, 1]), g, b, 0, 38), Err(CUerror::INVALID_CLUSTER_SIZE));
        assert_eq!(check_launch_attrs(&plain, cluster([2, 1, 1]), g, b, 0, 38), Err(CUerror::NOT_SUPPORTED));
        assert_eq!(check_launch_attrs(&plain, cluster([0, 1, 1]), g, b, 0, 38), Err(CUerror::INVALID_VALUE));
        // cooperative: up to the co-resident threadgroups of the device
        let limit = cooperative_limit(&plain, 32, 0, 38);
        assert_eq!(limit, 38 * co_resident_threadgroups(plain.simd_groups_per_core()));
        assert_eq!(check_launch_attrs(&plain, coop, [limit, 1, 1], b, 0, 38), Ok(()));
        assert_eq!(check_launch_attrs(&plain, coop, [limit + 1, 1, 1], b, 0, 38), Err(CUerror::COOPERATIVE_LAUNCH_TOO_LARGE));
        // threadgroup memory lowers it: 32 KiB per block leaves 2 per core
        assert_eq!(cooperative_limit(&plain, 32, 32768, 38), 2 * 38);
    }

    #[test]
    fn a_check_site_names_its_kernel() {
        assert_eq!(site_kernel("store 4\toob"), "oob");
        assert_eq!(site_kernel("load 8\tget a.cu:3:5\tk a.cu:9:1"), "k");
        assert_eq!(site_kernel(""), "");
    }

    #[test]
    fn a_failed_assert_stays_until_the_context_is_destroyed() {
        let mut sticky = None;
        assert_eq!(sticky_after_sync(&mut sticky, false), Ok(()));
        assert_eq!(sticky_after_sync(&mut sticky, true), Err(CUerror::ASSERT));
        // the flag is cleared on the device once read, but the context keeps the error
        assert_eq!(sticky_after_sync(&mut sticky, false), Err(CUerror::ASSERT));
        assert_eq!(sticky, Some(CUerror::ASSERT));
    }

    #[test]
    fn co_resident_threadgroups_follow_the_measured_rule() {
        // B5 on the M2 Max (38 cores): 32-thread CTAs, 576 resident; 128-thread CTAs at 13 per core, 456 = 38 * 12
        assert_eq!(co_resident_threadgroups(96), 15);
        assert_eq!(co_resident_threadgroups(13), 12);
        assert_eq!(co_resident_threadgroups(1), 1);
        assert_eq!(co_resident_threadgroups(0), 0);
    }

    // Needs zoc built with the agx feature: target/debug/zoc, or ZLUDA_AGX_ZOC.
    #[test]
    fn compiles_ptx_into_a_launchable_module() {
        let zoc = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../target/debug/zoc");
        if std::env::var_os("ZLUDA_AGX_ZOC").is_none() {
            if !zoc.exists() {
                eprintln!("skipped: no zoc");
                return;
            }
            std::env::set_var("ZLUDA_AGX_ZOC", &zoc);
        }
        std::env::set_var("ZLUDA_CACHE_DISABLE", "1");
        let ptx = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../ptx/src/test/_Z9vectorAddPKfS0_Pfi.ptx")).unwrap();
        let m = load_image(&ptx, "agx-g14c").unwrap();
        let k = &m.kernels["_Z9vectorAddPKfS0_Pfi"];
        // three pointers and an int, all passed in the uniforms
        let bytes: Vec<u32> = k.args.iter().map(|a| a.bytes).collect();
        assert_eq!(bytes, [8, 8, 8, 4]);
        assert!(k.args.iter().all(|a| !a.by_pointer));
        let (u, block) = module::pack_params(
            k,
            &[&1u64.to_le_bytes(), &2u64.to_le_bytes(), &3u64.to_le_bytes(), &4u32.to_le_bytes()],
            [1, 1, 1],
            None,
            0,
        )
        .unwrap();
        assert!(block.is_empty());
        assert_eq!(&u[0..14], &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0]);
    }

    // A fat binary as nvcc lays it out: a wrapper, a fatbin header, then file headers with their payloads.
    #[test]
    fn picks_the_newest_ptx_of_a_fat_binary() {
        fn file(kind: u16, sm: u32, payload: &[u8]) -> Vec<u8> {
            let mut h = vec![0u8; 64];
            h[0..2].copy_from_slice(&kind.to_le_bytes());
            h[2..4].copy_from_slice(&0x101u16.to_le_bytes());
            h[4..8].copy_from_slice(&64u32.to_le_bytes());
            h[8..12].copy_from_slice(&(payload.len() as u32).to_le_bytes());
            h[28..32].copy_from_slice(&sm.to_le_bytes());
            h.extend_from_slice(payload);
            h
        }
        let mut files = file(1, 52, b"old\0\0\0\0\0");
        files.extend(file(2, 86, b"SASS....")); // machine code, ignored
        files.extend(file(1, 86, b"new\0\0\0\0\0"));
        let mut bin = vec![0u8; 16];
        bin[0..4].copy_from_slice(&0xba55ed50u32.to_le_bytes());
        bin[4..6].copy_from_slice(&1u16.to_le_bytes());
        bin[6..8].copy_from_slice(&16u16.to_le_bytes());
        bin[8..16].copy_from_slice(&(files.len() as u64).to_le_bytes());
        bin.extend(files);
        // 8-byte aligned storage
        let mut words = vec![0u64; bin.len().div_ceil(8)];
        unsafe { std::ptr::copy_nonoverlapping(bin.as_ptr(), words.as_mut_ptr() as *mut u8, bin.len()) };
        let header = words.as_ptr() as *const u8;
        assert_eq!(unsafe { ptx_in_fatbin(header) }.unwrap(), b"new\0");
        let wrapper = cuda_types::dark_api::FatbincWrapper {
            magic: 0x466243b1,
            version: 1,
            data: header as *const _,
            filename_or_fatbins: std::ptr::null(),
        };
        assert_eq!(unsafe { ptx_in_fatbin(&wrapper as *const _ as *const u8) }.unwrap(), b"new\0");
    }

    #[test]
    fn get_proc_address_resolves_versioned_names() {
        let mut f = std::ptr::null_mut();
        let mut st = CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SYMBOL_NOT_FOUND;
        unsafe { cuGetProcAddress_v2(c"cuMemAlloc".as_ptr(), &mut f, 12000, 0, &mut st) }.unwrap();
        assert_eq!(f as usize, crate::cuMemAlloc_v2 as *const () as usize);
        assert_eq!(st, CUdriverProcAddressQueryResult::CU_GET_PROC_ADDRESS_SUCCESS);
        assert!(unsafe { cuGetProcAddress_v2(c"cuNoSuchFunction".as_ptr(), &mut f, 12000, 0, &mut st) }.is_err());
        assert!(f.is_null());
    }

    #[test]
    fn error_names() {
        assert_eq!(error_name(code(Err(CUerror::ASSERT))).to_str().unwrap(), "CUDA_ERROR_ASSERT");
        assert_eq!(error_name(code(Ok(()))).to_str().unwrap(), "CUDA_SUCCESS");
    }
}

#[cfg(test)]
mod link_tests {
    /// Links `extern_func.ptx` (which calls an `.extern` function) with a module defining it. Needs zoc
    /// (`ZLUDA_AGX_ZOC`, or `target/<profile>/zoc`); skipped without it.
    #[test]
    fn links_an_extern_function() {
        let exe = std::env::current_exe().unwrap();
        let zoc = exe.parent().and_then(|d| d.parent()).map(|d| d.join("zoc"));
        match (std::env::var_os("ZLUDA_AGX_ZOC"), zoc) {
            (Some(_), _) => {}
            (None, Some(z)) if z.exists() => std::env::set_var("ZLUDA_AGX_ZOC", z),
            _ => return eprintln!("no zoc: skipped"),
        }
        let main = include_bytes!("../../ptx/src/test/spirv_run/extern_func.ptx");
        let lib = b".version 6.5
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
.visible .entry other(.param .u64 p)
{
    ret;
}
";
        let (object, asm) = super::compile_ptx_linked(&[main, lib], "agx-g14c").unwrap();
        let m = agx_runtime::module::Module::load(&object, &asm).unwrap();
        assert!(m.kernels.contains_key("extern_func") && m.kernels.contains_key("other"), "{:?}", m.kernels.keys());
        // alone, the call has no definition
        assert!(super::compile_ptx(main, "agx-g14c").is_err());
    }

    #[test]
    fn tensor_map_encoding() {
        use cuda_types::cuda::*;
        use std::ffi::c_void;
        let mut map = CUtensorMap { opaque: [0; 16] };
        let (dims, strides, boxes, estrides) = ([16u64, 8], [64u64], [8u32, 4], [1u32, 1]);
        unsafe {
            super::cuTensorMapEncodeTiled(
                &mut map,
                CUtensorMapDataType::CU_TENSOR_MAP_DATA_TYPE_FLOAT32,
                2,
                0x1000 as *mut c_void,
                dims.as_ptr(),
                strides.as_ptr(),
                boxes.as_ptr(),
                estrides.as_ptr(),
                CUtensorMapInterleave(0),
                CUtensorMapSwizzle::CU_TENSOR_MAP_SWIZZLE_128B_ATOM_32B,
                CUtensorMapL2promotion(0),
                CUtensorMapFloatOOBfill(1),
            )
            .unwrap();
        }
        // the words of PTX-formalization's `.tensormap .f32 ... .swizzle 260`
        let w = map.opaque;
        assert_eq!((w[0], w[1], w[2], w[3], w[4], w[5], w[9]), (0x54484150, 7, 2, 0x1000, 16, 8, 64));
        assert_eq!((w[13], w[14], w[15]), (8 | 4 << 16, 3 << 16 | 1 << 24 | 1 << 48, 0x0101));
        unsafe { super::cuTensorMapReplaceAddress(&mut map, 0x2000 as *mut c_void).unwrap() };
        assert_eq!(map.opaque[3], 0x2000);
    }

}
