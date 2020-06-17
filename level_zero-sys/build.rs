
fn main() {
    println!("cargo:rustc-link-lib=dylib=ze_loader");
    // TODO: make this windows-only
    println!("cargo:rustc-link-search=native=C:\\Windows\\System32");
    println!("cargo:rerun-if-changed=build.rs");
}