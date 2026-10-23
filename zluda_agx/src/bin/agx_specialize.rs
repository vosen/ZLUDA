//! `agx_specialize PTX KERNEL --block x,y,z ARG...`: prints the PTX that launch-time specialization compiles for a
//! launch of KERNEL with these arguments (see `specialize.rs`), to check it against the generic code on captured
//! launches. Each ARG is one parameter's bytes in hex, little-endian (`2a000000` is the u32 42).
use std::process::ExitCode;

fn main() -> ExitCode {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.len() < 4 || a[2] != "--block" {
        eprintln!("usage: agx_specialize PTX KERNEL --block x,y,z ARG...");
        return ExitCode::from(2);
    }
    let src = std::fs::read_to_string(&a[0]).expect("PTX");
    let block: Vec<u32> = a[3].split(',').map(|v| v.parse().expect("block")).collect();
    let args: Vec<Vec<u8>> = a[4..]
        .iter()
        .map(|h| (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).expect("hex")).collect())
        .collect();
    let refs: Vec<&[u8]> = args.iter().map(|v| &v[..]).collect();
    match nvcuda_agx::specialize::specialize_ptx(&src, &a[1], &refs, [block[0], block[1], block[2]]) {
        Some((text, n)) => {
            eprintln!("{n} instructions rewritten");
            print!("{text}");
            ExitCode::SUCCESS
        }
        None => {
            eprintln!("nothing to specialize");
            ExitCode::FAILURE
        }
    }
}
