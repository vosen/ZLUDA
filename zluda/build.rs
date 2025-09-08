use vergen_gix::{Emitter, GixBuilder};

fn main() {
    let git = GixBuilder::default().sha(false).build().unwrap();
    Emitter::default()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
