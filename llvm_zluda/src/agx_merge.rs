//! One AGX object from several: the objects of a module's kernels compiled one at a time (compile_air's AGX code a
//! kernel at a time), merged into the object a whole-module compilation makes, which agx_runtime loads
//! (agx_runtime/src/module.rs): every kernel's code in `.text` with a `FUNC` symbol each, the kernels' records in
//! `.agx.kernels`, and the global data buffer's initial contents in `.agx.globals` with its symbols and
//! relocations.
//!
//! The kernels' objects have the same `.agx.globals` (each kernel's module keeps every global variable), which the
//! merged object takes once, and `.agx.kernels` records name their kernels. `.text` has no relocations, or only
//! calls (R_AGX_CALL32, llc -agx-call-relocations, with the backend's experimental calls): then the merged `.text`
//! has every kernel first and one copy of each called function that is the same in several kernels' objects (its
//! bytes and what it calls), each call resolved to it (Blender Cycles: 1338 copies of 213 functions otherwise). An
//! object with any other section that has contents is not merged (an error: the caller compiles the module whole).

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
/// A call's 32-bit offset from the start of the call instruction (two bytes before the field) to the function:
/// S + A - P + 2 (AGXELFObjectWriter)
const R_AGX_CALL32: u32 = 3;
const STB_LOCAL: u8 = 0;

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

/// .text as each object's after the previous ones', and its functions (with their values in it).
fn concatenate(elves: &[Elf], text_align: u64) -> Result<(Vec<u8>, Vec<Symbol>), String> {
    let mut text = Vec::new();
    let mut functions = Vec::new();
    for e in elves {
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
    }
    Ok((text, functions))
}

/// .text of objects whose calls are relocations, and its functions: the kernels (global functions), each object's
/// in turn, then one copy of each called (local) function that is the same in several objects (its name, its bytes
/// without the calls' fields, and the copies it calls), every call resolved to the copy kept. A kernel's code, from
/// its symbol to the end of .text (as agx_runtime loads a module with called functions), holds all it calls.
fn link_calls(elves: &[Elf], text_align: u64) -> Result<(Vec<u8>, Vec<Symbol>), String> {
    struct Func<'a> {
        symbol: Symbol,
        bytes: &'a [u8],
        // (the call's field, from the function's start; the function it calls)
        calls: Vec<(u64, usize)>,
    }
    let mut funcs: Vec<Func> = Vec::new();
    for e in elves {
        let Some(t) = e.index(".text") else { continue };
        let text = e.bytes(t);
        let symbols = e.symbols()?;
        let first = funcs.len();
        for s in &symbols {
            if s.shndx as usize == t && s.info & 0xf != STT_SECTION && !s.name.is_empty() {
                let bytes = text
                    .get(s.value as usize..(s.value + s.size) as usize)
                    .ok_or_else(|| format!("function {} outside .text", s.name))?;
                funcs.push(Func { symbol: Symbol { name: s.name.clone(), ..*s }, bytes, calls: Vec::new() });
            }
        }
        for (offset, symbol, kind, addend) in e.relocations(t)? {
            if kind != R_AGX_CALL32 {
                return Err(format!("relocation type {kind} of .text cannot be merged"));
            }
            let s = symbols.get(symbol as usize).ok_or("bad relocation symbol")?;
            if s.shndx as usize != t {
                return Err("a call to a function outside .text".into());
            }
            let target = s.value as i64 + addend;
            let from = (first..funcs.len())
                .find(|&i| funcs[i].symbol.value <= offset && offset + 4 <= funcs[i].symbol.value + funcs[i].symbol.size)
                .ok_or("a call outside every function")?;
            let to = (first..funcs.len())
                .find(|&i| funcs[i].symbol.value as i64 == target)
                .ok_or("a call to no function's start")?;
            let field = offset - funcs[from].symbol.value;
            funcs[from].calls.push((field, to));
        }
    }
    // The copy kept of each function (kernels: themselves), in the order first met; calls form no cycles (AGX has
    // no recursion), and a function's key holds the copies it calls.
    fn kept(
        i: usize,
        funcs: &[Func],
        memo: &mut Vec<Option<usize>>,
        seen: &mut std::collections::HashMap<(String, Vec<u8>, Vec<(u64, usize)>), usize>,
        depth: usize,
    ) -> Result<usize, String> {
        if let Some(k) = memo[i] {
            return Ok(k);
        }
        if depth > funcs.len() {
            return Err("recursive calls cannot be merged".into());
        }
        let f = &funcs[i];
        let mut calls = Vec::with_capacity(f.calls.len());
        for &(field, to) in &f.calls {
            calls.push((field, kept(to, funcs, memo, seen, depth + 1)?));
        }
        let k = if f.symbol.info >> 4 != STB_LOCAL {
            i
        } else {
            let mut bytes = f.bytes.to_vec();
            for &(field, _) in &f.calls {
                bytes[field as usize..field as usize + 4].fill(0);
            }
            *seen.entry((f.symbol.name.clone(), bytes, calls)).or_insert(i)
        };
        memo[i] = Some(k);
        Ok(k)
    }
    let mut memo = vec![None; funcs.len()];
    let mut seen = std::collections::HashMap::new();
    let mut keep = Vec::with_capacity(funcs.len());
    for i in 0..funcs.len() {
        keep.push(kept(i, &funcs, &mut memo, &mut seen, 0)?);
    }
    // Kernels first, then the copies kept, each at the alignment of the most aligned .text
    let mut order: Vec<usize> = (0..funcs.len()).filter(|&i| funcs[i].symbol.info >> 4 != STB_LOCAL).collect();
    order.extend((0..funcs.len()).filter(|&i| funcs[i].symbol.info >> 4 == STB_LOCAL && keep[i] == i));
    let mut text = Vec::new();
    let mut address = vec![u64::MAX; funcs.len()];
    for &i in &order {
        while text.len() as u64 % text_align != 0 {
            text.push(0);
        }
        address[i] = text.len() as u64;
        text.extend_from_slice(funcs[i].bytes);
    }
    for &i in &order {
        for &(field, to) in &funcs[i].calls {
            let p = address[i] + field;
            let s = address[keep[to]];
            let value = s as i64 - p as i64 + 2;
            let value = i32::try_from(value).map_err(|_| "a call out of range")?;
            text[p as usize..p as usize + 4].copy_from_slice(&value.to_le_bytes());
        }
    }
    let functions = order
        .iter()
        .map(|&i| Symbol { name: funcs[i].symbol.name.clone(), value: address[i], ..funcs[i].symbol })
        .collect();
    Ok((text, functions))
}

