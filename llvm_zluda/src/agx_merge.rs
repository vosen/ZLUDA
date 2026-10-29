//! One AGX object from several: the objects of a module's kernels compiled one at a time (compile_air's AGX code a
//! kernel at a time), merged into the object a whole-module compilation makes, which agx_runtime loads
//! (agx_runtime/src/module.rs): every kernel's code in `.text` with a `FUNC` symbol each, the kernels' records in
//! `.agx.kernels`, and the global data buffer's initial contents in `.agx.globals` with its symbols and
//! relocations.
//!
//! The kernels' objects have the same `.agx.globals` (each kernel's module keeps every global variable), which the
//! merged object takes once; `.text` has no relocations, and `.agx.kernels` records name their kernels. An object
//! with any other section that has contents is not merged (an error: the caller compiles the module whole).

const EM_AGX: u16 = 0x4147;
const SHT_PROGBITS: u32 = 1;
const SHT_SYMTAB: u32 = 2;
const SHT_STRTAB: u32 = 3;
const SHT_RELA: u32 = 4;
const SHT_NOBITS: u32 = 8;
const SHF_WRITE: u64 = 1;
const SHF_ALLOC: u64 = 2;
const SHF_EXECINSTR: u64 = 4;
const SHF_INFO_LINK: u64 = 0x40;
const STT_SECTION: u8 = 3;

struct Section {
    name: String,
    kind: u32,
    offset: usize,
    size: usize,
    link: u32,
    info: u32,
    align: u64,
}

struct Symbol {
    name: String,
    info: u8,
    other: u8,
    shndx: u16,
    value: u64,
    size: u64,
}

struct Elf<'a> {
    data: &'a [u8],
    sections: Vec<Section>,
}

fn u16_at(d: &[u8], o: usize) -> Result<u16, String> {
    d.get(o..o + 2).map(|b| u16::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| "truncated AGX object".into())
}
fn u32_at(d: &[u8], o: usize) -> Result<u32, String> {
    d.get(o..o + 4).map(|b| u32::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| "truncated AGX object".into())
}
fn u64_at(d: &[u8], o: usize) -> Result<u64, String> {
    d.get(o..o + 8).map(|b| u64::from_le_bytes(b.try_into().unwrap())).ok_or_else(|| "truncated AGX object".into())
}
fn cstr_at(d: &[u8], o: usize) -> Result<String, String> {
    let s = d.get(o..).ok_or("bad string table offset")?;
    let end = s.iter().position(|&b| b == 0).ok_or("unterminated string")?;
    Ok(String::from_utf8_lossy(&s[..end]).into_owned())
}

impl<'a> Elf<'a> {
    fn parse(data: &'a [u8]) -> Result<Self, String> {
        if data.get(..4) != Some(b"\x7fELF") || data.get(4) != Some(&2) || data.get(5) != Some(&1) {
            return Err("not a 64-bit little-endian ELF object".into());
        }
        if u16_at(data, 18)? != EM_AGX {
            return Err("not an AGX object".into());
        }
        let shoff = u64_at(data, 40)? as usize;
        let shentsize = u16_at(data, 58)? as usize;
        let shnum = u16_at(data, 60)? as usize;
        let shstrndx = u16_at(data, 62)? as usize;
        let header = |i: usize| -> Result<(u32, u32, u64, usize, usize, u32, u32, u64), String> {
            let o = shoff + i * shentsize;
            Ok((
                u32_at(data, o)?,
                u32_at(data, o + 4)?,
                u64_at(data, o + 8)?,
                u64_at(data, o + 24)? as usize,
                u64_at(data, o + 32)? as usize,
                u32_at(data, o + 40)?,
                u32_at(data, o + 44)?,
                u64_at(data, o + 48)?,
            ))
        };
        let names = header(shstrndx)?.3;
        let mut sections = Vec::with_capacity(shnum);
        for i in 0..shnum {
            let (name, kind, _flags, offset, size, link, info, align) = header(i)?;
            sections.push(Section {
                name: cstr_at(data, names + name as usize)?,
                kind,
                offset,
                size,
                link,
                info,
                align,
            });
        }
        Ok(Elf { data, sections })
    }

    fn index(&self, name: &str) -> Option<usize> {
        self.sections.iter().position(|s| s.name == name)
    }

