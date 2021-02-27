fn main() {
    println!("cargo:rustc-link-lib=dylib=rocblas");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
}
