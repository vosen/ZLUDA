fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-arg=delayimp.lib");
        println!("cargo:rustc-link-arg=/DELAYLOAD:MIOpen.dll");
        println!("cargo:rustc-link-arg=/DELAYLOAD:amdhip64_7.dll");
    }
}
