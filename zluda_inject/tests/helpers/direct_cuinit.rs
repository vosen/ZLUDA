#![crate_type = "bin"]

#[link(name = "nvcuda", kind = "raw-dylib")]
extern "system" {
    fn cuInit(flags: u32) -> u32;
}

fn main() {
    unsafe { cuInit(0) };
}
