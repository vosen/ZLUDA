//! Compute on Apple G14 GPUs (M2 family) through the Asahi DRM uAPI, without Mesa.
//!
//! This is the submission layer of ZLUDA's AGX backend: a device, its GPU address space, buffer objects, compute
//! queues, and the encoding of one compute dispatch (the USC pipeline words and the CDM control stream).
//!
//! The words follow the direct-DRM harness of the AGX Performance Simulator (`tests/drm_compute.c`), which was
//! verified on an M2 Max (G14C) under Asahi Linux. See Apple-M2-Playground `experiments/baseline/direct-drm-compute.md`
//! for the measured layout. The tests below pin those exact bytes. Fields that are not yet measured are listed in
//! the AGX Performance Simulator's `tasks/ptx-re-blockers.md`:
//! - scratch / stack memory (B1);
//! - the largest uniform preload (B9);
//! - partial workgroups.
//!
//! USC addresses (shader code, pipeline words) are 32-bit offsets from the queue's `usc_exec_base`.

pub mod error;
pub mod module;

use asahi_sys::*;
use std::ffi::CString;
use std::io;
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

pub const PAGE: u64 = 16384;

fn round_up(x: u64, a: u64) -> u64 {
    x.div_ceil(a) * a
}

/// First-fit allocator of address ranges. Freed ranges are merged with their neighbours.
#[derive(Debug)]
pub struct RangeAllocator {
    /// start -> length of each free range
    free: BTreeMap<u64, u64>,
}

impl RangeAllocator {
    pub fn new(start: u64, len: u64) -> RangeAllocator {
        let mut free = BTreeMap::new();
        if len != 0 {
            free.insert(start, len);
        }
        RangeAllocator { free }
    }

    pub fn alloc(&mut self, size: u64, align: u64) -> Option<u64> {
        let (start, len, at) = self.free.iter().find_map(|(&start, &len)| {
            let at = round_up(start, align);
            (at + size <= start + len).then_some((start, len, at))
        })?;
        self.free.remove(&start);
        if at > start {
            self.free.insert(start, at - start);
        }
        if at + size < start + len {
            self.free.insert(at + size, start + len - at - size);
        }
        Some(at)
    }

    pub fn free(&mut self, mut start: u64, mut len: u64) {
        if let Some((&prev, &prev_len)) = self.free.range(..start).next_back() {
            debug_assert!(prev + prev_len <= start, "double free of {start:#x}");
            if prev + prev_len == start {
                self.free.remove(&prev);
                start = prev;
                len += prev_len;
            }
        }
        if let Some(&next_len) = self.free.get(&(start + len)) {
            self.free.remove(&(start + len));
            len += next_len;
        }
        self.free.insert(start, len);
    }

    pub fn free_bytes(&self) -> u64 {
        self.free.values().sum()
    }
}

// The generic DRM ioctls are function-like macros in drm.h, so they are computed here: _IOC(dir, 'd', nr, size).
const fn drm_ioc(dir: u64, nr: u64, size: usize) -> u64 {
    (dir << 30) | ((size as u64) << 16) | ((b'd' as u64) << 8) | nr
}
const IOC_WRITE: u64 = 1;
const IOC_READWRITE: u64 = 3;
const DRM_IOCTL_GEM_CLOSE: u64 = drm_ioc(IOC_WRITE, 0x09, std::mem::size_of::<drm_gem_close>());
const DRM_IOCTL_SYNCOBJ_CREATE: u64 = drm_ioc(IOC_READWRITE, 0xbf, std::mem::size_of::<drm_syncobj_create>());
const DRM_IOCTL_SYNCOBJ_DESTROY: u64 = drm_ioc(IOC_READWRITE, 0xc0, std::mem::size_of::<drm_syncobj_destroy>());
const DRM_IOCTL_SYNCOBJ_WAIT: u64 = drm_ioc(IOC_READWRITE, 0xc3, std::mem::size_of::<drm_syncobj_wait>());
const DRM_IOCTL_SYNCOBJ_RESET: u64 = drm_ioc(IOC_READWRITE, 0xc4, std::mem::size_of::<drm_syncobj_array>());

fn stage_error(stage: &str, e: io::Error) -> io::Error {
    io::Error::new(e.kind(), error::StageError { stage: stage.to_owned(), source: e })
}

