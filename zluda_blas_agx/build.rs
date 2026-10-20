//! Compiles the MLIR kernels in kernels/ for every AGX chip into one fat AGX object, with the LLVM fork's
//! mlir/utils/agx-kernels/agx-build-fatbin.py and the mlir-opt, mlir-translate and llc of the LLVM build in
//! ZLUDA_LLVM_BUILD_DIR. The LLVM sources are found through ZLUDA_LLVM_SOURCE_DIR or the build's CMakeCache.txt.

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

fn main() {
    println!("cargo:rerun-if-env-changed=ZLUDA_LLVM_BUILD_DIR");
    println!("cargo:rerun-if-env-changed=ZLUDA_LLVM_SOURCE_DIR");
    println!("cargo:rerun-if-changed=kernels/sgemm.mlir");
    println!("cargo:rerun-if-changed=kernels/hgemm.mlir");
    println!("cargo:rerun-if-changed=kernels/epilogue.mlir");
    let build = PathBuf::from(env::var_os("ZLUDA_LLVM_BUILD_DIR").expect(
        "zluda_blas_agx compiles its MLIR kernels with the LLVM AGX fork: set ZLUDA_LLVM_BUILD_DIR to its build directory",
    ));
    let script = llvm_source(&build).join("mlir/utils/agx-kernels/agx-build-fatbin.py");
    let bin = build.join("bin");
    for name in ["sgemm", "hgemm", "epilogue"] {
        let out = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join(format!("{name}.agxfat"));
        let status = Command::new("python3")
            .arg(&script)
            .arg("--mlir-opt")
            .arg(bin.join("mlir-opt"))
            .arg("--mlir-translate")
            .arg(bin.join("mlir-translate"))
            .arg("--llc")
            .arg(bin.join("llc"))
            .arg(format!("kernels/{name}.mlir"))
            .arg("-o")
            .arg(&out)
            .status()
            .unwrap_or_else(|e| panic!("{}: {e}", script.display()));
        assert!(status.success(), "compiling kernels/{name}.mlir failed");
    }
}
