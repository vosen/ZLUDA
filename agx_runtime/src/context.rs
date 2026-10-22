//! A CUDA-style context on one AGX device: its GPU address space, a default stream, device allocations
//! (`cuMemAlloc` / `cuMemFree`), copies and fills, and the device attributes `cuDeviceGetAttribute` reports.
//!
//! Apple GPUs share memory with the CPU and the buffer objects are mapped write-back, so copies are CPU copies of the
//! mapped bytes after the work that might touch them has finished. CPU writes made while a kernel runs are only
//! guaranteed to be visible to later launches (ptx-re-blockers B7).

use crate::*;
use std::collections::BTreeMap;

/// Device attributes derived from the kernel's parameters and the measured G14 limits.
#[derive(Clone, Debug, PartialEq)]
pub struct DeviceProps {
    pub name: String,
    pub llvm_cpu: String,
    pub cores: u32,
    pub clock_khz: u32,
    pub warp_size: u32,
    pub max_threads_per_block: u32,
    pub max_block_dim: [u32; 3],
    pub max_grid_dim: [u32; 3],
    pub shared_memory_per_block: u32,
    pub registers_per_thread: u32,
    pub total_memory: u64,
}

impl DeviceProps {
    pub fn from_params(p: &drm_asahi_params_global, total_memory: u64) -> DeviceProps {
        let variant = (p.gpu_variant as u8 as char).to_ascii_uppercase();
        let cores = p.core_masks.iter().map(|m| m.count_ones()).sum();
        let marketing = match (p.gpu_generation, variant) {
            (13, 'G') => "Apple M1",
            (13, 'S') => "Apple M1 Pro",
            (13, 'C') => "Apple M1 Max",
            (13, 'D') => "Apple M1 Ultra",
            (14, 'G') => "Apple M2",
            (14, 'S') => "Apple M2 Pro",
            (14, 'C') => "Apple M2 Max",
            (14, 'D') => "Apple M2 Ultra",
            _ => "Apple GPU",
        };
        DeviceProps {
            name: format!("{marketing} (G{}{variant}, {cores} cores)", p.gpu_generation),
            llvm_cpu: format!("agx-g{}{}", p.gpu_generation, variant.to_ascii_lowercase()),
            cores,
            clock_khz: p.max_frequency_khz,
            warp_size: 32,
            // Measured on the M2 Max (AGX Performance Simulator tasks/ptx-re-blockers.md, B22): threadgroups of 1024
            // threads along x, y or z run; a 2048-thread threadgroup runs nothing.
            max_threads_per_block: 1024,
            max_block_dim: [1024, 1024, 1024],
            // The CDM global size words are 32-bit thread counts per dimension (2^32 - 32 threads along x run), so a
            // launch also needs grid * block < 2^32 in each dimension. y and z take more than CUDA's 65535 CTAs.
            max_grid_dim: [i32::MAX as u32, 65535, 65535],
            shared_memory_per_block: 65536,
            registers_per_thread: 128,
            total_memory,
        }
    }
}

fn system_memory() -> u64 {
    let mut s: libc::sysinfo = unsafe { std::mem::zeroed() };
    if unsafe { libc::sysinfo(&mut s) } != 0 {
        return 0;
    }
    s.totalram as u64 * s.mem_unit as u64
}

/// The memory limit of the process's cgroup (v2 memory.max) and its current use, if it has a limit.
fn cgroup_memory() -> Option<(u64, u64)> {
    let cgroup = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let path = cgroup.lines().find_map(|l| l.strip_prefix("0::"))?;
    let dir = std::path::Path::new("/sys/fs/cgroup").join(path.trim_start_matches('/'));
    let max = std::fs::read_to_string(dir.join("memory.max")).ok()?.trim().parse::<u64>().ok()?;
    let current = std::fs::read_to_string(dir.join("memory.current")).ok()?.trim().parse::<u64>().unwrap_or(0);
    Some((max, current))
}

/// The allocation containing `[va, va + len)`, by its start address.
fn find<T>(allocs: &BTreeMap<u64, T>, va: u64, len: u64, size: impl Fn(&T) -> u64) -> Option<u64> {
    let (&start, a) = allocs.range(..=va).next_back()?;
    (va.checked_add(len)? <= start + size(a)).then_some(start)
}

/// The memory of an allocation: an object of its own, or part of a physical allocation mapped at the address
/// (the virtual memory management API: cuMemCreate, cuMemMap).
enum Memory {
    Bo(Bo),
    // the mapping first: fields drop in order, and the binding goes before the object can
    Mapped { mapping: crate::Mapping, phys: std::rc::Rc<Bo>, offset: u64 },
}

