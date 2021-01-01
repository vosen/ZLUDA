//! Build detours.
fn build_detours() {
    cc::Build::new()
        .host("x86_64-pc-windows-msvc")
        .target("x86_64-pc-windows-msvc")
        .include("../ext/detours/src")
        .define("WIN32_LEAN_AND_MEAN", "1")
        .file("../ext/detours/src/creatwth.cpp")
        .file("../ext/detours/src/detours.cpp")
        .file("../ext/detours/src/disasm.cpp")
        .file("../ext/detours/src/image.cpp")
        .file("../ext/detours/src/modules.cpp")
        .compile("detours");
}

#[cfg(not(target_os = "windows"))]
fn main() {}

#[cfg(target_os = "windows")]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_detours();
}