#[repr(C)]
struct drm_syncobj_handle {
    handle: u32,
    flags: u32,
    fd: i32,
    pad: u32,
}
const DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE: u32 = 1;
const DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD: u64 = drm_ioc(IOC_READWRITE, 0xc1, std::mem::size_of::<drm_syncobj_handle>());

// linux/sync_file.h
#[repr(C)]
struct sync_file_info {
    name: [u8; 32],
    status: i32,
    flags: u32,
    num_fences: u32,
    pad: u32,
    sync_fence_info: u64,
}
const SYNC_IOC_FILE_INFO: u64 =
    (IOC_READWRITE << 30) | ((std::mem::size_of::<sync_file_info>() as u64) << 16) | ((b'>' as u64) << 8) | 4;

struct DeviceInner {
    fd: OwnedFd,
}

impl DeviceInner {
    fn ioctl<T>(&self, request: u64, arg: &mut T, stage: &str) -> io::Result<()> {
        loop {
            let r = unsafe { libc::ioctl(self.fd.as_raw_fd(), request as _, arg as *mut T) };
            if r == 0 {
                return Ok(());
            }
            let e = io::Error::last_os_error();
            match e.raw_os_error() {
                Some(libc::EINTR) | Some(libc::EAGAIN) => continue,
                _ => return Err(stage_error(stage, e)),
            }
        }
    }
}

/// An open Asahi render node and its global parameters.
pub struct Device {
    inner: Rc<DeviceInner>,
    pub params: drm_asahi_params_global,
}

impl Device {
    pub fn open(path: &str) -> io::Result<Device> {
        let c = CString::new(path).unwrap();
        let fd = unsafe { libc::open(c.as_ptr(), libc::O_RDWR | libc::O_CLOEXEC) };
        if fd < 0 {
            return Err(stage_error("open", io::Error::last_os_error()));
        }
        let inner = Rc::new(DeviceInner { fd: unsafe { OwnedFd::from_raw_fd(fd) } });
        let mut params: drm_asahi_params_global = unsafe { std::mem::zeroed() };
        let mut gp = drm_asahi_get_params {
            param_group: 0,
            pad: 0,
            pointer: &mut params as *mut _ as u64,
            size: std::mem::size_of_val(&params) as u64,
        };
        inner.ioctl(DRM_IOCTL_ASAHI_GET_PARAMS as u64, &mut gp, "get-params")?;
        Ok(Device { inner, params })
    }

    /// Multi-cluster G14 parts (M2 Pro / Max / Ultra) need an extra CDM launch word.
    pub fn g14x(&self) -> bool {
        self.params.gpu_generation >= 14 && self.params.num_clusters_total > 1
    }

    /// The `-mcpu` of the LLVM AGX backend for this GPU, e.g. `agx-g14c`.
    pub fn llvm_cpu(&self) -> String {
        format!("agx-g{}{}", self.params.gpu_generation, (self.params.gpu_variant as u8 as char).to_ascii_lowercase())
    }

    pub fn core_count(&self) -> u32 {
        self.params.core_masks.iter().map(|m| m.count_ones()).sum()
    }

    pub fn gpu_time(&self) -> io::Result<u64> {
        let mut t = drm_asahi_get_time { flags: 0, gpu_timestamp: 0 };
        self.inner.ioctl(DRM_IOCTL_ASAHI_GET_TIME as u64, &mut t, "get-time")?;
        Ok(t.gpu_timestamp)
    }
}

/// A GEM buffer object, mapped for the CPU and optionally bound in a VM.
pub struct Bo {
    dev: Rc<DeviceInner>,
    pub handle: u32,
    pub size: u64,
    /// GPU virtual address, or 0 if the object is not bound.
    pub va: u64,
    map: *mut u8,
    /// The VM the object is bound in, and the allocator its range returns to.
    binding: Option<(u32, Option<Rc<RefCell<RangeAllocator>>>)>,
}

impl Bo {
    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.map, self.size as usize) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.map, self.size as usize) }
    }
}

