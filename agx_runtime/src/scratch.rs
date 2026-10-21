//! Per-thread scratch (stack) memory for kernels that spill or use `.local` memory (AGX Performance Simulator
//! `tasks/ptx-re-blockers.md`, B1; measured on the M2 Max in Apple-M2-Playground
//! `results/2026-10-20-b1-direct-drm-scratch`).
//!
//! The hardware asks a helper program for each SIMD-group's scratch when the group starts and gives it back when it
//! ends. The helper is Mesa's `libagx_helper` (`data/libagx_helper.bin`, compiled by Mesa 25.3.6 from
//! `src/asahi/libagx/helper.cl`, MIT licensed), passed in the command's `helper.binary`. It hands out blocks of one
//! buffer laid out as Mesa's `agx_scratch.c` lays it out, whose header address is `helper.data`:
//!
//! - header: `u32 subgroups`, `u32 0`, then 128 cores of 88 bytes, each `{ u64 block list address, u32 alloc_cur,
//!   alloc_max, alloc_failed, pad, u32 alloc_count[16] }`. Core `c` of cluster `k` is entry
//!   `k * next_pow2(cores per cluster) + c`; absent cores have no block list;
//! - per present core, a block list of `subgroups` entries of 4 block words: `((1 << (log4 + 1)) - 1) | va >> 8` for
//!   the first block and `1 | va >> 8` for the others (0 when unused);
//! - the blocks, aligned to the block size: per core, `subgroups` allocations of `blocks` blocks.
//!
//! A kernel's USC Registers word carries its spill size bucket. Inside an allocation, byte `o` of the stack of lane
//! `l` is at `(adjust - 1 - o / 4) * 128 + 4 * l` for a `stack_adjust` of `adjust` 32-bit words.

/// Mesa's `libagx_helper`, ending in `stop` and its `trap` padding. It is position independent.
pub const HELPER: &[u8] = include_bytes!("data/libagx_helper.bin");

/// SIMD-groups per core with an allocation: Mesa's value, the only one measured (the helper refuses a group when a
/// core has none left, with an effect on the shader that is not known).
pub const SUBGROUPS: u32 = 128;

const HEADER_BYTES: u64 = 8 + 128 * 88;
const CORE_ENTRY_BYTES: u64 = 88;

/// The size classes of a per-thread scratch size.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    /// The Registers word's spill size: 1 + ceil(log2(ceil(dwords / 8))), at most 15.
    pub bucket: u32,
    /// Blocks hold `8 << (2 * log4)` 32-bit words per thread.
    pub log4: u32,
    /// Blocks per allocation, 1 to 4.
    pub blocks: u32,
}

impl Layout {
    /// The layout of `dwords` 32-bit words per thread (1 ..= 131072).
    pub fn new(dwords: u32) -> Layout {
        assert!((1..=(8 << 12) * 4).contains(&dwords));
        let units = dwords.div_ceil(8);
        let bucket = (1 + units.next_power_of_two().trailing_zeros()).min(15);
        let mut log4 = units.ilog2() / 2;
        let mut blocks = dwords.div_ceil(8 << (2 * log4));
        if log4 > 6 {
            log4 -= 1;
            blocks = 4;
        } else if blocks == 4 {
            log4 += 1;
            blocks = 1;
        }
        Layout { bucket, log4, blocks }
    }

    /// The 32-bit words per thread an allocation holds.
    pub fn dwords(&self) -> u64 {
        (8u64 << (2 * self.log4)) * self.blocks as u64
    }

    /// Bytes of one block: 32 lanes of 4-byte slots.
    pub fn block_bytes(&self) -> u64 {
        128 * (8u64 << (2 * self.log4))
    }
}

/// The scratch buffer of a GPU: where each part lies and its initial contents up to the blocks.
#[derive(Clone, Debug)]
pub struct Buffer {
    pub layout: Layout,
    /// Total size in bytes, a multiple of `align`.
    pub size: u64,
    /// The buffer's GPU address must be a multiple of this (the block size, at least a page).
    pub align: u64,
    /// Offset of the blocks; the bytes before it are `header`.
    pub blocks_offset: u64,
}

/// The present cores, as header entries (`cluster * next_pow2(cores_per_cluster) + core`).
pub fn core_entries(core_masks: &[u64], clusters: u32, cores_per_cluster: u32) -> Vec<u32> {
    let stride = cores_per_cluster.max(1).next_power_of_two();
    let mut out = Vec::new();
    for cl in 0..clusters.min(core_masks.len() as u32) {
        for co in 0..cores_per_cluster.min(64) {
            let id = cl * stride + co;
            if core_masks[cl as usize] & 1 << co != 0 && id < 128 {
                out.push(id);
            }
        }
    }
    out
}