impl Memory {
    fn unified(&self) -> bool {
        match self {
            Memory::Bo(bo) => bo.unified(),
            Memory::Mapped { .. } => false,
        }
    }

    fn as_slice(&self) -> &[u8] {
        match self {
            Memory::Bo(bo) => bo.as_slice(),
            Memory::Mapped { phys, offset, mapping } => unsafe {
                std::slice::from_raw_parts(phys.map.add(*offset as usize), mapping.size as usize)
            },
        }
    }

    fn as_mut_slice(&mut self) -> &mut [u8] {
        match self {
            Memory::Bo(bo) => bo.as_mut_slice(),
            // the physical allocation's pages, which other mappings may also show
            Memory::Mapped { phys, offset, mapping } => unsafe {
                std::slice::from_raw_parts_mut(phys.map.add(*offset as usize), mapping.size as usize)
            },
        }
    }
}

/// Fields drop in declaration order: the allocations and the queue are unbound before the VM is destroyed (a VM
/// destroyed with objects still bound strands their memory in the kernel, also past process exit; B42).
pub struct Context {
    allocs: BTreeMap<u64, (Memory, u64)>,
    pub stream: Queue,
    pub vm: Vm,
    pub props: DeviceProps,
    pub device: Device,
}

fn invalid(m: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, m.into())
}

impl Context {
    pub fn new(node: &str) -> io::Result<Context> {
        let device = Device::open(node)?;
        let props = DeviceProps::from_params(&device.params, system_memory());
        let mut vm = Vm::new(&device)?;
        let stream = Queue::new(&mut vm)?;
        Ok(Context { device, props, vm, stream, allocs: BTreeMap::new() })
    }

    /// cuMemAlloc: `size` bytes, zeroed, 16 KiB aligned.
    pub fn mem_alloc(&mut self, size: u64) -> io::Result<u64> {
        let bo = self.vm.alloc_data(size)?;
        let va = bo.va;
        self.allocs.insert(va, (Memory::Bo(bo), size.max(1)));
        Ok(va)
    }

    /// cuMemMap: `size` bytes of the physical allocation `phys` from `offset`, at `va` (in a range from
    /// `vm.reserve`).
    pub fn mem_map(&mut self, va: u64, phys: &std::rc::Rc<Bo>, offset: u64, size: u64) -> io::Result<()> {
        if find(&self.allocs, va, 0, |a| a.1).is_some() || find(&self.allocs, va + size.max(1) - 1, 0, |a| a.1).is_some()
            || self.allocs.range(va..va + size).next().is_some()
        {
            return Err(invalid(format!("{va:#x}+{size} overlaps a mapping")));
        }
        let mapping = self.vm.map_at(phys, offset, va, size)?;
        self.allocs.insert(va, (Memory::Mapped { mapping, phys: phys.clone(), offset }, size));
        Ok(())
    }

    /// cuMemUnmap: the mappings in `[va, va + size)` (each wholly inside it), after the default stream.
    pub fn mem_unmap(&mut self, va: u64, size: u64) -> io::Result<()> {
        let starts: Vec<u64> = self.allocs.range(va..va + size).map(|(&s, _)| s).collect();
        if starts.is_empty()
            || starts.iter().any(|s| !matches!(self.allocs[s].0, Memory::Mapped { .. }) || s + self.allocs[s].1 > va + size)
        {
            return Err(invalid(format!("{va:#x}+{size} is not a range of mappings")));
        }
        self.stream.synchronize()?;
        for s in starts {
            self.allocs.remove(&s);
        }
        Ok(())
    }

    /// The physical allocation mapped at `va`, if any (cuMemRetainAllocationHandle).
    pub fn mapped_allocation(&self, va: u64) -> Option<&std::rc::Rc<Bo>> {
        let start = find(&self.allocs, va, 0, |a| a.1)?;
        match &self.allocs[&start].0 {
            Memory::Mapped { phys, .. } => Some(phys),
            Memory::Bo(_) => None,
        }
    }

    /// The physical allocation mapped at `va` and the offset of `va` in it (cuMulticastBindAddr).
    pub fn mapped_location(&self, va: u64) -> Option<(std::rc::Rc<Bo>, u64)> {
        let start = find(&self.allocs, va, 0, |a| a.1)?;
        match &self.allocs[&start].0 {
            Memory::Mapped { phys, offset, .. } => Some((phys.clone(), offset + (va - start))),
            Memory::Bo(_) => None,
        }
    }

