//! Loading what the LLVM AGX backend produces: the kernels of a module, their resources and their argument layout.
//!
//! The backend's object is ELF with every kernel's code in `.text` (a `FUNC` symbol per kernel), the global data
//! buffer's initial contents in `.agx.globals`, and a record per kernel in `.agx.kernels` with its resources and the
//! layout of its arguments (llvm/docs/AGXUsage.rst, "Kernel ABI"). Objects from backends without `.agx.kernels` are
//! read with the resource comments of the assembly instead, and then carry no argument layout.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

fn err<T>(m: impl Into<String>) -> Result<T, Error> {
    Err(Error(m.into()))
}

/// The resources and layout of one kernel.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KernelInfo {
    pub name: String,
    /// Byte offset of the kernel's first instruction in `.text`.
    pub offset: u32,
    pub size: u32,
    /// Uniform registers the kernel reads, in 16-bit halves.
    pub uniform_halves: u32,
    /// 32-bit uniform register holding `threadgroups_per_grid.x` (`.y` and `.z` follow).
    pub threadgroups_uniform: u32,
    /// 32-bit uniform register pair holding the global data buffer address, if the module has globals.
    pub global_data_uniform: Option<u32>,
    /// General purpose registers, in 32-bit units.
    pub gprs: u32,
    pub threadgroup_bytes: u32,
    pub stack_bytes: u32,
    /// How each source-level argument is passed (from `.agx.kernels`; empty for objects without it).
    pub args: Vec<ArgSlot>,
    /// CTAs per cluster along x, y and z (`.agx.kernels` extension 1; 1, 1, 1 without clusters). A cluster runs
    /// as one threadgroup: thread (x, y, z) of CTA r of the cluster is thread (x, y, z * C + r), C = x * y * z.
    pub cluster: [u32; 3],
    /// The hidden arguments of a kernel built with the backend's `agx-checked` feature (`.agx.kernels`
    /// extension 2), which checks its memory accesses and reports the first bad one in an error record.
    pub checked: Option<Checked>,
}

/// Where a checked kernel takes its hidden arguments (LLVM's AGXUsage.rst, "Checked mode").
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Checked {
    /// 16-bit uniform half of the 64-bit error record address.
    pub record_half: u32,
    /// Half of the 32-bit threadgroup memory size (static plus dynamic).
    pub threadgroup_bytes_half: u32,
    pub slots: Vec<CheckedSlot>,
}

/// A pointer the checked kernel bounds-checks: the bounds of the allocation it points into go to `lo_half` and
/// `hi_half` (64-bit each; `lo = hi = pointer` if it points into none).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CheckedSlot {
    /// Source-level argument holding the pointer.
    pub arg: u32,
    /// Byte offset of the pointer in the argument's bytes; `None` if the argument is the pointer.
    pub offset: Option<u32>,
    pub lo_half: u32,
    pub hi_half: u32,
}

/// Bytes of the error record of a checked module.
pub const CHECKED_RECORD_BYTES: u64 = 64;

/// One argument of the source kernel (a PTX `.param`, a CUDA kernel parameter).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArgSlot {
    /// The uniforms hold a 64-bit pointer to the bytes rather than the bytes.
    pub by_pointer: bool,
    pub bytes: u32,
    /// First 16-bit uniform half.
    pub half: u32,
}

impl KernelInfo {
    /// The threadgroups and threadgroup size of a launch of `grid` CTAs of `block` threads: with clusters, one
    /// threadgroup per cluster, its CTAs side by side along z.
    pub fn hardware_launch(&self, grid: [u32; 3], block: [u32; 3]) -> Result<([u32; 3], [u32; 3]), Error> {
        let c = self.cluster;
        if c == [1, 1, 1] {
            return Ok((grid, block));
        }
        if (0..3).any(|i| grid[i] % c[i] != 0) {
            return err(format!("{}: grid {grid:?} is not a multiple of the cluster {c:?}", self.name));
        }
        let local = [block[0], block[1], block[2] * c[0] * c[1] * c[2]];
        if local.iter().map(|&v| v as u64).product::<u64>() > 1024 {
            return err(format!("{}: a cluster of {c:?} CTAs of {block:?} threads exceeds 1024 threads", self.name));
        }
        Ok((std::array::from_fn(|i| grid[i] / c[i]), local))
    }

    /// The register allocation of the USC Registers word, in 16-bit halves (8..=256).
    pub fn gpr_halves(&self) -> u32 {
        (2 * self.gprs).clamp(8, 256)
    }

    /// SIMD-groups of this kernel a G13/G14 core holds: 2 * floor(3328 / (2 * H)), at most 96, with H the register
    /// halves rounded up to the allocation granule of 8 (the USC Registers word counts groups of 8).
    pub fn simd_groups_per_core(&self) -> u32 {
        (2 * (1664 / self.gpr_halves().next_multiple_of(8))).min(96)
    }

    /// The largest threadgroup the kernel's registers allow (a threadgroup runs on one core), at most 1024
    /// (Mesa's agx_occupancy table; the backend prints it in the resource comments).
    /// The largest CTA: its threadgroup holds all the CTAs of a cluster.
    pub fn max_threads_per_threadgroup(&self) -> u32 {
        (32 * self.simd_groups_per_core()).min(1024) / (self.cluster[0] * self.cluster[1] * self.cluster[2]).max(1)
    }
}

