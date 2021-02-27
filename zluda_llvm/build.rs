use std::env;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.cpp");
    println!("cargo:rerun-if-changed=src/lib.rs");
    let llvm_cxxflags = env::var("DEP_LLVM_15_CXXFLAGS").unwrap();
    let mut cc = cc::Build::new();
    for flag in llvm_cxxflags.split_ascii_whitespace() {
        cc.flag(flag);
    }
    cc.shared_flag(true)
        .file("src/lib.cpp")
        .compile("llvm_zluda_cpp");
    // rustc-link-lib and rustc-link-search are already set by cc
}
