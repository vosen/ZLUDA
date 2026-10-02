use vergen_gix::{Emitter, Gix};

fn main() {
    let git = Gix::builder().sha(false).build();
    Emitter::default()
        .add_instructions(&git)
        .unwrap()
        .emit()
        .unwrap();
}
