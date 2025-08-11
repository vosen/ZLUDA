#![cfg(target_os = "windows")]

extern crate detours_sys;
extern crate winapi;

use std::{ffi::c_void, mem, ptr, slice, usize};

use detours_sys::{
    DetourAttach, DetourRestoreAfterWith, DetourTransactionAbort, DetourTransactionBegin,
    DetourTransactionCommit, DetourUpdateProcessWithDll, DetourUpdateThread,
};
use wchar::wch;
use winapi::{
    shared::minwindef::{BOOL, LPVOID},
    um::{
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        libloaderapi::GetModuleFileNameW,
        minwinbase::LPSECURITY_ATTRIBUTES,
        processthreadsapi::{
            CreateProcessA, GetCurrentProcessId, GetCurrentThreadId, OpenThread, ResumeThread,
            SuspendThread, TerminateProcess, LPPROCESS_INFORMATION, LPSTARTUPINFOA, LPSTARTUPINFOW,
        },
        tlhelp32::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        winbase::CREATE_SUSPENDED,
        winnt::{LPSTR, LPWSTR, THREAD_SUSPEND_RESUME},
    },
};
use winapi::{
    shared::minwindef::{DWORD, FALSE, HMODULE, TRUE},
    um::{libloaderapi::LoadLibraryExA, winnt::LPCSTR},
};
use winapi::{
    shared::minwindef::{FARPROC, HINSTANCE},
    um::{
        libloaderapi::GetProcAddress,
        processthreadsapi::{CreateProcessAsUserW, CreateProcessW},
        winbase::{CreateProcessWithLogonW, CreateProcessWithTokenW},
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HANDLE, LPCWSTR},
    },
};
use winapi::{
    shared::winerror::NO_ERROR,
    um::libloaderapi::{LoadLibraryA, LoadLibraryExW, LoadLibraryW},
};

include!("payload_guid.rs");

const NVCUDA_UTF8: &'static str = "NVCUDA.DLL";
const NVCUDA_UTF16: &[u16] = wch!("NVCUDA.DLL");
const NVML_UTF8: &'static str = "NVML.DLL";
const NVML_UTF16: &[u16] = wch!("NVML.DLL");
static mut ZLUDA_PATH_UTF8: Option<&'static [u8]> = None;
static mut ZLUDA_PATH_UTF16: Vec<u16> = Vec::new();
static mut ZLUDA_ML_PATH_UTF8: Option<&'static [u8]> = None;
static mut ZLUDA_ML_PATH_UTF16: Vec<u16> = Vec::new();
static mut CURRENT_MODULE_FILENAME: Vec<u8> = Vec::new();
static mut DETOUR_STATE: Option<DetourDetachGuard> = None;

#[no_mangle]
#[used]
pub static ZLUDA_REDIRECT: () = ();

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

