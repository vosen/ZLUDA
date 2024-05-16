extern crate cmake;
extern crate convert_case;

use convert_case::{Case, Casing, StateConverter};
use std::{
    env, io,
    path::PathBuf,
    process::{Command, Stdio},
};

use cmake::Config;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let llvm_components = ["core", "analysis", "bit_reader", "bit_writer", "i_r_reader"]
        .iter()
        .map(|comp| comp.from_case(Case::Snake));
    let msvc = is_msvc();
    let (llvm_dir, additonal_cmake_file) = get_llvm_dir();
    let (cmake_profile, out_dir) =
        build_cmake_targets(llvm_components.clone(), llvm_dir, additonal_cmake_file);
    emit_compile_and_linking_information(llvm_components, cmake_profile, out_dir, msvc)
}

fn is_msvc() -> bool {
    env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc"
}

fn get_llvm_dir() -> (PathBuf, PathBuf) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut additional_cmake_file = manifest_dir.clone();
    additional_cmake_file.push("build.cmake");
    let mut llvm_dir = manifest_dir.parent().unwrap().to_path_buf();
    llvm_dir.push("llvm-project");
    llvm_dir.push("llvm");
    println!("cargo:rerun-if-changed={}", llvm_dir.display());
    println!("cargo:rerun-if-changed={}", additional_cmake_file.display());
    (llvm_dir, additional_cmake_file)
}

fn build_cmake_targets<'a>(
    components: impl Iterator<Item = StateConverter<'a, &'static str>>,
    llvm_dir: PathBuf,
    additional_cmake_file: PathBuf,
) -> (String, PathBuf) {
    let mut cmake = Config::new(llvm_dir);
    use_ninja(&mut cmake);
    cmake
        .always_configure(true)
        // Should be detected automatically, but we have reports of
        // LLVM fiding ZLIB on Windows and then failing to link it.
        // Out of caution we explicitly disable all autodetectable components
        .define("LLVM_ENABLE_LIBXML2", "OFF")
        .define("LLVM_ENABLE_ZLIB", "OFF")
        .define("LLVM_ENABLE_ZSTD", "OFF")
        .define("LLVM_ENABLE_CURL", "OFF")
        .define("LLVM_ENABLE_HTTPLIB", "OFF")
        .define("LLVM_ENABLE_LIBEDIT", "OFF")
        .define("LLVM_ENABLE_TERMINFO", "OFF")
        .define("LLVM_BUILD_TOOLS", "OFF")
        .define("LLVM_TARGETS_TO_BUILD", "")
        .define("LLVM_ENABLE_PROJECTS", "")
        .define("CMAKE_PROJECT_INCLUDE_BEFORE", additional_cmake_file);
    // Unfortunately CMake crate does not support building mutliple targets at once
    for component in components {
        cmake
            .build_target(&format!("LLVM{}", component.to_case(Case::Pascal)))
            .build();
    }
    (
        cmake.get_profile().to_string(),
        cmake.build_target("llvm-config").build(),
    )
}

fn use_ninja(cmake: &mut Config) {
    if let Ok(exit_status) = Command::new("ninja")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .arg("--version")
        .status()
    {
        if !exit_status.success() {
            return;
        }
        cmake.generator("Ninja");
    }
}

fn emit_compile_and_linking_information<'a>(
    llvm_components: impl Iterator<Item = StateConverter<'a, &'static str>> + Clone,
    cmake_profile: String,
    out_dir: PathBuf,
    is_msvc: bool,
) {
    // MSBuild uses didfferent output path from ninja or Makefile.
    // Not sure how to query CMake about it, so we just try once with
    // ninja/Makefile path and then once with MSBuild path
    let llvm_config_output = execute_llvm_config(
        &out_dir,
        &["build", "bin", "llvm-config"],
        llvm_components.clone(),
    )
    .or_else(|_| {
        execute_llvm_config(
            &out_dir,
            &["build", &*cmake_profile, "bin", "llvm-config"],
            llvm_components,
        )
    })
    .unwrap();
    if !llvm_config_output.status.success() {
        panic!()
    }
    let output = String::from_utf8_lossy(&llvm_config_output.stdout);
    let lines = (&output).lines().collect::<Vec<_>>();
    let (cxxflags, ldflags, libdir, libnames, system_libs) = match &*lines {
        [cxxflags, ldflags, libdir, libnames, system_libs, ..] => {
            (cxxflags, ldflags, libdir, libnames, system_libs)
        }
        _ => panic!(),
    };
    println!("cargo:cxxflags={}", cxxflags);
    println!("cargo:rustc-link-arg={}", ldflags);
    println!("cargo:rustc-link-search={}", libdir);
    for lib in libnames.split_ascii_whitespace() {
        let lib = if is_msvc {
            // For some reason rustc appends .lib twice on msvc
            &lib[..lib.len() - 4]
        } else {
            // On Linux, we get "libLLVMIRReader.a", so we cut "lib" and ".a"
            &lib[3..lib.len() - 2]
        };
        println!("cargo:rustc-link-lib=static={}", lib);
    }
    for lib in system_libs.split_ascii_whitespace() {
        let lib = if is_msvc {
            &lib[..lib.len() - 4]
        } else {
            // On Linux, we get it as "-lxml2", so we cut "-l" part
            &lib[2..]
        };
        println!("cargo:rustc-link-lib={}", lib);
    }
    if !is_msvc {
        println!("cargo:rustc-link-lib=stdc++");
    }
}

fn execute_llvm_config<'a>(
    out_dir: &PathBuf,
    llvm_config_exe_relative: &[&str],
    llvm_components: impl Iterator<Item = StateConverter<'a, &'static str>>,
) -> io::Result<std::process::Output> {
    let mut llvm_config_path = out_dir.clone();
    llvm_config_path.extend(llvm_config_exe_relative);
    let mut llvm_config_cmd = Command::new(&llvm_config_path);
    llvm_config_cmd.args([
        "--cxxflags",
        "--ldflags",
        "--libdir",
        "--libnames",
        "--system-libs",
        "--link-static",
    ]);
    for component in llvm_components {
        llvm_config_cmd.arg(&component.to_case(Case::Flat));
    }
    llvm_config_cmd
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
}
