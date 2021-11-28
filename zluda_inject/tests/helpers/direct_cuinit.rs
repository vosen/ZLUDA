#![crate_type = "bin"]

extern "system" {
    fn cuInit(flags: u32) -> u32;
}

fn main() {
    unsafe { cuInit(0) };
}
