//! Compute on Apple G14 GPUs (M2 family) through the Asahi DRM uAPI, without Mesa.
//!
//! This is the submission layer of ZLUDA's AGX backend: a device, its GPU address space, buffer objects, compute
//! queues, and the encoding of one compute dispatch (the USC pipeline words and the CDM control stream).
//!
//! The words follow the direct-DRM harness of the AGX Performance Simulator (`tests/drm_compute.c`), which was
//! verified on an M2 Max (G14C) under Asahi Linux. See Apple-M2-Playground `experiments/baseline/direct-drm-compute.md`
//! for the measured layout. The tests below pin those exact bytes. Fields that are not yet measured are listed in
//! the AGX Performance Simulator's `tasks/ptx-re-blockers.md`:
//! - the largest uniform preload (B9);
//! - partial workgroups.
//!
//! USC addresses (shader code, pipeline words) are 32-bit offsets from the queue's `usc_exec_base`.

pub mod cache;
pub mod context;
pub mod error;
pub mod module;
pub mod printf;
pub mod ring;
pub mod scratch;

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
const DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT: u64 =
    drm_ioc(IOC_READWRITE, 0xca, std::mem::size_of::<drm_syncobj_timeline_wait>());
const DRM_IOCTL_SYNCOBJ_QUERY: u64 = drm_ioc(IOC_READWRITE, 0xcb, std::mem::size_of::<drm_syncobj_timeline_array>());
const DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT: u32 = 1 << 1;

fn stage_error(stage: &str, e: io::Error) -> io::Error {
    io::Error::new(e.kind(), error::StageError { stage: stage.to_owned(), source: e })
}

#[repr(C)]
struct drm_syncobj_handle {
    handle: u32,
    flags: u32,
    fd: i32,
    pad: u32,
    point: u64,
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
    /// Whether the CPU mapping is at the GPU address, so that the GPU address is also a valid CPU pointer.
    pub fn unified(&self) -> bool {
        self.va != 0 && self.map as u64 == self.va
    }

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
    /// Header entries of the present cores (`scratch::core_entries`).
    core_entries: Vec<u32>,
    scratch: Option<Scratch>,
    /// Counts USC allocations (module code, rings): a queue's next command invalidates the USC cache when this has
    /// changed since its last one.
    pub usc_generation: u64,
}

/// The helper program and the scratch buffer of an address space.
struct Scratch {
    helper: Bo,
    buffer: Bo,
    layout: scratch::Layout,
    /// Smaller buffers that commands in flight may still use; freed with the address space.
    #[allow(dead_code)]
    retired: Vec<Bo>,
}

pub const USC_HEAP_SIZE: u64 = 1 << 32;

impl Vm {
    pub fn new(dev: &Device) -> io::Result<Vm> {
        let p = &dev.params;
        let usc_base = round_up(p.vm_start.max(1 << 36), 1 << 32);
        let ksize = p.vm_kernel_min_size.max(32 << 30);
        let mut vc = drm_asahi_vm_create { kernel_start: p.vm_end - ksize, kernel_end: p.vm_end, vm_id: 0, pad: 0 };
        let data_base = usc_base + USC_HEAP_SIZE;
        // Generic pointers to threadgroup memory carry bit 63 (LLVM AGX, AGXLowerGenericAddresses), so device
        // addresses must stay below it.
        if p.vm_end > 1 << 63 {
            return Err(io::Error::new(io::ErrorKind::Other, "address-layout: VM reaches bit 63"));
        }
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
            core_entries: scratch::core_entries(&p.core_masks, p.num_clusters_total, p.num_cores_per_cluster),
            scratch: None,
            usc_generation: 0,
        })
    }

    /// The helper program of a command whose shader uses `dwords` 32-bit words of stack per thread: its tagged USC
    /// address and the address of the scratch buffer's header (`drm_asahi_helper_program`). The buffer grows to the
    /// largest size asked for and is shared by every command of the address space.
    pub fn scratch(&mut self, dwords: u32) -> io::Result<(u32, u64)> {
        if dwords > (8 << 12) * 4 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("{dwords} words of stack per thread")));
        }
        let have = self.scratch.as_ref().map_or(0, |s| s.layout.dwords());
        if (dwords as u64) > have {
            let layout = scratch::Layout::new(dwords);
            let buf = scratch::Buffer::new(layout, self.core_entries.len(), PAGE);
            let mut bo = self.alloc_data_aligned(buf.size, buf.align)?;
            let header = buf.header(bo.va, &self.core_entries);
            bo.as_mut_slice()[..header.len()].copy_from_slice(&header);
            match &mut self.scratch {
                Some(s) => {
                    let old = std::mem::replace(&mut s.buffer, bo);
                    s.retired.push(old);
                    s.layout = layout;
                }
                None => {
                    let mut helper = self.alloc_usc(scratch::HELPER.len() as u64)?;
                    helper.as_mut_slice()[..scratch::HELPER.len()].copy_from_slice(scratch::HELPER);
                    self.scratch = Some(Scratch { helper, buffer: bo, layout, retired: Vec::new() });
                }
            }
        }
        let s = self.scratch.as_ref().unwrap();
        // a tagged USC pointer, as Mesa's agx_helper_program
        Ok(((s.helper.va - self.usc_base) as u32 | 1, s.buffer.va))
    }

    /// The scratch buffer's header and block lists, to check the helper's counters (`scratch::core_counters`).
    pub fn scratch_header(&self) -> Option<&[u8]> {
        self.scratch.as_ref().map(|s| &s.buffer.as_slice()[..scratch::Buffer::new(s.layout, self.core_entries.len(), PAGE).blocks_offset as usize])
    }

    fn create(&self, size: u64, va: u64, bind_flags: u32, heap: Option<&Rc<RefCell<RangeAllocator>>>) -> io::Result<Bo> {
        let size = round_up(size.max(1), PAGE);
        let mut gc = drm_asahi_gem_create { size, flags: DRM_ASAHI_GEM_WRITEBACK as u32, vm_id: 0, handle: 0, pad: 0 };
        self.dev.ioctl(DRM_IOCTL_ASAHI_GEM_CREATE as u64, &mut gc, "gem-create")?;
        let mut mo = drm_asahi_gem_mmap_offset { handle: gc.handle, flags: 0, offset: 0 };
        let mut bo = Bo { dev: self.dev.clone(), handle: gc.handle, size, va: 0, map: std::ptr::null_mut(), binding: None };
        self.dev.ioctl(DRM_IOCTL_ASAHI_GEM_MMAP_OFFSET as u64, &mut mo, "gem-mmap-offset")?;
        // Bound objects are mapped at their GPU address when that CPU range is free, so that one pointer works on
        // both sides (unified memory: cuMemAllocManaged, GPU-visible host allocations).
        let mmap = |at: u64, flags| unsafe {
            libc::mmap(
                at as *mut libc::c_void,
                size as usize,
                libc::PROT_READ | libc::PROT_WRITE,
                libc::MAP_SHARED | flags,
                self.dev.fd.as_raw_fd(),
                mo.offset as libc::off_t,
            )
        };
        let mut map = if va != 0 { mmap(va, libc::MAP_FIXED_NOREPLACE) } else { libc::MAP_FAILED };
        if map != libc::MAP_FAILED && map as u64 != va {
            // a kernel without MAP_FIXED_NOREPLACE takes the address as a hint only
            unsafe { libc::munmap(map, size as usize) };
            map = libc::MAP_FAILED;
        }
        if map == libc::MAP_FAILED {
            map = mmap(0, 0);
        }
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
        self.usc_generation += 1;
        self.alloc_in(&self.usc.clone(), size, DRM_ASAHI_BIND_READ as u32, "USC heap full")
    }

    /// A read/write data object at the next free data address.
    pub fn alloc_data(&mut self, size: u64) -> io::Result<Bo> {
        self.alloc_in(&self.data.clone(), size, (DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE) as u32, "data heap full")
    }

    /// `alloc_data` at an address that is a multiple of `align` (a power of two, at least a page).
    pub fn alloc_data_aligned(&mut self, size: u64, align: u64) -> io::Result<Bo> {
        self.alloc_in_aligned(&self.data.clone(), size, align, (DRM_ASAHI_BIND_READ | DRM_ASAHI_BIND_WRITE) as u32, "data heap full")
    }

    fn alloc_in(&self, heap: &Rc<RefCell<RangeAllocator>>, size: u64, flags: u32, full: &str) -> io::Result<Bo> {
        self.alloc_in_aligned(heap, size, PAGE, flags, full)
    }

    fn alloc_in_aligned(&self, heap: &Rc<RefCell<RangeAllocator>>, size: u64, align: u64, flags: u32, full: &str) -> io::Result<Bo> {
        let size = round_up(size.max(1), PAGE);
        let va = heap.borrow_mut().alloc(size, align.max(PAGE)).ok_or_else(|| io::Error::new(io::ErrorKind::OutOfMemory, full))?;
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
        // unbind the scratch objects while the address space exists
        self.scratch = None;
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
    /// The scratch size bucket of a shader that uses the stack (`scratch::Layout::bucket`), or 0.
    pub spill_bucket: u32,
}

