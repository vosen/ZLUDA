use vergen_gix::{Emitter, GixBuilder};

fn main() {
    let git = GixBuilder::all_git().unwrap();
    Emitter::default()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
