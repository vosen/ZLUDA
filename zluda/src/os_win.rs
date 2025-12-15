use std::{ffi::c_void, ptr};
use windows::{
    core::w,
    Win32::{Foundation::HMODULE, System::LibraryLoader::LoadLibraryW},
};
use zluda_windows::{self, DelayLoadInfo, DliNotify};

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
static __pfnDliNotifyHook2: zluda_windows::PfnDliHook =
    zluda_windows::open_already_loaded_amdhip_probe_perflibs;

#[no_mangle]
static __pfnDliFailureHook2: zluda_windows::PfnDliHook = delay_load_downgrade_amdhip;

unsafe extern "system" fn delay_load_downgrade_amdhip(
    dli_notify: u32,
    pdli: *const DelayLoadInfo,
) -> *mut std::ffi::c_void {
    unsafe fn delay_load_downgrade_amdhip_impl(
        dli_notify: u32,
        pdli: *const DelayLoadInfo,
    ) -> Option<HMODULE> {
        zluda_windows::delay_load_check(
            ("amdhip64_7.dll", DliNotify::FailLoadLib),
            dli_notify,
            pdli,
        )?;
        LoadLibraryW(w!("amdhip64_6.dll")).ok()
    }
    match delay_load_downgrade_amdhip_impl(dli_notify, pdli) {
        Some(lib) => lib.0,
        None => ptr::null_mut(),
    }
}