static mut CREATE_PROCESS_A: unsafe extern "system" fn(
    lpApplicationName: LPCSTR,
    lpCommandLine: LPSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCSTR,
    lpStartupInfo: LPSTARTUPINFOA,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL = CreateProcessA;

static mut CREATE_PROCESS_W: unsafe extern "system" fn(
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL = CreateProcessW;

static mut CREATE_PROCESS_AS_USER_W: unsafe extern "system" fn(
    hToken: HANDLE,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL = CreateProcessAsUserW;

static mut CREATE_PROCESS_WITH_TOKEN_W: unsafe extern "system" fn(
    hToken: HANDLE,
    dwLogonFlags: DWORD,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL = CreateProcessWithTokenW;

static mut CREATE_PROCESS_WITH_LOGON_W: unsafe extern "system" fn(
    lpUsername: LPCWSTR,
    lpDomain: LPCWSTR,
    lpPassword: LPCWSTR,
    dwLogonFlags: DWORD,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL = CreateProcessWithLogonW;

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn ZludaGetProcAddress_NoRedirect(
    hModule: HMODULE,
    lpProcName: LPCSTR,
) -> FARPROC {
    GetProcAddress(hModule, lpProcName)
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW_NoRedirect(lpLibFileName: LPCWSTR) -> HMODULE {
    (LOAD_LIBRARY_W)(lpLibFileName)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryA(lpLibFileName: LPCSTR) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_PATH_UTF8.unwrap().as_ptr() as *const _
    } else if is_nvml_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF8.unwrap().as_ptr() as *const _
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_A)(nvcuda_file_name)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf16(lpLibFileName) {
        ZLUDA_PATH_UTF16.as_ptr()
    } else if is_nvml_dll_utf16(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF16.as_ptr()
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
        ZLUDA_PATH_UTF8.unwrap().as_ptr() as *const _
    } else if is_nvml_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF8.unwrap().as_ptr() as *const _
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
        ZLUDA_PATH_UTF16.as_ptr()
    } else if is_nvml_dll_utf16(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF16.as_ptr()
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_EX_W)(nvcuda_file_name, hFile, dwFlags)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessA(
    lpApplicationName: LPCSTR,
    lpCommandLine: LPSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCSTR,
    lpStartupInfo: LPSTARTUPINFOA,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_proc_result = CREATE_PROCESS_A(
        lpApplicationName,
        lpCommandLine,
        lpProcessAttributes,
        lpThreadAttributes,
        bInheritHandles,
        dwCreationFlags | CREATE_SUSPENDED,
        lpEnvironment,
        lpCurrentDirectory,
        lpStartupInfo,
        lpProcessInformation,
    );
    continue_create_process_hook(create_proc_result, dwCreationFlags, lpProcessInformation)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessW(
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_proc_result = CREATE_PROCESS_W(
        lpApplicationName,
        lpCommandLine,
        lpProcessAttributes,
        lpThreadAttributes,
        bInheritHandles,
        dwCreationFlags | CREATE_SUSPENDED,
        lpEnvironment,
        lpCurrentDirectory,
        lpStartupInfo,
        lpProcessInformation,
    );
    continue_create_process_hook(create_proc_result, dwCreationFlags, lpProcessInformation)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessAsUserW(
    hToken: HANDLE,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    lpProcessAttributes: LPSECURITY_ATTRIBUTES,
    lpThreadAttributes: LPSECURITY_ATTRIBUTES,
    bInheritHandles: BOOL,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_proc_result = CREATE_PROCESS_AS_USER_W(
        hToken,
        lpApplicationName,
        lpCommandLine,
        lpProcessAttributes,
        lpThreadAttributes,
        bInheritHandles,
        dwCreationFlags | CREATE_SUSPENDED,
        lpEnvironment,
        lpCurrentDirectory,
        lpStartupInfo,
        lpProcessInformation,
    );
    continue_create_process_hook(create_proc_result, dwCreationFlags, lpProcessInformation)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessWithLogonW(
    lpUsername: LPCWSTR,
    lpDomain: LPCWSTR,
    lpPassword: LPCWSTR,
    dwLogonFlags: DWORD,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_proc_result = CREATE_PROCESS_WITH_LOGON_W(
        lpUsername,
        lpDomain,
        lpPassword,
        dwLogonFlags,
        lpApplicationName,
        lpCommandLine,
        dwCreationFlags | CREATE_SUSPENDED,
        lpEnvironment,
        lpCurrentDirectory,
        lpStartupInfo,
        lpProcessInformation,
    );
    continue_create_process_hook(create_proc_result, dwCreationFlags, lpProcessInformation)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaCreateProcessWithTokenW(
    hToken: HANDLE,
    dwLogonFlags: DWORD,
    lpApplicationName: LPCWSTR,
    lpCommandLine: LPWSTR,
    dwCreationFlags: DWORD,
    lpEnvironment: LPVOID,
    lpCurrentDirectory: LPCWSTR,
    lpStartupInfo: LPSTARTUPINFOW,
    lpProcessInformation: LPPROCESS_INFORMATION,
) -> BOOL {
    let create_proc_result = CREATE_PROCESS_WITH_TOKEN_W(
        hToken,
        dwLogonFlags,
        lpApplicationName,
        lpCommandLine,
        dwCreationFlags | CREATE_SUSPENDED,
        lpEnvironment,
        lpCurrentDirectory,
        lpStartupInfo,
        lpProcessInformation,
    );
    continue_create_process_hook(create_proc_result, dwCreationFlags, lpProcessInformation)
}

// This type encapsulates typical calling sequence of detours and cleanup.
// We have two ways we do detours:
// * If we are loaded before nvcuda.dll, we hook LoadLibrary*
// * If we are loaded after nvcuda.dll, we override every cu* function
// Additionally, within both of those we attach to CreateProcess*
struct DetourDetachGuard {
    state: DetourUndoState,
    suspended_threads: Vec<*mut c_void>,
    // First element is the original fn, second is the new fn
    overriden_non_cuda_fns: Vec<(*mut *mut c_void, *mut c_void)>,
}

impl DetourDetachGuard {
    // First element in the pair is ptr to original fn, second argument is the
    // new function. We accept *mut *mut c_void instead of *mut c_void as the
    // first element in the pair, because somehow otherwise original functions
    // also get overriden, so for example ZludaLoadLibraryExW ends calling
    // itself recursively until stack overflow exception occurs
    unsafe fn new<'a>() -> Option<Self> {
        let mut result = DetourDetachGuard {
            state: DetourUndoState::DoNothing,
            suspended_threads: Vec::new(),
            overriden_non_cuda_fns: Vec::new(),
        };
        if DetourTransactionBegin() != NO_ERROR as i32 {
            return None;
        }
        result.state = DetourUndoState::AbortTransactionResumeThreads;
        if !Self::suspend_all_threads_except_current(&mut result.suspended_threads) {
            return None;
        }
        for thread_handle in result.suspended_threads.iter().copied() {
            if DetourUpdateThread(thread_handle) != NO_ERROR as i32 {
                return None;
            }
        }
        result.overriden_non_cuda_fns.extend_from_slice(&[
            (
                &mut LOAD_LIBRARY_A as *mut _ as *mut *mut c_void,
                ZludaLoadLibraryA as *mut c_void,
            ),
            (&mut LOAD_LIBRARY_W as *mut _ as _, ZludaLoadLibraryW as _),
            (
                &mut LOAD_LIBRARY_EX_A as *mut _ as _,
                ZludaLoadLibraryExA as _,
            ),
            (
                &mut LOAD_LIBRARY_EX_W as *mut _ as _,
                ZludaLoadLibraryExW as _,
            ),
            (
                &mut CREATE_PROCESS_A as *mut _ as _,
                ZludaCreateProcessA as _,
            ),
            (
                &mut CREATE_PROCESS_W as *mut _ as _,
                ZludaCreateProcessW as _,
            ),
            (
                &mut CREATE_PROCESS_AS_USER_W as *mut _ as _,
                ZludaCreateProcessAsUserW as _,
            ),
            (
                &mut CREATE_PROCESS_WITH_LOGON_W as *mut _ as _,
                ZludaCreateProcessWithLogonW as _,
            ),
            (
                &mut CREATE_PROCESS_WITH_TOKEN_W as *mut _ as _,
                ZludaCreateProcessWithTokenW as _,
            ),
        ]);
        for (original_fn, new_fn) in result.overriden_non_cuda_fns.iter().copied() {
            if DetourAttach(original_fn, new_fn) != NO_ERROR as i32 {
                return None;
            }
        }
        if DetourTransactionCommit() != NO_ERROR as i32 {
            return None;
        }
        result.state = DetourUndoState::DoNothing;
        // HACK ALERT
        // I really have no idea how this could happen.
        // Perhaps a thread was closed?
        if !result.resume_threads() {
            if cfg!(debug_assertions) {
                panic!();
            }
        }
        result.state = DetourUndoState::DetachDetours;
        Some(result)
    }

    unsafe fn suspend_all_threads_except_current(threads: &mut Vec<*mut c_void>) -> bool {
        let thread_snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0);
        if thread_snapshot == INVALID_HANDLE_VALUE {
            return false;
        }
        let current_thread = GetCurrentThreadId();
        let current_process = GetCurrentProcessId();
        let mut thread = mem::zeroed::<THREADENTRY32>();
        thread.dwSize = mem::size_of::<THREADENTRY32>() as u32;
        if Thread32First(thread_snapshot, &mut thread) == 0 {
            CloseHandle(thread_snapshot);
            return false;
        }
        loop {
            if thread.th32OwnerProcessID == current_process && thread.th32ThreadID != current_thread
            {
                let thread_handle = OpenThread(THREAD_SUSPEND_RESUME, 0, thread.th32ThreadID);
                if thread_handle == ptr::null_mut() {
                    CloseHandle(thread_snapshot);
                    return false;
                }
                if SuspendThread(thread_handle) == (-1i32 as u32) {
                    CloseHandle(thread_handle);
                    CloseHandle(thread_snapshot);
                    return false;
                }
                threads.push(thread_handle);
            }
            if Thread32Next(thread_snapshot, &mut thread) == 0 {
                break;
            }
        }
        CloseHandle(thread_snapshot);
        true
    }

    // returns true on success
    unsafe fn resume_threads(&self) -> bool {
        let mut success = true;
        for t in self.suspended_threads.iter().copied() {
            if ResumeThread(t) == -1i32 as u32 {
                success = false;
            }
            if CloseHandle(t) == 0 {
                success = false;
            }
        }
        success
    }
}

impl Drop for DetourDetachGuard {
    fn drop(&mut self) {
        match self.state {
            DetourUndoState::DoNothing => {}
            DetourUndoState::AbortTransactionResumeThreads => {
                unsafe { DetourTransactionAbort() };
                unsafe { self.resume_threads() };
            }
            DetourUndoState::DetachDetours => {
                // TODO: implement
            }
        }
    }
}

// Along with Drop impl this forms a state machine for undoing detours.
// I would like to model this as a an usual full state machine with fields in
// variants, but you can't move fields out of type that implements Drop
enum DetourUndoState {
    DoNothing,
    AbortTransactionResumeThreads,
    DetachDetours,
}

unsafe fn continue_create_process_hook(
    create_proc_result: BOOL,
    original_creation_flags: DWORD,
    process_information: LPPROCESS_INFORMATION,
) -> BOOL {
    if create_proc_result == 0 {
        return 0;
    }
    // Detours injection can fail for various reasons, like child being 32bit.
    // If we did not manage to inject then too bad, it's better if the child
    // continues uninjected than to break the parent
    if DetourUpdateProcessWithDll(
        (*process_information).hProcess,
        &mut ZLUDA_ML_PATH_UTF8.unwrap().as_ptr() as *mut _ as *mut _,
        1,
    ) != FALSE
        && DetourUpdateProcessWithDll(
            (*process_information).hProcess,
            &mut ZLUDA_PATH_UTF8.unwrap().as_ptr() as *mut _ as *mut _,
            1,
        ) != FALSE
        && DetourUpdateProcessWithDll(
            (*process_information).hProcess,
            &mut CURRENT_MODULE_FILENAME.as_ptr() as *mut _ as *mut _,
            1,
        ) != FALSE
    {
        detours_sys::DetourCopyPayloadToProcess(
            (*process_information).hProcess,
            &PAYLOAD_NVML_GUID,
            ZLUDA_ML_PATH_UTF16.as_ptr() as *mut _,
            (ZLUDA_ML_PATH_UTF16.len() * mem::size_of::<u16>()) as u32,
        );
        detours_sys::DetourCopyPayloadToProcess(
            (*process_information).hProcess,
            &PAYLOAD_NVCUDA_GUID,
            ZLUDA_PATH_UTF16.as_ptr() as *mut _,
            (ZLUDA_PATH_UTF16.len() * mem::size_of::<u16>()) as u32,
        );
    }
    if original_creation_flags & CREATE_SUSPENDED == 0 {
        if ResumeThread((*process_information).hThread) == -1i32 as u32 {
            TerminateProcess((*process_information).hProcess, 1);
            return 0;
        }
    }
    create_proc_result
}

fn is_nvcuda_dll_utf8(lib: *const u8) -> bool {
    is_dll_utf8(lib, NVCUDA_UTF8.as_bytes())
}

fn is_nvcuda_dll_utf16(lib: *const u16) -> bool {
    is_dll_utf16(lib, NVCUDA_UTF16)
}

fn is_nvml_dll_utf8(lib: *const u8) -> bool {
    is_dll_utf8(lib, NVML_UTF8.as_bytes())
}

fn is_nvml_dll_utf16(lib: *const u16) -> bool {
    is_dll_utf16(lib, NVML_UTF16)
}

fn is_dll_utf8(lib: *const u8, name: &[u8]) -> bool {
    is_dll_impl(lib, 0, name, |c| {
        if c >= 'a' as u8 && c <= 'z' as u8 {
            c - 32
        } else {
            c
        }
    })
}

fn is_dll_utf16(lib: *const u16, name: &[u16]) -> bool {
    is_dll_impl(lib, 0u16, name, |c| {
        if c >= 'a' as u16 && c <= 'z' as u16 {
            c - 32
        } else {
            c
        }
    })
}

fn is_dll_impl<T: Copy + PartialEq>(
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
unsafe extern "system" fn DllMain(instDLL: HINSTANCE, dwReason: u32, _: *const u8) -> i32 {
    if dwReason == DLL_PROCESS_ATTACH {
        if DetourRestoreAfterWith() == FALSE {
            return FALSE;
        }
        if !initialize_globals(instDLL) {
            return FALSE;
        }
        match DetourDetachGuard::new() {
            Some(g) => {
                DETOUR_STATE = Some(g);
                TRUE
            }
            None => FALSE,
        }
    } else if dwReason == DLL_PROCESS_DETACH {
        match DETOUR_STATE.take() {
            Some(_) => TRUE,
            None => FALSE,
        }
    } else {
        TRUE
    }
}

unsafe fn initialize_globals(current_module: HINSTANCE) -> bool {
    let mut module_name = vec![0; 128 as usize];
    loop {
        let size = GetModuleFileNameW(
            current_module,
            module_name.as_mut_ptr(),
            module_name.len() as u32,
        );
        if size == 0 {
            return false;
        }
        if size < module_name.len() as u32 {
            module_name.truncate(size as usize);
            module_name.push(0);
            CURRENT_MODULE_FILENAME = String::from_utf16_lossy(&module_name).into_bytes();
            break;
        }
        module_name.resize(module_name.len() * 2, 0);
    }
    if !load_global_string(
        &PAYLOAD_NVML_GUID,
        &mut ZLUDA_ML_PATH_UTF8,
        &mut ZLUDA_ML_PATH_UTF16,
    ) {
        return false;
    }
    if !load_global_string(
        &PAYLOAD_NVCUDA_GUID,
        &mut ZLUDA_PATH_UTF8,
        &mut ZLUDA_PATH_UTF16,
    ) {
        return false;
    }
    true
}

fn load_global_string(
    guid: &detours_sys::GUID,
    utf8_path: &mut Option<&'static [u8]>,
    utf16_path: &mut Vec<u16>,
) -> bool {
    if let Some(payload) = get_payload(guid) {
        *utf8_path = Some(payload);
        *utf16_path = unsafe { std::str::from_utf8_unchecked(payload) }
            .encode_utf16()
            .collect::<Vec<_>>();
        true
    } else {
        false
    }
}

fn get_payload(guid: &detours_sys::GUID) -> Option<&'static [u8]> {
    let mut size = 0;
    let payload_ptr = unsafe { detours_sys::DetourFindPayloadEx(guid, &mut size) };
    if payload_ptr != ptr::null_mut() {
        Some(unsafe { slice::from_raw_parts(payload_ptr as *const _, size as usize) })
    } else {
        None
    }
}
