#![crate_type = "cdylib"]

#[link(name = "nvcuda", kind = "raw-dylib")]
extern "system" {
    fn cuInit(flags: u32) -> u32;
}

#[no_mangle]
unsafe extern "system" fn do_cuinit(flags: u32) -> u32 {
    cuInit(flags)
}