impl Drop for Bo {
    fn drop(&mut self) {
        if let Some((vm_id, heap)) = self.binding.take() {
            let op = drm_asahi_gem_bind_op {
                flags: DRM_ASAHI_BIND_UNBIND as u32,
                handle: 0,
                offset: 0,
                range: self.size,
                addr: self.va,
            };
            let mut vb = drm_asahi_vm_bind {
                vm_id,
                num_binds: 1,
                stride: std::mem::size_of_val(&op) as u32,
                pad: 0,
                userptr: &op as *const _ as u64,
            };
            // Reuse the range only if it is really unmapped.
            if self.dev.ioctl(DRM_IOCTL_ASAHI_VM_BIND as u64, &mut vb, "vm-unbind").is_ok() {
                if let Some(heap) = heap {
                    heap.borrow_mut().free(self.va, self.size);
                }
            }
        }
        // Close explicitly: leaving objects to process exit leaked their shmem in the kernel
        // (simulator drm_compute.c, AGX_CLEAN_EXIT).
        unsafe { libc::munmap(self.map as *mut _, self.size as usize) };
        let mut close = drm_gem_close { handle: self.handle, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_GEM_CLOSE, &mut close, "gem-close");
    }
}

/// A GPU address space with Mesa's layout: a 4 GiB USC heap at the first 4 GiB boundary at or above 2^36, data
/// after it, and the kernel's range at the top. Fixed addresses below the USC heap are left to the caller.
pub struct Vm {
    dev: Rc<DeviceInner>,
    pub id: u32,
    pub usc_base: u64,
    usc: Rc<RefCell<RangeAllocator>>,
    data: Rc<RefCell<RangeAllocator>>,
}

pub const USC_HEAP_SIZE: u64 = 1 << 32;

impl Vm {
    pub fn new(dev: &Device) -> io::Result<Vm> {
        let p = &dev.params;
        let usc_base = round_up(p.vm_start.max(1 << 36), 1 << 32);
        let ksize = p.vm_kernel_min_size.max(32 << 30);
        let mut vc = drm_asahi_vm_create { kernel_start: p.vm_end - ksize, kernel_end: p.vm_end, vm_id: 0, pad: 0 };
        let data_base = usc_base + USC_HEAP_SIZE;
        if data_base + (1 << 30) >= vc.kernel_start {
            return Err(io::Error::new(io::ErrorKind::Other, "address-layout: VM too small"));
        }
        dev.inner.ioctl(DRM_IOCTL_ASAHI_VM_CREATE as u64, &mut vc, "vm-create")?;
        Ok(Vm {
            dev: dev.inner.clone(),
            id: vc.vm_id,
            usc_base,
            // offset 0 of the heap is never used, so a USC offset of 0 can mean "none"
            usc: Rc::new(RefCell::new(RangeAllocator::new(usc_base + PAGE, USC_HEAP_SIZE - PAGE))),
            data: Rc::new(RefCell::new(RangeAllocator::new(data_base, vc.kernel_start - data_base))),
        })
    }