/// A compiled module: its code, the initial global data buffer, and its kernels.
#[derive(Clone, Debug)]
pub struct Module {
    pub text: Vec<u8>,
    pub globals: Vec<u8>,
    pub kernels: HashMap<String, KernelInfo>,
    /// Offset and size of each named object in the global data buffer.
    pub global_symbols: HashMap<String, (u64, u64)>,
    /// Words of the global data buffer that hold addresses in it: (offset, width in bytes, target offset). The
    /// loader adds the buffer's address (`Module::relocated_globals`).
    pub global_relocs: Vec<(u64, u8, u64)>,
    /// The object itself when it has a DWARF line table (`.debug_line`), for mapping PCs to source lines
    /// (`agx_run --lines`).
    pub debug_object: Option<Vec<u8>>,
}

struct Elf<'a> {
    data: &'a [u8],
    sections: Vec<(String, u32, u64, u64, u32, u64)>, // name, type, offset, size, link, entsize
}

fn rd(d: &[u8], off: u64, n: usize) -> Result<u64, Error> {
    let o = off as usize;
    let b = d.get(o..o + n).ok_or_else(|| Error("truncated ELF".into()))?;
    let mut v = [0u8; 8];
    v[..n].copy_from_slice(b);
    Ok(u64::from_le_bytes(v))
}

fn cstr(d: &[u8], off: u64) -> Result<String, Error> {
    let s = d.get(off as usize..).ok_or_else(|| Error("bad string offset".into()))?;
    let end = s.iter().position(|&c| c == 0).ok_or_else(|| Error("unterminated string".into()))?;
    Ok(String::from_utf8_lossy(&s[..end]).into_owned())
}

