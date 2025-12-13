use vergen_gix::{Emitter, GixBuilder};

fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-arg=delayimp.lib");
        println!("cargo:rustc-link-arg=/DELAYLOAD:amdhip64_7.dll");
    }
    let git = GixBuilder::default().sha(false).build().unwrap();
    Emitter::default()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
