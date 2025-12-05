use cmake::Config;
use core::panic;
use std::path::PathBuf;
use std::process::Command;
use std::{env, io};

const COMPONENTS: &[&'static str] = &[
    "LLVMCore",
    "LLVMBitWriter",
    "AMDGPU",
    "llvm-libraries",
    "lldCommon",
    "lldELF",
    "LLVMBitReader",
    "LLVMAnalysis", // for module verify
];

fn main() {
    let mut cmake = Config::new(r"../ext/llvm-project/llvm");
    try_use_sccache(&mut cmake);
    try_use_ninja(&mut cmake);
    cmake
        // It's not like we can do anything about the warnings
        .define("LLVM_ENABLE_WARNINGS", "OFF")
        // For some reason Rust always links to release CRT
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreaded")
        .define("LLVM_ENABLE_TERMINFO", "OFF")
        .define("LLVM_ENABLE_LIBXML2", "OFF")
        .define("LLVM_ENABLE_LIBEDIT", "OFF")
        .define("LLVM_ENABLE_LIBPFM", "OFF")
        .define("LLVM_ENABLE_ZLIB", "OFF")
        .define("LLVM_ENABLE_ZSTD", "OFF")
        .define("LLVM_INCLUDE_BENCHMARKS", "OFF")
        .define("LLVM_INCLUDE_EXAMPLES", "OFF")
        .define("LLVM_INCLUDE_TESTS", "OFF")
        .define("LLVM_BUILD_TESTS", "OFF")
        .define("LLVM_BUILD_TOOLS", "OFF")
        .define("LLVM_OPTIMIZED_TABLEGEN", "ON")
        //.define("LLVM_USE_LINKER", "lld")
        .define("LLVM_TARGETS_TO_BUILD", "AMDGPU")
        .define("LLVM_ENABLE_PROJECTS", "llvm;lld");
    // Debug build is so slow that no debugging of ZLUDA is possible.
    // If you want to build with debug symbols, use dev-llvm profile.
    if try_get_build_profile_name() == "debug" {
        cmake.profile("Release");
    }
    cmake.build_target("llvm-config");
    let llvm_dir = cmake.build();
    for c in COMPONENTS {
        cmake.build_target(c);
        cmake.build();
    }
    let cmake_profile = cmake.get_profile();
    let llvm_build_path = get_llvm_build_path(&llvm_dir, cmake_profile);
    println!("cargo:build-path={}", llvm_build_path.display());
    let (cxxflags, ldflags, libdir, lib_names, system_libs) =
        llvm_config(&llvm_build_path.join("bin/llvm-config")).unwrap();
    println!("cargo:rustc-link-arg={ldflags}");
    println!("cargo:rustc-link-search=native={libdir}");
    for lib in system_libs.split_ascii_whitespace() {
        println!("cargo:rustc-link-arg={lib}");
    }
    link_lld_components();
    link_llvm_components(lib_names);
    compile_cxx_lib(cxxflags, &llvm_build_path);
}

// https://github.com/mozilla/sccache/blob/main/README.md#usage
fn try_use_sccache(cmake: &mut Config) {
    if let Ok(sccache) = std::env::var("SCCACHE_PATH") {
        cmake.define("CMAKE_CXX_COMPILER_LAUNCHER", &*sccache);
        cmake.define("CMAKE_C_COMPILER_LAUNCHER", &*sccache);
        match std::env::var_os("CARGO_CFG_TARGET_OS") {
            Some(os) if os == "windows" => {
                cmake.define(
                    "CMAKE_MSVC_DEBUG_INFORMATION_FORMAT",
                    "$<$<CONFIG:Debug,RelWithDebInfo>:Embedded>",
                );
                cmake.define("CMAKE_POLICY_CMP0141", "NEW");
            }
            _ => {}
        }
    }
}

fn try_use_ninja(cmake: &mut Config) {
    let mut cmd = Command::new("ninja");
    cmd.arg("--version");
    if let Ok(status) = cmd.status() {
        if status.success() {
            cmake.generator("Ninja");
        }
    }
}

