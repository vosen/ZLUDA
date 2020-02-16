#![cfg(windows)]

extern crate detours_sys;
#[macro_use]
extern crate guid;
extern crate winapi;

use std::mem;

use detours_sys::{
    DetourAttach, DetourDetach, DetourRestoreAfterWith, DetourTransactionBegin,
    DetourTransactionCommit, DetourUpdateThread,
};
use wchar::{wch, wch_c};
use winapi::shared::minwindef::{DWORD, FALSE, HMODULE, TRUE};
use winapi::um::libloaderapi::LoadLibraryExW;
use winapi::um::processthreadsapi::GetCurrentThread;
use winapi::um::winbase::lstrcmpiW;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HANDLE, LPCWSTR};

const NVCUDA_PATH: &[u16] = wch_c!(r"C:\WINDOWS\system32\nvcuda.dll");
const NOTCUDA_DLL: &[u16] = wch!(r"nvcuda.dll");
static mut NOTCUDA_PATH: Option<Vec<u16>> = None;

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
    let nvcuda_file_name = if lstrcmpiW(lpLibFileName, NVCUDA_PATH.as_ptr()) == 0 {
        NOTCUDA_PATH.as_ref().unwrap().as_ptr()
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_EX)(nvcuda_file_name, hFile, dwFlags)
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn DllMain(_: *const u8, dwReason: u32, _: *const u8) -> i32 {
    if dwReason == DLL_PROCESS_ATTACH {
        DetourRestoreAfterWith();
        match get_notcuda_dll_path() {
            Some((path, len)) => set_notcuda_dll_path(path, len),
            None => return FALSE,
        }
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourAttach(
            std::mem::transmute(&mut LOAD_LIBRARY_EX),
            NotCudaLoadLibraryExW as *mut _,
        );
        DetourTransactionCommit();
    } else if dwReason == DLL_PROCESS_DETACH {
        DetourTransactionBegin();
        DetourUpdateThread(GetCurrentThread());
        DetourDetach(
            std::mem::transmute(&mut LOAD_LIBRARY_EX),
            NotCudaLoadLibraryExW as *mut _,
        );
        DetourTransactionCommit();
    }
    TRUE
}

fn get_notcuda_dll_path() -> Option<(*const u16, usize)> {
    let guid = guid! {"C225FC0C-00D7-40B8-935A-7E342A9344C1"};
    let mut module = std::ptr::null_mut();
    loop {
        module = unsafe { detours_sys::DetourEnumerateModules(module) };
        if module == std::ptr::null_mut() {
            break;
        }
        let mut size = 0;
        let payload = unsafe {
            detours_sys::DetourFindPayload(module, std::mem::transmute(&guid), &mut size)
        };
        if payload != std::ptr::null_mut() {
            return Some((payload as *const _, (size as usize) / mem::size_of::<u16>()));
        }
    }
    None
}

unsafe fn set_notcuda_dll_path(path: *const u16, len: usize) {
    let len = len as usize;
    let mut result = Vec::<u16>::with_capacity(len + NOTCUDA_DLL.len() + 2);
    for i in 0..len {
        result.push(*path.add(i));
    }
    result.push(0x5c); // \
    for c in NOTCUDA_DLL.iter().copied() {
        result.push(c);
    }
    result.push(0);
    NOTCUDA_PATH = Some(result);
}