impl<'a> Elf<'a> {
    fn parse(data: &'a [u8]) -> Result<Elf<'a>, Error> {
        if data.len() < 64 || &data[..4] != b"\x7fELF" || data[4] != 2 || data[5] != 1 {
            return err("not a 64-bit little-endian ELF object");
        }
        if rd(data, 18, 2)? != 0x4147 {
            return err("not an AGX object (e_machine is not EM_AGX 0x4147)");
        }
        let (shoff, shentsize, shnum, shstrndx) = (rd(data, 40, 8)?, rd(data, 58, 2)?, rd(data, 60, 2)?, rd(data, 62, 2)?);
        let mut raw = Vec::new();
        for i in 0..shnum {
            let h = shoff + i * shentsize;
            raw.push((
                rd(data, h, 4)?,
                rd(data, h + 4, 4)? as u32,
                rd(data, h + 24, 8)?,
                rd(data, h + 32, 8)?,
                rd(data, h + 40, 4)? as u32,
                rd(data, h + 56, 8)?,
            ));
        }
        let strtab = raw.get(shstrndx as usize).ok_or_else(|| Error("bad section string table".into()))?.2;
        let sections = raw
            .into_iter()
            .map(|(n, t, o, s, l, e)| Ok((cstr(data, strtab + n)?, t, o, s, l, e)))
            .collect::<Result<_, Error>>()?;
        Ok(Elf { data, sections })
    }

    /// Relocations of `.agx.globals`, which may only point into it: R_AGX_ABS64 (1) and R_AGX_ABS32 (2), S + A.
    fn global_relocs(&self) -> Result<Vec<(u64, u8, u64)>, Error> {
        let Some(target) = self.sections.iter().position(|s| s.0 == ".agx.globals") else { return Ok(Vec::new()) };
        let Some(rela) = self.sections.iter().find(|s| s.0 == ".rela.agx.globals" && s.1 == 4) else {
            return Ok(Vec::new());
        };
        let symtab = self.sections.get(rela.4 as usize).ok_or_else(|| Error("bad relocation symbol table".into()))?;
        let size = self.sections[target].3;
        let mut out = Vec::new();
        for i in 0..rela.3 / 24 {
            let e = rela.2 + i * 24;
            let (offset, info, addend) = (rd(self.data, e, 8)?, rd(self.data, e + 8, 8)?, rd(self.data, e + 16, 8)?);
            let sym = symtab.2 + (info >> 32) * symtab.5.max(24);
            let (shndx, value) = (rd(self.data, sym + 6, 2)?, rd(self.data, sym + 8, 8)?);
            let width = match info as u32 {
                1 => 8,
                2 => 4,
                t => return err(format!("unsupported relocation type {t} in .agx.globals")),
            };
            let to = value.wrapping_add(addend);
            if shndx as usize != target || to > size || offset + width as u64 > size {
                return err("a relocation of .agx.globals points outside it (external symbols are not supported)");
            }
            out.push((offset, width, to));
        }
        Ok(out)
    }

    fn section(&self, name: &str) -> Option<&[u8]> {
        self.sections
            .iter()
            .find(|s| s.0 == name)
            .and_then(|s| self.data.get(s.2 as usize..(s.2 + s.3) as usize))
    }

    /// The records of `.agx.kernels`, by kernel name.
    fn kernel_records(&self) -> Result<Option<HashMap<String, KernelInfo>>, Error> {
        let Some(s) = self.section(".agx.kernels") else { return Ok(None) };
        let word = |i: usize| -> Result<u32, Error> {
            s.get(4 * i..4 * i + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| Error("truncated .agx.kernels".into()))
        };
        let mut out = HashMap::new();
        let mut at = 0;
        while 4 * at < s.len() {
            let (size, version, name_len) = (word(at)? as usize, word(at + 1)?, word(at + 2)? as usize);
            if version != 1 || size % 4 != 0 || size < 12 || 4 * at + size > s.len() {
                return err(format!("unsupported .agx.kernels record (version {version}, {size} bytes)"));
            }
            let name_at = 4 * (at + 3);
            let name = String::from_utf8_lossy(s.get(name_at..name_at + name_len).ok_or_else(|| Error("bad kernel name".into()))?).into_owned();
            let f = at + 3 + name_len.div_ceil(4);
            let nargs = word(f + 6)? as usize;
            if 4 * (f + 7 + 3 * nargs) > 4 * at + size {
                return err(format!("{name}: truncated argument table"));
            }
            let global = word(f + 2)?;
            let args = (0..nargs)
                .map(|i| Ok(ArgSlot { by_pointer: word(f + 7 + 3 * i)? == 1, bytes: word(f + 8 + 3 * i)?, half: word(f + 9 + 3 * i)? }))
                .collect::<Result<_, Error>>()?;
            // tagged extensions up to the end of the record
            let (mut cluster, mut checked, mut e, end) = ([1, 1, 1], None, f + 7 + 3 * nargs, at + size / 4);
            while e < end {
                match word(e)? {
                    1 if e + 4 <= end => {
                        cluster = [word(e + 1)?.max(1), word(e + 2)?.max(1), word(e + 3)?.max(1)];
                        e += 4;
                    }
                    2 if e + 5 <= end => {
                        let (abi, n) = (word(e + 1)?, word(e + 2)? as usize);
                        if abi != 1 || e + 5 + 4 * n > end {
                            return err(format!("{name}: unsupported checked mode (ABI {abi}, {n} slots)"));
                        }
                        let slots = (0..n)
                            .map(|i| {
                                let w = e + 5 + 4 * i;
                                let (arg, off) = (word(w)?, word(w + 1)?);
                                if arg as usize >= nargs {
                                    return err(format!("{name}: checked pointer in argument {arg} of {nargs}"));
                                }
                                Ok(CheckedSlot { arg, offset: (off != u32::MAX).then_some(off), lo_half: word(w + 2)?, hi_half: word(w + 3)? })
                            })
                            .collect::<Result<_, Error>>()?;
                        checked = Some(Checked { record_half: word(e + 3)?, threadgroup_bytes_half: word(e + 4)?, slots });
                        e += 5 + 4 * n;
                    }
                    _ => break,
                }
            }
            out.insert(
                name.clone(),
                KernelInfo {
                    name,
                    offset: 0,
                    size: 0,
                    uniform_halves: word(f)?,
                    threadgroups_uniform: word(f + 1)? / 2,
                    global_data_uniform: (global != u32::MAX).then_some(global / 2),
                    gprs: word(f + 3)?,
                    threadgroup_bytes: word(f + 4)?,
                    stack_bytes: word(f + 5)?,
                    args,
                    cluster,
                    checked,
                },
            );
            at += size / 4;
        }
        Ok(Some(out))
    }

    /// `FUNC` symbols defined in `.text`: (name, value, size).
    fn functions(&self) -> Result<Vec<(String, u64, u64)>, Error> {
        self.symbols(".text", 2)
    }

    /// Symbols of type `kind` (2 = FUNC, 1 = OBJECT) defined in section `section`: (name, value, size).
    fn symbols(&self, section: &str, kind: u64) -> Result<Vec<(String, u64, u64)>, Error> {
        let Some(text) = self.sections.iter().position(|s| s.0 == section) else {
            return if kind == 2 { err("no .text") } else { Ok(Vec::new()) };
        };
        let mut out = Vec::new();
        for s in self.sections.iter().filter(|s| s.1 == 2) {
            let strtab = self.sections.get(s.4 as usize).ok_or_else(|| Error("bad symbol string table".into()))?.2;
            let n = s.3 / s.5.max(1);
            for i in 0..n {
                let e = s.2 + i * s.5;
                let (name, info, shndx) = (rd(self.data, e, 4)?, rd(self.data, e + 4, 1)?, rd(self.data, e + 6, 2)?);
                if info & 0xf == kind && shndx as usize == text {
                    out.push((cstr(self.data, strtab + name)?, rd(self.data, e + 8, 8)?, rd(self.data, e + 16, 8)?));
                }
            }
        }
        Ok(out)
    }
}

fn first_number(s: &str) -> Option<u32> {
    let d: String = s.chars().skip_while(|c| !c.is_ascii_digit()).take_while(|c| c.is_ascii_digit()).collect();
    d.parse().ok()
}

/// Resource comments of each kernel, keyed by its label.
fn resources(assembly: &str) -> HashMap<String, [Option<u32>; 6]> {
    // [uniform halves, threadgroups at, global data at, gprs, threadgroup bytes, stack bytes]
    let mut out: HashMap<String, [Option<u32>; 6]> = HashMap::new();
    let mut current: Option<String> = None;
    for line in assembly.lines() {
        let t = line.trim();
        if let Some(label) = t.split_once(':').filter(|(l, _)| !l.is_empty() && !l.starts_with('.') && !l.contains(' ') && !t.starts_with("//")) {
            current = Some(label.0.to_string());
            continue;
        }
        let (Some(name), Some(c)) = (&current, t.strip_prefix("//")) else { continue };
        let c = c.trim();
        let r = out.entry(name.clone()).or_default();
        if let Some(rest) = c.strip_prefix("uniform registers:") {
            r[0] = first_number(rest);
            r[1] = rest.split_once("threadgroups_per_grid at u").and_then(|(_, x)| first_number(x));
            r[2] = rest.split_once("global data buffer at u").and_then(|(_, x)| first_number(x));
        } else if let Some(rest) = c.strip_prefix("general purpose registers:") {
            r[3] = first_number(rest);
        } else if let Some(rest) = c.strip_prefix("threadgroup memory:") {
            r[4] = first_number(rest);
        } else if let Some(rest) = c.strip_prefix("stack size:") {
            r[5] = first_number(rest);
        } else if c.contains("global data buffer at u") {
            r[2] = c.split_once("global data buffer at u").and_then(|(_, x)| first_number(x));
        }
    }
    out
}

impl Module {
    /// Reads a module from the backend's object file and its assembly.
    /// Reads a module from an object with `.agx.kernels` alone.
    pub fn load_object(object: &[u8]) -> Result<Module, Error> {
        Self::load(object, "")
    }

