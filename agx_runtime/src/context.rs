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
            // Metal's limit; the CDM launch words take larger local sizes, unmeasured.
            max_threads_per_block: 1024,
            max_block_dim: [1024, 1024, 64],
            // The CDM global size words are 32-bit thread counts.
            max_grid_dim: [u32::MAX / 1024, 65535, 65535],
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

/// The allocation containing `[va, va + len)`, by its start address.
fn find<T>(allocs: &BTreeMap<u64, T>, va: u64, len: u64, size: impl Fn(&T) -> u64) -> Option<u64> {
    let (&start, a) = allocs.range(..=va).next_back()?;
    (va.checked_add(len)? <= start + size(a)).then_some(start)
}

pub struct Context {
    pub device: Device,
    pub props: DeviceProps,
    pub vm: Vm,
    pub stream: Queue,
    allocs: BTreeMap<u64, (Bo, u64)>,
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
        self.allocs.insert(va, (bo, size.max(1)));
        Ok(va)
    }

    /// cuMemFree: waits for the default stream first, as CUDA does.
    pub fn mem_free(&mut self, va: u64) -> io::Result<()> {
        if !self.allocs.contains_key(&va) {
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
        self.allocs.insert(va, (bo, size.max(1)));
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

    /// cuMemGetInfo: (free, total). The GPU shares system memory, so free is the system's available memory.
    pub fn mem_info(&self) -> (u64, u64) {
        let mut s: libc::sysinfo = unsafe { std::mem::zeroed() };
        let free = if unsafe { libc::sysinfo(&mut s) } == 0 { s.freeram as u64 * s.mem_unit as u64 } else { 0 };
        (free, self.props.total_memory)
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
