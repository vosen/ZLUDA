//! Loading what the LLVM AGX backend produces: the kernels of a module, their resources and their argument layout.
//!
//! The backend's object is ELF with every kernel's code in `.text` (a `FUNC` symbol per kernel) and the global data
//! buffer's initial contents in `.agx.globals`. The resources a dispatch needs are only reported as comments before
//! each kernel in the assembly (llvm/docs/AGXUsage.rst, "Kernel ABI"), so both outputs of a compilation are read.

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

    fn section(&self, name: &str) -> Option<&[u8]> {
        self.sections
            .iter()
            .find(|s| s.0 == name)
            .and_then(|s| self.data.get(s.2 as usize..(s.2 + s.3) as usize))
    }

    /// `FUNC` symbols defined in `.text`: (name, value, size).
    fn functions(&self) -> Result<Vec<(String, u64, u64)>, Error> {
        let text = self.sections.iter().position(|s| s.0 == ".text").ok_or_else(|| Error("no .text".into()))?;
        let mut out = Vec::new();
        for s in self.sections.iter().filter(|s| s.1 == 2) {
            let strtab = self.sections.get(s.4 as usize).ok_or_else(|| Error("bad symbol string table".into()))?.2;
            let n = s.3 / s.5.max(1);
            for i in 0..n {
                let e = s.2 + i * s.5;
                let (name, info, shndx) = (rd(self.data, e, 4)?, rd(self.data, e + 4, 1)?, rd(self.data, e + 6, 2)?);
                if info & 0xf == 2 && shndx as usize == text {
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
    pub fn load(object: &[u8], assembly: &str) -> Result<Module, Error> {
        let elf = Elf::parse(object)?;
        let text = elf.section(".text").ok_or_else(|| Error("no .text".into()))?.to_vec();
        let globals = elf.section(".agx.globals").map(|s| s.to_vec()).unwrap_or_default();
        let res = resources(assembly);
        let mut kernels = HashMap::new();
        for (name, value, size) in elf.functions()? {
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
                },
            );
        }
        if kernels.is_empty() {
            return err("no kernels with resource comments");
        }
        Ok(Module { text, globals, kernels })
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

    const TGREV_O: &[u8] = include_bytes!("../tests/data/tgrev.o");
    const TGREV_S: &str = include_str!("../tests/data/tgrev.s");

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