    fn create(&self, size: u64, va: u64, bind_flags: u32, heap: Option<&Rc<RefCell<RangeAllocator>>>) -> io::Result<Bo> {
        let size = round_up(size.max(1), PAGE);
        let mut gc = drm_asahi_gem_create { size, flags: DRM_ASAHI_GEM_WRITEBACK as u32, vm_id: 0, handle: 0, pad: 0 };
        self.dev.ioctl(DRM_IOCTL_ASAHI_GEM_CREATE as u64, &mut gc, "gem-create")?;
        let mut mo = drm_asahi_gem_mmap_offset { handle: gc.handle, flags: 0, offset: 0 };
        let mut bo = Bo { dev: self.dev.clone(), handle: gc.handle, size, va: 0, map: std::ptr::null_mut(), binding: None };
        self.dev.ioctl(DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET as u64, &mut mo, "gem-mmap-offset")?;
        let map = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED,
                self.dev.fd.as_raw_fd(),
                mo.offset as libc::off_t,
            )
        };
        if map == libc::MAP_FAILED {
            return Err(stage_error("mmap", io::Error::last_os_error()));
        }
        bo.map = map as *mut u8;
        bo.as_mut_slice().fill(0);
        if va != 0 {
            let op = drm_asahi_gem_bind_op { flags: bind_flags, handle: bo.handle, offset: 0, range: size, addr: va };
            let mut vb = drm_asahi_vm_bind {
                vm_id: self.id,
                num_binds: 1,
                stride: std::mem::size_of_val(&op) as u32,
                pad: 0,
                userptr: &op as *const _ as u64,
            };
            self.dev.ioctl(DRM_IOCTL_ASAHI_VM_BIND as u64, &mut vb, "vm-bind")?;
            bo.va = va;
            bo.binding = Some((self.id, heap.cloned()));
        }
        Ok(bo)
    }

    /// A read-only object in the USC heap, for shader code and pipeline words. Its USC offset is
    /// `bo.va - vm.usc_base`.
    pub fn alloc_usc(&mut self, size: u64) -> io::Result<Bo> {
        self.alloc_in(&self.usc.clone(), size, DRM_ASAHI_BIND_READ as u32, "USC heap full")
    }

    /// A read/write data object at the next free data address.
    pub fn alloc_data(&mut self, size: u64) -> io::Result<Bo> {
        self.alloc_in(&self.data.clone(), size, (DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE) as u32, "data heap full")
    }

    fn alloc_in(&self, heap: &Rc<RefCell<RangeAllocator>>, size: u64, flags: u32, full: &str) -> io::Result<Bo> {
        let size = round_up(size.max(1), PAGE);
        let va = heap.borrow_mut().alloc(size, PAGE).ok_or_else(|| io::Error::new(io::ErrorKind::OutOfMemory, full))?;
        self.create(size, va, flags, Some(heap)).inspect_err(|_| heap.borrow_mut().free(va, size))
    }

    /// A read/write data object at a fixed GPU address (page aligned, below the USC heap).
    pub fn alloc_at(&mut self, va: u64, size: u64) -> io::Result<Bo> {
        if va % PAGE != 0 || va + round_up(size, PAGE) > self.usc_base {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("fixed address {va:#x} not usable")));
        }
        self.create(size, va, (DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE) as u32, None)
    }

    /// An unbound object (e.g. firmware timestamps).
    pub fn alloc_unbound(&mut self, size: u64) -> io::Result<Bo> {
        self.create(size, 0, 0, None)
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        let mut vd = drm_asahi_vm_destroy { vm_id: self.id, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_ASAHI_VM_DESTROY as u64, &mut vd, "vm-destroy");
    }
}

/// What a compute pipeline needs: everything in the USC pipeline words.
#[derive(Clone, Debug)]
pub struct Pipeline {
    /// USC offset of the shader's first instruction.
    pub code: u32,
    /// General purpose registers the shader uses, in 16-bit halves (8..=256).
    pub gpr_halves: u32,
    /// Uniform preload ranges: (first uniform half, number of halves, GPU address of the values).
    pub uniforms: Vec<(u32, u32, u64)>,
    /// Threadgroup memory per workgroup in bytes (0..=65536).
    pub shared_bytes: u32,
}

/// The largest uniform range one USC Uniform word is used for. The field holds at least 8 halves (measured);
/// its full width is unmeasured (blocker B9), so longer ranges are split into words of this size.
pub const UNIFORM_WORD_HALVES: u32 = 32;

impl Pipeline {
    /// The uniform halves the dispatch must declare: one past the highest preloaded half.
    pub fn uniform_halves(&self) -> u32 {
        self.uniforms.iter().map(|&(s, n, _)| s + n).max().unwrap_or(0)
    }

    /// The USC pipeline words, in the measured order: Uniform*, Shared, Shader, Registers, No preshader.
    pub fn usc_words(&self) -> Vec<u8> {
        assert!(self.code % 64 == 0, "shader code must be 64-byte aligned");
        assert!((8..=256).contains(&self.gpr_halves));
        assert!(self.shared_bytes <= 65536);
        let mut w = Vec::new();
        for &(start, count, va) in &self.uniforms {
            assert!(va % 4 == 0, "uniform values must be 4-byte aligned");
            let mut done = 0;
            while done < count {
                let n = (count - done).min(UNIFORM_WORD_HALVES);
                let s = start + done;
                assert!(s < 256);
                let v = va + 2 * done as u64;
                // Uniform: tag 0x1d, start half at bit 8, halves at bit 20, value address >> 2 at bit 26
                w.extend_from_slice(&(0x1du64 | (s as u64) << 8 | (n as u64) << 20 | (v >> 2) << 26).to_le_bytes());
                done += n;
            }
        }
        // Shared: tag 0x4d, compute layout 0x24 at bit 10; with threadgroup memory, bit 8 and the size in 256-byte
        // units at bit 24 (0 = all 64 KiB, which is also what a kernel without any gets).
        let shared = if self.shared_bytes == 0 {
            0x4d | 0x24 << 10
        } else {
            0x4d | 1 << 8 | 0x24 << 10 | ((self.shared_bytes.div_ceil(256) & 255) << 24)
        };
        w.extend_from_slice(&(shared as u32).to_le_bytes());
        // Shader: tag 0x0d, unk_2 = 3 (compute) at bit 10, code offset at bit 16 (6 bytes)
        w.extend_from_slice(&(0x0du64 | 3 << 10 | (self.code as u64) << 16).to_le_bytes()[..6]);
        // Registers: tag 0x8d, registers in groups of 8 halves at bit 8 (5 bits, 0 = 256), unk_4 = 1 at bit 24
        let groups = self.gpr_halves.div_ceil(8) & 31;
        w.extend_from_slice(&(0x8du32 | groups << 8 | 1 << 24).to_le_bytes());
        // No preshader
        w.extend_from_slice(&[0x88, 0x00]);
        w
    }
}