    fn bytes(&self, i: usize) -> &'a [u8] {
        let s = &self.sections[i];
        if s.kind == SHT_NOBITS {
            &[]
        } else {
            &self.data[s.offset..s.offset + s.size]
        }
    }

    fn symbols(&self) -> Result<Vec<Symbol>, String> {
        let Some(i) = self.sections.iter().position(|s| s.kind == SHT_SYMTAB) else { return Ok(Vec::new()) };
        let table = self.bytes(i);
        let strings = self.bytes(self.sections[i].link as usize);
        (0..table.len() / 24)
            .map(|k| {
                let e = &table[24 * k..24 * k + 24];
                Ok(Symbol {
                    name: cstr_at(strings, u32_at(e, 0)? as usize)?,
                    info: e[4],
                    other: e[5],
                    shndx: u16_at(e, 6)?,
                    value: u64_at(e, 8)?,
                    size: u64_at(e, 16)?,
                })
            })
            .collect()
    }

    /// (offset, symbol, type, addend) of the relocations of section `target`.
    fn relocations(&self, target: usize) -> Result<Vec<(u64, u32, u32, i64)>, String> {
        let mut out = Vec::new();
        for (i, s) in self.sections.iter().enumerate() {
            if s.kind == SHT_RELA && s.info as usize == target {
                let b = self.bytes(i);
                for k in 0..b.len() / 24 {
                    let info = u64_at(b, 24 * k + 8)?;
                    out.push((u64_at(b, 24 * k)?, (info >> 32) as u32, info as u32, u64_at(b, 24 * k + 16)? as i64));
                }
            }
        }
        Ok(out)
    }
}

