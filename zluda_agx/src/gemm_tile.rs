//! A guarded layout experiment for Geekbench 7's `gemm_block_size_bias`.
//!
//! The original B tile has eight rows 128 bytes apart, so its inner loop
//! reads four separate threadgroup words. This layout stores those four words
//! consecutively and lets the compiler emit one vector load. The fingerprint
//! covers the entire launch-specialized PTX after replacing its two scalar
//! dimension constants. A different compiler or source shape falls back to
//! the original code. No benchmark PTX is stored here.

const TEMPLATE_BYTES: usize = 11291;
const TEMPLATE_FNV128: u128 = 0xb631671a147addb9f02117ff83a6f570;

fn fingerprint(src: &str) -> u128 {
    src.bytes()
        .fold(0x6c62272e07bb014262b821756295c58du128, |hash, byte| {
            (hash ^ byte as u128).wrapping_mul(0x0000000001000000000000000000013b)
        })
}

fn replace_one(src: String, old: &str, new: &str) -> Option<String> {
    (src.matches(old).count() == 1).then(|| src.replacen(old, new, 1))
}

fn normalized(src: &str) -> Option<String> {
    let mut out = src.to_owned();
    for reg in [41, 42] {
        let prefix = format!("\tmov.u32 \t%r{reg}, ");
        let start = out.find(&prefix)? + prefix.len();
        let end = start + out[start..].find(';')?;
        if !out[start..end].bytes().all(|b| b.is_ascii_digit()) || out[start..].contains(&prefix) {
            return None;
        }
        out.replace_range(start..end, "<V>");
    }
    Some(out)
}

/// Return the rearranged PTX only for the GPU-validated shape and block.
pub fn transpose_b(src: &str, block: [u32; 3]) -> Option<String> {
    if block != [32, 4, 1]
        || std::env::var_os("ZLUDA_AGX_GEMM_TILE").as_deref() == Some(std::ffi::OsStr::new("0"))
    {
        return None;
    }
    let normalized = normalized(src)?;
    if normalized.len() != TEMPLATE_BYTES || fingerprint(&normalized) != TEMPLATE_FNV128 {
        return None;
    }
    let mut out = replace_one(src.to_owned(), ".reg .b32 \t%r<115>;", ".reg .b32 \t%r<133>;")?;
    let mut setup = String::from(concat!(
        "\tadd.s32 \t%r15, %r54, %r59;\n",
        "\tmul.lo.s32 \t%r115, %r3, 12;\n",
        "\tadd.s32 \t%r116, %r15, %r115;"
    ));
    for k in 1..4 {
        setup.push_str(&format!("\n\tadd.s32 \t%r{}, %r116, {};", 116 + k, 4 * k));
    }
    for k in 4..8 {
        setup.push_str(&format!("\n\tadd.s32 \t%r{}, %r116, {};", 116 + k, 512 + 4 * (k - 4)));
    }
    out = replace_one(out, "\tadd.s32 \t%r15, %r54, %r59;", &setup)?;
    for (old, new) in std::iter::once(15).chain(27..34).zip(116..124) {
        out = replace_one(
            out,
            &format!("st.shared.f32 \t[%r{old}],"),
            &format!("st.shared.f32 \t[%r{new}],"),
        )?;
    }
    let load = "\tshr.u32 \t%r124, %r114, 2;\n\tshl.b32 \t%r125, %r124, 9;\n\tshl.b32 \t%r126, %r3, 4;\n\tand.b32 \t%r127, %r114, 3;\n\tshl.b32 \t%r128, %r127, 2;\n\tadd.s32 \t%r129, %r125, %r126;\n\tadd.s32 \t%r130, %r129, %r128;\n\tadd.s32 \t%r131, %r130, 4096;\n\tadd.s32 \t%r132, %r54, %r131;\n\tld.shared.v2.f32 \t{%f93, %f118}, [%r132];";
    out = replace_one(out, "\tld.shared.f32 \t%f93, [%r70];", load)?;
    replace_one(out, "\tld.shared.f32 \t%f118, [%r90];", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unrelated_source_and_block_are_rejected() {
        assert!(transpose_b(".visible .entry gemm_block_size_bias() {}", [32, 4, 1]).is_none());
        assert!(transpose_b("", [64, 2, 1]).is_none());
    }
}
