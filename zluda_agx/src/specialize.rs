//! Launch-time specialization: a kernel launched again and again with the same scalar arguments and block size is
//! compiled once more with them as constants (ZLUDA_AGX_SPECIALIZE, see the AGX docs). The PTX of the kernel is
//! rewritten: `ld.param` of a 4-byte scalar parameter (`.u32`, `.s32`, `.b32`, `.f32`) becomes a `mov` of the
//! argument's value, and `mov.u32 %r, %ntid.{x,y,z}` a `mov` of the block's size. Everything else, the other kernels
//! and the parameter list included, stays as it is, so the specialized kernel takes the same arguments.

/// The module with `kernel` specialized for `args` (the launch's parameter bytes) and `block`, and the number of
/// instructions rewritten; `None` if the kernel is not found or nothing in it depends on them.
pub fn specialize_ptx(src: &str, kernel: &str, args: &[&[u8]], block: [u32; 3]) -> Option<(String, usize)> {
    let entry = find_entry(src, kernel)?;
    let open = entry + src[entry..].find('(')?;
    let close = open + src[open..].find(')')?;
    // index -> (type, name) of the 4-byte scalar parameters
    let mut scalars: Vec<Option<(&str, &str)>> = Vec::new();
    for p in src[open + 1..close].split(',') {
        let words: Vec<&str> = p.split_whitespace().collect();
        scalars.push(match words.as_slice() {
            [".param", ty @ (".u32" | ".s32" | ".b32" | ".f32"), name] if !name.contains('[') => Some((&ty[1..], *name)),
            _ => None,
        });
    }
    let body_start = close + src[close..].find('{')?;
    let body_end = body_start + matching_brace(&src[body_start..])?;
    let mut out = String::with_capacity(src.len());
    out.push_str(&src[..body_start]);
    let mut rewritten = 0;
    for line in src[body_start..body_end].split_inclusive('\n') {
        match rewrite(line, &scalars, args, block) {
            Some(new) => {
                out.push_str(&new);
                rewritten += 1;
            }
            None => out.push_str(line),
        }
    }
    out.push_str(&src[body_end..]);
    (rewritten > 0).then(|| (without_other_entries(&out, kernel), rewritten))
}

/// `src` without the kernels other than `kernel` (they would only add compile time); functions and globals stay,
/// and module-scope `.global` and `.const` variables become `.visible`, so that LLVM keeps all of them: the
/// specialized module then lays out its global data as the module does and uses the module's buffer.
fn without_other_entries(src: &str, kernel: &str) -> String {
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
fn rewrite(line: &str, scalars: &[Option<(&str, &str)>], args: &[&[u8]], block: [u32; 3]) -> Option<String> {
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
    let index = scalars.iter().position(|s| s.is_some_and(|(_, n)| n == name))?;
    let (pty, _) = scalars[index]?;
    let bytes: [u8; 4] = args.get(index)?.get(..4)?.try_into().ok()?;
    if args[index].len() != 4 || ty != pty {
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
}