// Source: https://stackoverflow.com/a/73603419
// We resort to this garbage because Cargo is ideologically opposed to
// being useful, see here: https://github.com/rust-lang/cargo/issues/11054
fn try_get_build_profile_name() -> String {
    let mut path = PathBuf::from(env::var("OUT_DIR").unwrap());
    path.pop();
    path.pop();
    path.pop();
    let name = path.file_name().unwrap().to_str().unwrap();
    if !["debug", "dev-llvm", "release", "release-lto"].contains(&name) {
        panic!("Unknown build profile: {}", name);
    }
    name.to_string()
}

fn get_llvm_build_path(llvm_build_dir: &PathBuf, cmake_profile: &str) -> PathBuf {
    let mut with_cmake = llvm_build_dir.clone();
    with_cmake.extend(&["build", cmake_profile]);
    if with_cmake.exists() {
        return with_cmake;
    }

    let mut llvm_build_path = llvm_build_dir.clone();
    llvm_build_path.extend(&["build"]);

    return llvm_build_path;
}

fn llvm_config(llvm_config_path: &PathBuf) -> io::Result<(String, String, String, String, String)> {
    let mut cmd = Command::new(llvm_config_path);
    cmd.args([
        "--link-static",
        "--cxxflags",
        "--ldflags",
        "--libdir",
        "--libnames",
        "--system-libs",
    ]);
    cmd.arg("core");
    cmd.arg("bitwriter");
    cmd.arg("amdgpu");
    cmd.arg("option");
    cmd.arg("codegen");
    cmd.arg("passes");
    // LLD seems to require this
    cmd.arg("lto");

    if cfg!(debug_assertions) {
        cmd.arg("analysis");
        cmd.arg("bitreader");
    }

    let output = cmd.output()?;
    if !output.status.success() {
        return Err(io::Error::from(io::ErrorKind::Other));
    }
    let output = unsafe { String::from_utf8_unchecked(output.stdout) };
    let mut lines = output.lines();
    let cxxflags = lines.next().unwrap();
    let ldflags = lines.next().unwrap();
    let libdir = lines.next().unwrap();
    let lib_names = lines.next().unwrap();
    let system_libs = lines.next().unwrap();
    Ok((
        cxxflags.to_string(),
        ldflags.to_string(),
        libdir.to_string(),
        lib_names.to_string(),
        system_libs.to_string(),
    ))
}

fn compile_cxx_lib(cxxflags: String, llvm_build_path: &PathBuf) {
    let mut cc = cc::Build::new();
    for flag in cxxflags.split_whitespace() {
        cc.flag(flag);
    }

    let lld_include = PathBuf::from("../ext/llvm-project/lld/include");
    cc.include(&lld_include);
    let lld_build_include = llvm_build_path.join("tools/lld/include");
    cc.include(&lld_build_include);

    cc.cpp(true).file("src/lib.cpp").compile("llvm_zluda_cpp");
    println!("cargo:rerun-if-changed=../ext/llvm-project");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn get_library_name(file_name: &str) -> Option<&str> {
    if let Some(name) = file_name
        .strip_prefix("lib")
        .and_then(|name| name.strip_suffix(".a"))
    {
        // Unix (Linux/Mac)
        // libLLVMfoo.a
        Some(name)
    } else if let Some(name) = file_name.strip_suffix(".lib") {
        // Windows
        // LLVMfoo.lib
        Some(name)
    } else {
        None
    }
}

fn link_llvm_components(components: String) {
    for component in components.split_whitespace() {
        let component = get_library_name(component).expect(&format!(
            "'{}' does not look like a static library name",
            component
        ));
        println!("cargo:rustc-link-lib={component}");
    }
}

fn link_lld_components() {
    println!("cargo:rustc-link-lib=lldELF");
    println!("cargo:rustc-link-lib=lldCommon");
}
