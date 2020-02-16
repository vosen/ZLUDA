
fn main() {
    println!("cargo:rustc-link-lib=dylib=ze_loader");
    println!("cargo:rerun-if-changed=build.rs");
}