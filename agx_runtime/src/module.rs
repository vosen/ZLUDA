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
}

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
    /// The register allocation of the USC Registers word, in 16-bit halves (8..=256).
    pub fn gpr_halves(&self) -> u32 {
        (2 * self.gprs).clamp(8, 256)
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
                },
            );
        }
        if kernels.is_empty() {
            return err("no kernels with resource comments");
        }
        let global_symbols = elf.symbols(".agx.globals", 1)?.into_iter().map(|(n, v, s)| (n, (v, s))).collect();
        let global_relocs = elf.global_relocs()?;
        Ok(Module { text, globals, kernels, global_symbols, global_relocs })
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
