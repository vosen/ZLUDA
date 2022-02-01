#![cfg(target_os = "windows")]

extern crate detours_sys;
extern crate winapi;

use std::{
    collections::HashMap,
    ffi::{c_void, CStr},
    io, mem,
    os::raw::c_uint,
    ptr, slice, usize,
};

use detours_sys::{
    DetourAllocateRegionWithinJumpBounds, DetourAttach, DetourEnumerateExports,
    DetourRestoreAfterWith, DetourTransactionAbort, DetourTransactionBegin,
    DetourTransactionCommit, DetourUpdateProcessWithDll, DetourUpdateThread,
};
use goblin::pe::{
    self,
    header::{CoffHeader, DOS_MAGIC, PE_MAGIC, PE_POINTER_OFFSET},
    optional_header::StandardFields64,
};
use memoffset::offset_of;
use tempfile::TempDir;
use wchar::wch;
use winapi::{
    shared::minwindef::{DWORD, FALSE, HMODULE, TRUE},
    um::{
        libloaderapi::{GetModuleHandleA, LoadLibraryExA},
        memoryapi::VirtualProtect,
        processthreadsapi::{FlushInstructionCache, GetCurrentProcess},
        sysinfoapi::GetSystemInfo,
        winnt::{LPCSTR, PAGE_READWRITE},
    },
};
use winapi::{
    shared::minwindef::{FARPROC, HINSTANCE},
    um::{
        libloaderapi::{GetModuleFileNameA, GetProcAddress},
        processthreadsapi::{CreateProcessAsUserW, CreateProcessW},
        winbase::{CreateProcessWithLogonW, CreateProcessWithTokenW},
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, HANDLE, LPCWSTR},
    },
};
use winapi::{
    shared::winerror::NO_ERROR,
    um::libloaderapi::{LoadLibraryA, LoadLibraryExW, LoadLibraryW},
};
use winapi::{
    shared::{
        minwindef::{BOOL, LPVOID},
        winerror::E_UNEXPECTED,
    },
    um::{
        handleapi::{CloseHandle, INVALID_HANDLE_VALUE},
        libloaderapi::GetModuleHandleW,
        minwinbase::LPSECURITY_ATTRIBUTES,
        processthreadsapi::{
            CreateProcessA, GetCurrentProcessId, GetCurrentThreadId, OpenThread, ResumeThread,
            SuspendThread, TerminateProcess, LPPROCESS_INFORMATION, LPSTARTUPINFOA, LPSTARTUPINFOW,
        },
        tlhelp32::{
            CreateToolhelp32Snapshot, Thread32First, Thread32Next, TH32CS_SNAPTHREAD, THREADENTRY32,
        },
        winbase::{CopyFileW, CreateSymbolicLinkW, CREATE_SUSPENDED},
        winnt::{LPSTR, LPWSTR, THREAD_SUSPEND_RESUME},
    },
};

include!("payload_guid.rs");

const NVCUDA_UTF8: &'static str = "NVCUDA.DLL";
const NVCUDA_UTF16: &[u16] = wch!("NVCUDA.DLL");
const NVML_UTF8: &'static str = "NVML.DLL";
const NVML_UTF16: &[u16] = wch!("NVML.DLL");
static mut ZLUDA_PATH_UTF8: Vec<u8> = Vec::new();
static mut ZLUDA_PATH_UTF16: Option<&'static [u16]> = None;
static mut ZLUDA_ML_PATH_UTF8: Vec<u8> = Vec::new();
static mut ZLUDA_ML_PATH_UTF16: Option<&'static [u16]> = None;
static mut CURRENT_MODULE_FILENAME: Vec<u8> = Vec::new();
static mut DETOUR_STATE: Option<DetourDetachGuard> = None;
const CUDA_ERROR_NOT_SUPPORTED: c_uint = 801;

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
    if let Some(detour_guard) = &DETOUR_STATE {
        if hModule != ptr::null_mut() && detour_guard.nvcuda_module == hModule {
            let proc_name = CStr::from_ptr(lpProcName);
            return match detour_guard.overriden_cuda_fns.get(proc_name) {
                Some((original_fn, _)) => mem::transmute::<*mut c_void, _>(*original_fn),
                None => ptr::null_mut(),
            };
        }
    }
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
        ZLUDA_PATH_UTF8.as_ptr() as *const _
    } else if is_nvml_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF8.as_ptr() as *const _
    } else {
        lpLibFileName
    };
    (LOAD_LIBRARY_A)(nvcuda_file_name)
}

