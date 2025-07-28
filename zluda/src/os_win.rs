use std::ffi::c_void;

const DLL_PROCESS_DETACH: u32 = 0;

#[allow(non_snake_case, unused_variables)]
#[no_mangle]
extern "system" fn DllMain(_dll_module: *const c_void, call_reason: u32, _: *const c_void) -> bool {
    if call_reason == DLL_PROCESS_DETACH {
        super::deinitialize();
    }

    true
}