/// Whether `object` has relocations of .text (calls, llc -agx-call-relocations), which `merge` resolves.
pub fn has_call_relocations(object: &[u8]) -> bool {
    Elf::parse(object).is_ok_and(|e| {
        e.index(".text").is_some_and(|t| e.sections.iter().any(|s| s.kind == SHT_RELA && s.info as usize == t))
    })
}

/// The merged object of `objects` (each a kernel's AGX object of the same module), or why they cannot be merged.
pub fn merge(objects: &[Vec<u8>]) -> Result<Vec<u8>, String> {
    let elves = objects.iter().map(|o| Elf::parse(o)).collect::<Result<Vec<_>, _>>()?;
    let first = elves.first().ok_or("no objects to merge")?;
    for e in &elves {
        for s in &e.sections {
            let known = matches!(s.name.as_str(), "" | ".text" | ".rela.text" | ".agx.kernels" | ".agx.globals" | ".rela.agx.globals"
                | ".note.GNU-stack" | ".symtab" | ".strtab" | ".shstrtab");
            if !known && s.size != 0 {
                return Err(format!("section {} (in a kernel's AGX object) cannot be merged", s.name));
            }
            let target = e.sections.get(s.info as usize).map(|t| t.name.as_str());
            if s.kind == SHT_RELA && target != Some(".agx.globals") && target != Some(".text") {
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
    let mut kernels = Vec::new();
    for e in &elves {
        if let Some(k) = e.index(".agx.kernels") {
            kernels.extend_from_slice(e.bytes(k));
        }
    }
    let calls = elves.iter().any(|e| e.index(".text").is_some_and(|t| e.sections.iter().any(|s| s.kind == SHT_RELA && s.info as usize == t)));
    let (text, functions) = if calls { link_calls(&elves, text_align)? } else { concatenate(&elves, text_align)? };
    {
        // (only global ones: with experimental calls, each kernel's object has its own copies of the functions it
        // calls, local, and its calls to them are resolved within the object, so they stay right in the merged .text)
        let mut names: Vec<&str> =
            functions.iter().filter(|s| s.info >> 4 != STB_LOCAL).map(|s| s.name.as_str()).collect();
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
    // (ELF wants the local symbols first: .agx.globals' locals and the local functions (the called ones of
    // experimental calls), then its globals and the kernels)
    for pass_local in [true, false] {
        for (i, s) in first_symbols.iter().enumerate().skip(1) {
            if globals.map_or(true, |g| s.shndx as usize != g) || (s.info >> 4 == STB_LOCAL) != pass_local {
                continue;
            }
            renumber[i] = symtab.len() as u32;
            symtab.push((string(&s.name), s.info, s.other, GLOBALS, s.value, s.size));
        }
        for f in functions.iter().filter(|f| (f.info >> 4 == STB_LOCAL) == pass_local) {
            symtab.push((string(&f.name), f.info, f.other, TEXT, f.value, f.size));
        }
    }
    let first_global = symtab.iter().skip(1).position(|s| s.1 >> 4 != STB_LOCAL).map_or(symtab.len(), |p| p + 1);
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