#[allow(non_snake_case)]
unsafe extern "system" fn ZludaLoadLibraryW(lpLibFileName: LPCWSTR) -> HMODULE {
    let nvcuda_file_name = if is_nvcuda_dll_utf16(lpLibFileName) {
        ZLUDA_PATH_UTF16.unwrap().as_ptr()
    } else if is_nvml_dll_utf16(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF16.unwrap().as_ptr()
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
    } else if is_nvml_dll_utf8(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF8.as_ptr() as *const _
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
    } else if is_nvml_dll_utf16(lpLibFileName as *const _) {
        ZLUDA_ML_PATH_UTF16.unwrap().as_ptr()
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

static mut MAIN: unsafe extern "system" fn() -> DWORD = zluda_main;
static mut COR_EXE_MAIN: unsafe extern "system" fn() -> DWORD = zluda_main_clr;

// https://docs.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-search-order#search-order-for-desktop-applications
// "If a DLL with the same module name is already loaded in memory, the system
// uses the loaded DLL, no matter which directory it is in. The system does not
// search for the DLL."
unsafe extern "system" fn zluda_main() -> DWORD {
    zluda_main_impl(MAIN)
}

unsafe extern "system" fn zluda_main_clr() -> DWORD {
    zluda_main_impl(COR_EXE_MAIN)
}

unsafe fn zluda_main_impl(original: unsafe extern "system" fn() -> DWORD) -> DWORD {
    let temp_dir = match do_zluda_preload() {
        Ok(f) => f,
        Err(e) => return e.raw_os_error().unwrap_or(E_UNEXPECTED) as DWORD,
    };
    let result = original();
    drop(temp_dir);
    result
}

unsafe fn do_zluda_preload() -> std::io::Result<TempDir> {
    let temp_dir = tempfile::tempdir()?;
    do_single_zluda_preload(&temp_dir, ZLUDA_PATH_UTF16.unwrap().as_ptr(), NVCUDA_UTF8)?;
    do_single_zluda_preload(&temp_dir, ZLUDA_ML_PATH_UTF16.unwrap().as_ptr(), NVML_UTF8)?;
    Ok(temp_dir)
}

unsafe fn do_single_zluda_preload(
    temp_dir: &TempDir,
    full_path: *const u16,
    file_name: &'static str,
) -> io::Result<()> {
    let mut temp_file_path = temp_dir.path().to_path_buf();
    temp_file_path.push(file_name);
    let mut temp_file_path_utf16 = temp_file_path
        .into_os_string()
        .to_string_lossy()
        .encode_utf16()
        .collect::<Vec<_>>();
    temp_file_path_utf16.push(0);
    // Probably we are not in developer mode, do a copty then
    if 0 == CreateSymbolicLinkW(
        temp_file_path_utf16.as_ptr(),
        full_path,
        0x2, //SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE
    ) {
        if 0 == CopyFileW(full_path, temp_file_path_utf16.as_ptr(), 1) {
            return Err(io::Error::last_os_error());
        }
    }
    if ptr::null_mut() == ZludaLoadLibraryW_NoRedirect(temp_file_path_utf16.as_ptr()) {
        return Err(io::Error::last_os_error());
    }
    Ok(())
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
    nvcuda_module: HMODULE,
    overriden_cuda_fns: HashMap<&'static CStr, (*mut c_void, *mut c_void)>,
}

impl DetourDetachGuard {
    // First element in the pair is ptr to original fn, second argument is the
    // new function. We accept *mut *mut c_void instead of *mut c_void as the
    // first element in the pair, because somehow otherwise original functions
    // also get overriden, so for example ZludaLoadLibraryExW ends calling
    // itself recursively until stack overflow exception occurs
    unsafe fn detour_functions<'a>(
        nvcuda_module: HMODULE,
        non_cuda_fns: Vec<(*mut *mut c_void, *mut c_void)>,
        cuda_fns: HashMap<&'static CStr, (*mut c_void, *mut c_void)>,
    ) -> Option<Self> {
        let mut result = DetourDetachGuard {
            state: DetourUndoState::DoNothing,
            suspended_threads: Vec::new(),
            overriden_non_cuda_fns: non_cuda_fns,
            nvcuda_module,
            overriden_cuda_fns: cuda_fns,
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
        for (original_fn, new_fn) in result.overriden_non_cuda_fns.iter().copied().chain(
            result
                .overriden_cuda_fns
                .values_mut()
                .map(|(original_ptr, new_ptr)| (original_ptr as *mut _, *new_ptr)),
        ) {
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
        &mut CURRENT_MODULE_FILENAME.as_ptr() as *mut _ as *mut _,
        1,
    ) != FALSE
    {
        detours_sys::DetourCopyPayloadToProcess(
            (*process_information).hProcess,
            &PAYLOAD_NVML_GUID,
            ZLUDA_ML_PATH_UTF16.unwrap().as_ptr() as *mut _,
            (ZLUDA_ML_PATH_UTF16.unwrap().len() * mem::size_of::<u16>()) as u32,
        );
        detours_sys::DetourCopyPayloadToProcess(
            (*process_information).hProcess,
            &PAYLOAD_NVCUDA_GUID,
            ZLUDA_PATH_UTF16.unwrap().as_ptr() as *mut _,
            (ZLUDA_PATH_UTF16.unwrap().len() * mem::size_of::<u16>()) as u32,
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
        if !initialize_current_module_name(instDLL) {
            return FALSE;
        }
        match get_zluda_dlls_paths() {
            Some((nvcuda_path, nvml_path)) => {
                ZLUDA_PATH_UTF16 = Some(nvcuda_path);
                ZLUDA_ML_PATH_UTF16 = Some(nvml_path);
                // from_utf16_lossy(...) handles terminating NULL correctly
                ZLUDA_PATH_UTF8 = String::from_utf16_lossy(nvcuda_path).into_bytes();
                ZLUDA_ML_PATH_UTF8 = String::from_utf16_lossy(nvml_path).into_bytes();
            }
            None => return FALSE,
        }
        // If the application (directly or not) links to nvcuda.dll, nvcuda.dll
        // will get loaded before we can act. In this case, instead of
        // redirecting LoadLibrary* to load ZLUDA, we override already loaded
        // functions
        let detach_guard = match get_cuinit() {
            Some((nvcuda_mod, _)) => detour_already_loaded_nvcuda(nvcuda_mod),
            None => detour_main(),
        };
        match detach_guard {
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

#[must_use]
unsafe fn initialize_current_module_name(current_module: HINSTANCE) -> bool {
    let mut name = vec![0; 128 as usize];
    loop {
        let size = GetModuleFileNameA(
            current_module,
            name.as_mut_ptr() as *mut _,
            name.len() as u32,
        );
        if size == 0 {
            return false;
        }
        if size < name.len() as u32 {
            name.truncate(size as usize);
            CURRENT_MODULE_FILENAME = name;
            return true;
        }
        name.resize(name.len() * 2, 0);
    }
}

unsafe fn get_cuinit() -> Option<(HMODULE, FARPROC)> {
    let nvcuda = GetModuleHandleA(b"nvcuda\0".as_ptr() as _);
    if nvcuda == ptr::null_mut() {
        return None;
    }
    let cuinit_addr = GetProcAddress(nvcuda, b"cuInit\0".as_ptr() as _);
    if cuinit_addr == ptr::null_mut() {
        return None;
    }
    Some((nvcuda as *mut _, cuinit_addr))
}

#[must_use]
unsafe fn detour_already_loaded_nvcuda(nvcuda_mod: HMODULE) -> Option<DetourDetachGuard> {
    let zluda_module = LoadLibraryW(ZLUDA_PATH_UTF16.unwrap().as_ptr());
    if zluda_module == ptr::null_mut() {
        return None;
    }
    let original_functions = gather_imports(nvcuda_mod);
    let override_functions = gather_imports(zluda_module);
    let mut override_fn_pairs = HashMap::with_capacity(original_functions.len());
    // TODO: optimize
    for (original_fn_name, original_fn_address) in original_functions {
        let override_fn_address =
            match override_functions.binary_search_by_key(&original_fn_name, |(name, _)| *name) {
                Ok(x) => override_functions[x].1,
                Err(_) => {
                    // TODO: print a warning in debug
                    cuda_unsupported as _
                }
            };
        override_fn_pairs.insert(
            original_fn_name,
            (original_fn_address as _, override_fn_address),
        );
    }
    let detour_functions = vec![
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
    ];
    DetourDetachGuard::detour_functions(nvcuda_mod, detour_functions, override_fn_pairs)
}

unsafe extern "system" fn cuda_unsupported() -> c_uint {
    CUDA_ERROR_NOT_SUPPORTED
}

unsafe fn gather_imports(module: HINSTANCE) -> Vec<(&'static CStr, *mut c_void)> {
    let mut result = Vec::new();
    DetourEnumerateExports(
        module as _,
        &mut result as *mut _ as *mut _,
        Some(gather_imports_impl),
    );
    result
}

unsafe extern "stdcall" fn gather_imports_impl(
    context: *mut c_void,
    _: u32,
    name: LPCSTR,
    code: *mut c_void,
) -> i32 {
    let result: &mut Vec<(&'static CStr, *mut c_void)> = &mut *(context as *mut Vec<_>);
    result.push((CStr::from_ptr(name), code));
    TRUE
}

#[must_use]
unsafe fn detour_main() -> Option<DetourDetachGuard> {
    if !override_entry_point() {
        return None;
    }
    let mut detour_functions = vec![
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
    ];
    detour_functions.extend(get_clr_entry_point());
    DetourDetachGuard::detour_functions(ptr::null_mut(), detour_functions, HashMap::new())
}

unsafe fn override_entry_point() -> bool {
    let exe_handle = GetModuleHandleW(ptr::null());
    let dos_signature = exe_handle as *mut u16;
    if *dos_signature != DOS_MAGIC {
        return false;
    }
    let pe_offset = *((exe_handle as *mut u8).add(PE_POINTER_OFFSET as usize) as *mut u32);
    let pe_sig = (exe_handle as *mut u8).add(pe_offset as usize) as *mut u32;
    if (*pe_sig) != PE_MAGIC {
        return false;
    }
    let coff_header = pe_sig.add(1) as *mut CoffHeader;
    let standard_coff_fields = coff_header.add(1) as *mut StandardFields64;
    if (*standard_coff_fields).magic != pe::optional_header::MAGIC_64 {
        return false;
    }
    let entry_point = mem::transmute::<_, unsafe extern "system" fn() -> DWORD>(
        (exe_handle as *mut u8).add((*standard_coff_fields).address_of_entry_point as usize),
    );
    let mut allocated_size = 0;
    let exe_region = DetourAllocateRegionWithinJumpBounds(exe_handle as _, &mut allocated_size);
    if (allocated_size as usize) < mem::size_of::<JmpThunk64>() || exe_region == ptr::null_mut() {
        return false;
    }
    MAIN = entry_point;
    *(exe_region as *mut JmpThunk64) = JmpThunk64::new(zluda_main);
    FlushInstructionCache(
        GetCurrentProcess(),
        exe_region,
        mem::size_of::<JmpThunk64>(),
    );
    let new_address_of_entry_point = (exe_region as *mut u8).offset_from(exe_handle as *mut u8);
    let entry_point_offset = offset_of!(StandardFields64, address_of_entry_point);
    let mut system_info = mem::zeroed();
    GetSystemInfo(&mut system_info);
    let pointer_to_address_of_entry_point =
        (standard_coff_fields as *mut u8).add(entry_point_offset) as *mut i32;
    let page_size = system_info.dwPageSize as usize;
    let page_start = (((pointer_to_address_of_entry_point as usize) / page_size) * page_size) as _;
    let mut old_protect = 0;
    if VirtualProtect(page_start, page_size, PAGE_READWRITE, &mut old_protect) == 0 {
        return false;
    }
    *pointer_to_address_of_entry_point = new_address_of_entry_point as i32;
    if VirtualProtect(page_start, page_size, old_protect, &mut old_protect) == 0 {
        return false;
    }
    true
}

// mov rax, $address;
// jmp rax;
// int 3;
#[repr(packed)]
#[allow(dead_code)]
#[cfg(target_pointer_width = "64")]
struct JmpThunk64 {
    mov_rax: [u8; 2],
    address: u64,
    jmp_rax: [u8; 2],
    int3: u8,
}

impl JmpThunk64 {
    fn new<T: Sized>(target: unsafe extern "system" fn() -> T) -> Self {
        JmpThunk64 {
            mov_rax: [0x48, 0xB8],
            address: target as u64,
            jmp_rax: [0xFF, 0xE0],
            int3: 0xcc,
        }
    }
}

unsafe fn get_clr_entry_point() -> Option<(*mut *mut c_void, *mut c_void)> {
    let mscoree = GetModuleHandleA(b"mscoree\0".as_ptr() as _);
    if mscoree == ptr::null_mut() {
        return None;
    }
    let proc = GetProcAddress(mscoree, b"_CorExeMain\0".as_ptr() as _);
    if proc == ptr::null_mut() {
        return None;
    }
    COR_EXE_MAIN = mem::transmute(proc);
    Some((&mut COR_EXE_MAIN as *mut _ as _, zluda_main_clr as _))
}

fn get_zluda_dlls_paths() -> Option<(&'static [u16], &'static [u16])> {
    match get_payload(&PAYLOAD_NVCUDA_GUID) {
        None => None,
        Some(nvcuda_payload) => match get_payload(&PAYLOAD_NVML_GUID) {
            None => return None,
            Some(nvml_payload) => return Some((nvcuda_payload, nvml_payload)),
        },
    }
}

fn get_payload(guid: &detours_sys::GUID) -> Option<&'static [u16]> {
    let mut module = ptr::null_mut();
    loop {
        module = unsafe { detours_sys::DetourEnumerateModules(module) };
        if module == ptr::null_mut() {
            return None;
        }
        let mut size = 0;
        let payload_ptr = unsafe { detours_sys::DetourFindPayload(module, guid, &mut size) };
        if payload_ptr != ptr::null_mut() {
            return Some(unsafe {
                slice::from_raw_parts(
                    payload_ptr as *const _,
                    (size as usize) / mem::size_of::<u16>(),
                )
            });
        }
    }
}