/// The CDM control stream of one direct dispatch, followed by a USC-cache-invalidating barrier and the stream
/// terminator. `global` is in threads and must be a multiple of `local` in every dimension.
pub fn cdm_dispatch(pipeline_words: u32, uniform_halves: u32, g14x: bool, global: [u32; 3], local: [u32; 3]) -> Vec<u8> {
    assert!(pipeline_words % 64 == 0);
    let threads: u64 = local.iter().map(|&x| x as u64).product();
    assert!(threads >= 1 && threads <= 1024);
    for i in 0..3 {
        assert!(global[i] >= 1 && global[i] % local[i] == 0, "partial workgroups are not supported");
    }
    let mut c = Vec::new();
    let put = |c: &mut Vec<u8>, v: u32| c.extend_from_slice(&v.to_le_bytes());
    // Launch word 0: uniform registers in groups of 64 halves at bit 1; direct mode; block type 0
    put(&mut c, uniform_halves.div_ceil(64).max(1) << 1);
    // Launch word 1: USC offset of the pipeline words
    put(&mut c, pipeline_words);
    if g14x {
        put(&mut c, 0);
        put(&mut c, 1 << 30);
    }
    for g in global {
        put(&mut c, g);
    }
    for l in local {
        put(&mut c, l);
    }
    // Barrier: Mesa's unk0..19, including the USC cache invalidate
    put(&mut c, 3 << 29 | 0xfffff);
    // Stream terminate
    put(&mut c, 2 << 29);
    put(&mut c, 0);
    c
}

/// Firmware timestamps of one compute command, in `command_timestamp_frequency_hz` units.
#[derive(Clone, Copy, Debug)]
pub struct Timing {
    pub start: u64,
    pub end: u64,
    pub host: Duration,
}

/// A compute queue with one out-syncobj and a timestamp object.
pub struct Queue {
    dev: Rc<DeviceInner>,
    pub id: u32,
    syncobj: u32,
    ts: Bo,
    ts_handle: u32,
    pub timeout: Duration,
}

impl Queue {
    pub fn new(vm: &mut Vm) -> io::Result<Queue> {
        let mut qc = drm_asahi_queue_create {
            flags: 0,
            vm_id: vm.id,
            priority: DRM_ASAHI_PRIORITY_MEDIUM as u32,
            queue_id: 0,
            usc_exec_base: vm.usc_base,
        };
        vm.dev.ioctl(DRM_IOCTL_ASAHI_QUEUE_CREATE as u64, &mut qc, "queue-create")?;
        let mut sc = drm_syncobj_create { handle: 0, flags: 0 };
        vm.dev.ioctl(DRM_IOCTL_SYNCOBJ_CREATE, &mut sc, "syncobj-create")?;
        let ts = vm.alloc_unbound(PAGE)?;
        let mut bind = drm_asahi_gem_bind_object {
            op: DRM_ASAHI_BIND_OBJECT_OP_BIND as u32,
            flags: DRM_ASAHI_BIND_OBJECT_USAGE_TIMESTAMPS as u32,
            handle: ts.handle,
            vm_id: 0,
            offset: 0,
            range: PAGE,
            object_handle: 0,
            pad: 0,
        };
        vm.dev.ioctl(DRM_IOCTL_ASAHI_GEM_BIND_OBJECT as u64, &mut bind, "bind-timestamps")?;
        Ok(Queue {
            dev: vm.dev.clone(),
            id: qc.queue_id,
            syncobj: sc.handle,
            ts,
            ts_handle: bind.object_handle,
            timeout: Duration::from_secs(3),
        })
    }

