use cmake::Config;
use std::io;
use std::path::PathBuf;
use std::process::Command;

const COMPONENTS: &[&'static str] = &[
    "LLVMCore",
    "LLVMBitWriter",
    #[cfg(debug_assertions)]
    "LLVMAnalysis", // for module verify
    #[cfg(debug_assertions)]
    "LLVMBitReader",
];

fn main() {
    let mut cmake = Config::new(r"../ext/llvm-project/llvm");
    try_use_ninja(&mut cmake);
    cmake
        // For some reason Rust always links to release MSVCRT
        .define("CMAKE_MSVC_RUNTIME_LIBRARY", "MultiThreadedDLL")
        .define("LLVM_ENABLE_TERMINFO", "OFF")
        .define("LLVM_ENABLE_LIBXML2", "OFF")
        .define("LLVM_ENABLE_LIBEDIT", "OFF")
        .define("LLVM_ENABLE_LIBPFM", "OFF")
        .define("LLVM_ENABLE_ZLIB", "OFF")
        .define("LLVM_ENABLE_ZSTD", "OFF")
        .define("LLVM_INCLUDE_BENCHMARKS", "OFF")
        .define("LLVM_INCLUDE_EXAMPLES", "OFF")
        .define("LLVM_INCLUDE_TESTS", "OFF")
        .define("LLVM_BUILD_TOOLS", "OFF")
        .define("LLVM_TARGETS_TO_BUILD", "")
        .define("LLVM_ENABLE_PROJECTS", "");
    cmake.build_target("llvm-config");
    let llvm_dir = cmake.build();
    for c in COMPONENTS {
        cmake.build_target(c);
        cmake.build();
    }
    let cmake_profile = cmake.get_profile();
    let (cxxflags, ldflags, libdir, lib_names, system_libs) =
        llvm_config(&llvm_dir, &["build", "bin", "llvm-config"])
            .or_else(|_| llvm_config(&llvm_dir, &["build", cmake_profile, "bin", "llvm-config"]))
            .unwrap();
    println!("cargo:rustc-link-arg={ldflags}");
    println!("cargo:rustc-link-search=native={libdir}");
    for lib in system_libs.split_ascii_whitespace() {
        println!("cargo:rustc-link-arg={lib}");
    }
    link_llvm_components(lib_names);
    compile_cxx_lib(cxxflags);
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

fn llvm_config(
    llvm_build_dir: &PathBuf,
    path_to_llvm_config: &[&str],
) -> io::Result<(String, String, String, String, String)> {
    let mut llvm_build_path = llvm_build_dir.clone();
    llvm_build_path.extend(path_to_llvm_config);
    let mut cmd = Command::new(llvm_build_path);
    cmd.args([
        "--link-static",
        "--cxxflags",
        "--ldflags",
        "--libdir",
        "--libnames",
        "--system-libs",
    ]);
    for c in COMPONENTS {
        cmd.arg(c[4..].to_lowercase());
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

fn compile_cxx_lib(cxxflags: String) {
    let mut cc = cc::Build::new();
    for flag in cxxflags.split_whitespace() {
        cc.flag(flag);
    }
    cc.cpp(true).file("src/lib.cpp").compile("llvm_zluda_cpp");
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
}

fn link_llvm_components(components: String) {
    for component in components.split_whitespace() {
        let component = if let Some(component) = component
            .strip_prefix("lib")
            .and_then(|component| component.strip_suffix(".a"))
        {
            // Unix (Linux/Mac)
            // libLLVMfoo.a
            component
        } else if let Some(component) = component.strip_suffix(".lib") {
            // Windows
            // LLVMfoo.lib
            component
        } else {
            panic!("'{}' does not look like a static library name", component)
        };
        println!("cargo:rustc-link-lib={component}");
    }
}