    pub fn load(object: &[u8], assembly: &str) -> Result<Module, Error> {
        let elf = Elf::parse(object)?;
        let text = elf.section(".text").ok_or_else(|| Error("no .text".into()))?.to_vec();
        let globals = elf.section(".agx.globals").map(|s| s.to_vec()).unwrap_or_default();
        let records = elf.kernel_records()?;
        let res = resources(assembly);
        let mut kernels = HashMap::new();
        for (name, value, size) in elf.functions()? {
            if value % 2 != 0 || value + size > text.len() as u64 {
                return err(format!("{name}: symbol outside .text"));
            }
            if let Some(records) = &records {
                let Some(k) = records.get(&name) else { continue };
                kernels.insert(name.clone(), KernelInfo { offset: value as u32, size: size as u32, ..k.clone() });
                continue;
            }
            let Some(r) = res.get(&name) else { continue };
            let (Some(uniform_halves), Some(threadgroups_uniform), Some(gprs)) = (r[0], r[1], r[3]) else {
                return err(format!("{name}: incomplete resource comments"));
            };
            if value % 2 != 0 || value + size > text.len() as u64 {
                return err(format!("{name}: symbol outside .text"));
            }
            kernels.insert(
                name.clone(),
                KernelInfo {
                    name,
                    offset: value as u32,
                    size: size as u32,
                    uniform_halves,
                    threadgroups_uniform,
                    global_data_uniform: r[2],
                    gprs,
                    threadgroup_bytes: r[4].unwrap_or(0),
                    stack_bytes: r[5].unwrap_or(0),
                    args: Vec::new(),
                    cluster: [1, 1, 1],
                    checked: None,
                },
            );
        }
        if kernels.is_empty() {
            return err("no kernels with resource comments");
        }
        let global_symbols = elf.symbols(".agx.globals", 1)?.into_iter().map(|(n, v, s)| (n, (v, s))).collect();
        let global_relocs = elf.global_relocs()?;
        let debug_object = elf.sections.iter().any(|s| s.0 == ".debug_line").then(|| object.to_vec());
        Ok(Module { text, globals, kernels, global_symbols, global_relocs, debug_object })
    }

    /// The initial global data buffer once placed at `va`, with its relocations applied.
    pub fn relocated_globals(&self, va: u64) -> Vec<u8> {
        let mut g = self.globals.clone();
        for &(offset, width, to) in &self.global_relocs {
            let v = (va + to).to_le_bytes();
            g[offset as usize..offset as usize + width as usize].copy_from_slice(&v[..width as usize]);
        }
        g
    }
}

/// Magic of a fat AGX object: one AGX object per chip, as packed by the LLVM fork's
/// `mlir/utils/agx-kernels/agx-build-fatbin.py` for library kernels written once for every generation. Layout
/// (little-endian): the magic, a `u32` entry count, a `u32` 0, then per entry 32 bytes of NUL-padded chip name (an
/// LLVM AGX -mcpu) and the `u64` offset and size of its object.
pub const FAT_MAGIC: &[u8; 8] = b"ZAGXFAT1";

/// The size of the fat AGX object at `p` (from its entry table).
///
/// # Safety
/// `p` must point at a fat AGX object (starting with [`FAT_MAGIC`]).
pub unsafe fn fat_object_len(p: *const u8) -> usize {
    let n = u32::from_le_bytes(std::slice::from_raw_parts(p.add(8), 4).try_into().unwrap()) as usize;
    let table = std::slice::from_raw_parts(p, 16 + 48 * n);
    (0..n)
        .map(|i| {
            let e = &table[16 + 48 * i + 32..16 + 48 * (i + 1)];
            (u64::from_le_bytes(e[..8].try_into().unwrap()) + u64::from_le_bytes(e[8..].try_into().unwrap())) as usize
        })
        .max()
        .unwrap_or(0)
        .max(16 + 48 * n)
}

/// The object for `chip` in a fat AGX object.
pub fn fat_object_entry<'a>(fat: &'a [u8], chip: &str) -> Result<&'a [u8], Error> {
    if fat.len() < 16 || &fat[..8] != FAT_MAGIC {
        return err("not a fat AGX object");
    }
    let n = u32::from_le_bytes(fat[8..12].try_into().unwrap()) as usize;
    let mut chips = Vec::new();
    for i in 0..n {
        let e = fat.get(16 + 48 * i..16 + 48 * (i + 1)).ok_or_else(|| Error("truncated fat AGX object".into()))?;
        let name = String::from_utf8_lossy(&e[..32]).trim_end_matches('\0').to_string();
        let (off, size) = (u64::from_le_bytes(e[32..40].try_into().unwrap()), u64::from_le_bytes(e[40..48].try_into().unwrap()));
        if name == chip {
            return fat.get(off as usize..(off + size) as usize).ok_or_else(|| Error("truncated fat AGX object".into()));
        }
        chips.push(name);
    }
    err(format!("the fat AGX object has no code for {chip} (it has {})", chips.join(", ")))
}