    /// Takes `bo` (bound) as an allocation of `size` bytes at its address, which mem_free releases: an imported
    /// dma-buf (cuIpcOpenMemHandle), registered host memory (cuMemHostRegister).
    pub fn insert_bo(&mut self, bo: Bo, size: u64) -> u64 {
        let va = bo.va;
        self.allocs.insert(va, (Memory::Bo(bo), size.max(1)));
        va
    }

    /// A dma-buf of the object behind `va`, with the offset of `va` in it and its size (cuIpcGetMemHandle).
    pub fn export_dmabuf(&self, va: u64) -> io::Result<(std::os::fd::OwnedFd, u64, u64)> {
        let start = find(&self.allocs, va, 0, |a| a.1).ok_or_else(|| invalid(format!("{va:#x} is not allocated")))?;
        match &self.allocs[&start].0 {
            Memory::Bo(bo) => Ok((bo.export_dmabuf()?, va - start, bo.size)),
            Memory::Mapped { phys, offset, .. } => Ok((phys.export_dmabuf()?, offset + (va - start), phys.size)),
        }
    }

    /// cuMemFree: waits for the default stream first, as CUDA does.
    pub fn mem_free(&mut self, va: u64) -> io::Result<()> {
        if !self.allocs.get(&va).is_some_and(|a| matches!(a.0, Memory::Bo(_))) {
            return Err(invalid(format!("{va:#x} is not an allocation")));
        }
        self.stream.synchronize()?;
        self.allocs.remove(&va);
        Ok(())
    }

