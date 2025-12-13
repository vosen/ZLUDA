fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-arg=/DELAYLOAD:amdhip64_7.dll");
    } else {
        println!("cargo:rustc-link-lib=dylib=amdhip64");
        println!("cargo:rustc-link-search=native=/opt/rocm/lib/");
    }
}