/// The largest uniform range one USC Uniform word is used for: 64 halves, as Mesa packs them (the 6-bit size field
/// holds 64 as 0). Measured on the M2 Max up to the whole 512-half file (ptx-re-blockers B9).
pub const UNIFORM_WORD_HALVES: u32 = 64;

/// The uniform file: 512 halves. Halves from 256 on are preloaded with the Uniform High word, as Mesa does.
pub const UNIFORM_FILE_HALVES: u32 = 512;

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
            assert!(start + count <= UNIFORM_FILE_HALVES);
            let mut done = 0;
            while done < count {
                let s = start + done;
                // A word stays on one side of half 256.
                let n = (count - done).min(UNIFORM_WORD_HALVES).min(if s < 256 { 256 - s } else { u32::MAX });
                let v = va + 2 * done as u64;
                // Uniform (tag 0x1d) or, from half 256, Uniform High (0x3d): start half at bit 8 (relative to 256 for
                // Uniform High), halves at bit 20 (64 as 0), value address >> 2 at bit 26
                let (tag, rel) = if s < 256 { (0x1du64, s) } else { (0x3du64, s - 256) };
                w.extend_from_slice(&(tag | (rel as u64) << 8 | ((n & 63) as u64) << 20 | (v >> 2) << 26).to_le_bytes());
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
        // Registers: tag 0x8d, registers in groups of 8 halves at bit 8 (5 bits, 0 = 256), the scratch bucket at bit 18
        // (4 bits, B1), unk_4 = 1 at bit 24
        assert!(self.spill_bucket <= 15);
        let groups = self.gpr_halves.div_ceil(8) & 31;
        w.extend_from_slice(&(0x8du32 | groups << 8 | self.spill_bucket << 18 | 1 << 24).to_le_bytes());
        // No preshader
        w.extend_from_slice(&[0x88, 0x00]);
        w
    }
}

/// The CDM control stream of one direct dispatch, followed by a USC-cache-invalidating barrier and the stream
/// terminator. `global` is in threads and must be a multiple of `local` in every dimension.
pub fn cdm_dispatch(pipeline_words: u32, uniform_halves: u32, g14x: bool, global: [u32; 3], local: [u32; 3]) -> Vec<u8> {
    cdm_dispatch_with(pipeline_words, uniform_halves, g14x, global, local, true)
}

/// `cdm_dispatch`, with the USC-cache-invalidating barrier only if `barrier`. Code and pipeline words rewritten in
/// place at one USC address are seen by the next command without it (1,000 rewrites, none stale, on the M2 Max:
/// ptx-re-blockers B8), and it costs about 1.8 us per command.
pub fn cdm_dispatch_with(
    pipeline_words: u32,
    uniform_halves: u32,
    g14x: bool,
    global: [u32; 3],
    local: [u32; 3],
    barrier: bool,
) -> Vec<u8> {
    assert!(pipeline_words % 64 == 0);
    let threads: u64 = local.iter().map(|&x| x as u64).product();
    assert!(threads >= 1 && threads <= 1024);
    for i in 0..3 {
        assert!(global[i] >= 1 && global[i] % local[i] == 0, "partial workgroups are not supported");
    }
    let mut c = Vec::new();
    let put = |c: &mut Vec<u8>, v: u32| c.extend_from_slice(&v.to_le_bytes());
    // Launch word 0: uniform registers in groups of 64 halves at bit 1 (3 bits, 8 groups as 0); direct mode; block
    // type 0
    assert!(uniform_halves <= UNIFORM_FILE_HALVES);
    put(&mut c, (uniform_halves.div_ceil(64).max(1) & 7) << 1);
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
    if barrier {
        put(&mut c, 3 << 29 | 0xfffff);
    }
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

/// Number of commands whose timestamps a queue keeps.
pub const TIMESTAMP_SLOTS: u64 = PAGE / 16;

/// The oldest command in (verified, upto] whose timestamps say it faulted, of those whose timestamp slot is still
/// theirs (the last `TIMESTAMP_SLOTS` of `submitted`).
fn first_faulted(verified: u64, upto: u64, submitted: u64, timing: impl Fn(u64) -> Option<(u64, u64)>) -> Option<u64> {
    let first = (verified + 1).max(submitted.saturating_sub(TIMESTAMP_SLOTS - 1)).max(1);
    (first..=upto).find(|&p| timing(p).is_some_and(job_faulted))
}

/// Whether the timestamps of a signalled command say it faulted: the firmware writes none for a faulted command.
fn job_faulted((start, end): (u64, u64)) -> bool {
    start == 0 && end == 0
}

/// A point on a queue's timeline: signalled when the command submitted with it and every earlier command of the
/// queue have finished. CUDA events and stream waits are built on these.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fence {
    pub syncobj: u32,
    pub point: u64,
}