/// The merged object of `objects` (each a kernel's AGX object of the same module), or why they cannot be merged.
pub fn merge(objects: &[Vec<u8>]) -> Result<Vec<u8>, String> {
    let elves = objects.iter().map(|o| Elf::parse(o)).collect::<Result<Vec<_>, _>>()?;
    let first = elves.first().ok_or("no objects to merge")?;
    for e in &elves {
        for s in &e.sections {
            let known = matches!(s.name.as_str(), "" | ".text" | ".agx.kernels" | ".agx.globals" | ".rela.agx.globals"
                | ".note.GNU-stack" | ".symtab" | ".strtab" | ".shstrtab");
            if !known && s.size != 0 {
                return Err(format!("section {} (in a kernel's AGX object) cannot be merged", s.name));
            }
            if s.kind == SHT_RELA && e.sections.get(s.info as usize).map(|t| t.name.as_str()) != Some(".agx.globals") {
                return Err(format!("relocations of section {} cannot be merged", e.sections[s.info as usize].name));
            }
        }
    }
    // .agx.globals: the same in every object
    let globals = first.index(".agx.globals");
    let globals_bytes = globals.map(|g| first.bytes(g)).unwrap_or(&[]);
    for e in &elves[1..] {
        let other = e.index(".agx.globals").map(|g| e.bytes(g)).unwrap_or(&[]);
        if other != globals_bytes {
            return Err("the kernels' objects have different global data".into());
        }
    }
    // .text: each object's code after the previous ones', at the alignment of the most aligned
    let text_align = elves
        .iter()
        .filter_map(|e| e.index(".text").map(|t| e.sections[t].align.max(1)))
        .max()
        .unwrap_or(4);
    let mut text = Vec::new();
    let mut functions = Vec::new(); // (symbol, value in the merged .text)
    let mut kernels = Vec::new();
    for e in &elves {
        let Some(t) = e.index(".text") else { continue };
        while text.len() as u64 % text_align != 0 {
            text.push(0);
        }
        let base = text.len() as u64;
        text.extend_from_slice(e.bytes(t));
        for s in e.symbols()? {
            if s.shndx as usize == t && s.info & 0xf != STT_SECTION && !s.name.is_empty() {
                let value = base + s.value;
                functions.push(Symbol { value, ..s });
            }
        }
        if let Some(k) = e.index(".agx.kernels") {
            kernels.extend_from_slice(e.bytes(k));
        }
    }
    {
        let mut names: Vec<&str> = functions.iter().map(|s| s.name.as_str()).collect();
        names.sort();
        if names.windows(2).any(|w| w[0] == w[1]) {
            return Err("a function is defined in two kernels' objects".into());
        }
    }

    // Sections of the merged object: 0 null, 1 .strtab (names of sections and symbols), 2 .text,
    // 3 .note.GNU-stack, 4 .agx.kernels, 5 .agx.globals, 6 .rela.agx.globals, 7 .symtab
    const TEXT: u16 = 2;
    const GLOBALS: u16 = 5;
    let mut strtab = vec![0u8];
    let mut string = |s: &str| -> u32 {
        if s.is_empty() {
            return 0;
        }
        let at = strtab.len() as u32;
        strtab.extend_from_slice(s.as_bytes());
        strtab.push(0);
        at
    };
    let section_names: Vec<u32> = [
        "", ".strtab", ".text", ".note.GNU-stack", ".agx.kernels", ".agx.globals", ".rela.agx.globals", ".symtab",
    ]
    .iter()
    .map(|n| string(n))
    .collect();

    // Symbols: the null one, the locals (of .agx.globals), then the globals (its symbols and the functions).
    let first_symbols = first.symbols()?;
    let mut symtab: Vec<(u32, u8, u8, u16, u64, u64)> = vec![(0, 0, 0, 0, 0, 0)];
    let mut renumber = vec![0u32; first_symbols.len()];
    for pass_local in [true, false] {
        for (i, s) in first_symbols.iter().enumerate().skip(1) {
            if globals.map_or(true, |g| s.shndx as usize != g) || (s.info >> 4 == 0) != pass_local {
                continue;
            }
            renumber[i] = symtab.len() as u32;
            symtab.push((string(&s.name), s.info, s.other, GLOBALS, s.value, s.size));
        }
    }
    // (ELF wants the local symbols first: .agx.globals' locals, then its globals and the functions, all global)
    let first_global = symtab.iter().skip(1).position(|s| s.1 >> 4 != 0).map_or(symtab.len(), |p| p + 1);
    for f in &functions {
        if f.info >> 4 == 0 {
            return Err(format!("local function symbol {} cannot be merged", f.name));
        }
        symtab.push((string(&f.name), f.info, f.other, TEXT, f.value, f.size));
    }
    let relocations = match globals {
        Some(g) => first.relocations(g)?,
        None => Vec::new(),
    };
    let mut rela = Vec::new();
    for (offset, symbol, kind, addend) in relocations {
        let s = *renumber.get(symbol as usize).ok_or("bad relocation symbol")?;
        if s == 0 && symbol != 0 {
            return Err("a relocation of .agx.globals names a symbol outside it".into());
        }
        rela.extend_from_slice(&offset.to_le_bytes());
        rela.extend_from_slice(&(((s as u64) << 32) | kind as u64).to_le_bytes());
        rela.extend_from_slice(&addend.to_le_bytes());
    }
    let mut symbytes = Vec::with_capacity(24 * symtab.len());
    for (name, info, other, shndx, value, size) in &symtab {
        symbytes.extend_from_slice(&name.to_le_bytes());
        symbytes.push(*info);
        symbytes.push(*other);
        symbytes.extend_from_slice(&shndx.to_le_bytes());
        symbytes.extend_from_slice(&value.to_le_bytes());
        symbytes.extend_from_slice(&size.to_le_bytes());
    }

    // Layout: the ELF header, the sections' contents (aligned), then the section headers.
    let globals_align = globals.map_or(8, |g| first.sections[g].align.max(1));
    let contents: [(&[u8], u64); 8] = [
        (&[], 0),
        (&strtab, 1),
        (&text, text_align),
        (&[], 1),
        (&kernels, 4),
        (globals_bytes, globals_align),
        (&rela, 8),
        (&symbytes, 8),
    ];
    let mut out = vec![0u8; 64];
    let mut placed = Vec::new();
    for (bytes, align) in contents.iter() {
        while out.len() as u64 % align.max(&1) != 0 {
            out.push(0);
        }
        placed.push(out.len() as u64);
        out.extend_from_slice(bytes);
    }
    while out.len() % 8 != 0 {
        out.push(0);
    }
    let shoff = out.len() as u64;
    let headers: [(u32, u64, u32, u32, u64); 8] = [
        // (type, flags, link, info, entsize)
        (0, 0, 0, 0, 0),
        (SHT_STRTAB, 0, 0, 0, 0),
        (SHT_PROGBITS, SHF_ALLOC | SHF_EXECINSTR, 0, 0, 0),
        (SHT_PROGBITS, 0, 0, 0, 0),
        (SHT_PROGBITS, 0, 0, 0, 0),
        (SHT_PROGBITS, SHF_ALLOC | SHF_WRITE, 0, 0, 0),
        (SHT_RELA, SHF_INFO_LINK, 7, GLOBALS as u32, 24),
        (SHT_SYMTAB, 0, 1, first_global as u32, 24),
    ];
    for (i, (kind, flags, link, info, entsize)) in headers.iter().enumerate() {
        out.extend_from_slice(&section_names[i].to_le_bytes());
        out.extend_from_slice(&kind.to_le_bytes());
        out.extend_from_slice(&flags.to_le_bytes());
        out.extend_from_slice(&0u64.to_le_bytes()); // address
        out.extend_from_slice(&(if i == 0 { 0 } else { placed[i] }).to_le_bytes());
        out.extend_from_slice(&(contents[i].0.len() as u64).to_le_bytes());
        out.extend_from_slice(&link.to_le_bytes());
        out.extend_from_slice(&info.to_le_bytes());
        out.extend_from_slice(&(if i == 0 { 0 } else { contents[i].1.max(1) }).to_le_bytes());
        out.extend_from_slice(&entsize.to_le_bytes());
    }
    // ELF header: the first object's, with the merged section headers.
    out[..64].copy_from_slice(&first.data[..64]);
    out[40..48].copy_from_slice(&shoff.to_le_bytes()); // e_shoff
    out[58..60].copy_from_slice(&64u16.to_le_bytes()); // e_shentsize
    out[60..62].copy_from_slice(&(headers.len() as u16).to_le_bytes()); // e_shnum
    out[62..64].copy_from_slice(&1u16.to_le_bytes()); // e_shstrndx
    Ok(out)
}