/// The uniforms of a launch from the bytes of each source-level argument (`cuLaunchKernel`'s `kernelParams`), and
/// the contents of the parameter block that holds the arguments passed by pointer. `param_block` is the GPU address
/// the block will have; its size is `param_block_size(k)`.
pub fn pack_params(
    k: &KernelInfo,
    params: &[&[u8]],
    threadgroups: [u32; 3],
    global_data: Option<u64>,
    param_block: u64,
) -> Result<(Vec<u16>, Vec<u8>), Error> {
    if params.len() != k.args.len() {
        return err(format!("{}: {} arguments given, {} expected", k.name, params.len(), k.args.len()));
    }
    let mut u = vec![0u16; k.uniform_halves as usize];
    let mut block = Vec::new();
    let put = |u: &mut Vec<u16>, half: u32, bytes: &[u8]| -> Result<(), Error> {
        for (i, pair) in bytes.chunks(2).enumerate() {
            let h = u.get_mut(half as usize + i).ok_or_else(|| Error(format!("{}: argument outside the uniforms", k.name)))?;
            *h = pair[0] as u16 | (*pair.get(1).unwrap_or(&0) as u16) << 8;
        }
        Ok(())
    };
    for (i, (slot, p)) in k.args.iter().zip(params).enumerate() {
        if p.len() != slot.bytes as usize {
            return err(format!("{}: argument {i} has {} bytes, {} expected", k.name, p.len(), slot.bytes));
        }
        if slot.half == u32::MAX {
            continue; // unused by the kernel
        }
        if slot.by_pointer {
            let at = block.len().next_multiple_of(16);
            block.resize(at, 0);
            block.extend_from_slice(p);
            put(&mut u, slot.half, &(param_block + at as u64).to_le_bytes())?;
        } else {
            put(&mut u, slot.half, p)?;
        }
    }
    for (d, t) in threadgroups.iter().enumerate() {
        put(&mut u, 2 * k.threadgroups_uniform + 2 * d as u32, &t.to_le_bytes())?;
    }
    match (k.global_data_uniform, global_data) {
        (Some(at), Some(va)) => put(&mut u, 2 * at, &va.to_le_bytes())?,
        (Some(_), None) => return err(format!("{}: the module has globals but no global data buffer was given", k.name)),
        _ => {}
    }
    Ok((u, block))
}

/// Bytes of the parameter block `pack_params` fills.
pub fn param_block_size(k: &KernelInfo) -> u64 {
    k.args.iter().filter(|a| a.by_pointer && a.half != u32::MAX).fold(0, |n, a| n.next_multiple_of(16) + a.bytes as u64)
}

/// The pointers a checked kernel bounds-checks, in the order of its slots, read from the launch's arguments.
pub fn checked_pointers(k: &KernelInfo, params: &[&[u8]]) -> Result<Vec<u64>, Error> {
    let Some(c) = &k.checked else { return Ok(Vec::new()) };
    c.slots
        .iter()
        .map(|s| {
            let at = s.offset.unwrap_or(0) as usize;
            let b = params.get(s.arg as usize).and_then(|p| p.get(at..at + 8));
            b.map(|b| u64::from_le_bytes(b.try_into().unwrap()))
                .ok_or_else(|| Error(format!("{}: checked pointer outside argument {}", k.name, s.arg)))
        })
        .collect()
}

/// Fills the hidden arguments of a checked kernel in `u` (from `pack_params`): the error record's address, the
/// threadgroup memory bytes, and per slot the `[lo, hi)` bounds of the allocation its pointer points into.
pub fn pack_checked(k: &KernelInfo, u: &mut [u16], record: u64, threadgroup_bytes: u32, bounds: &[(u64, u64)]) -> Result<(), Error> {
    let Some(c) = &k.checked else { return Ok(()) };
    if bounds.len() != c.slots.len() {
        return err(format!("{}: {} bounds given, {} checked pointers", k.name, bounds.len(), c.slots.len()));
    }
    let mut put = |half: u32, v: u64, halves: usize| -> Result<(), Error> {
        for i in 0..halves {
            *u.get_mut(half as usize + i).ok_or_else(|| Error(format!("{}: hidden argument outside the uniforms", k.name)))? =
                (v >> (16 * i)) as u16;
        }
        Ok(())
    };
    put(c.record_half, record, 4)?;
    put(c.threadgroup_bytes_half, threadgroup_bytes as u64, 2)?;
    for (s, &(lo, hi)) in c.slots.iter().zip(bounds) {
        put(s.lo_half, lo, 4)?;
        put(s.hi_half, hi, 4)?;
    }
    Ok(())
}

/// Symbol of the text describing each check site of a checked module: one line per site,
/// `<site>\t<access> <size>\t<frame>[\t<frame>...]`, innermost frame first, a frame being `function file:line:col`.
pub const CHECKED_SITES_SYMBOL: &str = "__agx_checked_sites";

/// The first bad access a checked kernel reported.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CheckedError {
    /// 1 out of bounds, 2 null, 3 misaligned, 4 threadgroup memory out of bounds, 5 stack out of bounds.
    pub kind: u32,
    pub store: bool,
    pub atomic: bool,
    pub address: u64,
    pub size: u32,
    pub site: u32,
    pub thread: [u32; 3],
    pub threadgroup: [u32; 3],
    /// The site's line of the module's site text, without the site number (empty if it has none).
    pub site_text: String,
}