/// An in-order compute queue (a CUDA stream): commands run in submission order (`cdm_barrier = 0`) and signal
/// increasing points of one timeline syncobj.
pub struct Queue {
    dev: Rc<DeviceInner>,
    pub id: u32,
    timeline: u32,
    /// The last point submitted.
    submitted: u64,
    ts: Bo,
    ts_handle: u32,
    pub timeout: Duration,
    /// Objects the GPU may still read, freed once their fence has signalled.
    in_flight: std::collections::VecDeque<(u64, Vec<Bo>)>,
    /// The last point whose timestamps were checked for a fault (`wait`).
    verified: std::cell::Cell<u64>,
    /// The VM's `usc_generation` at this queue's last command.
    usc_seen: std::cell::Cell<u64>,
    /// Rings for the per-launch objects: data (uniforms, control streams, parameter blocks) and USC (pipeline
    /// words). A launch costs no buffer object creation, mapping or binding.
    data_ring: (Bo, ring::RingAlloc),
    usc_ring: (Bo, ring::RingAlloc),
}

/// Ring sizes: enough for hundreds of launches in flight.
pub const DATA_RING_BYTES: u64 = 4 << 20;
pub const USC_RING_BYTES: u64 = 256 << 10;

/// Which ring of a queue.
#[derive(Clone, Copy)]
pub enum RingKind {
    Data,
    Usc,
}

/// Every command invalidates the USC cache, as Mesa's command stream does: each launch stages its pipeline words in
/// the USC ring, and with 45 or more launches queued the GPU can run stale words it cached while earlier commands ran
/// (AGX Performance Simulator tasks/zluda-issues.md, Z2). `ZLUDA_AGX_USC_BARRIER=0` invalidates only after new code
/// was allocated, which saves about 1.8 us per launch but is only safe with short queues.
fn always_usc_barrier() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| usc_barrier_from(std::env::var("ZLUDA_AGX_USC_BARRIER").ok().as_deref()))
}

fn usc_barrier_from(v: Option<&str>) -> bool {
    v != Some("0")
}

/// How long a queue waits for a command: `$ZLUDA_AGX_WAIT_MS` milliseconds, or without limit. The hardware has no
/// limit on how long a command runs (AGX Performance Simulator tasks/ptx-re-blockers.md, B6: one command ran 79.9 s
/// correctly on the M2 Max), and the firmware ends a deadlocked command after about 0.5 s with both timestamps 0,
/// which `wait` reports as a fault.
pub fn default_wait_timeout() -> Duration {
    wait_timeout_from(std::env::var("ZLUDA_AGX_WAIT_MS").ok().as_deref())
}

fn wait_timeout_from(ms: Option<&str>) -> Duration {
    match ms.and_then(|v| v.trim().parse::<u64>().ok()) {
        Some(ms) if ms > 0 => Duration::from_millis(ms),
        _ => Duration::MAX,
    }
}

