fn main() {
    println!("cargo:rustc-link-lib=dylib=rocsparse");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
}