    /// Submits one compute command running the CDM stream at `[base, end)` and waits for it.
    pub fn run_compute(&mut self, cdm_base: u64, cdm_end: u64) -> io::Result<Timing> {
        self.ts.as_mut_slice()[..16].fill(0);
        #[repr(C)]
        struct Cmd {
            h: drm_asahi_cmd_header,
            c: drm_asahi_cmd_compute,
        }
        let mut cmd: Cmd = unsafe { std::mem::zeroed() };
        cmd.h.cmd_type = DRM_ASAHI_CMD_COMPUTE as u16;
        cmd.h.size = std::mem::size_of::<drm_asahi_cmd_compute>() as u16;
        cmd.h.vdm_barrier = DRM_ASAHI_BARRIER_NONE as u16;
        cmd.h.cdm_barrier = 0;
        cmd.c.cdm_ctrl_stream_base = cdm_base;
        cmd.c.cdm_ctrl_stream_end = cdm_end;
        cmd.c.ts.start = drm_asahi_timestamp { handle: self.ts_handle, offset: 0 };
        cmd.c.ts.end = drm_asahi_timestamp { handle: self.ts_handle, offset: 8 };
        let out_sync = drm_asahi_sync { sync_type: DRM_ASAHI_SYNC_SYNCOBJ as u32, handle: self.syncobj, timeline_value: 0 };
        let mut sub = drm_asahi_submit {
            syncs: &out_sync as *const _ as u64,
            cmdbuf: &cmd as *const _ as u64,
            flags: 0,
            queue_id: self.id,
            in_sync_count: 0,
            out_sync_count: 1,
            cmdbuf_size: std::mem::size_of::<Cmd>() as u32,
            pad: 0,
        };
        let h0 = Instant::now();
        self.dev.ioctl(DRM_IOCTL_ASAHI_SUBMIT as u64, &mut sub, "submit")?;
        let mut now: libc::timespec = unsafe { std::mem::zeroed() };
        unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) };
        let deadline = now.tv_sec as i64 * 1_000_000_000 + now.tv_nsec as i64 + self.timeout.as_nanos() as i64;
        let mut wait: drm_syncobj_wait = unsafe { std::mem::zeroed() };
        wait.handles = &self.syncobj as *const _ as u64;
        wait.count_handles = 1;
        wait.timeout_nsec = deadline;
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_WAIT, &mut wait, "syncobj-wait")?;
        let host = h0.elapsed();
        let status = self.fence_status()?;
        let mut reset = drm_syncobj_array { handles: &self.syncobj as *const _ as u64, count_handles: 1, pad: 0 };
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_RESET, &mut reset, "syncobj-reset")?;
        if status < 0 {
            return Err(io::Error::other(error::GpuError { fence_status: status }));
        }
        let t = self.ts.as_slice();
        Ok(Timing {
            start: u64::from_le_bytes(t[0..8].try_into().unwrap()),
            end: u64::from_le_bytes(t[8..16].try_into().unwrap()),
            host,
        })
    }
}

impl Queue {
    /// The status of the out-fence: 1 when signalled, negative errno when the kernel failed the job.
    fn fence_status(&self) -> io::Result<i32> {
        let mut h = drm_syncobj_handle { handle: self.syncobj, flags: DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE, fd: -1, pad: 0 };
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD, &mut h, "syncobj-export")?;
        let fd = unsafe { OwnedFd::from_raw_fd(h.fd) };
        let mut info: sync_file_info = unsafe { std::mem::zeroed() };
        if unsafe { libc::ioctl(fd.as_raw_fd(), SYNC_IOC_FILE_INFO as _, &mut info) } != 0 {
            return Err(stage_error("sync-file-info", io::Error::last_os_error()));
        }
        Ok(info.status)
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        let mut qd = drm_asahi_queue_destroy { queue_id: self.id, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_ASAHI_QUEUE_DESTROY as u64, &mut qd, "queue-destroy");
        let mut sd = drm_syncobj_destroy { handle: self.syncobj, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_SYNCOBJ_DESTROY, &mut sd, "syncobj-destroy");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_allocator_reuses_and_merges_freed_ranges() {
        let mut a = RangeAllocator::new(0x10000, 0x10000);
        let x = a.alloc(0x4000, 0x4000).unwrap();
        let y = a.alloc(0x4000, 0x4000).unwrap();
        let z = a.alloc(0x4000, 0x4000).unwrap();
        assert_eq!((x, y, z), (0x10000, 0x14000, 0x18000));
        a.free(y, 0x4000);
        assert_eq!(a.alloc(0x4000, 0x4000), Some(y));
        assert_eq!(a.alloc(0x8000, 0x4000), None);
        a.free(x, 0x4000);
        a.free(z, 0x4000);
        a.free(y, 0x4000);
        assert_eq!(a.free_bytes(), 0x10000);
        assert_eq!(a.alloc(0x10000, 0x4000), Some(0x10000));
    }

