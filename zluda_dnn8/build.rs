#[cfg(windows)]
fn main() {
    extern crate embed_resource;
    embed_resource::compile("../zluda_windows/manifest.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
    println!("cargo:rustc-link-arg=delayimp.lib");
    println!("cargo:rustc-link-arg=/DELAYLOAD:amdhip64_7.dll");
}

#[cfg(not(windows))]
fn main() {}