fn monotonic_deadline(timeout: Duration) -> i64 {
    let mut now: libc::timespec = unsafe { std::mem::zeroed() };
    unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &mut now) };
    let t = now.tv_sec as i64 * 1_000_000_000 + now.tv_nsec as i64;
    t.saturating_add(timeout.as_nanos().min(i64::MAX as u128) as i64)
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
            timeline: sc.handle,
            submitted: 0,
            ts,
            ts_handle: bind.object_handle,
            timeout: default_wait_timeout(),
            in_flight: Default::default(),
            verified: Default::default(),
            usc_seen: std::cell::Cell::new(u64::MAX),
            data_ring: (vm.alloc_data(DATA_RING_BYTES)?, ring::RingAlloc::new(DATA_RING_BYTES)),
            usc_ring: (vm.alloc_usc(USC_RING_BYTES)?, ring::RingAlloc::new(USC_RING_BYTES)),
        })
    }

    /// The fence of the last submitted command (point 0, always signalled, before the first).
    pub fn last(&self) -> Fence {
        Fence { syncobj: self.timeline, point: self.submitted }
    }

    /// Submits one compute command running the CDM stream at `[base, end)` after `wait_for` (fences of other
    /// queues) and the earlier commands of this queue. Does not wait. Its timestamps stay readable with `timing`
    /// until `TIMESTAMP_SLOTS` more commands are submitted.
    pub fn submit_compute(&mut self, cdm_base: u64, cdm_end: u64, wait_for: &[Fence]) -> io::Result<Fence> {
        self.submit_compute_with_helper(cdm_base, cdm_end, None, wait_for)
    }

    /// `submit_compute` with a helper program: `(binary, data)` of `Vm::scratch` for a shader that uses the stack.
    pub fn submit_compute_with_helper(
        &mut self,
        cdm_base: u64,
        cdm_end: u64,
        helper: Option<(u32, u64)>,
        wait_for: &[Fence],
    ) -> io::Result<Fence> {
        let point = self.submitted + 1;
        let slot = (point % TIMESTAMP_SLOTS) as u32 * 16;
        self.ts.as_mut_slice()[slot as usize..slot as usize + 16].fill(0);
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
        if let Some((binary, data)) = helper {
            cmd.c.helper = drm_asahi_helper_program { binary, cfg: 0, data };
        }
        cmd.c.ts.start = drm_asahi_timestamp { handle: self.ts_handle, offset: slot };
        cmd.c.ts.end = drm_asahi_timestamp { handle: self.ts_handle, offset: slot + 8 };
        let sync = |f: &Fence| drm_asahi_sync {
            sync_type: DRM_ASAHI_SYNC_TIMELINE_SYNCOBJ as u32,
            handle: f.syncobj,
            timeline_value: f.point,
        };
        let mut syncs: Vec<drm_asahi_sync> = wait_for.iter().filter(|f| f.point != 0).map(sync).collect();
        let in_count = syncs.len() as u32;
        let fence = Fence { syncobj: self.timeline, point };
        syncs.push(sync(&fence));
        let mut sub = drm_asahi_submit {
            syncs: syncs.as_ptr() as u64,
            cmdbuf: &cmd as *const _ as u64,
            flags: 0,
            queue_id: self.id,
            in_sync_count: in_count,
            out_sync_count: 1,
            cmdbuf_size: std::mem::size_of::<Cmd>() as u32,
            pad: 0,
        };
        self.dev.ioctl(DRM_IOCTL_ASAHI_SUBMIT as u64, &mut sub, "submit")?;
        self.submitted = point;
        Ok(fence)
    }

    /// Waits up to `timeout` for `fence`, then fails if the kernel failed its command.
    pub fn wait(&self, fence: Fence, timeout: Duration) -> io::Result<()> {
        if fence.point == 0 {
            return Ok(());
        }
        let mut wait: drm_syncobj_timeline_wait = unsafe { std::mem::zeroed() };
        wait.handles = &fence.syncobj as *const _ as u64;
        wait.points = &fence.point as *const _ as u64;
        wait.count_handles = 1;
        wait.timeout_nsec = monotonic_deadline(timeout);
        // Wait for the fence to exist as well, so a point another thread has not submitted yet is not an error.
        wait.flags = DRM_SYNCOBJ_WAIT_FLAGS_WAIT_FOR_SUBMIT;
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT, &mut wait, "syncobj-wait")?;
        let status = self.fence_status(fence)?;
        if status < 0 {
            return Err(io::Error::other(error::GpuError { fence_status: status, point: Some(fence.point) }));
        }
        // The firmware signals a faulted compute command with success but writes neither of its timestamps (the slot
        // was zeroed at submit), so a command that ended at time 0 failed. Every command of the queue up to `fence` has
        // finished, so all of them are checked, oldest first; commands whose slot was reused are not.
        if fence.syncobj == self.timeline {
            let faulted = first_faulted(self.verified.get(), fence.point, self.submitted, |p| {
                self.timing(Fence { syncobj: self.timeline, point: p })
            });
            self.verified.set(self.verified.get().max(fence.point));
            if let Some(point) = faulted {
                return Err(io::Error::other(error::GpuError { fence_status: -libc::EFAULT, point: Some(point) }));
            }
        }
        Ok(())
    }

    /// Whether `fence` has signalled, without waiting (cuEventQuery / cuStreamQuery).
    pub fn is_done(&self, fence: Fence) -> io::Result<bool> {
        let mut point = 0u64;
        let mut q = drm_syncobj_timeline_array {
            handles: &fence.syncobj as *const _ as u64,
            points: &mut point as *mut _ as u64,
            count_handles: 1,
            flags: 0,
        };
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_QUERY, &mut q, "syncobj-query")?;
        Ok(point >= fence.point)
    }

    /// The firmware timestamps of a finished command, if its slot was not reused.
    pub fn timing(&self, fence: Fence) -> Option<(u64, u64)> {
        if fence.syncobj != self.timeline || fence.point == 0 || self.submitted - fence.point >= TIMESTAMP_SLOTS {
            return None;
        }
        let slot = (fence.point % TIMESTAMP_SLOTS) as usize * 16;
        let t = &self.ts.as_slice()[slot..slot + 16];
        Some((u64::from_le_bytes(t[0..8].try_into().unwrap()), u64::from_le_bytes(t[8..16].try_into().unwrap())))
    }

    /// Copies `bytes` into a ring for the next command to submit, waiting for older commands if the ring is full.
    /// Returns the GPU address.
    pub fn stage(&mut self, kind: RingKind, bytes: &[u8], align: u64) -> io::Result<u64> {
        let point = self.submitted + 1;
        loop {
            let (bo, alloc) = match kind {
                RingKind::Data => &mut self.data_ring,
                RingKind::Usc => &mut self.usc_ring,
            };
            if let Some(at) = alloc.alloc(bytes.len() as u64, align, point) {
                bo.as_mut_slice()[at as usize..at as usize + bytes.len()].copy_from_slice(bytes);
                return Ok(bo.va + at);
            }
            let Some(oldest) = alloc.oldest().filter(|&p| p < point) else {
                return Err(io::Error::new(io::ErrorKind::OutOfMemory, "launch objects larger than the ring"));
            };
            let fence = Fence { syncobj: self.timeline, point: oldest };
            self.wait(fence, self.timeout)?;
            self.retire_rings(oldest);
        }
    }

    fn retire_rings(&mut self, done: u64) {
        self.data_ring.1.retire(done);
        self.usc_ring.1.retire(done);
    }

    /// Frees what `stage` reserved for a command that was not submitted.
    pub fn unstage(&mut self) {
        let point = self.submitted + 1;
        self.data_ring.1.rollback(point);
        self.usc_ring.1.rollback(point);
    }

    /// USC offset of a USC-ring address.
    pub fn usc_offset(&self, va: u64, usc_base: u64) -> u32 {
        (va - usc_base) as u32
    }

    /// Keeps `objects` alive until `fence` (a point of this queue) has signalled.
    pub fn retire_after(&mut self, fence: Fence, objects: Vec<Bo>) {
        debug_assert_eq!(fence.syncobj, self.timeline);
        self.in_flight.push_back((fence.point, objects));
    }

    /// Frees the objects of finished commands. Points finish in order, so this stops at the first unfinished one.
    pub fn collect(&mut self) -> io::Result<()> {
        let rings_busy = self.data_ring.1.oldest().is_some() || self.usc_ring.1.oldest().is_some();
        let last = match self.in_flight.back() {
            Some(&(last, _)) => last,
            None if rings_busy => 0,
            None => return Ok(()),
        };
        let mut done = 0u64;
        let mut q = drm_syncobj_timeline_array {
            handles: &self.timeline as *const _ as u64,
            points: &mut done as *mut _ as u64,
            count_handles: 1,
            flags: 0,
        };
        self.dev.ioctl(DRM_IOCTL_SYNCOBJ_QUERY, &mut q, "syncobj-query")?;
        while self.in_flight.front().is_some_and(|&(p, _)| p <= done.min(last)) {
            self.in_flight.pop_front();
        }
        self.retire_rings(done);
        Ok(())
    }

    /// Waits for every submitted command (cuStreamSynchronize) and frees their objects.
    pub fn synchronize(&mut self) -> io::Result<()> {
        let last = self.last();
        let r = self.wait(last, self.timeout);
        if r.is_ok() {
            self.in_flight.clear();
            self.retire_rings(last.point);
        }
        r
    }

    /// Submits one compute command and waits for it.
    pub fn run_compute(&mut self, cdm_base: u64, cdm_end: u64) -> io::Result<Timing> {
        let h0 = Instant::now();
        let fence = self.submit_compute(cdm_base, cdm_end, &[])?;
        self.wait(fence, self.timeout)?;
        let host = h0.elapsed();
        let (start, end) = self.timing(fence).unwrap();
        Ok(Timing { start, end, host })
    }

    /// The status of a fence: 1 when signalled, negative errno when the kernel failed the command.
    fn fence_status(&self, fence: Fence) -> io::Result<i32> {
        let mut h = drm_syncobj_handle {
            handle: fence.syncobj,
            flags: DRM_SYNCOBJ_HANDLE_TO_FD_FLAGS_EXPORT_SYNC_FILE,
            fd: -1,
            pad: 0,
            point: fence.point,
        };
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
        // The timestamp object was bound with GEM_BIND_OBJECT; a bound object strands its memory (B42).
        let mut unbind = drm_asahi_gem_bind_object {
            op: DRM_ASAHI_BIND_OBJECT_OP_UNBIND as u32,
            flags: 0,
            handle: 0,
            vm_id: 0,
            offset: 0,
            range: 0,
            object_handle: self.ts_handle,
            pad: 0,
        };
        let _ = self.dev.ioctl(DRM_IOCTL_ASAHI_GEM_BIND_OBJECT as u64, &mut unbind, "unbind-timestamps");
        let mut qd = drm_asahi_queue_destroy { queue_id: self.id, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_ASAHI_QUEUE_DESTROY as u64, &mut qd, "queue-destroy");
        let mut sd = drm_syncobj_destroy { handle: self.timeline, pad: 0 };
        let _ = self.dev.ioctl(DRM_IOCTL_SYNCOBJ_DESTROY, &mut sd, "syncobj-destroy");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_usc_barrier_is_optional() {
        let with = cdm_dispatch_with(0x1000, 16, false, [32, 1, 1], [32, 1, 1], true);
        let without = cdm_dispatch_with(0x1000, 16, false, [32, 1, 1], [32, 1, 1], false);
        assert_eq!(with, cdm_dispatch(0x1000, 16, false, [32, 1, 1], [32, 1, 1]));
        assert_eq!(with.len(), without.len() + 4);
        let words = |c: &[u8]| c.chunks(4).map(|w| u32::from_le_bytes(w.try_into().unwrap())).collect::<Vec<_>>();
        assert!(words(&with).contains(&(3 << 29 | 0xfffff)));
        assert!(!words(&without).contains(&(3 << 29 | 0xfffff)));
        assert_eq!(words(&without)[words(&without).len() - 2..], [2 << 29, 0], "still terminated");
    }

    #[test]
    fn the_usc_barrier_is_on_unless_turned_off() {
        assert!(usc_barrier_from(None));
        assert!(usc_barrier_from(Some("1")));
        assert!(usc_barrier_from(Some("")));
        assert!(!usc_barrier_from(Some("0")));
    }

    #[test]
    fn queues_wait_without_limit_unless_asked() {
        assert_eq!(wait_timeout_from(None), Duration::MAX);
        assert_eq!(wait_timeout_from(Some("")), Duration::MAX);
        assert_eq!(wait_timeout_from(Some("0")), Duration::MAX);
        assert_eq!(wait_timeout_from(Some("x")), Duration::MAX);
        assert_eq!(wait_timeout_from(Some("2500")), Duration::from_millis(2500));
        assert!(monotonic_deadline(Duration::MAX) == i64::MAX, "no deadline saturates instead of wrapping");
    }

    #[test]
    fn a_fault_before_the_last_command_is_found() {
        // commands 1..=5 finished; 3 faulted
        let t = |p: u64| Some(if p == 3 { (0, 0) } else { (10 * p, 10 * p + 5) });
        assert_eq!(first_faulted(0, 5, 5, t), Some(3));
        // already verified past it: not reported twice
        assert_eq!(first_faulted(3, 5, 5, t), None);
        // the oldest fault is the one reported
        let t2 = |p: u64| Some(if p == 2 || p == 4 { (0, 0) } else { (1, 2) });
        assert_eq!(first_faulted(0, 5, 5, t2), Some(2));
        // only the commands whose slots are still theirs are checked
        let n = TIMESTAMP_SLOTS + 10;
        assert_eq!(first_faulted(0, n, n, |p| Some(if p == 5 { (0, 0) } else { (1, 2) })), None);
        assert_eq!(first_faulted(0, n, n, |p| Some(if p == n - 3 { (0, 0) } else { (1, 2) })), Some(n - 3));
        // waiting for an earlier fence checks only up to it
        assert_eq!(first_faulted(0, 2, 5, t), None);
    }

    #[test]
    fn unstamped_commands_faulted() {
        assert!(job_faulted((0, 0)));
        assert!(!job_faulted((5, 9)));
        let e = io::Error::other(error::GpuError { fence_status: -libc::EFAULT, point: None });
        assert_eq!(error::cuda_error(&e), 719);
    }

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
        let p = Pipeline { code: 0x2000, gpr_halves: 8, uniforms: vec![(8, 8, 0x11_0000_0000)], shared_bytes: 0, spill_bucket: 0 };
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

    // Mesa's Registers word for its spilling dispatch: 24 halves, scratch bucket 5 (B1 trace).
    #[test]
    fn registers_word_carries_the_scratch_bucket() {
        let p = Pipeline { code: 0, gpr_halves: 24, uniforms: vec![], shared_bytes: 0, spill_bucket: 5 };
        assert_eq!(hex(&p.usc_words()[10..14]), "8d031401");
    }

    // Shared word as built by the simulator's run_drm_tg_layout.py.
    #[test]
    fn shared_word_with_threadgroup_memory() {
        let p = Pipeline { code: 0, gpr_halves: 256, uniforms: vec![], shared_bytes: 4096, spill_bucket: 0 };
        let w = p.usc_words();
        assert_eq!(u32::from_le_bytes(w[0..4].try_into().unwrap()), 0x4d | 1 << 8 | 0x24 << 10 | 16 << 24);
        // 256 halves is register group field 0
        assert_eq!(&w[10..14], &(0x8du32 | 1 << 24).to_le_bytes());
    }

    #[test]
    fn long_uniform_ranges_are_split() {
        let p = Pipeline { code: 0, gpr_halves: 8, uniforms: vec![(0, 80, 0x1000)], shared_bytes: 0, spill_bucket: 0 };
        let w = p.usc_words();
        let a = u64::from_le_bytes(w[0..8].try_into().unwrap());
        let b = u64::from_le_bytes(w[8..16].try_into().unwrap());
        // 64 halves is size field 0
        assert_eq!((a & 0xff, a >> 8 & 0xff, a >> 20 & 0x3f, a >> 26), (0x1d, 0, 0, 0x1000 >> 2));
        assert_eq!((b & 0xff, b >> 8 & 0xff, b >> 20 & 0x3f, b >> 26), (0x1d, 64, 16, (0x1000 + 128) >> 2));
        assert_eq!(p.uniform_halves(), 80);
    }

    // Halves 256-511 use Uniform High, as Mesa's agx_usc_uniform; a range does not cross half 256 in one word.
    #[test]
    fn high_uniforms_use_uniform_high() {
        let p = Pipeline { code: 0, gpr_halves: 8, uniforms: vec![(240, 40, 0x1000)], shared_bytes: 0, spill_bucket: 0 };
        let w = p.usc_words();
        let a = u64::from_le_bytes(w[0..8].try_into().unwrap());
        let b = u64::from_le_bytes(w[8..16].try_into().unwrap());
        assert_eq!((a & 0xff, a >> 8 & 0xff, a >> 20 & 0x3f, a >> 26), (0x1d, 240, 16, 0x1000 >> 2));
        assert_eq!((b & 0xff, b >> 8 & 0xff, b >> 20 & 0x3f, b >> 26), (0x3d, 0, 24, (0x1000 + 32) >> 2));
        // The whole file is 8 groups of 64 halves, field value 0.
        let c = cdm_dispatch(0x1000, 512, false, [32, 1, 1], [32, 1, 1]);
        assert_eq!(u32::from_le_bytes(c[0..4].try_into().unwrap()), 0);
    }

    #[test]
    fn generic_drm_ioctl_numbers() {
        assert_eq!(DRM_IOCTL_GEM_CLOSE, 0x4008_6409);
        assert_eq!(DRM_IOCTL_SYNCOBJ_CREATE, 0xc008_64bf);
        assert_eq!(DRM_IOCTL_SYNCOBJ_HANDLE_TO_FD, 0xc018_64c1);
        assert_eq!(DRM_IOCTL_SYNCOBJ_TIMELINE_WAIT, 0xc030_64ca);
        assert_eq!(DRM_IOCTL_SYNCOBJ_QUERY, 0xc018_64cb);
        assert_eq!(SYNC_IOC_FILE_INFO, 0xc038_3e04);
    }
}

