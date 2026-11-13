#[cfg(windows)]
fn main() {
    extern crate embed_resource;
    embed_resource::compile("manifest.rc", embed_resource::NONE)
        .manifest_required()
        .unwrap();
}

#[cfg(not(windows))]
fn main() {}
