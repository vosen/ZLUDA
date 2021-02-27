#![crate_type = "cdylib"]

extern "system" {
    fn cuInit(flags: u32) -> u32;
}

#[no_mangle]
unsafe extern "system" fn do_cuinit(flags: u32) -> u32 {
    cuInit(flags)
}