fn invalid(m: String) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, m)
}

/// The uniforms of a `launch_params_async` of `k` over hardware `threadgroups` with the parameter block at
/// `param_block`, and the block's contents.
fn uniforms(
    k: &module::KernelInfo,
    params: &[&[u8]],
    threadgroups: [u32; 3],
    dynamic_shared: u32,
    bounds: &[(u64, u64)],
    table: (u64, u32),
    globals: Option<u64>,
    record: Option<u64>,
    param_block: u64,
) -> io::Result<(Vec<u16>, Vec<u8>)> {
    let (mut halves, block) = module::pack_params(k, params, threadgroups, globals, param_block).map_err(|e| invalid(e.to_string()))?;
    if k.checked.is_some() {
        module::pack_checked(k, &mut halves, record.unwrap_or(0), k.threadgroup_bytes + dynamic_shared, bounds, table)
            .map_err(|e| invalid(e.to_string()))?;
    }
    Ok((halves, block))
}

/// A launch of `module` as the hardware sees it: the kernel's code, the hardware grid and threadgroup sizes, and
/// the uniforms with the parameter block (if any) at `param_block`, the module's global data and error record at
/// the given addresses, and a checked kernel's allocation table (`allocations`) at `table_va`. Other memory is the
/// caller's to save.
pub fn replay(
    module: &module::Module,
    globals: Option<(u64, &[u8])>,
    record: Option<(u64, &[u8])>,
    kernel: &str,
    params: &[&[u8]],
    threadgroups: [u32; 3],
    local: [u32; 3],
    dynamic_shared: u32,
    bounds: &[(u64, u64)],
    allocations: &[(u64, u64)],
    table_va: u64,
    param_block: u64,
) -> io::Result<Replay> {
    let k = module.kernels.get(kernel).ok_or_else(|| invalid(format!("no kernel {kernel}")))?;
    let (threadgroups, local) = k.hardware_launch(threadgroups, local).map_err(|e| invalid(e.to_string()))?;
    let table = k.checked.as_ref().and_then(|c| c.table).map(|_| module::checked_table(allocations)).filter(|t| !t.is_empty());
    let table_arg = table.as_ref().map_or((0, 0), |t| (table_va, (t.len() / 16) as u32));
    let (uniforms, param_bytes) = uniforms(
        k,
        params,
        threadgroups,
        dynamic_shared,
        bounds,
        table_arg,
        globals.map(|g| g.0),
        record.map(|r| r.0),
        param_block,
    )?;
    let code = module.text[k.offset as usize..(k.offset + k.size) as usize].to_vec();
    let mut regions: Vec<(u64, Vec<u8>)> = Vec::new();
    if !param_bytes.is_empty() {
        regions.push((param_block, param_bytes));
    }
    regions.extend(globals.into_iter().chain(record).map(|(va, b)| (va, b.to_vec())));
    regions.extend(table.map(|t| (table_va, t)));
    Ok(Replay {
        code,
        grid: std::array::from_fn(|i| threadgroups[i] * local[i]),
        local,
        shared_bytes: k.threadgroup_bytes + dynamic_shared,
        stack_bytes: k.stack_bytes,
        uniforms,
        regions,
        text_offset: k.offset as u64,
        debug_object: module.debug_object.clone(),
    })
}