impl Buffer {
    pub fn new(layout: Layout, cores: usize, page: u64) -> Buffer {
        let block = layout.block_bytes();
        let blocks_offset = (HEADER_BYTES + SUBGROUPS as u64 * 16 * cores as u64).next_multiple_of(block);
        let per_core = block * layout.blocks as u64 * SUBGROUPS as u64;
        let align = block.max(page);
        let size = (blocks_offset + per_core * cores as u64).next_multiple_of(align);
        Buffer { layout, size, align, blocks_offset }
    }

    /// The header and block lists of the buffer at GPU address `va`, for the cores `entries` (`core_entries`).
    pub fn header(&self, va: u64, entries: &[u32]) -> Vec<u8> {
        let l = self.layout;
        let block = l.block_bytes();
        let mut h = vec![0u8; self.blocks_offset as usize];
        h[0..4].copy_from_slice(&SUBGROUPS.to_le_bytes());
        let mut list = HEADER_BYTES;
        let mut blocks = va + self.blocks_offset;
        for &id in entries {
            let e = (8 + id as u64 * CORE_ENTRY_BYTES) as usize;
            h[e..e + 8].copy_from_slice(&(va + list).to_le_bytes());
            for _ in 0..SUBGROUPS {
                let base = (blocks >> 8) as u32;
                let stride = (block >> 8) as u32;
                let at = list as usize;
                h[at..at + 4].copy_from_slice(&(((1u32 << (l.log4 + 1)) - 1) | base).to_le_bytes());
                for b in 1..4 {
                    let w = if l.blocks > b { 1 | (base + b * stride) } else { 0 };
                    h[at + 4 * b as usize..at + 4 * b as usize + 4].copy_from_slice(&w.to_le_bytes());
                }
                list += 16;
                blocks += block * l.blocks as u64;
            }
        }
        h
    }
}

/// The counters of a core's header entry: (alloc_cur, alloc_max, alloc_failed).
pub fn core_counters(header: &[u8], entry: u32) -> (u32, u32, u32) {
    let e = (8 + entry as u64 * CORE_ENTRY_BYTES + 8) as usize;
    let w = |i: usize| u32::from_le_bytes(header[e + 4 * i..e + 4 * i + 4].try_into().unwrap());
    (w(0), w(1), w(2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layouts_match_the_measured_runs() {
        // 80 dwords: bucket 5, 3 blocks of 4 KiB; 1036: bucket 9, 3 blocks of 64 KiB (B1).
        assert_eq!(Layout::new(80), Layout { bucket: 5, log4: 1, blocks: 3 });
        assert_eq!(Layout::new(80).block_bytes(), 4096);
        assert_eq!(Layout::new(1036), Layout { bucket: 9, log4: 3, blocks: 3 });
        assert_eq!(Layout::new(1036).block_bytes(), 65536);
        // 4 blocks become one of the next size; the largest sizes take 4 blocks
        assert_eq!(Layout::new(120), Layout { bucket: 5, log4: 2, blocks: 1 });
        assert_eq!(Layout::new(8 << 12), Layout { bucket: 13, log4: 6, blocks: 1 });
        assert_eq!(Layout::new(8 << 14), Layout { bucket: 15, log4: 6, blocks: 4 });
        assert_eq!(Layout::new(1), Layout { bucket: 1, log4: 0, blocks: 1 });
        for d in 1..5000 {
            assert!(Layout::new(d).dwords() >= d as u64, "{d}");
        }
    }

    #[test]
    fn sizes_match_the_measured_runs() {
        // 57 MiB and 912 MiB on the 38 cores of the M2 Max
        assert_eq!(Buffer::new(Layout::new(80), 38, 16384).size >> 20, 57);
        assert_eq!(Buffer::new(Layout::new(1036), 38, 16384).size >> 20, 912);
    }

    #[test]
    fn header_matches_the_hardware_run() {
        // The header after the 80-dword run on the M2 Max, at 0x1140000000 (B1, gpu/hdr32.bin): identical but for
        // the counters the helper updated.
        let golden = include_bytes!("data/scratch-header-80-dwords.bin");
        let masks = [0x3ff, 0x1ff, 0x1ff, 0x3ff];
        let entries = core_entries(&masks, 4, 10);
        assert_eq!(entries.len(), 38);
        let b = Buffer::new(Layout::new(80), entries.len(), 16384);
        let mut h = b.header(0x11_4000_0000, &entries);
        assert_eq!(h.len(), golden.len());
        let mut g = golden.to_vec();
        for id in 0..128u64 {
            let e = (8 + id * CORE_ENTRY_BYTES + 8) as usize;
            g[e..e + 80].fill(0);
            h[e..e + 80].fill(0);
        }
        assert!(h == g);
        // the helper freed everything and failed no allocation
        for &id in &entries {
            let (cur, _, failed) = core_counters(golden, id);
            assert_eq!((cur, failed), (0, 0));
        }
    }
}