    #[test]
    fn range_allocator_aligns_inside_a_free_range() {
        let mut a = RangeAllocator::new(0x1000, 0x20000);
        assert_eq!(a.alloc(0x4000, 0x8000), Some(0x8000));
        assert_eq!(a.alloc(0x1000, 0x1000), Some(0x1000));
        assert_eq!(a.free_bytes(), 0x20000 - 0x5000);
    }

    fn hex(b: &[u8]) -> String {
        b.iter().map(|x| format!("{x:02x}")).collect()
    }

    // Bytes recorded from the direct-DRM control dispatch on the M2 Max (direct-drm-compute.md).
    #[test]
    fn usc_words_match_the_measured_control() {
        let p = Pipeline { code: 0x2000, gpr_halves: 8, uniforms: vec![(8, 8, 0x11_0000_0000)], shared_bytes: 0 };
        assert_eq!(hex(&p.usc_words()), "1d08800000000011".to_owned() + "4d900000" + "0d0c00200000" + "8d010001" + "8800");
    }

    #[test]
    fn cdm_matches_the_measured_control() {
        let c = cdm_dispatch(0x1000, 16, true, [64, 1, 1], [32, 1, 1]);
        let want = "02000000".to_owned()
            + "00100000"
            + "0000000000000040"
            + "400000000100000001000000"
            + "200000000100000001000000"
            + "ffff0f60"
            + "0000004000000000";
        assert_eq!(hex(&c), want);
    }

    // Shared word as built by the simulator's run_drm_tg_layout.py.
    #[test]
    fn shared_word_with_threadgroup_memory() {
        let p = Pipeline { code: 0, gpr_halves: 256, uniforms: vec![], shared_bytes: 4096 };
        let w = p.usc_words();
        assert_eq!(u32::from_le_bytes(w[0..4].try_into().unwrap()), 0x4d | 1 << 8 | 0x24 << 10 | 16 << 24);
        // 256 halves is register group field 0
        assert_eq!(&w[10..14], &(0x8du32 | 1 << 24).to_le_bytes());
    }

    #[test]
    fn long_uniform_ranges_are_split() {
        let p = Pipeline { code: 0, gpr_halves: 8, uniforms: vec![(0, 40, 0x1000)], shared_bytes: 0 };
        let w = p.usc_words();
        let a = u64::from_le_bytes(w[0..8].try_into().unwrap());
        let b = u64::from_le_bytes(w[8..16].try_into().unwrap());
        assert_eq!((a >> 8 & 0xff, a >> 20 & 0x3f, a >> 26), (0, 32, 0x1000 >> 2));
        assert_eq!((b >> 8 & 0xff, b >> 20 & 0x3f, b >> 26), (32, 8, (0x1000 + 64) >> 2));
        assert_eq!(p.uniform_halves(), 40);
    }

    #[test]
    fn generic_drm_ioctl_numbers() {
        assert_eq!(DRM_IOCTL_GEM_CLOSE, 0x4008_6409);
        assert_eq!(DRM_IOCTL_SYNCOBJ_CREATE, 0xc008_64bf);
        assert_eq!(DRM_IOCTL_SYNCOBJ_RESET, 0xc010_64c4);
    }
}

/// A module uploaded to the GPU: its code in the USC heap and its global data buffer.
pub struct LoadedModule {
    pub module: module::Module,
    /// Owns the kernels' code (the GPU reads it through `entries`).
    #[allow(dead_code)]
    code: Bo,
    /// USC offset of each kernel's copy in `code`.
    entries: std::collections::HashMap<String, u32>,
    globals: Option<Bo>,
}

