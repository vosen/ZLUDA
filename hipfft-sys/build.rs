fn main() {
    println!("cargo:rustc-link-lib=dylib=hipfft");
    println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
}