impl CheckedError {
    /// Decodes an error record; `None` if no access was reported.
    pub fn decode(record: &[u8], sites: &str) -> Option<CheckedError> {
        let w = |i: usize| u32::from_le_bytes(record[4 * i..4 * i + 4].try_into().unwrap());
        if record.len() < 48 || w(0) == 0 {
            return None;
        }
        let site = w(5);
        let site_text = sites
            .lines()
            .find_map(|l| l.split_once('\t').filter(|(n, _)| n.parse() == Ok(site)).map(|(_, t)| t.to_string()))
            .unwrap_or_default();
        Some(CheckedError {
            kind: w(1) & 0xff,
            store: w(1) & 0x100 != 0,
            atomic: w(1) & 0x200 != 0,
            address: w(2) as u64 | (w(3) as u64) << 32,
            size: w(4),
            site,
            thread: [w(6), w(7), w(8)],
            threadgroup: [w(9), w(10), w(11)],
            site_text,
        })
    }

    pub fn misaligned(&self) -> bool {
        self.kind == 3
    }
}

impl std::fmt::Display for CheckedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = match self.kind {
            1 => "out of bounds",
            2 => "null",
            3 => "misaligned",
            4 => "threadgroup memory out of bounds",
            5 => "stack out of bounds",
            _ => "bad",
        };
        let what = if self.atomic { "atomic" } else if self.store { "store" } else { "load" };
        let [x, y, z] = self.thread;
        let [a, b, c] = self.threadgroup;
        write!(f, "{kind} {what} of {} bytes at {:#x} by thread ({x}, {y}, {z}) of block ({a}, {b}, {c})", self.size, self.address)?;
        for (i, frame) in self.site_text.split('\t').skip(1).enumerate() {
            write!(f, "\n    {} {frame}", if i == 0 { "at" } else { "inlined into" })?;
        }
        Ok(())
    }
}

/// One kernel argument, as the backend splits it.
#[derive(Clone, Copy, Debug)]
pub enum Arg {
    /// `i1`, `i8`, `i16`, `half`: one 16-bit unit.
    U16(u16),
    /// `i32`, `float`: two units, 32-bit aligned.
    U32(u32),
    /// `i64`, `double`, pointers: four units, aligned to 32 bits.
    U64(u64),
}