    /// cuMemAllocManaged: an allocation whose GPU address is also its CPU address. Fails if that CPU range is taken.
    pub fn mem_alloc_unified(&mut self, size: u64) -> io::Result<u64> {
        let bo = self.vm.alloc_data(size)?;
        if !bo.unified() {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "the CPU range of the GPU address is in use"));
        }
        let va = bo.va;
        self.allocs.insert(va, (Memory::Bo(bo), size.max(1)));
        Ok(va)
    }

    /// Whether `va` is in an allocation that the CPU can use at the same address.
    pub fn is_unified(&self, va: u64) -> bool {
        find(&self.allocs, va, 0, |a| a.1).is_some_and(|s| self.allocs[&s].0.unified())
    }

    /// The CPU address of `len` bytes of device memory at `va`, for copies the caller does itself (2D copies).
    /// Valid until the allocation is freed.
    pub fn cpu_ptr(&mut self, va: u64, len: u64) -> io::Result<*mut u8> {
        Ok(self.bytes_mut(va, len)?.as_mut_ptr())
    }

    /// Waits for the default stream (before the CPU touches memory the GPU may use).
    pub fn synchronize(&mut self) -> io::Result<()> {
        self.stream.synchronize()
    }

    /// The current contents of the allocation `va` points into, and its base, without waiting for the GPU (a dump
    /// after a fault).
    pub fn allocation(&self, va: u64) -> Option<(u64, &[u8])> {
        let start = find(&self.allocs, va, 0, |a| a.1)?;
        let (bo, size) = &self.allocs[&start];
        Some((start, &bo.as_slice()[..*size as usize]))
    }

    /// `[lo, hi)` of every allocation, in address order.
    pub fn allocation_ranges(&self) -> Vec<(u64, u64)> {
        self.allocs.iter().map(|(&va, (_, size))| (va, va + size)).collect()
    }

    /// The allocation's base and size for a pointer into it (cuMemGetAddressRange).
    pub fn address_range(&self, va: u64) -> Option<(u64, u64)> {
        let start = find(&self.allocs, va, 0, |a| a.1)?;
        Some((start, self.allocs[&start].1))
    }

    fn bytes(&self, va: u64, len: u64) -> io::Result<&[u8]> {
        let start = find(&self.allocs, va, len, |a| a.1).ok_or_else(|| invalid(format!("{va:#x}+{len} not allocated")))?;
        let off = (va - start) as usize;
        Ok(&self.allocs[&start].0.as_slice()[off..off + len as usize])
    }

    fn bytes_mut(&mut self, va: u64, len: u64) -> io::Result<&mut [u8]> {
        let start = find(&self.allocs, va, len, |a| a.1).ok_or_else(|| invalid(format!("{va:#x}+{len} not allocated")))?;
        let off = (va - start) as usize;
        Ok(&mut self.allocs.get_mut(&start).unwrap().0.as_mut_slice()[off..off + len as usize])
    }

    /// cuMemcpyHtoD (synchronous with respect to the default stream).
    pub fn memcpy_htod(&mut self, dst: u64, src: &[u8]) -> io::Result<()> {
        self.stream.synchronize()?;
        self.bytes_mut(dst, src.len() as u64)?.copy_from_slice(src);
        Ok(())
    }

    /// cuMemcpyDtoH.
    pub fn memcpy_dtoh(&mut self, dst: &mut [u8], src: u64) -> io::Result<()> {
        self.stream.synchronize()?;
        dst.copy_from_slice(self.bytes(src, dst.len() as u64)?);
        Ok(())
    }

    /// cuMemcpyDtoD (the regions may be in the same or different allocations, and may overlap).
    pub fn memcpy_dtod(&mut self, dst: u64, src: u64, len: u64) -> io::Result<()> {
        self.stream.synchronize()?;
        let tmp = self.bytes(src, len)?.to_vec();
        self.bytes_mut(dst, len)?.copy_from_slice(&tmp);
        Ok(())
    }

    /// cuMemsetD8 / D16 / D32: `count` copies of `pattern` (1, 2 or 4 bytes).
    pub fn memset(&mut self, dst: u64, pattern: &[u8], count: u64) -> io::Result<()> {
        self.stream.synchronize()?;
        let n = pattern.len() as u64;
        for chunk in self.bytes_mut(dst, count * n)?.chunks_exact_mut(n as usize) {
            chunk.copy_from_slice(pattern);
        }
        Ok(())
    }

    /// Reads a NUL-terminated string from device memory (for `%s` in device `printf`).
    pub fn read_c_string(&self, va: u64) -> Option<Vec<u8>> {
        let start = find(&self.allocs, va, 0, |a| a.1)?;
        let (bo, size) = &self.allocs[&start];
        let s = &bo.as_slice()[(va - start) as usize..*size as usize];
        Some(s[..s.iter().position(|&c| c == 0)?].to_vec())
    }

    /// cuMemGetInfo: (free, total). The GPU shares system memory and the UAPI reports no figures of its own
    /// (ptx-re-blockers B39): free is the system's available memory (MemAvailable, which counts reclaimable
    /// caches), both within the process's cgroup limit if it has one.
    pub fn mem_info(&self) -> (u64, u64) {
        let meminfo = std::fs::read_to_string("/proc/meminfo").unwrap_or_default();
        let field = |name: &str| {
            meminfo
                .lines()
                .find_map(|l| l.strip_prefix(name)?.trim().strip_suffix("kB")?.trim().parse::<u64>().ok())
                .map(|kb| kb * 1024)
        };
        let mut s: libc::sysinfo = unsafe { std::mem::zeroed() };
        let free = field("MemAvailable:").unwrap_or_else(|| {
            if unsafe { libc::sysinfo(&mut s) } == 0 { s.freeram as u64 * s.mem_unit as u64 } else { 0 }
        });
        let mut total = self.props.total_memory;
        let mut free = free.min(total);
        if let Some((limit, used)) = cgroup_memory() {
            total = total.min(limit);
            free = free.min(limit.saturating_sub(used));
        }
        (free, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_allocation_of_a_pointer() {
        let a: BTreeMap<u64, u64> = [(0x1000, 0x100), (0x4000, 0x10)].into();
        let size = |s: &u64| *s;
        assert_eq!(find(&a, 0x1000, 0x100, size), Some(0x1000));
        assert_eq!(find(&a, 0x10ff, 1, size), Some(0x1000));
        assert_eq!(find(&a, 0x10ff, 2, size), None);
        assert_eq!(find(&a, 0x0fff, 1, size), None);
        assert_eq!(find(&a, 0x4008, 8, size), Some(0x4000));
        assert_eq!(find(&a, 0x2000, 0, size), None);
        assert_eq!(find(&a, u64::MAX, 2, size), None);
    }

    #[test]
    fn m2_max_properties() {
        let mut p: drm_asahi_params_global = unsafe { std::mem::zeroed() };
        p.gpu_generation = 14;
        p.gpu_variant = b'C' as u32;
        p.max_frequency_khz = 1_398_000;
        // 38 cores in clusters of 10, 9, 9 and 10
        p.core_masks[..4].copy_from_slice(&[0x3ff, 0x1ff, 0x1ff, 0x3ff]);
        let d = DeviceProps::from_params(&p, 64 << 30);
        assert_eq!(d.name, "Apple M2 Max (G14C, 38 cores)");
        assert_eq!(d.llvm_cpu, "agx-g14c");
        assert_eq!((d.cores, d.clock_khz, d.warp_size), (38, 1_398_000, 32));
    }
}
