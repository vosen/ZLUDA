use std::{
    ffi::{c_void, CStr},
    ptr,
};
use zluda_windows;

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

#[no_mangle]
static __pfnDliFailureHook2: zluda_windows::PfnDliHook = delaylink_hook;

unsafe extern "system" fn delaylink_hook(
    dli_notify: u32,
    pdli: *const zluda_windows::DelayLoadInfo,
) -> *mut std::ffi::c_void {
    match delaylink_hook_impl(dli_notify, pdli) {
        Some(lib) => lib.into_raw() as *mut c_void,
        None => ptr::null_mut(),
    }
}

unsafe fn delaylink_hook_impl(
    dli_notify: u32,
    pdli: *const zluda_windows::DelayLoadInfo,
) -> Option<libloading::os::windows::Library> {
    if dli_notify != zluda_windows::DliNotify::FailLoadLib as u32 {
        return None;
    }
    let pdli = pdli.as_ref()?;
    let name = CStr::from_ptr(pdli.sz_dll);
    if !name.to_str().ok()?.eq_ignore_ascii_case("amdhip64_7.dll") {
        return None;
    }
    libloading::os::windows::Library::new("amdhip64_6.dll").ok()
}
