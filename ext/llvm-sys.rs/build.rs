extern crate cmake;
extern crate convert_case;

use convert_case::{Case, Casing, StateConverter};
use std::{
    env,
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
    let out_dir = build_cmake_targets(llvm_components.clone(), llvm_dir, additonal_cmake_file);
    emit_compile_and_linking_information(llvm_components, out_dir, msvc)
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
) -> PathBuf {
    let mut cmake = Config::new(llvm_dir);
    use_ninja(&mut cmake);
    cmake
        .always_configure(true)
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
    cmake.build_target("llvm-config").build()
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
    llvm_components: impl Iterator<Item = StateConverter<'a, &'static str>>,
    out_dir: PathBuf,
    is_msvc: bool,
) {
    let mut llvm_config_path = out_dir.clone();
    llvm_config_path.push("build");
    llvm_config_path.push("bin");
    llvm_config_path.push("llvm-config");
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
    let llvm_config_output = llvm_config_cmd
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .output()
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
