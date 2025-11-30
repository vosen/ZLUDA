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

pub fn try_load_library(name: &str) -> Result<libloading::Library, libloading::Error> {
    unsafe { libloading::Library::new(name) }
        .or_else(|_| unsafe { load_from_self_dir(name).map(Into::into) })
}

unsafe fn load_from_self_dir(
    name: &str,
) -> Result<libloading::os::windows::Library, libloading::Error> {
    zluda_windows::try_load_from_self_dir(name)
        .map(|handle| libloading::os::windows::Library::from_raw(handle.0 as _))
        .ok_or(libloading::Error::GetModuleHandleExWUnknown)
}
