#![cfg(target_os = "windows")]

extern crate detours_sys;
extern crate winapi;

use std::{mem, ptr, slice};

use detours_sys::{
    DetourAttach, DetourDetach, DetourRestoreAfterWith, DetourTransactionBegin,
    DetourTransactionCommit, DetourUpdateThread,
};
use wchar::wch;
use winapi::um::processthreadsapi::GetCurrentThread;
use winapi::um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HANDLE, LPCWSTR};
use winapi::{
    shared::minwindef::{DWORD, FALSE, HMODULE, TRUE},
    um::{libloaderapi::LoadLibraryExA, winnt::LPCSTR},
};
use winapi::{
    shared::winerror::NO_ERROR,
    um::libloaderapi::{LoadLibraryA, LoadLibraryExW, LoadLibraryW},
};

include!("payload_guid.rs");

const NVCUDA_UTF8: &'static str = "NVCUDA.DLL";
const NVCUDA_UTF16: &[u16] = wch!("NVCUDA.DLL");
static mut ZLUDA_PATH_UTF8: Vec<u8> = Vec::new();
static mut ZLUDA_PATH_UTF16: Option<&'static [u16]> = None;

static mut LOAD_LIBRARY_A: unsafe extern "system" fn(lpLibFileName: LPCSTR) -> HMODULE =
    LoadLibraryA;

static mut LOAD_LIBRARY_W: unsafe extern "system" fn(lpLibFileName: LPCWSTR) -> HMODULE =
    LoadLibraryW;

static mut LOAD_LIBRARY_EX_A: unsafe extern "system" fn(
    lpLibFileName: LPCSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE = LoadLibraryExA;

static mut LOAD_LIBRARY_EX_W: unsafe extern "system" fn(
    lpLibFileName: LPCWSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE = LoadLibraryExW;

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_PATH_UTF8.as_ptr() as *const _
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_A)(nvcuda_file_name)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf16(lpLibFileName) {
        ZLUDA_PATH_UTF16.unwrap().as_ptr()
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_W)(nvcuda_file_name)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryExA(
    lpLibFileName: LPCSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_PATH_UTF8.as_ptr() as *const _
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_EX_A)(nvcuda_file_name, hFile, dwFlags)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryExW(
    lpLibFileName: LPCWSTR,
    hFile: HANDLE,
    dwFlags: DWORD,
) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf16(lpLibFileName) {
        ZLUDA_PATH_UTF16.unwrap().as_ptr()
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_EX_W)(nvcuda_file_name, hFile, dwFlags)
}

fn is_nvcuda_dll_utf8(lib: *const u8) -> bool {
    is_nvcuda_dll(lib, 0, NVCUDA_UTF8.as_bytes(), |c| {
        if c >= 'a' as u8 && c <= 'z' as u8 {
            c - 32
        } else {
            c
        }
    })
}
fn is_nvcuda_dll_utf16(lib: *const u16) -> bool {
    is_nvcuda_dll(lib, 0u16, NVCUDA_UTF16, |c| {
        if c >= 'a' as u16 && c <= 'z' as u16 {
            c - 32
        } else {
            c
        }
    })
}

fn is_nvcuda_dll<T: Copy + PartialEq>(
    lib: *const T,
    zero: T,
    dll_name: &[T],
    uppercase: impl Fn(T) -> T,
) -> bool {
    let mut len = 0;
    loop {
        if unsafe { *lib.offset(len) } == zero {
            break;
        }
        len += 1;
    }
    if (len as usize) < dll_name.len() {
        return false;
    }
    let slice =
        unsafe { slice::from_raw_parts(lib.offset(len - dll_name.len() as isize), dll_name.len()) };
    for i in 0..dll_name.len() {
        if uppercase(slice[i]) != dll_name[i] {
            return false;
        }
    }
    true
}

#[allow(non_snake_case)]
#[no_mangle]
unsafe extern "system" fn DllMain(_: *const u8, dwReason: u32, _: *const u8) -> i32 {
    if dwReason == DLL_PROCESS_ATTACH {
        if DetourRestoreAfterWith() == FALSE {
            return FALSE;
        }
        match get_zluda_dll_path() {
            Some(path) => {
                ZLUDA_PATH_UTF16 = Some(path);
                ZLUDA_PATH_UTF8 = String::from_utf16_lossy(path).into_bytes();
            }
            None => return FALSE,
        }
        if DetourTransactionBegin() != NO_ERROR as i32 {
            return FALSE;
        }
        if DetourUpdateThread(GetCurrentThread()) != NO_ERROR as i32 {
            return FALSE;
        }
        if DetourAttach(
            mem::transmute(&mut LOAD_LIBRARY_A),
            ZludaLoadLibraryA as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourAttach(
            mem::transmute(&mut LOAD_LIBRARY_W),
            ZludaLoadLibraryW as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourAttach(
            mem::transmute(&mut LOAD_LIBRARY_EX_A),
            ZludaLoadLibraryExA as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourAttach(
            mem::transmute(&mut LOAD_LIBRARY_EX_W),
            ZludaLoadLibraryExW as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourTransactionCommit() != NO_ERROR as i32 {
            return FALSE;
        }
    } else if dwReason == DLL_PROCESS_DETACH {
        if DetourTransactionBegin() != NO_ERROR as i32 {
            return FALSE;
        }
        if DetourUpdateThread(GetCurrentThread()) != NO_ERROR as i32 {
            return FALSE;
        }
        if DetourDetach(
            mem::transmute(&mut LOAD_LIBRARY_A),
            ZludaLoadLibraryA as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourDetach(
            mem::transmute(&mut LOAD_LIBRARY_W),
            ZludaLoadLibraryW as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourDetach(
            mem::transmute(&mut LOAD_LIBRARY_EX_A),
            ZludaLoadLibraryExA as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourDetach(
            mem::transmute(&mut LOAD_LIBRARY_EX_W),
            ZludaLoadLibraryExW as *mut _,
        ) != NO_ERROR as i32
        {
            return FALSE;
        }
        if DetourTransactionCommit() != NO_ERROR as i32 {
            return FALSE;
        }
    }
    TRUE
}

fn get_zluda_dll_path() -> Option<&'static [u16]> {
    let mut module = ptr::null_mut();
    loop {
        module = unsafe { detours_sys::DetourEnumerateModules(module) };
        if module == ptr::null_mut() {
            break;
        }
        let mut size = 0;
        let payload = unsafe { detours_sys::DetourFindPayload(module, &PAYLOAD_GUID, &mut size) };
        if payload != ptr::null_mut() {
            return unsafe {
                Some(slice::from_raw_parts(
                    payload as *const _,
                    (size as usize) / mem::size_of::<u16>(),
                ))
            };
        }
    }
    None
}
