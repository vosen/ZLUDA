extern crate detours_sys;
extern crate winapi;

use detours_sys::{
    DetourAttach, DetourDetach, DetourRestoreAfterWith, DetourTransactionBegin,
    DetourTransactionCommit, DetourUpdateThread,
};
use wchar::wch_c;
use winapi::shared::minwindef::{DWORD, HMODULE, TRUE};
use winapi::um::libloaderapi::LoadLibraryExW;
use winapi::um::processthreadsapi::GetCurrentThread;
use winapi::um::winbase::lstrcmpiW;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HANDLE, LPCWSTR};

const NVCUDA_LONG_PATH: &[u16] = wch_c!(r"C:\WINDOWS\system32\nvcuda.dll");
const NVCUDA_SHORT_PATH: &[u16] = wch_c!("nvcuda.dll");

static mut LOAD_LIBRARY_EX: unsafe extern "system" fn(
    lpLibFileName: LPCWSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE = LoadLibraryExW;

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn NotCudaLoadLibraryExW(
    lpLibFileName: LPCWSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE {
    let nvcuda_file_name = if lstrcmpiW(lpLibFileName, NVCUDA_LONG_PATH.as_ptr()) == 0 {
        NVCUDA_SHORT_PATH.as_ptr()
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_EX)(nvcuda_file_name, hFile, dwFlags)
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn DllMain(_: *const u8, dwReason: u32, _: *const u8) -> i32 {
    if dwReason == DLL_PROCESS_ATTACH {
        let mut load_lib_ex = LoadLibraryExW as *mut _;
        DetourRestoreAfterWith();
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(
            &mut load_lib_ex,
            NotCudaLoadLibraryExW as *mut _,
        );
        DetourTransactionCommit();
        LOAD_LIBRARY_EX = std::mem::transmute(load_lib_ex);
    } else if dwReason == DLL_PROCESS_DETACH {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(
            &mut (LOAD_LIBRARY_EX as *mut _),
            NotCudaLoadLibraryExW as *mut _,
        );
        DetourTransactionCommit();
    }
    TRUE
}
