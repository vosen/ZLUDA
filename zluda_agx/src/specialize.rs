//! Launch-time specialization: a kernel launched again and again with the same scalar arguments and block size is
//! compiled once more with them as constants (ZLUDA_AGX_SPECIALIZE, see the AGX docs). The PTX of the kernel is
//! rewritten: `ld.param` of a 4-byte scalar parameter (`.u32`, `.s32`, `.b32`, `.f32`) becomes a `mov` of the
//! argument's value; an 8-byte parameter equal to an earlier parameter of the same type reads that earlier parameter;
//! and `mov.u32 %r, %ntid.{x,y,z}` becomes a `mov` of the block's size. Everything else, the other kernels
//! and the parameter list included, stays as it is, so the specialized kernel takes the same arguments.

/// Canonical equality classes of 8-byte launch arguments, appended to the specialization key. Each duplicate is
/// its argument index followed by the first equal earlier index. Values are omitted, so different pointers with
/// the same equality pattern share one compiled kernel.
pub fn equal_eight_byte_args(args: &[&[u8]]) -> Vec<u8> {
    let mut pairs = Vec::new();
    for (i, arg) in args.iter().enumerate() {
        if arg.len() != 8 {
            continue;
        }
        if let Some(j) = args[..i].iter().position(|earlier| earlier.len() == 8 && *earlier == *arg) {
            pairs.extend_from_slice(&(i as u32).to_le_bytes());
            pairs.extend_from_slice(&(j as u32).to_le_bytes());
        }
    }
    pairs
}

/// The module with `kernel` specialized for `args` (the launch's parameter bytes) and `block`, and the number of
/// instructions rewritten; `None` if the kernel is not found or nothing in it depends on them.
pub fn specialize_ptx(src: &str, kernel: &str, args: &[&[u8]], block: [u32; 3]) -> Option<(String, usize)> {
    let entry = find_entry(src, kernel)?;
    let open = entry + src[entry..].find('(')?;
    let close = open + src[open..].find(')')?;
    // index -> (type, name) of scalar parameters we can specialize
    let mut params: Vec<Option<(&str, &str)>> = Vec::new();
    for p in src[open + 1..close].split(',') {
        let words: Vec<&str> = p.split_whitespace().collect();
        params.push(match words.as_slice() {
            [".param", ty @ (".u32" | ".s32" | ".b32" | ".f32" | ".u64" | ".s64" | ".b64"), name]
                if !name.contains('[') => Some((&ty[1..], *name)),
            _ => None,
        });
    }
    let body_start = close + src[close..].find('{')?;
    let body_end = body_start + matching_brace(&src[body_start..])?;
    let mut out = String::with_capacity(src.len());
    out.push_str(&src[..body_start]);
    let mut rewritten = 0;
    for line in src[body_start..body_end].split_inclusive('\n') {
        match rewrite(line, &params, args, block) {
            Some(new) => {
                out.push_str(&new);
                rewritten += 1;
            }
            None => out.push_str(line),
        }
    }
    out.push_str(&src[body_end..]);
    if rewritten == 0 {
        return None;
    }
    let out = without_other_entries(&out, kernel);
    // This exact Geekbench 7 tile shape has been checked against captured GPU data.
    // Keep the optimization behind a complete PTX fingerprint until a general
    // shared-memory layout pass can prove the same transformation for other code.
    let out = crate::gemm_tile::transpose_b(&out, block).unwrap_or(out);
    Some((out, rewritten))
}

/// `src` without the kernels other than `kernel` (they would only add compile time); functions and globals stay,
/// and module-scope `.global` and `.const` variables become `.visible`, so that LLVM keeps all of them: the
/// specialized module then lays out its global data as the module does and uses the module's buffer.
pub(crate) fn without_other_entries(src: &str, kernel: &str) -> String {
    let src = &visible_globals(src);
    let mut out = String::with_capacity(src.len());
    let mut at = 0;
    while let Some(i) = src[at..].find(".entry") {
        let entry = at + i;
        let name_start = entry + ".entry".len();
        let name: String = src[name_start..].trim_start().chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect();
        // the whole declaration from the start of its line (`.visible .entry`, `.weak .entry`, ...) to its body's end
        let line = src[..entry].rfind('\n').map_or(0, |n| n + 1);
        let body = src[entry..].find('{').map(|b| entry + b);
        let end = body.and_then(|b| matching_brace(&src[b..]).map(|e| b + e));
        match end {
            Some(end) if name != kernel && line >= at => {
                out.push_str(&src[at..line]);
                at = end;
            }
            _ => {
                out.push_str(&src[at..name_start]);
                at = name_start;
            }
        }
    }
    out.push_str(&src[at..]);
    out
}