/// The uniform register values of a launch, as 16-bit halves: the arguments packed in order (each aligned to its
/// size, 64-bit ones to 32 bits), then `threadgroups_per_grid`, then the global data buffer address if any.
pub fn pack_uniforms(k: &KernelInfo, args: &[Arg], threadgroups: [u32; 3], global_data: Option<u64>) -> Result<Vec<u16>, Error> {
    let mut u: Vec<u16> = Vec::new();
    let align = |u: &mut Vec<u16>, n: usize| {
        while u.len() % n != 0 {
            u.push(0)
        }
    };
    for a in args {
        match *a {
            Arg::U16(v) => u.push(v),
            Arg::U32(v) => {
                align(&mut u, 2);
                u.extend([v as u16, (v >> 16) as u16]);
            }
            Arg::U64(v) => {
                align(&mut u, 2);
                u.extend((0..4).map(|i| (v >> (16 * i)) as u16));
            }
        }
    }
    align(&mut u, 2);
    if u.len() != 2 * k.threadgroups_uniform as usize {
        return err(format!(
            "{}: the arguments fill {} halves, but threadgroups_per_grid is at u{}",
            k.name,
            u.len(),
            k.threadgroups_uniform
        ));
    }
    for t in threadgroups {
        u.extend([t as u16, (t >> 16) as u16]);
    }
    match (k.global_data_uniform, global_data) {
        (Some(at), Some(va)) => {
            u.resize(2 * at as usize, 0);
            u.extend((0..4).map(|i| (va >> (16 * i)) as u16));
        }
        (Some(_), None) => return err(format!("{}: the module has globals but no global data buffer was given", k.name)),
        _ => {}
    }
    u.resize(u.len().max(k.uniform_halves as usize), 0);
    Ok(u)
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests/data/checked.ll: a pointer argument, and a pointer at byte 8 of a by-reference argument.
    #[test]
    fn checked_kernels_take_their_hidden_arguments() {
        let m = Module::load_object(include_bytes!("../tests/data/checked.o")).unwrap();
        let k = &m.kernels["put"];
        let c = k.checked.as_ref().unwrap();
        assert_eq!(k.args.len(), 3, "the hidden arguments are not source arguments");
        assert_eq!(
            c.slots,
            [
                CheckedSlot { arg: 0, offset: None, lo_half: 16, hi_half: 20 },
                CheckedSlot { arg: 2, offset: Some(8), lo_half: 24, hi_half: 28 },
            ]
        );
        let out = 0x1_0000_1000u64.to_le_bytes();
        let i = 5u32.to_le_bytes();
        let mut pair = [0u8; 16];
        pair[8..].copy_from_slice(&0x2_0000_0040u64.to_le_bytes());
        let params: [&[u8]; 3] = [&out, &i, &pair];
        assert_eq!(checked_pointers(k, &params).unwrap(), [0x1_0000_1000, 0x2_0000_0040]);
        let (mut u, _) = pack_params(k, &params, [1, 1, 1], Some(0x5000), 0x3000).unwrap();
        pack_checked(k, &mut u, 0x7000_0000, 96, &[(0x1_0000_0000, 0x1_0001_0000), (0x2_0000_0040, 0x2_0000_0040)]).unwrap();
        let q = |h: usize, n: usize| (0..n).fold(0u64, |v, i| v | (u[h + i] as u64) << (16 * i));
        assert_eq!(q(c.record_half as usize, 4), 0x7000_0000);
        assert_eq!(q(c.threadgroup_bytes_half as usize, 2), 96);
        assert_eq!((q(16, 4), q(20, 4)), (0x1_0000_0000, 0x1_0001_0000));
        assert_eq!((q(24, 4), q(28, 4)), (0x2_0000_0040, 0x2_0000_0040));
        assert_eq!(q(0, 4), 0x1_0000_1000, "the source arguments are unchanged");
        assert!(pack_checked(k, &mut u, 0, 0, &[]).is_err(), "every slot needs bounds");
        // too short an argument for its pointer
        let short: [&[u8]; 3] = [&out, &i, &pair[..12]];
        assert!(checked_pointers(k, &short).is_err());
    }

    #[test]
    fn checked_errors_are_decoded_with_their_site() {
        let m = Module::load_object(include_bytes!("../tests/data/checked.o")).unwrap();
        let (off, size) = m.global_symbols[CHECKED_SITES_SYMBOL];
        let t = &m.globals[off as usize..(off + size) as usize];
        let sites = std::str::from_utf8(&t[..t.iter().position(|&c| c == 0).unwrap()]).unwrap();
        let mut r = [0u8; 64];
        assert_eq!(CheckedError::decode(&r, sites), None);
        for (i, w) in [1u32, 0x101, 0x1004, 2, 4, 3, 7, 0, 0, 2, 1, 0].iter().enumerate() {
            r[4 * i..4 * i + 4].copy_from_slice(&w.to_le_bytes());
        }
        let e = CheckedError::decode(&r, sites).unwrap();
        assert_eq!((e.kind, e.store, e.atomic, e.address, e.size, e.site), (1, true, false, 0x2_0000_1004, 4, 3));
        assert_eq!((e.thread, e.threadgroup), ([7, 0, 0], [2, 1, 0]));
        assert!(!e.misaligned());
        assert_eq!(
            e.to_string(),
            "out of bounds store of 4 bytes at 0x200001004 by thread (7, 0, 0) of block (2, 1, 0)\n    at put put.cu:5:3"
        );
        // an inlined site, and a site the text does not have
        let text = "1\tload 8\tget a.cu:3:5\tk a.cu:9:1\n";
        r[20..24].copy_from_slice(&1u32.to_le_bytes());
        assert!(CheckedError::decode(&r, text).unwrap().to_string().ends_with("\n    at get a.cu:3:5\n    inlined into k a.cu:9:1"));
        r[20..24].copy_from_slice(&9u32.to_le_bytes());
        assert_eq!(CheckedError::decode(&r, text).unwrap().site_text, "");
    }

    const TGREV_O: &[u8] = include_bytes!("../tests/data/tgrev.o");
    const TGREV_S: &str = include_str!("../tests/data/tgrev.s");

    // A kernel that appends one printf record the way the runtime library's vprintf does.
    #[test]
    fn printf_records_resolve_through_the_global_data_buffer() {
        let m = Module::load(include_bytes!("../tests/data/printf.o"), include_str!("../tests/data/printf.s")).unwrap();
        let (off, size) = m.global_symbols[crate::printf::BUFFER_SYMBOL];
        assert_eq!(size, 256);
        assert!(m.kernels["say"].global_data_uniform.is_some());
        // What the device would leave after one thread with tid 1: the format string is the first object.
        let va = 0x1_0000_0000u64;
        let fmt = va + m.globals.windows(4).position(|w| w == b"%d\n\0").unwrap() as u64;
        let mut g = m.globals.clone();
        let b = &mut g[off as usize..(off + size) as usize];
        b[0..4].copy_from_slice(&24u32.to_le_bytes());
        b[8..12].copy_from_slice(&24u32.to_le_bytes());
        b[12..16].copy_from_slice(&4u32.to_le_bytes());
        b[16..24].copy_from_slice(&fmt.to_le_bytes());
        b[24..28].copy_from_slice(&42u32.to_le_bytes());
        let read = |a: u64| {
            let s = g.get((a - va) as usize..)?;
            Some(s[..s.iter().position(|&c| c == 0)?].to_vec())
        };
        let d = crate::printf::drain(&g[off as usize..(off + size) as usize], read);
        assert_eq!(d.text, b"42\n");
    }

    #[test]
    fn threadgroup_size_follows_the_register_count() {
        let k = |gprs| KernelInfo { gprs, ..Module::load_object(include_bytes!("../tests/data/byref.o")).unwrap().kernels["byref"].clone() };
        assert_eq!(k(4).max_threads_per_threadgroup(), 1024);
        assert_eq!(k(52).max_threads_per_threadgroup(), 1024);
        assert_eq!(k(53).max_threads_per_threadgroup(), 896);
        assert_eq!(k(64).max_threads_per_threadgroup(), 832);
        assert_eq!(k(80).max_threads_per_threadgroup(), 640);
        assert_eq!(k(82).max_threads_per_threadgroup(), 576);
        assert_eq!(k(128).max_threads_per_threadgroup(), 384);
        assert_eq!(k(4).simd_groups_per_core(), 96);
    }

    #[test]
    fn picks_the_chip_of_a_fat_object() {
        let mut fat = FAT_MAGIC.to_vec();
        fat.extend(2u32.to_le_bytes());
        fat.extend(0u32.to_le_bytes());
        for (name, off, size) in [("agx-g13g", 112u64, 3u64), ("agx-g14c", 128, 2)] {
            fat.extend(format!("{name:\0<32}").as_bytes());
            fat.extend(off.to_le_bytes());
            fat.extend(size.to_le_bytes());
        }
        fat.resize(112, 0);
        fat.extend(b"abc");
        fat.resize(128, 0);
        fat.extend(b"xy");
        assert_eq!(fat_object_entry(&fat, "agx-g14c").unwrap(), b"xy");
        assert_eq!(fat_object_entry(&fat, "agx-g13g").unwrap(), b"abc");
        assert!(fat_object_entry(&fat, "agx-g14g").unwrap_err().0.contains("agx-g13g, agx-g14c"));
        assert_eq!(unsafe { fat_object_len(fat.as_ptr()) }, 130);
    }

    // tests/data/global-relocs.ll: a global array of addresses of other globals.
    #[test]
    fn relocates_addresses_in_global_data() {
        let m = Module::load_object(include_bytes!("../tests/data/global-relocs.o")).unwrap();
        let (x, y, p) = (m.global_symbols["x"].0, m.global_symbols["y"].0, m.global_symbols["p"].0);
        assert_eq!(m.global_relocs, [(p, 8, x), (p + 8, 8, y)]);
        let g = m.relocated_globals(0x10_0000_0000);
        let word = |o: u64| u64::from_le_bytes(g[o as usize..o as usize + 8].try_into().unwrap());
        assert_eq!((word(p), word(p + 8)), (0x10_0000_0000 + x, 0x10_0000_0000 + y));
        assert_eq!((word(x), word(y + 8)), (1, 5));
    }

    // tests/data/byref.ll: the backend's .agx.kernels records, and packing cuLaunchKernel-style parameters.
    #[test]
    fn packs_parameters_from_the_kernel_records() {
        let m = Module::load_object(include_bytes!("../tests/data/byref.o")).unwrap();
        let k = &m.kernels["byref"];
        let v = |bytes, half| ArgSlot { by_pointer: false, bytes, half };
        assert_eq!(k.args, [v(8, 0), v(12, 4), v(4, 10)]);
        assert_eq!((k.uniform_halves, k.threadgroups_uniform, k.gprs), (18, 6, 3));
        assert!(k.size > 0);
        let s = [1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let (u, block) = pack_params(k, &[&0x1122_3344_5566_7788u64.to_le_bytes(), &s, &7u32.to_le_bytes()], [5, 1, 1], None, 0).unwrap();
        assert!(block.is_empty());
        assert_eq!(&u[0..4], &[0x7788, 0x5566, 0x3344, 0x1122]);
        assert_eq!(&u[4..10], &[0x0201, 0x0403, 0x0605, 0x0807, 0x0a09, 0x0c0b]);
        assert_eq!(&u[10..12], &[7, 0]);
        assert_eq!(&u[12..18], &[5, 0, 1, 0, 1, 0]);
        assert!(pack_params(k, &[&[0u8; 8], &s], [1, 1, 1], None, 0).is_err());

        let d = &m.kernels["dynamic"];
        assert_eq!(d.args, [v(8, 0), ArgSlot { by_pointer: true, bytes: 16, half: 4 }, v(4, 8)]);
        assert_eq!(param_block_size(d), 16);
        let t: Vec<u8> = (0..16).collect();
        let (u, block) = pack_params(d, &[&[0u8; 8], &t, &3u32.to_le_bytes()], [1, 1, 1], None, 0x5_0000_0000).unwrap();
        assert_eq!(block, t);
        assert_eq!(&u[4..8], &[0, 0, 5, 0]);
        assert_eq!(&u[8..10], &[3, 0]);
    }

    #[test]
    fn loads_a_backend_module() {
        let m = Module::load(TGREV_O, TGREV_S).unwrap();
        let k = &m.kernels["tgrev"];
        assert_eq!(
            k,
            &KernelInfo {
                name: "tgrev".into(),
                offset: 0,
                size: 98,
                uniform_halves: 18,
                threadgroups_uniform: 6,
                global_data_uniform: None,
                gprs: 5,
                threadgroup_bytes: 256,
                stack_bytes: 0,
                args: vec![],
                cluster: [1, 1, 1],
                checked: None,
            }
        );
        assert_eq!(m.text.len(), 98);
        assert_eq!(k.gpr_halves(), 10);
    }

    #[test]
    fn packs_the_documented_abi() {
        // kernel-args.ll: (ptr, i16, i32, i64, half, float) -> threadgroups_per_grid at u8, 22 halves
        let k = KernelInfo {
            name: "args".into(),
            offset: 0,
            size: 0,
            uniform_halves: 22,
            threadgroups_uniform: 8,
            global_data_uniform: None,
            gprs: 3,
            threadgroup_bytes: 0,
            stack_bytes: 0,
            args: vec![],
            cluster: [1, 1, 1],
            checked: None,
        };
        let u = pack_uniforms(
            &k,
            &[Arg::U64(0x1122_3344_5566_7788), Arg::U16(7), Arg::U32(9), Arg::U64(0xaaaa_bbbb_cccc_dddd), Arg::U16(1), Arg::U32(2)],
            [3, 1, 1],
            None,
        )
        .unwrap();
        assert_eq!(u.len(), 22);
        // ptr u0_u1; i16 u2l (u2h padding); i32 u3; i64 u4_u5; half u6l; float u7; threadgroups u8..u10
        assert_eq!(&u[0..4], &[0x7788, 0x5566, 0x3344, 0x1122]);
        assert_eq!(u[4], 7);
        assert_eq!(&u[6..8], &[9, 0]);
        assert_eq!(&u[8..12], &[0xdddd, 0xcccc, 0xbbbb, 0xaaaa]);
        assert_eq!(u[12], 1);
        assert_eq!(&u[14..16], &[2, 0]);
        assert_eq!(&u[16..22], &[3, 0, 1, 0, 1, 0]);
    }

    #[test]
    fn rejects_a_mismatched_argument_list() {
        let m = Module::load(TGREV_O, TGREV_S).unwrap();
        assert!(pack_uniforms(&m.kernels["tgrev"], &[Arg::U64(0)], [1, 1, 1], None).is_err());
    }
}
