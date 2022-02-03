#![crate_type = "bin"]

#[link(name = "do_cuinit")]
extern "system" {
    fn do_cuinit(flags: u32) -> u32;
}

fn main() {
    unsafe { do_cuinit(0) };
}
