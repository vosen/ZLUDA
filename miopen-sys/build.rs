fn main() {
    println!("cargo:rustc-link-lib=dylib=MIOpen");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
}