/// Kernel names in a PTX image, in source order. This is used to defer AGX
/// code generation until a CUDA module asks for a particular kernel.
pub(crate) fn entry_names(src: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut at = 0;
    while let Some(i) = src[at..].find(".entry") {
        let start = at + i + ".entry".len();
        let rest = src[start..].trim_start();
        let name: String = rest.chars().take_while(|c| c.is_alphanumeric() || *c == '_' || *c == '$').collect();
        if !name.is_empty() && rest[name.len()..].trim_start().starts_with('(') {
            names.push(name);
        }
        at = start;
    }
    names
}

/// `src` with `.visible` before the module-scope `.global` / `.const` declarations that have no linkage directive.
fn visible_globals(src: &str) -> String {
    let mut out = String::with_capacity(src.len() + 64);
    let mut depth = 0i32;
    for line in src.split_inclusive('\n') {
        let t = line.trim_start();
        if depth == 0 && (t.starts_with(".global ") || t.starts_with(".const ")) {
            out.push_str(&line[..line.len() - t.len()]);
            out.push_str(".visible ");
            out.push_str(t);
        } else {
            out.push_str(line);
        }
        depth += line.matches('{').count() as i32 - line.matches('}').count() as i32;
    }
    out
}

/// The byte offset of `.entry <kernel>` followed by `(`.
fn find_entry(src: &str, kernel: &str) -> Option<usize> {
    let mut at = 0;
    while let Some(i) = src[at..].find(".entry") {
        let start = at + i;
        let rest = src[start + ".entry".len()..].trim_start();
        if let Some(after) = rest.strip_prefix(kernel) {
            if after.trim_start().starts_with('(') {
                return Some(start);
            }
        }
        at = start + ".entry".len();
    }
    None
}

/// The offset just past the `}` that closes the `{` at offset 0.
fn matching_brace(s: &str) -> Option<usize> {
    let mut depth = 0i32;
    for (i, c) in s.char_indices() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }
    None
}

/// The replacement of one line of the kernel body, if it reads a specialized value.
fn rewrite(line: &str, params: &[Option<(&str, &str)>], args: &[&[u8]], block: [u32; 3]) -> Option<String> {
    let indent = &line[..line.len() - line.trim_start().len()];
    let text = line.trim();
    let newline = if line.ends_with('\n') { "\n" } else { "" };
    // mov.u32 %r, %ntid.x;
    if let Some(rest) = text.strip_prefix("mov.u32") {
        let (dst, src) = operands(rest)?;
        let dim = match src {
            "%ntid.x" => 0,
            "%ntid.y" => 1,
            "%ntid.z" => 2,
            _ => return None,
        };
        return Some(format!("{indent}mov.u32 \t{dst}, {};{newline}", block[dim]));
    }
    // ld.param.<ty> %r, [name] / [name+0]
    let rest = text.strip_prefix("ld.param.")?;
    let (ty, rest) = rest.split_at(rest.find(|c: char| c.is_whitespace())?);
    let (dst, addr) = operands(rest)?;
    let name = addr.strip_prefix('[')?.strip_suffix(']')?;
    let name = name.strip_suffix("+0").unwrap_or(name).trim();
    let index = params.iter().position(|s| s.is_some_and(|(_, n)| n == name))?;
    let (pty, _) = params[index]?;
    if ty != pty {
        return None;
    }
    if matches!(ty, "u64" | "s64" | "b64") {
        let value = args.get(index)?;
        if value.len() != 8 {
            return None;
        }
        let (_, earlier) = params[..index].iter().enumerate().find_map(|(j, p)| {
            let (earlier_ty, earlier) = (*p)?;
            (earlier_ty == ty && args.get(j).is_some_and(|a| *a == *value)).then_some((j, earlier))
        })?;
        return Some(format!("{indent}ld.param.{ty} \t{dst}, [{earlier}];{newline}"));
    }
    let bytes: [u8; 4] = args.get(index)?.get(..4)?.try_into().ok()?;
    if args[index].len() != 4 {
        return None;
    }
    let v = u32::from_le_bytes(bytes);
    let value = match ty {
        "f32" => format!("0f{v:08X}"),
        "s32" => format!("{}", v as i32),
        _ => format!("{v}"),
    };
    Some(format!("{indent}mov.{ty} \t{dst}, {value};{newline}"))
}

