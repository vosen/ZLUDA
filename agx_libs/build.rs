//! Compiles the MLIR kernels in kernels/ for every AGX chip into fat AGX objects, with the LLVM fork's
//! mlir/utils/agx-kernels/agx-build-fatbin.py and the mlir-opt, mlir-translate and llc of the LLVM build in
//! ZLUDA_LLVM_BUILD_DIR. The LLVM sources are found through ZLUDA_LLVM_SOURCE_DIR or the build's CMakeCache.txt.
//! The kernels go through AIR (`--via-air`: arith-emulate-f64 and convert-gpu-to-air in MLIR, AIR's portable passes,
//! then the AGX backend's agx-lower-air), so they use only what AIR has.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn llvm_source(build: &Path) -> PathBuf {
    if let Some(d) = env::var_os("ZLUDA_LLVM_SOURCE_DIR") {
        return PathBuf::from(d);
    }
    let cache = fs::read_to_string(build.join("CMakeCache.txt")).expect("ZLUDA_LLVM_BUILD_DIR has no CMakeCache.txt");
    let llvm = cache
        .lines()
        .find_map(|l| l.strip_prefix("CMAKE_HOME_DIRECTORY:INTERNAL="))
        .expect("CMakeCache.txt has no CMAKE_HOME_DIRECTORY");
    Path::new(llvm).parent().unwrap().to_path_buf()
}

/// A kernel file with a double precision copy of each of its functions (`<name>_f64`, every `@agx_*` symbol
/// renamed) in the same module: the kernels are written for any float type, so the copy changes f32 to f64, and with
/// `wide_ints` also i32 to i64 (the BLAS reductions keep an index in the bits of a float, and return it or the float
/// in one word).
fn with_f64_copy(text: &str, out: &Path, wide_ints: bool) {
    let open = text.find("gpu.module @").expect("no gpu.module");
    let body_start = text[open..].find('{').unwrap() + open + 1;
    let body_end = text.rfind('}').expect("unterminated module");
    let mut copy = text[body_start..body_end].replace("f32", "f64");
    if wide_ints {
        copy = copy.replace("i32", "i64");
    }
    // @agx_name -> @agx_name_f64
    let mut renamed = String::with_capacity(copy.len());
    let mut rest = copy.as_str();
    while let Some(at) = rest.find("@agx_") {
        let end = rest[at + 1..].find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).map_or(rest.len(), |i| at + 1 + i);
        renamed.push_str(&rest[..end]);
        renamed.push_str("_f64");
        rest = &rest[end..];
    }
    renamed.push_str(rest);
    let all = format!("{}\n  // double precision copies\n{}{}", &text[..body_end], renamed, &text[body_end..]);
    fs::write(out, all).unwrap();
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| panic!("{path}: {e}"))
}

/// sparse.mlir with the functions of sparse_ix.mlir for i32 and for i64 indices (`IXT` the index type, `_IX` the
/// name suffix) at the end of its module.
fn with_index_copies(text: &str, template: &str) -> String {
    let end = text.rfind('}').expect("unterminated module");
    let copies: String = ["i32", "i64"].iter().map(|ty| template.replace("IXT", ty).replace("_IX", &format!("_{ty}"))).collect();
    format!("{}{}{}", &text[..end], copies, &text[end..])
}

fn main() {
    println!("cargo:rerun-if-env-changed=ZLUDA_LLVM_BUILD_DIR");
    println!("cargo:rerun-if-env-changed=ZLUDA_LLVM_SOURCE_DIR");
    println!("cargo:rerun-if-changed=kernels/sgemm.mlir");
    println!("cargo:rerun-if-changed=kernels/hgemm.mlir");
    println!("cargo:rerun-if-changed=kernels/epilogue.mlir");
    println!("cargo:rerun-if-changed=kernels/conv.mlir");
    println!("cargo:rerun-if-changed=kernels/fft.mlir");
    println!("cargo:rerun-if-changed=kernels/sparse.mlir");
    println!("cargo:rerun-if-changed=kernels/sparse_ix.mlir");
    println!("cargo:rerun-if-changed=kernels/blas.mlir");
    println!("cargo:rerun-if-changed=kernels/blasc.mlir");
    println!("cargo:rerun-if-changed=kernels/dnn.mlir");
    println!("cargo:rerun-if-changed=kernels/rand.mlir");
    println!("cargo:rerun-if-changed=kernels/solver.mlir");
    let build = PathBuf::from(env::var_os("ZLUDA_LLVM_BUILD_DIR").expect(
        "agx_libs compiles its MLIR kernels with the LLVM AGX fork: set ZLUDA_LLVM_BUILD_DIR to its build directory",
    ));
    let script = llvm_source(&build).join("mlir/utils/agx-kernels/agx-build-fatbin.py");
    // the pipeline (mlir-opt, opt -O3, llc) lives in the script
    println!("cargo:rerun-if-changed={}", script.display());
    let bin = build.join("bin");
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    for (name, wide) in [("fft", false), ("blas", true), ("blasc", true), ("sparse", false), ("solver", false)] {
        let mut text = read(&format!("kernels/{name}.mlir"));
        if name == "sparse" {
            text = with_index_copies(&text, &read("kernels/sparse_ix.mlir"));
        }
        with_f64_copy(&text, &out_dir.join(format!("{name}.mlir")), wide);
    }
    for name in ["sgemm", "hgemm", "epilogue", "conv", "fft", "sparse", "blas", "blasc", "dnn", "rand", "solver"] {
        let out = out_dir.join(format!("{name}.agxfat"));
        let src = if matches!(name, "fft" | "blas" | "blasc" | "sparse" | "solver") { out_dir.join(format!("{name}.mlir")) } else { PathBuf::from(format!("kernels/{name}.mlir")) };
        let status = Command::new("python3")
            .arg(&script)
            .arg("--mlir-opt")
            .arg(bin.join("mlir-opt"))
            .arg("--mlir-translate")
            .arg(bin.join("mlir-translate"))
            .arg("--llc")
            .arg(bin.join("llc"))
            .arg("--via-air")
            .arg(&src)
            .arg("-o")
            .arg(&out)
            .status()
            .unwrap_or_else(|e| panic!("{}: {e}", script.display()));
        assert!(status.success(), "compiling kernels/{name}.mlir failed");
    }
}