impl LoadedModule {
    pub fn new(vm: &mut Vm, module: module::Module) -> io::Result<LoadedModule> {
        // Every kernel is copied to its own 64-byte aligned slot (the Shader word's code address has only been
        // measured 64-byte aligned, and the backend aligns kernels to 2 bytes). Kernels are self-contained:
        // branches are PC-relative and there are no calls between them. 64 bytes of slack follow each one.
        let mut layout = Vec::new();
        let mut end = 0u64;
        for k in module.kernels.values() {
            layout.push((k.name.clone(), end, k.offset as usize, k.size as usize));
            end = round_up(end + k.size as u64 + 64, 64);
        }
        let mut code = vm.alloc_usc(end.max(64))?;
        let base = (code.va - vm.usc_base) as u32;
        let mut entries = std::collections::HashMap::new();
        for (name, at, off, size) in layout {
            code.as_mut_slice()[at as usize..at as usize + size].copy_from_slice(&module.text[off..off + size]);
            entries.insert(name, base + at as u32);
        }
        let globals = if module.globals.is_empty() {
            None
        } else {
            let mut g = vm.alloc_data(module.globals.len() as u64)?;
            g.as_mut_slice()[..module.globals.len()].copy_from_slice(&module.globals);
            Some(g)
        };
        Ok(LoadedModule { module, code, entries, globals })
    }

    /// The current contents of the global data buffer (device variables).
    pub fn globals(&self) -> &[u8] {
        self.globals.as_ref().map_or(&[], |g| &g.as_slice()[..self.module.globals.len()])
    }

    /// Runs `kernel` over `threadgroups` workgroups of `local` threads and waits for it.
    /// `dynamic_shared` is added to the kernel's static threadgroup memory.
    pub fn launch(
        &self,
        vm: &mut Vm,
        queue: &mut Queue,
        g14x: bool,
        kernel: &str,
        args: &[module::Arg],
        threadgroups: [u32; 3],
        local: [u32; 3],
        dynamic_shared: u32,
    ) -> io::Result<Timing> {
        let invalid = |m: String| io::Error::new(io::ErrorKind::InvalidInput, m);
        let k = self.module.kernels.get(kernel).ok_or_else(|| invalid(format!("no kernel {kernel}")))?;
        if k.stack_bytes > 0 {
            // per-thread scratch for direct-DRM dispatches is not measured yet (ptx-re-blockers B1)
            return Err(io::Error::new(io::ErrorKind::Unsupported, format!("{kernel} needs {} bytes of stack", k.stack_bytes)));
        }
        let shared = k.threadgroup_bytes + dynamic_shared;
        if shared > 65536 {
            return Err(invalid(format!("{kernel} needs {shared} bytes of threadgroup memory")));
        }
        let halves = module::pack_uniforms(k, args, threadgroups, self.globals.as_ref().map(|g| g.va))
            .map_err(|e| invalid(e.to_string()))?;
        let mut unif = vm.alloc_data((2 * halves.len()).max(4) as u64)?;
        for (i, h) in halves.iter().enumerate() {
            unif.as_mut_slice()[2 * i..2 * i + 2].copy_from_slice(&h.to_le_bytes());
        }
        let code_off = self.entries[kernel];
        let pipeline = Pipeline {
            code: code_off,
            gpr_halves: k.gpr_halves(),
            uniforms: if halves.is_empty() { vec![] } else { vec![(0, halves.len() as u32, unif.va)] },
            shared_bytes: shared,
        };
        let words = pipeline.usc_words();
        let mut pw = vm.alloc_usc(words.len() as u64)?;
        pw.as_mut_slice()[..words.len()].copy_from_slice(&words);
        let global = std::array::from_fn(|i| threadgroups[i] * local[i]);
        let stream = cdm_dispatch((pw.va - vm.usc_base) as u32, pipeline.uniform_halves(), g14x, global, local);
        let mut cdm = vm.alloc_data(stream.len() as u64)?;
        cdm.as_mut_slice()[..stream.len()].copy_from_slice(&stream);
        queue.run_compute(cdm.va, cdm.va + stream.len() as u64)
    }
}