/// `dst, src;` of an instruction's operand text.
fn operands(rest: &str) -> Option<(&str, &str)> {
    let rest = rest.trim().strip_suffix(';')?;
    let (dst, src) = rest.split_once(',')?;
    Some((dst.trim(), src.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const PTX: &str = ".version 7.0
.target sm_75
.address_size 64
.const .align 4 .b8 table[8] = {1, 0, 0, 0, 2};
.visible .global .align 4 .u32 counter;
.visible .entry other(.param .u32 other_param_0) { ld.param.u32 %r1, [other_param_0]; ret; }
.visible .entry k(
	.param .u64 k_param_0,
	.param .u32 k_param_1,
	.param .f32 k_param_2,
	.param .s32 k_param_3
)
{
	.reg .b32 %r<5>;
	ld.param.u64 	%rd1, [k_param_0];
	ld.param.u32 	%r1, [k_param_1];
	ld.param.f32 	%f1, [k_param_2+0];
	ld.param.s32 	%r2, [k_param_3];
	mov.u32 	%r3, %ntid.x;
	mov.u32 	%r4, %ntid.y;
	{
	mov.u32 	%r4, %tid.x;
	}
	ret;
}
";

    #[test]
    fn scalars_and_block_become_constants() {
        let args: [&[u8]; 4] = [&7u64.to_le_bytes(), &42u32.to_le_bytes(), &1.5f32.to_le_bytes(), &(-3i32).to_le_bytes()];
        let (out, n) = specialize_ptx(PTX, "k", &args, [64, 2, 1]).unwrap();
        assert_eq!(n, 5);
        assert!(out.contains("ld.param.u64 \t%rd1, [k_param_0];"), "pointers stay");
        assert!(out.contains("mov.u32 \t%r1, 42;"));
        assert!(out.contains("mov.f32 \t%f1, 0f3FC00000;"));
        assert!(out.contains("mov.s32 \t%r2, -3;"));
        assert!(out.contains("mov.u32 \t%r3, 64;") && out.contains("mov.u32 \t%r4, 2;"));
        assert!(out.contains("mov.u32 \t%r4, %tid.x;"));
        // the other kernel is left out
        assert!(!out.contains("other") && out.contains(".visible .entry k("));
        // the module's variables are all kept
        assert!(out.contains("\n.visible .const .align 4 .b8 table[8] = {1, 0, 0, 0, 2};\n"));
        assert!(out.contains("\n.visible .global .align 4 .u32 counter;\n"));
        assert!(specialize_ptx(PTX, "missing", &args, [1, 1, 1]).is_none());
    }

    #[test]
    fn generic_kernel_images_keep_globals_and_one_entry() {
        assert_eq!(entry_names(PTX), ["other", "k"]);
        let only_k = without_other_entries(PTX, "k");
        assert_eq!(entry_names(&only_k), ["k"]);
        assert!(only_k.contains(".visible .const .align 4 .b8 table[8]"));
        assert!(only_k.contains(".visible .global .align 4 .u32 counter"));
    }

    #[test]
    fn equal_64_bit_arguments_share_a_parameter_only_for_that_equality_class() {
        let ptx = ".version 7.0\n.target sm_75\n.address_size 64\n.visible .entry copy(\n\
            \t.param .u64 src,\n\
            \t.param .u64 dst,\n\
            \t.param .u32 count\n\
            ) {\n\
            \t.reg .b64 %rd<3>;\n\
            \tld.param.u64 %rd1, [src];\n\
            \tld.param.u64 %rd2, [dst+0];\n\
            \tld.param.u32 %r1, [count];\n\
            \tret;\n}\n";
        let a = 0x1100_4000u64.to_le_bytes();
        let b = 0x1100_8000u64.to_le_bytes();
        let n = 128u32.to_le_bytes();
        let equal: [&[u8]; 3] = [&a, &a, &n];
        let distinct: [&[u8]; 3] = [&a, &b, &n];
        assert_ne!(equal_eight_byte_args(&equal), equal_eight_byte_args(&distinct));
        let (same, _) = specialize_ptx(ptx, "copy", &equal, [64, 1, 1]).unwrap();
        assert!(same.contains("ld.param.u64 \t%rd2, [src];"));
        let (different, _) = specialize_ptx(ptx, "copy", &distinct, [64, 1, 1]).unwrap();
        assert!(different.contains("ld.param.u64 %rd2, [dst+0];"));
    }
}