/// A launch as the hardware sees it, from [`replay`].
#[derive(Clone, Debug)]
pub struct Replay {
    pub code: Vec<u8>,
    /// Threads per dimension.
    pub grid: [u32; 3],
    pub local: [u32; 3],
    pub shared_bytes: u32,
    pub stack_bytes: u32,
    /// Uniform registers, in 16-bit halves.
    pub uniforms: Vec<u16>,
    /// GPU address and contents of the memory the module owns: the parameter block, the global data buffer and
    /// the error record.
    pub regions: Vec<(u64, Vec<u8>)>,
    /// Where the kernel's code starts in the module's `.text`, and the module's object if it has a line table.
    pub text_offset: u64,
    pub debug_object: Option<Vec<u8>>,
}

impl Replay {
    /// The `agx_run` arguments of the launch, with each region `i` of `regions` (the module's and the caller's)
    /// read from `<prefix><i>.bin` and dumped to `<prefix><i>.out`, the code from `code.bin`.
    pub fn agx_run_args(&self, regions: &[(u64, Vec<u8>)], prefix: &str) -> Vec<String> {
        let d = |v: [u32; 3]| format!("{},{},{}", v[0], v[1], v[2]);
        let mut a = vec!["code.bin".to_string(), "--grid".into(), d(self.grid), "--wg".into(), d(self.local)];
        if self.shared_bytes > 0 {
            a.extend(["--shared".into(), self.shared_bytes.to_string()]);
        }
        for (i, pair) in self.uniforms.chunks(2).enumerate() {
            let v = pair[0] as u32 | (*pair.get(1).unwrap_or(&0) as u32) << 16;
            if v != 0 {
                a.extend(["--uniform".into(), format!("{i}={v}")]);
            }
        }
        for (i, (va, bytes)) in regions.iter().enumerate() {
            a.extend(["--map".into(), format!("{va:#x}={prefix}{i}.bin")]);
            a.extend(["--dump".into(), format!("{va:#x}={}:{prefix}{i}.out", bytes.len())]);
        }
        if self.debug_object.is_some() {
            a.extend(["--lines".into(), "module.o".into(), "--lines-offset".into(), self.text_offset.to_string()]);
        }
        a
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
    /// The error record of the module's checked kernels (zeroed; the first bad access fills it).
    record: Option<Bo>,
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
            let init = module.relocated_globals(g.va);
            g.as_mut_slice()[..init.len()].copy_from_slice(&init);
            Some(g)
        };
        let record = if module.kernels.values().any(|k| k.checked.is_some()) {
            let mut r = vm.alloc_data(module::CHECKED_RECORD_BYTES)?;
            r.as_mut_slice()[..module::CHECKED_RECORD_BYTES as usize].fill(0);
            Some(r)
        } else {
            None
        };
        Ok(LoadedModule { module, code, entries, globals, record })
    }

    /// Whether the module has kernels built in checked mode.
    pub fn checked(&self) -> bool {
        self.record.is_some()
    }

    /// The bad access a checked kernel of the module reported, if any. Call it after the launches have finished.
    /// The record stays set, and every later checked launch of the module returns at once, until
    /// `reset_checked`.
    pub fn checked_error(&self) -> Option<module::CheckedError> {
        let r = self.record.as_ref()?;
        let sites = self.module.global_symbols.get(module::CHECKED_SITES_SYMBOL).map_or(String::new(), |&(off, size)| {
            let t = self.globals().get(off as usize..(off + size) as usize).unwrap_or_default();
            String::from_utf8_lossy(&t[..t.iter().position(|&c| c == 0).unwrap_or(t.len())]).into_owned()
        });
        module::CheckedError::decode(&r.as_slice()[..module::CHECKED_RECORD_BYTES as usize], &sites)
    }

    /// Clears the error record (a new context).
    pub fn reset_checked(&mut self) {
        if let Some(r) = self.record.as_mut() {
            r.as_mut_slice()[..module::CHECKED_RECORD_BYTES as usize].fill(0);
        }
    }

    /// The current contents of the global data buffer (device variables).
    pub fn globals(&self) -> &[u8] {
        self.globals.as_ref().map_or(&[], |g| &g.as_slice()[..self.module.globals.len()])
    }

    /// GPU address of the global data buffer, if the module has one.
    pub fn globals_va(&self) -> Option<u64> {
        self.globals.as_ref().map(|g| g.va)
    }

    /// Whether a device `assert` failed since the last call (CUDA_ERROR_ASSERT); clears the flag.
    pub fn take_assert_failure(&mut self) -> bool {
        let Some(&(off, _)) = self.module.global_symbols.get(printf::TRAP_FLAG_SYMBOL) else { return false };
        let Some(g) = self.globals.as_mut() else { return false };
        let w = &mut g.as_mut_slice()[off as usize..off as usize + 4];
        let failed = w != [0; 4];
        w.fill(0);
        failed
    }

    /// Formats and clears the device `printf` records written since the last drain. Strings outside the module's
    /// global data buffer (`%s` of a device buffer) are read through `read_other`. `None` if the module does not use
    /// `printf`. Call it after the launches that printed have finished.
    pub fn drain_printf(&mut self, read_other: impl Fn(u64) -> Option<Vec<u8>>) -> Option<printf::Drained> {
        let &(off, size) = self.module.global_symbols.get(printf::BUFFER_SYMBOL)?;
        let g = self.globals.as_mut()?;
        let (va, data) = (g.va, &g.as_slice()[..self.module.globals.len()]);
        let read_str = |addr: u64| -> Option<Vec<u8>> {
            if addr >= va && addr < va + data.len() as u64 {
                let s = &data[(addr - va) as usize..];
                Some(s[..s.iter().position(|&c| c == 0)?].to_vec())
            } else {
                read_other(addr)
            }
        };
        let out = printf::drain(data.get(off as usize..(off + size) as usize)?, read_str);
        printf::reset(&mut g.as_mut_slice()[off as usize..(off + size) as usize]);
        Some(out)
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
        let h0 = Instant::now();
        let fence = self.launch_async(vm, queue, g14x, kernel, args, threadgroups, local, dynamic_shared, &[])?;
        queue.wait(fence, queue.timeout)?;
        let host = h0.elapsed();
        let (start, end) = queue.timing(fence).unwrap_or_default();
        queue.collect()?;
        Ok(Timing { start, end, host })
    }

    /// Submits `kernel` on `queue` after `wait_for` and returns its fence (cuLaunchKernel). The arguments are copied
    /// now; the launch's command objects are freed once it has finished.
    pub fn launch_async(
        &self,
        vm: &mut Vm,
        queue: &mut Queue,
        g14x: bool,
        kernel: &str,
        args: &[module::Arg],
        threadgroups: [u32; 3],
        local: [u32; 3],
        dynamic_shared: u32,
        wait_for: &[Fence],
    ) -> io::Result<Fence> {
        queue.collect()?;
        let k = self.kernel(kernel, dynamic_shared)?;
        let (threadgroups, local) = k.hardware_launch(threadgroups, local).map_err(|e| invalid(e.to_string()))?;
        let halves = module::pack_uniforms(k, args, threadgroups, self.globals_va()).map_err(|e| invalid(e.to_string()))?;
        self.dispatch(vm, queue, g14x, k, &halves, Vec::new(), threadgroups, local, dynamic_shared, wait_for)
    }

    /// `launch_async` with the bytes of each source-level argument, as cuLaunchKernel's `kernelParams` point to
    /// them. Needs a module whose object has `.agx.kernels`. Arguments the kernel reads through a pointer are
    /// copied to a parameter block that lives until the launch has finished. For a checked kernel, `bounds` holds
    /// the `[lo, hi)` range of the allocation each of its checked pointers (`module::checked_pointers`) points
    /// into, and `allocations` every live allocation (staged as the allocation table of a kernel that has one);
    /// both are empty otherwise.
    pub fn launch_params_async(
        &self,
        vm: &mut Vm,
        queue: &mut Queue,
        g14x: bool,
        kernel: &str,
        params: &[&[u8]],
        threadgroups: [u32; 3],
        local: [u32; 3],
        dynamic_shared: u32,
        bounds: &[(u64, u64)],
        allocations: &[(u64, u64)],
        wait_for: &[Fence],
    ) -> io::Result<Fence> {
        queue.collect()?;
        let k = self.kernel(kernel, dynamic_shared)?;
        let (threadgroups, local) = k.hardware_launch(threadgroups, local).map_err(|e| invalid(e.to_string()))?;
        // the allocation table lives in the data ring until the launch has finished; a table too large for the
        // ring is left out (count 0: the kernel checks those pointers for null only)
        let mut table = (0, 0);
        if k.checked.as_ref().is_some_and(|c| c.table.is_some()) && !allocations.is_empty() {
            let bytes = module::checked_table(allocations);
            if (bytes.len() as u64) <= DATA_RING_BYTES / 4 {
                table = (queue.stage(RingKind::Data, &bytes, 16)?, allocations.len() as u32);
            }
        }
        let (mut halves, block) = self.uniforms(k, params, threadgroups, dynamic_shared, bounds, table, 0)?;
        if !block.is_empty() {
            // the pointers in the uniforms depend on where the block lands
            let va = queue.stage(RingKind::Data, &block, 16)?;
            halves = self.uniforms(k, params, threadgroups, dynamic_shared, bounds, table, va)?.0;
        }
        self.dispatch(vm, queue, g14x, k, &halves, Vec::new(), threadgroups, local, dynamic_shared, wait_for)
    }

    fn uniforms(
        &self,
        k: &module::KernelInfo,
        params: &[&[u8]],
        threadgroups: [u32; 3],
        dynamic_shared: u32,
        bounds: &[(u64, u64)],
        table: (u64, u32),
        param_block: u64,
    ) -> io::Result<(Vec<u16>, Vec<u8>)> {
        let record = self.record.as_ref().map(|r| r.va);
        uniforms(k, params, threadgroups, dynamic_shared, bounds, table, self.globals_va(), record, param_block)
    }

    /// What a launch hands the hardware, for replaying it elsewhere (the AGX Performance Simulator's `agx_run`):
    /// see [`replay`].
    pub fn replay(
        &self,
        kernel: &str,
        params: &[&[u8]],
        threadgroups: [u32; 3],
        local: [u32; 3],
        dynamic_shared: u32,
        bounds: &[(u64, u64)],
        allocations: &[(u64, u64)],
        table_va: u64,
        param_block: u64,
    ) -> io::Result<Replay> {
        let globals = self.globals_va().map(|va| (va, self.globals()));
        let record = self.record.as_ref().map(|r| (r.va, &r.as_slice()[..module::CHECKED_RECORD_BYTES as usize]));
        replay(
            &self.module,
            globals,
            record,
            kernel,
            params,
            threadgroups,
            local,
            dynamic_shared,
            bounds,
            allocations,
            table_va,
            param_block,
        )
    }

    fn kernel(&self, kernel: &str, dynamic_shared: u32) -> io::Result<&module::KernelInfo> {
        let k = self.module.kernels.get(kernel).ok_or_else(|| invalid(format!("no kernel {kernel}")))?;
        if k.threadgroup_bytes + dynamic_shared > 65536 {
            return Err(invalid(format!("{kernel} needs {} bytes of threadgroup memory", k.threadgroup_bytes + dynamic_shared)));
        }
        Ok(k)
    }

    fn dispatch(
        &self,
        vm: &mut Vm,
        queue: &mut Queue,
        g14x: bool,
        k: &module::KernelInfo,
        halves: &[u16],
        keep: Vec<Bo>,
        threadgroups: [u32; 3],
        local: [u32; 3],
        dynamic_shared: u32,
        wait_for: &[Fence],
    ) -> io::Result<Fence> {
        let r = (|| {
            // a shader that uses the stack gets its scratch from the helper program
            let stack_dwords = k.stack_bytes.div_ceil(4);
            let helper = if stack_dwords > 0 { Some(vm.scratch(stack_dwords)?) } else { None };
            let bytes: Vec<u8> = halves.iter().flat_map(|h| h.to_le_bytes()).collect();
            let unif = if bytes.is_empty() { 0 } else { queue.stage(RingKind::Data, &bytes, 64)? };
            let pipeline = Pipeline {
                code: self.entries[&k.name],
                gpr_halves: k.gpr_halves(),
                uniforms: if halves.is_empty() { vec![] } else { vec![(0, halves.len() as u32, unif)] },
                shared_bytes: k.threadgroup_bytes + dynamic_shared,
                spill_bucket: if stack_dwords > 0 { scratch::Layout::new(stack_dwords).bucket } else { 0 },
            };
            // The Shader word's code address has only been measured 64-byte aligned; pipeline words likewise.
            let pw = queue.stage(RingKind::Usc, &pipeline.usc_words(), 64)?;
            let global = std::array::from_fn(|i| threadgroups[i] * local[i]);
            // The USC cache is invalidated on every command, or with ZLUDA_AGX_USC_BARRIER=0 only after new code
            // was allocated (a module load).
            let barrier = queue.usc_seen.get() != vm.usc_generation || always_usc_barrier();
            queue.usc_seen.set(vm.usc_generation);
            let stream =
                cdm_dispatch_with((pw - vm.usc_base) as u32, pipeline.uniform_halves(), g14x, global, local, barrier);
            let cdm = queue.stage(RingKind::Data, &stream, 64)?;
            queue.submit_compute_with_helper(cdm, cdm + stream.len() as u64, helper, wait_for)
        })();
        let fence = r.inspect_err(|_| queue.unstage())?;
        if !keep.is_empty() {
            queue.retire_after(fence, keep);
        }
        Ok(fence)
    }
}
