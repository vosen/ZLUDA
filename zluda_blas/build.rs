#[cfg(windows)]
fn main() {
    extern crate embed_resource;
    embed_resource::compile("../zluda_windows/manifest.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}

#[cfg(not(windows))]
fn main() {}
