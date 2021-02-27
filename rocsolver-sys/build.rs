fn main() {
    println!("cargo:rustc-link-lib=dylib=rocsolver");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
}
